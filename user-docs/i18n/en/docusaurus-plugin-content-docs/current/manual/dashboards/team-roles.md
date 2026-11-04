---
id: team-roles
title: "kobana dashboards team-roles"
sidebar_label: "team-roles"
---

# kobana dashboards team-roles


Commands of `kobana dashboards team-roles`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`team-roles list`](#team-roles-list) | `GET /v1/team_roles` | List team roles |
| [`team-roles create`](#team-roles-create) | `POST /v1/team_roles` | Create custom role |
| [`team-roles update`](#team-roles-update) | `PUT /v1/team_roles` | Create or update a team role |
| [`team-roles get`](#team-roles-get) | `GET /v1/team_roles/{id}` | Get role |
| [`team-roles delete`](#team-roles-delete) | `DELETE /v1/team_roles/{id}` | Remove role |

## team-roles list

List team roles

```text
kobana dashboards team-roles list [OPTIONS]
```

`GET /v1/team_roles`

Returns the paginated list of roles configured for the organization team. Includes both system roles (`is_system = true`) and custom roles.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `search` | `string` | no | Text search by name or slug. |
| `is_active` | `string` | no | Filter by active (`true`, the default), inactive (`false`) or all (`all`) roles. Values: `true`, `false`, `all`. |
| `is_system` | `boolean,null` | no | Filter by system (`true`) or custom (`false`) roles. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of roles. |
| `404` | Resource not found in the current organization. |

## team-roles create

Create custom role

```text
kobana dashboards team-roles create [OPTIONS]
```

`POST /v1/team_roles`

Creates a custom role. `slug` must start with a lowercase letter and contain only lowercase letters, digits and `_`. With a person token, that person must have access to every workspace of the organization (or be an owner): a role applies in all of them. Otherwise 403.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Role name (shown in the UI). |
| `slug` | `string` | yes | Stable slug used to reference the role (e.g.: `support_manager`). |
| `permissions` | `array<string>` | yes | List of permission slugs granted by the role. |
| `description` | `string` | no | Optional description of what the role grants. |
| `is_active` | `boolean` | no | Indicates whether the role is active and can be assigned. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema dashboards.team-roles.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Role created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## team-roles update

Create or update a team role

```text
kobana dashboards team-roles update [OPTIONS]
```

`PUT /v1/team_roles`

Looks the role up by `id`, `public_id` or `external_id`. The `slug` carries no unique constraint and therefore does **not** identify a row. If found, updates it; otherwise creates it and returns `201`. With a person token, that person must have access to every workspace of the organization (or be an owner): a role applies in all of them. Otherwise 403.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Role name (shown in the UI). |
| `slug` | `string` | no | Stable slug used to reference the role (e.g.: `support_manager`). |
| `description` | `string` | no | Optional description of what the role grants. |
| `permissions` | `array<string>` | no | List of permission slugs granted by the role. |
| `is_active` | `boolean` | no | Indicates whether the role is active and can be assigned. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema dashboards.team-roles.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## team-roles get

Get role

```text
kobana dashboards team-roles get [ID] [OPTIONS]
```

`GET /v1/team_roles/{id}`

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Role detail. |
| `404` | Resource not found in the current organization. |

## team-roles delete

Remove role

```text
kobana dashboards team-roles delete [ID] [OPTIONS]
```

`DELETE /v1/team_roles/{id}`

Soft-deletes the role. System roles (`is_system = true`) cannot be removed. With a person token, that person must have access to every workspace of the organization (or be an owner): a role applies in all of them. Otherwise 403.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Role removed. |
| `404` | Resource not found in the current organization. |
