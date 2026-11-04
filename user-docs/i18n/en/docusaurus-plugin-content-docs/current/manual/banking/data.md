---
id: data
title: "kobana banking data"
sidebar_label: "data"
description: "Lookups (bank slips, Pix QR codes)"
---

# kobana banking data

Lookups (bank slips, Pix QR codes)

Commands of `kobana banking data`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`data bank-billet-queries list`](#data-bank-billet-queries-list) | `GET /v2/data/bank_billet_queries` | List Bank Billet Queries |
| [`data bank-billet-queries create`](#data-bank-billet-queries-create) | `POST /v2/data/bank_billet_queries` | Create a Bank Billet Query |
| [`data people list`](#data-people-list) | `GET /v2/data/people` | Search People |
| [`data people create`](#data-people-create) | `POST /v2/data/people` | Create a Person |
| [`data people get`](#data-people-get) | `GET /v2/data/people/{uid}` | View a Person |
| [`data people delete`](#data-people-delete) | `DELETE /v2/data/people/{uid}` | Delete a Person |
| [`data pix-qrcode-queries list`](#data-pix-qrcode-queries-list) | `GET /v2/data/pix_qrcode_queries` | List Pix QR Code Queries |
| [`data pix-qrcode-queries create`](#data-pix-qrcode-queries-create) | `POST /v2/data/pix_qrcode_queries` | Create a Pix QR Code Query |

## data bank-billet-queries list

List Bank Billet Queries

```text
kobana banking data bank-billet-queries list [OPTIONS]
```

`GET /v2/data/bank_billet_queries`

Lists all bank billet queries performed by the authenticated account. Allows filtering by status, tags, creation dates and other parameters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing bank billet queries |

## data bank-billet-queries create

Create a Bank Billet Query

```text
kobana banking data bank-billet-queries create [OPTIONS]
```

`POST /v2/data/bank_billet_queries`

Creates a new bank billet query through the digit line or bar code. The query is processed automatically and returns detailed information about the bank billet.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `uid` | `string` | no | Unique record ID in UUID format |
| `status` | `string` | no | Query status: * `pending` Pending (only when configured not to process automatically) * `success` Successfully completed * `error` Query failed Values: `pending`, `success`, `error`. |
| `error_code` | `string` | no | Error code when `status` is `error`: * `bank_billet_unavailable` Boleto unavailable: it may have already been paid, canceled or written off * `document_not_supported` The provider did not confirm the document for payment: the agreement or modality may not be supported * `provider_error` Provider failure while querying the boleto * `unavailable_environment` Query available only in production Values: `null`, `bank_billet_unavailable`, `document_not_supported`, `provider_error`, `unavailable_environment`. |
| `error_message` | `string` | no | Error message when the query is processed but data is not returned. |
| `line_or_barcode` | `string` | no | Readable Line or Bar Code, in the formats: Readable Line: 34191.79001 01043.510047 91020.150008 7 75870000001000 Bar Code: 34197758700000010001790010104351004791020150 |
| `line` | `string` | no | Digitizable Line |
| `barcode` | `string` | no | Bar code |
| `expire_at` | `string (date)` | no | Due Date |
| `pay_due_date` | `string (date)` | no | Payment deadline (grace period), when informed by the originating source |
| `amount` | `number` | no | Amount to pay |
| `minimum_amount` | `number` | no | Minimum amount |
| `maximum_amount` | `number` | no | Maximum amount |
| `original_amount` | `number` | no | Original amount |
| `valid_for_payment` | `boolean` | no | Indicates whether the boleto is eligible for payment at the time of the query |
| `payment_status_reason` | `string` | no | Reason returned when the boleto is not eligible for payment |
| `allow_change_value` | `boolean` | no | Indicates whether the payment amount can be changed within the returned limits |
| `allow_partial_payment` | `boolean` | no | Indicates whether the originating source allows partial payment |
| `issue_bank` | `object` | no | Issuing bank of the boleto |
| `beneficiary` | `object` | no | Beneficiary Data |
| `guarantor` | `object` | no | Data of the Ultimate Beneficiary |
| `guarantor_drawer` | `object` | no | Data of the Drawer Guarantor |
| `payer` | `object` | no | Payer data |
| `fine` | `object` | no | Fine |
| `interest` | `object` | no | Interest |
| `discount` | `object` | no | Discount |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Associated tags |
| `created_via_api` | `boolean` | no | Indicates if the record was created by API |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.data.bank-billet-queries.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Bank Billet Query created |
| `422` | Invalid Bank Billet Query |

## data people list

Search People

```text
kobana banking data people list [OPTIONS]
```

`GET /v2/data/people`

Lists all people registered in the authenticated account. Allows filtering by external code, person type, document, email, tags and creation period.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `external_code` | `string` | no | Filter by External Code. Finds all records whose field equals the string. |
| `kind` | `string` | no | Filter by Person Type (Natural or Legal). |
| `document_type` | `string` | no | Filter by Document Type (CPF or CNPJ). |
| `document_number` | `string` | no | Filter by Document Number. Finds all records whose field equals the string. |
| `tags` | `array` | no | Tags associated with the record. |
| `created_from` | `string` | no | Start date for creation date filter. |
| `created_to` | `string` | no | End date for creation date filter. |
| `per_page` | `integer` | no | Number of records per page. Maximum of 50. |
| `page` | `integer` | no | Page number. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Records found (page 2 with prev_url) |

## data people create

Create a Person

```text
kobana banking data people create [OPTIONS]
```

`POST /v2/data/people`

Creates a new person (natural or legal) with the provided information. The person will be associated with the authenticated account.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Full name or Company Name |
| `uid` | `string (uuid)` | no | Unique record identifier |
| `nickname` | `string` | no | Nickname or Alias |
| `kind` | `string` | no | Type of Person |
| `external_code` | `string` | no | Code or ID in your management system. |
| `birthday` | `string (date)` | no | Date of Birth. |
| `document_type` | `string` | no | Document Type. Valid values: CPF, CNPJ |
| `document_number` | `string` | no | Document Number. |
| `addresses` | `array<object>` | no | List of Addresses |
| `phones` | `array<object>` | no | List of Phone Numbers |
| `emails` | `array<object>` | no | List of Emails |
| `tags` | `array<string>` | no | Tags associated with the record |
| `tag_list` | `string` | no | List of tags separated by comma |
| `custom_data` | `object` | no | Available for sending a JSON with a free data structure |
| `notes` | `string` | no | Notes. Free text field |
| `created_via_api` | `boolean` | no | Indicates if the record was created by API |
| `created_at` | `string (date-time)` | no | Date and time of record creation in the database |
| `updated_at` | `string (date-time)` | no | Date and time of the last update of the record in the database |

More fields and nested objects: `kobana schema banking.data.people.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Person created |
| `422` | Validation error |

## data people get

View a Person

```text
kobana banking data people get [UID] [OPTIONS]
```

`GET /v2/data/people/{uid}`

Returns detailed information about a specific person, including personal data, addresses, phones, emails and custom data.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Person found |
| `404` | Person not found |

## data people delete

Delete a Person

```text
kobana banking data people delete [UID] [OPTIONS]
```

`DELETE /v2/data/people/{uid}`

Permanently deletes a person from the system. This action cannot be undone.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Record deleted |
| `404` | Record not found |

## data pix-qrcode-queries list

List Pix QR Code Queries

```text
kobana banking data pix-qrcode-queries list [OPTIONS]
```

`GET /v2/data/pix_qrcode_queries`

Lists all Pix QR Code queries performed by the authenticated account. Allows filtering by tags, external_id and other parameters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `external_id` | `string` | no | Filter by external ID |
| `tags` | `array` | no | Filter by tags |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing Pix QR Code queries |

## data pix-qrcode-queries create

Create a Pix QR Code Query

```text
kobana banking data pix-qrcode-queries create [OPTIONS]
```

`POST /v2/data/pix_qrcode_queries`

Creates a new Pix QR Code query through the qrcode. The query is processed automatically and returns detailed information about the Pix QR Code.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `uid` | `string` | no | Unique record ID in UUID format |
| `qrcode` | `string` | no | Pix QR Code for decoding |
| `pix_key` | `string` | no | Beneficiary Pix key |
| `expires_at` | `string (date-time)` | no | QR Code expiration date |
| `finality` | `string` | no | Purpose of payment (payment, withdrawal, change) |
| `amount` | `number` | no | Principal amount |
| `amount_change` | `number` | no | Change amount |
| `interest` | `number` | no | Interest amount |
| `fine` | `number` | no | Fine amount |
| `discount` | `number` | no | Discount amount |
| `total_amount` | `number` | no | Total amount to pay |
| `beneficiary` | `object` | no | Beneficiary/Receiver Data |
| `debtor` | `object` | no | Debtor Data |
| `description` | `string` | no | Message to payer |
| `can_be_paid` | `boolean` | no | Indicates whether the QR Code can be paid |
| `can_be_paid_with_different_value` | `boolean` | no | Indicates whether it can be paid with a different amount |
| `cannot_be_paid_reason` | `string` | no | Reason why it cannot be paid |
| `external_id` | `string` | no | ID in your system. Optional for internal control. |
| `custom_data` | `object` | no | Hash with key and value in JSON format. |
| `tags` | `array<string>` | no | Tags associated for categorization |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.data.pix-qrcode-queries.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Pix QR Code Query created |
| `422` | Invalid Pix QR Code Query |
