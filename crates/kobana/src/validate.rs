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

    // An identifier is one path segment: a separator would reach another
    // route (`1/cancel`), and `.`/`..` are resolved away by the URL parser
    if value == "." || value == ".." || value.contains("../") || value.contains("..\\") {
        return Err(KobanaError::Validation(trf!(
            pt: "{name} contém uma sequência de travessia de caminho",
            en: "{name} contains path traversal sequence",
            es: "{name} contiene una secuencia de recorrido de ruta",
        )));
    }
    if value.contains('/') || value.contains('\\') {
        return Err(KobanaError::Validation(trf!(
            pt: "{name} não pode conter / nem \\",
            en: "{name} cannot contain / or \\",
            es: "{name} no puede contener / ni \\",
        )));
    }

    // Reject control characters (ASCII < 0x20)
    if value.bytes().any(|b| b < 0x20 || b == 0x7f) {
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
    fn rejects_anything_that_is_not_one_path_segment() {
        for bad in ["1/cancel", "..", ".", "a\\b", "/", "../../v2/charge/pix/x"] {
            assert!(validate_identifier(bad, "id").is_err(), "{bad}");
        }
        // Dots inside an identifier are fine
        assert!(validate_identifier("v1.2", "id").is_ok());
        assert!(validate_identifier("a..b", "id").is_ok());
    }

    #[test]
    fn test_url_injection() {
        assert!(validate_identifier("123?admin=true", "id").is_err());
        assert!(validate_identifier("123#fragment", "id").is_err());
    }

    #[test]
    fn test_control_chars() {
        assert!(validate_identifier("123\x00", "id").is_err());
        assert!(validate_identifier("123\x7f", "id").is_err());
    }

    #[test]
    fn test_double_encoding() {
        assert!(validate_identifier("123%20", "id").is_err());
    }
}
