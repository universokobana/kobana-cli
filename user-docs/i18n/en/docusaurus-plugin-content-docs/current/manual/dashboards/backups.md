---
id: backups
title: "kobana dashboards backups"
sidebar_label: "backups"
---

# kobana dashboards backups


Commands of `kobana dashboards backups`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`backups list`](#backups-list) | `GET /v1/backups` | List backups |
| [`backups create`](#backups-create) | `POST /v1/backups` | Request backup |
| [`backups restore`](#backups-restore) | `POST /v1/backups/restore` | Restore backup |
| [`backups get`](#backups-get) | `GET /v1/backups/{id}` | Get backup |
| [`backups delete`](#backups-delete) | `DELETE /v1/backups/{id}` | Delete backup |
| [`backups cancel`](#backups-cancel) | `POST /v1/backups/{id}/cancel` | Cancel restore |
| [`backups download`](#backups-download) | `GET /v1/backups/{id}/download` | Download backup |

## backups list

List backups

```text
kobana dashboards backups list [OPTIONS]
```

`GET /v1/backups`

Lists the organization backups and restores (metadata only) and whether a new backup may be requested now (`backup_allowed`, `next_backup_at` — one every 12 hours in production). Organization-level.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `type` | `string` | no | `backup` or `restore`. Values: `backup`, `restore`. |
| `status` | `string` | no | Filter by status. Values: `pending`, `processing`, `completed`, `failed`, `canceled`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of backups. |
| `404` | Resource not found in the current organization. |

## backups create

Request backup

```text
kobana dashboards backups create [OPTIONS]
```

`POST /v1/backups`

Queues a full backup (ZIP) of the organization, every workspace included. Asynchronous: 202; poll `GET /backups/{id}`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `202` | Backup queued. |
| `404` | Resource not found in the current organization. |

## backups restore

Restore backup

```text
kobana dashboards backups restore [OPTIONS]
```

`POST /v1/backups/restore`

DESTRUCTIVE: restores a backup into the workspace of the call (`X-Workspace-Id`), rewriting portal users matched by e-mail/public id. Send `backup_id` (a completed backup of this organization — its file is copied) OR `file_name` (`.zip`) + `content_base64` (bounded by the 2 MB request limit). Requires `settings.update` and access to every workspace. Asynchronous: 202; follow it on `GET /backups/{id}` and cancel with `/cancel`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `backup_id` | `string (uuid)` | no | Completed backup of this organization to restore. |
| `file_name` | `string` | no | `.zip` file name, together with `content_base64`. |
| `content_base64` | `string` | no | The backup ZIP in base64. |

More fields and nested objects: `kobana schema dashboards.backups.restore`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | Restore queued (a `restore` record). |
| `400` | Missing body, invalid JSON or incompatible fields. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## backups get

Get backup

```text
kobana dashboards backups get [ID] [OPTIONS]
```

`GET /v1/backups/{id}`

Status and progress of a backup or restore. An in-progress restore whose queue job is gone is marked `failed`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The backup. |
| `404` | Resource not found in the current organization. |

## backups delete

Delete backup

```text
kobana dashboards backups delete [ID] [OPTIONS]
```

`DELETE /v1/backups/{id}`

Deletes the record and its file from storage. Permanent.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Removed. |
| `404` | Resource not found in the current organization. |

## backups cancel

Cancel restore

```text
kobana dashboards backups cancel [ID] [OPTIONS]
```

`POST /v1/backups/{id}/cancel`

Stops a pending or running restore. What was already restored stays. 422 for a backup or a restore that already ended.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The backup. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## backups download

Download backup

```text
kobana dashboards backups download [ID] [OPTIONS]
```

`GET /v1/backups/{id}/download`

Returns the backup ZIP. Requires `backups.export` and, for a person token, being an organization owner or holding access to every workspace — the backup holds all of them. Audited.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The ZIP file. |
| `404` | Resource not found in the current organization. |
