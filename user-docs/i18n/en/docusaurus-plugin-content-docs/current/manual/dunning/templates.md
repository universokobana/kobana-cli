---
id: templates
title: "kobana dunning templates"
sidebar_label: "templates"
---

# kobana dunning templates


Commands of `kobana dunning templates`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`templates list`](#templates-list) | `GET /v1/templates` | List templates |
| [`templates create`](#templates-create) | `POST /v1/templates` | Create template |
| [`templates get`](#templates-get) | `GET /v1/templates/{id}` | Get template |
| [`templates update`](#templates-update) | `PUT /v1/templates/{id}` | Update template |
| [`templates delete`](#templates-delete) | `DELETE /v1/templates/{id}` | Delete template |
| [`templates duplicate`](#templates-duplicate) | `POST /v1/templates/{id}/duplicate` | Duplicate template |

## templates list

List templates

```text
kobana dunning templates list [OPTIONS]
```

`GET /v1/templates`

Lists the organization’s message templates with pagination, search and filters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `search` | `string` | no | Search by name, subject or body. |
| `channel` | `string` | no | Filter by channel. Values: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | no | Filter by category. Values: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `tone` | `string` | no | Filter by tone. Values: `friendly`, `neutral`, `firm`, `urgent`. |
| `is_active` | `string` | no | Filter by activation: `true` = active, `false` = inactive. Values: `true`, `false`. |
| `sort_by` | `string` | no | Sort field (public snake_case name, e.g. `due_date`). Values: `name`, `channel`, `category`, `createdAt`, `updatedAt`. |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated template list. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## templates create

Create template

```text
kobana dunning templates create [OPTIONS]
```

`POST /v1/templates`

Creates a message template. Fails with 409 if a template with the same `name` or `external_id` already exists.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Template name (unique within the organization). |
| `channel` | `string` | yes | Delivery channel (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Values: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | yes | Category (`reminder` \| `overdue` \| `negotiation` \| `negativation_warning` \| `protest_warning` \| `payment_confirmation`). Values: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `body` | `string` | yes | Message body, with variables in `{{variable}}` format. |
| `tone` | `string` | no | Message tone (`friendly` \| `neutral` \| `firm` \| `urgent`). Values: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `subject` | `string` | no | Subject (used for email). |
| `variables` | `array<string>` | no | Variables available in the body/subject. |
| `activated_at` | `string (date-time)` | no | Activation timestamp (null = inactive). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key; unique within the organization). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.templates.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Template created. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## templates get

Get template

```text
kobana dunning templates get [ID] [OPTIONS]
```

`GET /v1/templates/{id}`

Returns a template by ID.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Template found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## templates update

Update template

```text
kobana dunning templates update [ID] [OPTIONS]
```

`PUT /v1/templates/{id}`

Updates a template (partial). Fails with 409 if the new `name` or `external_id` is already taken.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Template name (unique within the organization). |
| `channel` | `string` | no | Delivery channel (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Values: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | no | Category (`reminder` \| `overdue` \| `negotiation` \| `negativation_warning` \| `protest_warning` \| `payment_confirmation`). Values: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `tone` | `string` | no | Message tone (`friendly` \| `neutral` \| `firm` \| `urgent`). Values: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `subject` | `string` | no | Subject (used for email). |
| `body` | `string` | no | Message body, with variables in `{{variable}}` format. |
| `variables` | `array<string>` | no | Variables available in the body/subject. |
| `activated_at` | `string (date-time)` | no | Activation timestamp (null = inactive). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key; unique within the organization). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.templates.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Template updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## templates delete

Delete template

```text
kobana dunning templates delete [ID] [OPTIONS]
```

`DELETE /v1/templates/{id}`

Soft-deletes a template. Fails with 409 if it is in use by rule steps.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Template deleted. |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## templates duplicate

Duplicate template

```text
kobana dunning templates duplicate [ID] [OPTIONS]
```

`POST /v1/templates/{id}/duplicate`

Creates a copy of the template with the given name. The copy starts inactive (`activated_at` null) and without `external_id`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Template name (unique within the organization). |

More fields and nested objects: `kobana schema dunning.templates.duplicate`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Template duplicated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |
