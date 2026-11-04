use clap::{Arg, Command};
use kobana::client::{ApiRequest, KobanaClient};
use kobana::error::KobanaError;
use kobana::{tr, trf};
use kobana::spec::HttpMethod;

use super::Helper;
use crate::clap_i18n;
use crate::executor;

/// Helper: +emitir — Create a bank billet with simplified interface
pub struct EmitirBoleto;

impl Helper for EmitirBoleto {
    fn name(&self) -> &'static str {
        "+emitir"
    }

    fn product(&self) -> &'static str {
        "banking"
    }

    fn about(&self) -> &'static str {
        tr!(
            pt: "Emitir boleto com interface simplificada",
            en: "Issue a bank slip with a simplified interface",
            es: "Emitir un boleto con una interfaz simplificada",
        )
    }

    fn command(&self) -> Command {
        clap_i18n::command(
            "+emitir",
            format!(
                "kobana banking +emitir --valor <VALOR> --vencimento <DATA> --nome <NOME> --cpf-cnpj <DOC> --carteira <ID> {}",
                clap_i18n::options_placeholder()
            ),
        )
            .about(self.about())
            .arg(Arg::new("valor").long("valor").required(true).help(tr!(pt: "Valor do boleto (ex.: 150.50)", en: "Bank slip amount (e.g. 150.50)", es: "Monto del boleto (ej.: 150.50)")).value_name("VALOR"))
            .arg(Arg::new("vencimento").long("vencimento").required(true).help(tr!(pt: "Data de vencimento (AAAA-MM-DD)", en: "Due date (YYYY-MM-DD)", es: "Fecha de vencimiento (AAAA-MM-DD)")).value_name("DATA"))
            .arg(Arg::new("nome").long("nome").required(true).help(tr!(pt: "Nome do pagador", en: "Payer name", es: "Nombre del pagador")).value_name("NOME"))
            .arg(Arg::new("cpf-cnpj").long("cpf-cnpj").required(true).help(tr!(pt: "CPF ou CNPJ do pagador", en: "Payer CPF or CNPJ", es: "CPF o CNPJ del pagador")).value_name("DOC"))
            .arg(Arg::new("carteira").long("carteira").required(true).help(tr!(pt: "ID da carteira de cobrança", en: "Billing wallet ID", es: "ID de la cartera de cobro")).value_name("ID"))
            .arg(Arg::new("descricao").long("descricao").help(tr!(pt: "Descrição/instruções do boleto", en: "Bank slip description/instructions", es: "Descripción/instrucciones del boleto")).value_name("TEXT"))
    }

    fn execute<'a>(
        &'a self,
        client: &'a KobanaClient,
        matches: &'a clap::ArgMatches,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), KobanaError>> + Send + 'a>> {
        Box::pin(async move {
            super::reject_options(matches, "+emitir", &["params", "file"])?;
            let mut body = request_body(matches)?;
            super::merge_json_body(matches, &mut body)?;
            super::post(client, matches, "/v1/bank_billets", body).await
        })
    }
}

/// The `POST /v1/bank_billets` body the flags describe
fn request_body(matches: &clap::ArgMatches) -> Result<serde_json::Value, KobanaError> {
    let arg = |name: &str| matches.get_one::<String>(name).cloned();

    let mut body = serde_json::json!({
        "amount": super::parse_amount(&arg("valor").unwrap(), "--valor")?,
        "expire_at": arg("vencimento").unwrap(),
        "customer_person_name": arg("nome").unwrap(),
        "customer_cnpj_cpf": arg("cpf-cnpj").unwrap(),
        "bank_billet_account_id": super::parse_id(&arg("carteira").unwrap(), "--carteira")?,
    });

    if let Some(desc) = arg("descricao") {
        body["description"] = serde_json::json!(desc);
    }

    Ok(body)
}

/// Helper: +cancelar-lote — Cancel multiple bank billets
pub struct CancelarLoteBoletos;

impl Helper for CancelarLoteBoletos {
    fn name(&self) -> &'static str {
        "+cancelar-lote"
    }

    fn product(&self) -> &'static str {
        "banking"
    }

    fn about(&self) -> &'static str {
        tr!(
            pt: "Cancelar múltiplos boletos de uma vez",
            en: "Cancel several bank slips at once",
            es: "Cancelar varios boletos a la vez",
        )
    }

    fn command(&self) -> Command {
        clap_i18n::command(
            "+cancelar-lote",
            format!("kobana banking +cancelar-lote --ids <IDS> {}", clap_i18n::options_placeholder()),
        )
            .about(self.about())
            .arg(Arg::new("ids").long("ids").required(true).help(tr!(pt: "IDs dos boletos separados por vírgula", en: "Comma-separated bank slip IDs", es: "IDs de los boletos separados por comas")).value_name("IDS"))
    }

    fn execute<'a>(
        &'a self,
        client: &'a KobanaClient,
        matches: &'a clap::ArgMatches,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), KobanaError>> + Send + 'a>> {
        Box::pin(async move {
            super::reject_options(matches, "+cancelar-lote", &["params", "json", "file"])?;
            let ids = parse_ids(matches.get_one::<String>("ids").unwrap())?;
            let output = executor::Output::from_matches(matches)?.committed(true);
            let requests: Vec<ApiRequest> = ids
                .iter()
                .map(|id| ApiRequest {
                    method: HttpMethod::Put,
                    path: format!("/v1/bank_billets/{id}/cancel"),
                    query_params: None,
                    body: None,
                    idempotency_key: None,
                    headers: Vec::new(),
                    multipart: false,
                    files: Vec::new(),
                })
                .collect();

            // One dry-run report per request, as each would be sent
            if matches.get_flag("dry-run") {
                let reports: Vec<_> = requests.iter().map(|r| executor::dry_run_report(client, r)).collect();
                return crate::formatter::print_json(&serde_json::Value::Array(reports), true, false);
            }

            let mut results = Vec::new();
            let mut failed_status = None;
            for (id, request) in ids.iter().zip(&requests) {
                match executor::send(client, request, output.verbose).await {
                    Ok(response) => {
                        results.push(serde_json::json!({
                            "id": id,
                            "status": "cancelled",
                            "response": response,
                        }));
                    }
                    Err(e) => {
                        let status = match &e {
                            KobanaError::Api { status, .. } => Some(*status),
                            _ => None,
                        };
                        failed_status.get_or_insert(status);
                        results.push(serde_json::json!({
                            "id": id,
                            "status": "error",
                            "status_code": status,
                            "error": e.to_string(),
                        }));
                    }
                }
            }

            batch_outcome(results, failed_status, &output)
        })
    }
}

/// Print the per-id results when every cancel went through. When any failed,
/// the run is an API error (exit 1) carrying the same results in `details`, so
/// a script sees the failure without parsing the array. `failed_status` is the
/// HTTP status of the first failure (`None` inside it when no response came).
fn batch_outcome(
    results: Vec<serde_json::Value>,
    failed_status: Option<Option<u16>>,
    output: &executor::Output,
) -> Result<(), KobanaError> {
    let Some(status) = failed_status else {
        return output.print(&serde_json::Value::Array(results));
    };
    let total = results.len();
    let failed = results.iter().filter(|r| r["status"] == "error").count();
    Err(KobanaError::Api {
        // No HTTP response (network error): 0, as no status was received
        status: status.unwrap_or(0),
        message: trf!(
            pt: "{failed} de {total} cancelamentos falharam",
            en: "{failed} of {total} cancellations failed",
            es: "{failed} de {total} cancelaciones fallaron",
        ),
        body: Some(serde_json::Value::Array(results)),
    })
}

/// The ids of `--ids "1, 2,3"`, each one a path segment. Empty entries
/// (`1,,2`, a trailing comma) are dropped; a bad id fails the whole batch
/// before anything is sent.
fn parse_ids(value: &str) -> Result<Vec<String>, KobanaError> {
    let ids: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(String::from)
        .collect();
    if ids.is_empty() {
        return Err(KobanaError::Validation(
            tr!(
                pt: "--ids não tem nenhum id",
                en: "--ids has no id",
                es: "--ids no tiene ningún id",
            )
            .to_string(),
        ));
    }
    for id in &ids {
        kobana::validate::validate_identifier(id, "id")?;
    }
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn emitir(args: &[&str]) -> clap::ArgMatches {
        EmitirBoleto
            .command()
            .try_get_matches_from(std::iter::once("+emitir").chain(args.iter().copied()))
            .unwrap()
    }

    fn emitir_args(valor: &'static str, carteira: &'static str) -> Vec<&'static str> {
        vec![
            "--valor", valor, "--vencimento", "2026-05-01", "--nome", "Maria",
            "--cpf-cnpj", "012.345.678-90", "--carteira", carteira,
        ]
    }

    #[test]
    fn emitir_body_carries_the_parsed_numbers() {
        let body = request_body(&emitir(&emitir_args("150.50", "7"))).unwrap();
        assert_eq!(body["amount"], serde_json::json!(150.5));
        assert_eq!(body["bank_billet_account_id"], serde_json::json!(7));
    }

    #[test]
    fn emitir_refuses_numbers_that_do_not_parse() {
        // `150,50` used to become a R$ 0,00 billet on wallet 0
        for (valor, carteira) in [("150,50", "7"), ("150.50", "zz")] {
            let err = request_body(&emitir(&emitir_args(valor, carteira))).unwrap_err();
            assert!(matches!(err, KobanaError::Validation(_)), "{valor} {carteira}");
        }
    }

    #[test]
    fn cancel_ids_are_path_segments_and_empty_ones_are_dropped() {
        assert_eq!(parse_ids(" 1, 2,,3,").unwrap(), vec!["1", "2", "3"]);
        for bad in ["../../v2/charge/pix/x", "a?b", "1/cancel", "..", ",, ,"] {
            assert!(matches!(parse_ids(bad), Err(KobanaError::Validation(_))), "{bad}");
        }
    }

    #[test]
    fn a_batch_with_a_failure_is_an_api_error() {
        let ok = serde_json::json!({"id": "1", "status": "cancelled"});
        let bad = serde_json::json!({"id": "2", "status": "error", "status_code": 422});

        let output = executor::Output { fields: None, format: "json".into(), path: None, verbose: false, committed: true };
        assert!(batch_outcome(vec![ok.clone()], None, &output).is_ok());

        let err = batch_outcome(vec![ok.clone(), bad.clone()], Some(Some(422)), &output).unwrap_err();
        assert_eq!(err.exit_code(), kobana::error::ExitCode::ApiError);
        match err {
            KobanaError::Api { status, body, message } => {
                assert_eq!(status, 422);
                assert_eq!(body, Some(serde_json::json!([ok, bad])));
                assert!(message.contains('1') && message.contains('2'), "{message}");
            }
            other => panic!("{other:?}"),
        }
    }
}
