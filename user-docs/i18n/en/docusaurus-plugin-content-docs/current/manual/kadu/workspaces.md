---
id: workspaces
title: "kobana kadu workspaces"
sidebar_label: "workspaces"
---

# kobana kadu workspaces


Commands of `kobana kadu workspaces`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | List workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Create workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Get workspace details |
| [`workspaces update`](#workspaces-update) | `PATCH /v1/workspaces/{id}` | Update workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Delete workspaces (soft delete) |

## workspaces list

List workspaces

```text
kobana kadu workspaces list [OPTIONS]
```

`GET /v1/workspaces`

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

## workspaces create

Create workspace

```text
kobana kadu workspaces create [OPTIONS]
```

`POST /v1/workspaces`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `external_id` | `string` | no | Record id in the source system (free text). |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.workspaces.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — duplicate record / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## workspaces get

Get workspace details

```text
kobana kadu workspaces get [ID] [OPTIONS]
```

`GET /v1/workspaces/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## workspaces update

Update workspace

```text
kobana kadu workspaces update [ID] [OPTIONS]
```

`PATCH /v1/workspaces/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.workspaces.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## workspaces delete

Delete workspaces (soft delete)

```text
kobana kadu workspaces delete [ID] [OPTIONS]
```

`DELETE /v1/workspaces/{id}`

Requires the `delete` level (or `all`): `write` does not delete. Cascade: the workspace's activities, tasks, executions, webhooks, memories and secrets; cancels schedules and disables the tasks' endpoint.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
