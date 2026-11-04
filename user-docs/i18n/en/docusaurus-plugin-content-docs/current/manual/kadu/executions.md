---
id: executions
title: "kobana kadu executions"
sidebar_label: "executions"
---

# kobana kadu executions


Commands of `kobana kadu executions`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`executions search`](#executions-search) | `GET /v1/executions/search` | Text search across executions |
| [`executions list`](#executions-list) | `GET /v1/executions` | List executions |
| [`executions get`](#executions-get) | `GET /v1/executions/{id}` | Get execution details |
| [`executions cancel`](#executions-cancel) | `POST /v1/executions/{id}/cancel` | Cancel the execution |
| [`executions consolidate-memories`](#executions-consolidate-memories) | `POST /v1/executions/{id}/consolidate_memories` | Consolidates the execution's memories |
| [`executions continue`](#executions-continue) | `POST /v1/executions/{id}/continue` | Continue the conversation (next round) |
| [`executions events`](#executions-events) | `GET /v1/executions/{id}/events` | Execution events |
| [`executions logs`](#executions-logs) | `GET /v1/executions/{id}/logs` | Execution logs |
| [`executions todos`](#executions-todos) | `GET /v1/executions/{id}/todos` | Pending items (todos) of the execution |

## executions search

Text search across executions

```text
kobana kadu executions search [OPTIONS]
```

`GET /v1/executions/search`

Searches events, todos, logs, result and error. Up to 30 executions, most recent first, each with up to 3 snippets. Only sources the caller could read directly are included: logs require the organization's `executions_logs.view` permission, and on a person's token the role needs `agent.logs.list`, `agent.events.list` and `agent.todos.list` for logs, events and todos.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `q` | `string` | yes |  |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — `q` shorter than 2 characters |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## executions list

List executions

```text
kobana kadu executions list [OPTIONS]
```

`GET /v1/executions`

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

## executions get

Get execution details

```text
kobana kadu executions get [ID] [OPTIONS]
```

`GET /v1/executions/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## executions cancel

Cancel the execution

```text
kobana kadu executions cancel [ID] [OPTIONS]
```

`POST /v1/executions/{id}/cancel`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | cancelled (or the current terminal status, if already finished) |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## executions consolidate-memories

Consolidates the execution's memories

```text
kobana kadu executions consolidate-memories [ID] [OPTIONS]
```

`POST /v1/executions/{id}/consolidate_memories`

Reads the result, the events, the pending items and the user request of this finished execution and saves the durable facts as memories, in the Task scope — even with memories off in it. Redoes it if already consolidated (updates the same memories). It is a model call charged to the organization. Asynchronous.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `202` | queued |
| `404` | NOT_FOUND |
| `409` | CONFLICT — execution not finished / IDEMPOTENCY_KEY_REUSED |
| `503` | SERVICE_UNAVAILABLE — queue unavailable |

## executions continue

Continue the conversation (next round)

```text
kobana kadu executions continue [ID] [OPTIONS]
```

`POST /v1/executions/{id}/continue`

Only for a conversational Task (`conversational`): creates a child execution of this one that resumes the agent session (context, output files and state) and handles the request in `message`. It uses this execution's parameters, reference date and environment, and this execution must be finished (`success` or `failed`). Continuing an older round opens a branch. The new execution carries `parentExecutionId` (this one) and `rootExecutionId` (the first round).

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `message` | `string` | yes |  |
| `locale` | `string` | no |  |
| `input_file_ids` | `array<string (uuid)>` | no | New files for this round (`POST /files`, not yet used). |

More fields and nested objects: `kobana schema kadu.executions.continue`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | round created (in the Task queue) |
| `404` | NOT_FOUND |
| `409` | NOT_CONVERSATIONAL / EXECUTOR_NOT_SUPPORTED / EXECUTOR_CHANGED / NOT_FINISHED / SESSION_UNAVAILABLE |
| `422` | ENVIRONMENT_REQUIRED / MISSING_SECRETS (details.keys) / INVALID_PARAMETERS / INVALID_INPUT_FILES / COST_LIMIT_REACHED |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## executions events

Execution events

```text
kobana kadu executions events [ID] [OPTIONS]
```

`GET /v1/executions/{id}/events`

By `sequence`, up to 500 per call. To follow along, pass the last `sequence` received in `after_sequence` and repeat.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `after_sequence` | `integer,null` | no |  |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## executions logs

Execution logs

```text
kobana kadu executions logs [ID] [OPTIONS]
```

`GET /v1/executions/{id}/logs`

In order of arrival, up to 1000 per call. To follow a running execution, pass the last `id` received in `after_id` and repeat.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `after_id` | `integer,null` | no |  |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## executions todos

Pending items (todos) of the execution

```text
kobana kadu executions todos [ID] [OPTIONS]
```

`GET /v1/executions/{id}/todos`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
