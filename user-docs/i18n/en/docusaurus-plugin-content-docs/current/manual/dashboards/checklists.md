---
id: checklists
title: "kobana dashboards checklists"
sidebar_label: "checklists"
---

# kobana dashboards checklists


Commands of `kobana dashboards checklists`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`checklists progress`](#checklists-progress) | `GET /v1/checklists/{id}/progress` | Get tenant progress |
| [`checklists progress file`](#checklists-progress-file) | `GET /v1/checklists/{id}/progress/file` | Download task file |
| [`checklists progress file create`](#checklists-progress-file-create) | `POST /v1/checklists/{id}/progress/file` | Upload task file |
| [`checklists progress file delete`](#checklists-progress-file-delete) | `DELETE /v1/checklists/{id}/progress/file` | Remove task file |
| [`checklists progress update`](#checklists-progress-update) | `PUT /v1/checklists/{id}/progress` | Update tenant progress |
| [`checklists progress delete`](#checklists-progress-delete) | `DELETE /v1/checklists/{id}/progress` | Reset tenant progress |
| [`checklists list`](#checklists-list) | `GET /v1/checklists` | List checklists |
| [`checklists get`](#checklists-get) | `GET /v1/checklists/{id}` | Get checklist |

## checklists progress

Get tenant progress

```text
kobana dashboards checklists progress [ID] [OPTIONS] [COMMAND]
```

`GET /v1/checklists/{id}/progress`

Where a tenant stands on a checklist, task by task, with done/total per stage. Every portal user of the same tenant sees this same progress.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `tenant_id` | `string` | no | Tenant ID of the portal users who own the progress — every user of the same tenant sees the same checklist. Give this or `portal_user_id`. |
| `portal_user_id` | `string` | no | A portal user (id or external_id): the progress of their tenant is the one meant (a user without a tenant has a checklist of their own). Give this or `tenant_id`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The tenant's progress. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## checklists progress file

Download task file

```text
kobana dashboards checklists progress file [ID] [OPTIONS] [COMMAND]
```

`GET /v1/checklists/{id}/progress/file`

The file the tenant sent to a Request task.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `tenant_id` | `string` | no | Tenant ID of the portal users who own the progress — every user of the same tenant sees the same checklist. Give this or `portal_user_id`. |
| `portal_user_id` | `string` | no | A portal user (id or external_id): the progress of their tenant is the one meant (a user without a tenant has a checklist of their own). Give this or `tenant_id`. |
| `task_id` | `string` | yes | Task ID. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The file content. |
| `404` | Resource not found in the current organization. |

## checklists progress file create

Upload task file

```text
kobana dashboards checklists progress file create [ID] [OPTIONS]
```

`POST /v1/checklists/{id}/progress/file`

Sends, for the tenant, the file of a Request task (`content_base64`), replacing the one before.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `task_id` | `string` | yes |  |
| `name` | `string` | yes |  |
| `content_base64` | `string` | yes | The file content in base64 (up to the 2 MB request limit). |
| `tenant_id` | `string` | no | Tenant ID of the portal users who own the progress — every user of the same tenant sees the same checklist. Give this or `portal_user_id`. |
| `portal_user_id` | `string` | no | A portal user (id or external_id): the progress of their tenant is the one meant (a user without a tenant has a checklist of their own). Give this or `tenant_id`. |
| `content_type` | `string` | no |  |

More fields and nested objects: `kobana schema dashboards.checklists.progress.file.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | The file recorded. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## checklists progress file delete

Remove task file

```text
kobana dashboards checklists progress file delete [ID] [OPTIONS]
```

`DELETE /v1/checklists/{id}/progress/file`

Takes back the file the tenant sent to a task.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `tenant_id` | `string` | no | Tenant ID of the portal users who own the progress — every user of the same tenant sees the same checklist. Give this or `portal_user_id`. |
| `portal_user_id` | `string` | no | A portal user (id or external_id): the progress of their tenant is the one meant (a user without a tenant has a checklist of their own). Give this or `tenant_id`. |
| `task_id` | `string` | yes | Task ID. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Remove task file |
| `404` | Resource not found in the current organization. |

## checklists progress update

Update tenant progress

```text
kobana dashboards checklists progress update [ID] [OPTIONS]
```

`PUT /v1/checklists/{id}/progress`

Sets the status of one or more tasks for the tenant (creates the progress on first use). The tenant's users see the change in the portal.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `task_ids` | `array<string>` | yes |  |
| `status` | `string` | yes | `pending`, `in_progress` or `done`. A task nobody touched is `pending`. Values: `pending`, `in_progress`, `done`. |
| `tenant_id` | `string` | no | Tenant ID of the portal users who own the progress — every user of the same tenant sees the same checklist. Give this or `portal_user_id`. |
| `portal_user_id` | `string` | no | A portal user (id or external_id): the progress of their tenant is the one meant (a user without a tenant has a checklist of their own). Give this or `tenant_id`. |

More fields and nested objects: `kobana schema dashboards.checklists.progress.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The tenant's progress. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## checklists progress delete

Reset tenant progress

```text
kobana dashboards checklists progress delete [ID] [OPTIONS]
```

`DELETE /v1/checklists/{id}/progress`

Brings the tasks of `task_ids` (all of them, without it) back to pending and forgets the files sent. It is an update: the history stays in events and audit.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `tenant_id` | `string` | no | Tenant ID of the portal users who own the progress — every user of the same tenant sees the same checklist. Give this or `portal_user_id`. |
| `portal_user_id` | `string` | no | A portal user (id or external_id): the progress of their tenant is the one meant (a user without a tenant has a checklist of their own). Give this or `tenant_id`. |
| `task_ids` | `string` | no | Task IDs, comma separated. Without it, every task of the checklist. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The tenant's progress. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## checklists list

List checklists

```text
kobana dashboards checklists list [OPTIONS]
```

`GET /v1/checklists`

The Task List blocks of the workspace's published portal, page by page. The tasks are written in the builder; the progress belongs to each tenant (`GET /checklists/{id}/progress`).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The checklists. |
| `404` | Resource not found in the current organization. |

## checklists get

Get checklist

```text
kobana dashboards checklists get [ID] [OPTIONS]
```

`GET /v1/checklists/{id}`

One checklist of the published portal: stages and visible tasks.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The checklist. |
| `404` | Resource not found in the current organization. |
