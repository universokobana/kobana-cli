---
id: operation-approvals
title: "kobana kadu operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana kadu operation-approvals


Commands of `kobana kadu operation-approvals`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation_approvals` | Approvals by link of the token's person |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation_approvals/{id}` | Get an approval by link |

## operation-approvals list

Approvals by link of the token's person

```text
kobana kadu operation-approvals list [OPTIONS]
```

`GET /v1/operation_approvals`

Operations an assistant requested through MCP that only run after the person confirms them by link in the console. Lists only the approvals of the token's person, in the token's organization, newest first. Read-only: Continue and Cancel happen in the console. Scope: `kadu/system_events`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Values: `pending`, `approved`, `executed`, `declined`, `failed`, `expired`. |
| `resource` | `string` | no |  |
| `created_at[gte]` | `string` | no | ISO 8601 (date or date and time). |
| `created_at[lte]` | `string` | no | ISO 8601 (date or date and time). |
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — invalid filter or cursor |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## operation-approvals get

Get an approval by link

```text
kobana kadu operation-approvals get [ID] [OPTIONS]
```

`GET /v1/operation_approvals/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
