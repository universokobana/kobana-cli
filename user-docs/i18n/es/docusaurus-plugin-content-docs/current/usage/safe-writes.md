---
id: safe-writes
title: Modificaciones seguras
sidebar_label: Modificaciones seguras
sidebar_position: 4
description: Usa --dry-run para ver la solicitud antes de enviarla y --idempotency-key para repetir sin duplicar.
---

# Modificaciones seguras

## `--dry-run`

Muestra la solicitud que **se enviaría**, sin enviarla:

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

El token nunca aparece. Con `--file`, el informe lista los archivos y el `content_type` del formulario. Usa `--dry-run` en cualquier comando que cree, modifique o elimine, y siempre que un agente de IA esté armando el comando.

## Idempotencia

Toda modificación (`POST`, `PUT`, `PATCH`, `DELETE`) lleva una clave de idempotencia generada por el CLI. Para repetir una llamada sin riesgo de duplicar, fija la clave:

```bash
kobana banking charge pix create \
  --json '{"amount": 99.90, "pix_account_uid": "UID"}' \
  --idempotency-key pedido-4812
```

Reenviar el mismo comando con la misma clave devuelve el resultado de la primera llamada, en las APIs que soportan idempotencia.

## Validación de entrada

Los identificadores de ruta se validan antes de cualquier solicitud: el CLI rechaza valores vacíos, `.` y `..`, barras, caracteres de control, `?`, `#` y `%`. Un valor así termina con el [código de salida](/reference/exit-codes) `3`, sin tocar la API.
