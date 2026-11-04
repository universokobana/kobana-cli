---
id: portal-users
title: "kobana dashboards portal-users"
sidebar_label: "portal-users"
---

# kobana dashboards portal-users


Commands of `kobana dashboards portal-users`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`portal-users activity`](#portal-users-activity) | `GET /v1/portal_users/{id}/activity` | Get portal user activity |
| [`portal-users activity timeline`](#portal-users-activity-timeline) | `GET /v1/portal_users/{id}/activity/timeline` | List portal user activity timeline |
| [`portal-users list`](#portal-users-list) | `GET /v1/portal_users` | List portal users |
| [`portal-users create`](#portal-users-create) | `POST /v1/portal_users` | Create (invite) portal user |
| [`portal-users invite`](#portal-users-invite) | `POST /v1/portal_users/invite` | Invite portal user |
| [`portal-users get`](#portal-users-get) | `GET /v1/portal_users/{id}` | Get portal user |
| [`portal-users update`](#portal-users-update) | `PATCH /v1/portal_users/{id}` | Update portal user |
| [`portal-users delete`](#portal-users-delete) | `DELETE /v1/portal_users/{id}` | Delete portal user |
| [`portal-users email-preview`](#portal-users-email-preview) | `GET /v1/portal_users/{id}/email-preview` | Preview portal user email |
| [`portal-users resend`](#portal-users-resend) | `POST /v1/portal_users/{id}/resend` | Resend invitation or verification email |

## portal-users activity

Get portal user activity

```text
kobana dashboards portal-users activity [ID] [OPTIONS] [COMMAND]
```

`GET /v1/portal_users/{id}/activity`

Usage analytics of a portal user: sign-ins and pages viewed (all time), distinct pages, the newest activity, a daily series of views and sign-ins over the last `days` days (São Paulo days) and the most opened pages. The individual entries are at `GET /portal_users/{id}/activity/timeline`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `days` | `integer` | no | Days the `series` covers, today included (1 to 365). Default: 30. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Summary of the user activity. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## portal-users activity timeline

List portal user activity timeline

```text
kobana dashboards portal-users activity timeline [ID] [OPTIONS]
```

`GET /v1/portal_users/{id}/activity/timeline`

Sign-ins (`kind: login`, with the `method` used: `credentials`, `magic-link` or `sso`) and pages opened (`kind: page_view`, with `path` and `page_name`), newest first. Cursor paging: send `meta.next_before` as `before` for the next page; `null` means there is none. An operator viewing the portal as the user is not counted.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `before` | `string` | no | ISO 8601 instant: only entries older than it. |
| `limit` | `integer` | no | Entries per page (1 to 100). Default: 20. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | One page of the timeline. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## portal-users list

List portal users

```text
kobana dashboards portal-users list [OPTIONS]
```

`GET /v1/portal_users`

Returns the workspace's portal users, paginated. Sensitive fields (password, tokens) are never exposed.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `tenant_id` | `string` | no | Scope to the users of one owner ID (exact match). |
| `status` | `string` | no | Filter by status: `active`, `pending_invitation`, `pending_verification`, `suspended` or `locked`. Values: `active`, `pending_verification`, `pending_invitation`, `suspended`, `locked`. |
| `email` | `string` | no | Substring match on the email. |
| `external_id` | `string` | no | Filter by the exact `external_id`. |
| `sso_only` | `string` | no | Filter by `sso_only` (`true` or `false`). Values: `true`, `false`. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `last_login_after` | `string` | no | Last sign-in on or after this date (ISO 8601). |
| `last_login_before` | `string` | no | Last sign-in on or before this date (ISO 8601). |
| `person_id` | `string` | no | Filter by associated person UUID. |
| `person_document_number` | `string` | no | Filter by person CPF/CNPJ (exact match). |
| `person_name_contains` | `string` | no | Filter by person name (partial search, case-insensitive). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of portal users. |
| `404` | Resource not found in the current organization. |

## portal-users create

Create (invite) portal user

```text
kobana dashboards portal-users create [OPTIONS]
```

`POST /v1/portal_users`

Creates the user in the workspace with status `pending_invitation` and sends the invitation email. The invitation expires in 7 days.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string` | yes | User's email (required). Unique per workspace, case-insensitive. |
| `tenant_id` | `string` | no | Owner ID of the records the user will see in the portal. Send `null` to clear it. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `sso_only` | `boolean` | no | When `true`, the user can only sign in through SSO. |
| `custom_metadata` | `object` | no | Arbitrary organization-defined metadata (key/value). |

More fields and nested objects: `kobana schema dashboards.portal-users.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Invitation created and email queued for sending. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | A user or invitation already exists for this email in the workspace. |

## portal-users invite

Invite portal user

```text
kobana dashboards portal-users invite [OPTIONS]
```

`POST /v1/portal_users/invite`

Equivalent to `POST /portal_users`. Available as a verb route for SDKs that prefer descriptive URLs.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string` | yes | User's email (required). Unique per workspace, case-insensitive. |
| `tenant_id` | `string` | no | Owner ID of the records the user will see in the portal. Send `null` to clear it. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `sso_only` | `boolean` | no | When `true`, the user can only sign in through SSO. |
| `custom_metadata` | `object` | no | Arbitrary organization-defined metadata (key/value). |

More fields and nested objects: `kobana schema dashboards.portal-users.invite`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Invitation created and email queued for sending. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | A user or invitation already exists for this email in the workspace. |

## portal-users get

Get portal user

```text
kobana dashboards portal-users get [ID] [OPTIONS]
```

`GET /v1/portal_users/{id}`

The `{id}` parameter accepts the user's UUID or `external_id`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Portal user detail. |
| `404` | Resource not found in the current organization. |

## portal-users update

Update portal user

```text
kobana dashboards portal-users update [ID] [OPTIONS]
```

`PATCH /v1/portal_users/{id}`

Updates `email`, `tenant_id`, `external_id`, `sso_only`, `status` and `custom_metadata`. Allowed status transitions: `active` ↔ `suspended`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string` | no | User's email (required). Unique per workspace, case-insensitive. |
| `tenant_id` | `string` | no | Owner ID of the records the user will see in the portal. Send `null` to clear it. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `sso_only` | `boolean` | no | When `true`, the user can only sign in through SSO. |
| `status` | `string` | no | Allowed transitions: `active` ↔ `suspended`. On reactivation the user goes back to the status held before the suspension (someone who had not confirmed the e-mail stays pending). Values: `active`, `suspended`. |
| `unlock` | `boolean` | no | `true` ends the login lockout (too many wrong passwords): resets the attempts and lets the user sign in. 400 when the user is not locked. Values: `true`. |
| `custom_metadata` | `object` | no | Arbitrary organization-defined metadata (key/value). |

More fields and nested objects: `kobana schema dashboards.portal-users.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | User updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | A user or invitation already exists for this email in the workspace. |

## portal-users delete

Delete portal user

```text
kobana dashboards portal-users delete [ID] [OPTIONS]
```

`DELETE /v1/portal_users/{id}`

Pending invitations are permanently deleted; other users are suspended and their sessions ended. An already suspended user is left unchanged. `outcome` tells what happened.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Deletion confirmation. |
| `404` | Resource not found in the current organization. |

## portal-users email-preview

Preview portal user email

```text
kobana dashboards portal-users email-preview [ID] [OPTIONS]
```

`GET /v1/portal_users/{id}/email-preview`

Returns the subject and body of the invitation or verification email that would be sent to the user, without actually sending it. Available only for `pending_invitation` or `pending_verification` users.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Email preview. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## portal-users resend

Resend invitation or verification email

```text
kobana dashboards portal-users resend [ID] [OPTIONS]
```

`POST /v1/portal_users/{id}/resend`

Resends the invitation email (for `pending_invitation`, rotating the token) or verification email (for `pending_verification`). Other statuses return 400.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Email resent. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
