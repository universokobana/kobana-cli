use kobana::error::KobanaError;
use std::path::PathBuf;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

/// Initialize structured logging
///
/// - stderr: human-readable logs filtered by KOBANA_LOG
/// - file: JSON logs with daily rotation if KOBANA_LOG_FILE is set
pub fn init_logging() -> Result<(), KobanaError> {
    let env_filter = std::env::var("KOBANA_LOG").unwrap_or_else(|_| "warn".to_string());

    let stderr_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_ansi(true)
        .compact()
        .with_filter(
            EnvFilter::try_new(&env_filter)
                .unwrap_or_else(|_| EnvFilter::new("warn")),
        );

    // Check for file logging
    if let Ok(log_dir) = std::env::var("KOBANA_LOG_FILE") {
        let log_path = PathBuf::from(&log_dir);
        std::fs::create_dir_all(&log_path)
            .map_err(|e| KobanaError::Internal(format!("failed to create log dir: {e}")))?;

        let today = chrono::Local::now().format("%Y-%m-%d");
        let log_file = log_path.join(format!("kobana-{today}.log"));

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .map_err(|e| KobanaError::Internal(format!("failed to open log file: {e}")))?;

        let file_layer = fmt::layer()
            .with_writer(std::sync::Mutex::new(file))
            .json()
            .with_filter(EnvFilter::new("kobana=debug,kobana_cli=debug"));

        tracing_subscriber::registry()
            .with(stderr_layer)
            .with(file_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(stderr_layer)
            .init();
    }

    Ok(())
}
