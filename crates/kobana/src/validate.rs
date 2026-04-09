use crate::error::KobanaError;

/// Validate that a string identifier is safe for use in URLs
pub fn validate_identifier(value: &str, name: &str) -> Result<(), KobanaError> {
    if value.is_empty() {
        return Err(KobanaError::Validation(format!("{name} cannot be empty")));
    }

    // Reject path traversal
    if value.contains("../") || value.contains("..\\") {
        return Err(KobanaError::Validation(format!(
            "{name} contains path traversal sequence"
        )));
    }

    // Reject control characters (ASCII < 0x20)
    if value.bytes().any(|b| b < 0x20) {
        return Err(KobanaError::Validation(format!(
            "{name} contains control characters"
        )));
    }

    // Reject URL injection characters in identifiers
    if value.contains('?') || value.contains('#') {
        return Err(KobanaError::Validation(format!(
            "{name} contains invalid URL characters (? or #)"
        )));
    }

    // Reject double-encoding
    if value.contains('%') {
        return Err(KobanaError::Validation(format!(
            "{name} contains percent-encoded characters"
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
