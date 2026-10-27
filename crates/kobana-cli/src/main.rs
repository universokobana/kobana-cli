mod auth;
mod auth_commands;
mod clap_i18n;
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
mod product;
mod schema;
mod update;
mod validate;

use kobana::error::KobanaError;
use kobana::trf;

#[tokio::main]
async fn main() {
    // Load .env files
    config::load_dotenv();

    // The language must be fixed before any command or message is built
    let args: Vec<String> = std::env::args().collect();
    if let Some(lang) = clap_i18n::lang_from_args(&args) {
        kobana::i18n::set_lang(lang);
    }

    // Initialize structured logging
    if let Err(e) = logging::init_logging() {
        eprintln!(
            "{}",
            trf!(
                pt: "Aviso: não foi possível iniciar os logs: {e}",
                en: "Warning: failed to initialize logging: {e}",
                es: "Aviso: no se pudieron iniciar los logs: {e}",
            )
        );
    }

    if let Err(e) = run().await {
        e.exit();
    }
}

async fn run() -> Result<(), KobanaError> {
    // Phase 1: Load every registered product (parse specs, build command trees)
    let products = product::load_all()?;

    // Phase 2: Build clap command and parse args
    let root_cmd = commands::build_root_command(&products);
    let matches = root_cmd
        .try_get_matches()
        .unwrap_or_else(|e| clap_i18n::exit_with(e));

    if let Some(("help", help_matches)) = matches.subcommand() {
        let path: Vec<String> = help_matches
            .get_many::<String>("command")
            .map(|names| names.cloned().collect())
            .unwrap_or_default();
        return clap_i18n::print_help_for(commands::build_root_command(&products), &path);
    }

    // Handle special commands
    if let Some(("schema", schema_matches)) = matches.subcommand() {
        return schema::handle_schema(schema_matches, &products);
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
        let mut cmd = commands::build_root_command(&products);
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
                // Helpers wrap banking endpoints (+emitir, +cobrar, …)
                let banking = product::find(&products, "banking").ok_or_else(|| {
                    KobanaError::Internal("banking product is not registered".into())
                })?;
                let client = product::client_for(banking.product, &env, &token, helper_dry_run)?;
                return helper.execute(&client, sub_matches).await;
            }
        }
    }

    // Resolve endpoint
    let (loaded_product, endpoint, method_matches) =
        commands::resolve_endpoint(&matches, &products).ok_or_else(|| {
            KobanaError::Validation(
                kobana::tr!(
                    pt: "não foi possível identificar o endpoint pelos argumentos",
                    en: "could not resolve endpoint from arguments",
                    es: "no se pudo identificar el endpoint por los argumentos",
                )
                .into(),
            )
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

    // Create client — each product has its own API host and TLS requirements
    let client = product::client_for(loaded_product.product, &env, &token, dry_run)?;

    // Execute
    executor::execute(&client, endpoint, &method_matches, &matches).await
}
