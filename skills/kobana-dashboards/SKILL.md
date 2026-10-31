---
name: kobana-dashboards
description: "Kobana Dashboard Mágico: fontes de dados, entidades, portal do cliente, workspaces e builder de páginas."
metadata:
  version: 0.1.0
  openclaw:
    category: "analytics"
    requires:
      bins:
        - kobana
    cliHelp: "kobana dashboards --help"
---

# dashboards — Dashboard Mágico

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana dashboards <resource> <method> [flags]
```

## Authentication

Dashboards is its own API (`api.dashboards.kobana.com.br`) and does not share
credentials with the banking product:

- `KOBANA_TOKEN` must hold an RS256 access token issued by the Kobana
  Authorization Server (`account.kobana.com.br`) with
  `aud=https://api.dashboards.kobana.com.br`. Scopes have two axes: `dashboards/{resource}` limits what the token reaches, and an
  access level says what it may do there — `GET` needs `read`, `write` or `all`;
  `DELETE` (bulk included) needs `delete` or `all`; bulk imports need `import`
  and bulk exports `export` (or `all`); every other method needs `write` or
  `all`. One level never stands in for another.
- `X-Workspace-Id` picks the workspace. It may be omitted while the organization
  has a single workspace; with two or more the API answers `400 workspace_required`.
  Set `KOBANA_WORKSPACE_ID`, or pass `"X-Workspace-Id"` in `--params` for one call.

## API Resources

### Data
- `data-sources` — `list`, `get`, `update`, `delete`, plus `authorize`,
  `schedule`, `spec`, `sync`, `catalog`, `database create`, `mcp create`,
  `openapi create`, `openapi analyze create`, `operations update`, `logs get`,
  `test create`
- `entities` — CRUD plus `aggregate`, `usage`, `records get|create|delete`,
  `records import create`, `join`, `preview create`, `sync`, `tenant-syncs`, `logs get`
- `imports`, `exports` (`download`), `data-purges` (`list`, `get`, `delete`), `backups` (`cancel`,
  `download`, `restore create`)

### Portal
- `portal-users` — CRUD plus `resend`, `email-preview`, `invite create`,
  `activity get`, `activity timeline get`
- `portal-settings` — `list`, `update`, `domain`, `logo`, `sender-identity create`
- `portal-custom-domains` — `list`, `create`, `delete`, `dns-status`, `verify`
- `portal-oauth-providers` — `list`, `create`, `update`, `delete`, `test create`
- `portal-user-mappings` — `list`, `update`, `delete`

### Builder
- `builder` — `list`, `draft update`, `theme update`, `publish create`,
  `versions get|restore|rollback`, and the page-editing `tools` (`list-pages`,
  `get-page`, `insert-block`, `update-block`, `move-block`, …)
- `assets` (`download`, `presign create`), `asset-types`, `checklists` (`progress`)

### Organization
- `workspaces` — CRUD plus `export`, `usage`, `import create`
- `organization`, `settings`, `dashboard-users`, `team-roles`, `permissions`,
  `api-keys` (`revoke`, `rotate`)
- `email-templates` (`duplicate`, `preview`), `email-layouts`, `notifications`
  (`send`, `stats list`), `comments`
- `webhooks` — CRUD plus `rotate-secret`, `test`, `deliveries get|retry`;
  `webhook-deliveries list|get`
- `audit-logs` (`summary list`), `events`, `usage`, `ai-usage`, `rate-limits`, `me`

## Examples

```bash
export KOBANA_WORKSPACE_ID=WORKSPACE_ID

kobana dashboards data-sources list --fields "id,name,status"
kobana dashboards entities records get --params '{"id": "ENTITY_ID"}'

# Mutations: always dry-run first
kobana dashboards data-sources sync --dry-run --params '{"id": "DATA_SOURCE_ID"}'
```

## Discovering Commands

```bash
kobana dashboards --help
kobana dashboards entities --help
kobana schema dashboards.data-sources.sync
```
