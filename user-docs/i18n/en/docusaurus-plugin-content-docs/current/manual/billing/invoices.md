---
id: invoices
title: "kobana billing invoices"
sidebar_label: "invoices"
---

# kobana billing invoices


Commands of `kobana billing invoices`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`invoices installments`](#invoices-installments) | `GET /v1/invoices/{id}/installments` | List invoice installments |
| [`invoices installments create`](#invoices-installments-create) | `POST /v1/invoices/{id}/installments` | Create/update installment schedule |
| [`invoices installments mark-paid`](#invoices-installments-mark-paid) | `POST /v1/invoices/{id}/installments/{installment_id}/mark-paid` | Mark installment as paid |
| [`invoices installments pay`](#invoices-installments-pay) | `POST /v1/invoices/{id}/installments/{installment_id}/pay` | Pay installment |
| [`invoices pending-nfe`](#invoices-pending-nfe) | `GET /v1/invoices/pending-nfe` | List invoices with pending NF-e |
| [`invoices pending-payment`](#invoices-pending-payment) | `GET /v1/invoices/pending-payment` | List invoices with pending payment |
| [`invoices list`](#invoices-list) | `GET /v1/invoices` | List invoices |
| [`invoices create`](#invoices-create) | `POST /v1/invoices` | Create invoice |
| [`invoices update`](#invoices-update) | `PUT /v1/invoices` | Upsert invoice |
| [`invoices get`](#invoices-get) | `GET /v1/invoices/{id}` | Get invoice |
| [`invoices delete`](#invoices-delete) | `DELETE /v1/invoices/{id}` | Delete invoice |
| [`invoices apply-credits`](#invoices-apply-credits) | `POST /v1/invoices/{id}/apply-credits` | Apply credits to invoice |
| [`invoices finalize`](#invoices-finalize) | `POST /v1/invoices/{id}/finalize` | Finalize invoice |
| [`invoices issue-nfe`](#invoices-issue-nfe) | `POST /v1/invoices/{id}/issue-nfe` | Issue NF-e for invoice |
| [`invoices pay`](#invoices-pay) | `POST /v1/invoices/{id}/pay` | Pay invoice |
| [`invoices payment-methods`](#invoices-payment-methods) | `GET /v1/invoices/{id}/payment-methods` | List available payment methods |
| [`invoices pdf`](#invoices-pdf) | `GET /v1/invoices/{id}/pdf` | Download invoice PDF |
| [`invoices remove-credits`](#invoices-remove-credits) | `POST /v1/invoices/{id}/remove-credits` | Remove applied credits |
| [`invoices send-reminder`](#invoices-send-reminder) | `POST /v1/invoices/{id}/send-reminder` | Send invoice reminder |
| [`invoices undo-payment`](#invoices-undo-payment) | `POST /v1/invoices/{id}/undo-payment` | Undo invoice payment |
| [`invoices void`](#invoices-void) | `POST /v1/invoices/{id}/void` | Void invoice |

## invoices installments

List invoice installments

```text
kobana billing invoices installments [ID] [OPTIONS] [COMMAND]
```

`GET /v1/invoices/{id}/installments`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of installments. |
| `404` | Resource not found in the current organization. |

## invoices installments create

Create/update installment schedule

```text
kobana billing invoices installments create [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/installments`

Creates or replaces the invoice installment schedule. Minimum of 2 installments.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `installments` | `array<object>` | yes | Installment list. Minimum 2. |

More fields and nested objects: `kobana schema billing.invoices.installments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Installment schedule created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## invoices installments mark-paid

Mark installment as paid

```text
kobana billing invoices installments mark-paid [ID] [INSTALLMENT_ID] [OPTIONS]
```

`POST /v1/invoices/{id}/installments/{installment_id}/mark-paid`

Manually marks an installment as paid (reconciliation, no gateway call).

**Path arguments:** `id`, `installment_id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Installment marked as paid. |
| `404` | Resource not found in the current organization. |

## invoices installments pay

Pay installment

```text
kobana billing invoices installments pay [ID] [INSTALLMENT_ID] [OPTIONS]
```

`POST /v1/invoices/{id}/installments/{installment_id}/pay`

Processes payment for a specific installment via the gateway. The previous installment must be paid.

**Path arguments:** `id`, `installment_id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `payment_method` | `string` | yes | When provided, triggers the gateway (`pix`, `bank_slip` or `card`) or creates a transfer charge (`bank_transfer`, requires `bank_account_id` or `pix_key_id`). Omit to mark as paid manually. Values: `pix`, `bank_slip`, `card`, `bank_transfer`. |
| `payment_method_id` | `string (uuid)` | no | UUID of a saved payment method (e.g.: tokenized card). |
| `card` | `object` | no | Card data when `payment_method = card` and no `payment_method_id` is provided. |
| `bank_account_id` | `string (uuid)` | no | With `payment_method = bank_transfer`: UUID of the organization bank account the customer should transfer to (see `/bank_accounts`). Send this or `pix_key_id`, never both. |
| `pix_key_id` | `string (uuid)` | no | With `payment_method = bank_transfer`: UUID of the organization Pix key the customer should transfer to (see `/pix_keys`). Send this or `bank_account_id`, never both. |

More fields and nested objects: `kobana schema billing.invoices.installments.pay`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Installment payment result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## invoices pending-nfe

List invoices with pending NF-e

```text
kobana billing invoices pending-nfe [OPTIONS]
```

`GET /v1/invoices/pending-nfe`

Returns paid invoices that still need NF-e emission — no NF-e, or NF-e in `pending`, `processing` or `error` status.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Filter by `billing_account_id`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of invoices with pending NF-e. |
| `404` | Resource not found in the current organization. |

## invoices pending-payment

List invoices with pending payment

```text
kobana billing invoices pending-payment [OPTIONS]
```

`GET /v1/invoices/pending-payment`

Returns finalized invoices (`status = open`) past their `due_date`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Filter by `billing_account_id`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of invoices with pending payment. |
| `404` | Resource not found in the current organization. |

## invoices list

List invoices

```text
kobana billing invoices list [OPTIONS]
```

`GET /v1/invoices`

Returns a paginated list of the current organization's invoices.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `status` | `string` | no | Filter by status: `draft`, `open`, `paid`, `void` or `uncollectible`. Values: `draft`, `open`, `paid`, `void`, `uncollectible`. |
| `billing_account_id` | `string` | no | Filter by `billing_account_id`. |
| `company_id` | `string` | no | Filter by issuing `company_id`. |
| `subscription_id` | `string` | no | Filter by originating `subscription_id`. |
| `due_date_from` | `string` | no | Filter by start `due_date` (inclusive). |
| `due_date_to` | `string` | no | Filter by end `due_date` (inclusive). |
| `date_from` | `string` | no | Filter by start `created_at` (inclusive). |
| `date_to` | `string` | no | Filter by end `created_at` (inclusive). |
| `search` | `string` | no | Free-text search. |
| `total_min` | `number,null` | no | Filter by minimum total amount, in BRL (float, up to 4 decimal places). |
| `total_max` | `number,null` | no | Filter by maximum total amount, in BRL (float, up to 4 decimal places). |
| `document_number` | `string` | no | Filter by customer's CPF/CNPJ. |
| `without_payments` | `string` | no | When `true`, returns only invoices without associated payments. Values: `true`, `false`. |
| `plan_filter` | `string` | no | Filter by origin: `all`, `with_plan` (subscription) or `without_plan` (one-off). Values: `all`, `with_plan`, `without_plan`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of invoices. |
| `404` | Resource not found in the current organization. |

## invoices create

Create invoice

```text
kobana billing invoices create [OPTIONS]
```

`POST /v1/invoices`

Creates an invoice for a billing account. Accepts line items, optional installments (`installments`, minimum 2) and NF-e issuance policy.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | yes | Target billing account UUID. **Required**. |
| `line_items` | `array<object>` | yes | Line items. Minimum 1. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no | Issuing company UUID. Default: organization's default company. |
| `subscription_id` | `string (uuid)` | no | Originating subscription UUID. |
| `product_group_id` | `string (uuid)` | no | Product group UUID. |
| `collection_method` | `string` | no | Collection method. Default: `manual_invoice`. Values: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `due_date` | `string` | no | Due date. Default: today + the account's `payment_terms_days`. |
| `description` | `string` | no | Free-form description shown at the top of the invoice. |
| `footer` | `string` | no | Free-form footer shown at the bottom of the invoice. |
| `discount` | `number` | no | Fixed discount in BRL. |
| `discount_description` | `string` | no | Discount description (e.g.: "20% annual payment discount"). |
| `installments` | `array<object>` | no | Installment schedule (minimum 2). Each item has `amount` and `due_date`. |
| `nfe_issuance_policy` | `string` | no | NF-e issuance policy for installment invoices. Values: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `period_start` | `string` | no | Start of the billing period (auto-set for subscription invoices). |
| `period_end` | `string` | no | End of the billing period (auto-set for subscription invoices). |
| `custom_metadata` | `object` | no | Custom fields (validated against the organization's schema). |

More fields and nested objects: `kobana schema billing.invoices.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Invoice created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## invoices update

Upsert invoice

```text
kobana billing invoices update [OPTIONS]
```

`PUT /v1/invoices`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | Target billing account UUID. **Required**. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no | Issuing company UUID. Default: organization's default company. |
| `subscription_id` | `string (uuid)` | no | Originating subscription UUID. |
| `product_group_id` | `string (uuid)` | no | Product group UUID. |
| `collection_method` | `string` | no | Collection method. Default: `manual_invoice`. Values: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `due_date` | `string` | no | Due date. Default: today + the account's `payment_terms_days`. |
| `description` | `string` | no | Free-form description shown at the top of the invoice. |
| `footer` | `string` | no | Free-form footer shown at the bottom of the invoice. |
| `line_items` | `array<object>` | no | Line items. Minimum 1. |
| `discount` | `number` | no | Fixed discount in BRL. |
| `discount_description` | `string` | no | Discount description (e.g.: "20% annual payment discount"). |
| `installments` | `array<object>` | no | Installment schedule (minimum 2). Each item has `amount` and `due_date`. |
| `nfe_issuance_policy` | `string` | no | NF-e issuance policy for installment invoices. Values: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `period_start` | `string` | no | Start of the billing period (auto-set for subscription invoices). |
| `period_end` | `string` | no | End of the billing period (auto-set for subscription invoices). |
| `custom_metadata` | `object` | no | Custom fields (validated against the organization's schema). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.invoices.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## invoices get

Get invoice

```text
kobana billing invoices get [ID] [OPTIONS]
```

`GET /v1/invoices/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Invoice detail. |
| `404` | Resource not found in the current organization. |

## invoices delete

Delete invoice

```text
kobana billing invoices delete [ID] [OPTIONS]
```

`DELETE /v1/invoices/{id}`

Deletes a `draft`, `open` or `void` invoice (credits applied to an `open` invoice go back to the customer balance). A `paid` or `uncollectible` invoice cannot be deleted — undo the payment or void it first.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Invoice deleted. |
| `404` | Resource not found in the current organization. |

## invoices apply-credits

Apply credits to invoice

```text
kobana billing invoices apply-credits [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/apply-credits`

Applies billing-account credits to an open (`status = open`) invoice.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `credit_ids` | `array<string (uuid)>` | yes | UUIDs of the credits to apply (minimum 1). |
| `amount` | `number` | no | Amount to apply in BRL. Default: covers the entire invoice. |

More fields and nested objects: `kobana schema billing.invoices.apply-credits`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Credits applied. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## invoices finalize

Finalize invoice

```text
kobana billing invoices finalize [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/finalize`

Finalizes a draft invoice (`draft` → `open`).

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Invoice finalized. |
| `404` | Resource not found in the current organization. |

## invoices issue-nfe

Issue NF-e for invoice

```text
kobana billing invoices issue-nfe [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/issue-nfe`

Triggers issuance of the NF-e (Electronic Invoice) for the invoice.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | NF-e queued/issued. |
| `404` | Resource not found in the current organization. |

## invoices pay

Pay invoice

```text
kobana billing invoices pay [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/pay`

Two modes: (a) when `payment_method` is provided, creates a gateway payment (`pix`, `bank_slip` or `card`) and returns the method artifacts (QR code, barcode or card status) — or, with `bank_transfer`, a transfer charge to the organization bank account (`bank_account_id`) or Pix key (`pix_key_id`), with no gateway, returning the details in `bank_transfer` and staying pending until settled manually; (b) when omitted, marks the invoice as paid manually (reconciliation).

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `payment_method` | `string` | no | When provided, triggers the gateway (`pix`, `bank_slip` or `card`) or creates a transfer charge (`bank_transfer`, requires `bank_account_id` or `pix_key_id`). Omit to mark as paid manually. Values: `pix`, `bank_slip`, `card`, `bank_transfer`. |
| `payment_method_id` | `string (uuid)` | no | UUID of a saved payment method (e.g.: tokenized card). |
| `card` | `object` | no | Card data when `payment_method = card` and no `payment_method_id` is provided. |
| `bank_account_id` | `string (uuid)` | no | With `payment_method = bank_transfer`: UUID of the organization bank account the customer should transfer to (see `/bank_accounts`). Send this or `pix_key_id`, never both. |
| `pix_key_id` | `string (uuid)` | no | With `payment_method = bank_transfer`: UUID of the organization Pix key the customer should transfer to (see `/pix_keys`). Send this or `bank_account_id`, never both. |
| `amount` | `number` | no | Amount to charge in BRL. Default: invoice remaining amount. Required when `installment_ids` is given, and must equal the sum of the selected installments' `net`. |
| `installment_ids` | `array<string (uuid)>` | no | UUIDs of the installments the payment covers (split invoices). |

More fields and nested objects: `kobana schema billing.invoices.pay`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment result or invoice marked as paid. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## invoices payment-methods

List available payment methods

```text
kobana billing invoices payment-methods [ID] [OPTIONS]
```

`GET /v1/invoices/{id}/payment-methods`

Lists the payment methods available to pay this invoice, based on the organization's active gateways and portal settings.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Available payment methods. |
| `404` | Resource not found in the current organization. |

## invoices pdf

Download invoice PDF

```text
kobana billing invoices pdf [ID] [OPTIONS]
```

`GET /v1/invoices/{id}/pdf`

Generates and downloads the invoice PDF as a binary attachment (`application/pdf`).

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Invoice PDF. |
| `404` | Resource not found in the current organization. |

## invoices remove-credits

Remove applied credits

```text
kobana billing invoices remove-credits [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/remove-credits`

Removes all credits applied to the invoice, returning them to the billing account. Only works on open (`status = open`) invoices.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Credits removed. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## invoices send-reminder

Send invoice reminder

```text
kobana billing invoices send-reminder [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/send-reminder`

Sends an invoice payment reminder email to the billing account's `billing_email`.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Reminder sent. |
| `404` | Resource not found in the current organization. |

## invoices undo-payment

Undo invoice payment

```text
kobana billing invoices undo-payment [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/undo-payment`

Reverts the invoice payment (`paid` → `open`). The `reason` is recorded in the audit log.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `reason` | `string` | no | Reversal reason (recorded in the audit log). |

More fields and nested objects: `kobana schema billing.invoices.undo-payment`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment undone. |
| `404` | Resource not found in the current organization. |

## invoices void

Void invoice

```text
kobana billing invoices void [ID] [OPTIONS]
```

`POST /v1/invoices/{id}/void`

Voids an open invoice (`open` → `void`). The `reason` is recorded in the audit log.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `reason` | `string` | no | Void reason (recorded in the audit log). |

More fields and nested objects: `kobana schema billing.invoices.void`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Invoice voided. |
| `404` | Resource not found in the current organization. |
