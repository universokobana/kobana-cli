use std::process;

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
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        body: Option<serde_json::Value>,
    },

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("Internal error: {0}")]
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
