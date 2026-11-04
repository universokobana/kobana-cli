use kobana::error::KobanaError;
use kobana::{tr, trf};

use crate::{jq, ui};

/// JSON as the CLI prints it: through `--jq` when one was given (one result
/// per line, indented only on a terminal; nothing when it yields nothing),
/// otherwise `value` itself, indented when `pretty`, on one line of its own.
/// `to_terminal` says where it goes. C1 control characters are escaped
/// (`kobana::error::escape_c1`): serde escapes only the C0 ones.
///
/// `committed` says the command already changed something (a write went
/// through, credentials were saved). `--jq` then runs in a child process with
/// a time limit, and if it fails, crashes or hangs, the command warns and
/// prints the unfiltered JSON, and still succeeds: a failure exit would
/// invite a retry that repeats the change.
pub fn json_text(
    value: &serde_json::Value,
    pretty: bool,
    to_terminal: bool,
    committed: bool,
) -> Result<String, KobanaError> {
    let plain = || -> Result<String, KobanaError> {
        let text = if pretty { serde_json::to_string_pretty(value)? } else { serde_json::to_string(value)? };
        Ok(kobana::error::escape_c1(&text) + "\n")
    };
    let Some(expr) = jq::expr() else {
        return plain();
    };
    let indent = pretty && to_terminal;
    if !committed {
        return jq::render(expr, value, indent, to_terminal);
    }
    match jq::render_isolated(expr, value, indent, to_terminal) {
        Ok(text) => Ok(text),
        Err(e) => {
            ui::warn(&trf!(
                pt: "{e}; a operação foi feita e a resposta vai sem o filtro",
                en: "{e}; the operation went through and the response follows unfiltered",
                es: "{e}; la operación se realizó y la respuesta va sin el filtro",
            ));
            plain()
        }
    }
}

/// Print JSON to stdout with [`json_text`]
pub fn print_json(value: &serde_json::Value, pretty: bool, committed: bool) -> Result<(), KobanaError> {
    let to_terminal = std::io::IsTerminal::is_terminal(&std::io::stdout());
    print!("{}", json_text(value, pretty, to_terminal, committed)?);
    Ok(())
}

/// Format output based on the requested format
pub fn format_output(value: &serde_json::Value, format: &str) -> Result<String, KobanaError> {
    match format {
        "json" => serde_json::to_string_pretty(value)
            .map_err(|e| KobanaError::Internal(trf!(
                pt: "erro ao formatar o JSON: {e}",
                en: "JSON formatting error: {e}",
                es: "error al formatear el JSON: {e}",
            ))),
        "table" => format_table(value, ui::screen_reader()),
        "csv" => format_csv(value),
        other => Err(KobanaError::Validation(trf!(
            pt: "formato de saída desconhecido '{other}'. Use: json, table, csv",
            en: "unknown output format '{other}'. Use: json, table, csv",
            es: "formato de salida desconocido '{other}'. Usa: json, table, csv",
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
        // An object may wrap its records in `data`: filter_object looks inside
        serde_json::Value::Object(_) => filter_object(value, &field_list),
        // Scalars have no fields to pick
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

/// `plain`: the screen-reader layout, see below
fn format_table(value: &serde_json::Value, plain: bool) -> Result<String, KobanaError> {
    use comfy_table::{Table, ContentArrangement};

    let items = extract_items(value);
    if items.is_empty() {
        return Ok(tr!(pt: "(vazio)", en: "(empty)", es: "(vacío)").to_string());
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
    if plain {
        // A screen reader reads borders out character by character, and a
        // cell wrapped onto a second line splits its record: no borders,
        // one line per record
        table.load_preset(comfy_table::presets::NOTHING);
        table.set_content_arrangement(ContentArrangement::Disabled);
    } else {
        table.set_content_arrangement(ContentArrangement::Dynamic);
    }
    table.set_header(&columns);

    for item in &items {
        let row: Vec<String> = columns
            .iter()
            .map(|col| {
                item.get(col).map(|v| cell_text(v, false)).unwrap_or_default()
            })
            .collect();
        table.add_row(row);
    }

    Ok(table.to_string())
}

/// A value as one cell of a table or CSV. API strings are not trusted: a
/// payer's name or an e-mail subject is written by whoever sent it. Control
/// characters (an escape sequence that would move the cursor, recolour the
/// screen or plant an OSC hyperlink) become U+FFFD; in a CSV, a cell that a
/// spreadsheet would read as a formula (`=`, `+`, `-`, `@`) gets a leading
/// `'`, so it stays text. JSON output escapes them instead: serde the C0
/// ones and [`json_text`] the C1 ones; nothing evaluates it.
fn cell_text(value: &serde_json::Value, csv: bool) -> String {
    let text = match value {
        serde_json::Value::String(s) => s
            .chars()
            .map(|c| if c.is_control() || ('\u{80}'..='\u{9f}').contains(&c) { '\u{FFFD}' } else { c })
            .collect::<String>(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    };
    let formula = csv
        && matches!(value, serde_json::Value::String(_))
        && text.starts_with(['=', '+', '-', '@']);
    if formula {
        format!("'{text}")
    } else {
        text
    }
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
        .map_err(|e| KobanaError::Internal(trf!(pt: "erro no CSV: {e}", en: "CSV error: {e}", es: "error en el CSV: {e}")))?;

    for item in &items {
        let row: Vec<String> = columns
            .iter()
            .map(|col| {
                item.get(col).map(|v| cell_text(v, true)).unwrap_or_default()
            })
            .collect();
        wtr.write_record(&row)
            .map_err(|e| KobanaError::Internal(trf!(pt: "erro no CSV: {e}", en: "CSV error: {e}", es: "error en el CSV: {e}")))?;
    }

    let bytes = wtr
        .into_inner()
        .map_err(|e| KobanaError::Internal(trf!(pt: "erro no CSV: {e}", en: "CSV error: {e}", es: "error en el CSV: {e}")))?;
    String::from_utf8(bytes).map_err(|e| KobanaError::Internal(trf!(
        pt: "erro de codificação no CSV: {e}",
        en: "CSV encoding error: {e}",
        es: "error de codificación en el CSV: {e}",
    )))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_keep_escape_sequences_and_formulas_out_of_the_terminal_and_the_spreadsheet() {
        let hostile = serde_json::json!("\u{1b}]8;;https://evil\u{1b}\\Pay here\u{1b}]8;;\u{1b}\\ \u{85}x");
        let table = cell_text(&hostile, false);
        assert!(!table.contains('\u{1b}') && !table.contains('\u{85}'), "{table:?}");
        assert!(table.contains("Pay here"));

        assert_eq!(cell_text(&serde_json::json!("=HYPERLINK(\"https://evil\")"), true), "'=HYPERLINK(\"https://evil\")");
        assert_eq!(cell_text(&serde_json::json!("+55 11 99999"), true), "'+55 11 99999");
        assert_eq!(cell_text(&serde_json::json!("-10"), true), "'-10");
        // A table is not a spreadsheet, and a JSON number is not a formula
        assert_eq!(cell_text(&serde_json::json!("=x"), false), "=x");
        assert_eq!(cell_text(&serde_json::json!(-10.5), true), "-10.5");
        assert_eq!(cell_text(&serde_json::Value::Null, true), "");
    }

    #[test]
    fn the_screen_reader_table_has_no_borders_and_one_line_per_record() {
        let long = "x".repeat(400);
        let value = serde_json::json!({"data": [{"id": 1, "note": long}, {"id": 2, "note": "b"}]});
        let table = format_table(&value, true).unwrap();
        assert_eq!(table.lines().count(), 3, "header + 2 records: {table}");
        assert!(!table.chars().any(|c| "|+-=│─┼╞╡═".contains(c)), "{table}");
        assert!(table.lines().next().unwrap().contains("id"), "{table}");
        // The default keeps its borders
        assert!(format_table(&value, false).unwrap().contains('+'));
    }

    #[test]
    fn csv_and_table_go_through_the_same_cell_filter() {
        let value = serde_json::json!({"data": [{"name": "=cmd", "note": "a\u{1b}[31mb"}]});
        let csv = format_output(&value, "csv").unwrap();
        assert!(csv.contains("'=cmd") && !csv.contains('\u{1b}'), "{csv:?}");
        let table = format_output(&value, "table").unwrap();
        assert!(table.contains("=cmd") && !table.contains("'=cmd") && !table.contains('\u{1b}'), "{table:?}");
        // JSON escapes the control instead (serde) and nothing evaluates it
        let json = format_output(&value, "json").unwrap();
        assert!(json.contains("\\u001b") && json.contains("\"=cmd\""));
    }
}
