---
id: notification-channels
title: "kobana dunning notification-channels"
sidebar_label: "notification-channels"
---

# kobana dunning notification-channels


Commands of `kobana dunning notification-channels`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`notification-channels list`](#notification-channels-list) | `GET /v1/notification-channels` | List notification channels |
| [`notification-channels create`](#notification-channels-create) | `POST /v1/notification-channels` | Create notification channel |
| [`notification-channels get`](#notification-channels-get) | `GET /v1/notification-channels/{id}` | Get notification channel |
| [`notification-channels update`](#notification-channels-update) | `PATCH /v1/notification-channels/{id}` | Update notification channel |
| [`notification-channels delete`](#notification-channels-delete) | `DELETE /v1/notification-channels/{id}` | Delete notification channel |

## notification-channels list

List notification channels

```text
kobana dunning notification-channels list [OPTIONS]
```

`GET /v1/notification-channels`

Returns the organization channels. Credentials are never included.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `channel_type` | `string` | no | Filter by channel type. Values: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | no | Filter by provider. |
| `activated_at` | `string` | no | Filter by activation: `true` (active) \| `false`/`null` (inactive). Values: `true`, `false`, `null`. |
| `sort` | `string` | no | Sort field: `channel_type` \| `provider` \| `created_at` \| `updated_at`. Values: `channelType`, `provider`, `createdAt`, `updatedAt`. |
| `order` | `string` | no | Sort direction: `asc` \| `desc`. Values: `asc`, `desc`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of channels. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## notification-channels create

Create notification channel

```text
kobana dunning notification-channels create [OPTIONS]
```

`POST /v1/notification-channels`

Creates a channel. Credentials are encrypted at rest before saving. One channel per type + provider combination.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `channel_type` | `string` | yes | Channel type: `email` \| `sms` \| `whatsapp` \| `voice` \| `manual`. Values: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | yes | Channel provider (e.g. `sendgrid`, `twilio`, `meta`). |
| `credentials` | `object` | yes | Provider credentials (e.g. `api_key`/`api_secret`). Encrypted at rest on write and NEVER returned on read. Write-only. |
| `settings` | `object` | no | Non-sensitive channel settings (free-form JSON). |
| `cost_per_message` | `number` | no | Cost per sent message (decimal string). |
| `activated_at` | `string (date-time)` | no | Activation date; `null` = inactive channel. |
| `external_id` | `string` | no | Channel identifier in the integrator external system. |
| `custom_data` | `object` | no | Custom data (JSON) defined by the integrator. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags for organization. |

More fields and nested objects: `kobana schema dunning.notification-channels.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Channel created (without credentials). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## notification-channels get

Get notification channel

```text
kobana dunning notification-channels get [ID] [OPTIONS]
```

`GET /v1/notification-channels/{id}`

Returns a channel by ID. Credentials are never included.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Channel found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## notification-channels update

Update notification channel

```text
kobana dunning notification-channels update [ID] [OPTIONS]
```

`PATCH /v1/notification-channels/{id}`

Updates a channel (partial). If `credentials` is sent, it is re-encrypted; omitting keeps the current secret.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `channel_type` | `string` | no | Channel type: `email` \| `sms` \| `whatsapp` \| `voice` \| `manual`. Values: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | no | Channel provider (e.g. `sendgrid`, `twilio`, `meta`). |
| `credentials` | `object` | no | Provider credentials (e.g. `api_key`/`api_secret`). Encrypted at rest on write and NEVER returned on read. Write-only. |
| `settings` | `object` | no | Non-sensitive channel settings (free-form JSON). |
| `cost_per_message` | `number` | no | Cost per sent message (decimal string). |
| `activated_at` | `string (date-time)` | no | Activation date; `null` = inactive channel. |
| `external_id` | `string` | no | Channel identifier in the integrator external system. |
| `custom_data` | `object` | no | Custom data (JSON) defined by the integrator. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags for organization. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.notification-channels.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Channel updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## notification-channels delete

Delete notification channel

```text
kobana dunning notification-channels delete [ID] [OPTIONS]
```

`DELETE /v1/notification-channels/{id}`

Removes the channel (soft delete).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Channel removed. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
