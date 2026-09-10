//! Kobana products — the first segment of the CLI syntax:
//! `kobana <produto> <servico> <recurso> <metodo>`.
//!
//! Each product is its own API, with its own hosts and its own OpenAPI specs.
//! Adding one means adding a spec file under `specs/` and one entry to
//! [`REGISTRY`] — nothing else in the CLI is product-aware.

use std::collections::BTreeMap;
use std::sync::Arc;

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
    /// How the spec's tree maps onto CLI services
    pub layout: Layout,
}

/// How a spec's command tree becomes CLI services.
pub enum Layout {
    /// The whole spec is a single service under `name`.
    /// Used by banking v1 and by every other product's v1.
    Single {
        name: &'static str,
        about: &'static str,
    },
    /// Each top-level node of the spec tree becomes its own service —
    /// banking v2 exposes `charge`, `payment`, `transfer`, … this way, so `v2`
    /// itself never appears in the CLI.
    SplitTopLevel,
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
            layout: Layout::Single {
                name: "v1",
                about: "API v1 (boletos, clientes, webhooks)",
            },
        },
        SpecEntry {
            json: BANKING_V2_SPEC,
            version_prefix: "/v2",
            layout: Layout::SplitTopLevel,
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
        layout: Layout::Single {
            name: "v1",
            about: "API v1 (workspaces, inboxes, agentes, e-mails, webhooks)",
        },
    }],
    // The inbox edge terminates mTLS and forwards the certificate fingerprint
    // to the origin, so requests must present a client certificate.
    client_cert_env: Some("KOBANA_INBOX_CLIENT_CERT"),
},
];

/// A product with its specs parsed and its command trees built.
pub struct LoadedProduct {
    pub product: &'static Product,
    /// CLI service name → command tree
    pub services: BTreeMap<String, LoadedService>,
}

/// One CLI service (`v1`, `charge`, `payment`, …) with its command tree.
pub struct LoadedService {
    pub about: String,
    pub tree: Arc<CommandNode>,
}

impl LoadedProduct {
    /// Look up a service by its CLI name
    pub fn service(&self, name: &str) -> Option<&LoadedService> {
        self.services.get(name)
    }
}

/// Parse every registered product's specs and build its command trees.
pub fn load_all() -> Result<Vec<LoadedProduct>, KobanaError> {
    REGISTRY.iter().map(load).collect()
}

fn load(product: &'static Product) -> Result<LoadedProduct, KobanaError> {
    let mut services = BTreeMap::new();

    for entry in product.specs {
        let spec = ApiSpec::parse(entry.json)?;
        let tree = spec.build_command_tree(entry.version_prefix);

        match entry.layout {
            Layout::Single { name, about } => {
                services.insert(
                    name.to_string(),
                    LoadedService {
                        about: about.to_string(),
                        tree: Arc::new(tree),
                    },
                );
            }
            Layout::SplitTopLevel => {
                for (name, node) in tree.children {
                    let about = service_about(&name).to_string();
                    services.insert(
                        name,
                        LoadedService {
                            about,
                            tree: Arc::new(node),
                        },
                    );
                }
            }
        }
    }

    Ok(LoadedProduct { product, services })
}

/// Find a loaded product by its CLI slug
pub fn find<'a>(products: &'a [LoadedProduct], slug: &str) -> Option<&'a LoadedProduct> {
    products.iter().find(|p| p.product.slug == slug)
}

/// Whether a slug names a registered product (no specs needed)
pub fn is_product(slug: &str) -> bool {
    REGISTRY.iter().any(|p| p.slug == slug)
}

/// Human-readable about text for services split out of a spec tree
fn service_about(name: &str) -> &'static str {
    match name {
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
        _ => "API Kobana",
    }
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
    fn banking_loads_v1_and_v2_services() {
        let products = load_all().expect("specs must parse");
        let banking = find(&products, "banking").expect("banking must load");

        // v1 stays a single service; v2 is split into its domains
        assert!(banking.service("v1").is_some());
        assert!(banking.service("charge").is_some());
        assert!(banking.service("payment").is_some());
        // `v2` itself is never a CLI segment
        assert!(banking.service("v2").is_none());
    }

    #[test]
    fn endpoint_paths_keep_their_version_prefix() {
        let products = load_all().unwrap();
        let banking = find(&products, "banking").unwrap();

        let v1 = &banking.service("v1").unwrap().tree;
        let billets = v1.children.get("bank-billets").expect("v1 bank-billets");
        assert!(billets
            .endpoints
            .iter()
            .any(|e| e.path_template == "/v1/bank_billets"));

        let charge = &banking.service("charge").unwrap().tree;
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
            for service in loaded.services.values() {
                walk(&service.tree, &mut templates);
            }
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
            for (name, service) in &loaded.services {
                check(&service.tree, &format!("{} {name}", loaded.product.slug));
            }
        }
    }

    #[test]
    fn inbox_is_registered_with_its_resources() {
        let products = load_all().unwrap();
        let inbox = find(&products, "inbox").expect("inbox must load");
        let v1 = &inbox.service("v1").expect("inbox v1").tree;

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
