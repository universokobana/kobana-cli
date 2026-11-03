pub mod boleto;
pub mod pix;

use kobana::client::KobanaClient;
use kobana::error::KobanaError;
use kobana::trf;

/// Trait for helper commands (multi-step workflows)
pub trait Helper: Send + Sync {
    /// Name of the helper (e.g., "+emitir")
    fn name(&self) -> &'static str;
    /// Slug of the product the helper lives under (`kobana <product> +verb`)
    fn product(&self) -> &'static str;
    /// Short description
    #[allow(dead_code)]
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
