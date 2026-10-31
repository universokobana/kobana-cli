---
name: kobana-kadu
description: "Agente Kadu: tasks do agente de IA, execuções (disparo, logs, eventos, pendências, conversa), ambientes, secrets, agendamentos, arquivos, memórias, webhooks e uso."
metadata:
  version: 0.1.0
  openclaw:
    category: "automation"
    requires:
      bins:
        - kobana
    cliHelp: "kobana kadu --help"
---

# kadu — Agente Kadu

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana kadu <resource> <method> [flags]
```

## Authentication

Kadu is its own API (`api.kadu.ia.br`) and does not share credentials with any
other product:

- `KOBANA_TOKEN` must hold an RS256 access token (`typ: at+jwt`) issued by the
  Kobana Authorization Server (`account.kobana.com.br`) with
  `aud=https://api.kadu.ia.br`. The organization is the token's `org` claim —
  there is no workspace header; workspaces are a resource.
- Scopes have two axes: `kadu/{resource}` (or `kadu/all`) limits what the token
  reaches, and an access level says what it may do there — `GET` needs `read`,
  `DELETE` needs `delete`, every other method needs `write` (or `all`). Missing
  scope answers `403 INSUFFICIENT_SCOPE`.
- A token issued for a person is also limited by that person's role in the
  organization (`403 USER_PERMISSION_DENIED`), and some routes by what Kobana
  enabled for the organization (`403 FEATURE_NOT_ENABLED`). Running a task
  needs an active plan (`403 ENTITLEMENT_REQUIRED`).
- No sandbox: `--env sandbox` fails. `--env development` targets the local dev
  server (`localhost:3030/api`).

## API Resources

### Tasks and executions
- `tasks` — CRUD plus `readiness` (pre-check), `run`, `endpoint get|update|delete`
- `executions` — `list`, `get`, `cancel`, `continue` (next round of a
  conversational task), `logs`, `events`, `todos`, `search list`
- `todos` — `update` (resolve a pending item)
- `files` — `list`, `create`, `get`, `confirm`, `download` (input files)

### Configuration
- `workspaces`, `activities` — CRUD
- `environments`, `secrets`, `schedules`, `memories` — CRUD
- `webhooks` — CRUD plus `rotate-secret`; `webhook-deliveries list|get|replay`
- `repository` — `list`, `sync create` (skills repository)

### Organization
- `organizations` (`list`, `get`, `update`), `users`, `memberships`, `roles`
- `usage`, `system-events`, `operation-approvals` (read only), `me`

## Pagination

List routes page by cursor (`cursor` + `limit` in, `meta.next_cursor` out).
`--page-all` follows the cursor until it ends.

## Examples

```bash
kobana kadu tasks list --fields "id,slug,label"
kobana kadu tasks readiness --params '{"id": "TASK_ID"}'

# Mutations: always dry-run first
kobana kadu tasks run --dry-run --params '{"id": "TASK_ID"}' \
  --json '{"reference_date": "2026-10-30", "environment_id": "ENVIRONMENT_ID"}'

kobana kadu executions get --params '{"id": "EXECUTION_ID"}' --fields "id,status"
kobana kadu executions logs --params '{"id": "EXECUTION_ID"}'
kobana kadu executions list --page-all --fields "id,status,created_at"
```

## Discovering Commands

```bash
kobana kadu --help
kobana kadu executions --help
kobana schema kadu.tasks.run
```
