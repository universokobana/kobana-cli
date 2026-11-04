---
id: tasks
title: "kobana kadu tasks"
sidebar_label: "tasks"
---

# kobana kadu tasks


Commands of `kobana kadu tasks`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`tasks endpoint`](#tasks-endpoint) | `GET /v1/tasks/{id}/endpoint` | Task HTTP run endpoint |
| [`tasks endpoint update`](#tasks-endpoint-update) | `PUT /v1/tasks/{id}/endpoint` | Configure the Task's HTTP endpoint |
| [`tasks endpoint delete`](#tasks-endpoint-delete) | `DELETE /v1/tasks/{id}/endpoint` | Disable the Task's HTTP endpoint |
| [`tasks list`](#tasks-list) | `GET /v1/tasks` | List tasks |
| [`tasks create`](#tasks-create) | `POST /v1/tasks` | Create task |
| [`tasks get`](#tasks-get) | `GET /v1/tasks/{id}` | Get task details |
| [`tasks update`](#tasks-update) | `PATCH /v1/tasks/{id}` | Update task |
| [`tasks delete`](#tasks-delete) | `DELETE /v1/tasks/{id}` | Delete tasks (soft delete) |
| [`tasks readiness`](#tasks-readiness) | `GET /v1/tasks/{id}/readiness` | Run pre-check |
| [`tasks run`](#tasks-run) | `POST /v1/tasks/{id}/run` | Run task |

## tasks endpoint

Task HTTP run endpoint

```text
kobana kadu tasks endpoint [ID] [OPTIONS] [COMMAND]
```

`GET /v1/tasks/{id}/endpoint`

Configuration of the public endpoint `POST /api/run-task/{task_id}`. Credentials are never returned: only the `*_set` flags.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## tasks endpoint update

Configure the Task's HTTP endpoint

```text
kobana kadu tasks endpoint update [ID] [OPTIONS]
```

`PUT /v1/tasks/{id}/endpoint`

Creates or updates. An omitted credential keeps the stored one as long as the auth type does not change. Avoid `auth: params`: the secret goes in the URL and ends up in proxy and CDN access logs; prefer `bearer` or `header`.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `enabled` | `boolean` | yes |  |
| `auth` | `string` | yes | Values: `basic`, `bearer`, `header`, `params`. |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |
| `auth_param_key` | `string` | no |  |
| `auth_param_value` | `string` | no |  |

More fields and nested objects: `kobana schema kadu.tasks.endpoint.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | INCOMPLETE_AUTH — credential missing for the chosen auth type |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## tasks endpoint delete

Disable the Task's HTTP endpoint

```text
kobana kadu tasks endpoint delete [ID] [OPTIONS]
```

`DELETE /v1/tasks/{id}/endpoint`

Keeps the configuration; a PUT with `enabled: true` re-enables it.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## tasks list

List tasks

```text
kobana kadu tasks list [OPTIONS]
```

`GET /v1/tasks`

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

## tasks create

Create task

```text
kobana kadu tasks create [OPTIONS]
```

`POST /v1/tasks`

The task is created in the default pricing tier; tier and AI model are set by Kobana (`model` in the body is rejected).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `workspace_id` | `string (uuid)` | yes |  |
| `activity_id` | `string (uuid)` | yes |  |
| `skill_slug` | `string` | yes | `<plugin>:<skill>` from the organization's repository: the workspace's plugin (when it has one), a skill present in the synced catalog (when there is a catalog), never `kadu:*`. |
| `parameters_schema` | `object` | no |  |
| `parameters_sensitive` | `boolean` | no |  |
| `external_id` | `string` | no | Record id in the source system (free text). |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.tasks.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | VALIDATION_ERROR — workspace/activity outside the organization or `skill_slug` not allowed |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## tasks get

Get task details

```text
kobana kadu tasks get [ID] [OPTIONS]
```

`GET /v1/tasks/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## tasks update

Update task

```text
kobana kadu tasks update [ID] [OPTIONS]
```

`PATCH /v1/tasks/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no |  |
| `skill_slug` | `string` | no | `<plugin>:<skill>` from the organization's repository: the workspace's plugin (when it has one), a skill present in the synced catalog (when there is a catalog), never `kadu:*`. |
| `parameters_schema` | `object` | no |  |
| `parameters_sensitive` | `boolean` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.tasks.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | VALIDATION_ERROR — `skill_slug` not allowed |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## tasks delete

Delete tasks (soft delete)

```text
kobana kadu tasks delete [ID] [OPTIONS]
```

`DELETE /v1/tasks/{id}`

Requires the `delete` level (or `all`): `write` does not delete. Cascade: the task's executions and files; cancels schedules and disables the endpoint.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## tasks readiness

Run pre-check

```text
kobana kadu tasks readiness [ID] [OPTIONS]
```

`GET /v1/tasks/{id}/readiness`

Whether the Task can run in the environment: valid environment and a value for every required secret (in the Task's workspace or global). Also returns the Task's queue. The run revalidates everything.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `environment_id` | `string` | no |  |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## tasks run

Run task

```text
kobana kadu tasks run [ID] [OPTIONS]
```

`POST /v1/tasks/{id}/run`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `reference_date` | `string` | yes |  |
| `environment_id` | `string (uuid)` | yes |  |
| `parameters` | `object` | no |  |
| `locale` | `string` | no |  |
| `additional_instructions` | `string` | no |  |
| `input_file_ids` | `array<string (uuid)>` | no | Files uploaded through `POST /files` (confirmed and not yet used). |
| `mode` | `string` | no | `resolve_pendencies` only resolves the pending items of the Task's previous executions (console: "Resolve pending items"); `parameters` is ignored. Values: `default`, `resolve_pendencies`. |

More fields and nested objects: `kobana schema kadu.tasks.run`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | COST_LIMIT_REACHED / INVALID_PARAMETERS / ENVIRONMENT_REQUIRED / MISSING_SECRETS (details.keys) / INVALID_INPUT_FILES |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
