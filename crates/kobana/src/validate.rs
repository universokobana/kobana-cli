use crate::error::KobanaError;
use crate::trf;

/// Validate that a string identifier is safe for use in URLs
pub fn validate_identifier(value: &str, name: &str) -> Result<(), KobanaError> {
    if value.is_empty() {
        return Err(KobanaError::Validation(trf!(
            pt: "{name} não pode ser vazio",
            en: "{name} cannot be empty",
            es: "{name} no puede estar vacío",
        )));
    }

    // Reject path traversal
    if value.contains("../") || value.contains("..\\") {
        return Err(KobanaError::Validation(trf!(
            pt: "{name} contém uma sequência de travessia de caminho",
            en: "{name} contains path traversal sequence",
            es: "{name} contiene una secuencia de recorrido de ruta",
        )));
    }

    // Reject control characters (ASCII < 0x20)
    if value.bytes().any(|b| b < 0x20) {
        return Err(KobanaError::Validation(trf!(
            pt: "{name} contém caracteres de controle",
            en: "{name} contains control characters",
            es: "{name} contiene caracteres de control",
        )));
    }

    // Reject URL injection characters in identifiers
    if value.contains('?') || value.contains('#') {
        return Err(KobanaError::Validation(trf!(
            pt: "{name} contém caracteres inválidos para URL (? ou #)",
            en: "{name} contains invalid URL characters (? or #)",
            es: "{name} contiene caracteres inválidos para URL (? o #)",
        )));
    }

    // Reject double-encoding
    if value.contains('%') {
        return Err(KobanaError::Validation(trf!(
            pt: "{name} contém caracteres codificados com %",
            en: "{name} contains percent-encoded characters",
            es: "{name} contiene caracteres codificados con %",
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifier() {
        assert!(validate_identifier("12345", "id").is_ok());
        assert!(validate_identifier("019d6b00-4751-719d-8a6f-20cb9223bea4", "uid").is_ok());
    }

    #[test]
    fn test_path_traversal() {
        assert!(validate_identifier("../etc/passwd", "id").is_err());
    }

    #[test]
    fn test_url_injection() {
        assert!(validate_identifier("123?admin=true", "id").is_err());
        assert!(validate_identifier("123#fragment", "id").is_err());
    }

    #[test]
    fn test_control_chars() {
        assert!(validate_identifier("123\x00", "id").is_err());
    }

    #[test]
    fn test_double_encoding() {
        assert!(validate_identifier("123%20", "id").is_err());
    }
}
