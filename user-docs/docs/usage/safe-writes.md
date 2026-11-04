---
id: safe-writes
title: Alterações seguras
sidebar_label: Alterações seguras
sidebar_position: 4
description: Use --dry-run para ver a requisição antes de enviar e --idempotency-key para repetir sem duplicar.
---

# Alterações seguras

## `--dry-run`

Mostra a requisição que **seria** enviada, sem enviá-la:

```bash
kobana billing invoices finalize INVOICE_ID --dry-run
```

```json
{
  "dry_run": true,
  "method": "POST",
  "url": "https://api.billing.kobana.com.br/v1/invoices/INVOICE_ID/finalize",
  "headers": {},
  "query_params": {},
  "body": null,
  "files": [],
  "content_type": "application/json",
  "idempotency_key": "d2da129f-2bed-4598-b9cb-1b59b9ee6b31"
}
```

O token nunca aparece. Com `--file`, o relatório lista os arquivos e o `content_type` do formulário. Use `--dry-run` em qualquer comando que cria, altera ou exclui — e sempre que um agente de IA estiver montando o comando.

## Idempotência

Toda alteração (`POST`, `PUT`, `PATCH`, `DELETE`) leva uma chave de idempotência gerada pelo CLI. Para repetir uma chamada sem risco de duplicar, fixe a chave:

```bash
kobana banking charge pix create \
  --json '{"amount": 99.90, "pix_account_uid": "UID"}' \
  --idempotency-key pedido-4812
```

Reenviar o mesmo comando com a mesma chave devolve o resultado da primeira chamada, nas APIs que suportam idempotência.

## Validação de entrada

Identificadores de caminho são validados antes de qualquer requisição: o CLI recusa valores vazios, `.` e `..`, barras, caracteres de controle, `?`, `#` e `%`. Um valor assim termina com o [código de saída](/reference/exit-codes) `3`, sem tocar na API.
