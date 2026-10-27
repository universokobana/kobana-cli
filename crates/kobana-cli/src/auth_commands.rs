use kobana::error::KobanaError;
use kobana::{tr, trf};

use crate::config;
use crate::credential_store::{self, StoredCredentials};
use crate::oauth;

/// Wrapper around a (name, description) tuple so inquire can display it nicely.
#[derive(Clone)]
struct LabeledOption {
    name: &'static str,
    description: &'static str,
    width: usize,
}

impl std::fmt::Display for LabeledOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<width$}  {}",
            self.name,
            self.description,
            width = self.width
        )
    }
}

fn multi_select_help() -> &'static str {
    tr!(
        pt: "↑↓ Navegar · Espaço Selecionar · →/← Todos/Nenhum · Enter Confirmar · Esc Cancelar",
        en: "↑↓ Move · Space Select · →/← All/None · Enter Confirm · Esc Cancel",
        es: "↑↓ Navegar · Espacio Seleccionar · →/← Todos/Ninguno · Enter Confirmar · Esc Cancelar",
    )
}

fn selection_cancelled(e: inquire::InquireError) -> String {
    trf!(
        pt: "seleção de escopos cancelada: {e}",
        en: "scope selection cancelled: {e}",
        es: "selección de alcances cancelada: {e}",
    )
}

/// Interactive scope selection with drill-down:
/// 1. Pick top-level resource groups (admin, charge, payment, etc.)
/// 2. For each selected group with sub-scopes, pick which sub-scopes to include
/// 3. Pick permission level (read or write)
fn prompt_scopes() -> Result<String, KobanaError> {
    // --- Screen 1: top-level resource groups ---
    let group_width = oauth::TOP_LEVEL_GROUPS
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0);

    let group_options: Vec<LabeledOption> = oauth::TOP_LEVEL_GROUPS
        .iter()
        .map(|(name, description)| LabeledOption {
            name,
            description: kobana::i18n::pick(*description),
            width: group_width,
        })
        .collect();

    let all_group_indices: Vec<usize> = (0..group_options.len()).collect();

    let selected_groups = inquire::MultiSelect::new(
        tr!(
            pt: "Selecione os grupos de recursos",
            en: "Select the resource groups",
            es: "Selecciona los grupos de recursos",
        ),
        group_options,
    )
    .with_default(&all_group_indices)
    .with_page_size(20)
    .with_help_message(multi_select_help())
    .prompt()
    .map_err(|e| KobanaError::Auth(selection_cancelled(e)))?;

    if selected_groups.is_empty() {
        return Err(KobanaError::Auth(
            tr!(
                pt: "nenhum escopo selecionado — escolha pelo menos um grupo",
                en: "no scopes selected — at least one group is required",
                es: "ningún alcance seleccionado — elige al menos un grupo",
            )
            .into(),
        ));
    }

    // --- Screens 2..N: drill-down per group ---
    let mut final_scopes: Vec<&'static str> = Vec::new();

    for group in &selected_groups {
        // Find all scopes under this group
        let sub_scopes: Vec<(&'static str, [&'static str; 3])> = oauth::ALL_SCOPES
            .iter()
            .filter(|(name, _)| {
                let top = name.split('.').next().unwrap_or(name);
                top == group.name
            })
            .copied()
            .collect();

        // Leaf group (like `login`): just include it, no drill-down screen
        if sub_scopes.len() <= 1 {
            for (name, _) in sub_scopes {
                final_scopes.push(name);
            }
            continue;
        }

        // Compute column width for this group's sub-scopes
        let sub_width = sub_scopes
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);

        let sub_options: Vec<LabeledOption> = sub_scopes
            .iter()
            .map(|(name, description)| LabeledOption {
                name,
                description: kobana::i18n::pick(*description),
                width: sub_width,
            })
            .collect();

        let all_sub_indices: Vec<usize> = (0..sub_options.len()).collect();

        let selected_subs = inquire::MultiSelect::new(
            &trf!(
                pt: "  └─ {}: selecione os sub-escopos",
                en: "  └─ {}: select the sub-scopes",
                es: "  └─ {}: selecciona los sub-alcances",
                group.name
            ),
            sub_options,
        )
        .with_default(&all_sub_indices)
        .with_page_size(20)
        .with_help_message(multi_select_help())
        .prompt()
        .map_err(|e| KobanaError::Auth(selection_cancelled(e)))?;

        for opt in selected_subs {
            final_scopes.push(opt.name);
        }
    }

    if final_scopes.is_empty() {
        return Err(KobanaError::Auth(
            tr!(
                pt: "nenhum escopo selecionado — escolha pelo menos um",
                en: "no scopes selected — at least one scope is required",
                es: "ningún alcance seleccionado — elige al menos uno",
            )
            .into(),
        ));
    }

    // --- Final screen: permission level ---
    let permission_options = vec![
        LabeledOption {
            name: "read",
            description: tr!(pt: "Somente leitura", en: "Read only", es: "Solo lectura"),
            width: 5,
        },
        LabeledOption {
            name: "write",
            description: tr!(pt: "Leitura e escrita", en: "Read and write", es: "Lectura y escritura"),
            width: 5,
        },
    ];

    let permission = inquire::Select::new(
        tr!(pt: "Nível de permissão", en: "Permission level", es: "Nivel de permiso"),
        permission_options,
    )
    .with_starting_cursor(0)
    .with_help_message(tr!(
        pt: "↑↓ Navegar · Enter Confirmar · Esc Cancelar",
        en: "↑↓ Move · Enter Confirm · Esc Cancel",
        es: "↑↓ Navegar · Enter Confirmar · Esc Cancelar",
    ))
    .prompt()
    .map_err(|e| {
        KobanaError::Auth(trf!(
            pt: "seleção de permissão cancelada: {e}",
            en: "permission selection cancelled: {e}",
            es: "selección de permiso cancelada: {e}",
        ))
    })?;

    final_scopes.push(permission.name);

    Ok(final_scopes.join("+"))
}

/// Handle `kobana auth <subcommand>`
pub async fn handle_auth(
    matches: &clap::ArgMatches,
    root_matches: &clap::ArgMatches,
) -> Result<(), KobanaError> {
    match matches.subcommand() {
        Some(("login", login_matches)) => handle_login(login_matches, root_matches).await,
        Some(("logout", _)) => handle_logout(),
        Some(("status", _)) => handle_status(),
        Some(("export", _)) => handle_export(),
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

async fn handle_login(
    matches: &clap::ArgMatches,
    root_matches: &clap::ArgMatches,
) -> Result<(), KobanaError> {
    let verbose = root_matches.get_flag("verbose");
    let client_id = matches.get_one::<String>("client-id").cloned();
    let client_secret = matches.get_one::<String>("client-secret").cloned();
    let env = config::resolve_environment(
        root_matches.get_one::<String>("env").map(|s| s.as_str()),
    );

    // Resolve scopes:
    //   1. --scopes flag (non-interactive)
    //   2. Interactive TUI if stdin is a terminal
    //   3. All scopes as fallback (non-interactive environments)
    let scopes = if let Some(s) = matches.get_one::<String>("scopes") {
        s.replace(',', "+")
    } else if std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        prompt_scopes()?
    } else {
        oauth::default_scopes()
    };

    let token_response = if let (Some(id), Some(secret)) = (&client_id, &client_secret) {
        // Client credentials flow (requires both id + secret)
        eprintln!(
            "{}",
            tr!(
                pt: "Autenticando com client credentials...",
                en: "Authenticating with client credentials...",
                es: "Autenticando con client credentials...",
            )
        );
        oauth::client_credentials_flow(id, secret, &env).await?
    } else {
        // Authorization Code + PKCE flow
        // Priority: --client-id flag > KOBANA_CLIENT_ID env > default "kobana-cli"
        let id = client_id
            .or_else(|| std::env::var("KOBANA_CLIENT_ID").ok())
            .unwrap_or_else(|| oauth::DEFAULT_CLIENT_ID.to_string());

        // Secret is optional with PKCE
        let secret = client_secret
            .or_else(|| std::env::var("KOBANA_CLIENT_SECRET").ok());

        eprintln!(
            "{}",
            trf!(
                pt: "Autenticando com PKCE (cliente: {id})...",
                en: "Authenticating with PKCE (client: {id})...",
                es: "Autenticando con PKCE (cliente: {id})...",
            )
        );
        oauth::authorization_code_flow(&id, secret.as_deref(), &scopes, &env, verbose).await?
    };

    let expires_at = token_response
        .expires_in
        .map(|secs| chrono::Utc::now().timestamp() + secs);

    let env_name = env.as_str();

    let creds = StoredCredentials {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        token_type: token_response
            .token_type
            .unwrap_or_else(|| "Bearer".into()),
        expires_at,
        environment: env_name.into(),
    };

    credential_store::save_credentials(&creds)?;

    let scopes_list: Vec<&str> = scopes.split('+').collect();
    let cred_path = credential_store::credentials_path();
    let backend = credential_store::key_backend();

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "status": "success",
            "message": tr!(
                pt: "Autenticação concluída. Credenciais criptografadas salvas.",
                en: "Authentication successful. Encrypted credentials saved.",
                es: "Autenticación completada. Credenciales cifradas guardadas.",
            ),
            "environment": creds.environment,
            "credentials_file": cred_path.to_string_lossy(),
            "encryption": trf!(
                pt: "AES-256-GCM (chave no keyring do sistema ou no `.key` local; backend atual: {backend})",
                en: "AES-256-GCM (key in OS keyring or local `.key`; current backend: {backend})",
                es: "AES-256-GCM (clave en el keyring del sistema o en el `.key` local; backend actual: {backend})",
            ),
            "expires_at": creds.expires_at,
            "scopes": scopes_list,
        }))
        .unwrap_or_default()
    );

    Ok(())
}

fn handle_logout() -> Result<(), KobanaError> {
    credential_store::delete_credentials()?;
    println!(
        "{}",
        serde_json::json!({
            "logged_out": true,
            "message": tr!(
                pt: "Credenciais removidas.",
                en: "Credentials removed.",
                es: "Credenciales eliminadas.",
            )
        })
    );
    Ok(())
}

fn handle_status() -> Result<(), KobanaError> {
    if let Ok(token) = std::env::var("KOBANA_TOKEN") {
        if !token.is_empty() {
            let env = config::resolve_environment(None);
            println!(
                "{}",
                serde_json::json!({
                    "authenticated": true,
                    "method": "KOBANA_TOKEN",
                    "environment": format!("{:?}", env),
                })
            );
            return Ok(());
        }
    }

    match credential_store::load_credentials() {
        Ok(creds) => {
            let expired = creds
                .expires_at
                .map(|exp| chrono::Utc::now().timestamp() > exp)
                .unwrap_or(false);

            println!(
                "{}",
                serde_json::json!({
                    "authenticated": true,
                    "method": "saved_credentials",
                    "environment": creds.environment,
                    "expired": expired,
                    "expires_at": creds.expires_at,
                    "has_refresh_token": creds.refresh_token.is_some(),
                })
            );
        }
        Err(_) => {
            println!(
                "{}",
                serde_json::json!({
                    "authenticated": false,
                    "message": crate::auth::not_configured_message(),
                })
            );
        }
    }

    Ok(())
}

fn handle_export() -> Result<(), KobanaError> {
    if let Ok(token) = std::env::var("KOBANA_TOKEN") {
        if !token.is_empty() {
            println!(
                "{}",
                serde_json::json!({
                    "access_token": token,
                    "token_type": "Bearer",
                    "source": "KOBANA_TOKEN",
                })
            );
            return Ok(());
        }
    }

    let creds = credential_store::load_credentials()?;
    println!(
        "{}",
        serde_json::json!({
            "access_token": creds.access_token,
            "refresh_token": creds.refresh_token,
            "token_type": creds.token_type,
            "expires_at": creds.expires_at,
            "environment": creds.environment,
            "source": "saved_credentials",
        })
    );

    Ok(())
}
