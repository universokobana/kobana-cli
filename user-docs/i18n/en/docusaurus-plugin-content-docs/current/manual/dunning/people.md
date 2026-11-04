---
id: people
title: "kobana dunning people"
sidebar_label: "people"
---

# kobana dunning people


Commands of `kobana dunning people`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`people bulk delete`](#people-bulk-delete) | `DELETE /v1/people/bulk` | Bulk-delete customers |
| [`people contact-points list`](#people-contact-points-list) | `GET /v1/people/{id}/contact-points` | List contact points |
| [`people contact-points create`](#people-contact-points-create) | `POST /v1/people/{id}/contact-points` | Create contact point |
| [`people contact-points update`](#people-contact-points-update) | `PATCH /v1/people/{id}/contact-points/{cpId}` | Update contact point |
| [`people contact-points delete`](#people-contact-points-delete) | `DELETE /v1/people/{id}/contact-points/{cpId}` | Delete contact point |
| [`people list`](#people-list) | `GET /v1/people` | List customers |
| [`people create`](#people-create) | `POST /v1/people` | Create customer |
| [`people get`](#people-get) | `GET /v1/people/{id}` | Get customer |
| [`people update`](#people-update) | `PUT /v1/people/{id}` | Update customer |
| [`people delete`](#people-delete) | `DELETE /v1/people/{id}` | Delete customer |
| [`people statement-pdf`](#people-statement-pdf) | `GET /v1/people/{id}/statement-pdf` | Debtor statement (PDF) |

## people bulk delete

Bulk-delete customers

```text
kobana dunning people bulk delete [OPTIONS]
```

`DELETE /v1/people/bulk`

Deletes (soft delete, irreversible) up to 100 customers in one call, with the ids in the JSON body. A customer with open charges (`pending`/`overdue`/`negotiated`) cannot be deleted and is returned in `failed` with `has_active_charges`. A partial result is not an error. Requires the `delete` access level (or `all`) — `write` is not enough. It replaced `POST /people/bulk-archive`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | Target record IDs (1 to 100 per call). Duplicates are ignored. |

More fields and nested objects: `kobana schema dunning.people.bulk.delete`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Per-item result of the bulk deletion. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people contact-points list

List contact points

```text
kobana dunning people contact-points list [ID] [OPTIONS]
```

`GET /v1/people/{id}/contact-points`

Lists a customer’s contact points, with pagination and filters by channel and status.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `channel` | `string` | no | Filter by channel (`email` \| `sms` \| `whatsapp` \| `voice` \| `postal`). Values: `email`, `sms`, `whatsapp`, `voice`, `postal`. |
| `status` | `string` | no | Filter by status (`active` \| `deleted`). Values: `active`, `deleted`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated contact-point list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people contact-points create

Create contact point

```text
kobana dunning people contact-points create [ID] [OPTIONS]
```

`POST /v1/people/{id}/contact-points`

Creates a contact point for the customer. Returns 409 if an active contact point with the same channel and value already exists.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `channel` | `string` | yes | Channel (`email` \| `sms` \| `whatsapp` \| `voice` \| `postal`). Values: `email`, `sms`, `whatsapp`, `voice`, `postal`. |
| `value` | `string` | yes | NORMALIZED contact value: email lowercased; phone and ZIP digits only. |
| `label` | `string` | no | Free-form label (e.g. personal/commercial/mobile/whatsapp). |
| `is_primary` | `boolean` | no | Marks the channel’s primary contact (used first in recipient resolution). |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |

More fields and nested objects: `kobana schema dunning.people.contact-points.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Contact point created. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## people contact-points update

Update contact point

```text
kobana dunning people contact-points update [ID] [CPID] [OPTIONS]
```

`PATCH /v1/people/{id}/contact-points/{cpId}`

Updates a contact point’s label/primary/verification and lets you manually set/clear opt-out and third-party flags.

**Path arguments:** `id`, `cpId`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no | Free-form label (e.g. personal/commercial/mobile/whatsapp). |
| `is_primary` | `boolean` | no | Marks the channel’s primary contact (used first in recipient resolution). |
| `verified` | `boolean` | no | `true` marks the contact as verified now; `false` clears the verification. |
| `opt_out` | `boolean` | no | `true` records a per-contact opt-out now; `false` reverts it. |
| `third_party` | `boolean` | no | `true` flags the contact as third-party now; `false` reverts it. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.people.contact-points.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Contact point updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people contact-points delete

Delete contact point

```text
kobana dunning people contact-points delete [ID] [CPID] [OPTIONS]
```

`DELETE /v1/people/{id}/contact-points/{cpId}`

Soft-deletes a contact point. Suppressions and history remain for the audit trail.

**Path arguments:** `id`, `cpId`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Contact point deleted. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people list

List customers

```text
kobana dunning people list [OPTIONS]
```

`GET /v1/people`

Lists the organization’s customers with pagination and filters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `sort_by` | `string` | no | Sort field (public snake_case name, e.g. `due_date`). |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |
| `search` | `string` | no | Search by name, document or email. |
| `classification_id` | `string` | no | Filter by classification (UUID). |
| `status` | `string` | no | Filter by record status (`active` \| `deleted`). Values: `active`, `deleted`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated customer list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people create

Create customer

```text
kobana dunning people create [OPTIONS]
```

`POST /v1/people`

Creates a customer in the portfolio. `external_id` and `document_number` drive deduplication in imports and integrations.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Customer name. |
| `classification_id` | `string (uuid)` | no | Customer classification (rule segmentation). |
| `document_number` | `string` | no | CPF/CNPJ (digits only). |
| `document_type` | `string` | no | Document type (`cpf` \| `cnpj`). Values: `cpf`, `cnpj`, `null`. |
| `kind` | `string` | no | Nature (`natural` person or `juridical` entity). Values: `natural`, `juridical`, `null`. |
| `legal_name` | `string` | no | Legal name. |
| `nickname` | `string` | no | Nickname/trade name. |
| `birthday` | `string (date-time)` | no | Birth date (ISO 8601). |
| `addresses` | `array<object>` | no | Addresses. |
| `phones` | `array<object>` | no | Phone numbers. |
| `emails` | `array<object>` | no | Email addresses. |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `tags` | `array<string>` | no | Free-form tags. |
| `notes` | `string` | no | Internal notes. |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |

More fields and nested objects: `kobana schema dunning.people.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Customer created. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people get

Get customer

```text
kobana dunning people get [ID] [OPTIONS]
```

`GET /v1/people/{id}`

Returns a customer by ID.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Customer found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people update

Update customer

```text
kobana dunning people update [ID] [OPTIONS]
```

`PUT /v1/people/{id}`

Updates a customer’s data (partial).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `classification_id` | `string (uuid)` | no | Customer classification (rule segmentation). |
| `document_number` | `string` | no | CPF/CNPJ (digits only). |
| `document_type` | `string` | no | Document type (`cpf` \| `cnpj`). Values: `cpf`, `cnpj`, `null`. |
| `kind` | `string` | no | Nature (`natural` person or `juridical` entity). Values: `natural`, `juridical`, `null`. |
| `name` | `string` | no | Customer name. |
| `legal_name` | `string` | no | Legal name. |
| `nickname` | `string` | no | Nickname/trade name. |
| `birthday` | `string (date-time)` | no | Birth date (ISO 8601). |
| `addresses` | `array<object>` | no | Addresses. |
| `phones` | `array<object>` | no | Phone numbers. |
| `emails` | `array<object>` | no | Email addresses. |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `tags` | `array<string>` | no | Free-form tags. |
| `notes` | `string` | no | Internal notes. |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_data_mode` | `string` | no | How `custom_data` is applied on update: `replace` (default) overwrites the whole object; `merge` deep-merges the payload over the stored value — nested objects merge, scalars/arrays/null overwrite the key. Values: `replace`, `merge`. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.people.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Customer updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people delete

Delete customer

```text
kobana dunning people delete [ID] [OPTIONS]
```

`DELETE /v1/people/{id}`

Soft-deletes a customer. Charges and history remain for the audit trail.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Customer deleted. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## people statement-pdf

Debtor statement (PDF)

```text
kobana dunning people statement-pdf [ID] [OPTIONS]
```

`GET /v1/people/{id}/statement-pdf`

Generates the debtor statement as a PDF: all of the person’s charges (document, due date, status, original and current amount) with totals for open, overdue and already paid. Scoped to the organization. Responds with `application/pdf` as a binary attachment.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Debtor statement PDF (binary `application/pdf` attachment). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
