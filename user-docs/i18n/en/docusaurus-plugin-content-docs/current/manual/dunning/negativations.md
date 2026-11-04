---
id: negativations
title: "kobana dunning negativations"
sidebar_label: "negativations"
---

# kobana dunning negativations


Commands of `kobana dunning negativations`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`negativations list`](#negativations-list) | `GET /v1/negativations` | List bureau reports |
| [`negativations create`](#negativations-create) | `POST /v1/negativations` | Create bureau report |
| [`negativations get`](#negativations-get) | `GET /v1/negativations/{id}` | Get bureau report |
| [`negativations update`](#negativations-update) | `PUT /v1/negativations/{id}` | Update bureau report |
| [`negativations remove`](#negativations-remove) | `POST /v1/negativations/{id}/remove` | Request bureau report removal |

## negativations list

List bureau reports

```text
kobana dunning negativations list [OPTIONS]
```

`GET /v1/negativations`

Lists the organization’s bureau reports with pagination and filters. Use `include=person,charge` to embed customer and charge summaries.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `sort_by` | `string` | no | Sort field (`created_at` \| `updated_at` \| `amount` \| `registered_at`). Values: `createdAt`, `updatedAt`, `amount`, `registeredAt`. |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |
| `status` | `string` | no | Filter by status (`pending` \| `active` \| `removed` \| `failed`). Values: `pending`, `active`, `removed`, `failed`. |
| `bureau` | `string` | no | Filter by credit bureau (`serasa` \| `spc` \| `boa_vista`). Values: `serasa`, `spc`, `boa_vista`. |
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
| `200` | Paginated bureau report list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## negativations create

Create bureau report

```text
kobana dunning negativations create [OPTIONS]
```

`POST /v1/negativations`

Creates a bureau report for a charge. The record starts as `pending` and is ONLY submitted to the bureau after human review and approval (legal workflow; `negativation.review_required` event). Returns 409 if a `pending`/`active` report already exists for the same charge and bureau.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_id` | `string (uuid)` | yes | Reported customer (UUID). |
| `charge_id` | `string (uuid)` | yes | Reported charge (UUID). |
| `bureau` | `string` | yes | Credit bureau (`serasa` \| `spc` \| `boa_vista`). Values: `serasa`, `spc`, `boa_vista`. |
| `amount` | `number` | yes | Amount to report (positive decimal number). |
| `external_id` | `string` | no | Identifier in YOUR system or the bureau protocol number. |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.negativations.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Bureau report created (awaiting human review). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## negativations get

Get bureau report

```text
kobana dunning negativations get [ID] [OPTIONS]
```

`GET /v1/negativations/{id}`

Returns a bureau report by ID, with customer and charge summaries.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bureau report found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## negativations update

Update bureau report

```text
kobana dunning negativations update [ID] [OPTIONS]
```

`PUT /v1/negativations/{id}`

Updates a bureau report (partial). This is the approval step of the legal workflow: it requires the review (`approve`) permission and is where status moves from `pending` to `active` after human review.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Status (`pending` = awaiting mandatory human review, `active` = live at the bureau, `removed` = withdrawn, `failed` = failed). Values: `pending`, `active`, `removed`, `failed`. |
| `external_id` | `string` | no | Identifier in YOUR system or the bureau protocol number. |
| `registered_at` | `string (date-time)` | no | Date the report went live at the bureau (ISO 8601). |
| `removed_at` | `string (date-time)` | no | Date the report was removed from the bureau (ISO 8601). |
| `removal_reason` | `string` | no | Reason for removing the report. |
| `response_data` | `object` | no | Raw bureau response (integration payload). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.negativations.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bureau report updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## negativations remove

Request bureau report removal

```text
kobana dunning negativations remove [ID] [OPTIONS]
```

`POST /v1/negativations/{id}/remove`

Requests removal of a report from the bureau. Only allowed for `active` records; status becomes `removed` and the reason (if sent) is stored in `removal_reason`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `removal_reason` | `string` | no | Reason for removing the report. |

More fields and nested objects: `kobana schema dunning.negativations.remove`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Removal recorded; report updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
