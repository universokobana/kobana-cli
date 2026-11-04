---
id: bank-billet-payments
title: "kobana banking bank-billet-payments"
sidebar_label: "bank-billet-payments"
---

# kobana banking bank-billet-payments


Commands of `kobana banking bank-billet-payments`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`bank-billet-payments list`](#bank-billet-payments-list) | `GET /v1/bank_billet_payments` | List Billet Payments |
| [`bank-billet-payments create`](#bank-billet-payments-create) | `POST /v1/bank_billet_payments` | Make Billet Payment |
| [`bank-billet-payments get`](#bank-billet-payments-get) | `GET /v1/bank_billet_payments/{id}` | Billet Payment Information |
| [`bank-billet-payments delete`](#bank-billet-payments-delete) | `DELETE /v1/bank_billet_payments/{id}` | Delete Billet Payment |

## bank-billet-payments list

List Billet Payments

```text
kobana banking bank-billet-payments list [OPTIONS]
```

`GET /v1/bank_billet_payments`

List Billet Payments.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `paid_from` | `string` | no | From Payment Date. |
| `paid_to` | `string` | no | To Payment Date. |
| `bank_billet_id` | `string` | no | Billet ID. |
| `bank_billet_account_id` | `string` | no | Wallet ID. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing billet payments |

## bank-billet-payments create

Make Billet Payment

```text
kobana banking bank-billet-payments create [OPTIONS]
```

`POST /v1/bank_billet_payments`

Make Billet Payment.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `bank_billet_id` | `integer` | yes | Billet ID. |
| `bank_billet_account_id` | `integer` | yes | Wallet ID. |
| `paid_amount` | `number` | yes | Paid Amount |
| `paid_at` | `string (date)` | yes | Receipt Date. |
| `kind` | `integer` | yes | Type of Receipt: * `0` Direct to beneficiary/By hand * `1` Boleto (Confirmed by bank via return file or API) * `2` Pix (Confirmed by bank via Pix) Values: `0`, `1`, `2`. |
| `id` | `integer` | no | Boleto Receipt ID |
| `discharge_id` | `integer` | no | Return ID. |
| `amount` | `number` | no | Boleto amount |
| `paid_bank` | `string` | no | Receiving Bank. |
| `paid_agency` | `string` | no | Receiving Agency (with digit when available). |
| `paid_agency_number` | `string` | no | Receiving Agency Number (without digit). |
| `paid_agency_digit` | `string` | no | Receiving Agency Digit. |
| `bank_rate` | `number` | no | Bank fee |
| `credit_at` | `string (date)` | no | Credit date. |
| `direct` | `boolean` | no | Direct receipt to beneficiary |
| `expire_at` | `string (date)` | no | Maturity date. |
| `document_number` | `string` | no | Document Number. |
| `our_number` | `string` | no | Our Number. |
| `created_via_api` | `boolean` | no | Sent via API |
| `created_at` | `string (date-time)` | no | Date and time of creation. |
| `updated_at` | `string (date-time)` | no | Date and time of last update. |
| `customer_person_name` | `string` | no | Customer name |
| `paid_by_pix` | `boolean` | no | Receipt via Pix |
| `financial_provider_external_id` | `string` | no | ID of receipt in the financial institution |
| `line_parsed` | `string (json)` | no | Parsed line with information coming from the bank: Bank return content. It may change according to the bank and format 240 or 400. If any of the fields are blank, it is because your bank does not provide that information. \| Value \| Description \| \|---\|---\| \| paid_at \| Receipt date \| \| paid_amount \| Paid amount \| \| bank_rate \| Boleto fee paid to the bank \| \| credit_at \| Credit date \| \| credit_amount \| Credit amount \| \| our_number \| Our number \| \| processed_our_number_raw \| Formatted our number \| \| discount \| Discount \| \| billet_fine \| Increases (Fine/Interest charged) \| \| other_fines \| Other additional charges \| \| other_credit \| Other paid values \| \| value_rebate \| Other fees \| \| error_code \| Error code \| \| event_type \| Bank occurrence \| \| agency_number \| Agency \| \| account_number \| Account \| \| banco_recebedor \| Receiving bank \| \| agencia_recebedora \| Receiving agency \| \| control_number \| Control number \| \| document_number \| Document number \| |

More fields and nested objects: `kobana schema banking.bank-billet-payments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment made successfully |
| `422` | Invalid data |

## bank-billet-payments get

Billet Payment Information

```text
kobana banking bank-billet-payments get [ID] [OPTIONS]
```

`GET /v1/bank_billet_payments/{id}`

Queries specific information about a Bank Billet Payment.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Billet payment found |
| `404` | Billet payment not found |

## bank-billet-payments delete

Delete Billet Payment

```text
kobana banking bank-billet-payments delete [ID] [OPTIONS]
```

`DELETE /v1/bank_billet_payments/{id}`

Delete Billet Payment.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Billet Payment deleted |
| `404` | Billet Payment not found |
