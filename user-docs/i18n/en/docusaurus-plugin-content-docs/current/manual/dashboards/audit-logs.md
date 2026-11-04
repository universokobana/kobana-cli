---
id: audit-logs
title: "kobana dashboards audit-logs"
sidebar_label: "audit-logs"
---

# kobana dashboards audit-logs


Commands of `kobana dashboards audit-logs`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`audit-logs summary`](#audit-logs-summary) | `GET /v1/audit_logs/summary` | Audit activity summary |
| [`audit-logs list`](#audit-logs-list) | `GET /v1/audit_logs` | List audit logs |
| [`audit-logs get`](#audit-logs-get) | `GET /v1/audit_logs/{id}` | Get an audit log |

## audit-logs summary

Audit activity summary

```text
kobana dashboards audit-logs summary [OPTIONS]
```

`GET /v1/audit_logs/summary`

Activity of the last `hours` hours: total entries, the 10 most frequent actions and the 20 most recent entries. Requires the audit export permission.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `hours` | `number` | no | Window size in hours, 1 to 8760. Default 24. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Activity summary. |
| `400` | Bad request: malformed body, missing required field or a filter with an invalid value. |
| `404` | Resource not found in the current organization. |

## audit-logs list

List audit logs

```text
kobana dashboards audit-logs list [OPTIONS]
```

`GET /v1/audit_logs`

Audit trail of the workspace and of the organization, newest first. The trail cannot be changed.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `user_id` | `string` | no | Exact user identifier. |
| `action` | `string` | no | Exact action (e.g. `webhook.created`). |
| `resource_type` | `string` | no | Exact resource type (e.g. `webhook`). |
| `resource_id` | `string` | no | Exact resource ID. |
| `search` | `string` | no | Substring of the action, resource type, user ID or user e-mail, case-insensitive. |
| `date_from` | `string` | no | Lower bound of `created_at` (ISO 8601 date or date-time). |
| `date_to` | `string` | no | Upper bound of `created_at` (ISO 8601 date or date-time). A bare date covers the whole day (UTC). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | A page of audit entries. |
| `400` | Bad request: malformed body, missing required field or a filter with an invalid value. |
| `404` | Resource not found in the current organization. |

## audit-logs get

Get an audit log

```text
kobana dashboards audit-logs get [ID] [OPTIONS]
```

`GET /v1/audit_logs/{id}`

Returns one audit entry with its previous and new values.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The audit entry. |
| `404` | Resource not found in the current organization. |
