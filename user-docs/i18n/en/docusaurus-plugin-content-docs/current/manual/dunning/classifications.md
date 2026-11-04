---
id: classifications
title: "kobana dunning classifications"
sidebar_label: "classifications"
---

# kobana dunning classifications


Commands of `kobana dunning classifications`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`classifications list`](#classifications-list) | `GET /v1/classifications` | List classifications |
| [`classifications create`](#classifications-create) | `POST /v1/classifications` | Create classification |
| [`classifications get`](#classifications-get) | `GET /v1/classifications/{id}` | Get classification |
| [`classifications update`](#classifications-update) | `PUT /v1/classifications/{id}` | Update classification |
| [`classifications delete`](#classifications-delete) | `DELETE /v1/classifications/{id}` | Delete classification |

## classifications list

List classifications

```text
kobana dunning classifications list [OPTIONS]
```

`GET /v1/classifications`

Lists the organization’s classifications, ordered by `priority` and `name` by default. Use `include=_count` for counters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `is_default` | `string` | no | Filter by default classification (`true` \| `false`). Values: `true`, `false`. |
| `auto_assign` | `string` | no | Filter by automatic assignment (`true` \| `false`). Values: `true`, `false`. |
| `activated_at` | `string` | no | Filter by activation: `true` = active, `false` or `null` = inactive. Values: `true`, `false`, `null`. |
| `include` | `string` | no | Relations to embed. Supported: `_count` (customers and rules). |
| `sort` | `string` | no | Sort field (`name` \| `code` \| `priority` \| `created_at` \| `updated_at`). Values: `name`, `code`, `priority`, `createdAt`, `updatedAt`. |
| `order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated classification list. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## classifications create

Create classification

```text
kobana dunning classifications create [OPTIONS]
```

`POST /v1/classifications`

Creates a classification. Setting `is_default` unsets the previous default; a duplicate `code` fails with 409.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Classification name. |
| `code` | `string` | yes | Stable code (lowercase, digits and underscore; unique within the organization). |
| `description` | `string` | no | Classification description. |
| `color` | `string` | no | Hex color shown in the UI (e.g. `#22c55e`). |
| `priority` | `integer` | no | Evaluation priority (lower evaluates first). |
| `is_default` | `boolean` | no | true when this is the organization’s default classification (only one). |
| `auto_assign` | `boolean` | no | true when customers are auto-assigned by the criteria. |
| `criteria` | `object` | no | Automatic assignment criteria (payment behavior). |
| `activated_at` | `string (date-time)` | no | Activation timestamp (null = inactive). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.classifications.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Classification created (with counters). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## classifications get

Get classification

```text
kobana dunning classifications get [ID] [OPTIONS]
```

`GET /v1/classifications/{id}`

Returns a classification by ID. Use `include=_count` for counters.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `include` | `string` | no | Relations to embed. Supported: `_count` (customers and rules). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Classification found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## classifications update

Update classification

```text
kobana dunning classifications update [ID] [OPTIONS]
```

`PUT /v1/classifications/{id}`

Updates a classification (partial). A duplicate `code` fails with 409.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Classification name. |
| `code` | `string` | no | Stable code (lowercase, digits and underscore; unique within the organization). |
| `description` | `string` | no | Classification description. |
| `color` | `string` | no | Hex color shown in the UI (e.g. `#22c55e`). |
| `priority` | `integer` | no | Evaluation priority (lower evaluates first). |
| `is_default` | `boolean` | no | true when this is the organization’s default classification (only one). |
| `auto_assign` | `boolean` | no | true when customers are auto-assigned by the criteria. |
| `criteria` | `object` | no | Automatic assignment criteria (payment behavior). |
| `activated_at` | `string (date-time)` | no | Activation timestamp (null = inactive). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.classifications.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Classification updated (with counters). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## classifications delete

Delete classification

```text
kobana dunning classifications delete [ID] [OPTIONS]
```

`DELETE /v1/classifications/{id}`

Soft-deletes a classification. Fails with 409 if there are associated customers or rules. The `code` stays reserved.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Classification deleted. |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |
