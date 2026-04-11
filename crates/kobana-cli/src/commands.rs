use clap::{Arg, Command};
use kobana::spec::{CommandNode, ResolvedEndpoint};

use crate::helpers;

/// Build a clap Command tree from a CommandNode tree
pub fn build_command_tree(node: &CommandNode, name: &str) -> Command {
    let mut cmd = Command::new(name.to_string())
        .subcommand_required(true)
        .arg_required_else_help(true);

    // Add child subcommands (resource groups)
    for (child_name, child_node) in &node.children {
        let child_cmd = build_command_tree(child_node, child_name);
        cmd = cmd.subcommand(child_cmd);
    }

    // Add method subcommands (endpoints)
    for endpoint in &node.endpoints {
        let method_cmd = build_method_command(endpoint);
        cmd = cmd.subcommand(method_cmd);
    }

    cmd
}

/// Build a clap Command for a single endpoint method
fn build_method_command(endpoint: &ResolvedEndpoint) -> Command {
    let mut cmd = Command::new(endpoint.cli_method.clone());

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

/// Build the top-level kobana Command with all services
pub fn build_root_command(v1_tree: &CommandNode, v2_tree: &CommandNode) -> Command {
    let mut root = Command::new("kobana")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Kobana API CLI — acesso completo a API Kobana v1 e v2")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("params")
                .long("params")
                .global(true)
                .help("Query/URL parameters as JSON")
                .value_name("JSON"),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .global(true)
                .help("Request body as JSON")
                .value_name("JSON"),
        )
        .arg(
            Arg::new("fields")
                .long("fields")
                .global(true)
                .help("Comma-separated list of fields to include in response")
                .value_name("FIELDS"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help("Show the request without executing"),
        )
        .arg(
            Arg::new("sandbox")
                .long("sandbox")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help("Use sandbox environment"),
        )
        .arg(
            Arg::new("production")
                .long("production")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help("Use production environment"),
        )
        .arg(
            Arg::new("development")
                .long("development")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help("Use development environment (localhost:5005)"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help("Show request/response details on stderr"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .global(true)
                .help("Save response to file")
                .value_name("PATH"),
        )
        .arg(
            Arg::new("output-format")
                .long("output-format")
                .global(true)
                .help("Output format: json (default), table, csv")
                .value_name("FORMAT")
                .default_value("json"),
        )
        .arg(
            Arg::new("idempotency-key")
                .long("idempotency-key")
                .global(true)
                .help("Custom idempotency key for mutations")
                .value_name("KEY"),
        )
        .arg(
            Arg::new("page-all")
                .long("page-all")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help("Auto-paginate and output NDJSON"),
        )
        .arg(
            Arg::new("page-limit")
                .long("page-limit")
                .global(true)
                .help("Maximum pages to fetch (default: 10)")
                .value_name("N")
                .default_value("10"),
        )
        .arg(
            Arg::new("page-delay")
                .long("page-delay")
                .global(true)
                .help("Delay between pages in ms (default: 100)")
                .value_name("MS")
                .default_value("100"),
        );

    // Add v1 as a top-level subcommand
    let v1_cmd = build_command_tree(v1_tree, "v1").about("API v1 (boletos, clientes, webhooks)");
    root = root.subcommand(v1_cmd);

    // Add v2 services as top-level subcommands
    for (service_name, service_node) in &v2_tree.children {
        let service_cmd = build_command_tree(service_node, service_name)
            .about(service_about(service_name));
        root = root.subcommand(service_cmd);
    }

    // Add special commands
    root = root.subcommand(
        Command::new("schema")
            .about("Introspect API schema for an endpoint")
            .arg(
                Arg::new("endpoint")
                    .help("Endpoint path (e.g., charge.pix.create)")
                    .value_name("ENDPOINT"),
            )
            .arg(
                Arg::new("list")
                    .long("list")
                    .action(clap::ArgAction::SetTrue)
                    .help("List available services/resources"),
            ),
    );

    // Add helper commands
    for helper in helpers::all_helpers() {
        root = root.subcommand(helper.command());
    }

    // Add completions command
    root = root.subcommand(
        Command::new("completions")
            .about("Generate shell completions")
            .arg(
                Arg::new("shell")
                    .help("Shell to generate completions for (bash, zsh, fish, powershell, elvish)")
                    .required(true)
                    .value_name("SHELL"),
            ),
    );

    root = root.subcommand(
        Command::new("auth")
            .about("Authentication management")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("login")
                    .about("Login to Kobana (OAuth + PKCE)")
                    .arg(
                        Arg::new("client-id")
                            .long("client-id")
                            .help("OAuth client ID (default: kobana-cli)")
                            .value_name("ID"),
                    )
                    .arg(
                        Arg::new("client-secret")
                            .long("client-secret")
                            .help("OAuth client secret (only for Client Credentials flow)")
                            .value_name("SECRET"),
                    )
                    .arg(
                        Arg::new("scopes")
                            .long("scopes")
                            .help("OAuth scopes (comma-separated, default: all)")
                            .value_name("SCOPES"),
                    )
                    .arg(
                        Arg::new("production")
                            .long("production")
                            .action(clap::ArgAction::SetTrue)
                            .help("Use production environment"),
                    ),
            )
            .subcommand(Command::new("logout").about("Remove saved credentials"))
            .subcommand(Command::new("status").about("Show authentication status"))
            .subcommand(Command::new("export").about("Export credentials as JSON")),
    );

    root
}

/// Human-readable about text for v2 services
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

/// Resolve which endpoint was matched from the clap matches
pub fn resolve_endpoint<'a>(
    matches: &clap::ArgMatches,
    v1_tree: &'a CommandNode,
    v2_tree: &'a CommandNode,
) -> Option<(&'a ResolvedEndpoint, clap::ArgMatches)> {
    let (service_name, service_matches) = matches.subcommand()?;

    if service_name == "schema" || service_name == "auth" {
        return None; // handled separately
    }

    let tree = if service_name == "v1" {
        v1_tree
    } else {
        v2_tree.children.get(service_name)?
    };

    resolve_in_tree(tree, service_matches)
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
