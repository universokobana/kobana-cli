//! Kobana products — the first segment of the CLI syntax:
//! `kobana <produto> <recurso> <metodo>`.
//!
//! API versions are an implementation detail of the URL, never a CLI segment:
//! a product's specs are merged into one tree, so `/v1/bank_billets` and
//! `/v2/charge/pix` read as `banking bank-billets` and `banking charge pix`.
//!
//! Each product is its own API, with its own hosts and its own OpenAPI specs.
//! Adding one means adding a spec file under `specs/` and one entry to
//! [`REGISTRY`] — nothing else in the CLI is product-aware.

use kobana::client::KobanaClient;
use kobana::error::KobanaError;
use kobana::spec::{ApiSpec, CommandNode};

use crate::config::Environment;

// Embedded OpenAPI specs
const BANKING_V1_SPEC: &str = include_str!("../specs/banking-v1.json");
const BANKING_V2_SPEC: &str = include_str!("../specs/banking-v2.json");
const INBOX_V1_SPEC: &str = include_str!("../specs/inbox-v1.json");

/// A Kobana product exposed as the first CLI segment.
pub struct Product {
    /// CLI slug (e.g., `banking`)
    pub slug: &'static str,
    /// Human-readable help text
    pub about: &'static str,
    /// API hosts per environment
    pub hosts: Hosts,
    /// The specs this product serves
    pub specs: &'static [SpecEntry],
    /// Env var holding the path to a PEM client certificate, for products
    /// whose edge requires mTLS. `None` when the product does not use it.
    pub client_cert_env: Option<&'static str>,
}

/// API hosts for a product, per environment. Never include a version prefix —
/// that lives in the spec's own paths.
pub struct Hosts {
    pub production: &'static str,
    pub sandbox: &'static str,
    /// `None` when the product has no documented local dev host
    pub development: Option<&'static str>,
}

/// One OpenAPI spec belonging to a product.
pub struct SpecEntry {
    /// Embedded OpenAPI JSON
    pub json: &'static str,
    /// API version prefix carried by the spec's own paths, e.g. `/v1`.
    /// Stripped for tree placement; kept in the request URL.
    pub version_prefix: &'static str,
}

impl Product {
    /// Base URL for this product in the given environment.
    ///
    /// Falls back to production when the product has no development host, so
    /// `--env development` never silently targets the wrong product.
    pub fn base_url(&self, env: &Environment) -> &'static str {
        match env {
            Environment::Production => self.hosts.production,
            Environment::Sandbox => self.hosts.sandbox,
            Environment::Development => self.hosts.development.unwrap_or(self.hosts.production),
        }
    }
}

/// Build an HTTP client for a product in the given environment.
///
/// Products behind mTLS load a PEM client certificate from the path in their
/// `client_cert_env` variable. When that variable is unset the client is built
/// without one: local development does not need it, and the API answers with a
/// descriptive 401 otherwise — so this warns instead of refusing to run. The
/// warning is skipped for `dry_run`, which issues no request.
pub fn client_for(
    product: &Product,
    env: &Environment,
    token: &str,
    dry_run: bool,
) -> Result<KobanaClient, KobanaError> {
    let base_url = product.base_url(env);

    let Some(var) = product.client_cert_env else {
        return KobanaClient::new(base_url, token);
    };

    match std::env::var(var) {
        Ok(path) if !path.is_empty() => {
            let pem = std::fs::read(&path).map_err(|e| {
                KobanaError::Auth(format!("could not read {var} ({path}): {e}"))
            })?;
            KobanaClient::with_client_cert(base_url, token, &pem)
        }
        _ => {
            if !dry_run {
                warn(&format!(
                "⚠ {} requires a client certificate (mTLS). Set {} to a PEM file \
                     holding the certificate chain and its private key.",
                    product.slug, var
                ));
            }
            KobanaClient::new(base_url, token)
        }
    }
}

fn warn(msg: &str) {
    if std::io::IsTerminal::is_terminal(&std::io::stderr()) {
        eprintln!("\x1b[33m{msg}\x1b[0m");
    } else {
        eprintln!("{msg}");
    }
}

/// Every product wired into the CLI.
///
/// Only `banking` is served today. The other Kobana products have public APIs
/// documented at `docs.<product>.kobana.com.br` and their hosts follow the same
/// `api.<product>[.sandbox].kobana.com.br` shape, but their specs are not
/// embedded yet — see the notes in AGENTS.md before adding one.
pub static REGISTRY: &[Product] = &[
Product {
    slug: "banking",
    about: "Gateway Bancário — cobranças, pagamentos, transferências (API v1 e v2)",
    hosts: Hosts {
        production: "https://api.kobana.com.br",
        sandbox: "https://api-sandbox.kobana.com.br",
        development: Some("http://localhost:5005/api"),
    },
    specs: &[
        SpecEntry {
            json: BANKING_V1_SPEC,
            version_prefix: "/v1",
        },
        SpecEntry {
            json: BANKING_V2_SPEC,
            version_prefix: "/v2",
        },
    ],
    client_cert_env: None,
},
Product {
    slug: "inbox",
    about: "Inbox Autônomo — caixas de entrada, agentes, e-mails, webhooks",
    hosts: Hosts {
        production: "https://api.inbox.kobana.com.br",
        sandbox: "https://api.inbox.sandbox.kobana.com.br",
        development: Some("http://localhost:3028/api"),
    },
    specs: &[SpecEntry {
        json: INBOX_V1_SPEC,
        version_prefix: "/v1",
    }],
    // The inbox edge terminates mTLS and forwards the certificate fingerprint
    // to the origin, so requests must present a client certificate.
    client_cert_env: Some("KOBANA_INBOX_CLIENT_CERT"),
},
];

/// A product with its specs parsed and merged into one command tree.
pub struct LoadedProduct {
    pub product: &'static Product,
    /// Every resource the product exposes, from all of its specs
    pub tree: CommandNode,
}

/// Parse every registered product's specs and build its command tree.
pub fn load_all() -> Result<Vec<LoadedProduct>, KobanaError> {
    REGISTRY.iter().map(load).collect()
}

fn load(product: &'static Product) -> Result<LoadedProduct, KobanaError> {
    let mut tree = CommandNode::default();

    for entry in product.specs {
        let spec = ApiSpec::parse(entry.json)?;
        merge(&mut tree, spec.build_command_tree(entry.version_prefix));
    }

    Ok(LoadedProduct { product, tree })
}

/// Merge one spec's tree into a product's tree.
///
/// Products span several API versions (banking serves v1 and v2) and they all
/// land in the same tree, so two specs can contribute to the same resource.
fn merge(into: &mut CommandNode, from: CommandNode) {
    for endpoint in from.endpoints {
        // First spec wins: a name can only be one command
        if !into.endpoints.iter().any(|e| e.cli_method == endpoint.cli_method) {
            into.endpoints.push(endpoint);
        }
    }

    for (name, child) in from.children {
        merge(into.children.entry(name).or_default(), child);
    }
}

/// Find a loaded product by its CLI slug
pub fn find<'a>(products: &'a [LoadedProduct], slug: &str) -> Option<&'a LoadedProduct> {
    products.iter().find(|p| p.product.slug == slug)
}

/// Whether a slug names a registered product (no specs needed)
pub fn is_product(slug: &str) -> bool {
    REGISTRY.iter().any(|p| p.slug == slug)
}

/// Human-readable about text for a product's top-level resources.
/// Resources without an entry are described by the API spec itself, or show
/// no description at all.
pub fn resource_about(name: &str) -> Option<&'static str> {
    let about = match name {
        // banking v2 domains
        "charge" => "Cobranças (Pix, boletos, Pix automático)",
        "payment" => "Pagamentos (boletos, Pix, taxas, concessionárias)",
        "transfer" => "Transferências (Pix, TED, interna)",
        "financial" => "Financeiro (contas, saldos, extratos)",
        "admin" => "Administração (subcontas, usuários, conexões)",
        "mailbox" => "Caixa postal (EDI, arquivos)",
        "data" => "Consultas (boletos, QR codes Pix)",
        "edi" => "EDI (caixas EDI)",
        "me" => "Informações da conta",
        "payments" => "Pagamentos (unificado)",
        "transfers" => "Transferências (unificado)",
        "security" => "Segurança (tokens de acesso)",
        _ => return None,
    };
    Some(about)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banking_is_registered() {
        assert!(is_product("banking"));
    }

    #[test]
    fn rejects_unimplemented_products() {
        // These products exist at Kobana but are not wired into the CLI yet
        for slug in ["finance", "billing"] {
            assert!(!is_product(slug), "{slug} should not be available yet");
        }
    }

    #[test]
    fn rejects_service_names_as_products() {
        // Guards against a regression to the old `kobana <service> ...` syntax
        for slug in ["v1", "charge", "payment", "schema", "auth"] {
            assert!(!is_product(slug), "{slug} is not a product");
        }
    }

    #[test]
    fn banking_merges_v1_and_v2_into_one_tree() {
        let products = load_all().expect("specs must parse");
        let banking = find(&products, "banking").expect("banking must load");

        // v1 resources and v2 domains sit side by side under the product
        assert!(banking.tree.children.contains_key("bank-billets")); // v1
        assert!(banking.tree.children.contains_key("customers")); // v1
        assert!(banking.tree.children.contains_key("charge")); // v2
        assert!(banking.tree.children.contains_key("payment")); // v2

        // Versions are never CLI segments
        assert!(!banking.tree.children.contains_key("v1"));
        assert!(!banking.tree.children.contains_key("v2"));
    }

    #[test]
    fn endpoint_paths_keep_their_version_prefix() {
        let products = load_all().unwrap();
        let banking = find(&products, "banking").unwrap();

        let billets = banking
            .tree
            .children
            .get("bank-billets")
            .expect("bank-billets");
        assert!(billets
            .endpoints
            .iter()
            .any(|e| e.path_template == "/v1/bank_billets"));

        let charge = &banking.tree.children["charge"];
        let pix = charge.children.get("pix").expect("charge pix");
        assert!(pix
            .endpoints
            .iter()
            .any(|e| e.path_template == "/v2/charge/pix"));
    }

    /// Every endpoint the CLI can dispatch must point at a path that literally
    /// exists in the OpenAPI spec — tree placement must never invent, drop or
    /// mangle a version prefix.
    #[test]
    fn every_endpoint_path_exists_in_its_spec() {
        let mut spec_paths = std::collections::BTreeSet::new();
        for entry in REGISTRY.iter().flat_map(|p| p.specs) {
            let raw: serde_json::Value = serde_json::from_str(entry.json).unwrap();
            for path in raw["paths"].as_object().unwrap().keys() {
                spec_paths.insert(path.clone());
            }
        }

        fn walk(node: &CommandNode, out: &mut Vec<String>) {
            out.extend(node.endpoints.iter().map(|e| e.path_template.clone()));
            for child in node.children.values() {
                walk(child, out);
            }
        }

        let products = load_all().unwrap();
        let mut templates = Vec::new();
        for loaded in &products {
            walk(&loaded.tree, &mut templates);
        }

        assert!(templates.len() > 200, "expected a full command surface, got {}", templates.len());
        for t in &templates {
            assert!(spec_paths.contains(t), "path_template {t} is not in any spec");
        }
    }

    /// clap panics at startup if a command exposes the same subcommand name
    /// twice, so the tree must be a valid command tree for every product: no
    /// repeated method on a node, and no method colliding with a child node.
    /// This is what `/v1/inboxes/{id}/recipients` (POST + DELETE) used to break.
    #[test]
    fn no_node_exposes_a_duplicate_subcommand_name() {
        fn check(node: &CommandNode, path: &str) {
            let mut seen = std::collections::BTreeSet::new();
            for e in &node.endpoints {
                assert!(
                    seen.insert(e.cli_method.as_str()),
                    "{path} exposes method '{}' twice",
                    e.cli_method
                );
                assert!(
                    !node.children.contains_key(&e.cli_method),
                    "{path} exposes '{}' as both a method and a resource",
                    e.cli_method
                );
            }
            for (name, child) in &node.children {
                check(child, &format!("{path} {name}"));
            }
        }

        for loaded in &load_all().unwrap() {
            check(&loaded.tree, loaded.product.slug);
        }
    }

    #[test]
    fn inbox_is_registered_with_its_resources() {
        let products = load_all().unwrap();
        let inbox = find(&products, "inbox").expect("inbox must load");
        let v1 = &inbox.tree;

        for resource in [
            "workspaces",
            "inboxes",
            "emails",
            "agents",
            "agent-runs",
            "webhooks",
            "system-events",
        ] {
            assert!(v1.children.contains_key(resource), "missing {resource}");
        }

        // Multi-verb action paths become resource nodes
        let recipients = v1.children["inboxes"]
            .children
            .get("recipients")
            .expect("inboxes recipients");
        let mut methods: Vec<&str> =
            recipients.endpoints.iter().map(|e| e.cli_method.as_str()).collect();
        methods.sort();
        assert_eq!(methods, vec!["create", "delete"]);

        assert_eq!(
            inbox.product.base_url(&Environment::Production),
            "https://api.inbox.kobana.com.br"
        );
    }

    /// Two specs of the same product land in one tree. Resources that exist in
    /// both must merge instead of replacing each other, and a command name that
    /// exists in both is kept from the first spec listed.
    #[test]
    fn merge_combines_specs_without_dropping_resources() {
        const A: &str = r#"{
            "info": {"version": "1.0"},
            "paths": {
                "/v1/billets": {"get": {"responses": {}}},
                "/v1/customers": {"get": {"responses": {}}}
            }
        }"#;
        const B: &str = r#"{
            "info": {"version": "1.0"},
            "paths": {
                "/v2/billets": {"get": {"responses": {}}, "post": {"responses": {}}},
                "/v2/charge": {"get": {"responses": {}}}
            }
        }"#;

        let mut tree = ApiSpec::parse(A).unwrap().build_command_tree("/v1");
        merge(&mut tree, ApiSpec::parse(B).unwrap().build_command_tree("/v2"));

        // Resources unique to either spec survive
        assert!(tree.children.contains_key("customers"));
        assert!(tree.children.contains_key("charge"));

        // A shared resource merges: `create` comes from B, `list` from A
        let billets = &tree.children["billets"];
        let mut methods: Vec<&str> =
            billets.endpoints.iter().map(|e| e.cli_method.as_str()).collect();
        methods.sort();
        assert_eq!(methods, vec!["create", "list"]);

        // On a name clash the first spec wins, so `list` keeps the v1 path
        let list = billets.endpoints.iter().find(|e| e.cli_method == "list").unwrap();
        assert_eq!(list.path_template, "/v1/billets");
    }

    #[test]
    fn only_inbox_requires_a_client_certificate() {
        for p in REGISTRY {
            match p.slug {
                "inbox" => assert_eq!(p.client_cert_env, Some("KOBANA_INBOX_CLIENT_CERT")),
                _ => assert_eq!(p.client_cert_env, None, "{} should not need mTLS", p.slug),
            }
        }
    }

    #[test]
    fn base_url_falls_back_to_production_without_dev_host() {
        let banking = &REGISTRY[0];
        assert_eq!(
            banking.base_url(&Environment::Sandbox),
            "https://api-sandbox.kobana.com.br"
        );

        let no_dev = Product {
            slug: "x",
            about: "",
            hosts: Hosts {
                production: "https://prod",
                sandbox: "https://sandbox",
                development: None,
            },
            specs: &[],
            client_cert_env: None,
        };
        assert_eq!(no_dev.base_url(&Environment::Development), "https://prod");
    }
}
