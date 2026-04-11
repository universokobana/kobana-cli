use kobana::error::KobanaError;

use crate::config;
use crate::credential_store::{self, StoredCredentials};
use crate::oauth;

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
        _ => Err(KobanaError::Validation("Unknown auth subcommand".into())),
    }
}

async fn handle_login(
    matches: &clap::ArgMatches,
    root_matches: &clap::ArgMatches,
) -> Result<(), KobanaError> {
    let verbose = root_matches.get_flag("verbose");
    let client_id = matches.get_one::<String>("client-id").cloned();
    let client_secret = matches.get_one::<String>("client-secret").cloned();
    let production = matches.get_flag("production");
    let development = matches.get_flag("development");
    let env = if development {
        config::Environment::Development
    } else if production {
        config::Environment::Production
    } else {
        config::resolve_environment(false, false, false)
    };

    // Resolve scopes: --scopes flag or all scopes by default
    let scopes = matches
        .get_one::<String>("scopes")
        .map(|s| s.replace(',', "+"))
        .unwrap_or_else(oauth::default_scopes);

    let token_response = if let (Some(id), Some(secret)) = (&client_id, &client_secret) {
        // Client credentials flow (requires both id + secret)
        eprintln!("Authenticating with client credentials...");
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

        eprintln!("Authenticating with PKCE (client: {id})...");
        oauth::authorization_code_flow(&id, secret.as_deref(), &scopes, &env, verbose).await?
    };

    let expires_at = token_response
        .expires_in
        .map(|secs| chrono::Utc::now().timestamp() + secs);

    let env_name = match env {
        config::Environment::Production => "production",
        config::Environment::Sandbox => "sandbox",
        config::Environment::Development => "development",
    };

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

    println!(
        "{}",
        serde_json::json!({
            "authenticated": true,
            "environment": creds.environment,
            "expires_at": creds.expires_at,
        })
    );

    Ok(())
}

fn handle_logout() -> Result<(), KobanaError> {
    credential_store::delete_credentials()?;
    println!(
        "{}",
        serde_json::json!({
            "logged_out": true,
            "message": "Credentials removed."
        })
    );
    Ok(())
}

fn handle_status() -> Result<(), KobanaError> {
    if let Ok(token) = std::env::var("KOBANA_TOKEN") {
        if !token.is_empty() {
            let env = config::resolve_environment(false, false, false);
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
                    "message": "No authentication configured. Set KOBANA_TOKEN or run 'kobana auth login'.",
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
