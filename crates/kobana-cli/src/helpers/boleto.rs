use clap::{Arg, Command};
use kobana::client::{ApiRequest, KobanaClient};
use kobana::error::KobanaError;
use kobana::spec::HttpMethod;

use super::Helper;

/// Helper: +emitir — Create a bank billet with simplified interface
pub struct EmitirBoleto;

impl Helper for EmitirBoleto {
    fn name(&self) -> &'static str {
        "+emitir"
    }

    fn about(&self) -> &'static str {
        "Emitir boleto com interface simplificada"
    }

    fn command(&self) -> Command {
        Command::new("+emitir")
            .about("Emitir boleto com interface simplificada")
            .arg(Arg::new("valor").long("valor").required(true).help("Valor do boleto (ex: 150.50)").value_name("VALOR"))
            .arg(Arg::new("vencimento").long("vencimento").required(true).help("Data de vencimento (YYYY-MM-DD)").value_name("DATA"))
            .arg(Arg::new("nome").long("nome").required(true).help("Nome do pagador").value_name("NOME"))
            .arg(Arg::new("cpf-cnpj").long("cpf-cnpj").required(true).help("CPF ou CNPJ do pagador").value_name("DOC"))
            .arg(Arg::new("carteira").long("carteira").required(true).help("ID da carteira de cobrança").value_name("ID"))
            .arg(Arg::new("descricao").long("descricao").help("Descrição/instruções do boleto").value_name("TEXT"))
            .arg(Arg::new("dry-run").long("dry-run").action(clap::ArgAction::SetTrue).help("Mostra a requisição sem executar"))
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
        "Cancelar múltiplos boletos de uma vez"
    }

    fn command(&self) -> Command {
        Command::new("+cancelar-lote")
            .about("Cancelar múltiplos boletos de uma vez")
            .arg(Arg::new("ids").long("ids").required(true).help("IDs dos boletos separados por vírgula").value_name("IDS"))
            .arg(Arg::new("dry-run").long("dry-run").action(clap::ArgAction::SetTrue).help("Mostra a requisição sem executar"))
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
