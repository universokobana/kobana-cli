use kobana::error::KobanaError;
use kobana::{tr, trf};

/// Validate --params JSON
pub fn validate_params(params: &serde_json::Value) -> Result<(), KobanaError> {
    if !params.is_object() {
        return Err(KobanaError::Validation(
            tr!(
                pt: "--params precisa ser um objeto JSON",
                en: "--params must be a JSON object",
                es: "--params debe ser un objeto JSON",
            )
            .into(),
        ));
    }

    if let Some(obj) = params.as_object() {
        for (key, value) in obj {
            // Validate that keys are reasonable
            if key.is_empty() {
                return Err(KobanaError::Validation(
                    tr!(
                        pt: "o nome de um parâmetro não pode ser vazio",
                        en: "parameter key cannot be empty",
                        es: "el nombre de un parámetro no puede estar vacío",
                    )
                    .into(),
                ));
            }

            // Validate string values for injection
            if let Some(s) = value.as_str() {
                validate_param_value(s, key)?;
            }
        }
    }

    Ok(())
}

/// Validate --json request body
pub fn validate_body(body: &serde_json::Value) -> Result<(), KobanaError> {
    if !body.is_object() && !body.is_array() {
        return Err(KobanaError::Validation(
            tr!(
                pt: "--json precisa ser um objeto ou uma lista JSON",
                en: "--json must be a JSON object or array",
                es: "--json debe ser un objeto o una lista JSON",
            )
            .into(),
        ));
    }
    Ok(())
}

/// Validate `--output` before anything is sent: the path may come from an
/// agent. It may not climb out of a directory (`..`), hold control
/// characters, or name a symlink or anything but a regular file — writing
/// through a link would overwrite whatever it points to. An absolute path or
/// one under the current directory is the caller's choice.
pub fn validate_output_path(path: &str) -> Result<(), KobanaError> {
    if path.is_empty() || path.chars().any(char::is_control) {
        return Err(KobanaError::Validation(
            tr!(
                pt: "--output precisa ser um caminho sem caracteres de controle",
                en: "--output must be a path without control characters",
                es: "--output debe ser una ruta sin caracteres de control",
            )
            .into(),
        ));
    }
    let candidate = std::path::Path::new(path);
    if candidate
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(KobanaError::Validation(trf!(
            pt: "--output não pode subir de pasta com '..': {path}",
            en: "--output cannot climb directories with '..': {path}",
            es: "--output no puede subir de carpeta con '..': {path}",
        )));
    }
    if let Ok(meta) = std::fs::symlink_metadata(candidate) {
        if !meta.file_type().is_file() {
            return Err(KobanaError::Validation(trf!(
                pt: "--output precisa ser um arquivo comum (não um link, pasta ou dispositivo): {path}",
                en: "--output must be a regular file (not a link, directory or device): {path}",
                es: "--output debe ser un archivo común (no un enlace, carpeta o dispositivo): {path}",
            )));
        }
    }
    Ok(())
}

/// Open the `--output` file for writing, safe against what may change after
/// [`validate_output_path`] ran: on Unix a symlink put in its place is not
/// followed (`O_NOFOLLOW`), a FIFO does not block (`O_NONBLOCK`), and anything
/// that is not a regular file once open is refused before it is truncated.
pub fn open_output(path: &str) -> std::io::Result<std::fs::File> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o644).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(path)?;
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            tr!(
                pt: "não é um arquivo comum",
                en: "not a regular file",
                es: "no es un archivo común",
            ),
        ));
    }
    // Truncated only now that it is known to be a regular file
    file.set_len(0)?;
    Ok(file)
}

/// Validate a parameter value for common injection patterns
fn validate_param_value(value: &str, param_name: &str) -> Result<(), KobanaError> {
    // Control characters
    if value.bytes().any(|b| b < 0x20 && b != b'\n' && b != b'\r' && b != b'\t') {
        return Err(KobanaError::Validation(trf!(
            pt: "o parâmetro '{param_name}' contém caracteres de controle",
            en: "parameter '{param_name}' contains control characters",
            es: "el parámetro '{param_name}' contiene caracteres de control",
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_stay_regular_files_without_parent_jumps() {
        let dir = std::env::temp_dir().join(format!("kobana-output-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("out.json");
        std::fs::write(&file, "x").unwrap();

        assert!(validate_output_path("./boleto.pdf").is_ok());
        assert!(validate_output_path(&file.display().to_string()).is_ok(), "existing file");
        assert!(validate_output_path(&dir.join("new.json").display().to_string()).is_ok(), "new file");

        for bad in ["", "../x.json", "a/../../x", "x\nY"] {
            assert!(matches!(validate_output_path(bad), Err(KobanaError::Validation(_))), "{bad:?}");
        }
        assert!(validate_output_path(&dir.display().to_string()).is_err(), "directory");
        #[cfg(unix)]
        {
            let link = dir.join("link.json");
            std::os::unix::fs::symlink(&file, &link).unwrap();
            assert!(validate_output_path(&link.display().to_string()).is_err(), "symlink");
        }
        std::fs::remove_dir_all(dir).ok();
    }

    #[cfg(unix)]
    #[test]
    fn opening_the_output_does_not_follow_a_link_swapped_in() {
        let dir = std::env::temp_dir().join(format!("kobana-open-output-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("victim");
        std::fs::write(&target, "keep").unwrap();
        let link = dir.join("out.json");
        std::os::unix::fs::symlink(&target, &link).unwrap();

        assert!(open_output(&link.display().to_string()).is_err());
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "keep");

        // A regular file is truncated and written
        let file = dir.join("ok.json");
        std::fs::write(&file, "old content").unwrap();
        use std::io::Write;
        open_output(&file.display().to_string()).unwrap().write_all(b"new").unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "new");
        std::fs::remove_dir_all(dir).ok();
    }
}
