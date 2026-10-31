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

## Authentication

Finance is its own API (`api.finance.kobana.com.br`) and does not share
credentials with the banking product. Every request needs **two** things:

- `KOBANA_TOKEN` — an RS256 access token issued by the Kobana Authorization
  Server. Its `org` claim is the tenant. Scopes have two axes: `finance/{resource}` limits what the token reaches, and an
  access level says what it may do there — `GET` needs `read`, `write` or `all`;
  `DELETE` (bulk included) needs `delete` or `all`; bulk imports need `import`
  and bulk exports `export` (or `all`); every other method needs `write` or
  `all`. One level never stands in for another.
  Sending a payment to the bank or charging needs `write` on
  `finance/payable_dispatches` / `finance/receivable_charges`.
- `X-Workspace-Id` — the workspace inside that organization. **Required on every
  route** (`400 WORKSPACE_REQUIRED` without it). Set `KOBANA_WORKSPACE_ID`, or
  pass `"X-Workspace-Id"` in `--params` to override it for one call.

## API Resources

### Accounts and transactions
- `financial-accounts` — `list`, `create`, `get`, `update`, `delete`, `adjust-balance`,
  `balances get|create|delete`, `balances latest get`
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
- `custom-classifiers` — CRUD plus `values create|update|delete`
- `automatic-rules` — CRUD plus `reorder`
- `attachments` — `list`, `create`, `get`, `delete`, plus `download`, `links`, `unlink`
- `ai-agents` — CRUD plus `triggers` (CRUD and `token`); `ai-skills` — CRUD

### Fiscal and identity
- `fiscal-documents` — `list`, `get` (received fiscal documents)
- `me list` — the organization, person and scopes behind the token

## Examples

```bash
export KOBANA_WORKSPACE_ID=WORKSPACE_ID

kobana finance financial-accounts list --fields "id,name,balance"
kobana finance cash-flow list --params '{"start_date": "2026-01-01", "end_date": "2026-01-31"}'

# Aggregates keep large result sets out of the context window
kobana finance payables aggregates list --params '{"group_by": "category"}'

# Mutations: always dry-run first
kobana finance payables send-to-bank --dry-run --params '{"id": "PAYABLE_ID"}'
```

Listings use **cursor pagination** (`limit`, then `meta.next_cursor` as
`cursor`). `--page-all` walks `page`/`per_page` and does not follow cursors, so
page finance listings by hand.

## Discovering Commands

```bash
kobana finance --help
kobana finance payables --help
kobana schema finance.payables.create
```
