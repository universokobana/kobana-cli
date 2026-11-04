pub mod boleto;
pub mod pix;

use kobana::client::{ApiRequest, KobanaClient};
use kobana::error::KobanaError;
use kobana::spec::HttpMethod;
use kobana::trf;

use crate::executor;

/// Trait for helper commands (multi-step workflows)
pub trait Helper: Send + Sync {
    /// Name of the helper (e.g., "+emitir")
    fn name(&self) -> &'static str;
    /// Slug of the product the helper lives under (`kobana <product> +verb`)
    fn product(&self) -> &'static str;
    /// Short description
    fn about(&self) -> &'static str;
    /// Build the clap Command for this helper
    fn command(&self) -> clap::Command;
    /// Execute the helper
    fn execute<'a>(
        &'a self,
        client: &'a KobanaClient,
        matches: &'a clap::ArgMatches,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), KobanaError>> + Send + 'a>>;
}

/// Registry of all available helpers
pub fn all_helpers() -> Vec<Box<dyn Helper>> {
    vec![
        Box::new(boleto::EmitirBoleto),
        Box::new(boleto::CancelarLoteBoletos),
        Box::new(pix::CobrarPix),
    ]
}

/// The helpers that live under a product
pub fn for_product(product: &str) -> Vec<Box<dyn Helper>> {
    all_helpers().into_iter().filter(|h| h.product() == product).collect()
}

/// Find a product's helper by name
pub fn find_helper(product: &str, name: &str) -> Option<Box<dyn Helper>> {
    for_product(product).into_iter().find(|h| h.name() == name)
}

/// Global options a helper has no use for: refused, not silently dropped
pub(crate) fn reject_options(matches: &clap::ArgMatches, helper: &str, options: &[&str]) -> Result<(), KobanaError> {
    for option in options {
        let given = matches
            .value_source(option)
            .is_some_and(|source| source == clap::parser::ValueSource::CommandLine);
        if given {
            return Err(KobanaError::Validation(trf!(
                pt: "{helper} não usa --{option}",
                en: "{helper} does not take --{option}",
                es: "{helper} no usa --{option}",
            )));
        }
    }
    Ok(())
}

/// `--json` fields the flags do not set, added to a helper's body (`payer`
/// email, `tags`…). A flag wins over the same key in `--json`.
pub(crate) fn merge_json_body(matches: &clap::ArgMatches, body: &mut serde_json::Value) -> Result<(), KobanaError> {
    let Some(extra) = executor::json_arg(matches, "json")? else {
        return Ok(());
    };
    crate::validate::validate_body(&extra)?;
    let serde_json::Value::Object(extra) = extra else {
        return Err(KobanaError::Validation(
            kobana::tr!(
                pt: "--json precisa ser um objeto",
                en: "--json must be an object",
                es: "--json debe ser un objeto",
            )
            .into(),
        ));
    };
    let body = body.as_object_mut().expect("helper bodies are objects");
    for (key, value) in extra {
        body.entry(key).or_insert(value);
    }
    Ok(())
}

/// A POST of one body, the way a dynamic command sends it: `--dry-run`,
/// `--idempotency-key`, `--verbose`, `--fields`, `--output-format`, `--output`
pub(crate) async fn post(
    client: &KobanaClient,
    matches: &clap::ArgMatches,
    path: &str,
    body: serde_json::Value,
) -> Result<(), KobanaError> {
    let request = ApiRequest {
        method: HttpMethod::Post,
        path: path.to_string(),
        query_params: None,
        body: Some(body),
        idempotency_key: executor::idempotency_key(matches, HttpMethod::Post),
        headers: Vec::new(),
        multipart: false,
        files: Vec::new(),
    };
    let output = executor::Output::from_matches(matches)?.committed(true);
    if matches.get_flag("dry-run") {
        return crate::formatter::print_json(&executor::dry_run_report(client, &request), true, false);
    }
    let response = executor::send(client, &request, output.verbose).await?;
    output.print(&response)
}

/// An amount in reais from a helper flag (`--valor 150.50`). A value that does
/// not parse is refused: falling back to 0 would issue a charge of R$ 0,00.
pub(crate) fn parse_amount(value: &str, flag: &str) -> Result<f64, KobanaError> {
    match value.trim().parse::<f64>() {
        Ok(n) if n.is_finite() && n > 0.0 => Ok(n),
        _ if value.contains(',') => Err(KobanaError::Validation(trf!(
            pt: "{flag} inválido: '{value}' (use ponto como separador decimal, ex.: 150.50)",
            en: "invalid {flag}: '{value}' (use a point as the decimal separator, e.g. 150.50)",
            es: "{flag} inválido: '{value}' (use punto como separador decimal, ej.: 150.50)",
        ))),
        _ => Err(KobanaError::Validation(trf!(
            pt: "{flag} inválido: '{value}' (esperado um valor positivo, ex.: 150.50)",
            en: "invalid {flag}: '{value}' (expected a positive amount, e.g. 150.50)",
            es: "{flag} inválido: '{value}' (se esperaba un monto positivo, ej.: 150.50)",
        ))),
    }
}

/// An integer id from a helper flag (`--carteira 12`)
pub(crate) fn parse_id(value: &str, flag: &str) -> Result<i64, KobanaError> {
    value.trim().parse::<i64>().ok().filter(|n| *n > 0).ok_or_else(|| {
        KobanaError::Validation(trf!(
            pt: "{flag} inválido: '{value}' (esperado um número inteiro positivo)",
            en: "invalid {flag}: '{value}' (expected a positive integer)",
            es: "{flag} inválido: '{value}' (se esperaba un número entero positivo)",
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The matches a helper gets: parsed from the root, global options included
    fn helper_matches(args: &[&str]) -> clap::ArgMatches {
        let products = crate::product::load_all().unwrap();
        let root = crate::commands::build_root_command(&products).try_get_matches_from(args).unwrap();
        let (_, banking) = root.subcommand().unwrap();
        banking.subcommand().unwrap().1.clone()
    }

    #[test]
    fn helpers_see_the_global_options() {
        let m = helper_matches(&[
            "kobana", "banking", "+cobrar", "--valor", "1", "--vencimento", "x", "--conta-pix", "U",
            "--dry-run", "--fields", "uid", "--output-format", "table", "--idempotency-key", "K",
        ]);
        assert!(m.get_flag("dry-run"));
        let output = executor::Output::from_matches(&m).unwrap();
        assert_eq!((output.fields.as_deref(), output.format.as_str()), (Some("uid"), "table"));
        assert_eq!(executor::idempotency_key(&m, HttpMethod::Post).as_deref(), Some("K"));
    }

    #[test]
    fn json_adds_fields_the_flags_do_not_set() {
        let m = helper_matches(&[
            "kobana", "banking", "+cobrar", "--valor", "1", "--vencimento", "x", "--conta-pix", "U",
            "--json", r#"{"amount": 99, "tags": ["a"]}"#,
        ]);
        let mut body = serde_json::json!({"amount": 1.0});
        merge_json_body(&m, &mut body).unwrap();
        assert_eq!(body, serde_json::json!({"amount": 1.0, "tags": ["a"]}));
    }

    #[test]
    fn options_a_helper_does_not_use_are_refused() {
        let m = helper_matches(&["kobana", "banking", "+cancelar-lote", "--ids", "1", "--params", "{}"]);
        assert!(matches!(reject_options(&m, "+cancelar-lote", &["params"]), Err(KobanaError::Validation(_))));
        let m = helper_matches(&["kobana", "banking", "+cancelar-lote", "--ids", "1"]);
        assert!(reject_options(&m, "+cancelar-lote", &["params", "json", "file"]).is_ok());
    }

    #[test]
    fn amounts_that_do_not_parse_are_refused() {
        assert_eq!(parse_amount("150.50", "--valor").unwrap(), 150.5);
        assert_eq!(parse_amount(" 10 ", "--valor").unwrap(), 10.0);
        for bad in ["150,50", "abc", "", "0", "-1", "NaN", "inf"] {
            let err = parse_amount(bad, "--valor").unwrap_err();
            assert!(matches!(err, KobanaError::Validation(_)), "{bad}");
        }
    }

    #[test]
    fn ids_that_do_not_parse_are_refused() {
        assert_eq!(parse_id("12", "--carteira").unwrap(), 12);
        for bad in ["zz", "", "0", "-3", "1.5"] {
            assert!(matches!(parse_id(bad, "--carteira"), Err(KobanaError::Validation(_))), "{bad}");
        }
    }
}
