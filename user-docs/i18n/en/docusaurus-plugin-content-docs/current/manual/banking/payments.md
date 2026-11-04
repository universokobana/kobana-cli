---
id: payments
title: "kobana banking payments"
sidebar_label: "payments"
description: "Payments (unified)"
---

# kobana banking payments

Payments (unified)

Commands of `kobana banking payments`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`payments list`](#payments-list) | `GET /v2/payments` | List Payments |
| [`payments create`](#payments-create) | `POST /v2/payments` | Create a Payment |
| [`payments barcode`](#payments-barcode) | `POST /v2/payments/barcode` | Create a Payment by barcode |
| [`payments get`](#payments-get) | `GET /v2/payments/{uid}` | View a Payment |
| [`payments approve`](#payments-approve) | `PUT /v2/payments/{uid}/approve` | Release (approve) a Payment |
| [`payments cancel`](#payments-cancel) | `PUT /v2/payments/{uid}/cancel` | Cancel a Payment |

## payments list

List Payments

```text
kobana banking payments list [OPTIONS]
```

`GET /v2/payments`

Lists all payments of any type (PIX, Bank Billet, DARF, Taxes, Utilities). Supports filters and pagination. This is a unified endpoint that returns payments of all types through a single query.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `kind` | `string` | no | Filter by payment type Values: `bank_billet`, `darf`, `utility`, `gru`, `pix`, `tax`. |
| `status` | `string` | no | Filter by status Values: `pending`, `scheduled`, `confirmed`, `rejected`, `canceled`. |
| `created_from` | `string` | no | Filter by initial creation date (format: YYYY-MM-DD, inclusive) |
| `created_to` | `string` | no | Filter by final creation date (format: YYYY-MM-DD, includes the entire day) |
| `updated_from` | `string` | no | Filter by initial update date (format: YYYY-MM-DD, inclusive) |
| `updated_to` | `string` | no | Filter by final update date (format: YYYY-MM-DD, includes the entire day) |
| `scheduled_from` | `string` | no | Filter by initial scheduled date (format: YYYY-MM-DD) |
| `scheduled_to` | `string` | no | Filter by final scheduled date (format: YYYY-MM-DD) |
| `scheduled` | `boolean` | no | Filter only scheduled (true) or non-scheduled (false) payments |
| `identifier` | `string` | no | Search by identifier (partial search) |
| `external_id` | `string` | no | Filter by external ID |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payments Found |

## payments create

Create a Payment

```text
kobana banking payments create [OPTIONS]
```

`POST /v2/payments`

Generic endpoint for creating payments of any type. The type must be specified in the `kind` field.

**Prerequisites**

To issue a payment, you must have a Financial Account, informed in `financial_account_uid`. The payment account that records the operation is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if informed, it must belong to `financial_account_uid`.

**Supported Types**

**PIX**
Use `kind: "pix"` and provide the `pix` key:
```json
{
  "kind": "pix",
  "financial_account_uid": "...",
  "amount": 100.0,
  "pix": {
    "qrcode": "00020126330014br.gov.bcb.pix..."
  }
}
```

**Bank Billet**
Use `kind: "bank_billet"` and provide the `bank_billet` key:
```json
{
  "kind": "bank_billet",
  "financial_account_uid": "...",
  "amount": 200.0,
  "bank_billet": {
    "code": "81770000001-8 00005659970-7 41131079703-9 0014334-8"
  }
}
```

**DARF**
Use `kind: "darf"` and provide the `darf` key:
```json
{
  "kind": "darf",
  "financial_account_uid": "...",
  "amount": 300.0,
  "darf": {
    "tax_code": "0107",
    "calculation_date": "2024-12-31",
    "expire_at": "2025-01-10"
  }
}
```

**Taxes**
Use `kind: "tax"` and provide the `tax` key with bar code:
```json
{
  "kind": "tax",
  "financial_account_uid": "...",
  "amount": 400.0,
  "tax": {
    "kind": "iptu",
    "code": "81770000001-8 00005659970-7..."
  }
}
```

**Utility Bills**
Use `kind: "utility"` and provide the `utility` key:
```json
{
  "kind": "utility",
  "financial_account_uid": "...",
  "amount": 500.0,
  "utility": {
    "code": "81770000001-8 00005659970-7..."
  }
}
```

**Webhook Events**

The `payment.db.created` event is triggered in this operation.

> **Unified Endpoint:** This endpoint unifies the creation of all types of payments. To create payments of a specific type, you can also use the dedicated endpoints in `/api/v2/payment/{type}`.

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.payments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Utility Bill Payment created |
| `422` | Tax with format=code and missing code |

## payments barcode

Create a Payment by barcode

```text
kobana banking payments barcode [OPTIONS]
```

`POST /v2/payments/barcode`

Creates a payment from just the barcode or the typable line. The payment type is resolved from the code itself, following the FEBRABAN v08 layout, and returned in the `kind` field of the response:

| Code | `kind` |
|---|---|
| 47 positions, or 44 not starting with `8` | `bank_billet` |
| starts with `8`, segment `2`, `3`, `4`, `6` or `7` | `utility` |
| starts with `8`, segment `1` or `5` | `tax`, with `tax.kind = "unknown"` and `tax.format = "code"` |

The code is rejected with `422` when a check digit does not match (the general one, in boleto and collection codes, and the field or block ones of the typable line), when it uses segment `9`, `0` or `8`, when the length does not match the first digit (47 positions starting with `8`, 48 not starting with `8`), or when it is an FGTS slip (segment `5` with agreement `0181` or `0182` in positions 16 to 19).

**Prerequisites**

`payment_account_uid` is required and defines the source financial account. `financial_account_uid` is not accepted in this endpoint.

`amount` is optional when the code carries the amount to pay: collection codes with value identifier `6` or `8` (3rd position) and boletos with a non-zero amount. Without `amount`, the payment uses the code's amount; for a boleto, it is the face value, without interest, fine or discount. It is required for collection codes with identifier `7` or `9`, where the code's amount is only a reference, and when the code comes with a zero amount.

**Webhook Events**

The `payment.db.created` event is triggered by this operation.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `code` | `string` | yes | Barcode (44 positions) or typable line (47 or 48 positions). Spaces, periods, hyphens and slashes are discarded. |
| `payment_account_uid` | `string (uuid)` | yes | UID of the source Payment Account. Required: there is no automatic selection. |
| `amount` | `number` | no | Amount. Optional when the code carries the amount to pay (collection code with value identifier `6` or `8`, or boleto with a non-zero amount): without it, the code's amount applies, which for a boleto is the face value, without interest, fine or discount. Required for collection codes with identifier `7` or `9` and when the code comes with a zero amount. When sent, it always prevails over the code's amount: to pay an overdue boleto with charges, send the amount including them. |
| `scheduled_to` | `string (date)` | no | Scheduling Date |
| `beneficiary` | `object` | no | Beneficiary information. Required when the code is a boleto and the financial account is BB. |
| `identifier` | `string` | no | Payment identification on the financial provider's receipt. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |

More fields and nested objects: `kobana schema banking.payments.barcode`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Boleto payment created |
| `422` | Code with reference amount, without amount |

## payments get

View a Payment

```text
kobana banking payments get [UID] [OPTIONS]
```

`GET /v2/payments/{uid}`

Consults specific information about a Payment of any type. The type is identified automatically and the response is serialized appropriately.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | GRU Tax with format=data |
| `404` | Payment Not Found |

## payments approve

Release (approve) a Payment

```text
kobana banking payments approve [UID] [OPTIONS]
```

`PUT /v2/payments/{uid}/approve`

> **Asynchronous Processing:** creates a release (approve) command for a Payment that is awaiting approval.

The release is done asynchronously: when receiving the request, we create a Command that is processed in the background. You receive the ID of the created command in the response and can consult it at any time.

> ⚠️ **Bank support:** not all providers expose individual payment release via API. When the provider does not support the operation, the response is `422`.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Release command created successfully |
| `422` | Payment is not in a state that allows release or provider does not support |

## payments cancel

Cancel a Payment

```text
kobana banking payments cancel [UID] [OPTIONS]
```

`PUT /v2/payments/{uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the Payment.
The cancellation of a Payment is done asynchronously, that is, when receiving the request, our API stores the request and creates a Command. This command is processed in the background and after being confirmed, the Payment has its status changed to Canceled. You receive the ID of the created command in the response of this request and can consult it at any time.

**Webhook Events**

When the request is made, a `payment.{type}.cancel.requested` event is triggered (where \{type\} can be pix, bank_billet, etc).
Our server then starts attempts to send the cancellation to the financial institution.
Only after confirmed at the financial institution, then the payment is marked as canceled in our database.
At this moment the `payment.{type}.cancel.confirmed` event is triggered.

> ⚠️ **Attention to Canceled status:** The Canceled status is terminal. The payment can no longer be processed after cancellation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment cancelled without provider interaction |
| `422` | Payment has already been confirmed, rejected, or canceled |
