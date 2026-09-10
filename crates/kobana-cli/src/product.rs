//! Kobana products — the first segment of the CLI syntax:
//! `kobana <produto> <servico> <recurso> <metodo>`.
//!
//! Each product is its own API, with its own hosts and its own OpenAPI specs.
//! Adding one means adding a spec file under `specs/` and one entry to
//! [`REGISTRY`] — nothing else in the CLI is product-aware.

use std::collections::BTreeMap;
use std::sync::Arc;

use kobana::error::KobanaError;
use kobana::spec::{ApiSpec, CommandNode};

use crate::config::Environment;

// Embedded OpenAPI specs
const BANKING_V1_SPEC: &str = include_str!("../specs/banking-v1.json");
const BANKING_V2_SPEC: &str = include_str!("../specs/banking-v2.json");

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

/// Every product wired into the CLI.
///
/// Only `banking` is served today. The other Kobana products have public APIs
/// documented at `docs.<product>.kobana.com.br` and their hosts follow the same
/// `api.<product>[.sandbox].kobana.com.br` shape, but their specs are not
/// embedded yet — see the notes in AGENTS.md before adding one.
pub static REGISTRY: &[Product] = &[Product {
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
}];

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
        for slug in ["finance", "billing", "inbox"] {
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
        for entry in REGISTRY[0].specs {
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
        for service in find(&products, "banking").unwrap().services.values() {
            walk(&service.tree, &mut templates);
        }

        assert!(templates.len() > 200, "expected a full command surface, got {}", templates.len());
        for t in &templates {
            assert!(spec_paths.contains(t), "path_template {t} is not in any spec");
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
        };
        assert_eq!(no_dev.base_url(&Environment::Development), "https://prod");
    }
}
