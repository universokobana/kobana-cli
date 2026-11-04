---
id: roles
title: "kobana kadu roles"
sidebar_label: "roles"
---

# kobana kadu roles


Commands of `kobana kadu roles`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`roles list`](#roles-list) | `GET /v1/roles` | List roles |
| [`roles create`](#roles-create) | `POST /v1/roles` | Create role |

## roles list

List roles

```text
kobana kadu roles list [OPTIONS]
```

`GET /v1/roles`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Field in snake_case: created_at (default), updated_at or a filterable field. |
| `sort_order` | `string` | no | Default: desc. Values: `asc`, `desc`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## roles create

Create role

```text
kobana kadu roles create [OPTIONS]
```

`POST /v1/roles`

Permissions in the format `*`, `agent.*`, `agent.<resource>.*` or `agent.<resource>.<action>`. On a person's token, none can go beyond the role of whoever creates it.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `permissions` | `array<string>` | no |  |

More fields and nested objects: `kobana schema kadu.roles.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
