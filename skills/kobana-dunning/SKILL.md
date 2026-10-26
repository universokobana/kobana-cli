---
name: kobana-dunning
description: "Kobana Régua de Cobrança: clientes, cobranças, réguas, acordos, negativações, protestos e webhooks."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana dunning --help"
---

# dunning — Régua de Cobrança

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana dunning <resource> <method> [flags]
```

Dunning is its own API (`api.dunning.kobana.com.br`) and does not share
credentials with the banking product. `KOBANA_TOKEN` must hold an RS256 access
token issued by the Kobana Authorization Server (`account.kobana.com.br`) with
`aud=https://api.dunning.kobana.com.br`.

## API Resources

### Debtors and charges
- `people` — CRUD plus `statement-pdf`, `bulk-archive create`, `contact-points`
- `charges` — CRUD plus `notify`, `settlement-pdf`, `bulk-archive create`,
  `bulk-assign-rule create`, `bulk-cancel create`
- `classifications` — CRUD
- `interactions` — CRUD

### Collection rules and notifications
- `collection-rules` — CRUD plus `steps` (`get`, `create`, `update`, `delete`)
- `templates` — CRUD plus `duplicate`
- `email-layouts` — `list`, `create`
- `notifications` — CRUD plus `resend`
- `notification-channels` — CRUD
- `notification-preferences` — `list`, `update`
- `negotiation-campaigns` — `list`, `create`, `delete`

### Recovery
- `agreements` — CRUD plus `accept`, `cancel`, `pdf`, `installments get|update`
- `disputes` — `list`, `create`, `get`, `update`
- `negativations` — CRUD plus `remove`
- `protests` — CRUD plus `cancel`
- `tasks` — CRUD plus `complete`, `bulk-assign create`, `bulk-complete create`

### Data and integration
- `imports` — `list`, `create`, `get`
- `exports` — `charges list`, `people list`, `jobs list|create|download`
- `metrics recovery list`, `audit-logs list`, `me list`
- `webhook-endpoints` — CRUD plus `rotate-secret`, `test`, `deliveries get|retry`

## Examples

```bash
kobana dunning charges list --params '{"status": "overdue", "per_page": 25}' \
  --fields "id,person_id,current_amount,due_date,status"

kobana dunning metrics recovery list

# Mutations: always dry-run first
kobana dunning charges notify --dry-run --params '{"id": "CHARGE_ID"}'
kobana dunning agreements accept --dry-run --params '{"id": "AGREEMENT_ID"}'
```

## Discovering Commands

```bash
kobana dunning --help
kobana dunning charges --help
kobana schema dunning.charges.create
```
