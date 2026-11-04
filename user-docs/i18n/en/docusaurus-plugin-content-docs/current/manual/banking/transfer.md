---
id: transfer
title: "kobana banking transfer"
sidebar_label: "transfer"
description: "Transfers (Pix, TED, internal)"
---

# kobana banking transfer

Transfers (Pix, TED, internal)

Commands of `kobana banking transfer`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`transfer accounts list`](#transfer-accounts-list) | `GET /v2/transfer/accounts` | List Transfer Accounts |
| [`transfer accounts create`](#transfer-accounts-create) | `POST /v2/transfer/accounts` | Create Transfer Account |
| [`transfer accounts get`](#transfer-accounts-get) | `GET /v2/transfer/accounts/{uid}` | Fetch Transfer Account |
| [`transfer accounts update`](#transfer-accounts-update) | `PATCH /v2/transfer/accounts/{uid}` | Update Transfer Account |
| [`transfer batches list`](#transfer-batches-list) | `GET /v2/transfer/batches` | List Transfer Batches |
| [`transfer batches get`](#transfer-batches-get) | `GET /v2/transfer/batches/{uid}` | View a Transfer Batch |
| [`transfer batches approve`](#transfer-batches-approve) | `PUT /v2/transfer/batches/{uid}/approve` | Approve a Transfer Batch |
| [`transfer batches cancel`](#transfer-batches-cancel) | `PUT /v2/transfer/batches/{uid}/cancel` | Cancel a Transfer Batch |
| [`transfer batches reprove`](#transfer-batches-reprove) | `PUT /v2/transfer/batches/{uid}/reprove` | Reprove a Transfer Batch |
| [`transfer internal list`](#transfer-internal-list) | `GET /v2/transfer/internal` | List Inter-Account Transfers |
| [`transfer internal create`](#transfer-internal-create) | `POST /v2/transfer/internal` | Create an Inter-Account Transfer |
| [`transfer internal get`](#transfer-internal-get) | `GET /v2/transfer/internal/{uid}` | View an Inter-Account Transfer |
| [`transfer internal approve`](#transfer-internal-approve) | `PUT /v2/transfer/internal/{uid}/approve` | Release (approve) an internal Transfer |
| [`transfer internal cancel`](#transfer-internal-cancel) | `PUT /v2/transfer/internal/{uid}/cancel` | Cancel an Inter-Account Transfer |
| [`transfer internal reschedule`](#transfer-internal-reschedule) | `PUT /v2/transfer/internal/{uid}/reschedule` | Reschedule (change date) an internal Transfer |
| [`transfer internal-batches create`](#transfer-internal-batches-create) | `POST /v2/transfer/internal_batches` | Create an Inter-Account Transfer Batch |
| [`transfer pix list`](#transfer-pix-list) | `GET /v2/transfer/pix` | List Pix Transfers |
| [`transfer pix create`](#transfer-pix-create) | `POST /v2/transfer/pix` | Create a Pix Transfer |
| [`transfer pix get`](#transfer-pix-get) | `GET /v2/transfer/pix/{uid}` | View a Pix Transfer |
| [`transfer pix approve`](#transfer-pix-approve) | `PUT /v2/transfer/pix/{uid}/approve` | Release (approve) a Pix Transfer |
| [`transfer pix cancel`](#transfer-pix-cancel) | `PUT /v2/transfer/pix/{uid}/cancel` | Cancel a Pix Transfer |
| [`transfer pix reschedule`](#transfer-pix-reschedule) | `PUT /v2/transfer/pix/{uid}/reschedule` | Reschedule (change date) a Pix Transfer |
| [`transfer pix-batches create`](#transfer-pix-batches-create) | `POST /v2/transfer/pix_batches` | Create a Pix Transfer Batch |
| [`transfer ted list`](#transfer-ted-list) | `GET /v2/transfer/ted` | List TED Transfers |
| [`transfer ted create`](#transfer-ted-create) | `POST /v2/transfer/ted` | Create a TED Transfer |
| [`transfer ted get`](#transfer-ted-get) | `GET /v2/transfer/ted/{uid}` | View a TED Transfer |
| [`transfer ted approve`](#transfer-ted-approve) | `PUT /v2/transfer/ted/{uid}/approve` | Release (Approve) a TED Transfer |
| [`transfer ted cancel`](#transfer-ted-cancel) | `PUT /v2/transfer/ted/{uid}/cancel` | Cancel a TED Transfer |
| [`transfer ted reschedule`](#transfer-ted-reschedule) | `PUT /v2/transfer/ted/{uid}/reschedule` | Reschedule (Change Date) a TED Transfer |
| [`transfer ted-batches create`](#transfer-ted-batches-create) | `POST /v2/transfer/ted_batches` | Create a TED Transfer Batch |

## transfer accounts list

List Transfer Accounts

```text
kobana banking transfer accounts list [OPTIONS]
```

`GET /v2/transfer/accounts`

Lists all transfer accounts registered in the account. Supports pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `financial_account_uid` | `string` | no | Filter by financial account UID |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Transfer Accounts accessed with read scope |

## transfer accounts create

Create Transfer Account

```text
kobana banking transfer accounts create [OPTIONS]
```

`POST /v2/transfer/accounts`

Creates a new transfer account associated with a financial account. Use when the financial account needs more than one transfer account. The account is created without a bank connection (`connection_uid` returns `null`); associate a connection before enabling remittances/registration. Requires `write` scope + `transfer.accounts`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string` | yes | UUID of the Financial Account that will receive the new transfer account |
| `agreement_code` | `string` | no | Agreement code |
| `external_id` | `string` | no | External identifier of the transfer account |
| `remittance_enabled` | `boolean` | no | Enables or disables generation of EDI remittances for this account. |
| `provider_data` | `object` | no | Provider-specific attributes. |

More fields and nested objects: `kobana schema banking.transfer.accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Transfer Account Created |
| `404` | Financial Account not found |
| `422` | Validation error. `agreement_code` above the limit. |

## transfer accounts get

Fetch Transfer Account

```text
kobana banking transfer accounts get [UID] [OPTIONS]
```

`GET /v2/transfer/accounts/{uid}`

Returns the details of a specific transfer account by UID

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Transfer Account accessed with read scope |
| `404` | Transfer Account Not Found |

## transfer accounts update

Update Transfer Account

```text
kobana banking transfer accounts update [UID] [OPTIONS]
```

`PATCH /v2/transfer/accounts/{uid}`

Updates editable attributes of a transfer account. Accepts `remittance_enabled` (boolean, controls EDI remittance generation), `provider_data.service_point_number` (string, service point — for example `numeroPa` in Sicoob TED) and `provider_data.cooperative_cnpj` (string, Sicoob cooperative CNPJ). The body accepts partial PATCH: send only the fields you want to update.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `uid` | `string` | no | UUID of the transfer account |
| `financial_account_uid` | `string` | no | UUID of associated financial account |
| `connection_uid` | `string (uuid)` | no | Banking connection of the account. |
| `agreement_code` | `string` | no | Agreement code |
| `provider_data` | `object` | no | Provider-specific data. For Sicoob, exposes `service_point_number` and `cooperative_cnpj`. |
| `external_id` | `string` | no | External identifier of the transfer account |
| `balance_validation_enabled_at` | `string (date-time)` | no | Date and time when balance validation was enabled |
| `remittance_enabled_at` | `string (date-time)` | no | Date and time when EDI remittance was enabled |
| `remittance_enabled` | `boolean` | no | Enables or disables generation of EDI remittances for this account. |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.transfer.accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Partial PATCH — only service_point_number (without remittance_enabled) |
| `404` | Transfer Account Not Found |
| `422` | Validation error. `remittance_enabled` must be true or false. |

## transfer batches list

List Transfer Batches

```text
kobana banking transfer batches list [OPTIONS]
```

`GET /v2/transfer/batches`

List Transfer Batches.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing batches |

## transfer batches get

View a Transfer Batch

```text
kobana banking transfer batches get [UID] [OPTIONS]
```

`GET /v2/transfer/batches/{uid}`

Query specific information about a Transfer Batch.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Transfer Batch found |
| `404` | Transfer Batch not found |

## transfer batches approve

Approve a Transfer Batch

```text
kobana banking transfer batches approve [UID] [OPTIONS]
```

`PUT /v2/transfer/batches/{uid}/approve`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates an approval command for the Transfer Batch.
  The approval of a Transfer Batch is done asynchronously, that is, when receiving the request, our API stores the request and creates a Command. This command is processed in the background and after confirmed, the Batch has its status changed to Approved. You receive in the response of this request the ID of the created command and can query it at any time.

  ## Webhook Events

  When the request is made, a `transfer.transfer_batch.approve.requested` event is triggered.
  Our server then begins sending attempts to release the transfer to the financial institution.
  Only after confirmed at the financial institution, the batch is then marked as approved in our database.
  At this point the `transfer.transfer_batch.approve.confirmed` event is triggered.

  > ⚠️ **Attention to Approved status:** The Approved status is not terminal. When the financial institution confirms the transfers, they will have their status changed to Confirmed

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `404` | Transfer Batch not found |
| `422` | Transfer Batch is not awaiting release |

## transfer batches cancel

Cancel a Transfer Batch

```text
kobana banking transfer batches cancel [UID] [OPTIONS]
```

`PUT /v2/transfer/batches/{uid}/cancel`

> **Cancellation:** This endpoint cancels the Transfer Batch. The behavior depends on the registration state of the batch.

  - **Not registered with provider:** The batch is cancelled immediately in a local manner, without interaction with the financial institution. The response contains the updated batch.
  - **Registered with provider (cancellation API supported):** A cancellation command is created and processed asynchronously. The response contains the created Command.
  - **Registered with provider (cancellation API not supported):** A cancellation command is created per Transfer individually. The response contains the updated batch.

  Cancellation is possible in the states: `pending`, `awaiting_approval`, `confirmed` and `approved`.

  > ⚠️ **Attention to Cancelled status:** The Cancelled status is terminal.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Cancelled successfully |
| `422` | Transfer Batch cannot be cancelled in the current state |

## transfer batches reprove

Reprove a Transfer Batch

```text
kobana banking transfer batches reprove [UID] [OPTIONS]
```

`PUT /v2/transfer/batches/{uid}/reprove`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a repproval command for the Transfer Batch.
  The reproval of a Transfer Batch is done asynchronously, that is, when receiving the request, our API stores the request and creates a Command. This command is processed in the background and after confirmed, the Batch has its status changed to Reproved. You receive in the response of this request the ID of the created command and can query it at any time.

  ## Webhook Events

  When the request is made, a `transfer.transfer_batch.reprove.requested` event is triggered.
  Our server then begins sending attempts to release the transfer to the financial institution.
  Only after confirmed at the financial institution, the batch is then marked as approved in our database.
  At this point the `transfer.transfer_batch.reprove.confirmed` event is triggered.

  > ⚠️ **Attention to Reproved status:** The Reproved status is terminal. When the financial institution cancels the transfers, they will have their status changed to Reproved

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `404` | Transfer Batch not found |
| `422` | Transfer Batch is not awaiting release |

## transfer internal list

List Inter-Account Transfers

```text
kobana banking transfer internal list [OPTIONS]
```

`GET /v2/transfer/internal`

List Inter-Account Transfers.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing transfers |

## transfer internal create

Create an Inter-Account Transfer

```text
kobana banking transfer internal create [OPTIONS]
```

`POST /v2/transfer/internal`

This endpoint is used to generate an Inter-Account transfer.

**Prerequisite**

To issue a transfer it is necessary to have a Financial Account, specified in `financial_account_uid`. The transfer account that registers the operation is defined in `transfer_account_uid`; if omitted, the oldest one linked to the financial account is used; if provided, it must belong to `financial_account_uid`.

**Webhook Events**

The `transfer.db.created` event is triggered in this operation.

> **Next steps:** After creating a transfer, you will want at some point to send a Batch to the financial institution. For this, use the `POST /api/v2/transfer/internal_batches` endpoint.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount |
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `beneficiary` | `object` | yes | Payee data |
| `internal` | `object` | yes | Bank account data |
| `scheduled_to` | `string (date)` | no | Scheduling Date |
| `transfer_purpose` | `string` | no | Transfer Objective |
| `transfer_account_uid` | `string (uuid)` | no | Origin transfer account (optional). If omitted, the oldest one from the financial account is used. If provided, must belong to `financial_account_uid`. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |

More fields and nested objects: `kobana schema banking.transfer.internal.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Inter-Account Transfer created |
| `422` | Validation error |

## transfer internal get

View an Inter-Account Transfer

```text
kobana banking transfer internal get [UID] [OPTIONS]
```

`GET /v2/transfer/internal/{uid}`

Query specific information about an Inter-Account Transfer.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Inter-Account Transfer found |
| `404` | Inter-Account Transfer not found |

## transfer internal approve

Release (approve) an internal Transfer

```text
kobana banking transfer internal approve [UID] [OPTIONS]
```

`PUT /v2/transfer/internal/{uid}/approve`

> **Asynchronous Processing:** creates a release (approve) command for the Transfer awaiting approval.

> ⚠️ **Provider support:** no financial provider currently supports internal transfer release via API — the call responds `422` ("does not have support for this operation via API for this financial provider"). The endpoint exists for symmetry and will automatically enable support when a provider implements it.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `422` | Provider without API release support |

## transfer internal cancel

Cancel an Inter-Account Transfer

```text
kobana banking transfer internal cancel [UID] [OPTIONS]
```

`PUT /v2/transfer/internal/{uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the Transfer.
The cancellation of a Transfer is done asynchronously, that is, when receiving the request, our API stores the request and creates a Command. This command is processed in the background and after confirmed, the Transfer has its status changed to Cancelled. You receive in the response of this request the ID of the created command and can query it at any time.

**Webhook Events**

When the request is made, a `transfer.internal.cancel.requested` event is triggered.
Our server then begins attempts to send the cancellation to the financial institution.
Only after confirmed at the financial institution, the transfer is then marked as cancelled in our database.
At this point the `transfer.internal.cancel.confirmed` event is triggered.

> ⚠️ **Attention to Cancelled status:** The Cancelled status is terminal. The transfer can no longer be processed after cancellation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `422` | Transfer has already been confirmed, rejected, or canceled |

## transfer internal reschedule

Reschedule (change date) an internal Transfer

```text
kobana banking transfer internal reschedule [UID] [OPTIONS]
```

`PUT /v2/transfer/internal/{uid}/reschedule`

> **Asynchronous Processing:** creates a rescheduling command (change of payment date) for the Transfer.

> ⚠️ **Provider support:** no financial provider currently supports internal transfer rescheduling via API — the call responds `422` ("does not have support for this operation via API for this financial provider"). The endpoint exists for symmetry and will automatically enable support when a provider implements it.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `date` | `string (date)` | yes | New payment date (YYYY-MM-DD). |

More fields and nested objects: `kobana schema banking.transfer.internal.reschedule`.

**Responses**

| HTTP | Description |
|---|---|
| `422` | Provider without API rescheduling support |

## transfer internal-batches create

Create an Inter-Account Transfer Batch

```text
kobana banking transfer internal-batches create [OPTIONS]
```

`POST /v2/transfer/internal_batches`

This endpoint is used to generate an Inter-Account transfer batch.

**Prerequisite**

To issue a transfer batch it is necessary to have a Financial Account, specified in `financial_account_uid`. The transfer account applied to the batch is defined in `transfer_account_uid`; if omitted, the oldest one linked to the financial account is used; if provided, it must belong to `financial_account_uid`.

**Webhook Events**

The `transfer.batch.db.created` event is triggered in this operation.

When registering a Transfer Batch, the system creates a command to register it with the financial institution. After the batch registration is confirmed at the institution, the `transfer.internal_batch.confirmed` event is triggered.

The `transfer.db.created` event is also triggered in this operation if the batch contains a new transfer.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `transfers` | `array<anyOf>` | yes | List of new or existing Transfers Between Accounts. |
| `transfer_account_uid` | `string (uuid)` | no | Transfer account applied to the entire batch (optional). If omitted, the oldest one from the financial account is used. If provided, must belong to the batch's financial account. |

More fields and nested objects: `kobana schema banking.transfer.internal-batches.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Inter-Account Transfer Batch created |
| `422` | Validation error — duplicate external_id |

## transfer pix list

List Pix Transfers

```text
kobana banking transfer pix list [OPTIONS]
```

`GET /v2/transfer/pix`

List Pix Transfers.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing transfers |

## transfer pix create

Create a Pix Transfer

```text
kobana banking transfer pix create [OPTIONS]
```

`POST /v2/transfer/pix`

This endpoint is used to generate a Pix transfer.

**Prerequisite**

To issue a transfer it is necessary to have a Financial Account, specified in `financial_account_uid`. The transfer account that registers the operation is defined in `transfer_account_uid`; if omitted, the oldest one linked to the financial account is used; if provided, it must belong to `financial_account_uid`.

**Webhook Events**

The `transfer.db.created` event is triggered in this operation.

> **Next steps:** After creating a transfer, you will want at some point to send a Batch to the financial institution. For this, use the `POST /api/v2/transfer/pix_batches` endpoint.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount |
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `beneficiary` | `object` | yes | Payee data |
| `type` | `string` | yes | Pix type `key`: Key. `bank_account`: Bank account data |
| `scheduled_to` | `string (date)` | no | Scheduling Date |
| `transfer_purpose` | `string` | no | Transfer Objective |
| `transfer_account_uid` | `string (uuid)` | no | Origin transfer account (optional). If omitted, the oldest one from the financial account is used. If provided, must belong to `financial_account_uid`. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |
| `key_type` | `string` | no | Key Type (Required if the `type` field is `key`) `cpf`: CPF. `cnpj`: CNPJ. `random`: Random. `email`: Email. `phone`: Mobile phone |
| `key` | `string` | no | Pix Key (Required if the `type` field is `key`) |
| `bank_account` | `object` | no | Bank account data (Required if the `type` field is `bank_account`) |
| `identifier` | `string` | no | Payment identification on the receipt from the financial provider. (Bank exclusive use) Banks that accept this field (via API): Itaú |

More fields and nested objects: `kobana schema banking.transfer.pix.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Pix Transfer created |
| `422` | Validation error |

## transfer pix get

View a Pix Transfer

```text
kobana banking transfer pix get [UID] [OPTIONS]
```

`GET /v2/transfer/pix/{uid}`

Query specific information about a Pix Transfer.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix Transfer found |
| `404` | Pix Transfer not found |

## transfer pix approve

Release (approve) a Pix Transfer

```text
kobana banking transfer pix approve [UID] [OPTIONS]
```

`PUT /v2/transfer/pix/{uid}/approve`

> **Asynchronous Processing:** creates a release (approve) command for the Pix Transfer awaiting approval.

> ⚠️ **Provider support:** not every financial provider supports release via API. When not supported, the API responds `422` ("does not have support for this operation via API for this financial provider").

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `422` | Transfer cannot be released |

## transfer pix cancel

Cancel a Pix Transfer

```text
kobana banking transfer pix cancel [UID] [OPTIONS]
```

`PUT /v2/transfer/pix/{uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the Transfer.
The cancellation of a Transfer is done asynchronously, that is, when receiving the request, our API stores the request and creates a Command. This command is processed in the background and after confirmed, the Transfer has its status changed to Cancelled. You receive in the response of this request the ID of the created command and can query it at any time.

**Webhook Events**

When the request is made, a `transfer.pix.cancel.requested` event is triggered.
Our server then begins attempts to send the cancellation to the financial institution.
Only after confirmed at the financial institution, the transfer is then marked as cancelled in our database.
At this point the `transfer.pix.cancel.confirmed` event is triggered.

> ⚠️ **Attention to Cancelled status:** The Cancelled status is terminal. The transfer can no longer be processed after cancellation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `422` | Transfer has already been confirmed, rejected, or canceled |

## transfer pix reschedule

Reschedule (change date) a Pix Transfer

```text
kobana banking transfer pix reschedule [UID] [OPTIONS]
```

`PUT /v2/transfer/pix/{uid}/reschedule`

> **Asynchronous Processing:** creates a rescheduling command (change of payment date) for the Pix Transfer.

> ⚠️ **Provider support:** not every financial provider supports rescheduling via API. When not supported, the API responds `422` ("does not have support for this operation via API for this financial provider").

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `date` | `string (date)` | yes | New payment date (YYYY-MM-DD). |

More fields and nested objects: `kobana schema banking.transfer.pix.reschedule`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `422` | Transfer cannot be rescheduled |

## transfer pix-batches create

Create a Pix Transfer Batch

```text
kobana banking transfer pix-batches create [OPTIONS]
```

`POST /v2/transfer/pix_batches`

This endpoint is used to generate a pix transfer batch.

**Prerequisite**

To issue a transfer batch it is necessary to have a Financial Account, specified in `financial_account_uid`. The transfer account applied to the batch is defined in `transfer_account_uid`; if omitted, the oldest one linked to the financial account is used; if provided, it must belong to `financial_account_uid`.

**Webhook Events**

The `transfer.batch.db.created` event is triggered in this operation.

When registering a Transfer Batch, the system creates a command to register it with the financial institution. After the batch registration is confirmed at the institution, the `transfer.pix_batch.confirmed` event is triggered.

The `transfer.db.created` event is also triggered in this operation if the batch contains a new transfer.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `transfers` | `array<anyOf>` | yes | List of new or existing Pix Transfers. |
| `transfer_account_uid` | `string (uuid)` | no | Transfer account applied to the entire batch (optional). If omitted, the oldest one from the financial account is used. If provided, must belong to the batch's financial account. |

More fields and nested objects: `kobana schema banking.transfer.pix-batches.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Pix Transfer Batch created |
| `422` | Validation error — duplicate external_id |

## transfer ted list

List TED Transfers

```text
kobana banking transfer ted list [OPTIONS]
```

`GET /v2/transfer/ted`

List TED Transfers.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing transfers |

## transfer ted create

Create a TED Transfer

```text
kobana banking transfer ted create [OPTIONS]
```

`POST /v2/transfer/ted`

This endpoint is used to generate a TED transfer.

**Prerequisite**

To issue a transfer, you must have a Financial Account, specified in `financial_account_uid`. The transfer account that records the operation is defined in `transfer_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Webhook Events**

The `transfer.db.created` event is triggered in this operation.

> **Next steps:** After creating a transfer, you may want to send a Batch to the financial institution at some point. To do this, use the `POST /api/v2/transfer/ted_batches` endpoint.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount |
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `scheduled_to` | `string (date)` | no | Scheduling Date |
| `transfer_purpose` | `string` | no | Transfer Objective |
| `transfer_account_uid` | `string (uuid)` | no | Origin transfer account (optional). If omitted, the oldest one from the financial account is used. If provided, must belong to `financial_account_uid`. |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |
| `beneficiary` | `object` | no | Payee data |
| `bank_account` | `object` | no | Bank account data |
| `judicial_deposit_number` | `string` | no | Judicial deposit number (judicial deposit TED, Banco do Brasil only — mutually exclusive with `bank_account`) |
| `judicial_deposit_bank_code` | `string` | no | COMPE code of the judicial deposit's destination bank (required with `judicial_deposit_number`). Use `001` for a deposit at Banco do Brasil itself |

More fields and nested objects: `kobana schema banking.transfer.ted.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | TED Transfer Created |
| `422` | Validation error |

## transfer ted get

View a TED Transfer

```text
kobana banking transfer ted get [UID] [OPTIONS]
```

`GET /v2/transfer/ted/{uid}`

Queries specific information about a TED Transfer.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | TED Transfer Found |
| `404` | TED Transfer Not Found |

## transfer ted approve

Release (Approve) a TED Transfer

```text
kobana banking transfer ted approve [UID] [OPTIONS]
```

`PUT /v2/transfer/ted/{uid}/approve`

> **Asynchronous Processing:** creates a release (approve) command for the Transfer awaiting approval.

The release is processed asynchronously: the API saves the request, creates a Command, and responds with its ID. After confirmation from the financial institution, the Transfer is released.

> ⚠️ **Bank Support:** not all financial providers support release via API. When not supported, the API responds `422` with the message "does not have support for this operation via API for this financial provider".

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `422` | Transfer cannot be released |

## transfer ted cancel

Cancel a TED Transfer

```text
kobana banking transfer ted cancel [UID] [OPTIONS]
```

`PUT /v2/transfer/ted/{uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the Transfer.
The cancellation of a Transfer is done asynchronously, that is, upon receiving the request, our API saves the request and creates a Command. This command is processed in the background and after confirmation, the Transfer has its status changed to Cancelled. You receive in the response of this request the ID of the created command and can consult it at any time.

**Webhook Events**

When the request is made, a `transfer.ted.cancel.requested` event is triggered.
Our server then begins attempts to send the cancellation to the financial institution.
Only after confirmation from the financial institution is the transfer marked as cancelled in our database.
At this moment the `transfer.ted.cancel.confirmed` event is triggered.

> ⚠️ **Attention to Cancelled Status:** The Cancelled status is terminal. The transfer cannot be processed further after cancellation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `422` | Transfer has already been confirmed, rejected, or canceled |

## transfer ted reschedule

Reschedule (Change Date) a TED Transfer

```text
kobana banking transfer ted reschedule [UID] [OPTIONS]
```

`PUT /v2/transfer/ted/{uid}/reschedule`

> **Asynchronous Processing:** creates a rescheduling command (change of payment date) for the Transfer.

> ⚠️ **Bank Support:** not all financial providers support rescheduling via API. When not supported, the API responds `422` ("does not have support for this operation via API for this financial provider").

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `date` | `string (date)` | yes | New payment date (YYYY-MM-DD). |

More fields and nested objects: `kobana schema banking.transfer.ted.reschedule`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `422` | Transfer cannot be rescheduled |

## transfer ted-batches create

Create a TED Transfer Batch

```text
kobana banking transfer ted-batches create [OPTIONS]
```

`POST /v2/transfer/ted_batches`

This endpoint is used to generate a TED transfer batch.

**Prerequisite**

To issue a transfer batch, you must have a Financial Account, specified in `financial_account_uid`. The transfer account applied to the batch is defined in `transfer_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Webhook Events**

The `transfer.batch.db.created` event is triggered in this operation.

When registering a Transfer Batch, the system creates a command to register it in the financial institution. After the batch registration is confirmed in the institution, the `transfer.ted_batch.confirmed` event is triggered.

The `transfer.db.created` event is also triggered in this operation if the batch contains a new transfer.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | yes | UID of the Origin Financial Account |
| `transfers` | `array<anyOf>` | yes | List of new or existing TED Transfers. |
| `transfer_account_uid` | `string (uuid)` | no | Transfer account applied to the entire batch (optional). If omitted, the oldest one from the financial account is used. If provided, must belong to the batch's financial account. |

More fields and nested objects: `kobana schema banking.transfer.ted-batches.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | TED Transfer Batch Created |
| `422` | Validation error — duplicate external_id |
