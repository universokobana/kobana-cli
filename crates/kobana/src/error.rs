use std::process;

use crate::trf;

/// Exit codes following the CLI spec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success = 0,
    ApiError = 1,
    AuthError = 2,
    ValidationError = 3,
    SchemaError = 4,
    InternalError = 5,
}

#[derive(Debug, thiserror::Error)]
pub enum KobanaError {
    #[error("{}", trf!(
        pt: "Erro da API ({status}): {message}",
        en: "API error ({status}): {message}",
        es: "Error de la API ({status}): {message}",
    ))]
    Api {
        status: u16,
        message: String,
        body: Option<serde_json::Value>,
    },

    #[error("{}", trf!(
        pt: "Erro de autenticação: {_0}",
        en: "Authentication error: {_0}",
        es: "Error de autenticación: {_0}",
    ))]
    Auth(String),

    #[error("{}", trf!(
        pt: "Erro de validação: {_0}",
        en: "Validation error: {_0}",
        es: "Error de validación: {_0}",
    ))]
    Validation(String),

    #[error("{}", trf!(
        pt: "Erro de schema: {_0}",
        en: "Schema error: {_0}",
        es: "Error de schema: {_0}",
    ))]
    Schema(String),

    #[error("{}", trf!(
        pt: "Erro interno: {_0}",
        en: "Internal error: {_0}",
        es: "Error interno: {_0}",
    ))]
    Internal(String),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl KobanaError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Api { .. } | Self::Http(_) => ExitCode::ApiError,
            Self::Auth(_) => ExitCode::AuthError,
            Self::Validation(_) => ExitCode::ValidationError,
            Self::Schema(_) => ExitCode::SchemaError,
            Self::Internal(_) | Self::Json(_) | Self::Io(_) => ExitCode::InternalError,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Api { status, message, body } => {
                let mut error = serde_json::json!({
                    "error": {
                        "code": status,
                        "message": message,
                    }
                });
                if let Some(b) = body {
                    error["error"]["details"] = b.clone();
                }
                error
            }
            other => {
                serde_json::json!({
                    "error": {
                        "code": other.exit_code() as u16,
                        "message": other.to_string(),
                    }
                })
            }
        }
    }

    /// Print error as JSON to stdout and exit with the appropriate code
    pub fn exit(&self) -> ! {
        let json = self.to_json();
        println!("{}", serde_json::to_string_pretty(&json).unwrap_or_default());
        process::exit(self.exit_code() as i32)
    }
}
