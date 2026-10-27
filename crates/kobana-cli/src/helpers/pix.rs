use clap::{Arg, Command};
use kobana::client::{ApiRequest, KobanaClient};
use kobana::error::KobanaError;
use kobana::tr;
use kobana::spec::HttpMethod;

use super::Helper;
use crate::clap_i18n;

/// Helper: +cobrar — Create a Pix charge with simplified interface
pub struct CobrarPix;

impl Helper for CobrarPix {
    fn name(&self) -> &'static str {
        "+cobrar"
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
                "kobana +cobrar --valor <VALOR> --conta-pix <UID> {}",
                clap_i18n::options_placeholder()
            ),
        )
            .about(self.about())
            .arg(Arg::new("valor").long("valor").required(true).help(tr!(pt: "Valor da cobrança", en: "Charge amount", es: "Monto del cobro")).value_name("VALOR"))
            .arg(Arg::new("conta-pix").long("conta-pix").required(true).help(tr!(pt: "UID da conta Pix", en: "Pix account UID", es: "UID de la cuenta Pix")).value_name("UID"))
            .arg(Arg::new("nome").long("nome").help(tr!(pt: "Nome do pagador", en: "Payer name", es: "Nombre del pagador")).value_name("NOME"))
            .arg(Arg::new("cpf-cnpj").long("cpf-cnpj").help(tr!(pt: "CPF ou CNPJ do pagador", en: "Payer CPF or CNPJ", es: "CPF o CNPJ del pagador")).value_name("DOC"))
            .arg(Arg::new("descricao").long("descricao").help(tr!(pt: "Descrição da cobrança", en: "Charge description", es: "Descripción del cobro")).value_name("TEXT"))
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
        let conta_pix = matches.get_one::<String>("conta-pix").unwrap().clone();
        let nome = matches.get_one::<String>("nome").cloned();
        let cpf_cnpj = matches.get_one::<String>("cpf-cnpj").cloned();
        let descricao = matches.get_one::<String>("descricao").cloned();
        let dry_run = matches.get_flag("dry-run");

        Box::pin(async move {
            let mut body = serde_json::json!({
                "amount": valor,
                "pix_account_uid": conta_pix,
            });

            if nome.is_some() || cpf_cnpj.is_some() {
                let mut payer = serde_json::Map::new();
                if let Some(n) = nome {
                    payer.insert("name".to_string(), serde_json::json!(n));
                }
                if let Some(doc) = cpf_cnpj {
                    payer.insert("document".to_string(), serde_json::json!(doc));
                }
                body["payer"] = serde_json::Value::Object(payer);
            }

            if let Some(desc) = descricao {
                body["description"] = serde_json::json!(desc);
            }

            if dry_run {
                let output = serde_json::json!({
                    "dry_run": true,
                    "method": "POST",
                    "url": format!("{}/v2/charge/pix", client.base_url()),
                    "body": body,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                return Ok(());
            }

            let request = ApiRequest {
                method: HttpMethod::Post,
                path: "/v2/charge/pix".to_string(),
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
