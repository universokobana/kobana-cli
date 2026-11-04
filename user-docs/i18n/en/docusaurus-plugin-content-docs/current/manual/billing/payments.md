---
id: payments
title: "kobana billing payments"
sidebar_label: "payments"
---

# kobana billing payments


Commands of `kobana billing payments`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`payments list`](#payments-list) | `GET /v1/payments` | List payments |
| [`payments create`](#payments-create) | `POST /v1/payments` | Process payment |
| [`payments bulk-update-invoice`](#payments-bulk-update-invoice) | `POST /v1/payments/bulk-update-invoice` | Reassign payments to another invoice |
| [`payments get`](#payments-get) | `GET /v1/payments/{id}` | Get payment |
| [`payments delete`](#payments-delete) | `DELETE /v1/payments/{id}` | Delete payment |
| [`payments cancel`](#payments-cancel) | `POST /v1/payments/{id}/cancel` | Cancel payment |
| [`payments change-card`](#payments-change-card) | `POST /v1/payments/{id}/change-card` | Change payment card |
| [`payments change-due-date`](#payments-change-due-date) | `POST /v1/payments/{id}/change-due-date` | Change bank slip due date |
| [`payments refund`](#payments-refund) | `POST /v1/payments/{id}/refund` | Refund payment |
| [`payments retry`](#payments-retry) | `POST /v1/payments/{id}/retry` | Retry payment |
| [`payments send-email`](#payments-send-email) | `POST /v1/payments/{id}/send-email` | Send bank slip by email |
| [`payments sync`](#payments-sync) | `POST /v1/payments/{id}/sync` | Sync payment with gateway |

## payments list

List payments

```text
kobana billing payments list [OPTIONS]
```

`GET /v1/payments`

Returns a paginated list of the current organization's payments.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `person_id` | `string` | no | Filter by associated person UUID. |
| `person_document_number` | `string` | no | Filter by person CPF/CNPJ (exact match). |
| `person_name_contains` | `string` | no | Filter by person name (partial search, case-insensitive). |
| `status` | `string` | no | Filter by payment status. Values: `pending`, `processing`, `succeeded`, `failed`, `canceled`, `partial`, `refunded`, `partially_refunded`. |
| `invoice_id` | `string` | no | Filter by payments of a specific invoice. |
| `billing_account_id` | `string` | no | Filter by payments of a billing account. |
| `company_id` | `string` | no | Filter by payments of a specific company. |
| `gateway_id` | `string` | no | Filter by payments processed by a specific gateway. |
| `payment_method_id` | `string` | no | Filter by payments of a specific payment method. |
| `payment_method_type` | `string` | no | Filter by payment method: `card`, `bank_slip`, `pix` or `bank_transfer`. Values: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `external_id` | `string` | no | Filter by external ID (free field for legacy system identifiers). |
| `gateway_payment_id` | `string` | no | Filter by the payment identifier at the upstream gateway. |
| `invoice_filter` | `string` | no | Filter by invoice presence: `all`, `with_invoice` or `without_invoice`. Values: `all`, `with_invoice`, `without_invoice`. |
| `search` | `string` | no | Free-text search. |
| `amount_min` | `number,null` | no | Minimum charged amount (in BRL). |
| `amount_max` | `number,null` | no | Maximum charged amount (in BRL). |
| `paid_amount_min` | `number,null` | no | Minimum amount actually received (in BRL). |
| `paid_amount_max` | `number,null` | no | Maximum amount actually received (in BRL). |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `paid_after` | `string` | no | Paid-at start date — ISO 8601 format. |
| `paid_before` | `string` | no | Paid-at end date — ISO 8601 format. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of payments. |
| `404` | Resource not found in the current organization. |

## payments create

Process payment

```text
kobana billing payments create [OPTIONS]
```

`POST /v1/payments`

Processes a payment for an existing invoice. Accepts a previously registered `payment_method_id` **or** a `payment_method` block with card/PIX/bank-slip data. When `amount` is omitted, charges the invoice open balance.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `invoice_id` | `string (uuid)` | yes | UUID of the invoice to be paid. |
| `payment_method_id` | `string (uuid)` | no | UUID of a payment method previously registered for the account. |
| `payment_method` | `object` | no |  |
| `amount` | `number` | no | Amount to charge in BRL (float, up to 4 decimal places). Default: invoice open balance. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Free-form metadata defined by the client for internal correlation. |

More fields and nested objects: `kobana schema billing.payments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payments bulk-update-invoice

Reassign payments to another invoice

```text
kobana billing payments bulk-update-invoice [OPTIONS]
```

`POST /v1/payments/bulk-update-invoice`

Bulk-updates the invoice associated with the given payments. Useful for fixing links when payments arrived pointing to the wrong invoice. A payment that already settles more than one invoice is not moved: it comes back in `failed` (with `id` and `error`), counted by `failed_count`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `payment_ids` | `array<string (uuid)>` | yes | List of UUIDs of the payments to update. |
| `invoice_id` | `string (uuid)` | yes | UUID of the target invoice. |

More fields and nested objects: `kobana schema billing.payments.bulk-update-invoice`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payments reassigned. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payments get

Get payment

```text
kobana billing payments get [ID] [OPTIONS]
```

`GET /v1/payments/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment detail. |
| `404` | Resource not found in the current organization. |

## payments delete

Delete payment

```text
kobana billing payments delete [ID] [OPTIONS]
```

`DELETE /v1/payments/{id}`

Removes a payment. Confirmed payments cannot be deleted — use `cancel` or `refund` instead.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment removed. |
| `404` | Resource not found in the current organization. |

## payments cancel

Cancel payment

```text
kobana billing payments cancel [ID] [OPTIONS]
```

`POST /v1/payments/{id}/cancel`

Cancels an open payment. When `cancel_on_gateway` is `true` (default), the cancellation is propagated to the gateway.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `reason` | `string` | no | Void reason (recorded in the audit log). |
| `cancel_on_gateway` | `boolean` | no | When `true` (default), also cancels at the upstream gateway. |

More fields and nested objects: `kobana schema billing.payments.cancel`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment canceled. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payments change-card

Change payment card

```text
kobana billing payments change-card [ID] [OPTIONS]
```

`POST /v1/payments/{id}/change-card`

Replaces the credit card on a failed Pagar.me payment. The new card must be pre-registered in `payment_methods`.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `credit_card_id` | `string (uuid)` | yes | UUID of the billing account's active credit card. |

More fields and nested objects: `kobana schema billing.payments.change-card`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Card changed. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payments change-due-date

Change bank slip due date

```text
kobana billing payments change-due-date [ID] [OPTIONS]
```

`POST /v1/payments/{id}/change-due-date`

Changes the due date of a pending bank-slip payment issued via the Kobana gateway. The new date must be in the future.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `due_date` | `string` | yes | New due date in `YYYY-MM-DD` format. |

More fields and nested objects: `kobana schema billing.payments.change-due-date`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Due date updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payments refund

Refund payment

```text
kobana billing payments refund [ID] [OPTIONS]
```

`POST /v1/payments/{id}/refund`

Refunds a successful payment. Without `amount`, refunds the full amount; with a partial amount, issues a partial refund.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | no | Amount to refund in BRL (float, up to 4 decimal places). Default: full payment amount. |
| `reason` | `string` | no | Refund reason (recorded in the audit log). |

More fields and nested objects: `kobana schema billing.payments.refund`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment refunded. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payments retry

Retry payment

```text
kobana billing payments retry [ID] [OPTIONS]
```

`POST /v1/payments/{id}/retry`

Reprocesses a failed payment. Optionally switches to a new card (via `card`) or to a saved payment method (`payment_method_id`).

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `card` | `object` | no |  |
| `payment_method_id` | `string (uuid)` | no | UUID of a saved payment method to use in the retry. |

More fields and nested objects: `kobana schema billing.payments.retry`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment retried. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payments send-email

Send bank slip by email

```text
kobana billing payments send-email [ID] [OPTIONS]
```

`POST /v1/payments/{id}/send-email`

Sends the bank-slip link by email to the given recipient. Only available for pending bank-slip payments.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string (email)` | yes | Recipient's email address. |
| `message` | `string` | no | Optional message included in the email body. |

More fields and nested objects: `kobana schema billing.payments.send-email`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Email sent. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payments sync

Sync payment with gateway

```text
kobana billing payments sync [ID] [OPTIONS]
```

`POST /v1/payments/{id}/sync`

Syncs the payment status with the upstream gateway (Kobana PIX/boleto or Pagar.me refund). Returns the refreshed payment.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment synced. |
| `404` | Resource not found in the current organization. |
