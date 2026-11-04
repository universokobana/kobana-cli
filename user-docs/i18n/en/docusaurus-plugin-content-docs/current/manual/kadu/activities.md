---
id: activities
title: "kobana kadu activities"
sidebar_label: "activities"
---

# kobana kadu activities


Commands of `kobana kadu activities`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`activities list`](#activities-list) | `GET /v1/activities` | List activities |
| [`activities create`](#activities-create) | `POST /v1/activities` | Create activity |
| [`activities get`](#activities-get) | `GET /v1/activities/{id}` | Get activity details |
| [`activities update`](#activities-update) | `PATCH /v1/activities/{id}` | Update activity |
| [`activities delete`](#activities-delete) | `DELETE /v1/activities/{id}` | Delete activities (soft delete) |

## activities list

List activities

```text
kobana kadu activities list [OPTIONS]
```

`GET /v1/activities`

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

## activities create

Create activity

```text
kobana kadu activities create [OPTIONS]
```

`POST /v1/activities`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `workspace_id` | `string (uuid)` | yes |  |
| `external_id` | `string` | no | Record id in the source system (free text). |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.activities.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | VALIDATION_ERROR — workspace outside the organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## activities get

Get activity details

```text
kobana kadu activities get [ID] [OPTIONS]
```

`GET /v1/activities/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## activities update

Update activity

```text
kobana kadu activities update [ID] [OPTIONS]
```

`PATCH /v1/activities/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.activities.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## activities delete

Delete activities (soft delete)

```text
kobana kadu activities delete [ID] [OPTIONS]
```

`DELETE /v1/activities/{id}`

Requires the `delete` level (or `all`): `write` does not delete. Cascade: the activity's tasks, executions and memories; cancels schedules and disables the tasks' endpoint.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
