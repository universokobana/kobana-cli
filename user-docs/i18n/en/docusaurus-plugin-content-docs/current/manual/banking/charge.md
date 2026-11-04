---
id: charge
title: "kobana banking charge"
sidebar_label: "charge"
description: "Charges (Pix, bank slips, automatic Pix)"
---

# kobana banking charge

Charges (Pix, bank slips, automatic Pix)

Commands of `kobana banking charge`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`charge automatic-pix accounts list`](#charge-automatic-pix-accounts-list) | `GET /v2/charge/automatic_pix/accounts` | Lists all automatic PIX accounts |
| [`charge automatic-pix accounts create`](#charge-automatic-pix-accounts-create) | `POST /v2/charge/automatic_pix/accounts` | Create new automatic PIX account |
| [`charge automatic-pix accounts get`](#charge-automatic-pix-accounts-get) | `GET /v2/charge/automatic_pix/accounts/{uid}` | View automatic PIX account |
| [`charge automatic-pix accounts update`](#charge-automatic-pix-accounts-update) | `PATCH /v2/charge/automatic_pix/accounts/{uid}` | Update automatic PIX account |
| [`charge automatic-pix accounts delete`](#charge-automatic-pix-accounts-delete) | `DELETE /v2/charge/automatic_pix/accounts/{uid}` | Delete automatic PIX account |
| [`charge automatic-pix locations list`](#charge-automatic-pix-locations-list) | `GET /v2/charge/automatic_pix/locations` | Lists all automatic Pix Locations |
| [`charge automatic-pix locations create`](#charge-automatic-pix-locations-create) | `POST /v2/charge/automatic_pix/locations` | Create new automatic PIX Location |
| [`charge automatic-pix locations get`](#charge-automatic-pix-locations-get) | `GET /v2/charge/automatic_pix/locations/{uid}` | Query a specific automatic PIX Location |
| [`charge automatic-pix pix list`](#charge-automatic-pix-pix-list) | `GET /v2/charge/automatic_pix/pix` | Lists all automatic PIX charges |
| [`charge automatic-pix pix get`](#charge-automatic-pix-pix-get) | `GET /v2/charge/automatic_pix/pix/{uid}` | Information of automatic PIX charge |
| [`charge automatic-pix pix update`](#charge-automatic-pix-pix-update) | `PATCH /v2/charge/automatic_pix/pix/{uid}` | Update automatic PIX charge |
| [`charge automatic-pix pix cancel`](#charge-automatic-pix-pix-cancel) | `PUT /v2/charge/automatic_pix/pix/{uid}/cancel` | Cancel automatic PIX charge |
| [`charge automatic-pix pix retry`](#charge-automatic-pix-pix-retry) | `PUT /v2/charge/automatic_pix/pix/{uid}/retry` | Retry automatic PIX charge |
| [`charge automatic-pix recurrences list`](#charge-automatic-pix-recurrences-list) | `GET /v2/charge/automatic_pix/recurrences` | Lists all automatic PIX recurrences |
| [`charge automatic-pix recurrences create`](#charge-automatic-pix-recurrences-create) | `POST /v2/charge/automatic_pix/recurrences` | Create new automatic PIX recurrence |
| [`charge automatic-pix recurrences pix`](#charge-automatic-pix-recurrences-pix) | `POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/pix` | Create new automatic PIX charge |
| [`charge automatic-pix recurrences requests`](#charge-automatic-pix-recurrences-requests) | `POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/requests` | Create new automatic PIX request |
| [`charge automatic-pix recurrences get`](#charge-automatic-pix-recurrences-get) | `GET /v2/charge/automatic_pix/recurrences/{uid}` | Information of automatic PIX recurrence |
| [`charge automatic-pix recurrences update`](#charge-automatic-pix-recurrences-update) | `PATCH /v2/charge/automatic_pix/recurrences/{uid}` | Update automatic PIX recurrence |
| [`charge automatic-pix recurrences cancel`](#charge-automatic-pix-recurrences-cancel) | `PUT /v2/charge/automatic_pix/recurrences/{uid}/cancel` | Cancel automatic PIX recurrence |
| [`charge automatic-pix requests list`](#charge-automatic-pix-requests-list) | `GET /v2/charge/automatic_pix/requests` | Lists all automatic PIX requests |
| [`charge automatic-pix requests get`](#charge-automatic-pix-requests-get) | `GET /v2/charge/automatic_pix/requests/{uid}` | Information of automatic PIX request |
| [`charge automatic-pix requests update`](#charge-automatic-pix-requests-update) | `PATCH /v2/charge/automatic_pix/requests/{uid}` | Update automatic PIX request |
| [`charge automatic-pix requests cancel`](#charge-automatic-pix-requests-cancel) | `PUT /v2/charge/automatic_pix/requests/{uid}/cancel` | Cancel automatic PIX request |
| [`charge payments list`](#charge-payments-list) | `GET /v2/charge/payments` | List Receipts |
| [`charge payments create`](#charge-payments-create) | `POST /v2/charge/payments` | Report Receipt |
| [`charge payments get`](#charge-payments-get) | `GET /v2/charge/payments/{uid}` | Receipt Information |
| [`charge payments delete`](#charge-payments-delete) | `DELETE /v2/charge/payments/{uid}` | Delete Receipt |
| [`charge pix commands get`](#charge-pix-commands-get) | `GET /v2/charge/pix/{pix_uid}/commands/{uid}` | View a Command |
| [`charge pix commands list`](#charge-pix-commands-list) | `GET /v2/charge/pix/{pix_uid}/commands` | List Pix Commands |
| [`charge pix list`](#charge-pix-list) | `GET /v2/charge/pix` | List Pix |
| [`charge pix create`](#charge-pix-create) | `POST /v2/charge/pix` | Create a Pix |
| [`charge pix cancel`](#charge-pix-cancel) | `PUT /v2/charge/pix/{pix_uid}/cancel` | Cancel a Pix |
| [`charge pix update`](#charge-pix-update) | `PUT /v2/charge/pix/{pix_uid}/update` | Update a Pix |
| [`charge pix get`](#charge-pix-get) | `GET /v2/charge/pix/{uid}` | View a Pix |
| [`charge pix delete`](#charge-pix-delete) | `DELETE /v2/charge/pix/{uid}` | Delete a Pix |
| [`charge pix-accounts list`](#charge-pix-accounts-list) | `GET /v2/charge/pix_accounts` | List Pix Accounts |
| [`charge pix-accounts create`](#charge-pix-accounts-create) | `POST /v2/charge/pix_accounts` | Create Pix Account |
| [`charge pix-accounts get`](#charge-pix-accounts-get) | `GET /v2/charge/pix_accounts/{uid}` | View a Pix account |
| [`charge pix-accounts update`](#charge-pix-accounts-update) | `PUT /v2/charge/pix_accounts/{uid}` | Update Pix Account |
| [`charge pix-accounts delete`](#charge-pix-accounts-delete) | `DELETE /v2/charge/pix_accounts/{uid}` | Delete Pix Account |

## charge automatic-pix accounts list

Lists all automatic PIX accounts

```text
kobana banking charge automatic-pix accounts list [OPTIONS]
```

`GET /v2/charge/automatic_pix/accounts`

Lists all user's automatic PIX accounts with pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number for pagination |
| `per_page` | `integer` | no | Number of items per page (default: 50, maximum: 50) |
| `external_id` | `string` | no | Filter by external ID |
| `financial_account_uid` | `string` | no | Filter by financial account UID |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of automatic PIX accounts filtered by financial_account_uid |

## charge automatic-pix accounts create

Create new automatic PIX account

```text
kobana banking charge automatic-pix accounts create [OPTIONS]
```

`POST /v2/charge/automatic_pix/accounts`

Creates a new automatic PIX account.

**Important**

After creating the account, it is necessary to associate a connection to it for automatic PIX to work.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string` | yes | UID of the associated financial account |
| `custom_name` | `string` | no | Custom name of the account |
| `external_id` | `string` | no | External ID for integration |
| `enabled` | `boolean` | no | Defines whether the account is enabled. |
| `custom_data` | `object` | no | Custom data |
| `tags` | `array<string>` | no | Tags of the account |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Automatic PIX account created successfully |
| `422` | Invalid data |

## charge automatic-pix accounts get

View automatic PIX account

```text
kobana banking charge automatic-pix accounts get [UID] [OPTIONS]
```

`GET /v2/charge/automatic_pix/accounts/{uid}`

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Details of automatic PIX account |
| `404` | Automatic PIX account not found |

## charge automatic-pix accounts update

Update automatic PIX account

```text
kobana banking charge automatic-pix accounts update [UID] [OPTIONS]
```

`PATCH /v2/charge/automatic_pix/accounts/{uid}`

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `custom_name` | `string` | no | Custom name of the account |
| `external_id` | `string` | no | External ID for integration |
| `enabled` | `boolean` | no | Defines whether the account is enabled. |
| `custom_data` | `object` | no | Custom data |
| `tags` | `array<string>` | no | Tags of the account |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX account updated |

## charge automatic-pix accounts delete

Delete automatic PIX account

```text
kobana banking charge automatic-pix accounts delete [UID] [OPTIONS]
```

`DELETE /v2/charge/automatic_pix/accounts/{uid}`

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX account deleted |
| `422` | Automatic PIX account has associated records (e.g: recurrence) |

## charge automatic-pix locations list

Lists all automatic Pix Locations

```text
kobana banking charge automatic-pix locations list [OPTIONS]
```

`GET /v2/charge/automatic_pix/locations`

Lists all user's automatic Pix Locations with pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number for pagination |
| `per_page` | `integer` | no | Number of items per page (default: 50, maximum: 50) |
| `external_id` | `string` | no | Filter by external ID |
| `charge_automatic_pix_account_uid` | `string` | no | Filter by automatic Pix account UID |
| `financial_provider_external_id` | `string` | no | Filter by external ID of financial provider |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of automatic Pix Locations |

## charge automatic-pix locations create

Create new automatic PIX Location

```text
kobana banking charge automatic-pix locations create [OPTIONS]
```

`POST /v2/charge/automatic_pix/locations`

Creates a new automatic PIX Location.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `charge_automatic_pix_account_uid` | `string` | yes | UID of the Automatic Pix account |
| `external_id` | `string` | no | External ID of the location |
| `tags` | `array<string>` | no | Tags for organization |
| `custom_data` | `object` | no | Custom data |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.locations.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Automatic PIX Location created successfully |
| `422` | Invalid data for creation |

## charge automatic-pix locations get

Query a specific automatic PIX Location

```text
kobana banking charge automatic-pix locations get [UID] [OPTIONS]
```

`GET /v2/charge/automatic_pix/locations/{uid}`

Returns details of a specific automatic PIX Location.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX Location found |
| `404` | Automatic PIX Location not found |

## charge automatic-pix pix list

Lists all automatic PIX charges

```text
kobana banking charge automatic-pix pix list [OPTIONS]
```

`GET /v2/charge/automatic_pix/pix`

Lists all automatic PIX charges with pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `automatic_pix_recurrence_uid` | `string` | no | Filter by automatic PIX recurrence UID |
| `external_id` | `string` | no | Filter by external ID |
| `page` | `integer` | no | Page number for pagination |
| `per_page` | `integer` | no | Number of items per page (default: 50, maximum: 50) |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of automatic PIX charges |

## charge automatic-pix pix get

Information of automatic PIX charge

```text
kobana banking charge automatic-pix pix get [UID] [OPTIONS]
```

`GET /v2/charge/automatic_pix/pix/{uid}`

Query specific information of an automatic PIX charge.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX charge found |
| `404` | Automatic PIX charge not found |

## charge automatic-pix pix update

Update automatic PIX charge

```text
kobana banking charge automatic-pix pix update [UID] [OPTIONS]
```

`PATCH /v2/charge/automatic_pix/pix/{uid}`

Updates information of an existing automatic PIX charge.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `external_id` | `string` | no | External ID of the charge. |
| `tags` | `array<string>` | no | Tags. |
| `custom_data` | `object` | no | Custom data. |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.pix.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX charge updated successfully |
| `404` | Automatic PIX charge not found |
| `422` | Duplicate external ID when updating |

## charge automatic-pix pix cancel

Cancel automatic PIX charge

```text
kobana banking charge automatic-pix pix cancel [UID] [OPTIONS]
```

`PUT /v2/charge/automatic_pix/pix/{uid}/cancel`

Cancels an existing automatic PIX charge.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX charge cancelled successfully |
| `404` | Automatic PIX charge not found |
| `422` | Error when cancelling |

## charge automatic-pix pix retry

Retry automatic PIX charge

```text
kobana banking charge automatic-pix pix retry [UID] [OPTIONS]
```

`PUT /v2/charge/automatic_pix/pix/{uid}/retry`

Retry an automatic PIX charge that failed or expired.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `date` | `string (date)` | yes | Date to retry the charge. |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.pix.retry`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX charge scheduled for retry successfully |
| `404` | Automatic PIX charge not found |
| `422` | Invalid date |

## charge automatic-pix recurrences list

Lists all automatic PIX recurrences

```text
kobana banking charge automatic-pix recurrences list [OPTIONS]
```

`GET /v2/charge/automatic_pix/recurrences`

Lists all user's automatic PIX recurrences with pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number for pagination |
| `per_page` | `integer` | no | Number of items per page (default: 50, maximum: 50) |
| `external_id` | `string` | no | Filter by external ID |
| `charge_automatic_pix_account_uid` | `string` | no | Filter by automatic Pix account UID |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of automatic PIX recurrences |

## charge automatic-pix recurrences create

Create new automatic PIX recurrence

```text
kobana banking charge automatic-pix recurrences create [OPTIONS]
```

`POST /v2/charge/automatic_pix/recurrences`

Creates a new automatic PIX recurrence.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `charge_automatic_pix_account_uid` | `string` | yes | UID of the Automatic Pix account. The financial account and the connection are inherited from this account. |
| `document` | `string` | yes | Number, identifier, or code that represents the object of the authorization (contract, order, etc.). |
| `start_at` | `string (date_time)` | yes | Start date of the recurrence. |
| `recurrence_term` | `string` | yes | Term of the recurrence. Values: `indefinite`, `fixed`. |
| `cycle` | `string` | yes | Cycle of the recurrence. Values: `weekly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `amount_term` | `string` | yes | Type of value of the recurrence. Values: `fixed`, `variable`. |
| `retry_policy` | `string` | yes | Retry policy. Values: `disallow`, `allow_3r7d`. |
| `debtor` | `object` | yes | Information of the debtor person |
| `pix_uid` | `string` | no | UID of the Pix (optional). When provided along with location_uid: the generated QR code will be from the instant journey (Journey 3) for instant Pix or offer (Journey 4) for Pix with expiration. |
| `location_uid` | `string` | no | UID of the location. When provided, the QR Code will be generated according to the journey (recurrent / instant / offer). |
| `description` | `string` | no | Description of the recurrence. |
| `finish_at` | `string (date_time)` | no | End date of the recurrence. |
| `amount` | `number` | no | Amount (when amount_term is fixed). |
| `minimum_amount` | `number` | no | Minimum amount (when amount_term is variable). |
| `agreement_code` | `string` | no | Number of the Payment Agreement. |
| `external_id` | `string` | no | External ID of the recurrence. |
| `custom_data` | `object` | no | Custom data. |
| `tags` | `array<string>` | no | Tags. |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.recurrences.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Automatic PIX recurrence created with pix_uid and location_uid (JORNADA_3) |
| `422` | External ID already exists |

## charge automatic-pix recurrences pix

Create new automatic PIX charge

```text
kobana banking charge automatic-pix recurrences pix [RECURRENCE_UID] [OPTIONS]
```

`POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/pix`

Creates a new automatic PIX charge for a recurrence.

**Path arguments:** `recurrence_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount of the charge. |
| `retry_policy` | `string` | yes | Retry policy. Values: `disallow`, `allow_3r7d`. |
| `next_business_day` | `boolean` | yes | Whether to consider the next business day. |
| `expire_at` | `string (date_time)` | yes | Expiration date of the charge. |
| `additional_info` | `string` | no | Additional information about the charge. |
| `txid` | `string` | no | Transaction identifier (optional; generated automatically if omitted). |
| `external_id` | `string` | no | External ID of the charge. |
| `tags` | `array<string>` | no | Tags. |
| `custom_data` | `object` | no | Custom data. |
| `debtor` | `object` | no | Additional debtor data |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.recurrences.pix`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Automatic PIX charge created with custom txid |
| `422` | Duplicate external ID |

## charge automatic-pix recurrences requests

Create new automatic PIX request

```text
kobana banking charge automatic-pix recurrences requests [RECURRENCE_UID] [OPTIONS]
```

`POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/requests`

Creates a new automatic PIX request for a recurrence.

**Path arguments:** `recurrence_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `expire_at` | `string (date_time)` | yes | Expiration date of the request. |
| `addressee_financial_account` | `object` | yes | Financial Account of the Recipient (payer user) |
| `external_id` | `string` | no | External ID of the request. |
| `custom_data` | `object` | no | Custom data. |
| `tags` | `array<string>` | no | Tags. |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.recurrences.requests`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Automatic PIX request created successfully |
| `422` | Duplicate external ID |

## charge automatic-pix recurrences get

Information of automatic PIX recurrence

```text
kobana banking charge automatic-pix recurrences get [UID] [OPTIONS]
```

`GET /v2/charge/automatic_pix/recurrences/{uid}`

Query specific information of an automatic PIX recurrence.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX recurrence found |
| `404` | Automatic PIX recurrence not found |

## charge automatic-pix recurrences update

Update automatic PIX recurrence

```text
kobana banking charge automatic-pix recurrences update [UID] [OPTIONS]
```

`PATCH /v2/charge/automatic_pix/recurrences/{uid}`

Updates information of an existing automatic PIX recurrence.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `external_id` | `string` | no | External ID of the recurrence. |
| `tags` | `array<string>` | no | Tags. |
| `custom_data` | `object` | no | Custom data. |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.recurrences.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX recurrence updated successfully |
| `404` | Automatic PIX recurrence not found |
| `422` | External ID already exists when updating |

## charge automatic-pix recurrences cancel

Cancel automatic PIX recurrence

```text
kobana banking charge automatic-pix recurrences cancel [UID] [OPTIONS]
```

`PUT /v2/charge/automatic_pix/recurrences/{uid}/cancel`

Cancels an existing automatic PIX recurrence.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX recurrence cancelled successfully |
| `422` | Error when cancelling |

## charge automatic-pix requests list

Lists all automatic PIX requests

```text
kobana banking charge automatic-pix requests list [OPTIONS]
```

`GET /v2/charge/automatic_pix/requests`

Lists all automatic PIX requests with pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number for pagination |
| `per_page` | `integer` | no | Number of items per page (default: 50, maximum: 50) |
| `automatic_pix_recurrence_uid` | `string` | no | Filter by automatic PIX recurrence UID |
| `charge_automatic_pix_account_uid` | `string` | no | Filter by automatic Pix account UID |
| `external_id` | `string` | no | Filter by external ID |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of automatic PIX requests |

## charge automatic-pix requests get

Information of automatic PIX request

```text
kobana banking charge automatic-pix requests get [UID] [OPTIONS]
```

`GET /v2/charge/automatic_pix/requests/{uid}`

Query specific information of an automatic PIX request.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX request found |
| `404` | Automatic PIX request not found |

## charge automatic-pix requests update

Update automatic PIX request

```text
kobana banking charge automatic-pix requests update [UID] [OPTIONS]
```

`PATCH /v2/charge/automatic_pix/requests/{uid}`

Updates information of an existing automatic PIX request.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `external_id` | `string` | no | External ID of the request. |
| `tags` | `array<string>` | no | Tags. |
| `custom_data` | `object` | no | Custom data. |

More fields and nested objects: `kobana schema banking.charge.automatic-pix.requests.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX request updated successfully |
| `404` | Automatic PIX request not found |
| `422` | Duplicate external ID when updating |

## charge automatic-pix requests cancel

Cancel automatic PIX request

```text
kobana banking charge automatic-pix requests cancel [UID] [OPTIONS]
```

`PUT /v2/charge/automatic_pix/requests/{uid}/cancel`

Cancels an existing automatic PIX request.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Automatic PIX request cancelled successfully |
| `404` | Automatic PIX request not found |
| `422` | Error when cancelling |

## charge payments list

List Receipts

```text
kobana banking charge payments list [OPTIONS]
```

`GET /v2/charge/payments`

List Receipts.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number. |
| `per_page` | `integer` | no | Number of records per page. |
| `paid_from` | `string` | no | From the Receipt Date. |
| `paid_to` | `string` | no | Until the Receipt Date. |
| `amount_from` | `number` | no | From the Receipt Value. |
| `amount_to` | `number` | no | Until the Receipt Value. |
| `kind` | `string` | no | Type of Receipt. Direct to beneficiary/In hand, boleto (Confirmed by bank via return file or API) or PIX (Confirmed by bank). Values: `direct`, `bank_billet`, `pix`. |
| `payable_type` | `string` | no | Type of Payable Resource. Values: `BankBillet`, `Pix`. |
| `payable_public_id` | `string` | no | UID of Payable Resource. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Searching by receipt type |

## charge payments create

Report Receipt

```text
kobana banking charge payments create [OPTIONS]
```

`POST /v2/charge/payments`

Reports a Receipt.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount of the Receipt. |
| `kind` | `string` | yes | Type of Receipt. Values: `direct`, `bank_billet`, `pix`. |
| `paid_at` | `string (date)` | yes | Receipt Date. |
| `payable` | `object` | yes | Resource associated with the payment (Boleto or Pix). |
| `uid` | `string (uuid)` | no | UID of the Receipt. |
| `debtor` | `object` | no | Information of the debtor (drawee) of the boleto. |
| `payer` | `object` | no | Information of who actually made the payment (payer). Automatically filled via daily bank query, available D+2 business days after payment. Null when the query has not yet been made or the bank did not return the payer's data. |
| `bank` | `object` | no | Banking information. |
| `receiving` | `object` | no | Receipt information. |
| `credit_at` | `string (date)` | no | Credit date. |
| `created_via_api` | `boolean` | no | Sent by API. |
| `transaction_identifier` | `string` | no | Transaction Identifier. |
| `status` | `string` | no | Status. |
| `duplicated` | `object` | no | Present when the receipt is a duplicate. Contains the UID of the original receipt. |
| `created_at` | `string (date)` | no | Date and time of creation. |
| `updated_at` | `string (date)` | no | Date and time of change. |

More fields and nested objects: `kobana schema banking.charge.payments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | PIX receipt reported successfully |
| `422` | Validation error |

## charge payments get

Receipt Information

```text
kobana banking charge payments get [UID] [OPTIONS]
```

`GET /v2/charge/payments/{uid}`

Query specific information of a Receipt.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Receipt found |
| `404` | Receipt not found |

## charge payments delete

Delete Receipt

```text
kobana banking charge payments delete [UID] [OPTIONS]
```

`DELETE /v2/charge/payments/{uid}`

Deletes a Receipt.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Receipt deleted |
| `404` | Receipt not found |

## charge pix commands get

View a Command

```text
kobana banking charge pix commands get [PIX_UID] [UID] [OPTIONS]
```

`GET /v2/charge/pix/{pix_uid}/commands/{uid}`

Query detailed information of a specific command executed on a Pix charge.

**Path arguments:** `pix_uid`, `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix update found |
| `404` | Pix update not found |

## charge pix commands list

List Pix Commands

```text
kobana banking charge pix commands list [PIX_UID] [OPTIONS]
```

`GET /v2/charge/pix/{pix_uid}/commands`

Lists all commands (operations) executed on a specific Pix charge, including updates, cancellations, and other operations.

**Path arguments:** `pix_uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix found |
| `404` | Pix not found |

## charge pix list

List Pix

```text
kobana banking charge pix list [OPTIONS]
```

`GET /v2/charge/pix`

Lists all registered Pix charges. Allows filtering by status, TXID, due date, payment date, CNPJ/CPF of payer and tags.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `txid` | `string` | no | Filter by Pix TXID |
| `status` | `string` | no | Filter by Pix status |
| `pix_account_id` | `integer` | no | Filter by Pix account ID |
| `pix_account_uid` | `string` | no | Filter by Pix account UID |
| `expire_from` | `string` | no | Start date of the Pix due date filter |
| `expire_to` | `string` | no | End date of the Pix due date filter |
| `paid_from` | `string` | no | Start date of the Pix payment date filter |
| `paid_to` | `string` | no | End date of the Pix payment date filter |
| `created_from` | `string` | no | Start date of the Pix creation date filter |
| `created_to` | `string` | no | End date of the Pix creation date filter |
| `cnpj_cpf` | `string` | no | Filter by payer CNPJ/CPF |
| `tags` | `string` | no | Filter by pix tags |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Searching Pix by tags |
| `422` | Invalid cnpj_cpf |

## charge pix create

Create a Pix

```text
kobana banking charge pix create [OPTIONS]
```

`POST /v2/charge/pix`

This endpoint is used for generating a Pix charge.

Pix creation is synchronous, but registration with the financial institution is performed in the background. Therefore, the request response will not contain the `qrcode` field.

To receive the qrcode, it is necessary to sign a webhook to receive the complete Pix payload when the registration is confirmed.

**Prerequisite**

To issue a Pix charge, it is necessary to have a Pix Account registered in your account. Every Pix belongs to a Pix Account that represents the payment institution and the key that will be used to issue the Pix.
The Pix Account creation must be done through the system's web interface or through the endpoint `POST /api/v2/charge/pix_accounts`.

**Webhook Events**

When registering a Pix, the system creates a command to register the PIX with the financial institution and the `pix.register.requested` event is triggered. After the Pix registration is confirmed at the institution, the `pix.register.confirmed` event is triggered.

The `pix.db.created` event is also triggered in this operation.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount in reals |
| `pix_account_uid` | `string (uuid)` | yes | UID of Pix Account |
| `expire_at` | `string (date-time)` | yes | Date and time of expiration. ISO8601 format |
| `id` | `integer` | no | ID of Pix (DEPRECATED: use uid) |
| `uid` | `string (uuid)` | no | UID of Pix |
| `payer` | `object` | no | Payer data |
| `status` | `string` | no | Pix status. * `opened`: Open Pix charge (unpaid). [Initial State] * `paid`: Pix charge paid. * `overdue`: Overdue Pix charge. * `canceled`: Pix charge canceled. * `generation_failed`: Pix charge created, but was not registered with the bank. Values: `opened`, `paid`, `overdue`, `canceled`, `generation_failed`. |
| `pix_account_id` | `integer` | no | ID of Pix Account (DEPRECATED: use pix_account_uid) |
| `txid` | `anyOf` | no | Pix TXID - Unique identifier of the transaction |
| `canceled_at` | `string (date-time)` | no | Date and time of Pix cancellation |
| `registered_at` | `string (date-time)` | no | Date and time of registration with the provider |
| `paid_at` | `string (date-time)` | no | Date and time of payment |
| `paid_amount` | `number` | no | Amount actually paid |
| `registration_status` | `string` | no | Provider registration status Values: `pending`, `skipped`, `requested`, `confirmed`, `rejected`, `failed`. |
| `revoke_days` | `anyOf` | no | Number of days active after expiration. REQUIRED and must be greater than zero when Pix has interest or fine configured. |
| `message` | `anyOf` | no | Request message to the payer |
| `message_payer` | `anyOf` | no | Message for the payer (read-only field) |
| `beneficiary` | `object` | no | Pix beneficiary data |
| `url` | `string` | no | URL for Pix viewing |
| `formats` | `object` | no | URLs with formats available for Pix |
| `additional_info` | `anyOf` | no | Additional information for the payer (maximum 10 key-value pairs) |
| `qrcode` | `object` | no | Pix QR Code |
| `registration_kind` | `string` | no | Pix type. * `instant`: Immediate (default) * `billing`: Billing Values: `instant`, `billing`. |
| `kind` | `string` | no | Pix type. * `instant`: Immediate (default) * `billing`: Billing Values: `instant`, `billing`. |
| `command_ids` | `array<integer>` | no |  |
| `payments` | `array<string (uuid)>` | no | List of UIDs of Receipts linked to Pix |
| `end_to_end_id` | `anyOf` | no | End To End Id of Pix - Unique identifier of the transfer |
| `custom_data` | `anyOf` | no | Hash with key and value in JSON format. |
| `external_id` | `string` | no | External identifier for Pix |
| `virtual_pix_uid` | `string` | no | UID of Pix generated by BackOffice member. (BackOffice must be enabled). |
| `fine_type` | `integer` | no | Type of fine: * `0` No fine (`none`) * `1` By amount (`amount`) * `2` By percentage (`percentage`) On return the data come in the `fine` object. Values: `0`, `1`, `2`. |
| `fine_amount` | `number` | no | Fine amount in reals. Required when `fine_type` = `amount`. |
| `fine_percentage` | `number` | no | Fine percentage (0.0 to 100.0). Required when `fine_type` = `percentage`. |
| `interest_type` | `integer` | no | Type of interest: * `0` Does not apply / exempt (`none`) * `1` Daily amount — calendar days (`daily_amount_calendar`) * `2` Daily percentage — calendar days (`daily_percentage_calendar`) * `3` Monthly percentage — calendar days (`monthly_percentage_calendar`) * `4` Annual percentage — calendar days (`yearly_percentage_calendar`) * `5` Daily amount — business days (`daily_amount_business`) * `6` Daily percentage — business days (`daily_percentage_business`) * `7` Monthly percentage — business days (`monthly_percentage_business`) * `8` Annual percentage — business days (`yearly_percentage_business`) On return the data come in the `interest` object. Values: `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`. |
| `interest_amount` | `number` | no | Daily interest amount in reals. Required when `interest_type` is by amount (`daily_amount_*`). |
| `interest_percentage` | `number` | no | Interest percentage. Required when `interest_type` is by percentage. |
| `reduction_type` | `integer` | no | Type of reduction: * `1` By amount (`amount`) * `2` By percentage (`percentage`) * `3` Does not apply — removes the reduction (`none`) On return the data come in the `reduction` object. Values: `1`, `2`, `3`. |
| `reduction_amount` | `number` | no | Reduction amount in reals. Required when `reduction_type` = `amount`. |
| `reduction_percentage` | `number` | no | Reduction percentage. Required when `reduction_type` = `percentage`. |
| `discount_type` | `integer` | no | Type of discount applied to the `discount_*` fields (null/absent = no discount): * `1` Fixed amount (`fixed_amount`) * `2` Fixed percentage (`fixed_percentage`) * `3` Early payment by amount — calendar days (`advance_amount_calendar`) * `4` Early payment by amount — business days (`advance_amount_business`) * `5` Early payment by percentage — calendar days (`advance_percentage_calendar`) * `6` Early payment by percentage — business days (`advance_percentage_business`) Early payment modes (`3`–`6`) accept only one range. On return the type comes in `discounts.type`. Values: `1`, `2`, `3`, `4`, `5`, `6`. |
| `discount_amount` | `number` | no | Value of 1st discount in reals. Required when `discount_type` is by amount. |
| `discount_percentage` | `number` | no | Percentage of 1st discount. Required when `discount_type` is by percentage. |
| `discount_days` | `integer` | no | Days before expiration when the 1st discount is valid. |
| `discount_second_amount` | `number` | no | Value of 2nd discount in reals. |
| `discount_second_percentage` | `number` | no | Percentage of 2nd discount. |
| `discount_second_days` | `integer` | no | Days before expiration when the 2nd discount is valid. |
| `discount_third_amount` | `number` | no | Value of 3rd discount in reals. |
| `discount_third_percentage` | `number` | no | Percentage of 3rd discount. |
| `discount_third_days` | `integer` | no | Days before expiration when the 3rd discount is valid. |
| `fine` | `object` | no | Fine configuration (RETURN/response as nested object). On SEND/request use the flat fields `fine_type`, `fine_amount` and `fine_percentage`. |
| `reduction` | `object` | no | Reduction configuration (RETURN/response as nested object). On SEND/request use the flat fields `reduction_type`, `reduction_amount` and `reduction_percentage`. |
| `interest` | `object` | no | Interest configuration (RETURN/response as nested object). On SEND/request use the flat fields `interest_type`, `interest_amount` and `interest_percentage`. |
| `discounts` | `object` | no | Discounts configuration (RETURN/response as nested object with up to 3 discounts). On SEND/request use the flat fields `discount_*`, `discount_second_*` and `discount_third_*`. |
| `ignore_whatsapp` | `boolean` | no | Never send this Pix via WhatsApp |
| `password_protected_mode` | `integer` | no | Password protection mode: 0=disabled, 1=last 4 digits, 2=first 5 digits Values: `0`, `1`, `2`. |
| `tags` | `array<string>` | no | Tags associated with the pix |
| `tag_list` | `string` | no | Tags associated with the pix |
| `created_via_api` | `boolean` | no | Indicates if Pix was created by API |
| `financial_provider_external_id` | `string` | no | ID in the financial institution |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.charge.pix.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Pix created with customer data |
| `422` | Validation error |

## charge pix cancel

Cancel a Pix

```text
kobana banking charge pix cancel [PIX_UID] [OPTIONS]
```

`PUT /v2/charge/pix/{pix_uid}/cancel`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates a cancellation command for the PIX.
Cancellation of a PIX is done asynchronously, that is, when receiving the request, our API stores the request and creates a Command. This command is processed in the background and after confirmed, the Pix is cancelled. You receive in the response of this request the ID of the created command and can query it anytime. See more in List Pix Commands and View a Command.

**Webhook Events**

When the request is made, a `pix.command.cancel.started` event is triggered.
Our server then begins attempts to send the cancellation to the financial institution.
Only after it is confirmed at the financial institution, then the pix is cancelled in our database.
At this moment the `pix.command.cancel.confirmed` event is triggered.

The `pix.db.updated` event is also triggered on confirmation of this operation.
> ⚠️ **Attention to return payload:** The response of a PIX cancellation call is the payload of a Command. Inside the command payload there is the `pix` key with the PIX data in question.

**Path arguments:** `pix_uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully |
| `422` | Pix already cancelled |

## charge pix update

Update a Pix

```text
kobana banking charge pix update [PIX_UID] [OPTIONS]
```

`PUT /v2/charge/pix/{pix_uid}/update`

> **Asynchronous Processing:** This endpoint is processed asynchronously and creates an update command for the PIX.
PIX update is done asynchronously, that is, when receiving the request, our API stores the changes you are requesting and creates a Command. This command is processed in the background and after confirmed, the Pix is updated. You receive in the response of this request the ID of the created command and can query it anytime. See more in List Pix Commands and View a Command.

**Webhook Events**

When the request is made, a `pix.update.requested` event is triggered.
Our server then begins attempts to send the change to the financial institution.
Only after it is confirmed at the financial institution, then the pix is updated.
At this moment the `pix.update.confirmed` event is triggered.

The `pix.db.updated` event is also triggered on confirmation of this operation.
> ⚠️ **Attention to return payload:** The response of a PIX update call is the payload of a Command. Inside the command payload there is the `pix` key with the PIX data in question.

**Path arguments:** `pix_uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | no | Amount |
| `pix_account_uid` | `string (uuid)` | no | UID of the Pix Account to change the receiver's Pix key. This field is useful when you want to change the Pix key with the financial provider. **Important**: Only Pix accounts that are active (status `active`) are accepted. |
| `payer` | `object` | no | Payer data |
| `status` | `string` | no | Pix status. `opened`: Open Pix charge (unpaid). [Initial State] `paid`: Pix charge paid. `overdue`: Overdue Pix charge. `canceled`: Pix charge canceled. `generation_failed`: Pix charge created, but was not registered with the bank. |
| `expire_at` | `string (date-time)` | no | Date and time of expiration. ISO8601 format |
| `revoke_days` | `integer` | no | Number of days active after expiration |
| `message` | `string` | no | Request message to the payer |
| `additional_info` | `object` | no | Additional information for the payer |
| `fine_type` | `integer` | no | Type of fine: * `0` No fine (`none`) * `1` By amount (`amount`) * `2` By percentage (`percentage`) On return the data come in the `fine` object. Values: `0`, `1`, `2`. |
| `fine_amount` | `number` | no | Fine amount in reals. Required when `fine_type` = `amount`. |
| `fine_percentage` | `number` | no | Fine percentage (0.0 to 100.0). Required when `fine_type` = `percentage`. |
| `interest_type` | `integer` | no | Type of interest: * `0` Does not apply / exempt (`none`) * `1` Daily amount — calendar days (`daily_amount_calendar`) * `2` Daily percentage — calendar days (`daily_percentage_calendar`) * `3` Monthly percentage — calendar days (`monthly_percentage_calendar`) * `4` Annual percentage — calendar days (`yearly_percentage_calendar`) * `5` Daily amount — business days (`daily_amount_business`) * `6` Daily percentage — business days (`daily_percentage_business`) * `7` Monthly percentage — business days (`monthly_percentage_business`) * `8` Annual percentage — business days (`yearly_percentage_business`) On return the data come in the `interest` object. Values: `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`. |
| `interest_amount` | `number` | no | Daily interest amount in reals. Required when `interest_type` is by amount (`daily_amount_*`). |
| `interest_percentage` | `number` | no | Interest percentage. Required when `interest_type` is by percentage. |
| `reduction_type` | `integer` | no | Type of reduction: * `1` By amount (`amount`) * `2` By percentage (`percentage`) * `3` Does not apply — removes the reduction (`none`) On return the data come in the `reduction` object. Values: `1`, `2`, `3`. |
| `reduction_amount` | `number` | no | Reduction amount in reals. Required when `reduction_type` = `amount`. |
| `reduction_percentage` | `number` | no | Reduction percentage. Required when `reduction_type` = `percentage`. |
| `discount_type` | `integer` | no | Type of discount applied to the `discount_*` fields (null/absent = no discount): * `1` Fixed amount (`fixed_amount`) * `2` Fixed percentage (`fixed_percentage`) * `3` Early payment by amount — calendar days (`advance_amount_calendar`) * `4` Early payment by amount — business days (`advance_amount_business`) * `5` Early payment by percentage — calendar days (`advance_percentage_calendar`) * `6` Early payment by percentage — business days (`advance_percentage_business`) Early payment modes (`3`–`6`) accept only one range. On return the type comes in `discounts.type`. Values: `1`, `2`, `3`, `4`, `5`, `6`. |
| `discount_amount` | `number` | no | Value of 1st discount in reals. Required when `discount_type` is by amount. |
| `discount_percentage` | `number` | no | Percentage of 1st discount. Required when `discount_type` is by percentage. |
| `discount_days` | `integer` | no | Days before expiration when the 1st discount is valid. |
| `discount_second_amount` | `number` | no | Value of 2nd discount in reals. |
| `discount_second_percentage` | `number` | no | Percentage of 2nd discount. |
| `discount_second_days` | `integer` | no | Days before expiration when the 2nd discount is valid. |
| `discount_third_amount` | `number` | no | Value of 3rd discount in reals. |
| `discount_third_percentage` | `number` | no | Percentage of 3rd discount. |
| `discount_third_days` | `integer` | no | Days before expiration when the 3rd discount is valid. |
| `created_via_api` | `boolean` | no | Indicates if Pix was created by API |
| `financial_provider_external_id` | `string` | no | ID in the financial institution |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.charge.pix.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Command created successfully - with pix_account_uid |
| `422` | Pix paid |

## charge pix get

View a Pix

```text
kobana banking charge pix get [UID] [OPTIONS]
```

`GET /v2/charge/pix/{uid}`

Query specific information of a Pix.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix found |
| `404` | Pix not found |

## charge pix delete

Delete a Pix

```text
kobana banking charge pix delete [UID] [OPTIONS]
```

`DELETE /v2/charge/pix/{uid}`

Deletes a cancelled Pix.

This operation permanently deletes the PIX from our database. Even after deletion, the PIX is counted in the volumetry for the contracted plan

**Webhook Events**

The `pix.db.deleted` event is triggered in this operation.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Pix deleted |
| `404` | Pix not found |
| `422` | Pix open |

## charge pix-accounts list

List Pix Accounts

```text
kobana banking charge pix-accounts list [OPTIONS]
```

`GET /v2/charge/pix_accounts`

Lists the Pix accounts registered in the account.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix accounts found |

## charge pix-accounts create

Create Pix Account

```text
kobana banking charge pix-accounts create [OPTIONS]
```

`POST /v2/charge/pix_accounts`

This endpoint is used to register a new pix account.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `financial_account_uid` | `string` | yes | UID of the financial account. The provider and beneficiary data are derived automatically. |
| `key` | `string` | yes | Pix key |
| `custom_name` | `string` | no | Custom name of the pix account |
| `enabled` | `boolean` | no | Enable Pix Account |
| `default` | `boolean` | no | Default account |

More fields and nested objects: `kobana schema banking.charge.pix-accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Pix account created |
| `422` | Invalid Pix account |

## charge pix-accounts get

View a Pix account

```text
kobana banking charge pix-accounts get [UID] [OPTIONS]
```

`GET /v2/charge/pix_accounts/{uid}`

Query specific information of a Pix account.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix account found |
| `404` | Pix account not found |

## charge pix-accounts update

Update Pix Account

```text
kobana banking charge pix-accounts update [UID] [OPTIONS]
```

`PUT /v2/charge/pix_accounts/{uid}`

This endpoint is used to update a pix account.

**Path arguments:** `uid`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `custom_name` | `string` | no | Custom name of the pix account |
| `key` | `string` | no | Pix key |
| `enabled` | `boolean` | no | Enable Pix Account |
| `default` | `boolean` | no | Default account |
| `financial_account_uid` | `string` | no | UID of the financial account (to associate with an existing account) |

More fields and nested objects: `kobana schema banking.charge.pix-accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix account updated |
| `404` | Pix account not found |
| `422` | Invalid Pix key |

## charge pix-accounts delete

Delete Pix Account

```text
kobana banking charge pix-accounts delete [UID] [OPTIONS]
```

`DELETE /v2/charge/pix_accounts/{uid}`

Deletes a Pix account.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `202` | Pix account deleted |
| `404` | Pix account not found |
