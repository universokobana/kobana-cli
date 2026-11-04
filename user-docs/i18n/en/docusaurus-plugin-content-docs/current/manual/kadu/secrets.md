---
id: secrets
title: "kobana kadu secrets"
sidebar_label: "secrets"
---

# kobana kadu secrets


Commands of `kobana kadu secrets`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`secrets list`](#secrets-list) | `GET /v1/secrets` | List secrets |
| [`secrets create`](#secrets-create) | `POST /v1/secrets` | Create secret |
| [`secrets get`](#secrets-get) | `GET /v1/secrets/{id}` | Get secret details |
| [`secrets update`](#secrets-update) | `PATCH /v1/secrets/{id}` | Update secret |
| [`secrets delete`](#secrets-delete) | `DELETE /v1/secrets/{id}` | Delete secret (soft delete) |

## secrets list

List secrets

```text
kobana kadu secrets list [OPTIONS]
```

`GET /v1/secrets`

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

## secrets create

Create secret

```text
kobana kadu secrets create [OPTIONS]
```

`POST /v1/secrets`

The value is stored encrypted and never returned.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `environment_id` | `string (uuid)` | yes |  |
| `key` | `string` | yes |  |
| `value` | `string` | yes |  |
| `workspace_id` | `string (uuid)` | no |  |
| `notes` | `string` | no |  |

More fields and nested objects: `kobana schema kadu.secrets.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | VALIDATION_ERROR — environment or workspace outside the organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## secrets get

Get secret details

```text
kobana kadu secrets get [ID] [OPTIONS]
```

`GET /v1/secrets/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## secrets update

Update secret

```text
kobana kadu secrets update [ID] [OPTIONS]
```

`PATCH /v1/secrets/{id}`

A missing or empty `value` keeps the stored value.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `environment_id` | `string (uuid)` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `key` | `string` | no |  |
| `value` | `string` | no |  |
| `notes` | `string` | no |  |

More fields and nested objects: `kobana schema kadu.secrets.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | VALIDATION_ERROR — environment or workspace outside the organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## secrets delete

Delete secret (soft delete)

```text
kobana kadu secrets delete [ID] [OPTIONS]
```

`DELETE /v1/secrets/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
