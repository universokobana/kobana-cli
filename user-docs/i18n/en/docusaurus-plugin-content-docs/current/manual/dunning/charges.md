---
id: charges
title: "kobana dunning charges"
sidebar_label: "charges"
---

# kobana dunning charges


Commands of `kobana dunning charges`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`charges bulk delete`](#charges-bulk-delete) | `DELETE /v1/charges/bulk` | Bulk-delete charges |
| [`charges list`](#charges-list) | `GET /v1/charges` | List charges |
| [`charges create`](#charges-create) | `POST /v1/charges` | Create charge |
| [`charges bulk-assign-rule`](#charges-bulk-assign-rule) | `POST /v1/charges/bulk-assign-rule` | Bulk-assign a collection rule |
| [`charges bulk-cancel`](#charges-bulk-cancel) | `POST /v1/charges/bulk-cancel` | Bulk-cancel charges |
| [`charges get`](#charges-get) | `GET /v1/charges/{id}` | Get charge |
| [`charges update`](#charges-update) | `PUT /v1/charges/{id}` | Update charge |
| [`charges delete`](#charges-delete) | `DELETE /v1/charges/{id}` | Delete charge |
| [`charges notify`](#charges-notify) | `POST /v1/charges/{id}/notify` | Notify charge |
| [`charges settlement-pdf`](#charges-settlement-pdf) | `GET /v1/charges/{id}/settlement-pdf` | Settlement letter (PDF) |

## charges bulk delete

Bulk-delete charges

```text
kobana dunning charges bulk delete [OPTIONS]
```

`DELETE /v1/charges/bulk`

Deletes (soft delete, irreversible) up to 100 charges in one call, with the ids in the JSON body, cascading to related tasks and offers. A charge with associated records (notifications, interactions, negativations, protests) cannot be deleted and is returned in `failed` with `has_related_records`. Requires the `delete` access level (or `all`) — `write` is not enough. It replaced `POST /charges/bulk-archive`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | Target record IDs (1 to 100 per call). Duplicates are ignored. |

More fields and nested objects: `kobana schema dunning.charges.bulk.delete`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Per-item result of the bulk deletion. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## charges list

List charges

```text
kobana dunning charges list [OPTIONS]
```

`GET /v1/charges`

Lists the organization’s charges with pagination and filters. Past-due pending charges are promoted to `overdue` on read.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `sort_by` | `string` | no | Sort field (public snake_case name, e.g. `due_date`). |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |
| `search` | `string` | no | Search by document number, description, `external_id`, customer name or document. |
| `person_id` | `string` | no | Filter by a customer’s charges (UUID). |
| `status` | `string` | no | Filter by status. Accepts a single value or a comma-separated list (`pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`). |
| `due_date_from` | `string` | no | Due date from (inclusive, `YYYY-MM-DD` or ISO datetime). |
| `due_date_to` | `string` | no | Due date to (inclusive, `YYYY-MM-DD` or ISO datetime). |
| `include` | `string` | no | Relations to include in the response (`person`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated charge list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## charges create

Create charge

```text
kobana dunning charges create [OPTIONS]
```

`POST /v1/charges`

Creates a charge for a customer. Without `collection_rule_id`, the organization’s default collection rule is applied. `current_amount` = original + interest + fine − discount; an already past-due charge starts as `overdue`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_id` | `string (uuid)` | yes | Customer (debtor) of the charge. |
| `original_amount` | `number` | yes | Original charge amount. |
| `issue_date` | `string` | yes | Issue date. |
| `due_date` | `string` | yes | Due date. |
| `collection_rule_id` | `string (uuid)` | no | Collection rule tracking the charge. If omitted on create, the organization’s default rule is used. |
| `document_number` | `string` | no | Source document number (invoice, contract, etc.). |
| `description` | `string` | no | Free-form charge description. |
| `interest_amount` | `number` | no | Accrued interest (default 0). |
| `fine_amount` | `number` | no | Fine amount (default 0). |
| `discount_amount` | `number` | no | Applied discount (default 0). |
| `status` | `string` | no | Status (`pending` \| `paid` \| `overdue` \| `cancelled` \| `negotiated` \| `protested` \| `negatived` \| `written_off`). Values: `pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`. |
| `payment_method` | `string` | no | Payment method (`boleto` \| `pix` \| `credit_card` \| `transfer`). Values: `boleto`, `pix`, `credit_card`, `transfer`, `null`. |
| `barcode` | `string` | no | Boleto barcode/digitable line. |
| `pix_emv` | `string` | no | Pix copy-and-paste code (EMV). |
| `payment_url` | `string (uri)` | no | Payment page URL. |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |
| `charge_policy` | `object` | no |  |

More fields and nested objects: `kobana schema dunning.charges.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Charge created. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## charges bulk-assign-rule

Bulk-assign a collection rule

```text
kobana dunning charges bulk-assign-rule [OPTIONS]
```

`POST /v1/charges/bulk-assign-rule`

Applies a collection rule to up to 100 charges. Rule entry respects the engine idempotency: a charge with an active/paused enrollment keeps the current one. Only `pending`/`overdue` charges are eligible; the rest are returned in `failed` with `invalid_status`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | Target record IDs (1 to 100 per call). Duplicates are ignored. |
| `collection_rule_id` | `string (uuid)` | yes | ID of the collection rule to apply (must belong to the organization). |

More fields and nested objects: `kobana schema dunning.charges.bulk-assign-rule`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Per-item result of the bulk rule assignment. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## charges bulk-cancel

Bulk-cancel charges

```text
kobana dunning charges bulk-cancel [OPTIONS]
```

`POST /v1/charges/bulk-cancel`

Cancels up to 100 charges in one call. Each cancellation goes through the collection engine (validates the status transition, exits the enrollment and suppresses pending notifications). A partial result is not an error: non-cancellable items are returned in `failed` with a reason (`not_found`, `invalid_status`).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | Target record IDs (1 to 100 per call). Duplicates are ignored. |

More fields and nested objects: `kobana schema dunning.charges.bulk-cancel`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Per-item result of the bulk cancellation. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## charges get

Get charge

```text
kobana dunning charges get [ID] [OPTIONS]
```

`GET /v1/charges/{id}`

Returns a charge by ID. Use `include=engine` to receive engine telemetry (rule enrollment and step executions).

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `include` | `string` | no | Relations to include in the response (`person`, `engine` — rule telemetry). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Charge found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## charges update

Update charge

```text
kobana dunning charges update [ID] [OPTIONS]
```

`PUT /v1/charges/{id}`

Updates a charge’s data (partial). `status`, `paid_amount` and `paid_at` belong to the engine: resending the stored value is accepted, but changing them answers 422 `ENGINE_MANAGED_FIELD` — payments go through `POST /charges/{id}/pay`. The only transition accepted here is `status: cancelled` (409 if the current status does not allow it).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_id` | `string (uuid)` | no | Customer (debtor) of the charge. |
| `collection_rule_id` | `string (uuid)` | no | Collection rule tracking the charge. If omitted on create, the organization’s default rule is used. |
| `document_number` | `string` | no | Source document number (invoice, contract, etc.). |
| `description` | `string` | no | Free-form charge description. |
| `original_amount` | `number` | no | Original charge amount. |
| `interest_amount` | `number` | no | Accrued interest (default 0). |
| `fine_amount` | `number` | no | Fine amount (default 0). |
| `discount_amount` | `number` | no | Applied discount (default 0). |
| `issue_date` | `string` | no | Issue date. |
| `due_date` | `string` | no | Due date. |
| `status` | `string` | no | Status (`pending` \| `paid` \| `overdue` \| `cancelled` \| `negotiated` \| `protested` \| `negatived` \| `written_off`). Values: `pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`. |
| `payment_method` | `string` | no | Payment method (`boleto` \| `pix` \| `credit_card` \| `transfer`). Values: `boleto`, `pix`, `credit_card`, `transfer`, `null`. |
| `barcode` | `string` | no | Boleto barcode/digitable line. |
| `pix_emv` | `string` | no | Pix copy-and-paste code (EMV). |
| `payment_url` | `string (uri)` | no | Payment page URL. |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |
| `charge_policy` | `object` | no |  |
| `paid_amount` | `number` | no | Amount actually paid. |
| `paid_at` | `string` | no | Payment date. |
| `custom_data_mode` | `string` | no | How `custom_data` is applied on update: `replace` (default) overwrites the whole object; `merge` deep-merges the payload over the stored value — nested objects merge, scalars/arrays/null overwrite the key. Values: `replace`, `merge`. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.charges.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Charge updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## charges delete

Delete charge

```text
kobana dunning charges delete [ID] [OPTIONS]
```

`DELETE /v1/charges/{id}`

Soft-deletes a charge along with related tasks and offers. A charge with associated notifications, interactions, negativations or protests cannot be deleted (409).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Charge deleted (returns a confirmation message). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## charges notify

Notify charge

```text
kobana dunning charges notify [ID] [OPTIONS]
```

`POST /v1/charges/{id}/notify`

Creates a manual notification for the charge and queues it for processing. The recipient must be one of the person’s registered contacts (422 `UNKNOWN_RECIPIENT` otherwise). Updates `last_notification_at`; the rule progress (`current_step`) does not change — only the engine advances it.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `channel` | `string` | yes | Channel (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Values: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `recipient` | `string` | yes | Recipient: email for `email`, phone for `sms`/`whatsapp`/`voice` — always a registered contact of the charge’s person; free-form for `manual`. |
| `body` | `string` | yes | Message body. |
| `subject` | `string` | no | Subject (used for email). |
| `collection_rule_step_id` | `string (uuid)` | no | Step of the charge’s collection rule to associate with the notification (a label; does not change `current_step`). |

More fields and nested objects: `kobana schema dunning.charges.notify`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Notification created and queued (`queued` status). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## charges settlement-pdf

Settlement letter (PDF)

```text
kobana dunning charges settlement-pdf [ID] [OPTIONS]
```

`GET /v1/charges/{id}/settlement-pdf`

Generates the charge’s settlement letter as a PDF (legal document), scoped to the organization. Allowed only when the charge is paid (status `paid`); otherwise returns 409 (`CHARGE_NOT_SETTLED`). Responds with `application/pdf` as a binary attachment.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Settlement letter PDF (binary `application/pdf` attachment). |
| `404` | Resource not found or outside your organization. |
| `409` | Charge is not settled — the letter can only be generated for `paid` status (code `CHARGE_NOT_SETTLED`). |
| `413` | Request body larger than 2 MB. |
