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
