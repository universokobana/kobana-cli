use clap::{Arg, Command};
use kobana::client::KobanaClient;
use kobana::error::KobanaError;
use kobana::tr;

use super::Helper;
use crate::clap_i18n;

/// Helper: +cobrar — Create a Pix charge with simplified interface
pub struct CobrarPix;

impl Helper for CobrarPix {
    fn name(&self) -> &'static str {
        "+cobrar"
    }

    fn product(&self) -> &'static str {
        "banking"
    }

    fn about(&self) -> &'static str {
        tr!(
            pt: "Criar cobrança Pix com interface simplificada",
            en: "Create a Pix charge with a simplified interface",
            es: "Crear un cobro Pix con una interfaz simplificada",
        )
    }

    fn command(&self) -> Command {
        clap_i18n::command(
            "+cobrar",
            format!(
                "kobana banking +cobrar --valor <VALOR> --vencimento <DATA> --conta-pix <UID> {}",
                clap_i18n::options_placeholder()
            ),
        )
            .about(self.about())
            .arg(Arg::new("valor").long("valor").required(true).help(tr!(pt: "Valor da cobrança", en: "Charge amount", es: "Monto del cobro")).value_name("VALOR"))
            .arg(Arg::new("vencimento").long("vencimento").required(true).help(tr!(
                pt: "Data e hora de vencimento, ISO 8601 (ex.: 2026-12-02T10:00:00-03:00)",
                en: "Due date and time, ISO 8601 (e.g. 2026-12-02T10:00:00-03:00)",
                es: "Fecha y hora de vencimiento, ISO 8601 (ej.: 2026-12-02T10:00:00-03:00)",
            )).value_name("DATA"))
            .arg(Arg::new("conta-pix").long("conta-pix").required(true).help(tr!(pt: "UID da conta Pix", en: "Pix account UID", es: "UID de la cuenta Pix")).value_name("UID"))
            .arg(Arg::new("nome").long("nome").requires("cpf-cnpj").help(tr!(pt: "Nome do pagador (com --cpf-cnpj)", en: "Payer name (with --cpf-cnpj)", es: "Nombre del pagador (con --cpf-cnpj)")).value_name("NOME"))
            .arg(Arg::new("cpf-cnpj").long("cpf-cnpj").requires("nome").help(tr!(pt: "CPF ou CNPJ do pagador (com --nome)", en: "Payer CPF or CNPJ (with --nome)", es: "CPF o CNPJ del pagador (con --nome)")).value_name("DOC"))
            .arg(Arg::new("descricao").long("descricao").help(tr!(pt: "Descrição da cobrança", en: "Charge description", es: "Descripción del cobro")).value_name("TEXT"))
    }

    fn execute<'a>(
        &'a self,
        client: &'a KobanaClient,
        matches: &'a clap::ArgMatches,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), KobanaError>> + Send + 'a>> {
        Box::pin(async move {
            super::reject_options(matches, "+cobrar", &["params", "file"])?;
            let mut body = request_body(matches)?;
            super::merge_json_body(matches, &mut body)?;
            super::post(client, matches, "/v2/charge/pix", body).await
        })
    }
}

/// The `POST /v2/charge/pix` body the flags describe
fn request_body(matches: &clap::ArgMatches) -> Result<serde_json::Value, KobanaError> {
    let arg = |name: &str| matches.get_one::<String>(name).cloned();
    let valor = super::parse_amount(&arg("valor").unwrap(), "--valor")?;

    let mut body = serde_json::json!({
        "amount": valor,
        "pix_account_uid": arg("conta-pix").unwrap(),
        "expire_at": arg("vencimento").unwrap(),
    });

    // The API takes the payer whole: name and document together
    if let (Some(name), Some(document)) = (arg("nome"), arg("cpf-cnpj")) {
        body["payer"] = serde_json::json!({
            "name": name,
            "document_number": document,
        });
    }

    if let Some(desc) = arg("descricao") {
        body["description"] = serde_json::json!(desc);
    }

    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<clap::ArgMatches, clap::Error> {
        CobrarPix.command().try_get_matches_from(std::iter::once("+cobrar").chain(args.iter().copied()))
    }

    const REQUIRED: [&str; 6] = ["--valor", "10.5", "--vencimento", "2026-12-01T10:00:00-03:00", "--conta-pix", "U"];

    #[test]
    fn body_follows_the_v2_pix_schema() {
        let mut args = REQUIRED.to_vec();
        args.extend(["--nome", "João", "--cpf-cnpj", "111.321.322-09"]);
        let body = request_body(&parse(&args).unwrap()).unwrap();

        assert_eq!(
            body,
            serde_json::json!({
                "amount": 10.5,
                "pix_account_uid": "U",
                "expire_at": "2026-12-01T10:00:00-03:00",
                "payer": {"name": "João", "document_number": "111.321.322-09"},
            })
        );
    }

    #[test]
    fn an_amount_that_does_not_parse_is_refused() {
        let mut args = REQUIRED.to_vec();
        args[1] = "10,50";
        assert!(matches!(request_body(&parse(&args).unwrap()), Err(KobanaError::Validation(_))));
    }

    #[test]
    fn due_date_is_required_and_the_payer_comes_whole() {
        assert!(parse(&["--valor", "1", "--conta-pix", "U"]).is_err(), "no --vencimento");

        let mut only_name = REQUIRED.to_vec();
        only_name.extend(["--nome", "João"]);
        assert!(parse(&only_name).is_err(), "--nome without --cpf-cnpj");

        let mut only_doc = REQUIRED.to_vec();
        only_doc.extend(["--cpf-cnpj", "1"]);
        assert!(parse(&only_doc).is_err(), "--cpf-cnpj without --nome");

        let body = request_body(&parse(&REQUIRED).unwrap()).unwrap();
        assert!(body.get("payer").is_none());
    }
}
