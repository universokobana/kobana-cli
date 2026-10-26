---
name: kobana-billing
description: "Kobana Faturamento Automático: assinaturas, planos, produtos, faturas, NF-e, propostas, cobranças recorrentes."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana billing --help"
---

# billing — Faturamento Automático

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana billing <resource> <method> [flags]
```

Billing is its own API (`api.billing.kobana.com.br`) and does not share
credentials with the banking product. `KOBANA_TOKEN` must hold an RS256 access
token issued by the Kobana Authorization Server (`account.kobana.com.br`) with
`aud=https://api.billing.kobana.com.br`. `GET` needs the `read`, `write` or `all`
scope; every other method needs `write` or `all`.

## API Resources

### Recurring billing
- `subscriptions` — `list`, `create`, `get`, `update`, `delete`, plus `activate`,
  `pause`, `resume`, `cancel`, `change-plan`, `change-anniversary`, `create-invoice`, `items`,
  `revert-to-confirmed`, `sync-plan-items`
- `subscription-changes config`, `subscription-item-changes list|get`
- `plans` — CRUD plus `duplicate`, `items`, `bulk-archive`, `bulk-delete`, `bulk-move-group`
- `plan-groups` — CRUD plus `archive`, `reorder`
- `plan-changes` — `list`, `create`, `get`, `config`, `rules`, `stats`

### Catalog
- `products` — CRUD plus `archive`, `publish`, `restore`, `reorder`, and the
  `bulk-*` operations
- `product-groups` — CRUD plus `archive`, `reorder`
- `service-items` — CRUD

### Invoicing
- `invoices` — CRUD plus `finalize`, `pay`, `undo-payment`, `void`,
  `apply-credits`, `remove-credits`, `send-reminder`, `issue-nfe`, `pdf`,
  `installments`, `payment-methods`, `pending-nfe`, `pending-payment`
- `nfes` — CRUD plus `issue`, `cancel`, `retry-cancel`, `validate-barueri`,
  `import-barueri`, `fetch-barueri`, `fetch-from-sefaz`, `sync`, `pdf`, `xml`
- `payments` — `list`, `create`, `get`, `delete`, plus `cancel`, `refund`,
  `retry`, `change-card`, `change-due-date`, `send-email`, `sync`
- `payment-methods` — CRUD plus `set-default`
- `late-fee-policies` — CRUD plus `default`, `simulate create`
- `bank-accounts`, `pix-keys` — CRUD; `banks list`
- `holidays` — CRUD plus `sync create`

### Accounts & customers
- `billing-accounts` — CRUD plus `close`, `suspend`, `reactivate`, `credits`,
  `entitlements`, `invoices`, `subscriptions`
- `customers` — CRUD plus `recalculate-mrr`, `statement`, `users`
- `companies` — CRUD plus `deactivate`, `reactivate`, `set-default`, `fiscal`,
  `fiscal-profile`, `matriz`
- `organization list|update`, `dashboard-users`, `team-roles`, `portal-users list`
- `me list` — the organization and scopes behind the token

### Proposals, credits and taxes
- `proposals` — CRUD plus `send`, `accept`, `undo-accept`, `publish`,
  `unpublish`, `cancel`, `duplicate`, `generate-pdf`, `items`, `pricing`, `stats`
- `proposal-templates` — CRUD plus `activate`, `deactivate`, `duplicate`, `set-default`
- `coupons` — CRUD plus `deactivate`, `redemptions`
- `credits`, `withholdings` (`convert-to-credits`), `tax-rules`, `tax-periods`
- `certificates`, `usage` (`summary`, `snapshots`, `credits`), `credit-purchases create`

## Examples

```bash
kobana billing subscriptions list --params '{"status": "active", "per_page": 25}' \
  --fields "id,status,plan_id,next_billing_at"

# Mutations: always dry-run first
kobana billing subscriptions pause --dry-run --params '{"id": "SUBSCRIPTION_ID"}'
kobana billing invoices finalize --dry-run --params '{"id": "INVOICE_ID"}'
```

## Discovering Commands

```bash
kobana billing --help
kobana billing subscriptions --help
kobana schema billing.subscriptions.create
```
