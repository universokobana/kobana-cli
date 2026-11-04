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
mod jq;
mod logging;
mod oauth;
mod pagination;
mod product;
mod schema;
mod ui;
mod update;
mod validate;

use kobana::error::KobanaError;
use kobana::trf;

#[tokio::main]
async fn main() {
    // Load .env files
    let project_dotenv = config::load_dotenv();

    // The language must be fixed before any command or message is built
    let args: Vec<String> = std::env::args().collect();
    if let Some(lang) = clap_i18n::lang_from_args(&args) {
        kobana::i18n::set_lang(lang);
    }
    // Colors and the accessibility settings, before anything is printed
    ui::init(&args);

    for var in project_dotenv.ignored {
        ui::warn(&trf!(
            pt: "{var} no .env do diretório atual foi ignorada; defina-a no ambiente ou no .env da pasta de configuração",
            en: "{var} in the current directory's .env was ignored; set it in the environment or in the config folder's .env",
            es: "{var} en el .env del directorio actual se ignoró; defínela en el entorno o en el .env de la carpeta de configuración",
        ));
    }
    // What a checked-out `.env` did set is never silent either
    if !project_dotenv.applied.is_empty() {
        let vars = project_dotenv.applied.join(", ");
        ui::warn(&trf!(
            pt: "o .env do diretório atual definiu {vars}",
            en: "the current directory's .env set {vars}",
            es: "el .env del directorio actual definió {vars}",
        ));
    }

    // Initialize structured logging
    if let Err(e) = logging::init_logging() {
        ui::warn(&trf!(
            pt: "não foi possível iniciar os logs: {e}",
            en: "failed to initialize logging: {e}",
            es: "no se pudieron iniciar los logs: {e}",
        ));
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
    ui::set_no_color_flag(matches.get_flag("no-color"));

    // `--jq` is checked before anything runs: a typo never costs a request.
    // An empty one is no filter, as in gh.
    if let Some(expr) = matches.get_one::<String>("jq").filter(|e| !e.trim().is_empty()) {
        jq::set(expr)?;
    }

    // Warn on stderr when a newer version is out, checking the network at most
    // once a day (best effort, swallows errors). `update` reports it itself and
    // `completions` is sourced by shells at start-up, so both skip it; so does
    // a run without a person at a terminal (see `should_auto_check`).
    if !matches!(matches.subcommand_name(), Some("update" | "completions")) && update::should_auto_check() {
        update::auto_check().await;
    }

    if matches.subcommand_name() == Some("accessibility") {
        print!("{}", ui::accessibility_text());
        return Ok(());
    }

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
        // `--jq` filters the JSON report, so it implies `--json` (which only reports)
        let as_json = commands::update_json_flag(update_matches)? || jq::expr().is_some();
        return update::handle_update(check_only, as_json).await;
    }

    // `kobana <product> auth …`
    if let Some((name, sub)) = matches.subcommand() {
        if let (Some(product), Some(("auth", auth_matches))) = (product::by_slug(name), sub.subcommand()) {
            return auth_commands::handle_auth(product, auth_matches, &matches).await;
        }
    }

    if let Some(("completions", comp_matches)) = matches.subcommand() {
        let shell = comp_matches.get_one::<String>("shell").unwrap();
        let mut cmd = commands::build_root_command(&products);
        return completions::generate_completions(shell, &mut cmd);
    }

    // `kobana <product> +verb` helpers (+emitir, +cobrar, …)
    if let Some((name, sub)) = matches.subcommand() {
        if let Some((helper_name, helper_matches)) = sub.subcommand() {
            if let Some(helper) = helpers::find_helper(name, helper_name) {
                let loaded = product::find(&products, name).ok_or_else(|| {
                    KobanaError::Internal(format!("product {name} is not registered"))
                })?;
                let env = config::resolve_environment(
                    matches.get_one::<String>("env").map(|s| s.as_str()),
                )?;
                let helper_dry_run = helper_matches.get_flag("dry-run");
                let token = token_unless_dry_run(loaded.product, &env, explicit_env(&matches), helper_dry_run).await?;
                let client = product::client_for(loaded.product, &env, &token, helper_dry_run)?;
                return helper.execute(&client, helper_matches).await;
            }
        }
    }

    // Resolve endpoint
    let (loaded_product, endpoint) =
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
    )?;

    let dry_run = matches.get_flag("dry-run");
    let token = token_unless_dry_run(loaded_product.product, &env, explicit_env(&matches), dry_run).await?;

    // Create client — each product has its own API host and TLS requirements
    let client = product::client_for(loaded_product.product, &env, &token, dry_run)?;

    // Execute
    executor::execute(&client, endpoint, &matches).await
}

/// Whether `--env` was typed on this command line
fn explicit_env(matches: &clap::ArgMatches) -> bool {
    matches.get_one::<String>("env").is_some()
}

/// The product's token, or none for a dry run. A dry run sends nothing and
/// shows no `Authorization`, so it must not touch the credentials either:
/// resolving would refresh over the network and rewrite the credentials file.
///
/// A token that is not bound to an environment (`KOBANA_<PRODUCT>_TOKEN`, a
/// credentials file without `environment`) only goes to a plain-HTTP host
/// when `--env` was typed: a production token must not travel in the clear
/// because something in the environment said `development`.
async fn token_unless_dry_run(
    product: &product::Product,
    env: &config::Environment,
    explicit_env: bool,
    dry_run: bool,
) -> Result<String, KobanaError> {
    if dry_run {
        return Ok(String::new());
    }
    let token = auth::resolve(product, env).await?;
    auth::refuse_unbound_token_in_the_clear(&token, product.base_url(env)?, explicit_env)?;
    Ok(token.access_token)
}
