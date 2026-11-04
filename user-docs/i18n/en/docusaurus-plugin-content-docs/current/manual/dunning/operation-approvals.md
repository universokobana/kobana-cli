---
id: operation-approvals
title: "kobana dunning operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana dunning operation-approvals


Commands of `kobana dunning operation-approvals`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation-approvals` | List my approvals |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation-approvals/{id}` | Get an approval |

## operation-approvals list

List my approvals

```text
kobana dunning operation-approvals list [OPTIONS]
```

`GET /v1/operation-approvals`

Lists the approvals by link of the token person, in the token organization and the call workspace, most recent first. Read only: nothing here approves, cancels or reopens an approval. No role permission nor resource scope of its own — the data is the person's own; read level is enough.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `status` | `string` | no | Only this status. `expired` includes a lapsed pending one; `pending` only one still valid. Values: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | no | Requested at or after this instant (ISO 8601, inclusive). |
| `created_before` | `string` | no | Requested before this instant (ISO 8601, exclusive). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The person's approvals. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## operation-approvals get

Get an approval

```text
kobana dunning operation-approvals get [ID] [OPTIONS]
```

`GET /v1/operation-approvals/{id}`

Status and outcome of an approval by link, by the `approval.id` the destructive tool returned in `approval_required`. Call it after the person says they decided on the screen — not in a loop. Another person's or organization's approval, or an unknown id: the same 404. `result` comes with secrets masked.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The approval. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
