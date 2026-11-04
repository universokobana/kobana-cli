---
id: audit-logs
title: "kobana dunning audit-logs"
sidebar_label: "audit-logs"
description: "Query audit trail"
---

# kobana dunning audit-logs

Query audit trail

Commands of `kobana dunning audit-logs`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`audit-logs`](#audit-logs) | `GET /v1/audit-logs` | Query audit trail |

## audit-logs

Query audit trail

```text
kobana dunning audit-logs [OPTIONS]
```

`GET /v1/audit-logs`

Lists the organization’s audit trail records (newest first), with filters by action, entity, actor and period.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `action` | `string` | no | Filter by action (partial match, `contains`). |
| `entity_type` | `string` | no | Filter by entity type. |
| `entity_id` | `string` | no | Filter by entity (UUID). |
| `actor_type` | `string` | no | Filter by actor type (`user` \| `system` \| `job`). Values: `user`, `system`, `job`. |
| `actor_id` | `string` | no | Filter by actor (user ID, job name or system name). |
| `from` | `string` | no | Period start (ISO 8601, inclusive). |
| `to` | `string` | no | Period end (ISO 8601, inclusive). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of audit records. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
