---
id: recipes
title: Recetas
sidebar_label: Recetas
sidebar_position: 1
description: Comandos listos para tareas comunes en cada producto — todos con --dry-run para que revises antes de enviar.
---

# Recetas

Tareas comunes, producto por producto. Reemplaza los valores en mayúsculas (`INVOICE_ID`, `UID`…) por los tuyos. Las recetas que modifican datos incluyen `--dry-run`: ejecútalas así primero, revisa la solicitud y solo entonces quita la flag. Consulta [Modificaciones seguras](/usage/safe-writes).

> **Antes de empezar:** cada producto tiene su propia credencial. Consulta [Autenticación](/getting-started/authentication).

## Gateway Bancario (`banking`)

### Emitir un boleto

```bash
kobana banking +emitir --valor 150.50 --vencimento 2026-12-01 \
  --nome "Maria Silva" --cpf-cnpj "012.345.678-90" --carteira 1 --dry-run
```

### Cobrar por Pix

```bash
kobana banking +cobrar --valor 99.90 --vencimento 2026-12-01T23:59:59-03:00 \
  --conta-pix UID --nome "João" --cpf-cnpj "012.345.678-90" --dry-run
```

Sin el helper, el mismo cobro es una llamada directa:

```bash
kobana banking charge pix create \
  --json '{"amount": 99.90, "pix_account_uid": "UID"}' --dry-run
```

### Exportar boletos abiertos a CSV

```bash
kobana banking bank-billets list --params '{"status": "opened"}' \
  --fields "id,amount,due_at" --output-format csv --output boletos.csv
```

### Cancelar boletos

```bash
kobana banking bank-billets cancel 12345 --dry-run
kobana banking +cancelar-lote --ids "123,456,789" --dry-run
```

### Consultar el saldo de una cuenta

```bash
kobana banking financial accounts balances list \
  --params '{"financial_account_uid": "UID"}'
```

### Transferir por Pix

```bash
kobana banking transfer pix create \
  --json '{"amount": 500, "pix_key": "email@example.com"}' --dry-run
```

## Facturación Automática (`billing`)

### Pausar y reanudar una suscripción

```bash
kobana billing subscriptions pause SUBSCRIPTION_ID --dry-run
kobana billing subscriptions resume SUBSCRIPTION_ID --dry-run
```

### Cerrar una factura y emitir la factura electrónica

```bash
kobana billing invoices finalize INVOICE_ID --dry-run
kobana billing invoices issue-nfe INVOICE_ID --dry-run
```

## Financiero Inteligente (`finance`)

Toda ruta de `finance` necesita el [espacio de trabajo](/getting-started/authentication#espacio-de-trabajo-x-workspace-id): `export KOBANA_WORKSPACE_ID=...`.

### Cuentas por pagar pendientes

```bash
kobana finance payables list --params '{"status": "pending"}' \
  --fields "id,description,amount,due_date"
```

### Totales en lugar de la lista completa

```bash
kobana finance payables aggregates --params '{"group_by": "category"}'
```

Los endpoints `aggregates` devuelven sumas, no registros — la forma más económica de responder "¿cuánto?" sin llenar el contexto de un agente.

### Flujo de caja del mes

```bash
kobana finance cash-flow \
  --params '{"start_date": "2026-01-01", "end_date": "2026-01-31"}'
```

### Enviar una cuenta por pagar al banco

```bash
kobana finance payables send-to-bank PAYABLE_ID --dry-run
```

## Cobranza (`dunning`)

```bash
# Cobros vencidos
kobana dunning charges list --params '{"status": "overdue"}' \
  --fields "id,person_id,current_amount,due_date"

# Avisar al deudor
kobana dunning charges notify CHARGE_ID --dry-run

# Aceptar un acuerdo de pago en cuotas
kobana dunning agreements accept AGREEMENT_ID --dry-run
```

## Inbox Autónomo (`inbox`)

```bash
# Inyectar un .eml sin procesar en una bandeja (multipart)
kobana inbox emails create --file file=./mensagem.eml \
  --json '{"inbox_id": "INBOX_ID"}' --dry-run

# Reenviar una entrega de webhook que falló
kobana inbox webhooks deliveries replay WEBHOOK_ID DELIVERY_ID --dry-run
```

## Dashboard Mágico (`dashboards`)

```bash
kobana dashboards data-sources list --fields "id,name,status"
kobana dashboards data-sources sync DATA_SOURCE_ID --dry-run
```

## Agente Kadu (`kadu`)

```bash
# Verificación previa y ejecución de una Task
kobana kadu tasks readiness TASK_ID
kobana kadu tasks run TASK_ID \
  --json '{"reference_date": "2026-10-30", "environment_id": "ENVIRONMENT_ID"}' --dry-run

# Seguir la ejecución
kobana kadu executions logs EXECUTION_ID

# Todas las ejecuciones, siguiendo el cursor
kobana kadu executions list --page-all --fields "id,status,created_at"
```

## Para cualquier producto

### Contar y filtrar con `jq`

```bash
kobana billing invoices list --page-all --fields "id,status" \
  | jq -s 'group_by(.status) | map({status: .[0].status, total: length})'
```

### Probar en sandbox

```bash
kobana banking bank-billets list --env sandbox
export KOBANA_ENVIRONMENT=sandbox
```

`--env sandbox` en un producto sin sandbox (`inbox`, `dashboards`, `kadu`) termina con error en lugar de ir a producción.

### Repetir sin duplicar

```bash
kobana banking charge pix create --json '{"amount": 99.90, "pix_account_uid": "UID"}' \
  --idempotency-key pedido-4812
```

### Cambiar de espacio de trabajo en una sola llamada

```bash
kobana finance payables list --params '{"X-Workspace-Id": "outro-uuid"}'
```
