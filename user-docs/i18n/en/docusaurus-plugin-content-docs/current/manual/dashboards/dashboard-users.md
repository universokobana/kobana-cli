---
id: dashboard-users
title: "kobana dashboards dashboard-users"
sidebar_label: "dashboard-users"
---

# kobana dashboards dashboard-users


Commands of `kobana dashboards dashboard-users`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`dashboard-users workspaces`](#dashboard-users-workspaces) | `GET /v1/dashboard_users/{id}/workspaces` | Get member workspace access |
| [`dashboard-users workspaces update`](#dashboard-users-workspaces-update) | `PUT /v1/dashboard_users/{id}/workspaces` | Set member workspace access |
| [`dashboard-users list`](#dashboard-users-list) | `GET /v1/dashboard_users` | List dashboard users |
| [`dashboard-users create`](#dashboard-users-create) | `POST /v1/dashboard_users` | Invite dashboard user |
| [`dashboard-users update`](#dashboard-users-update) | `PUT /v1/dashboard_users` | Create or update a dashboard user |
| [`dashboard-users get`](#dashboard-users-get) | `GET /v1/dashboard_users/{id}` | Get dashboard user |
| [`dashboard-users delete`](#dashboard-users-delete) | `DELETE /v1/dashboard_users/{id}` | Remove dashboard user |
| [`dashboard-users resend`](#dashboard-users-resend) | `POST /v1/dashboard_users/{id}/resend` | Resend member invitation |

## dashboard-users workspaces

Get member workspace access

```text
kobana dashboards dashboard-users workspaces [ID] [OPTIONS] [COMMAND]
```

`GET /v1/dashboard_users/{id}/workspaces`

Lists every workspace of the organization, flagging the ones the member reaches — also the list of what can be granted.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Workspaces of the organization with the member access. |
| `404` | Resource not found in the current organization. |

## dashboard-users workspaces update

Set member workspace access

```text
kobana dashboards dashboard-users workspaces update [ID] [OPTIONS]
```

`PUT /v1/dashboard_users/{id}/workspaces`

Replaces the member set of workspaces. Nobody changes their own access (400), only an owner changes an owner access, and only an owner grants or revokes a workspace they cannot open themselves (403). A machine token may grant any workspace but never changes an owner access. Removing the last owner of a workspace answers 403.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `workspace_ids` | `array<string (uuid)>` | yes | The final set of workspaces the member should reach. Removed ones are paused, not deleted. |

More fields and nested objects: `kobana schema dashboards.dashboard-users.workspaces.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Workspaces of the organization with the member access. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## dashboard-users list

List dashboard users

```text
kobana dashboards dashboard-users list [OPTIONS]
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

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of dashboard users. |
| `404` | Resource not found in the current organization. |

## dashboard-users create

Invite dashboard user

```text
kobana dashboards dashboard-users create [OPTIONS]
```

`POST /v1/dashboard_users`

Invites a new member to the organization team. `email` and at least one role in `role_ids` are required. The user receives an email invite and starts with `pending_invitation` status.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string (email)` | yes | Email of the user to invite. |
| `role_ids` | `array<string (uuid)>` | yes | List of role UUIDs (`team_roles`) to assign to the user. Must contain at least one item. |
| `name` | `string` | no | User name (optional — may be filled in by the user on first login). |
| `external_id` | `string` | no | External user identifier, stored as an attribute of the member. It does not define the user identity: that is always resolved by `email`. |
| `permissions` | `array<string>` | no | Additional ad-hoc permissions (on top of those inherited from roles). Accepts the same permission slugs. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema dashboards.dashboard-users.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | User invited. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Invalid data. With code `IDENTITY_CONFLICT`, the given `external_id` is the identifier of another identity, with a different email. |

## dashboard-users update

Create or update a dashboard user

```text
kobana dashboards dashboard-users update [OPTIONS]
```

`PUT /v1/dashboard_users`

Looks the user up by `id`, `public_id`, `external_id` or `email` — the last one is unique per organization. If found, updates it; otherwise creates it and returns `201`. When it updates, with a person token, that person must have access to every workspace the member works in (or be an organization owner); otherwise 403.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string (email)` | no | User's email. Besides being a resource field, it doubles as the natural lookup key. |
| `name` | `string` | no | User name (optional — may be filled in by the user on first login). |
| `external_id` | `string` | no | External user identifier, stored as an attribute of the member. It does not define the user identity: that is always resolved by `email`. |
| `role_ids` | `array<string (uuid)>` | no | List of role UUIDs (`team_roles`) to assign to the user. Must contain at least one item. |
| `permissions` | `array<string>` | no | Additional ad-hoc permissions (on top of those inherited from roles). Accepts the same permission slugs. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema dashboards.dashboard-users.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Invalid data. With code `IDENTITY_CONFLICT`, the given `external_id` is the identifier of another identity, with a different email. |

## dashboard-users get

Get dashboard user

```text
kobana dashboards dashboard-users get [ID] [OPTIONS]
```

`GET /v1/dashboard_users/{id}`

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Dashboard user detail. |
| `404` | Resource not found in the current organization. |

## dashboard-users delete

Remove dashboard user

```text
kobana dashboards dashboard-users delete [ID] [OPTIONS]
```

`DELETE /v1/dashboard_users/{id}`

Removes the dashboard user from the organization. Irreversible — the user must be invited again to regain access. With a person token, that person must have access to every workspace the member works in (or be an organization owner); otherwise 403.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | User removed. |
| `404` | Resource not found in the current organization. |

## dashboard-users resend

Resend member invitation

```text
kobana dashboards dashboard-users resend [ID] [OPTIONS]
```

`POST /v1/dashboard_users/{id}/resend`

Re-sends the invitation e-mail, with a fresh sign-in link, to a member whose invitation is still pending. Capped at 5 e-mails per hour per recipient (429 above that).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Invitation re-sent. |
| `404` | Resource not found in the current organization. |
| `422` | The member has no pending invitation (`INVALID_STATUS`). |
| `502` | The e-mail could not be sent (`EMAIL_ERROR`). |
