---
id: asset-types
title: "kobana dashboards asset-types"
sidebar_label: "asset-types"
---

# kobana dashboards asset-types


Commands of `kobana dashboards asset-types`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`asset-types list`](#asset-types-list) | `GET /v1/asset_types` | List asset types |
| [`asset-types create`](#asset-types-create) | `POST /v1/asset_types` | Create asset type |
| [`asset-types get`](#asset-types-get) | `GET /v1/asset_types/{id}` | Get asset type |
| [`asset-types update`](#asset-types-update) | `PATCH /v1/asset_types/{id}` | Rename asset type |
| [`asset-types delete`](#asset-types-delete) | `DELETE /v1/asset_types/{id}` | Delete asset type |

## asset-types list

List asset types

```text
kobana dashboards asset-types list [OPTIONS]
```

`GET /v1/asset_types`

The file types of the workspace, by name. A User Files block of the portal lists the files of the types it picks.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The asset types. |
| `404` | Resource not found in the current organization. |

## asset-types create

Create asset type

```text
kobana dashboards asset-types create [OPTIONS]
```

`POST /v1/asset_types`

Creates a file type. Then use its `id` as the `type_id` of files.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Type name (Proposals, Contracts…), unique in the workspace ignoring case; up to 100 characters. |

More fields and nested objects: `kobana schema dashboards.asset-types.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | The created type. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## asset-types get

Get asset type

```text
kobana dashboards asset-types get [ID] [OPTIONS]
```

`GET /v1/asset_types/{id}`

One file type.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The asset type. |
| `404` | Resource not found in the current organization. |

## asset-types update

Rename asset type

```text
kobana dashboards asset-types update [ID] [OPTIONS]
```

`PATCH /v1/asset_types/{id}`

Changes the name of a file type.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Type name (Proposals, Contracts…), unique in the workspace ignoring case; up to 100 characters. |

More fields and nested objects: `kobana schema dashboards.asset-types.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The updated type. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## asset-types delete

Delete asset type

```text
kobana dashboards asset-types delete [ID] [OPTIONS]
```

`DELETE /v1/asset_types/{id}`

Removes the type. Its files stay, without a type — and leave the User Files blocks that listed it.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The type was removed. |
| `404` | Resource not found in the current organization. |
