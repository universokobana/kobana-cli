use kobana::error::KobanaError;

/// Validate --params JSON
pub fn validate_params(params: &serde_json::Value) -> Result<(), KobanaError> {
    if !params.is_object() {
        return Err(KobanaError::Validation(
            "--params must be a JSON object".into(),
        ));
    }

    if let Some(obj) = params.as_object() {
        for (key, value) in obj {
            // Validate that keys are reasonable
            if key.is_empty() {
                return Err(KobanaError::Validation(
                    "parameter key cannot be empty".into(),
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
            "--json must be a JSON object or array".into(),
        ));
    }
    Ok(())
}

/// Validate a parameter value for common injection patterns
fn validate_param_value(value: &str, param_name: &str) -> Result<(), KobanaError> {
    // Control characters
    if value.bytes().any(|b| b < 0x20 && b != b'\n' && b != b'\r' && b != b'\t') {
        return Err(KobanaError::Validation(format!(
            "parameter '{param_name}' contains control characters"
        )));
    }

    Ok(())
}
