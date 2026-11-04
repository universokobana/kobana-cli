---
id: agreements
title: "kobana dunning agreements"
sidebar_label: "agreements"
---

# kobana dunning agreements


Commands of `kobana dunning agreements`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`agreements installments`](#agreements-installments) | `GET /v1/agreements/{id}/installments` | List agreement installments |
| [`agreements installments update`](#agreements-installments-update) | `PUT /v1/agreements/{id}/installments` | Update agreement installment |
| [`agreements list`](#agreements-list) | `GET /v1/agreements` | List agreements |
| [`agreements create`](#agreements-create) | `POST /v1/agreements` | Create agreement |
| [`agreements get`](#agreements-get) | `GET /v1/agreements/{id}` | Get agreement |
| [`agreements update`](#agreements-update) | `PUT /v1/agreements/{id}` | Update agreement |
| [`agreements delete`](#agreements-delete) | `DELETE /v1/agreements/{id}` | Delete agreement |
| [`agreements accept`](#agreements-accept) | `POST /v1/agreements/{id}/accept` | Accept agreement |
| [`agreements cancel`](#agreements-cancel) | `POST /v1/agreements/{id}/cancel` | Cancel agreement |
| [`agreements pdf`](#agreements-pdf) | `GET /v1/agreements/{id}/pdf` | Agreement term (PDF) |

## agreements installments

List agreement installments

```text
kobana dunning agreements installments [ID] [OPTIONS] [COMMAND]
```

`GET /v1/agreements/{id}/installments`

Lists an agreement’s installments (ordered by number) with an aggregated summary.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Agreement installments with summary. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## agreements installments update

Update agreement installment

```text
kobana dunning agreements installments update [ID] [OPTIONS]
```

`PUT /v1/agreements/{id}/installments`

Updates an installment (payment/status). Allowed only for `accepted` or `active` agreements; cancelled installments cannot be changed (409). The first payment activates the agreement; once all installments are paid, the agreement is completed.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `installment_number` | `integer` | yes | Installment number (starts at 1). |
| `paid_amount` | `number` | no | Amount paid for the installment. |
| `paid_at` | `string` | no | Installment payment date. |
| `status` | `string` | no | Status (`pending` \| `paid` \| `overdue` \| `cancelled`). Values: `pending`, `paid`, `overdue`, `cancelled`. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.agreements.installments.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Installment updated (returns the installment and the updated agreement). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## agreements list

List agreements

```text
kobana dunning agreements list [OPTIONS]
```

`GET /v1/agreements`

Lists the organization’s agreements with pagination and filters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `sort_by` | `string` | no | Sort field (public snake_case name, e.g. `due_date`). |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |
| `status` | `string` | no | Filter by agreement status. Values: `proposed`, `accepted`, `active`, `completed`, `cancelled`, `defaulted`. |
| `person_id` | `string` | no | Filter by a customer’s agreements (UUID). |
| `include` | `string` | no | Relations to include in the response (`person`, `installments`, `negotiation_offer`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated agreement list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## agreements create

Create agreement

```text
kobana dunning agreements create [OPTIONS]
```

`POST /v1/agreements`

Creates an agreement with `proposed` status. `final_total` = `original_total` − `discount_amount` (must be greater than zero) and `installment_value` = `final_total` / `number_of_installments`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_id` | `string (uuid)` | yes | Customer (debtor) of the agreement. |
| `original_total` | `number` | yes | Original total of the renegotiated debt. |
| `number_of_installments` | `integer` | yes | Number of installments (1 to 120). |
| `first_due_date` | `string` | yes | First installment due date (the rest fall due monthly). |
| `negotiation_offer_id` | `string (uuid)` | no | Negotiation offer that originated the agreement. |
| `discount_amount` | `number` | no | Granted discount (default 0). |
| `terms_accepted` | `string` | no | Terms accepted by the debtor (text/version). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.agreements.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Agreement created. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## agreements get

Get agreement

```text
kobana dunning agreements get [ID] [OPTIONS]
```

`GET /v1/agreements/{id}`

Returns an agreement by ID (always includes summarized customer data).

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `include` | `string` | no | Relations to include in the response (`installments`, `negotiation_offer`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Agreement found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## agreements update

Update agreement

```text
kobana dunning agreements update [ID] [OPTIONS]
```

`PUT /v1/agreements/{id}`

Updates an agreement (partial). Allowed only while status is `proposed` or `accepted` (409 otherwise).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `original_total` | `number` | no | Original total of the renegotiated debt. |
| `discount_amount` | `number` | no | Granted discount (default 0). |
| `number_of_installments` | `integer` | no | Number of installments (1 to 120). |
| `first_due_date` | `string` | no | First installment due date (the rest fall due monthly). |
| `terms_accepted` | `string` | no | Terms accepted by the debtor (text/version). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_data_mode` | `string` | no | How `custom_data` is applied on update: `replace` (default) overwrites the whole object; `merge` deep-merges the payload over the stored value — nested objects merge, scalars/arrays/null overwrite the key. Values: `replace`, `merge`. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.agreements.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Agreement updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## agreements delete

Delete agreement

```text
kobana dunning agreements delete [ID] [OPTIONS]
```

`DELETE /v1/agreements/{id}`

Soft-deletes an agreement, marking it `cancelled`. `completed` agreements cannot be deleted (409).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Agreement deleted. |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## agreements accept

Accept agreement

```text
kobana dunning agreements accept [ID] [OPTIONS]
```

`POST /v1/agreements/{id}/accept`

Accepts a `proposed` agreement (409 for other statuses): records `accepted_at`/`accepted_ip` and creates the installments with monthly due dates starting at `first_due_date`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `accepted_ip` | `string` | no | IP recorded on acceptance (audit trail). |
| `terms_accepted` | `string` | no | Terms accepted by the debtor (text/version). |

More fields and nested objects: `kobana schema dunning.agreements.accept`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Agreement accepted (returns the agreement with the created installments). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## agreements cancel

Cancel agreement

```text
kobana dunning agreements cancel [ID] [OPTIONS]
```

`POST /v1/agreements/{id}/cancel`

Cancels an agreement, recording `cancelled_at` and the reason, and cancels pending/overdue installments. `completed` or already-cancelled agreements return 409.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `reason` | `string` | yes | Cancellation reason (required; stored in `cancellation_reason`). |

More fields and nested objects: `kobana schema dunning.agreements.cancel`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Agreement cancelled (returns the agreement with its installments). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## agreements pdf

Agreement term (PDF)

```text
kobana dunning agreements pdf [ID] [OPTIONS]
```

`GET /v1/agreements/{id}/pdf`

Generates the agreement term as a PDF (legal document) with creditor, debtor, totals and installments, scoped to the organization. Responds with `application/pdf` as a binary attachment.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Agreement term PDF (binary `application/pdf` attachment). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
