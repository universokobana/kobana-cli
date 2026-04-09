use kobana::client::{ApiRequest, ApiResponse, KobanaClient};
use kobana::error::KobanaError;
use kobana::spec::ResolvedEndpoint;

use crate::formatter::{filter_fields, format_output};

/// Handle paginated requests with --page-all
///
/// Emits NDJSON (one JSON line per page) for streaming consumption.
pub async fn paginate_all(
    client: &KobanaClient,
    endpoint: &ResolvedEndpoint,
    base_path: &str,
    base_params: &Option<serde_json::Value>,
    page_limit: usize,
    page_delay_ms: u64,
    fields: &Option<String>,
    output_format: &str,
    verbose: bool,
) -> Result<(), KobanaError> {
    let mut page = 1u64;
    let mut pages_fetched = 0usize;
    let per_page = base_params
        .as_ref()
        .and_then(|p| p.get("per_page"))
        .and_then(|v| v.as_u64())
        .unwrap_or(50);

    loop {
        if pages_fetched >= page_limit {
            eprintln!("Page limit reached ({page_limit})");
            break;
        }

        // Build params with current page
        let mut params = base_params.clone().unwrap_or(serde_json::json!({}));
        if let Some(obj) = params.as_object_mut() {
            obj.insert("page".to_string(), serde_json::json!(page));
            if !obj.contains_key("per_page") {
                obj.insert("per_page".to_string(), serde_json::json!(per_page));
            }
        }

        let request = ApiRequest {
            method: endpoint.http_method,
            path: base_path.to_string(),
            query_params: Some(params),
            body: None,
            idempotency_key: None,
        };

        if verbose {
            eprintln!(">> Page {page}: {} {}{}", endpoint.http_method, client.base_url(), base_path);
        }

        let response = client.execute(&request).await?;
        let items = extract_page_items(&response);
        let item_count = items.len();

        if verbose {
            eprintln!("<< Page {page}: {item_count} items (status {})", response.status);
        }

        if item_count == 0 {
            break;
        }

        // Apply field mask and output as NDJSON
        let output = if output_format == "json" {
            // NDJSON: one JSON per line for each item
            for item in &items {
                let filtered = if let Some(ref f) = fields {
                    filter_fields(item, f)
                } else {
                    item.clone()
                };
                println!("{}", serde_json::to_string(&filtered)?);
            }
            pages_fetched += 1;
            page += 1;

            if item_count < per_page as usize {
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

        print!("{output}");
        pages_fetched += 1;
        page += 1;

        if item_count < per_page as usize {
            break;
        }

        if page_delay_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(page_delay_ms)).await;
        }
    }

    if verbose {
        eprintln!("Pagination complete: {pages_fetched} pages fetched");
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
