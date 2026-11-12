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
  "headers": {
    "X-Request-Source": "cli"
  },
  "query_params": {},
  "body": null,
  "files": [],
  "content_type": "application/json",
  "idempotency_key": "d2da129f-2bed-4598-b9cb-1b59b9ee6b31"
}
```

El token nunca aparece. Con `--file`, el informe lista los archivos y el `content_type` del formulario. Usa `--dry-run` en cualquier comando que cree, modifique o elimine, y siempre que un agente de IA esté armando el comando.

## Confirmación (`--yes`)

Las operaciones que mueven dinero o deshacen algo preguntan antes de enviar. Hoy son las de tarjeta de `finance`: pagar una factura (`card-statements payments create`), deshacer el pago (`card-statements payments delete`), reabrir una factura (`card-statements reopen`), recargar (`credit-cards top-ups`) y eliminar un movimiento o una recurrencia (`card-transactions delete`, `card-recurrences delete`).

```bash
kobana finance card-statements payments create STATEMENT_ID --json '{"amount": 1500}'
```

```
Pagar una factura (POST /v1/card-statements/STATEMENT_ID/payments). ¿Confirmas? (s/N)
```

Responder que no termina con código `0`, sin enviar nada. Fuera de una terminal — script, CI, agente de IA o `KOBANA_PROMPT_DISABLED` — nadie puede responder, así que el CLI se niega con el [código de salida](/reference/exit-codes) `3`. Pasa `--yes` para confirmar de antemano. `--dry-run` nunca pregunta.

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
