use clap::{Arg, Command};
use kobana::spec::{CommandNode, ResolvedEndpoint};

use crate::helpers;
use crate::product::{self, LoadedProduct};

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

/// Build the command tree for a product: resources and methods.
/// API versions are not CLI segments — `banking bank-billets` and
/// `banking charge pix` come from the v1 and v2 specs alike.
fn build_product_command(loaded: &LoadedProduct) -> Command {
    let mut cmd = Command::new(loaded.product.slug)
        .about(loaded.product.about)
        .subcommand_required(true)
        .arg_required_else_help(true);

    for (name, node) in &loaded.tree.children {
        let mut child = build_command_tree(node, name);
        if let Some(about) = product::resource_about(name) {
            child = child.about(about);
        }
        cmd = cmd.subcommand(child);
    }

    for endpoint in &loaded.tree.endpoints {
        cmd = cmd.subcommand(build_method_command(endpoint));
    }

    cmd
}

/// Build the top-level kobana Command with all services
pub fn build_root_command(products: &[LoadedProduct]) -> Command {
    let mut root = Command::new("kobana")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Kobana API CLI — kobana <produto> <recurso> <metodo>")
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
            Arg::new("env")
                .long("env")
                .global(true)
                .value_name("ENV")
                .value_parser(clap::builder::PossibleValuesParser::new([
                    "production",
                    "sandbox",
                    "development",
                ]))
                .help(
                    "API environment: production (default), sandbox, or development (localhost:5005). \
                     Can also be set via KOBANA_ENVIRONMENT.",
                ),
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

    // Add the product level: kobana <produto> <servico> <recurso> <metodo>
    for loaded in products {
        root = root.subcommand(build_product_command(loaded));
    }

    // Add special commands
    root = root.subcommand(
        Command::new("schema")
            .about("Introspect API schema for an endpoint")
            .arg(
                Arg::new("endpoint")
                    .help("Endpoint path (e.g., banking.charge.pix.create, inbox.emails.list)")
                    .value_name("ENDPOINT"),
            )
            .arg(
                Arg::new("list")
                    .long("list")
                    .action(clap::ArgAction::SetTrue)
                    .help("List available products and resources"),
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
                    ),
            )
            .subcommand(Command::new("logout").about("Remove saved credentials"))
            .subcommand(Command::new("status").about("Show authentication status"))
            .subcommand(Command::new("export").about("Export credentials as JSON")),
    );

    // `kobana update` — check for new releases and self-update standalone installs
    root.subcommand(
        Command::new("update")
            .about("Check for a new version and update the CLI")
            .arg(
                Arg::new("check")
                    .long("check")
                    .action(clap::ArgAction::SetTrue)
                    .help("Only check for updates, do not install"),
            )
            .arg(
                Arg::new("json")
                    .long("json")
                    .action(clap::ArgAction::SetTrue)
                    .help("Print output as JSON instead of a human-readable message"),
            ),
    )
}

/// Resolve which endpoint was matched from the clap matches
pub fn resolve_endpoint<'a>(
    matches: &clap::ArgMatches,
    products: &'a [LoadedProduct],
) -> Option<(&'a LoadedProduct, &'a ResolvedEndpoint, clap::ArgMatches)> {
    let (product_name, product_matches) = matches.subcommand()?;

    // Special commands (schema, auth, update, completions, helpers) are not products
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
