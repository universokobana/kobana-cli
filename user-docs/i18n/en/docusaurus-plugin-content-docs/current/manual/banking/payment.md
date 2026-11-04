---
id: payment
title: "kobana banking payment"
sidebar_label: "payment"
description: "Payments (bank slips, Pix, taxes, utilities)"
---

# kobana banking payment

Payments (bank slips, Pix, taxes, utilities)

Commands of `kobana banking payment`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`payment accounts list`](#payment-accounts-list) | `GET /v2/payment/accounts` | List Payment Accounts |
| [`payment accounts create`](#payment-accounts-create) | `POST /v2/payment/accounts` | Create Payment Account |
| [`payment accounts get`](#payment-accounts-get) | `GET /v2/payment/accounts/{uid}` | Fetch Payment Account |
| [`payment accounts update`](#payment-accounts-update) | `PATCH /v2/payment/accounts/{uid}` | Update Payment Account |
| [`payment bank-billet-batches create`](#payment-bank-billet-batches-create) | `POST /v2/payment/bank_billet_batches` | Create a Bank Billet Payment Batch |
| [`payment bank-billets list`](#payment-bank-billets-list) | `GET /v2/payment/bank_billets` | List Billet Payments |
| [`payment bank-billets create`](#payment-bank-billets-create) | `POST /v2/payment/bank_billets` | Create a Bank Billet Payment |
| [`payment bank-billets get`](#payment-bank-billets-get) | `GET /v2/payment/bank_billets/{uid}` | View a Bank Billet Payment |
| [`payment bank-billets cancel`](#payment-bank-billets-cancel) | `PUT /v2/payment/bank_billets/{uid}/cancel` | Cancel a Bank Billet Payment |
| [`payment batches list`](#payment-batches-list) | `GET /v2/payment/batches` | List Payment Batches |
| [`payment batches get`](#payment-batches-get) | `GET /v2/payment/batches/{uid}` | View a Payment Batch |
| [`payment batches approve`](#payment-batches-approve) | `PUT /v2/payment/batches/{uid}/approve` | Approve a Payment Batch |
| [`payment batches cancel`](#payment-batches-cancel) | `PUT /v2/payment/batches/{uid}/cancel` | Cancel a Payment Batch |
| [`payment batches reprove`](#payment-batches-reprove) | `PUT /v2/payment/batches/{uid}/reprove` | Reject a Payment Batch |
| [`payment dda-accounts list`](#payment-dda-accounts-list) | `GET /v2/payment/dda_accounts` | List DDA Accounts |
| [`payment dda-accounts create`](#payment-dda-accounts-create) | `POST /v2/payment/dda_accounts` | Create a DDA Account |
| [`payment dda-accounts get`](#payment-dda-accounts-get) | `GET /v2/payment/dda_accounts/{uid}` | Fetch DDA Account |
| [`payment dda-accounts update`](#payment-dda-accounts-update) | `PUT /v2/payment/dda_accounts/{uid}` | Update DDA Account |
| [`payment dda-accounts disable`](#payment-dda-accounts-disable) | `PUT /v2/payment/dda_accounts/{uid}/disable` | Disable DDA Account |
| [`payment dda-accounts enable`](#payment-dda-accounts-enable) | `PUT /v2/payment/dda_accounts/{uid}/enable` | Enable DDA Account |
| [`payment dda-bank-billets list`](#payment-dda-bank-billets-list) | `GET /v2/payment/dda_bank_billets` | List DDA Bank Billets |
| [`payment dda-bank-billets get`](#payment-dda-bank-billets-get) | `GET /v2/payment/dda_bank_billets/{uid}` | Fetch DDA Bank Billet |
| [`payment dda-bank-billets delete`](#payment-dda-bank-billets-delete) | `DELETE /v2/payment/dda_bank_billets/{uid}` | Delete DDA Bank Billet |
| [`payment dda-bank-billets reject`](#payment-dda-bank-billets-reject) | `PUT /v2/payment/dda_bank_billets/{uid}/reject` | Reject DDA Bank Billet |
| [`payment dda-bank-billets release`](#payment-dda-bank-billets-release) | `PUT /v2/payment/dda_bank_billets/{uid}/release` | Release DDA Bank Billet for Payment |
| [`payment pix list`](#payment-pix-list) | `GET /v2/payment/pix` | List PIX Payments |
| [`payment pix create`](#payment-pix-create) | `POST /v2/payment/pix` | Create a PIX Payment |
| [`payment pix get`](#payment-pix-get) | `GET /v2/payment/pix/{uid}` | View a PIX Payment |
| [`payment pix cancel`](#payment-pix-cancel) | `PUT /v2/payment/pix/{uid}/cancel` | Cancel a PIX Payment |
| [`payment pix-batches create`](#payment-pix-batches-create) | `POST /v2/payment/pix_batches` | Create a PIX Payment Batch |
| [`payment tax-batches create`](#payment-tax-batches-create) | `POST /v2/payment/tax_batches` | Create a Tax Payment Batch |
| [`payment taxes list`](#payment-taxes-list) | `GET /v2/payment/taxes` | List Tax Payments |
| [`payment taxes create`](#payment-taxes-create) | `POST /v2/payment/taxes` | Create a Tax Payment |
| [`payment taxes get`](#payment-taxes-get) | `GET /v2/payment/taxes/{uid}` | Display Tax Payment |
| [`payment taxes cancel`](#payment-taxes-cancel) | `PUT /v2/payment/taxes/{uid}/cancel` | Cancel a Tax Payment |
| [`payment utilities list`](#payment-utilities-list) | `GET /v2/payment/utilities` | List Utility Payments |
| [`payment utilities create`](#payment-utilities-create) | `POST /v2/payment/utilities` | Create a Utility Payment |
| [`payment utilities get`](#payment-utilities-get) | `GET /v2/payment/utilities/{uid}` | View a Utility Payment |
| [`payment utilities cancel`](#payment-utilities-cancel) | `PUT /v2/payment/utilities/{uid}/cancel` | Cancel a Utility Payment |
| [`payment utility-batches create`](#payment-utility-batches-create) | `POST /v2/payment/utility_batches` | Create a Batch of Utility Bill Payments |

## payment accounts list

List Payment Accounts

```text
kobana banking payment accounts list [OPTIONS]
```

`GET /v2/payment/accounts`

Lists all payment accounts registered in the account. Supports pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `financial_account_uid` | `string` | no | Filter by financial account UID |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment Accounts accessed with read scope |

## payment accounts create

Create Payment Account

```text
kobana banking payment accounts create [OPTIONS]
```

`POST /v2/payment/accounts`

Creates a new payment account associated with a financial account. Use when the financial account needs more than one payment account (for example, multiple agreements at the same bank). The account is created without a bank connection (`connection_uid` returns `null`); associate a connection before enabling remittances/registration. Requires `write` scope + `payment.accounts`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string` | yes | UUID of the Financial Account that will receive the new payment account |
| `agreement_code` | `string` | no | Agreement code |
| `external_id` | `string` | no | External identifier of the payment account |
| `remittance_enabled` | `boolean` | no | Enables or disables generation of EDI remittances for this account. |
| `provider_data` | `object` | no | Provider-specific attributes. |

More fields and nested objects: `kobana schema banking.payment.accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment Account Created |
| `404` | Financial Account not found |
| `422` | Validation error. `agreement_code` above the limit. |

## payment accounts get

Fetch Payment Account

```text
kobana banking payment accounts get [UID] [OPTIONS]
```

`GET /v2/payment/accounts/{uid}`

Returns the details of a specific payment account by UID

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment Account accessed with read scope |
| `404` | Payment Account Not Found |

## payment accounts update

Update Payment Account

```text
kobana banking payment accounts update [UID] [OPTIONS]
```

`PATCH /v2/payment/accounts/{uid}`

Updates editable attributes of a payment account. Accepts `remittance_enabled` (boolean, controls EDI remittance generation), `provider_data.service_point_number` (string, service point — for example `numeroPa` in Sicoob) and `provider_data.cooperative_cnpj` (string, Sicoob cooperative CNPJ). The body accepts partial PATCH: send only the fields you want to update.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `uid` | `string` | no | UUID of the payment account |
| `financial_account_uid` | `string` | no | UUID of associated financial account |
| `connection_uid` | `string (uuid)` | no | Banking connection of the account. |
| `agreement_code` | `string` | no | Agreement code |
| `provider_data` | `object` | no | Provider-specific data. |
| `external_id` | `string` | no | External identifier of the payment account |
| `balance_validation_enabled_at` | `string (date-time)` | no | Date and time when balance validation was enabled |
| `remittance_enabled_at` | `string (date-time)` | no | Date and time when EDI remittance was enabled |
| `remittance_enabled` | `boolean` | no | Enables or disables generation of EDI remittances for this account. |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.payment.accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Partial PATCH — only service_point_number (without remittance_enabled) |
| `404` | Payment Account Not Found |
| `422` | Validation error. `remittance_enabled` must be true or false. |

## payment bank-billet-batches create

Create a Bank Billet Payment Batch

```text
kobana banking payment bank-billet-batches create [OPTIONS]
```

`POST /v2/payment/bank_billet_batches`

This endpoint is used to generate a payment batch.

**Prerequisite**

To issue a payment batch, you must have a Financial Account, specified in `financial_account_uid`. The payment account applied to the batch is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Webhook Events**

The `payment.batch.db.created` event is triggered in this operation.

When registering a Payment Batch, the system creates a command to register it in the financial institution. After the batch registration is confirmed in the institution, the `payment.bank_billet_batch.confirmed` event is triggered.

The `payment.db.created` event is also triggered in this operation if the batch contains a new payment.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `payments` | `array<anyOf>` | yes | List of New or Existing Boleto Payments. |
| `payment_account_uid` | `string (uuid)` | no | Payment account applied to the entire batch (optional). If omitted, the oldest from the financial account is used. If provided, must belong to the batch's financial account. |

More fields and nested objects: `kobana schema banking.payment.bank-billet-batches.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment Batch Created |
| `422` | Validation error — duplicate external_id |

## payment bank-billets list

List Billet Payments

```text
kobana banking payment bank-billets list [OPTIONS]
```

`GET /v2/payment/bank_billets`

Lists all bank billet payments made. Supports pagination and returns detailed information about each payment.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payments Found |

## payment bank-billets create

Create a Bank Billet Payment

```text
kobana banking payment bank-billets create [OPTIONS]
```

`POST /v2/payment/bank_billets`

This endpoint is used to generate a payment.

**Prerequisite**

To issue a payment, you must have a Financial Account, specified in `financial_account_uid`. The payment account that records the operation is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Webhook Events**

The `payment.db.created` event is triggered in this operation.

> **Next steps:** After creating a payment, you may want to send a Batch to the financial institution at some point. To do this, use the `POST /api/v2/payment/bank_billet_batches` endpoint.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount |
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `code` | `string` | yes | Barcode or Typable Line |
| `scheduled_to` | `string (date)` | no | Scheduling Date |
| `payment_account_uid` | `string (uuid)` | no | Payment account of origin (optional). If omitted, the oldest from the financial account is used. If provided, must belong to `financial_account_uid`. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |
| `transfer_purpose` | `string` | no | Payment purpose. Defines the batch service type in the remittance file sent to the bank. Optional. When not provided, the remittance is sent as `98` (Miscellaneous Payments). Provide it only if the bank requires a specific batch type: Itaú, for example, rejects remittances whose batch type the company has not contracted. Payments with different purposes are sent in separate remittances, because the service type describes the whole batch. For payments, only Itaú currently reads this field in the remittance. At the other banks the batch type is fixed: the field is accepted and returned in the response, but does not change the file sent. `20`: Supplier Payment. `98`: Miscellaneous Payments Values: `20`, `98`. |
| `beneficiary` | `object` | no | Beneficiary Data. Required when the financial account is BB; otherwise, optional — when provided, replaces the data extracted from the barcode. |

More fields and nested objects: `kobana schema banking.payment.bank-billets.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment Created |
| `422` | Validation error |

## payment bank-billets get

View a Bank Billet Payment

```text
kobana banking payment bank-billets get [UID] [OPTIONS]
```

`GET /v2/payment/bank_billets/{uid}`

Queries specific information about a Bank Billet Payment.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment Found |
| `404` | Payment Not Found |

## payment bank-billets cancel

Cancel a Bank Billet Payment

```text
kobana banking payment bank-billets cancel [UID] [OPTIONS]
```

`PUT /v2/payment/bank_billets/{uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the Payment.
The cancellation of a Payment is done asynchronously, that is, upon receiving the request, our API saves the request and creates a Command. This command is processed in the background and after confirmation, the Payment has its status changed to Cancelled. You receive in the response of this request the ID of the created command and can consult it at any time.

**Webhook Events**

When the request is made, a `payment.bank_billet.cancel.requested` event is triggered.
Our server then begins attempts to send the cancellation to the financial institution.
Only after confirmation from the financial institution is the payment marked as cancelled in our database.
At this moment the `payment.bank_billet.cancel.confirmed` event is triggered.

> ⚠️ **Attention to Cancelled Status:** The Cancelled status is terminal. The payment cannot be processed further after cancellation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment cancelled without provider interaction |
| `422` | Payment has already been confirmed, rejected, or canceled |

## payment batches list

List Payment Batches

```text
kobana banking payment batches list [OPTIONS]
```

`GET /v2/payment/batches`

Returns a paginated list of all payment batches in the account. The batches are returned in descending order by creation date, with the most recent first. This endpoint allows you to filter and paginate the results to facilitate navigation through large volumes of batches.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Batches Found |

## payment batches get

View a Payment Batch

```text
kobana banking payment batches get [UID] [OPTIONS]
```

`GET /v2/payment/batches/{uid}`

Queries specific information about a Bank Billet Payment.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment Batch Found |
| `404` | Payment Batch Not Found |

## payment batches approve

Approve a Payment Batch

```text
kobana banking payment batches approve [UID] [OPTIONS]
```

`PUT /v2/payment/batches/{uid}/approve`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates an approval command for the Payment Batch.
  The approval of a Payment Batch is done asynchronously, that is, upon receiving the request, our API saves the request and creates a Command. This command is processed in the background and after confirmation, the Batch has its status changed to Approved. You receive in the response of this request the ID of the created command and can consult it at any time.

  ## Webhook Events

  When the request is made, a `payment.payment_batch.approve.requested` event is triggered.
  Our server then begins attempts to send the release to the financial institution.
  Only after confirmation from the financial institution is the batch marked as approved in our database.
  At this moment the `payment.payment_batch.approve.confirmed` event is triggered.

  > ⚠️ **Attention to Approved Status:** The Approved status is not terminal. When the financial institution confirms the payments, they will have their status changed to Confirmed

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `404` | Payment Batch Not Found |
| `422` | Transfer Batch is not awaiting release |

## payment batches cancel

Cancel a Payment Batch

```text
kobana banking payment batches cancel [UID] [OPTIONS]
```

`PUT /v2/payment/batches/{uid}/cancel`

> **Asynchronous Processing:** creates a cancellation command for the Payment Batch.

  When the financial provider exposes a batch cancellation API (ex: Banco do Brasil),
  1 cancellation Command is created in the batch. For providers without batch API, the
  cancellation is done payment by payment (fallback per-item). Before registration, the
  batch is cancelled locally without provider interaction.

  ## Webhook Events

  When the request is made, a `payment.{type}_batch.cancel.requested` event is triggered.
  Only after confirmation from the financial institution is the batch marked as cancelled,
  triggering `payment.{type}_batch.cancel.confirmed`.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Cancellation command created successfully |
| `422` | Payment Batch cannot be cancelled in the current state |

## payment batches reprove

Reject a Payment Batch

```text
kobana banking payment batches reprove [UID] [OPTIONS]
```

`PUT /v2/payment/batches/{uid}/reprove`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a rejection command for the Payment Batch.
  The rejection of a Payment Batch is done asynchronously, that is, upon receiving the request, our API saves the request and creates a Command. This command is processed in the background and after confirmation, the Batch has its status changed to Rejected. You receive in the response of this request the ID of the created command and can consult it at any time.

  ## Webhook Events

  When the request is made, a `payment.payment_batch.reprove.requested` event is triggered.
  Our server then begins attempts to send the release to the financial institution.
  Only after confirmation from the financial institution is the batch marked as approved in our database.
  At this moment the `payment.payment_batch.reprove.confirmed` event is triggered.

  > ⚠️ **Attention to Rejected Status:** The Rejected status is terminal. When the financial institution cancels the payments, they will have their status changed to Rejected

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `404` | Payment Batch Not Found |
| `422` | Transfer Batch is not awaiting release |

## payment dda-accounts list

List DDA Accounts

```text
kobana banking payment dda-accounts list [OPTIONS]
```

`GET /v2/payment/dda_accounts`

Lists all DDA accounts registered in the account. Supports pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Empty list when there are no DDA accounts |

## payment dda-accounts create

Create a DDA Account

```text
kobana banking payment dda-accounts create [OPTIONS]
```

`POST /v2/payment/dda_accounts`

Creates a new DDA account.

**Webhook Events**

The `payment.dda.account.db.created` event is triggered in this operation.

After creation, the system automatically generates the DDA letter (PDF), triggering the `payment.dda.account.letter.pending`, `payment.dda.account.letter.waiting_signatures`, and `payment.dda.account.letter.signed` events as the signature flow progresses.

After Manager approval, the `payment.dda.account.confirmed` or `payment.dda.account.rejected` events are triggered.

After `confirmed`, the system initiates the account registration at the bank asynchronously, triggering `payment.dda.account.register.requested` and `payment.dda.account.register.confirmed` (or `failed`/`error`) according to the provider's response.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `cnpj` | `string` | yes | CNPJ or CPF of the company/person for which the DDA will be enabled. Must be a valid document. Each Kobana account can have only one active DDA account. |
| `signers` | `array<object>` | no | List of 1 to 3 signatories of the DDA membership letter. Each signatory will receive an email with a link for digital signature. |
| `business_legal_name` | `string` | no | Legal name of the company associated with the CNPJ of the DDA account. Optional field. |
| `phone_number` | `string` | no | Contact phone of the company associated with the CNPJ of the DDA account. Optional field. |
| `address` | `object` | no | Address of the company associated with the CNPJ of the DDA account. Optional field; all subfields are also optional. |

More fields and nested objects: `kobana schema banking.payment.dda-accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | DDA Account created successfully |
| `422` | Address subfield with non-scalar value (array) |

## payment dda-accounts get

Fetch DDA Account

```text
kobana banking payment dda-accounts get [UID] [OPTIONS]
```

`GET /v2/payment/dda_accounts/{uid}`

Returns the details of a specific DDA account by UID.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | DDA Account Found |
| `404` | DDA Account Not Found |

## payment dda-accounts update

Update DDA Account

```text
kobana banking payment dda-accounts update [UID] [OPTIONS]
```

`PUT /v2/payment/dda_accounts/{uid}`

Updates the signatories of a DDA account. Only allowed when the account is editable (editable=true).

**Webhook Events**

The `payment.dda.account.db.updated` event is triggered in this operation.

If the account had a letter in `external_letter_status: waiting_signatures`, the system synchronizes the new signatories with the signature provider asynchronously. If the provider restarts the signature flow, the `payment.dda.account.letter.pending` and subsequent events will be triggered.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `signers` | `array<object>` | no | List of 1 to 3 signatories of the DDA membership letter. Each signatory will receive an email with a link for digital signature. |
| `business_legal_name` | `string` | no | Legal name of the company associated with the CNPJ of the DDA account. Optional field. |
| `phone_number` | `string` | no | Contact phone of the company associated with the CNPJ of the DDA account. Optional field. |
| `address` | `object` | no | Address of the company associated with the CNPJ of the DDA account. Optional field; all subfields are also optional. Replaces the entire previously saved hash — does not merge partially. |

More fields and nested objects: `kobana schema banking.payment.dda-accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | DDA Account updated successfully |
| `422` | Account not editable |

## payment dda-accounts disable

Disable DDA Account

```text
kobana banking payment dda-accounts disable [UID] [OPTIONS]
```

`PUT /v2/payment/dda_accounts/{uid}/disable`

Requests disabling of an existing DDA account. Does not remove `enabled_at` directly.

**Webhook Events**

The `payment.dda.account.db.updated` event is triggered in this operation, reflecting the change in `disable_requested_at`. The disabling itself is only confirmed asynchronously, when the provider responds: `payment.dda.account.disabled` (success) or `payment.dda.account.failed` (failure).

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | DDA Account disabled successfully |
| `404` | DDA Account Not Found |

## payment dda-accounts enable

Enable DDA Account

```text
kobana banking payment dda-accounts enable [UID] [OPTIONS]
```

`PUT /v2/payment/dda_accounts/{uid}/enable`

Requests enabling of a previously disabled DDA account. Subject to a 21-minute cooldown period after disabling.

**Webhook Events**

The `payment.dda.account.db.updated` event is triggered in this operation, reflecting the change in `enable_requested_at`. The enabling itself is only confirmed asynchronously, when the provider responds: `payment.dda.account.enabled` (success) or `payment.dda.account.failed` (failure/incomplete provider response).

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | DDA Account enabled successfully |
| `404` | DDA Account Not Found |
| `422` | DDA Account cannot be enabled without signed letter and approved status |

## payment dda-bank-billets list

List DDA Bank Billets

```text
kobana banking payment dda-bank-billets list [OPTIONS]
```

`GET /v2/payment/dda_bank_billets`

Lists all DDA bank billets linked to the account. Supports pagination.

**Webhook Events**

DDA bank billets arrive exclusively via automatic synchronization (there is no manual creation). The related webhook events are:

- `payment.dda.bank_billet.db.created` — triggered when a new billet is synchronized from the provider
- `payment.dda.bank_billet.external_status_changed` — triggered when the external status of the billet changes (ex: `open` → `paid`)

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Empty list when there are no DDA bank billets |

## payment dda-bank-billets get

Fetch DDA Bank Billet

```text
kobana banking payment dda-bank-billets get [UID] [OPTIONS]
```

`GET /v2/payment/dda_bank_billets/{uid}`

Returns the details of a specific DDA bank billet by UID.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | DDA Bank Billet Found |
| `404` | DDA Bank Billet Not Found |

## payment dda-bank-billets delete

Delete DDA Bank Billet

```text
kobana banking payment dda-bank-billets delete [UID] [OPTIONS]
```

`DELETE /v2/payment/dda_bank_billets/{uid}`

Deletes (soft delete) an existing DDA bank billet.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | DDA Bank Billet deleted successfully |
| `404` | DDA Bank Billet Not Found |

## payment dda-bank-billets reject

Reject DDA Bank Billet

```text
kobana banking payment dda-bank-billets reject [UID] [OPTIONS]
```

`PUT /v2/payment/dda_bank_billets/{uid}/reject`

Rejects a DDA bank billet with `pending` status. Rejected billets will not be processed for payment.

**Webhook Events**

The `payment.dda.bank_billet.db.updated` event is triggered in this operation with the status changes (`pending` → `rejected`) and the `rejected_at` field. This event is triggered on any DDA bank billet update, not just this operation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | DDA Bank Billet rejected successfully |
| `404` | DDA Bank Billet Not Found |
| `422` | Invalid state transition |

## payment dda-bank-billets release

Release DDA Bank Billet for Payment

```text
kobana banking payment dda-bank-billets release [UID] [OPTIONS]
```

`PUT /v2/payment/dda_bank_billets/{uid}/release`

Releases a DDA bank billet with `pending` status for payment. Only billets with `external_status: open` can be released.

**Webhook Events**

The `payment.dda.bank_billet.db.updated` event is triggered in this operation with the status changes (`pending` → `released`) and the `released_at` field. This event is triggered on any DDA bank billet update, not just this operation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | DDA Bank Billet released for payment successfully |
| `404` | DDA Bank Billet Not Found |
| `422` | Invalid state transition |

## payment pix list

List PIX Payments

```text
kobana banking payment pix list [OPTIONS]
```

`GET /v2/payment/pix`

Lists all payments made via PIX. Supports pagination and returns detailed information about each payment.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payments Found |

## payment pix create

Create a PIX Payment

```text
kobana banking payment pix create [OPTIONS]
```

`POST /v2/payment/pix`

This endpoint is used to generate a payment.

**Prerequisite**

To issue a payment, you must have a Financial Account, specified in `financial_account_uid`. The payment account that records the operation is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Webhook Events**

The `payment.db.created` event is triggered in this operation.

> **Next steps:** After creating a payment, you may want to send a Batch to the financial institution at some point. To do this, use the `POST /api/v2/payment/pix_batches` endpoint.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `qrcode` | `string` | yes | QR Code Copy and Paste |
| `amount` | `number` | no | Amount. Optional, the value will be extracted automatically from `qrcode`. |
| `scheduled_to` | `string (date)` | no | Scheduling Date |
| `payment_account_uid` | `string (uuid)` | no | Payment account of origin (optional). If omitted, the oldest from the financial account is used. If provided, must belong to `financial_account_uid`. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `transfer_purpose` | `string` | no | Payment purpose. Defines the batch service type in the remittance file sent to the bank. Optional. When not provided, the remittance is sent as `98` (Miscellaneous Payments). Provide it only if the bank requires a specific batch type: Itaú, for example, rejects remittances whose batch type the company has not contracted. Payments with different purposes are sent in separate remittances, because the service type describes the whole batch. For payments, only Itaú currently reads this field in the remittance. At the other banks the batch type is fixed: the field is accepted and returned in the response, but does not change the file sent. At Itaú, the Pix QR-Code batch uses the fixed service type `20`: the provided value is accepted and returned, but does not change the remittance. `01`: Collection. `03`: Electronic Payment Boleto. `04`: Bank Reconciliation. `05`: Debits. `06`: Check Custody. `07`: Cash Management. `08`: Margin Inquiry/Information. `09`: Payroll Loan Registration/Withholding. `10`: Dividend Payment. `11`: Payroll Loan Maintenance. `12`: Payroll Loan Installments. `13`: Payroll Loan Disallowance (INSS). `14`: Payable Taxes Inquiry. `20`: Supplier Payment. `22`: Bills, Taxes and Duties Payment. `23`: Interoperability between Payment Institution Accounts. `25`: Compror. `26`: Revolving Compror. `29`: Payer Allegation. `30`: Salary Payment. `32`: Fee Payment. `33`: Stipend Payment. `34`: Prebend Payment (remuneration to priests and clergy). `40`: Vendor. `41`: Term Vendor. `50`: Insured Claims Payment. `60`: Traveler in Transit Expenses Payment. `70`: Authorized Payment. `75`: Accredited Payment. `77`: Remuneration Payment. `80`: Authorized Representatives / Sellers Payment. `90`: Benefits Payment. `98`: Miscellaneous Payments Values: `01`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `20`, `22`, `23`, `25`, `26`, `29`, `30`, `32`, `33`, `34`, `40`, `41`, `50`, `60`, `70`, `75`, `77`, `80`, `90`, `98`. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |
| `identifier` | `string` | no | Payment identification on the receipt from the financial provider. (Bank exclusive use) Banks that accept this field (via API): Itaú |
| `beneficiary` | `object` | no | Beneficiary data. |

More fields and nested objects: `kobana schema banking.payment.pix.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment Created |
| `422` | Validation error |

## payment pix get

View a PIX Payment

```text
kobana banking payment pix get [UID] [OPTIONS]
```

`GET /v2/payment/pix/{uid}`

Queries specific information about a PIX Payment.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment Found |
| `404` | Payment Not Found |

## payment pix cancel

Cancel a PIX Payment

```text
kobana banking payment pix cancel [UID] [OPTIONS]
```

`PUT /v2/payment/pix/{uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the Payment.
The cancellation of a Payment is done asynchronously, that is, upon receiving the request, our API saves the request and creates a Command. This command is processed in the background and after confirmation, the Payment has its status changed to Cancelled. You receive in the response of this request the ID of the created command and can consult it at any time.

**Webhook Events**

When the request is made, a `payment.pix.cancel.requested` event is triggered.
Our server then begins attempts to send the cancellation to the financial institution.
Only after confirmation from the financial institution is the payment marked as cancelled in our database.
At this moment the `payment.pix.cancel.confirmed` event is triggered.

> ⚠️ **Attention to Cancelled Status:** The Cancelled status is terminal. The payment cannot be processed further after cancellation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment cancelled without provider interaction |
| `422` | Payment has already been confirmed, rejected, or canceled |

## payment pix-batches create

Create a PIX Payment Batch

```text
kobana banking payment pix-batches create [OPTIONS]
```

`POST /v2/payment/pix_batches`

This endpoint is used to generate a payment batch.

**Prerequisite**

To issue a payment batch, you must have a Financial Account, specified in `financial_account_uid`. The payment account applied to the batch is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Webhook Events**

The `payment.batch.db.created` event is triggered in this operation.

When registering a Payment Batch, the system creates a command to register it in the financial institution. After the batch registration is confirmed in the institution, the `payment.pix_batch.confirmed` event is triggered.

The `payment.db.created` event is also triggered in this operation if the batch contains a new payment.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `payments` | `array<anyOf>` | yes | List of New or Existing Pix Payments. |
| `payment_account_uid` | `string (uuid)` | no | Payment account applied to the entire batch (optional). If omitted, the oldest from the financial account is used. If provided, must belong to the batch's financial account. |

More fields and nested objects: `kobana schema banking.payment.pix-batches.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment Batch Created |
| `422` | Validation error — duplicate external_id |

## payment tax-batches create

Create a Tax Payment Batch

```text
kobana banking payment tax-batches create [OPTIONS]
```

`POST /v2/payment/tax_batches`

This endpoint is used to generate a batch of municipal, state, and federal tax payments.

> ⚠️ **New Feature Under Development!** This API is in beta version - this means we are constantly improving and adjusting its functionality. Although it is fully functional, some things may change as we receive feedback from developers.
>
> For questions about this feature, please contact us.

**Prerequisite**

To issue a payment batch, you must have a Financial Account, specified in `financial_account_uid`. The payment account applied to the batch is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Webhook Events**

The `payment.batch.db.created` event is triggered in this operation.

When registering a Payment Batch, the system creates a command to register it in the financial institution. After the batch registration is confirmed in the institution, the `payment.tax_batch.confirmed` event is triggered.

The `payment.db.created` event is also triggered in this operation if the batch contains a new payment.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `payments` | `array<anyOf>` | yes | List of New or Existing Tax Payments. |
| `payment_account_uid` | `string (uuid)` | no | Payment account applied to the entire batch (optional). If omitted, the oldest from the financial account is used. If provided, must belong to the batch's financial account. |

More fields and nested objects: `kobana schema banking.payment.tax-batches.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Batch created with format=data (GRU) |
| `422` | Validation error — duplicate external_id |

## payment taxes list

List Tax Payments

```text
kobana banking payment taxes list [OPTIONS]
```

`GET /v2/payment/taxes`

This endpoint is used to generate a municipal, state, or federal tax payment.

> ⚠️ **New Feature Under Development!** This API is in beta version - this means we are constantly improving and adjusting its functionality. Although it is fully functional, some things may change as we receive feedback from developers.
>
> For questions about this feature, please contact us.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payments Found |

## payment taxes create

Create a Tax Payment

```text
kobana banking payment taxes create [OPTIONS]
```

`POST /v2/payment/taxes`

**Prerequisites**

To issue a payment, you must have a Financial Account, specified in `financial_account_uid`. The payment account that records the operation is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount |
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `kind` | `string` | yes | Type of Tax Values: `itbi`, `icms`, `iss`, `iptu`, `fgts`, `dare`, `darf`, `gru`, `gps`. |
| `scheduled_to` | `string (date)` | no | Scheduling Date |
| `payment_account_uid` | `string (uuid)` | no | Payment account of origin (optional). If omitted, the oldest from the financial account is used. If provided, must belong to `financial_account_uid`. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `transfer_purpose` | `string` | no | Payment purpose. Defines the batch service type in the remittance file sent to the bank. Optional. When not provided, the remittance is sent as `98` (Miscellaneous Payments). Provide it only if the bank requires a specific batch type: Itaú, for example, rejects remittances whose batch type the company has not contracted. Payments with different purposes are sent in separate remittances, because the service type describes the whole batch. For payments, only Itaú currently reads this field in the remittance. At the other banks the batch type is fixed: the field is accepted and returned in the response, but does not change the file sent. When `format` is `code`, only `20` and `98` are accepted: the tax with a barcode travels in the same batch as utility bills. `01`: Collection. `03`: Electronic Payment Boleto. `04`: Bank Reconciliation. `05`: Debits. `06`: Check Custody. `07`: Cash Management. `08`: Margin Inquiry/Information. `09`: Payroll Loan Registration/Withholding. `10`: Dividend Payment. `11`: Payroll Loan Maintenance. `12`: Payroll Loan Installments. `13`: Payroll Loan Disallowance (INSS). `14`: Payable Taxes Inquiry. `20`: Supplier Payment. `22`: Bills, Taxes and Duties Payment. `23`: Interoperability between Payment Institution Accounts. `25`: Compror. `26`: Revolving Compror. `29`: Payer Allegation. `30`: Salary Payment. `32`: Fee Payment. `33`: Stipend Payment. `34`: Prebend Payment (remuneration to priests and clergy). `40`: Vendor. `41`: Term Vendor. `50`: Insured Claims Payment. `60`: Traveler in Transit Expenses Payment. `70`: Authorized Payment. `75`: Accredited Payment. `77`: Remuneration Payment. `80`: Authorized Representatives / Sellers Payment. `90`: Benefits Payment. `98`: Miscellaneous Payments Values: `01`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `20`, `22`, `23`, `25`, `26`, `29`, `30`, `32`, `33`, `34`, `40`, `41`, `50`, `60`, `70`, `75`, `77`, `80`, `90`, `98`. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |
| `format` | `string` | no | Tax format. `code`: Provide barcode/typable line (default). `data`: Provide structured fields (available for kinds: darf, gru, gps, fgts). Values: `code`, `data`. |
| `code` | `string` | no | Barcode or Typable Line of the Tax. Required when format=code. |
| `data` | `object` | no | Structured fields of the tax. Required when format=data. |

More fields and nested objects: `kobana schema banking.payment.taxes.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment created with format=data (DARF) |
| `422` | Invalid parameters |

## payment taxes get

Display Tax Payment

```text
kobana banking payment taxes get [UID] [OPTIONS]
```

`GET /v2/payment/taxes/{uid}`

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment Found |
| `404` | Payment Not Found |

## payment taxes cancel

Cancel a Tax Payment

```text
kobana banking payment taxes cancel [UID] [OPTIONS]
```

`PUT /v2/payment/taxes/{uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the Payment.
The cancellation of a Payment is done asynchronously, that is, upon receiving the request, our API saves the request and creates a Command. This command is processed in the background and after confirmation, the Payment has its status changed to Cancelled. You receive in the response of this request the ID of the created command and can consult it at any time.

**Webhook Events**

When the request is made, a `payment.tax.cancel.requested` event is triggered.
Our server then begins attempts to send the cancellation to the financial institution.
Only after confirmation from the financial institution is the payment marked as cancelled in our database.
At this moment the `payment.tax.cancel.confirmed` event is triggered.

> ⚠️ **Attention to Cancelled Status:** The Cancelled status is terminal. The payment cannot be processed further after cancellation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment cancelled without provider interaction |
| `422` | Payment has already been confirmed, rejected, or canceled |

## payment utilities list

List Utility Payments

```text
kobana banking payment utilities list [OPTIONS]
```

`GET /v2/payment/utilities`

Lists all utility payments (water, electricity, gas, telephone, etc.) made. Supports pagination and returns detailed information.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payments Found |

## payment utilities create

Create a Utility Payment

```text
kobana banking payment utilities create [OPTIONS]
```

`POST /v2/payment/utilities`

This endpoint is used to generate a payment.

**Prerequisite**

To issue a payment, you must have a Financial Account, specified in `financial_account_uid`. The payment account that records the operation is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Webhook Events**

The `payment.db.created` event is triggered in this operation.

> **Next steps:** After creating a payment, you may want to send a Batch to the financial institution at some point. To do this, use the `POST /api/v2/payment/utility_batches` endpoint.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount |
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `code` | `string` | yes | Barcode or Typable Line |
| `scheduled_to` | `string (date)` | no | Scheduling Date |
| `payment_account_uid` | `string (uuid)` | no | Payment account of origin (optional). If omitted, the oldest from the financial account is used. If provided, must belong to `financial_account_uid`. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |
| `transfer_purpose` | `string` | no | Payment purpose. Defines the batch service type in the remittance file sent to the bank. Optional. When not provided, the remittance is sent as `98` (Miscellaneous Payments). Provide it only if the bank requires a specific batch type: Itaú, for example, rejects remittances whose batch type the company has not contracted. Payments with different purposes are sent in separate remittances, because the service type describes the whole batch. For payments, only Itaú currently reads this field in the remittance. At the other banks the batch type is fixed: the field is accepted and returned in the response, but does not change the file sent. `20`: Supplier Payment. `98`: Miscellaneous Payments Values: `20`, `98`. |

More fields and nested objects: `kobana schema banking.payment.utilities.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment Created |
| `422` | Validation error |

## payment utilities get

View a Utility Payment

```text
kobana banking payment utilities get [UID] [OPTIONS]
```

`GET /v2/payment/utilities/{uid}`

Queries specific information about a Utility Payment.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment Found |
| `404` | Payment Not Found |

## payment utilities cancel

Cancel a Utility Payment

```text
kobana banking payment utilities cancel [UID] [OPTIONS]
```

`PUT /v2/payment/utilities/{uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the Payment.
The cancellation of a Payment is done asynchronously, that is, when receiving the request, our API stores the request and creates a Command. This command is processed in the background and after being confirmed, the Payment has its status changed to Canceled. You receive the ID of the created command in the response of this request and can consult it at any time.

**Webhook Events**

When the request is made, a `payment.utility.cancel.requested` event is triggered.
Our server then starts attempts to send the cancellation to the financial institution.
Only after confirmed at the financial institution, then the payment is marked as canceled in our database.
At this moment the `payment.utility.cancel.confirmed` event is triggered.

> ⚠️ **Attention to Canceled status:** The Canceled status is terminal. The payment can no longer be processed after cancellation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment cancelled without provider interaction |
| `422` | Payment has already been confirmed, rejected, or canceled |

## payment utility-batches create

Create a Batch of Utility Bill Payments

```text
kobana banking payment utility-batches create [OPTIONS]
```

`POST /v2/payment/utility_batches`

This endpoint is used for the generation of a payment batch.

**Prerequisite**

To issue a payment batch, you must have a Financial Account, informed in `financial_account_uid`. The payment account applied to the batch is defined in `payment_account_uid`; if omitted, the oldest linked to the financial account is used; if informed, it must belong to `financial_account_uid`.

**Webhook Events**

The `payment.batch.db.created` event is triggered in this operation.

When creating a Payment Batch, the system creates a command to register it with the financial institution. After the batch registration is confirmed at the institution, the `payment.utility_batch.confirmed` event is triggered.

The `payment.db.created` event is also triggered in this operation if the batch contains a new payment.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `payments` | `array<anyOf>` | yes | List of New or Existing Utility Account Payments. |
| `payment_account_uid` | `string (uuid)` | no | Payment account applied to the entire batch (optional). If omitted, the oldest from the financial account is used. If provided, must belong to the batch's financial account. |

More fields and nested objects: `kobana schema banking.payment.utility-batches.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment Batch Created |
| `422` | Validation error — duplicate external_id |
