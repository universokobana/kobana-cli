use clap::{Arg, Command};
use kobana::spec::{CommandNode, ResolvedEndpoint};
use kobana::tr;

use crate::art;
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

/// Usage line of an endpoint: its path ids, then the options
fn endpoint_usage(path: &str, endpoint: &ResolvedEndpoint, subcommands: bool) -> String {
    let ids: String = endpoint
        .path_params
        .iter()
        .map(|p| format!(" [{}]", p.to_uppercase()))
        .collect();
    let command = if subcommands {
        format!(" {}", clap_i18n::command_placeholder().replace('<', "[").replace('>', "]"))
    } else {
        String::new()
    };
    format!("{path}{ids} {}{command}", clap_i18n::options_placeholder())
}

/// Build a clap Command tree from a CommandNode tree. `path` is the command
/// line that reaches it, e.g. `kobana banking bank-billets`.
pub fn build_command_tree(node: &CommandNode, name: &str, path: &str) -> Command {
    // A node that runs by its own name (`kobana banking me`) takes the
    // endpoint's arguments; its method stays as a hidden subcommand
    let default = node.default_endpoint(name);
    let mut cmd = match default {
        Some(endpoint) => {
            // Hidden subcommands (its own method) do not count
            let subcommands = !node.children.is_empty() || node.endpoints.len() > 1;
            let mut cmd = clap_i18n::command(name.to_string(), endpoint_usage(path, endpoint, subcommands));
            if let Some(summary) = &endpoint.operation.summary {
                cmd = cmd.about(summary.clone());
            }
            if let Some(desc) = &endpoint.operation.description {
                cmd = cmd.long_about(desc.clone());
            }
            if endpoint.operation.deprecated {
                cmd = cmd.hide(true);
            }
            with_path_ids(cmd, endpoint)
        }
        None => clap_i18n::command(name.to_string(), node_usage(path))
            .subcommand_required(true)
            .arg_required_else_help(true),
    }
    .aliases(node.aliases.clone());

    // Add child subcommands (resource groups)
    for (child_name, child_node) in &node.children {
        let child_cmd = build_command_tree(child_node, child_name, &format!("{path} {child_name}"));
        cmd = cmd.subcommand(child_cmd);
    }

    // Add method subcommands (endpoints)
    for endpoint in &node.endpoints {
        let mut method_cmd = build_method_command(endpoint, path);
        if default.is_some_and(|d| std::ptr::eq(d, endpoint)) {
            method_cmd = method_cmd.hide(true);
        }
        cmd = cmd.subcommand(method_cmd);
    }

    cmd
}

/// The ids of the endpoint's path as positional arguments, in the order of the
/// path: `kobana banking bank-billets get 123` instead of `--params '{"id":123}'`.
/// `--params` still works and is read first.
fn with_path_ids(cmd: Command, endpoint: &ResolvedEndpoint) -> Command {
    if endpoint.path_params.is_empty() {
        return cmd;
    }
    let names: Vec<String> = endpoint.path_params.iter().map(|p| p.to_uppercase()).collect();
    cmd.arg(
        Arg::new(PATH_IDS)
            .num_args(0..=names.len())
            .value_names(names)
            .help_heading(clap_i18n::arguments_heading())
            .help(tr!(
                pt: "Identificadores do caminho, na ordem; também podem vir em --params",
                en: "Path identifiers, in order; they can also come in --params",
                es: "Identificadores de la ruta, en orden; también pueden venir en --params",
            )),
    )
}

/// Id of the positional arguments that carry an endpoint's path ids
pub const PATH_IDS: &str = "path-ids";

/// The path ids given as positional arguments, wherever they sit in the
/// command line (`get 1` or, for a node that runs itself, `me 1`)
pub fn path_ids(matches: &clap::ArgMatches) -> Vec<String> {
    let mut ids = Vec::new();
    let mut current = Some(matches);
    while let Some(m) = current {
        if let Ok(Some(values)) = m.try_get_many::<String>(PATH_IDS) {
            ids.extend(values.cloned());
        }
        current = m.subcommand().map(|(_, sub)| sub);
    }
    ids
}

/// Build a clap Command for a single endpoint method
fn build_method_command(endpoint: &ResolvedEndpoint, parent_path: &str) -> Command {
    let mut cmd = clap_i18n::command(
        endpoint.cli_method.clone(),
        endpoint_usage(&format!("{parent_path} {}", endpoint.cli_method), endpoint, false),
    )
    .aliases(endpoint.aliases().iter().copied());
    cmd = with_path_ids(cmd, endpoint);

    // Set help/about from operation summary
    if let Some(summary) = &endpoint.operation.summary {
        cmd = cmd.about(summary.clone());
    }

    // Add long help from description
    if let Some(desc) = &endpoint.operation.description {
        cmd = cmd.long_about(desc.clone());
    }

    // A deprecated operation keeps running for the scripts that call it, but
    // leaves the help: the spec's description names what replaced it
    if endpoint.operation.deprecated {
        cmd = cmd.hide(true);
    }

    // The line it had before becoming an action of its parent keeps working
    // (`… recipients remove create`), out of the help
    if endpoint.legacy_create {
        cmd = cmd.subcommand(
            clap_i18n::command(
                "create".to_string(),
                leaf_usage(&format!("{parent_path} {} create", endpoint.cli_method)),
            )
            .hide(true),
        );
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
        let mut child = build_command_tree(node, name, &format!("{path} {name}"))
            .aliases(product::resource_aliases(loaded.product.slug, name).iter().copied());
        if let Some(about) = product::resource_about(loaded.product.slug, name) {
            child = child.about(about);
        }
        cmd = cmd.subcommand(child);
    }

    for endpoint in &loaded.tree.endpoints {
        cmd = cmd.subcommand(build_method_command(endpoint, &path));
    }

    // Helpers (`kobana banking +emitir`, …)
    for helper in helpers::for_product(loaded.product.slug) {
        cmd = cmd.subcommand(helper.command());
    }

    // `kobana kadu "Boleto pago!"` is no node of the tree: see `art::speech_in`
    if loaded.product.slug == art::SPEAKER {
        cmd = cmd.after_help(tr!(
            pt: "O Kadu também fala num balão: kobana kadu \"Boleto pago!\"",
            en: "Kadu also speaks in a bubble: kobana kadu \"Boleto pago!\"",
            es: "Kadu también habla en un globo: kobana kadu \"Boleto pago!\"",
        ));
    }

    cmd.subcommand(auth_command(&format!("{path} auth")))
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
            Arg::new("file")
                .long("file")
                .global(true)
                .action(clap::ArgAction::Append)
                .help(tr!(
                    pt: "Arquivo enviado como parte de um formulário multipart, campo=caminho (repetível; os campos de --json viram partes de texto)",
                    en: "File sent as a part of a multipart form, field=path (repeatable; --json fields become text parts)",
                    es: "Archivo enviado como parte de un formulario multipart, campo=ruta (repetible; los campos de --json se vuelven partes de texto)",
                ))
                .value_name(tr!(pt: "CAMPO=CAMINHO", en: "FIELD=PATH", es: "CAMPO=RUTA")),
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
                // Checked by clap, before any request: a POST with a bad
                // format used to create the record and then fail
                .value_parser(clap::builder::PossibleValuesParser::new(["json", "table", "csv"]))
                .hide_possible_values(true)
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
            Arg::new("jq")
                .long("jq")
                .global(true)
                .value_name(tr!(pt: "EXPRESSÃO", en: "EXPRESSION", es: "EXPRESIÓN"))
                .help(tr!(
                    pt: "Filtra a saída JSON com uma expressão jq, ex.: '.data[].id' ou, com --page-all (um item por vez), '.id'; não precisa do jq instalado",
                    en: "Filter the JSON output with a jq expression, e.g. '.data[].id' or, with --page-all (one item at a time), '.id'; no jq install needed",
                    es: "Filtra la salida JSON con una expresión jq, ej.: '.data[].id' o, con --page-all (un elemento a la vez), '.id'; no necesita jq instalado",
                )),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help(tr!(
                    pt: "Desliga as cores (também: NO_COLOR ou CLICOLOR=0); veja `kobana accessibility`",
                    en: "Turn colors off (also: NO_COLOR or CLICOLOR=0); see `kobana accessibility`",
                    es: "Desactiva los colores (también: NO_COLOR o CLICOLOR=0); vea `kobana accessibility`",
                )),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .global(true)
                .value_name(tr!(pt: "IDIOMA", en: "LANG", es: "IDIOMA"))
                // `--lang fr` used to fall back to English without a word
                .value_parser(clap::builder::PossibleValuesParser::new(["pt", "en", "es"]))
                .hide_possible_values(true)
                .help(tr!(
                    pt: "Idioma das mensagens: pt, en ou es (padrão: KOBANA_LANG ou o do sistema)",
                    en: "Language of the messages: pt, en or es (default: KOBANA_LANG or the system's)",
                    es: "Idioma de los mensajes: pt, en o es (predeterminado: KOBANA_LANG o el del sistema)",
                )),
        );
    let mut root = clap_i18n::root(root.after_help(clap_i18n::learn_more()));

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
                // Shadows the global `--json <JSON>` (same id and long name,
                // which clap requires); read it with `update_json_flag`
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

    // `kobana accessibility` (`a11y`), like `gh accessibility`
    root = root.subcommand(
        clap_i18n::command("accessibility", leaf_usage("kobana accessibility"))
            // clap's "[aliases: …]" is English-only: the about names it instead
            .alias("a11y")
            .about(tr!(
                pt: "Mostra as opções de acessibilidade: cores, leitor de tela, perguntas e scripts (também: a11y)",
                en: "Show the accessibility options: colors, screen reader, prompts and scripts (also: a11y)",
                es: "Muestra las opciones de accesibilidad: colores, lector de pantalla, preguntas y scripts (también: a11y)",
            )),
    );

    // `kobana art` and `oi`: Kadu, the logo and the stork (art.rs). `kobana
    // kadu` is the Agente Kadu product, so they live outside it.
    root = root.subcommand(
        clap_i18n::command(
            "art",
            format!(
                "kobana art {} [{}]",
                clap_i18n::options_placeholder(),
                clap_i18n::command_placeholder().trim_matches(['<', '>'])
            ),
        )
        .about(tr!(
            pt: "Mostra a arte da Kobana no terminal; sem comando, a cegonha",
            en: "Show the Kobana art in the terminal; with no command, the stork",
            es: "Muestra el arte de Kobana en el terminal; sin comando, la cigüeña",
        ))
        .subcommand(clap_i18n::command("kadu", leaf_usage("kobana art kadu")).about(tr!(
            pt: "O Kadu, fiel à foto (55 × 44)",
            en: "Kadu, true to the photo (55 × 44)",
            es: "Kadu, fiel a la foto (55 × 44)",
        )))
        .subcommand(clap_i18n::command("anim", leaf_usage("kobana art anim")).about(tr!(
            pt: "O Kadu balança, sorri e pisca, em cerca de 6 s (48 × 37)",
            en: "Kadu sways, smiles and blinks, for about 6 s (48 × 37)",
            es: "Kadu se balancea, sonríe y parpadea, durante unos 6 s (48 × 37)",
        )))
        .subcommand(clap_i18n::command("logo", leaf_usage("kobana art logo")).about(tr!(
            pt: "Logo da Kobana (64 × 6)",
            en: "The Kobana logo (64 × 6)",
            es: "Logo de Kobana (64 × 6)",
        )))
        .subcommand(clap_i18n::command("cegonha", leaf_usage("kobana art cegonha")).about(tr!(
            pt: "A cegonha da Kobana (96 × 32)",
            en: "The Kobana stork (96 × 32)",
            es: "La cigüeña de Kobana (96 × 32)",
        ))),
    );
    root = root.subcommand(
        clap_i18n::command("oi", leaf_usage("kobana oi"))
            .aliases(["hi", "hola"])
            .about(tr!(
                pt: "O Kadu dá oi (também: hi, hola)",
                en: "Kadu says hi (also: hi, hola)",
                es: "Kadu saluda (también: hi, hola)",
            )),
    );

    root.subcommand(clap_i18n::help_subcommand())
}

/// `kobana update --json`. Its flag shares the id of the global
/// `--json <JSON>`, so `kobana --json '{}' update` hands it a string, which
/// `get_flag` would panic on.
pub fn update_json_flag(update_matches: &clap::ArgMatches) -> Result<bool, kobana::error::KobanaError> {
    match update_matches.try_get_one::<bool>("json") {
        Ok(flag) => Ok(flag.copied().unwrap_or(false)),
        Err(_) => Err(kobana::error::KobanaError::Validation(
            tr!(
                pt: "`update` não recebe --json <JSON>; para a saída em JSON use `kobana update --json`",
                en: "`update` takes no --json <JSON>; for JSON output use `kobana update --json`",
                es: "`update` no recibe --json <JSON>; para la salida en JSON use `kobana update --json`",
            )
            .to_string(),
        )),
    }
}

/// Resolve which endpoint was matched from the clap matches
pub fn resolve_endpoint<'a>(
    matches: &clap::ArgMatches,
    products: &'a [LoadedProduct],
) -> Option<(&'a LoadedProduct, &'a ResolvedEndpoint)> {
    let (product_name, product_matches) = matches.subcommand()?;

    // Special commands (schema, auth, update, completions, help, helpers) are not products
    let loaded = product::find(products, product_name)?;

    let endpoint = resolve_in_tree(&loaded.tree, product_matches)?;
    Some((loaded, endpoint))
}

fn resolve_in_tree<'a>(node: &'a CommandNode, matches: &clap::ArgMatches) -> Option<&'a ResolvedEndpoint> {
    resolve_node(node, "", matches)
}

/// `name` is the node's name in its parent. A node reached with no method
/// after it runs its default endpoint (`kobana banking me`).
fn resolve_node<'a>(node: &'a CommandNode, name: &str, matches: &clap::ArgMatches) -> Option<&'a ResolvedEndpoint> {
    let Some((sub_name, sub_matches)) = matches.subcommand() else {
        return node.default_endpoint(name);
    };

    // A child node (resource group); clap reports an aliased one by its name
    if let Some(child) = node.children.get(sub_name) {
        return resolve_node(child, sub_name, sub_matches);
    }

    // An endpoint method; its hidden `<action> create` (the line before it
    // became an action) runs the same endpoint
    node.endpoints.iter().find(|endpoint| endpoint.cli_method == sub_name)
}

/// `<path> login|logout|status|export` — the product's credentials. `path`
/// is the command line that reaches it, e.g. `kobana billing auth`.
pub fn auth_command(path: &str) -> Command {
    clap_i18n::command("auth", node_usage(path))
        .about(tr!(
            pt: "Entra, sai e mostra as credenciais deste produto",
            en: "Log in, log out and show this product's credentials",
            es: "Inicia sesión, cierra sesión y muestra las credenciales de este producto",
        ))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap_i18n::command("login", leaf_usage(&format!("{path} login")))
                .about(tr!(
                    pt: "Entra pela Conta Kobana (OAuth + PKCE)",
                    en: "Log in through the Conta Kobana (OAuth + PKCE)",
                    es: "Inicia sesión con la Conta Kobana (OAuth + PKCE)",
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
                            pt: "Client secret OAuth: usa o fluxo Client Credentials, só no banking e com --client-id (também: KOBANA_CLIENT_SECRET)",
                            en: "OAuth client secret: uses the Client Credentials flow, banking only, with --client-id (also: KOBANA_CLIENT_SECRET)",
                            es: "Client secret OAuth: usa el flujo Client Credentials, solo en banking y con --client-id (también: KOBANA_CLIENT_SECRET)",
                        ))
                        .value_name("SECRET"),
                )
                .arg(
                    Arg::new("scopes")
                        .long("scopes")
                        .help(tr!(
                            pt: "Escopos OAuth separados por vírgula, ex.: billing/customers,write (padrão: os publicados pelo produto)",
                            en: "Comma-separated OAuth scopes, e.g. billing/customers,write (default: the ones the product publishes)",
                            es: "Alcances OAuth separados por comas, p. ej. billing/customers,write (predeterminado: los que publica el producto)",
                        ))
                        .value_name(tr!(pt: "ESCOPOS", en: "SCOPES", es: "ALCANCES")),
                ),
        )
        .subcommand(clap_i18n::command("logout", leaf_usage(&format!("{path} logout"))).about(tr!(
            pt: "Remove as credenciais salvas",
            en: "Remove saved credentials",
            es: "Elimina las credenciales guardadas",
        )))
        .subcommand(clap_i18n::command("status", leaf_usage(&format!("{path} status"))).about(tr!(
            pt: "Mostra o estado da autenticação",
            en: "Show the authentication status",
            es: "Muestra el estado de la autenticación",
        )))
        .subcommand(clap_i18n::command("export", leaf_usage(&format!("{path} export"))).about(tr!(
            pt: "Exporta as credenciais em JSON",
            en: "Export the credentials as JSON",
            es: "Exporta las credenciales en JSON",
        )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_takes_only_the_three_languages() {
        let products = product::load_all().unwrap();
        let parse = |lang: &str| build_root_command(&products).try_get_matches_from(["kobana", "--lang", lang, "schema"]);
        for ok in ["pt", "en", "es"] {
            assert!(parse(ok).is_ok(), "{ok}");
        }
        assert_eq!(parse("fr").unwrap_err().kind(), clap::error::ErrorKind::InvalidValue);
    }

    #[test]
    fn the_art_commands_answer_to_their_aliases_and_stay_apart_from_the_kadu_product() {
        let products = product::load_all().unwrap();
        let command = |args: &[&str]| {
            let matches = build_root_command(&products).try_get_matches_from(args).unwrap();
            matches.subcommand_name().unwrap().to_string()
        };
        assert_eq!(command(&["kobana", "art"]), "art");
        assert_eq!(command(&["kobana", "art", "anim"]), "art");
        assert_eq!(command(&["kobana", "hi"]), "oi");
        assert_eq!(command(&["kobana", "hola"]), "oi");
        assert_eq!(command(&["kobana", "kadu", "tasks", "list"]), "kadu");
    }

    #[test]
    fn output_format_is_checked_before_anything_runs() {
        let products = product::load_all().unwrap();
        let parse = |format: &str| {
            build_root_command(&products).try_get_matches_from([
                "kobana", "banking", "bank-billets", "create", "--output-format", format,
            ])
        };
        for ok in ["json", "table", "csv"] {
            assert!(parse(ok).is_ok(), "{ok}");
        }
        assert_eq!(parse("xml").unwrap_err().kind(), clap::error::ErrorKind::InvalidValue);
    }

    #[test]
    fn update_json_flag_does_not_clash_with_the_global_json() {
        let products = product::load_all().unwrap();
        let update = |args: &[&str]| {
            let matches = build_root_command(&products).try_get_matches_from(args).unwrap();
            update_json_flag(matches.subcommand().unwrap().1)
        };
        assert!(!update(&["kobana", "update"]).unwrap());
        assert!(update(&["kobana", "update", "--json"]).unwrap());
        // The global `--json <JSON>` reaches `update` as a string: an error, not a panic
        assert!(matches!(
            update(&["kobana", "--json", "{}", "update"]),
            Err(kobana::error::KobanaError::Validation(_))
        ));
    }

    #[test]
    fn helpers_live_under_their_product_only() {
        let products = product::load_all().unwrap();
        let root = build_root_command(&products);
        for helper in helpers::all_helpers() {
            let name = helper.name();
            assert!(root.find_subcommand(name).is_none(), "{name} at the top level");
            for cmd in root.get_subcommands() {
                let found = cmd.find_subcommand(name).is_some();
                assert_eq!(found, cmd.get_name() == helper.product(), "{name} under {}", cmd.get_name());
            }
        }
        let banking = root.find_subcommand("banking").unwrap();
        for name in ["+emitir", "+cancelar-lote", "+cobrar"] {
            assert!(banking.find_subcommand(name).is_some(), "{name} missing under banking");
        }
    }

    /// The endpoint a command line reaches, as `METHOD path`
    fn reaches(args: &[&str]) -> String {
        let products = product::load_all().unwrap();
        let mut line = vec!["kobana"];
        line.extend(args);
        let matches = build_root_command(&products).try_get_matches_from(line).unwrap();
        let (_, endpoint) = resolve_endpoint(&matches, &products).expect("a command line that reaches an endpoint");
        format!("{} {}", endpoint.http_method, endpoint.path_template)
    }

    #[test]
    fn a_singleton_runs_by_its_name_and_by_its_old_one() {
        assert_eq!(reaches(&["banking", "me"]), "GET /v2/me");
        assert_eq!(reaches(&["banking", "me", "get"]), "GET /v2/me");
        assert_eq!(reaches(&["banking", "me", "list"]), "GET /v2/me");
        assert_eq!(reaches(&["billing", "organization"]), "GET /v1/organization");
        assert_eq!(reaches(&["billing", "organization", "update"]), "PATCH /v1/organization");
    }

    #[test]
    fn an_action_runs_without_the_verb_of_its_method() {
        assert_eq!(
            reaches(&["banking", "mailbox", "entries", "email", "activate"]),
            "PUT /v2/mailbox/entries/{entry_uid}/email/activate"
        );
        assert_eq!(
            reaches(&["banking", "mailbox", "entries", "email", "activate", "update"]),
            "PUT /v2/mailbox/entries/{entry_uid}/email/activate"
        );
        assert_eq!(reaches(&["banking", "userinfo"]), "GET /v1/userinfo");
        assert_eq!(reaches(&["banking", "customers", "cnpj-cpf", "list"]), "GET /v1/customers/cnpj_cpf");
    }

    #[test]
    fn a_node_never_changes_anything_by_its_name_alone() {
        let products = product::load_all().unwrap();
        // A noun with a write, a delete and a resource with its own methods
        for args in [
            ["dashboards", "builder", "draft"].as_slice(),
            ["banking", "bank-billets"].as_slice(),
            ["banking", "payment", "pix-batches"].as_slice(),
        ] {
            let mut line = vec!["kobana"];
            line.extend(args);
            let matches = build_root_command(&products).try_get_matches_from(line);
            assert!(
                matches.is_err() || resolve_endpoint(&matches.unwrap(), &products).is_none(),
                "{args:?} runs by its name"
            );
        }
    }

    #[test]
    fn path_ids_are_positional() {
        let products = product::load_all().unwrap();
        let ids = |args: &[&str]| {
            let mut line = vec!["kobana"];
            line.extend(args);
            path_ids(&build_root_command(&products).try_get_matches_from(line).unwrap())
        };
        assert_eq!(ids(&["banking", "bank-billets", "get", "123"]), ["123"]);
        assert_eq!(ids(&["banking", "charge", "pix", "commands", "get", "P1", "C1"]), ["P1", "C1"]);
        assert_eq!(ids(&["banking", "bank-billets", "get"]), Vec::<String>::new());
        assert_eq!(ids(&["banking", "boletos", "get", "9"]), ["9"]);
        // More ids than the path has
        assert!(build_root_command(&products)
            .try_get_matches_from(["kobana", "banking", "bank-billets", "get", "1", "2"])
            .is_err());
    }

    #[test]
    fn a_prefix_the_parent_repeats_is_dropped_and_still_answers() {
        assert_eq!(reaches(&["banking", "edi", "boxes", "list"]), "GET /v2/edi/edi_boxes");
        assert_eq!(reaches(&["banking", "edi", "edi-boxes", "list"]), "GET /v2/edi/edi_boxes");
    }
}
