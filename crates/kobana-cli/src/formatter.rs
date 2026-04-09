use kobana::error::KobanaError;

/// Format output based on the requested format
pub fn format_output(value: &serde_json::Value, format: &str) -> Result<String, KobanaError> {
    match format {
        "json" => serde_json::to_string_pretty(value)
            .map_err(|e| KobanaError::Internal(format!("JSON formatting error: {e}"))),
        "table" => format_table(value),
        "csv" => format_csv(value),
        other => Err(KobanaError::Validation(format!(
            "unknown output format '{other}'. Use: json, table, csv"
        ))),
    }
}

/// Filter JSON response to only include specified fields
pub fn filter_fields(value: &serde_json::Value, fields: &str) -> serde_json::Value {
    let field_list: Vec<&str> = fields.split(',').map(|s| s.trim()).collect();

    match value {
        serde_json::Value::Array(arr) => {
            let filtered: Vec<serde_json::Value> =
                arr.iter().map(|item| filter_object(item, &field_list)).collect();
            serde_json::Value::Array(filtered)
        }
        serde_json::Value::Object(_) => filter_object(value, &field_list),
        // If there's a "data" wrapper, filter inside it
        other => other.clone(),
    }
}

fn filter_object(value: &serde_json::Value, fields: &[&str]) -> serde_json::Value {
    // Handle data wrapper pattern from Kobana API
    if let Some(data) = value.get("data") {
        return match data {
            serde_json::Value::Array(arr) => {
                let filtered: Vec<serde_json::Value> =
                    arr.iter().map(|item| filter_single_object(item, fields)).collect();
                serde_json::json!({ "data": filtered })
            }
            serde_json::Value::Object(_) => {
                serde_json::json!({ "data": filter_single_object(data, fields) })
            }
            other => serde_json::json!({ "data": other }),
        };
    }

    filter_single_object(value, fields)
}

fn filter_single_object(value: &serde_json::Value, fields: &[&str]) -> serde_json::Value {
    if let Some(obj) = value.as_object() {
        let mut filtered = serde_json::Map::new();
        for &field in fields {
            if let Some(v) = obj.get(field) {
                filtered.insert(field.to_string(), v.clone());
            }
        }
        serde_json::Value::Object(filtered)
    } else {
        value.clone()
    }
}

fn format_table(value: &serde_json::Value) -> Result<String, KobanaError> {
    use comfy_table::{Table, ContentArrangement};

    let items = extract_items(value);
    if items.is_empty() {
        return Ok("(empty)".to_string());
    }

    // Collect all unique keys from all items
    let mut columns: Vec<String> = Vec::new();
    for item in &items {
        if let Some(obj) = item.as_object() {
            for key in obj.keys() {
                if !columns.contains(key) {
                    columns.push(key.clone());
                }
            }
        }
    }

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(&columns);

    for item in &items {
        let row: Vec<String> = columns
            .iter()
            .map(|col| {
                item.get(col)
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Null => "".to_string(),
                        other => other.to_string(),
                    })
                    .unwrap_or_default()
            })
            .collect();
        table.add_row(row);
    }

    Ok(table.to_string())
}

fn format_csv(value: &serde_json::Value) -> Result<String, KobanaError> {
    let items = extract_items(value);
    if items.is_empty() {
        return Ok(String::new());
    }

    let mut columns: Vec<String> = Vec::new();
    for item in &items {
        if let Some(obj) = item.as_object() {
            for key in obj.keys() {
                if !columns.contains(key) {
                    columns.push(key.clone());
                }
            }
        }
    }

    let mut wtr = csv::Writer::from_writer(Vec::new());
    wtr.write_record(&columns)
        .map_err(|e| KobanaError::Internal(format!("CSV error: {e}")))?;

    for item in &items {
        let row: Vec<String> = columns
            .iter()
            .map(|col| {
                item.get(col)
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Null => String::new(),
                        other => other.to_string(),
                    })
                    .unwrap_or_default()
            })
            .collect();
        wtr.write_record(&row)
            .map_err(|e| KobanaError::Internal(format!("CSV error: {e}")))?;
    }

    let bytes = wtr
        .into_inner()
        .map_err(|e| KobanaError::Internal(format!("CSV error: {e}")))?;
    String::from_utf8(bytes).map_err(|e| KobanaError::Internal(format!("CSV encoding error: {e}")))
}

/// Extract a list of items from the response, handling data wrapper
fn extract_items(value: &serde_json::Value) -> Vec<&serde_json::Value> {
    // Try data wrapper first
    if let Some(data) = value.get("data") {
        if let Some(arr) = data.as_array() {
            return arr.iter().collect();
        }
        return vec![data];
    }

    if let Some(arr) = value.as_array() {
        return arr.iter().collect();
    }

    if value.is_object() {
        return vec![value];
    }

    vec![]
}
