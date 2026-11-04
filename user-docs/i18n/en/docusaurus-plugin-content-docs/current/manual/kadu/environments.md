---
id: environments
title: "kobana kadu environments"
sidebar_label: "environments"
---

# kobana kadu environments


Commands of `kobana kadu environments`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`environments list`](#environments-list) | `GET /v1/environments` | List environments |
| [`environments create`](#environments-create) | `POST /v1/environments` | Create environment |
| [`environments get`](#environments-get) | `GET /v1/environments/{id}` | Get environment details |
| [`environments update`](#environments-update) | `PATCH /v1/environments/{id}` | Update environment |
| [`environments delete`](#environments-delete) | `DELETE /v1/environments/{id}` | Delete environment (soft delete) |

## environments list

List environments

```text
kobana kadu environments list [OPTIONS]
```

`GET /v1/environments`

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

## environments create

Create environment

```text
kobana kadu environments create [OPTIONS]
```

`POST /v1/environments`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `color` | `string` | yes |  |
| `branch` | `string` | no |  |

More fields and nested objects: `kobana schema kadu.environments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — an active environment with this label already exists / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## environments get

Get environment details

```text
kobana kadu environments get [ID] [OPTIONS]
```

`GET /v1/environments/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## environments update

Update environment

```text
kobana kadu environments update [ID] [OPTIONS]
```

`PATCH /v1/environments/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no |  |
| `color` | `string` | no |  |
| `branch` | `string` | no |  |

More fields and nested objects: `kobana schema kadu.environments.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — label already in use / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## environments delete

Delete environment (soft delete)

```text
kobana kadu environments delete [ID] [OPTIONS]
```

`DELETE /v1/environments/{id}`

Rejected while the environment has secrets or schedules.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | ENVIRONMENT_IN_USE — `details: { secrets, schedules }` / REPO_MANAGED — declared in `kadu.yaml`: remove it from there |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
