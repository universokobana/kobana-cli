---
id: recipes
title: Recipes
sidebar_label: Recipes
sidebar_position: 1
description: Ready-made commands for common tasks in each product — all with --dry-run so you can check before sending.
---

# Recipes

Common tasks, product by product. Replace the uppercase values (`INVOICE_ID`, `UID`…) with your own. Recipes that change data include `--dry-run`: run them that way first, check the request, and only then remove the flag. See [Safe writes](/usage/safe-writes).

> **Before you start:** each product has its own credential. See [Authentication](/getting-started/authentication).

## Banking Gateway (`banking`)

### Issue a bank slip

```bash
kobana banking +emitir --valor 150.50 --vencimento 2026-12-01 \
  --nome "Maria Silva" --cpf-cnpj "012.345.678-90" --carteira 1 --dry-run
```

### Charge by Pix

```bash
kobana banking +cobrar --valor 99.90 --vencimento 2026-12-01T23:59:59-03:00 \
  --conta-pix UID --nome "João" --cpf-cnpj "012.345.678-90" --dry-run
```

Without the helper, the same charge is a direct call:

```bash
kobana banking charge pix create \
  --json '{"amount": 99.90, "pix_account_uid": "UID"}' --dry-run
```

### Export open bank slips to CSV

```bash
kobana banking bank-billets list --params '{"status": "opened"}' \
  --fields "id,amount,due_at" --output-format csv --output boletos.csv
```

### Cancel bank slips

```bash
kobana banking bank-billets cancel 12345 --dry-run
kobana banking +cancelar-lote --ids "123,456,789" --dry-run
```

### Check an account balance

```bash
kobana banking financial accounts balances list \
  --params '{"financial_account_uid": "UID"}'
```

### Transfer by Pix

```bash
kobana banking transfer pix create \
  --json '{"amount": 500, "pix_key": "email@example.com"}' --dry-run
```

## Automatic Billing (`billing`)

### Pause and resume a subscription

```bash
kobana billing subscriptions pause SUBSCRIPTION_ID --dry-run
kobana billing subscriptions resume SUBSCRIPTION_ID --dry-run
```

### Close an invoice and issue the tax invoice

```bash
kobana billing invoices finalize INVOICE_ID --dry-run
kobana billing invoices issue-nfe INVOICE_ID --dry-run
```

## Smart Finance (`finance`)

Every `finance` route needs the [workspace](/getting-started/authentication#workspace-x-workspace-id): `export KOBANA_WORKSPACE_ID=...`.

### Pending payables

```bash
kobana finance payables list --params '{"status": "pending"}' \
  --fields "id,description,amount,due_date"
```

### Totals instead of the whole list

```bash
kobana finance payables aggregates --params '{"group_by": "category"}'
```

`aggregates` endpoints return sums, not records — the cheapest way to answer "how much?" without filling an agent's context.

### Cash flow for the month

```bash
kobana finance cash-flow \
  --params '{"start_date": "2026-01-01", "end_date": "2026-01-31"}'
```

### Send a payable to the bank for payment

```bash
kobana finance payables send-to-bank PAYABLE_ID --dry-run
```

## Dunning (`dunning`)

```bash
# Overdue charges
kobana dunning charges list --params '{"status": "overdue"}' \
  --fields "id,person_id,current_amount,due_date"

# Notify the debtor
kobana dunning charges notify CHARGE_ID --dry-run

# Accept an installment agreement
kobana dunning agreements accept AGREEMENT_ID --dry-run
```

## Autonomous Inbox (`inbox`)

```bash
# Inject a raw .eml into an inbox (multipart)
kobana inbox emails create --file file=./mensagem.eml \
  --json '{"inbox_id": "INBOX_ID"}' --dry-run

# Resend a webhook delivery that failed
kobana inbox webhooks deliveries replay WEBHOOK_ID DELIVERY_ID --dry-run
```

## Magic Dashboard (`dashboards`)

```bash
kobana dashboards data-sources list --fields "id,name,status"
kobana dashboards data-sources sync DATA_SOURCE_ID --dry-run
```

## Kadu Agent (`kadu`)

```bash
# Pre-check and trigger a Task
kobana kadu tasks readiness TASK_ID
kobana kadu tasks run TASK_ID \
  --json '{"reference_date": "2026-10-30", "environment_id": "ENVIRONMENT_ID"}' --dry-run

# Follow the execution
kobana kadu executions logs EXECUTION_ID

# All executions, following the cursor
kobana kadu executions list --page-all --fields "id,status,created_at"
```

## For any product

### Count and filter with `jq`

```bash
kobana billing invoices list --page-all --fields "id,status" \
  | jq -s 'group_by(.status) | map({status: .[0].status, total: length})'
```

### Test in the sandbox

```bash
kobana banking bank-billets list --env sandbox
export KOBANA_ENVIRONMENT=sandbox
```

`--env sandbox` on a product without a sandbox (`inbox`, `dashboards`, `kadu`) ends with an error instead of going to production.

### Retry without duplicating

```bash
kobana banking charge pix create --json '{"amount": 99.90, "pix_account_uid": "UID"}' \
  --idempotency-key pedido-4812
```

### Switch workspace in a single call

```bash
kobana finance payables list --params '{"X-Workspace-Id": "outro-uuid"}'
```
