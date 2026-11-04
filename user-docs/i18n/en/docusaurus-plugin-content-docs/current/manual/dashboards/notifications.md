---
id: notifications
title: "kobana dashboards notifications"
sidebar_label: "notifications"
---

# kobana dashboards notifications


Commands of `kobana dashboards notifications`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`notifications stats`](#notifications-stats) | `GET /v1/notifications/stats` | Notification statistics |
| [`notifications list`](#notifications-list) | `GET /v1/notifications` | List notifications |
| [`notifications create`](#notifications-create) | `POST /v1/notifications` | Create notification |
| [`notifications send`](#notifications-send) | `POST /v1/notifications/{id}/send` | Send notification |

## notifications stats

Notification statistics

```text
kobana dashboards notifications stats [OPTIONS]
```

`GET /v1/notifications/stats`

Counts by status, type and channel of the notifications created in the last `days` days, with the delivery and read rates.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `days` | `integer` | no | Window in days, 1 to 365. Default 30. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The statistics. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## notifications list

List notifications

```text
kobana dashboards notifications list [OPTIONS]
```

`GET /v1/notifications`

Lists the notifications of the workspace and the organization-wide ones, newest first.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `type` | `string` | no | Only this notification type. Values: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `channel` | `string` | no | Only this channel. Values: `email`, `in_app`, `webhook`. |
| `status` | `string` | no | Only this status. Values: `pending`, `sent`, `delivered`, `failed`, `read`, `skipped`. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `search` | `string` | no | Substring of the recipient or the subject, case-insensitive. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of notifications. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## notifications create

Create notification

```text
kobana dashboards notifications create [OPTIONS]
```

`POST /v1/notifications`

Renders the active template of the type with the variables, stores the notification and sends it right away — the response carries the outcome (`sent`, or `failed` with `failure_reason`). With a future `scheduled_for` it stays `pending` until it is due. Every creation counts toward the per-recipient manual-send limit (429).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `type` | `string` | yes | Notification type (required): picks the organization's active template, or the platform default. Values: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `recipient` | `string` | yes | Destination address (required) — an email for the `email` channel. |
| `channel` | `string` | no | Delivery channel. Defaults to `email`. Values: `email`, `in_app`, `webhook`. |
| `portal_user_id` | `string (uuid)` | no | Portal user the notification is for; must belong to the organization (else 404). |
| `resource_type` | `string` | no | Kind of the related record. |
| `resource_id` | `string (uuid)` | no | ID (UUID) of the related record. |
| `scheduled_for` | `string (date-time)` | no | ISO 8601 date-time to send at, instead of right away. |
| `variables` | `object` | no | Values for the template variables: up to 100, names of letters, digits, `_`, `.` or `-`; values text (up to 10,000 characters), number, boolean or `null`. |

More fields and nested objects: `kobana schema dashboards.notifications.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Notification created. |
| `400` | Invalid request (missing required field, invalid type or variables). |
| `404` | Resource not found in the current organization. |

## notifications send

Send notification

```text
kobana dashboards notifications send [ID] [OPTIONS]
```

`POST /v1/notifications/{id}/send`

Sends a stored notification now: the first send of a `pending` (scheduled) or `skipped` one, or a resend of a `sent`, `delivered`, `read` or `failed` one. Every call sends the email again and counts toward the per-recipient manual-send limit (429).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The updated notification. |
| `400` | Invalid request (missing required field, invalid type or variables). |
| `404` | Resource not found in the current organization. |
| `502` | The email provider refused the send; the reason is in the message. |
