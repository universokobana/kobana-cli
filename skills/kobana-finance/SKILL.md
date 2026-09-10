---
name: kobana-finance
description: "Kobana Financeiro Inteligente: contas financeiras, lançamentos, contas a pagar e receber, fluxo de caixa, conciliação."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana finance --help"
---

# finance — Financeiro Inteligente

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana finance <resource> <method> [flags]
```

Finance is its own API (`api.finance.kobana.com.br`) and does not share
credentials with the banking product. `KOBANA_TOKEN` must hold a finance token.

## API Resources

### Accounts and transactions
- `financial-accounts` — `list`, `create`, `get`, `update`, `delete`, `adjust-balance`
- `financial-transactions` — CRUD plus `aggregates`, `ids`
- `banks list`, `cash-flow list`

### Payables and receivables
- `payables` — CRUD plus `send-to-bank`, `cancel-dispatch`, `aggregates`, `ids`
- `receivables` — CRUD plus `charge`, `aggregates`, `ids`
- `receipts` — `list`, `get`
- `transfers` — `create`, `get`
- `reconciliations` — `create`, `get`, `delete`

### Classification
- `categories`, `classification-centers`, `people`, `companies`, `taxes` — CRUD
- `automatic-rules` — CRUD plus `reorder`
- `attachments` — `list`, `create`, `get`, `delete`, plus `download`, `links`

## Examples

```bash
kobana finance financial-accounts list --fields "id,name,balance"
kobana finance cash-flow list --params '{"start_date": "2026-01-01", "end_date": "2026-01-31"}'

# Aggregates keep large result sets out of the context window
kobana finance payables aggregates --params '{"group_by": "category"}'

# Mutations: always dry-run first
kobana finance payables send-to-bank --dry-run --params '{"id": "PAYABLE_ID"}'
```

## Discovering Commands

```bash
kobana finance --help
kobana finance payables --help
kobana schema finance.payables.create
```
