---
id: notifications
title: "kobana dunning notifications"
sidebar_label: "notifications"
---

# kobana dunning notifications


Commands of `kobana dunning notifications`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`notifications list`](#notifications-list) | `GET /v1/notifications` | List notifications |
| [`notifications create`](#notifications-create) | `POST /v1/notifications` | Create notification |
| [`notifications get`](#notifications-get) | `GET /v1/notifications/{id}` | Get notification |
| [`notifications update`](#notifications-update) | `PUT /v1/notifications/{id}` | Update notification |
| [`notifications delete`](#notifications-delete) | `DELETE /v1/notifications/{id}` | Delete notification |
| [`notifications resend`](#notifications-resend) | `POST /v1/notifications/{id}/resend` | Resend notification |

## notifications list

List notifications

```text
kobana dunning notifications list [OPTIONS]
```

`GET /v1/notifications`

Lists the organization’s notifications with pagination and filters. Sortable via `sort_by` (`created_at` | `updated_at` | `sent_at` | `delivered_at` | `channel` | `status`).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `sort_by` | `string` | no | Sort field (public snake_case name, e.g. `due_date`). |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |
| `channel` | `string` | no | Filter by channel; accepts a single value or a comma-separated list (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `status` | `string` | no | Filter by status; accepts a single value or a comma-separated list (`pending` \| `queued` \| `sent` \| `delivered` \| `read` \| `failed` \| `bounced`). |
| `person_id` | `string` | no | Filter by customer (UUID). |
| `charge_id` | `string` | no | Filter by charge (UUID). |
| `include` | `string` | no | Relations to include, comma-separated (`person`, `charge`, `step`; `template` is an alias for `step`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated notification list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## notifications create

Create notification

```text
kobana dunning notifications create [OPTIONS]
```

`POST /v1/notifications`

Creates a manual notification for a customer, optionally linked to a charge and a rule step. The recipient is validated according to the channel.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_id` | `string (uuid)` | yes | Recipient customer. |
| `channel` | `string` | yes | Channel (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Values: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `recipient` | `string` | yes | Recipient; validated according to the channel (email for `email`, phone for `sms`/`whatsapp`/`voice`). |
| `body` | `string` | yes | Message body. |
| `charge_id` | `string (uuid)` | no | Charge related to the notification. |
| `collection_rule_step_id` | `string (uuid)` | no | Collection rule step that originated the notification. |
| `subject` | `string` | no | Subject (for channels that support it, e.g., email). |
| `external_id` | `string` | no | Identifier in YOUR system or in the delivery provider. |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.notifications.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Notification created with `pending` status. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## notifications get

Get notification

```text
kobana dunning notifications get [ID] [OPTIONS]
```

`GET /v1/notifications/{id}`

Returns a notification by ID; use `include` to attach relations.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `include` | `string` | no | Relations to include, comma-separated (`person`, `charge`, `step`; `template` is an alias for `step`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Notification found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## notifications update

Update notification

```text
kobana dunning notifications update [ID] [OPTIONS]
```

`PUT /v1/notifications/{id}`

Updates a notification’s status, delivery timestamps and metadata (partial). Useful for integrations confirming send/delivery.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | New status; fills the matching timestamp (sentAt/deliveredAt/readAt/failedAt) when not provided. Values: `pending`, `queued`, `sent`, `delivered`, `read`, `failed`, `bounced`. |
| `sent_at` | `string` | no | ISO 8601 timestamp or `YYYY-MM-DD` date; null clears the field. |
| `delivered_at` | `string` | no | ISO 8601 timestamp or `YYYY-MM-DD` date; null clears the field. |
| `read_at` | `string` | no | ISO 8601 timestamp or `YYYY-MM-DD` date; null clears the field. |
| `failed_at` | `string` | no | ISO 8601 timestamp or `YYYY-MM-DD` date; null clears the field. |
| `failure_reason` | `string` | no | Failure reason reported by the provider. |
| `external_id` | `string` | no | Identifier in YOUR system or in the delivery provider. |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `tags` | `array<string>` | no | Free-form tags. |
| `cost` | `number` | no | Send cost (number, minimum 0). |

More fields and nested objects: `kobana schema dunning.notifications.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Notification updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## notifications delete

Delete notification

```text
kobana dunning notifications delete [ID] [OPTIONS]
```

`DELETE /v1/notifications/{id}`

PERMANENTLY deletes a notification (hard delete — the model has no soft delete). Only a notification not yet delivered: a delivered one is probative evidence and responds `409`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Notification deleted. |
| `404` | Resource not found or outside your organization. |
| `409` | The notification was already delivered — it is probative evidence and cannot be deleted. |
| `413` | Request body larger than 2 MB. |

## notifications resend

Resend notification

```text
kobana dunning notifications resend [ID] [OPTIONS]
```

`POST /v1/notifications/{id}/resend`

Creates a NEW `pending` notification copied from the original (the new record’s `metadata` references the original via `resend_of`). Only allowed when the original is `failed` or `bounced`; otherwise returns 400. Supports `Idempotency-Key`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `201` | New notification created from the original. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
