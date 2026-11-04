---
id: protests
title: "kobana dunning protests"
sidebar_label: "protests"
---

# kobana dunning protests


Commands of `kobana dunning protests`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`protests list`](#protests-list) | `GET /v1/protests` | List protests |
| [`protests create`](#protests-create) | `POST /v1/protests` | Create protest |
| [`protests get`](#protests-get) | `GET /v1/protests/{id}` | Get protest |
| [`protests update`](#protests-update) | `PUT /v1/protests/{id}` | Update protest |
| [`protests cancel`](#protests-cancel) | `POST /v1/protests/{id}/cancel` | Request protest cancellation |

## protests list

List protests

```text
kobana dunning protests list [OPTIONS]
```

`GET /v1/protests`

Lists the organization’s protests with pagination and filters. Use `include=person,charge` to embed customer and charge summaries.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `sort_by` | `string` | no | Sort field (`created_at` \| `updated_at` \| `amount` \| `protest_date` \| `sent_at`). Values: `createdAt`, `updatedAt`, `amount`, `protestDate`, `sentAt`. |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |
| `status` | `string` | no | Filter by status (`pending` \| `sent` \| `intimated` \| `protested` \| `paid` \| `cancelled`). Values: `pending`, `sent`, `intimated`, `protested`, `paid`, `cancelled`. |
| `person_id` | `string` | no | Filter by customer (UUID). |
| `charge_id` | `string` | no | Filter by charge (UUID). |
| `include` | `string` | no | Comma-separated relations to embed (`person`, `charge`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated protest list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## protests create

Create protest

```text
kobana dunning protests create [OPTIONS]
```

`POST /v1/protests`

Creates a protest for a charge. The record starts as `pending` and is ONLY sent to the notary after human review and approval (legal workflow; `protest.review_required` event). Returns 409 if an active protest (`pending`/`sent`/`intimated`/`protested`) already exists for the same charge.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_id` | `string (uuid)` | yes | Protested customer (UUID). |
| `charge_id` | `string (uuid)` | yes | Protested charge (UUID). |
| `amount` | `number` | yes | Amount to protest (positive decimal number). |
| `notary` | `string` | no | Notary office name. |
| `notary_code` | `string` | no | Notary office code/identifier. |
| `fees` | `number` | no | Notary fees/costs (decimal number). |
| `external_id` | `string` | no | Identifier in YOUR system or the notary protocol number. |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.protests.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Protest created (awaiting human review). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## protests get

Get protest

```text
kobana dunning protests get [ID] [OPTIONS]
```

`GET /v1/protests/{id}`

Returns a protest by ID, with customer and charge summaries.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Protest found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## protests update

Update protest

```text
kobana dunning protests update [ID] [OPTIONS]
```

`PUT /v1/protests/{id}`

Updates a protest (partial). This is the approval step of the legal workflow: it requires the review (`approve`) permission and is where status advances from `pending` to `sent`/`intimated`/`protested` after human review.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Status (`pending` = awaiting mandatory human review, `sent` = sent to the notary, `intimated` = debtor notified, `protested` = protested, `paid` = paid, `cancelled` = cancelled). Values: `pending`, `sent`, `intimated`, `protested`, `paid`, `cancelled`. |
| `notary` | `string` | no | Notary office name. |
| `notary_code` | `string` | no | Notary office code/identifier. |
| `fees` | `number` | no | Notary fees/costs (decimal number). |
| `external_id` | `string` | no | Identifier in YOUR system or the notary protocol number. |
| `sent_at` | `string (date-time)` | no | Date sent to the notary (ISO 8601). |
| `intimation_date` | `string (date-time)` | no | Debtor notification date (ISO 8601). |
| `protest_date` | `string (date-time)` | no | Protest registration date (ISO 8601). |
| `payment_date` | `string (date-time)` | no | Payment date at the notary (ISO 8601). |
| `cancellation_date` | `string (date-time)` | no | Protest cancellation date (ISO 8601). |
| `response_data` | `object` | no | Raw notary response (integration payload). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.protests.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Protest updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## protests cancel

Request protest cancellation

```text
kobana dunning protests cancel [ID] [OPTIONS]
```

`POST /v1/protests/{id}/cancel`

Requests cancellation of a protest. Not allowed for protests already `cancelled` or `paid`; status becomes `cancelled` and the reason (if sent) is stored in `responseData.cancellationReason`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `cancellation_reason` | `string` | no | Cancellation reason. |

More fields and nested objects: `kobana schema dunning.protests.cancel`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Cancellation recorded; protest updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
