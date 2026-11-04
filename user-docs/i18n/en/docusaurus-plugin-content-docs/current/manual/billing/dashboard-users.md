---
id: dashboard-users
title: "kobana billing dashboard-users"
sidebar_label: "dashboard-users"
---

# kobana billing dashboard-users


Commands of `kobana billing dashboard-users`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`dashboard-users list`](#dashboard-users-list) | `GET /v1/dashboard_users` | List dashboard users |
| [`dashboard-users create`](#dashboard-users-create) | `POST /v1/dashboard_users` | Invite dashboard user |
| [`dashboard-users update`](#dashboard-users-update) | `PUT /v1/dashboard_users` | Create or update a dashboard user |
| [`dashboard-users get`](#dashboard-users-get) | `GET /v1/dashboard_users/{id}` | Get dashboard user |
| [`dashboard-users delete`](#dashboard-users-delete) | `DELETE /v1/dashboard_users/{id}` | Remove dashboard user |

## dashboard-users list

List dashboard users

```text
kobana billing dashboard-users list [OPTIONS]
```

`GET /v1/dashboard_users`

Returns a paginated list of dashboard users (team members) for the current organization.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `search` | `string` | no | Text search by name or email. |
| `status` | `string` | no | Filter by status: `all` (default), `active`, `pending_invitation` or `suspended`. Values: `all`, `active`, `pending_invitation`, `suspended`. |
| `role` | `string` | no | Filter by users that have the given role (role slug). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of dashboard users. |
| `404` | Resource not found in the current organization. |

## dashboard-users create

Invite dashboard user

```text
kobana billing dashboard-users create [OPTIONS]
```

`POST /v1/dashboard_users`

Invites a new member to the organization team. `email` and at least one role in `role_ids` are required. The user receives an email invite and starts with `pending_invitation` status.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string (email)` | yes | Email of the user to invite. |
| `role_ids` | `array<string (uuid)>` | yes | List of role UUIDs (`team_roles`) to assign to the user. Must contain at least one item. |
| `name` | `string` | no | User name (optional — may be filled in by the user on first login). |
| `external_id` | `string` | no | External user identifier in Kobana OAuth, when already known. |
| `permissions` | `array<string>` | no | Additional ad-hoc permissions (on top of those inherited from roles). Accepts the same permission slugs. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.dashboard-users.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | User invited. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## dashboard-users update

Create or update a dashboard user

```text
kobana billing dashboard-users update [OPTIONS]
```

`PUT /v1/dashboard_users`

Looks the user up by `id`, `public_id`, `external_id` or `email` — the last one is unique per organization. If found, updates it; otherwise creates it and returns `201`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string (email)` | no | User's email. Besides being a resource field, it doubles as the natural lookup key. |
| `name` | `string` | no | User name (optional — may be filled in by the user on first login). |
| `external_id` | `string` | no | External user identifier in Kobana OAuth, when already known. |
| `role_ids` | `array<string (uuid)>` | no | List of role UUIDs (`team_roles`) to assign to the user. Must contain at least one item. |
| `permissions` | `array<string>` | no | Additional ad-hoc permissions (on top of those inherited from roles). Accepts the same permission slugs. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.dashboard-users.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## dashboard-users get

Get dashboard user

```text
kobana billing dashboard-users get [ID] [OPTIONS]
```

`GET /v1/dashboard_users/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Dashboard user detail. |
| `404` | Resource not found in the current organization. |

## dashboard-users delete

Remove dashboard user

```text
kobana billing dashboard-users delete [ID] [OPTIONS]
```

`DELETE /v1/dashboard_users/{id}`

Removes the dashboard user from the organization. Irreversible — the user must be invited again to regain access.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | User removed. |
| `404` | Resource not found in the current organization. |
