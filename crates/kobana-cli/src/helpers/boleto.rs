use clap::{Arg, Command};
use kobana::client::{ApiRequest, KobanaClient};
use kobana::error::KobanaError;
use kobana::tr;
use kobana::spec::HttpMethod;

use super::Helper;
use crate::clap_i18n;

/// Helper: +emitir — Create a bank billet with simplified interface
pub struct EmitirBoleto;

impl Helper for EmitirBoleto {
    fn name(&self) -> &'static str {
        "+emitir"
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
                "kobana +emitir --valor <VALOR> --vencimento <DATA> --nome <NOME> --cpf-cnpj <DOC> --carteira <ID> {}",
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
            .arg(Arg::new("dry-run").long("dry-run").action(clap::ArgAction::SetTrue).help(tr!(pt: "Mostra a requisição sem executar", en: "Show the request without executing", es: "Muestra la solicitud sin ejecutarla")))
    }

    fn execute<'a>(
        &'a self,
        client: &'a KobanaClient,
        matches: &'a clap::ArgMatches,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), KobanaError>> + Send + 'a>> {
        let valor: f64 = matches
            .get_one::<String>("valor")
            .unwrap()
            .parse()
            .unwrap_or(0.0);
        let vencimento = matches.get_one::<String>("vencimento").unwrap().clone();
        let nome = matches.get_one::<String>("nome").unwrap().clone();
        let cpf_cnpj = matches.get_one::<String>("cpf-cnpj").unwrap().clone();
        let carteira: i64 = matches
            .get_one::<String>("carteira")
            .unwrap()
            .parse()
            .unwrap_or(0);
        let descricao = matches.get_one::<String>("descricao").cloned();
        let dry_run = matches.get_flag("dry-run");

        Box::pin(async move {
            let mut body = serde_json::json!({
                "amount": valor,
                "expire_at": vencimento,
                "customer_person_name": nome,
                "customer_cnpj_cpf": cpf_cnpj,
                "bank_billet_account_id": carteira,
            });

            if let Some(desc) = descricao {
                body["description"] = serde_json::json!(desc);
            }

            if dry_run {
                let output = serde_json::json!({
                    "dry_run": true,
                    "method": "POST",
                    "url": format!("{}/v1/bank_billets", client.base_url()),
                    "body": body,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                return Ok(());
            }

            let request = ApiRequest {
                method: HttpMethod::Post,
                path: "/v1/bank_billets".to_string(),
                query_params: None,
                body: Some(body),
                idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
                headers: Vec::new(),
            };

            let response = client.execute(&request).await?;
            println!("{}", serde_json::to_string_pretty(&response.body)?);
            Ok(())
        })
    }
}

/// Helper: +cancelar-lote — Cancel multiple bank billets
pub struct CancelarLoteBoletos;

impl Helper for CancelarLoteBoletos {
    fn name(&self) -> &'static str {
        "+cancelar-lote"
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
            format!("kobana +cancelar-lote --ids <IDS> {}", clap_i18n::options_placeholder()),
        )
            .about(self.about())
            .arg(Arg::new("ids").long("ids").required(true).help(tr!(pt: "IDs dos boletos separados por vírgula", en: "Comma-separated bank slip IDs", es: "IDs de los boletos separados por comas")).value_name("IDS"))
            .arg(Arg::new("dry-run").long("dry-run").action(clap::ArgAction::SetTrue).help(tr!(pt: "Mostra a requisição sem executar", en: "Show the request without executing", es: "Muestra la solicitud sin ejecutarla")))
    }

    fn execute<'a>(
        &'a self,
        client: &'a KobanaClient,
        matches: &'a clap::ArgMatches,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), KobanaError>> + Send + 'a>> {
        let ids_str = matches.get_one::<String>("ids").unwrap().clone();
        let dry_run = matches.get_flag("dry-run");

        Box::pin(async move {
            let ids: Vec<&str> = ids_str.split(',').map(|s| s.trim()).collect();

            if dry_run {
                let output = serde_json::json!({
                    "dry_run": true,
                    "action": "cancel_billets",
                    "count": ids.len(),
                    "ids": ids,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                return Ok(());
            }

            let mut results = Vec::new();
            for id in &ids {
                let request = ApiRequest {
                    method: HttpMethod::Put,
                    path: format!("/v1/bank_billets/{id}/cancel"),
                    query_params: None,
                    body: None,
                    idempotency_key: None,
                    headers: Vec::new(),
                };

                match client.execute(&request).await {
                    Ok(response) => {
                        results.push(serde_json::json!({
                            "id": id,
                            "status": "cancelled",
                            "response": response.body,
                        }));
                    }
                    Err(e) => {
                        results.push(serde_json::json!({
                            "id": id,
                            "status": "error",
                            "error": e.to_string(),
                        }));
                    }
                }
            }

            println!("{}", serde_json::to_string_pretty(&serde_json::Value::Array(results))?);
            Ok(())
        })
    }
}
