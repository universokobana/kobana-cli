use clap::{Arg, Command};
use kobana::spec::{CommandNode, ResolvedEndpoint};
use kobana::tr;

use crate::clap_i18n;
use crate::helpers;
use crate::product::{self, LoadedProduct};

/// Usage line of a command that takes a subcommand
fn node_usage(path: &str) -> String {
    format!(
        "{path} {} {}",
        clap_i18n::options_placeholder(),
        clap_i18n::command_placeholder()
    )
}

/// Usage line of a command that runs as is
fn leaf_usage(path: &str) -> String {
    format!("{path} {}", clap_i18n::options_placeholder())
}

/// Build a clap Command tree from a CommandNode tree. `path` is the command
/// line that reaches it, e.g. `kobana banking bank-billets`.
pub fn build_command_tree(node: &CommandNode, name: &str, path: &str) -> Command {
    let mut cmd = clap_i18n::command(name.to_string(), node_usage(path))
        .subcommand_required(true)
        .arg_required_else_help(true);

    // Add child subcommands (resource groups)
    for (child_name, child_node) in &node.children {
        let child_cmd = build_command_tree(child_node, child_name, &format!("{path} {child_name}"));
        cmd = cmd.subcommand(child_cmd);
    }

    // Add method subcommands (endpoints)
    for endpoint in &node.endpoints {
        let method_cmd = build_method_command(endpoint, path);
        cmd = cmd.subcommand(method_cmd);
    }

    cmd
}

/// Build a clap Command for a single endpoint method
fn build_method_command(endpoint: &ResolvedEndpoint, parent_path: &str) -> Command {
    let mut cmd = clap_i18n::command(
        endpoint.cli_method.clone(),
        leaf_usage(&format!("{parent_path} {}", endpoint.cli_method)),
    );

    // Set help/about from operation summary
    if let Some(summary) = &endpoint.operation.summary {
        cmd = cmd.about(summary.clone());
    }

    // Add long help from description
    if let Some(desc) = &endpoint.operation.description {
        cmd = cmd.long_about(desc.clone());
    }

    cmd
}

/// Build the command tree for a product: resources and methods.
/// API versions are not CLI segments — `banking bank-billets` and
/// `banking charge pix` come from the v1 and v2 specs alike.
fn build_product_command(loaded: &LoadedProduct) -> Command {
    let path = format!("kobana {}", loaded.product.slug);
    let mut cmd = clap_i18n::command(loaded.product.slug, node_usage(&path))
        .about(loaded.product.about())
        .subcommand_required(true)
        .arg_required_else_help(true);

    for (name, node) in &loaded.tree.children {
        let mut child = build_command_tree(node, name, &format!("{path} {name}"));
        if let Some(about) = product::resource_about(loaded.product.slug, name) {
            child = child.about(about);
        }
        cmd = cmd.subcommand(child);
    }

    for endpoint in &loaded.tree.endpoints {
        cmd = cmd.subcommand(build_method_command(endpoint, &path));
    }

    cmd
}

/// Build the top-level kobana Command with all services
pub fn build_root_command(products: &[LoadedProduct]) -> Command {
    let root = clap_i18n::command("kobana", node_usage("kobana"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(tr!(
            pt: "Kobana API CLI — kobana <produto> <recurso> <metodo>",
            en: "Kobana API CLI — kobana <product> <resource> <method>",
            es: "Kobana API CLI — kobana <producto> <recurso> <metodo>",
        ))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("params")
                .long("params")
                .global(true)
                .help(tr!(
                    pt: "Parâmetros de URL e de consulta, em JSON",
                    en: "Query/URL parameters as JSON",
                    es: "Parámetros de URL y de consulta, en JSON",
                ))
                .value_name("JSON"),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .global(true)
                .help(tr!(
                    pt: "Corpo da requisição, em JSON",
                    en: "Request body as JSON",
                    es: "Cuerpo de la solicitud, en JSON",
                ))
                .value_name("JSON"),
        )
        .arg(
            Arg::new("fields")
                .long("fields")
                .global(true)
                .help(tr!(
                    pt: "Campos a incluir na resposta, separados por vírgula",
                    en: "Comma-separated list of fields to include in response",
                    es: "Campos a incluir en la respuesta, separados por comas",
                ))
                .value_name(tr!(pt: "CAMPOS", en: "FIELDS", es: "CAMPOS")),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help(tr!(
                    pt: "Mostra a requisição sem executar",
                    en: "Show the request without executing",
                    es: "Muestra la solicitud sin ejecutarla",
                )),
        )
        .arg(
            Arg::new("env")
                .long("env")
                .global(true)
                .value_name(tr!(pt: "AMBIENTE", en: "ENV", es: "ENTORNO"))
                .value_parser(clap::builder::PossibleValuesParser::new([
                    "production",
                    "sandbox",
                    "development",
                ]))
                .hide_possible_values(true)
                .help(tr!(
                    pt: "Ambiente da API: production, sandbox ou development (cada produto \
                         tem os seus hosts). Também pode vir de KOBANA_ENVIRONMENT.",
                    en: "API environment: production, sandbox or development (each product \
                         has its own hosts). Can also be set via KOBANA_ENVIRONMENT.",
                    es: "Entorno de la API: production, sandbox o development (cada producto \
                         tiene sus hosts). También puede venir de KOBANA_ENVIRONMENT.",
                )),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help(tr!(
                    pt: "Mostra os detalhes da requisição e da resposta no stderr",
                    en: "Show request/response details on stderr",
                    es: "Muestra los detalles de la solicitud y la respuesta en stderr",
                )),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .global(true)
                .help(tr!(
                    pt: "Salva a resposta em um arquivo",
                    en: "Save response to file",
                    es: "Guarda la respuesta en un archivo",
                ))
                .value_name(tr!(pt: "CAMINHO", en: "PATH", es: "RUTA")),
        )
        .arg(
            Arg::new("output-format")
                .long("output-format")
                .global(true)
                .help(tr!(
                    pt: "Formato da saída: json (padrão), table, csv",
                    en: "Output format: json (default), table, csv",
                    es: "Formato de salida: json (predeterminado), table, csv",
                ))
                .value_name(tr!(pt: "FORMATO", en: "FORMAT", es: "FORMATO"))
                .default_value("json")
                .hide_default_value(true),
        )
        .arg(
            Arg::new("idempotency-key")
                .long("idempotency-key")
                .global(true)
                .help(tr!(
                    pt: "Chave de idempotência própria para alterações",
                    en: "Custom idempotency key for mutations",
                    es: "Clave de idempotencia propia para modificaciones",
                ))
                .value_name(tr!(pt: "CHAVE", en: "KEY", es: "CLAVE")),
        )
        .arg(
            Arg::new("page-all")
                .long("page-all")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help(tr!(
                    pt: "Percorre todas as páginas e imprime NDJSON",
                    en: "Auto-paginate and output NDJSON",
                    es: "Recorre todas las páginas e imprime NDJSON",
                )),
        )
        .arg(
            Arg::new("page-limit")
                .long("page-limit")
                .global(true)
                .help(tr!(
                    pt: "Máximo de páginas a buscar (padrão: 10)",
                    en: "Maximum pages to fetch (default: 10)",
                    es: "Máximo de páginas a obtener (predeterminado: 10)",
                ))
                .value_name("N")
                .default_value("10")
                .hide_default_value(true),
        )
        .arg(
            Arg::new("page-delay")
                .long("page-delay")
                .global(true)
                .help(tr!(
                    pt: "Intervalo entre páginas em ms (padrão: 100)",
                    en: "Delay between pages in ms (default: 100)",
                    es: "Intervalo entre páginas en ms (predeterminado: 100)",
                ))
                .value_name("MS")
                .default_value("100")
                .hide_default_value(true),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .global(true)
                .value_name(tr!(pt: "IDIOMA", en: "LANG", es: "IDIOMA"))
                .help(tr!(
                    pt: "Idioma das mensagens: pt, en ou es (padrão: KOBANA_LANG ou o do sistema)",
                    en: "Language of the messages: pt, en or es (default: KOBANA_LANG or the system's)",
                    es: "Idioma de los mensajes: pt, en o es (predeterminado: KOBANA_LANG o el del sistema)",
                )),
        );
    let mut root = clap_i18n::root(root);

    // Add the product level: kobana <produto> <servico> <recurso> <metodo>
    for loaded in products {
        root = root.subcommand(build_product_command(loaded));
    }

    // Add special commands
    root = root.subcommand(
        clap_i18n::command(
            "schema",
            format!(
                "kobana schema {} [{}]",
                clap_i18n::options_placeholder(),
                tr!(pt: "ENDPOINT", en: "ENDPOINT", es: "ENDPOINT")
            ),
        )
        .about(tr!(
            pt: "Mostra o schema da API de um endpoint",
            en: "Introspect API schema for an endpoint",
            es: "Muestra el schema de la API de un endpoint",
        ))
        .arg(
            Arg::new("endpoint")
                .help_heading(clap_i18n::arguments_heading())
                .help(tr!(
                    pt: "Caminho do endpoint (ex.: banking.charge.pix.create, inbox.emails.list)",
                    en: "Endpoint path (e.g., banking.charge.pix.create, inbox.emails.list)",
                    es: "Ruta del endpoint (ej.: banking.charge.pix.create, inbox.emails.list)",
                ))
                .value_name("ENDPOINT"),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .action(clap::ArgAction::SetTrue)
                .help(tr!(
                    pt: "Lista os produtos e recursos disponíveis",
                    en: "List available products and resources",
                    es: "Lista los productos y recursos disponibles",
                )),
        ),
    );

    // Add helper commands
    for helper in helpers::all_helpers() {
        root = root.subcommand(helper.command());
    }

    // Add completions command
    root = root.subcommand(
        clap_i18n::command("completions", format!("kobana completions <{}>", tr!(pt: "SHELL", en: "SHELL", es: "SHELL")))
            .about(tr!(
                pt: "Gera o autocompletar para o shell",
                en: "Generate shell completions",
                es: "Genera el autocompletado para el shell",
            ))
            .arg(
                Arg::new("shell")
                    .help_heading(clap_i18n::arguments_heading())
                    .help(tr!(
                        pt: "Shell para o qual gerar o autocompletar (bash, zsh, fish, powershell, elvish)",
                        en: "Shell to generate completions for (bash, zsh, fish, powershell, elvish)",
                        es: "Shell para el que generar el autocompletado (bash, zsh, fish, powershell, elvish)",
                    ))
                    .required(true)
                    .value_name("SHELL"),
            ),
    );

    root = root.subcommand(
        clap_i18n::command("auth", node_usage("kobana auth"))
            .about(tr!(
                pt: "Gerencia a autenticação",
                en: "Authentication management",
                es: "Gestiona la autenticación",
            ))
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                clap_i18n::command("login", leaf_usage("kobana auth login"))
                    .about(tr!(
                        pt: "Entra na Kobana (OAuth + PKCE)",
                        en: "Login to Kobana (OAuth + PKCE)",
                        es: "Inicia sesión en Kobana (OAuth + PKCE)",
                    ))
                    .arg(
                        Arg::new("client-id")
                            .long("client-id")
                            .help(tr!(
                                pt: "Client ID OAuth (padrão: kobana-cli)",
                                en: "OAuth client ID (default: kobana-cli)",
                                es: "Client ID OAuth (predeterminado: kobana-cli)",
                            ))
                            .value_name("ID"),
                    )
                    .arg(
                        Arg::new("client-secret")
                            .long("client-secret")
                            .help(tr!(
                                pt: "Client secret OAuth (só para o fluxo Client Credentials)",
                                en: "OAuth client secret (only for Client Credentials flow)",
                                es: "Client secret OAuth (solo para el flujo Client Credentials)",
                            ))
                            .value_name("SECRET"),
                    )
                    .arg(
                        Arg::new("scopes")
                            .long("scopes")
                            .help(tr!(
                                pt: "Escopos OAuth, separados por vírgula (padrão: todos)",
                                en: "OAuth scopes (comma-separated, default: all)",
                                es: "Alcances OAuth, separados por comas (predeterminado: todos)",
                            ))
                            .value_name(tr!(pt: "ESCOPOS", en: "SCOPES", es: "ALCANCES")),
                    ),
            )
            .subcommand(clap_i18n::command("logout", leaf_usage("kobana auth logout")).about(tr!(
                pt: "Remove as credenciais salvas",
                en: "Remove saved credentials",
                es: "Elimina las credenciales guardadas",
            )))
            .subcommand(clap_i18n::command("status", leaf_usage("kobana auth status")).about(tr!(
                pt: "Mostra o estado da autenticação",
                en: "Show authentication status",
                es: "Muestra el estado de la autenticación",
            )))
            .subcommand(clap_i18n::command("export", leaf_usage("kobana auth export")).about(tr!(
                pt: "Exporta as credenciais em JSON",
                en: "Export credentials as JSON",
                es: "Exporta las credenciales en JSON",
            ))),
    );

    // `kobana update` — check for new releases and self-update standalone installs
    root = root.subcommand(
        clap_i18n::command("update", leaf_usage("kobana update"))
            .about(tr!(
                pt: "Procura uma nova versão e atualiza o CLI",
                en: "Check for a new version and update the CLI",
                es: "Busca una nueva versión y actualiza el CLI",
            ))
            .arg(
                Arg::new("check")
                    .long("check")
                    .action(clap::ArgAction::SetTrue)
                    .help(tr!(
                        pt: "Só verifica se há atualização, sem instalar",
                        en: "Only check for updates, do not install",
                        es: "Solo verifica si hay actualización, sin instalar",
                    )),
            )
            .arg(
                Arg::new("json")
                    .long("json")
                    .action(clap::ArgAction::SetTrue)
                    .help(tr!(
                        pt: "Imprime a saída em JSON em vez de uma mensagem",
                        en: "Print output as JSON instead of a human-readable message",
                        es: "Imprime la salida en JSON en lugar de un mensaje",
                    )),
            ),
    );

    root.subcommand(clap_i18n::help_subcommand())
}

/// Resolve which endpoint was matched from the clap matches
pub fn resolve_endpoint<'a>(
    matches: &clap::ArgMatches,
    products: &'a [LoadedProduct],
) -> Option<(&'a LoadedProduct, &'a ResolvedEndpoint, clap::ArgMatches)> {
    let (product_name, product_matches) = matches.subcommand()?;

    // Special commands (schema, auth, update, completions, help, helpers) are not products
    let loaded = product::find(products, product_name)?;

    let (endpoint, method_matches) = resolve_in_tree(&loaded.tree, product_matches)?;
    Some((loaded, endpoint, method_matches))
}

fn resolve_in_tree<'a>(
    node: &'a CommandNode,
    matches: &clap::ArgMatches,
) -> Option<(&'a ResolvedEndpoint, clap::ArgMatches)> {
    if let Some((sub_name, sub_matches)) = matches.subcommand() {
        // Check if it's a child node (resource group)
        if let Some(child) = node.children.get(sub_name) {
            return resolve_in_tree(child, sub_matches);
        }

        // Check if it's an endpoint method
        for endpoint in &node.endpoints {
            if endpoint.cli_method == sub_name {
                return Some((endpoint, sub_matches.clone()));
            }
        }
    }

    None
}
