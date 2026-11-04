---
id: workspaces
title: "kobana dashboards workspaces"
sidebar_label: "workspaces"
---

# kobana dashboards workspaces


Commands of `kobana dashboards workspaces`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | List workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Create workspace |
| [`workspaces import`](#workspaces-import) | `POST /v1/workspaces/import` | Import workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Get workspace |
| [`workspaces update`](#workspaces-update) | `PATCH /v1/workspaces/{id}` | Rename workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Delete workspace |
| [`workspaces export`](#workspaces-export) | `GET /v1/workspaces/{id}/export` | Export workspace |
| [`workspaces usage`](#workspaces-usage) | `GET /v1/workspaces/{id}/usage` | Count what deleting the workspace would hide |

## workspaces list

List workspaces

```text
kobana dashboards workspaces list [OPTIONS]
```

`GET /v1/workspaces`

Lists the workspaces of the organization you can open, with your role in each — the same ones as the dashboard switcher. A machine token sees every workspace of the organization. Does not require `X-Workspace-Id`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `search` | `string` | no | Substring of the workspace name, case-insensitive. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of workspaces. |
| `404` | Resource not found in the current organization. |

## workspaces create

Create workspace

```text
kobana dashboards workspaces create [OPTIONS]
```

`POST /v1/workspaces`

Creates an empty workspace in the organization. The token person becomes its owner. Created by a machine token, the workspace has no owner: the organization owners receive it on their next sign-in, and `PUT /dashboard_users/{id}/workspaces` grants it to other members.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Workspace name, 1 to 100 characters. |

More fields and nested objects: `kobana schema dashboards.workspaces.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Workspace created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## workspaces import

Import workspace

```text
kobana dashboards workspaces import [OPTIONS]
```

`POST /v1/workspaces/import`

Restores a file from `GET /workspaces/{id}/export` into a NEW workspace owned by the token person. Answers what could not be brought back (`warnings`) so it can be finished by hand. Requires a person token (a machine token gets 403) holding the permissions to create workspaces, configure the portal and create data sources. The entities in the file go through the same checks as when created one by one (keys, expressions, the field limit of the workspace). The body is limited to 2 MB.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `data` | `object` | yes | The exported file, as returned by the export. |
| `name` | `string` | no | Name of the new workspace; defaults to the one in the file. |

More fields and nested objects: `kobana schema dashboards.workspaces.import`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Workspace imported. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | The file is not a valid workspace export (`INVALID_WORKSPACE_FILE`). |

## workspaces get

Get workspace

```text
kobana dashboards workspaces get [ID] [OPTIONS]
```

`GET /v1/workspaces/{id}`

Returns a workspace you can open. A workspace you hold no grant on answers 404.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Workspace found. |
| `404` | Resource not found in the current organization. |

## workspaces update

Rename workspace

```text
kobana dashboards workspaces update [ID] [OPTIONS]
```

`PATCH /v1/workspaces/{id}`

Renames the workspace. Only the workspace owners can (403 for a plain member).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Workspace name, 1 to 100 characters. |

More fields and nested objects: `kobana schema dashboards.workspaces.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Workspace renamed. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## workspaces delete

Delete workspace

```text
kobana dashboards workspaces delete [ID] [OPTIONS]
```

`DELETE /v1/workspaces/{id}`

Deletes the workspace (soft delete), for its owners only: the portal goes offline, connectors stop syncing and members lose access. The last workspace of the organization cannot be deleted.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Workspace deleted. |
| `404` | Resource not found in the current organization. |
| `422` | It is the last workspace of the organization (`WORKSPACE_LAST`). |

## workspaces export

Export workspace

```text
kobana dashboards workspaces export [ID] [OPTIONS]
```

`GET /v1/workspaces/{id}/export`

Downloads the workspace configuration (portal builder, portal access and settings, user mapping, connectors and entities) as a JSON file, for its owners only. Secrets and data stay out. The response is the file itself, with no `data` envelope; send it as `data` to `POST /workspaces/import`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Export file (`Content-Disposition: attachment`). |
| `404` | Resource not found in the current organization. |

## workspaces usage

Count what deleting the workspace would hide

```text
kobana dashboards workspaces usage [ID] [OPTIONS]
```

`GET /v1/workspaces/{id}/usage`

Counts what deleting the workspace would hide — the same summary as the dashboard delete dialog.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Workspace counts. |
| `404` | Resource not found in the current organization. |
