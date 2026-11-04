---
id: schedules
title: "kobana kadu schedules"
sidebar_label: "schedules"
---

# kobana kadu schedules


Commands of `kobana kadu schedules`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`schedules list`](#schedules-list) | `GET /v1/schedules` | List schedules |
| [`schedules create`](#schedules-create) | `POST /v1/schedules` | Schedule a Task in an environment |
| [`schedules get`](#schedules-get) | `GET /v1/schedules/{id}` | Get schedule details |
| [`schedules update`](#schedules-update) | `PATCH /v1/schedules/{id}` | Update, pause or resume a schedule |
| [`schedules delete`](#schedules-delete) | `DELETE /v1/schedules/{id}` | Cancel schedule (soft delete) |

## schedules list

List schedules

```text
kobana kadu schedules list [OPTIONS]
```

`GET /v1/schedules`

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

## schedules create

Schedule a Task in an environment

```text
kobana kadu schedules create [OPTIONS]
```

`POST /v1/schedules`

One schedule per Task and environment. Schedules do not stack: the run is skipped if the Task already has an active or queued execution.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `task_id` | `string (uuid)` | yes |  |
| `environment_id` | `string (uuid)` | yes |  |
| `cron` | `string` | yes |  |
| `timezone` | `string` | no |  |
| `paused` | `boolean` | no |  |

More fields and nested objects: `kobana schema kadu.schedules.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `409` | CONFLICT — the Task already has a schedule in the environment (`details.schedule_id`) / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CRON / VALIDATION_ERROR — Task or environment outside the organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## schedules get

Get schedule details

```text
kobana kadu schedules get [ID] [OPTIONS]
```

`GET /v1/schedules/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## schedules update

Update, pause or resume a schedule

```text
kobana kadu schedules update [ID] [OPTIONS]
```

`PATCH /v1/schedules/{id}`

`paused: true` keeps the cron and makes the dispatcher skip the schedule.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `cron` | `string` | no |  |
| `timezone` | `string` | no |  |
| `paused` | `boolean` | no |  |

More fields and nested objects: `kobana schema kadu.schedules.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | INVALID_CRON |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## schedules delete

Cancel schedule (soft delete)

```text
kobana kadu schedules delete [ID] [OPTIONS]
```

`DELETE /v1/schedules/{id}`

A new POST for the same Task and environment re-enables the schedule.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declared in the skill frontmatter's `schedules`: remove it from there |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
