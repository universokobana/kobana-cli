use clap::Command;
use clap_complete::{generate, Shell};
use kobana::error::KobanaError;
use std::io;

/// Handle `kobana completions <shell>` — generate shell completions
pub fn generate_completions(shell_name: &str, cmd: &mut Command) -> Result<(), KobanaError> {
    let shell = match shell_name.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" | "pwsh" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        other => {
            return Err(KobanaError::Validation(format!(
                "unknown shell '{other}'. Supported: bash, zsh, fish, powershell, elvish"
            )));
        }
    };

    generate(shell, cmd, "kobana", &mut io::stdout());
    Ok(())
}
