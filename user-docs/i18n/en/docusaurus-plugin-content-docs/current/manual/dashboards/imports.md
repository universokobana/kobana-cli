---
id: imports
title: "kobana dashboards imports"
sidebar_label: "imports"
---

# kobana dashboards imports


Commands of `kobana dashboards imports`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports` | List imports |
| [`imports create`](#imports-create) | `POST /v1/imports` | Import portal users from CSV |
| [`imports get`](#imports-get) | `GET /v1/imports/{id}` | Get import |

## imports list

List imports

```text
kobana dashboards imports list [OPTIONS]
```

`GET /v1/imports`

Lists the import requests of the workspace, with success and error counters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `resource_type` | `string` | no | Imported resource, e.g. `portal_users`. |
| `status` | `string` | no | Filter by import status. Values: `pending`, `processing`, `completed`, `completed_with_errors`, `failed`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of imports. |
| `404` | Resource not found in the current organization. |

## imports create

Import portal users from CSV

```text
kobana dashboards imports create [OPTIONS]
```

`POST /v1/imports`

Sends a base64 CSV and queues the import of portal users into the workspace. The dashboard uploads the file first; the API takes it in the body (2 MB request limit). Follows the organization setting "allow creating portal users" (403 when off). Asynchronous: 202; poll `GET /imports/{id}`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `resource_type` | `string` | yes | What to import. Today: `portal_users`. Values: `portal_users`. |
| `file_name` | `string` | yes | Original file name, e.g. `users.csv`. |
| `content_base64` | `string` | yes | The CSV, base64-encoded. |
| `content_type` | `string` | no | File type. Default `text/csv`. |

More fields and nested objects: `kobana schema dashboards.imports.create`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | Import queued. |
| `400` | Missing body, invalid JSON or incompatible fields. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## imports get

Get import

```text
kobana dashboards imports get [ID] [OPTIONS]
```

`GET /v1/imports/{id}`

Status, counters and per-row errors of an import. An import still `pending` 2 minutes after the request is shown as `failed` (stalled queue); the read does not change the record.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The import. |
| `404` | Resource not found in the current organization. |
