---
id: exports
title: "kobana dashboards exports"
sidebar_label: "exports"
---

# kobana dashboards exports


Commands of `kobana dashboards exports`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`exports list`](#exports-list) | `GET /v1/exports` | List exports |
| [`exports create`](#exports-create) | `POST /v1/exports` | Request export |
| [`exports get`](#exports-get) | `GET /v1/exports/{id}` | Get export |
| [`exports download`](#exports-download) | `GET /v1/exports/{id}/download` | Download export file |

## exports list

List exports

```text
kobana dashboards exports list [OPTIONS]
```

`GET /v1/exports`

Lists the export requests of the workspace.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `resource_type` | `string` | no | Exported resource, e.g. `portal_users`. |
| `status` | `string` | no | Filter by export status. Values: `pending`, `processing`, `completed`, `failed`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of exports. |
| `404` | Resource not found in the current organization. |

## exports create

Request export

```text
kobana dashboards exports create [OPTIONS]
```

`POST /v1/exports`

Queues an export of the workspace portal users as CSV, XLSX or JSON. Asynchronous: answers 202 with the `pending` export; poll `GET /exports/{id}` until `completed` and download from `/download`. The file expires after 7 days.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `resource_type` | `string` | yes | What to export. Today: `portal_users`. Values: `portal_users`. |
| `format` | `string` | no | `csv` (default), `xlsx` or `json`. Values: `csv`, `xlsx`, `json`. |
| `filters` | `object` | no | Optional filters of the exported rows. |

More fields and nested objects: `kobana schema dashboards.exports.create`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | Export queued. |
| `400` | Missing body, invalid JSON or incompatible fields. |
| `404` | Resource not found in the current organization. |

## exports get

Get export

```text
kobana dashboards exports get [ID] [OPTIONS]
```

`GET /v1/exports/{id}`

Status and metadata of one export request.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The export. |
| `404` | Resource not found in the current organization. |

## exports download

Download export file

```text
kobana dashboards exports download [ID] [OPTIONS]
```

`GET /v1/exports/{id}/download`

Returns the file of a completed export (the dashboard redirects to a signed URL; the API serves the bytes). 404 while it is not ready.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The file, as an attachment. |
| `404` | Resource not found in the current organization. |
