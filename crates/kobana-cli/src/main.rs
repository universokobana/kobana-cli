mod auth;
mod auth_commands;
mod commands;
mod completions;
mod config;
mod credential_store;
mod executor;
mod formatter;
mod helpers;
mod logging;
mod oauth;
mod pagination;
mod schema;
mod update;
mod validate;

use kobana::error::KobanaError;
use kobana::spec::ApiSpec;

// Embed OpenAPI specs at compile time
const V1_SPEC_JSON: &str = include_str!("../specs/v1.json");
const V2_SPEC_JSON: &str = include_str!("../specs/v2.json");

#[tokio::main]
async fn main() {
    // Load .env files
    config::load_dotenv();

    // Initialize structured logging
    if let Err(e) = logging::init_logging() {
        eprintln!("Warning: failed to initialize logging: {e}");
    }

    if let Err(e) = run().await {
        e.exit();
    }
}

async fn run() -> Result<(), KobanaError> {
    // Phase 1: Parse specs and build command trees
    let v1_spec = ApiSpec::parse(V1_SPEC_JSON)?;
    let v2_spec = ApiSpec::parse(V2_SPEC_JSON)?;

    let v1_tree = v1_spec.build_command_tree();
    let v2_tree = v2_spec.build_command_tree();

    // Phase 2: Build clap command and parse args
    let root_cmd = commands::build_root_command(&v1_tree, &v2_tree);
    let matches = root_cmd.get_matches();

    // Handle special commands
    if let Some(("schema", schema_matches)) = matches.subcommand() {
        return schema::handle_schema(schema_matches, &v1_spec, &v2_spec, &v1_tree, &v2_tree);
    }

    if let Some(("update", update_matches)) = matches.subcommand() {
        let check_only = update_matches.get_flag("check");
        let as_json = update_matches.get_flag("json");
        return update::handle_update(check_only, as_json).await;
    }

    if let Some(("auth", auth_matches)) = matches.subcommand() {
        return auth_commands::handle_auth(auth_matches, &matches).await;
    }

    if let Some(("completions", comp_matches)) = matches.subcommand() {
        let shell = comp_matches.get_one::<String>("shell").unwrap();
        let mut cmd = commands::build_root_command(&v1_tree, &v2_tree);
        return completions::generate_completions(shell, &mut cmd);
    }

    // Silently check for a new version once per day (best effort, swallows errors).
    // Skip for special commands that should be deterministic or pipe-clean.
    update::auto_check().await;

    // Check for helper commands (+emitir, +cobrar, etc.)
    if let Some((sub_name, sub_matches)) = matches.subcommand() {
        if sub_name.starts_with('+') {
            if let Some(helper) = helpers::find_helper(sub_name) {
                let env = config::resolve_environment(
                    matches.get_one::<String>("env").map(|s| s.as_str()),
                );
                let helper_dry_run = sub_matches.get_flag("dry-run");
                let token = if helper_dry_run {
                    auth::resolve_token().unwrap_or_default()
                } else {
                    auth::resolve_token()?
                };
                let client = kobana::client::KobanaClient::new(env.base_url(), &token)?;
                return helper.execute(&client, sub_matches).await;
            }
        }
    }

    // Resolve endpoint
    let (endpoint, method_matches) =
        commands::resolve_endpoint(&matches, &v1_tree, &v2_tree).ok_or_else(|| {
            KobanaError::Validation("could not resolve endpoint from arguments".into())
        })?;

    // Resolve environment
    let env = config::resolve_environment(
        matches.get_one::<String>("env").map(|s| s.as_str()),
    );

    // For dry-run, we don't need auth
    let dry_run = matches.get_flag("dry-run");
    let token = if dry_run {
        auth::resolve_token().unwrap_or_default()
    } else {
        auth::resolve_token()?
    };

    // Create client
    let client = kobana::client::KobanaClient::new(env.base_url(), &token)?;

    // Execute
    executor::execute(&client, endpoint, &method_matches, &matches).await
}
