pub mod boleto;
pub mod pix;

use kobana::client::KobanaClient;
use kobana::error::KobanaError;

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
