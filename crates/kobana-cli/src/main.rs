mod auth;
mod auth_commands;
mod commands;
mod config;
mod credential_store;
mod executor;
mod formatter;
mod oauth;
mod schema;

use kobana::error::KobanaError;
use kobana::spec::ApiSpec;

// Embed OpenAPI specs at compile time
const V1_SPEC_JSON: &str = include_str!("../specs/v1.json");
const V2_SPEC_JSON: &str = include_str!("../specs/v2.json");

#[tokio::main]
async fn main() {
    // Load .env files
    config::load_dotenv();

    // Initialize logging
    let env_filter = std::env::var("KOBANA_LOG").unwrap_or_else(|_| "warn".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();

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

    if let Some(("auth", auth_matches)) = matches.subcommand() {
        return auth_commands::handle_auth(auth_matches).await;
    }

    // Resolve endpoint
    let (endpoint, method_matches) =
        commands::resolve_endpoint(&matches, &v1_tree, &v2_tree).ok_or_else(|| {
            KobanaError::Validation("could not resolve endpoint from arguments".into())
        })?;

    // Resolve environment
    let sandbox = matches.get_flag("sandbox");
    let production = matches.get_flag("production");
    let env = config::resolve_environment(sandbox, production);

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
