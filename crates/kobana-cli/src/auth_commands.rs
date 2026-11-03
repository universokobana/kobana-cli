use kobana::error::KobanaError;
use kobana::{tr, trf};

use crate::auth::{self, TokenSource};
use crate::config::{self, Environment};
use crate::credential_store::{self, StoredCredentials};
use crate::oauth;
use crate::product::Product;

/// A (name, description) pair laid out in two columns for inquire
#[derive(Clone)]
struct LabeledOption {
    name: String,
    description: String,
    width: usize,
}

impl std::fmt::Display for LabeledOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.description.is_empty() {
            return write!(f, "{}", self.name);
        }
        write!(
            f,
            "{:<width$}  {}",
            self.name,
            self.description,
            width = self.width
        )
    }
}

fn labeled(items: Vec<(String, String)>) -> Vec<LabeledOption> {
    let width = items.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
    items
        .into_iter()
        .map(|(name, description)| LabeledOption { name, description, width })
        .collect()
}

/// Confirmation line of a multi-select: the names only, not the columns
fn names_only(options: &[inquire::list_option::ListOption<&LabeledOption>]) -> String {
    options.iter().map(|o| o.value.name.as_str()).collect::<Vec<_>>().join(", ")
}

fn multi_select_help() -> &'static str {
    tr!(
        pt: "↑↓ Navegar · Espaço Selecionar · →/← Todos/Nenhum · Digite para filtrar · Enter Confirmar · Esc Cancelar",
        en: "↑↓ Move · Space Select · →/← All/None · Type to filter · Enter Confirm · Esc Cancel",
        es: "↑↓ Navegar · Espacio Seleccionar · →/← Todos/Ninguno · Escribe para filtrar · Enter Confirmar · Esc Cancelar",
    )
}

fn selection_cancelled(e: inquire::InquireError) -> KobanaError {
    KobanaError::Auth(trf!(
        pt: "seleção cancelada: {e}",
        en: "selection cancelled: {e}",
        es: "selección cancelada: {e}",
    ))
}

fn is_terminal() -> bool {
    std::io::IsTerminal::is_terminal(&std::io::stdin())
}

/// The part of `{product}/{group}.{rest}` before the first `.`
fn scope_group<'a>(product: &str, scope: &'a str) -> &'a str {
    let resource = scope
        .strip_prefix(product)
        .and_then(|s| s.strip_prefix('/'))
        .unwrap_or(scope);
    resource.split('.').next().unwrap_or(resource)
}

/// Description of a resource scope, when the CLI knows one
fn scope_label(product: &str, scope: &str) -> String {
    if product != "banking" {
        return String::new();
    }
    let legacy = scope.strip_prefix("banking/").unwrap_or(scope);
    oauth::BANKING_SCOPE_LABELS
        .iter()
        .find(|(name, _)| *name == legacy)
        .map(|(_, labels)| kobana::i18n::pick(*labels).to_string())
        .unwrap_or_default()
}

fn group_label(product: &str, group: &str) -> String {
    if product != "banking" {
        return String::new();
    }
    oauth::BANKING_GROUP_LABELS
        .iter()
        .find(|(name, _)| *name == group)
        .map(|(_, labels)| kobana::i18n::pick(*labels).to_string())
        .unwrap_or_default()
}

fn pick_many(
    message: &str,
    options: Vec<LabeledOption>,
) -> Result<Vec<LabeledOption>, KobanaError> {
    let all: Vec<usize> = (0..options.len()).collect();
    inquire::MultiSelect::new(message, options)
        .with_default(&all)
        .with_page_size(20)
        .with_formatter(&names_only)
        .with_help_message(multi_select_help())
        .prompt()
        .map_err(selection_cancelled)
}

/// Screens 2..N: the product's published resource scopes, grouped by the
/// segment before the first `.` when a group has more than one scope
/// (banking's `charge.*`, `payment.*`…), then the access levels.
fn prompt_scopes(product: &Product, catalog: &oauth::ScopeCatalog) -> Result<Vec<String>, KobanaError> {
    let slug = product.slug;
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
    for scope in &catalog.resources {
        let group = scope_group(slug, scope).to_string();
        match groups.iter_mut().find(|(g, _)| *g == group) {
            Some((_, scopes)) => scopes.push(scope.clone()),
            None => groups.push((group, vec![scope.clone()])),
        }
    }

    let mut chosen: Vec<String> = Vec::new();
    if groups.iter().any(|(_, scopes)| scopes.len() > 1) {
        let options = labeled(
            groups
                .iter()
                .map(|(g, _)| (g.clone(), group_label(slug, g)))
                .collect(),
        );
        let selected = pick_many(
            &trf!(
                pt: "{slug}: selecione os grupos de recursos",
                en: "{slug}: select the resource groups",
                es: "{slug}: selecciona los grupos de recursos",
            ),
            options,
        )?;
        for group in selected {
            let scopes = &groups.iter().find(|(g, _)| *g == group.name).expect("listed").1;
            if scopes.len() == 1 {
                chosen.extend(scopes.iter().cloned());
                continue;
            }
            let options = labeled(
                scopes
                    .iter()
                    .map(|s| (s.clone(), scope_label(slug, s)))
                    .collect(),
            );
            let subs = pick_many(
                &trf!(
                    pt: "  └─ {}: selecione os sub-escopos",
                    en: "  └─ {}: select the sub-scopes",
                    es: "  └─ {}: selecciona los sub-alcances",
                    group.name
                ),
                options,
            )?;
            chosen.extend(subs.into_iter().map(|o| o.name));
        }
    } else {
        let options = labeled(
            catalog
                .resources
                .iter()
                .map(|s| (s.clone(), scope_label(slug, s)))
                .collect(),
        );
        let selected = pick_many(
            &trf!(
                pt: "{slug}: selecione os recursos",
                en: "{slug}: select the resources",
                es: "{slug}: selecciona los recursos",
            ),
            options,
        )?;
        chosen.extend(selected.into_iter().map(|o| o.name));
    }

    if chosen.is_empty() {
        return Err(KobanaError::Auth(
            tr!(
                pt: "nenhum escopo selecionado — escolha pelo menos um recurso",
                en: "no scopes selected — at least one resource is required",
                es: "ningún alcance seleccionado — elige al menos un recurso",
            )
            .into(),
        ));
    }

    // --- Final screen: access levels (global wildcards) ---
    let options = labeled(
        oauth::ACCESS_LEVELS
            .iter()
            .map(|(name, labels)| (name.to_string(), kobana::i18n::pick(*labels).to_string()))
            .collect(),
    );
    let levels = inquire::MultiSelect::new(
        tr!(
            pt: "Níveis de acesso (valem para todos os recursos escolhidos)",
            en: "Access levels (apply to every chosen resource)",
            es: "Niveles de acceso (valen para todos los recursos elegidos)",
        ),
        options,
    )
    .with_default(&[0])
    .with_formatter(&names_only)
    .with_help_message(multi_select_help())
    .prompt()
    .map_err(selection_cancelled)?;
    chosen.extend(levels.into_iter().map(|o| o.name));

    Ok(chosen)
}

/// Handle `kobana <product> auth <subcommand>`
pub async fn handle_auth(
    product: &'static Product,
    matches: &clap::ArgMatches,
    root_matches: &clap::ArgMatches,
) -> Result<(), KobanaError> {
    let env = config::resolve_environment(
        root_matches.get_one::<String>("env").map(|s| s.as_str()),
    )?;
    match matches.subcommand() {
        Some(("login", m)) => handle_login(product, m, &env, root_matches.get_flag("verbose")).await,
        Some(("logout", _)) => handle_logout(product, &env),
        Some(("status", _)) => handle_status(product, &env),
        Some(("export", _)) => handle_export(product, &env).await,
        _ => Err(KobanaError::Validation(
            tr!(
                pt: "subcomando de auth desconhecido",
                en: "Unknown auth subcommand",
                es: "subcomando de auth desconocido",
            )
            .into(),
        )),
    }
}

fn parse_scopes(raw: &str) -> Vec<String> {
    raw.split([',', '+', ' '])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

async fn handle_login(
    product: &'static Product,
    matches: &clap::ArgMatches,
    env: &Environment,
    verbose: bool,
) -> Result<(), KobanaError> {
    let interactive = is_terminal();
    let slug = product.slug;
    // Flags win over their env vars, and both choose the flow the same way
    let client_id = matches
        .get_one::<String>("client-id")
        .cloned()
        .or_else(|| std::env::var("KOBANA_CLIENT_ID").ok().filter(|v| !v.is_empty()));
    let client_secret = matches
        .get_one::<String>("client-secret")
        .cloned()
        .or_else(|| std::env::var("KOBANA_CLIENT_SECRET").ok().filter(|v| !v.is_empty()));

    // A secret means client credentials: an application token, not a
    // person's. The Conta Kobana only binds tokens to a product's `resource`
    // on behalf of a user, so this flow is banking's legacy API token. The
    // login of a person (PKCE) takes no secret.
    if let Some(secret) = &client_secret {
        if slug != "banking" {
            return Err(KobanaError::Validation(trf!(
                pt: "--client-secret/KOBANA_CLIENT_SECRET (client credentials) só vale para banking; {slug} exige o login de uma pessoa",
                en: "--client-secret/KOBANA_CLIENT_SECRET (client credentials) only works for banking; {slug} requires a person's login",
                es: "--client-secret/KOBANA_CLIENT_SECRET (client credentials) solo vale para banking; {slug} exige el inicio de sesión de una persona",
            )));
        }
        let Some(id) = &client_id else {
            return Err(KobanaError::Validation(
                tr!(
                    pt: "--client-secret/KOBANA_CLIENT_SECRET exige --client-id ou KOBANA_CLIENT_ID",
                    en: "--client-secret/KOBANA_CLIENT_SECRET requires --client-id or KOBANA_CLIENT_ID",
                    es: "--client-secret/KOBANA_CLIENT_SECRET exige --client-id o KOBANA_CLIENT_ID",
                )
                .into(),
            ));
        };
        eprintln!(
            "{}",
            tr!(
                pt: "Autenticando com client credentials...",
                en: "Authenticating with client credentials...",
                es: "Autenticando con client credentials...",
            )
        );
        let response = oauth::client_credentials_flow(id, secret, env).await?;
        return save_and_report(product, env, response, Some(id.clone()), None, Vec::new());
    }

    let resource = product.oauth_resource(env)?;

    let scopes = match matches.get_one::<String>("scopes") {
        Some(s) => parse_scopes(s),
        None => {
            let catalog = oauth::discover_scopes(slug, resource.mcp, env).await?;
            if verbose {
                eprintln!("{} {}", tr!(pt: "Escopos de", en: "Scopes from", es: "Alcances de"), catalog.source);
            }
            if interactive {
                prompt_scopes(product, &catalog)?
            } else {
                // Non-interactive: every published resource, read only
                let mut scopes = catalog.resources;
                scopes.push("read".into());
                scopes
            }
        }
    };
    if scopes.is_empty() {
        return Err(KobanaError::Validation(
            tr!(
                pt: "nenhum escopo informado",
                en: "no scopes given",
                es: "ningún alcance indicado",
            )
            .into(),
        ));
    }

    // Priority: --client-id flag > KOBANA_CLIENT_ID env > default "kobana-cli"
    let id = client_id.unwrap_or_else(|| oauth::DEFAULT_CLIENT_ID.to_string());

    let account = oauth::authorization_server(env);
    eprintln!(
        "{}",
        trf!(
            pt: "Entrando em {slug} pela Conta Kobana ({account}, cliente: {id})...",
            en: "Logging in to {slug} through the Conta Kobana ({account}, client: {id})...",
            es: "Iniciando sesión en {slug} con la Conta Kobana ({account}, cliente: {id})...",
        )
    );
    let response = oauth::authorization_code_flow(
        &id,
        &scopes.join(" "),
        resource.api,
        env,
        verbose,
    )
    .await?;

    save_and_report(product, env, response, Some(id), Some(resource.api.to_string()), scopes)
}

fn save_and_report(
    product: &Product,
    env: &Environment,
    response: oauth::TokenResponse,
    client_id: Option<String>,
    resource: Option<String>,
    scopes: Vec<String>,
) -> Result<(), KobanaError> {
    let slug = product.slug;
    let env_name = env.as_str();
    let creds = StoredCredentials {
        access_token: response.access_token,
        refresh_token: response.refresh_token,
        token_type: response.token_type.unwrap_or_else(|| "Bearer".into()),
        expires_at: response
            .expires_in
            .map(|secs| chrono::Utc::now().timestamp() + secs),
        environment: env_name.into(),
        product: Some(slug.into()),
        client_id,
        resource,
        scopes,
    };
    credential_store::save_credentials(slug, env_name, &creds)?;

    let backend = credential_store::key_backend();
    let mut report = serde_json::json!({
        "status": "success",
        "message": trf!(
            pt: "Autenticação em {slug} concluída. Credenciais criptografadas salvas.",
            en: "Authenticated to {slug}. Encrypted credentials saved.",
            es: "Autenticación en {slug} completada. Credenciales cifradas guardadas.",
        ),
        "product": slug,
        "environment": env_name,
        "resource": creds.resource,
        "credentials_file": credential_store::credentials_path(slug, env_name).to_string_lossy(),
        "encryption": trf!(
            pt: "AES-256-GCM (chave no keyring do sistema ou no `.key` local; backend atual: {backend})",
            en: "AES-256-GCM (key in OS keyring or local `.key`; current backend: {backend})",
            es: "AES-256-GCM (clave en el keyring del sistema o en el `.key` local; backend actual: {backend})",
        ),
        "expires_at": creds.expires_at,
        "scopes": creds.scopes,
    });
    // A token in the environment wins over the one just saved: say so
    if let Some((_, source)) = auth::env_token(product) {
        let var = source.label();
        report["warning"] = serde_json::json!(trf!(
            pt: "{var} está definida e tem precedência sobre estas credenciais",
            en: "{var} is set and takes precedence over these credentials",
            es: "{var} está definida y tiene precedencia sobre estas credenciales",
        ));
    }
    println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    Ok(())
}

fn handle_logout(product: &Product, env: &Environment) -> Result<(), KobanaError> {
    let removed = credential_store::delete_credentials(product.slug, env.as_str())?;
    let report = serde_json::json!({
        "logged_out": removed,
        "product": product.slug,
        "environment": env.as_str(),
        "message": if removed {
            tr!(pt: "Credenciais removidas.", en: "Credentials removed.", es: "Credenciales eliminadas.")
        } else {
            tr!(pt: "Não havia credenciais salvas.", en: "There were no saved credentials.", es: "No había credenciales guardadas.")
        },
    });
    println!("{report}");
    Ok(())
}

/// What `auth status` reports. "Not configured" only when nothing is: a
/// credentials file or saved credentials that cannot be read report why,
/// as the API calls would fail with that same error.
fn product_status(product: &Product, env: &Environment) -> serde_json::Value {
    if let Some((_, source)) = auth::env_token(product) {
        return serde_json::json!({ "authenticated": true, "method": source.label() });
    }
    if let Some(path) = auth::credentials_file_path() {
        let source = TokenSource::CredentialsFile(path.clone());
        match auth::credentials_file_for(product, env) {
            Ok(Some(_)) => {
                return serde_json::json!({ "authenticated": true, "method": source.label(), "file": path });
            }
            // Another product's or environment's file: the saved ones count
            Ok(None) => {}
            Err(e) => {
                return serde_json::json!({
                    "authenticated": false,
                    "method": source.label(),
                    "file": path,
                    "error": e.to_string(),
                });
            }
        }
    }
    if !credential_store::has_credentials(product.slug, env.as_str()) {
        return serde_json::json!({
            "authenticated": false,
            "message": auth::not_configured_for(product),
        });
    }
    match credential_store::load_credentials(product.slug, env.as_str()) {
        Ok(creds) => serde_json::json!({
            "authenticated": true,
            "method": TokenSource::Saved.label(),
            "expired": auth::is_expired(&creds),
            "expires_at": creds.expires_at,
            "has_refresh_token": creds.refresh_token.is_some(),
            "resource": creds.resource,
            "scopes": creds.scopes,
        }),
        Err(e) => serde_json::json!({
            "authenticated": false,
            "method": TokenSource::Saved.label(),
            "credentials_file": credential_store::credentials_path(product.slug, env.as_str()).to_string_lossy(),
            "error": e.to_string(),
        }),
    }
}

fn handle_status(product: &Product, env: &Environment) -> Result<(), KobanaError> {
    let mut report = product_status(product, env);
    report["product"] = product.slug.into();
    report["environment"] = env.as_str().into();
    println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    Ok(())
}

/// `auth export`: the token the API calls would use, from the same source
/// and refreshed the same way, with where it came from
async fn handle_export(product: &Product, env: &Environment) -> Result<(), KobanaError> {
    let token = auth::resolve(product, env).await?;
    let mut report = serde_json::json!({
        "access_token": token.access_token,
        "refresh_token": token.refresh_token,
        "token_type": token.token_type,
        "expires_at": token.expires_at,
        "environment": token.environment,
        "product": product.slug,
        "resource": token.resource,
        "source": token.source.label(),
    });
    if let TokenSource::CredentialsFile(path) = &token.source {
        report["file"] = path.clone().into();
    }
    println!("{report}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_by_the_segment_before_the_first_dot() {
        assert_eq!(scope_group("banking", "banking/charge.bank_billets"), "charge");
        assert_eq!(scope_group("banking", "banking/payment.dda.bank_billets"), "payment");
        assert_eq!(scope_group("billing", "billing/customers"), "customers");
    }

    #[test]
    fn scopes_split_on_commas_plus_and_spaces() {
        assert_eq!(
            parse_scopes("billing/customers, billing/plans+write read"),
            vec!["billing/customers", "billing/plans", "write", "read"]
        );
    }

    #[test]
    fn banking_scopes_keep_their_labels() {
        assert!(!scope_label("banking", "banking/charge.bank_billets").is_empty());
        assert!(scope_label("billing", "billing/customers").is_empty());
    }
}
