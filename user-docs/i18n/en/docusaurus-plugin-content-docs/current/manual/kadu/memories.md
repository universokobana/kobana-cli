---
id: memories
title: "kobana kadu memories"
sidebar_label: "memories"
---

# kobana kadu memories


Commands of `kobana kadu memories`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`memories list`](#memories-list) | `GET /v1/memories` | List memories |
| [`memories create`](#memories-create) | `POST /v1/memories` | Create memory |
| [`memories consolidate`](#memories-consolidate) | `POST /v1/memories/consolidate` | Consolidates the memories of a period |
| [`memories get`](#memories-get) | `GET /v1/memories/{id}` | Get memory details |
| [`memories update`](#memories-update) | `PATCH /v1/memories/{id}` | Update memory |
| [`memories delete`](#memories-delete) | `DELETE /v1/memories/{id}` | Delete memory (soft delete) |
| [`memories rollback`](#memories-rollback) | `POST /v1/memories/{id}/rollback` | Rolls the memory back to a version |
| [`memories versions`](#memories-versions) | `GET /v1/memories/{id}/versions` | Lists the memory versions |

## memories list

List memories

```text
kobana kadu memories list [OPTIONS]
```

`GET /v1/memories`

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

## memories create

Create memory

```text
kobana kadu memories create [OPTIONS]
```

`POST /v1/memories`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `content` | `string` | yes |  |
| `workspace_id` | `string (uuid)` | no |  |
| `activity_id` | `string (uuid)` | no |  |
| `entity_type` | `string` | no |  |
| `entity_value` | `string` | no |  |
| `key` | `string` | no |  |
| `summary` | `string` | no |  |
| `status` | `string` | no |  |
| `external_id` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.memories.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT (duplicate external_id in the organization) / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — workspace/activity outside the organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## memories consolidate

Consolidates the memories of a period

```text
kobana kadu memories consolidate [OPTIONS]
```

`POST /v1/memories/consolidate`

Reads the finished executions (`success`/`failed`) of the period not consolidated yet — including tasks with memories off — and saves the durable facts as memories, in each Task's scope. Each execution is a model call charged to the organization. `dry_run: true` returns only the preview. Asynchronous: memories show up as the jobs run.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `from` | `string` | yes | First day (YYYY-MM-DD, UTC), inclusive. |
| `to` | `string` | yes | Last day (YYYY-MM-DD, UTC), inclusive. Up to 366 days. |
| `task_id` | `string (uuid)` | no | Only this Task's executions. |
| `dry_run` | `boolean` | no |  |

More fields and nested objects: `kobana schema kadu.memories.consolidate`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | preview (dry_run) |
| `202` | queued |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | VALIDATION_ERROR — invalid period or Task |
| `503` | SERVICE_UNAVAILABLE — queue unavailable |

## memories get

Get memory details

```text
kobana kadu memories get [ID] [OPTIONS]
```

`GET /v1/memories/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## memories update

Update memory

```text
kobana kadu memories update [ID] [OPTIONS]
```

`PATCH /v1/memories/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `content` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `activity_id` | `string (uuid)` | no |  |
| `entity_type` | `string` | no |  |
| `entity_value` | `string` | no |  |
| `key` | `string` | no |  |
| `summary` | `string` | no |  |
| `status` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.memories.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | VALIDATION_ERROR — workspace/activity outside the organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## memories delete

Delete memory (soft delete)

```text
kobana kadu memories delete [ID] [OPTIONS]
```

`DELETE /v1/memories/{id}`

Requires the `delete` level (or `all`): `write` does not delete.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## memories rollback

Rolls the memory back to a version

```text
kobana kadu memories rollback [ID] [OPTIONS]
```

`POST /v1/memories/{id}/rollback`

Saves a NEW version with the chosen one's content, which becomes current; the history stays whole (`rollback_from`).

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `version` | `integer` | yes |  |

More fields and nested objects: `kobana schema kadu.memories.rollback`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND (memory or version) |
| `409` | CONFLICT — already the current version / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — the version's workspace/activity is no longer valid |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## memories versions

Lists the memory versions

```text
kobana kadu memories versions [ID] [OPTIONS]
```

`GET /v1/memories/{id}/versions`

Newest first. Every content change (by the API, the agent, consolidation or the console) saves a version; `current: true` marks the one in force.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
