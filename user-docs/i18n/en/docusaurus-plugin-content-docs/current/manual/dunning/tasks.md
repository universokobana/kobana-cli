---
id: tasks
title: "kobana dunning tasks"
sidebar_label: "tasks"
---

# kobana dunning tasks


Commands of `kobana dunning tasks`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`tasks list`](#tasks-list) | `GET /v1/tasks` | List tasks |
| [`tasks create`](#tasks-create) | `POST /v1/tasks` | Create task |
| [`tasks bulk-assign`](#tasks-bulk-assign) | `POST /v1/tasks/bulk-assign` | Bulk-assign tasks |
| [`tasks bulk-complete`](#tasks-bulk-complete) | `POST /v1/tasks/bulk-complete` | Bulk-complete tasks |
| [`tasks get`](#tasks-get) | `GET /v1/tasks/{id}` | Get task |
| [`tasks update`](#tasks-update) | `PUT /v1/tasks/{id}` | Update task |
| [`tasks complete`](#tasks-complete) | `POST /v1/tasks/{id}/complete` | Complete task |

## tasks list

List tasks

```text
kobana dunning tasks list [OPTIONS]
```

`GET /v1/tasks`

Lists the organization’s tasks with pagination and filters. Sortable via `sort_by` (`title` | `priority` | `status` | `due_at` | `created_at` | `updated_at`).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `sort_by` | `string` | no | Sort field (public snake_case name, e.g. `due_date`). |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |
| `status` | `string` | no | Filter by status (`pending` \| `in_progress` \| `completed` \| `cancelled`). Values: `pending`, `in_progress`, `completed`, `cancelled`. |
| `priority` | `string` | no | Filter by priority (`low` \| `medium` \| `high` \| `urgent`). Values: `low`, `medium`, `high`, `urgent`. |
| `task_type` | `string` | no | Filter by type (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Values: `call`, `email`, `visit`, `review`, `follow_up`. |
| `assigned_to_id` | `string` | no | Filter by assignee (UUID). |
| `person_id` | `string` | no | Filter by customer (UUID). |
| `charge_id` | `string` | no | Filter by charge (UUID). |
| `include` | `string` | no | Relations to include, comma-separated (`person`, `assigned_to`, `charge`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated task list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## tasks create

Create task

```text
kobana dunning tasks create [OPTIONS]
```

`POST /v1/tasks`

Creates a task, optionally linked to a customer, a charge and an assignee (all within the organization). Supports `Idempotency-Key`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `title` | `string` | yes | Task title. |
| `task_type` | `string` | yes | Type (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Values: `call`, `email`, `visit`, `review`, `follow_up`. |
| `person_id` | `string (uuid)` | no | Customer linked to the task. |
| `charge_id` | `string (uuid)` | no | Charge linked to the task. |
| `assigned_to_id` | `string (uuid)` | no | User assigned to the task. |
| `description` | `string` | no | Detailed description. |
| `priority` | `string` | no | Initial priority (`low` \| `medium` \| `high` \| `urgent`; default `medium`). Values: `low`, `medium`, `high`, `urgent`. |
| `status` | `string` | no | Initial status (default `pending`). Values: `pending`, `in_progress`, `completed`, `cancelled`. |
| `due_at` | `string (date-time)` | no | Task due date (ISO 8601). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.tasks.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Task created (includes customer, assignee and charge summaries). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## tasks bulk-assign

Bulk-assign tasks

```text
kobana dunning tasks bulk-assign [OPTIONS]
```

`POST /v1/tasks/bulk-assign`

Assigns (or clears, with `assignedToId: null`) the assignee of up to 100 tasks. The target user must belong to the organization. A cancelled task is returned in `failed` with `invalid_status`. A partial result is not an error.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | Target record IDs (1 to 100 per call). Duplicates are ignored. |
| `assigned_to_id` | `string (uuid)` | yes | Assignee user ID, or `null` to clear the assignment. |

More fields and nested objects: `kobana schema dunning.tasks.bulk-assign`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Per-item result of the bulk assignment. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## tasks bulk-complete

Bulk-complete tasks

```text
kobana dunning tasks bulk-complete [OPTIONS]
```

`POST /v1/tasks/bulk-complete`

Completes up to 100 tasks in one call. An already completed task is returned in `failed` with `already_completed`; a cancelled one, with `invalid_status`. A partial result is not an error.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | Target record IDs (1 to 100 per call). Duplicates are ignored. |

More fields and nested objects: `kobana schema dunning.tasks.bulk-complete`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Per-item result of the bulk completion. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## tasks get

Get task

```text
kobana dunning tasks get [ID] [OPTIONS]
```

`GET /v1/tasks/{id}`

Returns a task by ID, with customer, assignee and charge summaries.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Task found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## tasks update

Update task

```text
kobana dunning tasks update [ID] [OPTIONS]
```

`PUT /v1/tasks/{id}`

Updates a task’s data (partial).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_id` | `string (uuid)` | no | Customer linked to the task. |
| `charge_id` | `string (uuid)` | no | Charge linked to the task. |
| `assigned_to_id` | `string (uuid)` | no | User assigned to the task. |
| `title` | `string` | no | Task title. |
| `description` | `string` | no | Detailed description. |
| `task_type` | `string` | no | Type (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Values: `call`, `email`, `visit`, `review`, `follow_up`. |
| `priority` | `string` | no | Priority (`low` \| `medium` \| `high` \| `urgent`). Values: `low`, `medium`, `high`, `urgent`. |
| `status` | `string` | no | New status; `completed` auto-fills `completed_at`. Values: `pending`, `in_progress`, `completed`, `cancelled`. |
| `due_at` | `string (date-time)` | no | Task due date (ISO 8601). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_data_mode` | `string` | no | How `custom_data` is applied on update: `replace` (default) overwrites the whole object; `merge` deep-merges the payload over the stored value — nested objects merge, scalars/arrays/null overwrite the key. Values: `replace`, `merge`. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.tasks.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Task updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## tasks complete

Complete task

```text
kobana dunning tasks complete [ID] [OPTIONS]
```

`POST /v1/tasks/{id}/complete`

Marks the task as completed and fills `completed_at`. Returns 409 if the task is already completed or cancelled.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Task completed. |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |
