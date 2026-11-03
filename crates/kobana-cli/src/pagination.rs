use kobana::client::{ApiRequest, ApiResponse, KobanaClient};
use kobana::error::KobanaError;
use kobana::trf;
use kobana::spec::{ParameterLocation, ResolvedEndpoint};

use std::io::Write;

use crate::executor::Output;
use crate::formatter::{filter_fields, format_output};

/// How an endpoint pages its results, read from the parameters its spec declares
#[derive(Debug, PartialEq, Eq)]
enum Strategy {
    /// `page` + `per_page`; the last page is the one shorter than `per_page`
    PageNumber,
    /// `cursor` in, `meta.next_cursor` out; the last page has no next cursor
    Cursor,
}

fn strategy(endpoint: &ResolvedEndpoint) -> Strategy {
    let declares_cursor = endpoint
        .operation
        .parameters
        .iter()
        .any(|p| p.location == ParameterLocation::Query && p.name == "cursor");
    if declares_cursor {
        Strategy::Cursor
    } else {
        Strategy::PageNumber
    }
}

/// The cursor of the next page: `meta.next_cursor`, or a top-level `next_cursor`.
/// `None` (absent, null or empty) means this was the last page.
fn next_cursor(body: &serde_json::Value) -> Option<String> {
    body.pointer("/meta/next_cursor")
        .or_else(|| body.get("next_cursor"))
        .and_then(|v| v.as_str())
        .filter(|c| !c.is_empty())
        .map(str::to_string)
}

/// Where `--page-all` writes: the `--output` file, or stdout. The file is
/// created on the first write, so a run that fails before any page arrives
/// leaves an existing file alone, as a single request does.
struct Sink {
    path: Option<String>,
    inner: Option<Box<dyn Write>>,
}

impl Sink {
    fn new(path: Option<&str>) -> Self {
        Self { path: path.map(String::from), inner: None }
    }

    fn writer(&mut self) -> std::io::Result<&mut Box<dyn Write>> {
        if self.inner.is_none() {
            let writer: Box<dyn Write> = match &self.path {
                Some(path) => Box::new(std::io::BufWriter::new(std::fs::File::create(path)?)),
                None => Box::new(std::io::stdout().lock()),
            };
            self.inner = Some(writer);
        }
        Ok(self.inner.as_mut().expect("just set"))
    }

    fn write(&mut self, text: &str) -> Result<(), KobanaError> {
        self.writer()
            .and_then(|w| w.write_all(text.as_bytes()))
            .map_err(|e| self.failed(e))
    }

    fn finish(mut self) -> Result<(), KobanaError> {
        let flushed = match self.inner.as_mut() {
            Some(w) => w.flush(),
            None => Ok(()),
        };
        flushed.map_err(|e| self.failed(e))?;
        if let (Some(path), Some(_)) = (&self.path, &self.inner) {
            eprintln!(
                "{}",
                trf!(
                    pt: "Resposta salva em {path}",
                    en: "Response saved to {path}",
                    es: "Respuesta guardada en {path}",
                )
            );
        }
        Ok(())
    }

    fn failed(&self, e: std::io::Error) -> KobanaError {
        let path = self.path.as_deref().unwrap_or("stdout");
        KobanaError::Internal(trf!(
            pt: "não foi possível gravar em {path}: {e}",
            en: "failed to write to {path}: {e}",
            es: "no se pudo guardar en {path}: {e}",
        ))
    }
}

/// Handle paginated requests with --page-all
///
/// Emits NDJSON (one JSON line per item) for streaming consumption, to stdout
/// or the `--output` file. Endpoints that declare a `cursor` query parameter
/// follow `meta.next_cursor`; the rest walk `page`/`per_page`.
#[allow(clippy::too_many_arguments)]
pub async fn paginate_all(
    client: &KobanaClient,
    endpoint: &ResolvedEndpoint,
    base_path: &str,
    base_params: &Option<serde_json::Value>,
    headers: &[(String, String)],
    page_limit: usize,
    page_delay_ms: u64,
    output: &Output,
) -> Result<(), KobanaError> {
    let (fields, output_format, verbose) = (&output.fields, output.format.as_str(), output.verbose);
    let mut sink = Sink::new(output.path.as_deref());
    let strategy = strategy(endpoint);
    let mut page = 1u64;
    let mut cursor: Option<String> = None;
    let mut pages_fetched = 0usize;
    let per_page = base_params
        .as_ref()
        .and_then(|p| p.get("per_page"))
        .and_then(|v| v.as_u64())
        .unwrap_or(50);

    loop {
        if pages_fetched >= page_limit {
            eprintln!(
                "{}",
                trf!(
                    pt: "Limite de páginas atingido ({page_limit})",
                    en: "Page limit reached ({page_limit})",
                    es: "Límite de páginas alcanzado ({page_limit})",
                )
            );
            break;
        }

        // Build params with current page
        let mut params = base_params.clone().unwrap_or(serde_json::json!({}));
        if let Some(obj) = params.as_object_mut() {
            match strategy {
                Strategy::PageNumber => {
                    obj.insert("page".to_string(), serde_json::json!(page));
                    if !obj.contains_key("per_page") {
                        obj.insert("per_page".to_string(), serde_json::json!(per_page));
                    }
                }
                // The first request keeps a cursor the caller passed in --params
                Strategy::Cursor => {
                    if let Some(c) = &cursor {
                        obj.insert("cursor".to_string(), serde_json::json!(c));
                    }
                }
            }
        }

        let request = ApiRequest {
            method: endpoint.http_method,
            path: base_path.to_string(),
            query_params: Some(params),
            body: None,
            idempotency_key: None,
            headers: headers.to_vec(),
            multipart: false,
            files: Vec::new(),
        };

        if verbose {
            let (method, base_url) = (endpoint.http_method, client.base_url());
            eprintln!(
                "{}",
                trf!(
                    pt: ">> Página {page}: {method} {base_url}{base_path}",
                    en: ">> Page {page}: {method} {base_url}{base_path}",
                    es: ">> Página {page}: {method} {base_url}{base_path}",
                )
            );
        }

        let response = client.execute(&request).await?;
        let items = extract_page_items(&response);
        let item_count = items.len();
        // Whether another page follows this one
        let has_more = match strategy {
            Strategy::PageNumber => item_count >= per_page as usize,
            Strategy::Cursor => {
                cursor = next_cursor(&response.body);
                cursor.is_some()
            }
        };

        if verbose {
            let status = response.status;
            eprintln!(
                "{}",
                trf!(
                    pt: "<< Página {page}: {item_count} itens (status {status})",
                    en: "<< Page {page}: {item_count} items (status {status})",
                    es: "<< Página {page}: {item_count} elementos (estado {status})",
                )
            );
        }

        if item_count == 0 {
            break;
        }

        // Apply field mask and output as NDJSON
        let formatted = if output_format == "json" {
            // NDJSON: one JSON per line for each item
            for item in &items {
                let filtered = if let Some(ref f) = fields {
                    filter_fields(item, f)
                } else {
                    item.clone()
                };
                sink.write(&format!("{}\n", serde_json::to_string(&filtered)?))?;
            }
            pages_fetched += 1;
            page += 1;

            if !has_more {
                break;
            }

            // Delay between pages
            if page_delay_ms > 0 {
                tokio::time::sleep(std::time::Duration::from_millis(page_delay_ms)).await;
            }
            continue;
        } else {
            // For table/csv, collect all items first
            let all_items: serde_json::Value = serde_json::Value::Array(
                items
                    .iter()
                    .map(|item| {
                        if let Some(ref f) = fields {
                            filter_fields(item, f)
                        } else {
                            item.clone()
                        }
                    })
                    .collect(),
            );
            format_output(&all_items, output_format)?
        };

        sink.write(&formatted)?;
        pages_fetched += 1;
        page += 1;

        if !has_more {
            break;
        }

        if page_delay_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(page_delay_ms)).await;
        }
    }

    sink.finish()?;

    if verbose {
        eprintln!(
            "{}",
            trf!(
                pt: "Paginação concluída: {pages_fetched} páginas buscadas",
                en: "Pagination complete: {pages_fetched} pages fetched",
                es: "Paginación completada: {pages_fetched} páginas obtenidas",
            )
        );
    }

    Ok(())
}

/// Extract items from a page response, handling various response formats
fn extract_page_items(response: &ApiResponse) -> Vec<serde_json::Value> {
    let body = &response.body;

    // Try "data" wrapper (Kobana v2 pattern)
    if let Some(data) = body.get("data") {
        if let Some(arr) = data.as_array() {
            return arr.clone();
        }
    }

    // Try direct array
    if let Some(arr) = body.as_array() {
        return arr.clone();
    }

    // Single item
    if body.is_object() && !body.as_object().unwrap().is_empty() {
        return vec![body.clone()];
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use kobana::spec::ApiSpec;

    #[test]
    fn the_output_file_is_written_only_once_a_page_arrives() {
        let path = std::env::temp_dir().join(format!("kobana-page-all-{}.ndjson", std::process::id()));
        std::fs::write(&path, "previous").unwrap();
        let shown = path.display().to_string();

        // Nothing written: the file stays as it was
        Sink::new(Some(&shown)).finish().unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "previous");

        let mut sink = Sink::new(Some(&shown));
        sink.write("{\"id\":1}\n").unwrap();
        sink.write("{\"id\":2}\n").unwrap();
        sink.finish().unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "{\"id\":1}\n{\"id\":2}\n");
        std::fs::remove_file(path).ok();
    }

    fn endpoint(params: &str) -> ResolvedEndpoint {
        let spec = format!(
            r#"{{"info": {{"version": "1"}}, "paths": {{"/v1/things": {{"get": {{
                "parameters": {params}, "responses": {{}}}}}}}}}}"#
        );
        let tree = ApiSpec::parse(&spec).unwrap().build_command_tree("/v1");
        tree.children["things"].endpoints[0].clone()
    }

    #[test]
    fn an_endpoint_that_declares_a_cursor_pages_by_cursor() {
        let cursor =
            endpoint(r#"[{"name": "cursor", "in": "query"}, {"name": "limit", "in": "query"}]"#);
        assert_eq!(strategy(&cursor), Strategy::Cursor);

        let paged = endpoint(r#"[{"name": "page", "in": "query"}]"#);
        assert_eq!(strategy(&paged), Strategy::PageNumber);

        // A header that happens to be called `cursor` does not count
        let header = endpoint(r#"[{"name": "cursor", "in": "header"}]"#);
        assert_eq!(strategy(&header), Strategy::PageNumber);
    }

    #[test]
    fn next_cursor_ends_on_null_absent_or_empty() {
        let next = serde_json::json!({"data": [], "meta": {"next_cursor": "abc", "limit": 20}});
        assert_eq!(next_cursor(&next).as_deref(), Some("abc"));
        let top = serde_json::json!({"data": [], "next_cursor": "xyz"});
        assert_eq!(next_cursor(&top).as_deref(), Some("xyz"));

        for last in [
            serde_json::json!({"data": [], "meta": {"next_cursor": null}}),
            serde_json::json!({"data": [], "meta": {"next_cursor": ""}}),
            serde_json::json!({"data": []}),
        ] {
            assert_eq!(next_cursor(&last), None, "{last}");
        }
    }
}
