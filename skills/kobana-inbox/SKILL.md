---
name: kobana-inbox
description: "Kobana Inbox Autônomo: workspaces, inboxes, e-mails, agentes e webhooks."
metadata:
  version: 0.1.0
  openclaw:
    category: "inbox"
    requires:
      bins:
        - kobana
    cliHelp: "kobana inbox --help"
---

# inbox — Inbox Autônomo

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana inbox <resource> <method> [flags]
```

## Authentication

The Inbox API is **not** the banking API and does not share its credentials:

- `KOBANA_TOKEN` must hold a **JWT** (HS512) with `iss=kobana`, `aud=inbox/<env>`
  and `sub=<workspace.external_id>`. Scopes are per resource
  (`inbox.workspaces`, `inbox.emails`, …).
- `KOBANA_INBOX_CLIENT_CERT` must point at a PEM file holding a client
  certificate chain and its private key — the API sits behind mTLS. Without it
  the CLI warns and requests are rejected.

`kobana auth login` is for the banking product and does not produce an inbox JWT.

## API Resources

### workspaces
- `list`, `create`, `get`, `update`, `delete`

### inboxes
- `list`, `create`, `get`, `update`, `delete`
- `recipients create` / `recipients delete` — allowed recipients of an inbox

### emails
- `list`, `get`
- `eml` — download the original .eml
- `reprocess` — reprocess an email

### agents
- `list`, `create`, `get`, `update`, `delete`

### agent-runs
- `list`, `get`

### webhooks
- `list`, `create`, `get`, `update`, `delete`
- `test` — send a `webhook.test` event
- `regenerate-secret` — rotate the signing secret
- `deliveries list` / `deliveries get` / `deliveries replay`

### system-events
- `list`, `get`

## Examples

```bash
# Resources keyed by {id} take it through --params
kobana inbox emails get --params '{"id": "EMAIL_ID"}'
kobana inbox emails reprocess --params '{"id": "EMAIL_ID"}'

# Webhook deliveries are nested under a webhook
kobana inbox webhooks deliveries list --params '{"id": "WEBHOOK_ID"}'
kobana inbox webhooks deliveries replay \
  --params '{"id": "WEBHOOK_ID", "deliveryId": "DELIVERY_ID"}'

# Always dry-run mutations first
kobana inbox inboxes create --dry-run --json '{"name": "Financeiro"}'
```

## Discovering Commands

```bash
kobana inbox --help
kobana schema inbox.emails.list
kobana schema --list
```
