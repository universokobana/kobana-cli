---
id: team-roles
title: "kobana billing team-roles"
sidebar_label: "team-roles"
---

# kobana billing team-roles


Commands of `kobana billing team-roles`. Every global option ([Global options](/manual/commands/global-options)) applies here.

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
kobana billing team-roles list [OPTIONS]
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
| `is_active` | `boolean,null` | no | Filter by active (`true`) or inactive (`false`) roles. |
| `is_system` | `boolean,null` | no | Filter by system (`true`) or custom (`false`) roles. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of roles. |
| `404` | Resource not found in the current organization. |

## team-roles create

Create custom role

```text
kobana billing team-roles create [OPTIONS]
```

`POST /v1/team_roles`

Creates a custom role. `slug` must start with a lowercase letter and contain only lowercase letters, digits and `_`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Role name (shown in the UI). |
| `slug` | `string` | yes | Stable slug used to reference the role (e.g.: `billing_manager`). |
| `permissions` | `array<string>` | yes | List of permission slugs granted by the role. |
| `description` | `string` | no | Optional description of what the role grants. |
| `is_active` | `boolean` | no | Indicates whether the role is active and can be assigned. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.team-roles.create`.

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
kobana billing team-roles update [OPTIONS]
```

`PUT /v1/team_roles`

Looks the role up by `id`, `public_id` or `external_id`. The `slug` carries no unique constraint and therefore does **not** identify a row. If found, updates it; otherwise creates it and returns `201`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Role name (shown in the UI). |
| `slug` | `string` | no | Stable slug used to reference the role (e.g.: `billing_manager`). |
| `description` | `string` | no | Optional description of what the role grants. |
| `permissions` | `array<string>` | no | List of permission slugs granted by the role. |
| `is_active` | `boolean` | no | Indicates whether the role is active and can be assigned. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.team-roles.update`.

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
kobana billing team-roles get [ID] [OPTIONS]
```

`GET /v1/team_roles/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Role detail. |
| `404` | Resource not found in the current organization. |

## team-roles delete

Remove role

```text
kobana billing team-roles delete [ID] [OPTIONS]
```

`DELETE /v1/team_roles/{id}`

Soft-deletes the role. System roles (`is_system = true`) cannot be removed.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Role removed. |
| `404` | Resource not found in the current organization. |
