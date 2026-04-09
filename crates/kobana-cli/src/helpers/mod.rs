pub mod boleto;
pub mod pix;

use kobana::client::KobanaClient;
use kobana::error::KobanaError;

/// Trait for helper commands (multi-step workflows)
pub trait Helper: Send + Sync {
    /// Name of the helper (e.g., "+emitir")
    fn name(&self) -> &'static str;
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

/// Find a helper by name
pub fn find_helper(name: &str) -> Option<Box<dyn Helper>> {
    all_helpers().into_iter().find(|h| h.name() == name)
}
