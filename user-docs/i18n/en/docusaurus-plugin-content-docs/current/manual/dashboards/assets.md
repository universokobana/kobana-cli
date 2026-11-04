---
id: assets
title: "kobana dashboards assets"
sidebar_label: "assets"
---

# kobana dashboards assets


Commands of `kobana dashboards assets`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`assets list`](#assets-list) | `GET /v1/assets` | List assets |
| [`assets create`](#assets-create) | `POST /v1/assets` | Create asset |
| [`assets presign`](#assets-presign) | `POST /v1/assets/presign` | Get an upload URL |
| [`assets get`](#assets-get) | `GET /v1/assets/{id}` | Get asset |
| [`assets update`](#assets-update) | `PATCH /v1/assets/{id}` | Update asset |
| [`assets delete`](#assets-delete) | `DELETE /v1/assets/{id}` | Delete asset |
| [`assets download`](#assets-download) | `GET /v1/assets/{id}/download` | Download asset |

## assets list

List assets

```text
kobana dashboards assets list [OPTIONS]
```

`GET /v1/assets`

The file library of the workspace — images and documents the portal shows or offers for download. Sortable by `created_at` (default), `name` or `size_bytes`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `kind` | `string` | no | Filter by kind: `image`, `document` or `other`. Values: `image`, `document`, `other`. |
| `visibility` | `string` | no | Who may download it: `public` (anyone with the link), `restricted` (users signed in to the workspace portal) or `private` (dashboard members only). New files start `restricted`. Values: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | no | Only the files of this tenant. |
| `type_id` | `string` | no | Only the files of this type. |
| `tag` | `string` | no | Only the files carrying this tag. |
| `search` | `string` | no | Part of the file name (case-insensitive). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of assets. |
| `404` | Resource not found in the current organization. |

## assets create

Create asset

```text
kobana dashboards assets create [OPTIONS]
```

`POST /v1/assets`

Records a file in the library, one of two ways: the `key` of a file already sent through the URL of `POST /assets/presign`, or the content itself in `content_base64`. The file starts `restricted` unless `visibility` says otherwise. Fires the `asset.created` webhook.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Display name, with its extension (required). The extension decides the accepted type when `content_type` is absent. |
| `key` | `string` | no | The `key` returned by `POST /assets/presign`, after sending the file to the signed URL. Send `key` or `content_base64`, never both. |
| `content_base64` | `string` | no | The file content in base64, for small files: the request body is capped at 2 MB (about 1.5 MB of file). For bigger files, use `POST /assets/presign`. |
| `content_type` | `string` | no | MIME type. When omitted, it is guessed from the extension. Accepted: PNG, JPEG, GIF, WebP and AVIF images; PDF, text, CSV, Markdown, JSON, XML, Office and ZIP. SVG and HTML are not accepted. |
| `alt_text` | `string` | no | Alternative text of an image (accessibility). |
| `visibility` | `string` | no | Who may download it: `public` (anyone with the link), `restricted` (users signed in to the workspace portal) or `private` (dashboard members only). New files start `restricted`. Values: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | no | Tenant ID of the portal users who see the file — every user of the same tenant sees the same files, on the User Files blocks. `null` = a workspace file, with no tenant. |
| `type_id` | `string (uuid)` | no | File type (`GET /asset_types`: Proposals, Contracts…). A User Files block lists the types it picks; a typed file with no tenant shows to every user on the blocks of that type. `null` = no type. |
| `tags` | `array<string>` | no | Tags of the file (up to 20, each up to 50 characters; repeated ones are dropped). Replaces the current ones. |

More fields and nested objects: `kobana schema dashboards.assets.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Asset recorded. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## assets presign

Get an upload URL

```text
kobana dashboards assets presign [OPTIONS]
```

`POST /v1/assets/presign`

A short-lived signed URL to send the file straight to storage with `PUT` — the way for files over the 2 MB body limit. After the `PUT`, record the file at `POST /assets` with the returned `key`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `filename` | `string` | yes | File name with its extension. |
| `size` | `integer` | yes | Exact size, in bytes, of the file you will send (max 25 MB). |
| `content_type` | `string` | no | MIME type. When omitted, it is guessed from the extension. Accepted: PNG, JPEG, GIF, WebP and AVIF images; PDF, text, CSV, Markdown, JSON, XML, Office and ZIP. SVG and HTML are not accepted. |

More fields and nested objects: `kobana schema dashboards.assets.presign`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Upload URL issued. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## assets get

Get asset

```text
kobana dashboards assets get [ID] [OPTIONS]
```

`GET /v1/assets/{id}`

One file of the library, with its link, visibility, type and size.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The asset. |
| `404` | Resource not found in the current organization. |

## assets update

Update asset

```text
kobana dashboards assets update [ID] [OPTIONS]
```

`PATCH /v1/assets/{id}`

Renames the file, changes its alternative text or its visibility. Making it public opens its storage URL to anyone; `restricted` or `private` closes it.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Display name of the file (by default the original name, with its extension). |
| `alt_text` | `string` | no | Alternative text of an image. Send `null` to clear it. |
| `visibility` | `string` | no | Who may download it: `public` (anyone with the link), `restricted` (users signed in to the workspace portal) or `private` (dashboard members only). New files start `restricted`. Values: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | no | Tenant ID of the portal users who see the file — every user of the same tenant sees the same files, on the User Files blocks. `null` = a workspace file, with no tenant. |
| `type_id` | `string (uuid)` | no | File type (`GET /asset_types`: Proposals, Contracts…). A User Files block lists the types it picks; a typed file with no tenant shows to every user on the blocks of that type. `null` = no type. |
| `tags` | `array<string>` | no | Tags of the file (up to 20, each up to 50 characters; repeated ones are dropped). Replaces the current ones. |

More fields and nested objects: `kobana schema dashboards.assets.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Asset updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## assets delete

Delete asset

```text
kobana dashboards assets delete [ID] [OPTIONS]
```

`DELETE /v1/assets/{id}`

Removes the file from the library. The stored file stays in storage, so links already published do not break. Fires the `asset.deleted` webhook.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Asset deleted. |
| `404` | Resource not found in the current organization. |

## assets download

Download asset

```text
kobana dashboards assets download [ID] [OPTIONS]
```

`GET /v1/assets/{id}/download`

The file content, whatever its visibility, with its own `Content-Type`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `inline` | `string` | no | With `true`, images and PDF come for display in place (`Content-Disposition: inline`); other types always come as an attachment. Values: `true`, `false`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The file content. |
| `404` | Resource not found in the current organization. |
