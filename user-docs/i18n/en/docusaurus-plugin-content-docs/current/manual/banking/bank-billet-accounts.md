---
id: bank-billet-accounts
title: "kobana banking bank-billet-accounts"
sidebar_label: "bank-billet-accounts"
---

# kobana banking bank-billet-accounts


Commands of `kobana banking bank-billet-accounts`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`bank-billet-accounts list`](#bank-billet-accounts-list) | `GET /v1/bank_billet_accounts` | List Wallets |
| [`bank-billet-accounts create`](#bank-billet-accounts-create) | `POST /v1/bank_billet_accounts` | Create Collection Wallet |
| [`bank-billet-accounts get`](#bank-billet-accounts-get) | `GET /v1/bank_billet_accounts/{id}` | Wallet Information |
| [`bank-billet-accounts update`](#bank-billet-accounts-update) | `PUT /v1/bank_billet_accounts/{id}` | Update Wallet |
| [`bank-billet-accounts delete`](#bank-billet-accounts-delete) | `DELETE /v1/bank_billet_accounts/{id}` | Delete Collection Wallet |
| [`bank-billet-accounts ask`](#bank-billet-accounts-ask) | `GET /v1/bank_billet_accounts/{id}/ask` | Request Collection Wallet Approval |
| [`bank-billet-accounts set-default`](#bank-billet-accounts-set-default) | `PUT /v1/bank_billet_accounts/{id}/set_default` | Change Default Collection Wallet |
| [`bank-billet-accounts validate`](#bank-billet-accounts-validate) | `PUT /v1/bank_billet_accounts/{id}/validate` | Validate Collection Wallet |

## bank-billet-accounts list

List Wallets

```text
kobana banking bank-billet-accounts list [OPTIONS]
```

`GET /v1/bank_billet_accounts`

List Wallets.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Wallets Found |

## bank-billet-accounts create

Create Collection Wallet

```text
kobana banking bank-billet-accounts create [OPTIONS]
```

`POST /v1/bank_billet_accounts`

Creates a new collection wallet for issuing bank billets. The wallet needs to be approved before it can be used in production.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `bank_contract_slug` | `string` | yes | Wallet slug |
| `financial_account_uid` | `string (uuid)` | yes | UID of the Financial Account. When informed, the agency and account data will be automatically populated from the financial account. |
| `beneficiary_name` | `string` | yes | Beneficiary Name |
| `beneficiary_cnpj_cpf` | `string` | yes | CNPJ/CPF of Beneficiary |
| `beneficiary_address_street` | `string` | yes | Beneficiary street |
| `beneficiary_address_street_number` | `string` | yes | Beneficiary street number |
| `beneficiary_address_complement` | `string` | yes | Beneficiary address complement |
| `beneficiary_address_neighborhood` | `string` | yes | Beneficiary neighborhood |
| `beneficiary_address_city` | `string` | yes | Beneficiary city |
| `beneficiary_address_state` | `string` | yes | Beneficiary state |
| `beneficiary_address_zipcode` | `string` | yes | Beneficiary postal code (format 99999999) |
| `beneficiary_address` | `string` | yes | Beneficiary complete address. |
| `id` | `integer` | no | Wallet ID |
| `next_our_number` | `integer` | no | Next Our Number. Default: 1 |
| `next_remittance_number` | `integer` | no | Last remittance sequential Default: 1 |
| `agency_number` | `string` | no | Agency. Automatically filled when financial_account_uid is provided. |
| `agency_digit` | `string` | no | Agency Digit. Automatically filled when financial_account_uid is provided. |
| `account_number` | `string` | no | Account. Automatically filled when financial_account_uid is provided. |
| `account_digit` | `string` | no | Account check digit. Automatically filled when financial_account_uid is provided. |
| `extra1` | `string` | no | Extra field 1. Depending on the wallet chosen, it may be required. |
| `extra1_digit` | `string` | no | Digit of extra field 1. Depending on the wallet chosen, it may be required. |
| `extra2` | `string` | no | Extra field 2. Depending on the wallet chosen, it may be required. |
| `extra2_digit` | `string` | no | Digit of extra field 2. Depending on the wallet chosen, it may be required. |
| `extra3` | `string` | no | Remittance code |
| `payment_place` | `string` | no | Payment location |
| `contract` | `string` | no | Contract |
| `contract_type` | `string` | no | Contract type |
| `financial_account` | `object` | no | Financial account associated with the wallet |
| `bolepix_enabled_at` | `string` | no | Date and time when the hybrid boleto was enabled. |
| `bolepix` | `object` | no | Hybrid boleto configuration |
| `name` | `string` | no | Account name. Not received at creation or update, only returned in query and listing |
| `status` | `string` | no | Wallet status: When the wallet has just been registered, it receives the status `pending`. At this point the user must accept the terms and start the homologation. When clicking on "Proceed with Homologation", the status changes to `homologating`. A boleto and a Homologation remittance will be generated to be sent to the bank. After completion, the status changes to `validating`. From this point on, the user needs to inform the value of the boleto that was generated or send the return processed by the bank. When the value is correctly informed, or in the processed return, the bank says everything is correct, the status changes to `active`. * `pending` Homologation not started * `homologating` In homologation, awaiting boleto payment * `validating` Boleto paid, awaiting validation * `active` Active and ready to use Values: `pending`, `homologating`, `validating`, `active`. |
| `default` | `boolean` | no | Default. The `default` field determines the collection wallet that will be used in the creation of the boleto when no wallet is provided. When the first wallet is homologated (moves to `status` = `active`), it receives the value `default` = `true` |
| `configuration` | `object` | no | Default data configuration for boleto |
| `bank_contract` | `object` | no | Wallet data. Not received at creation or update, only returned in query and listing. |
| `custom_name` | `string` | no | Wallet name for identification within Kobana. |
| `kind` | `string` | no | CNAB type: * `cnab400` CNAB 400 * `cnab240` CNAB 240 Values: `cnab400`, `cnab240`. |
| `remittance_disabled` | `boolean` | no | Disables remittance generation. |
| `remittance_disabled_at` | `string (date-time)` | no | Date and time when remittance generation was disabled. |
| `allow_expiration_on_weekends` | `boolean` | no | Allow maturity on weekends and holidays |
| `created_at` | `string (date-time)` | no | Wallet creation date and time |
| `updated_at` | `string (date-time)` | no | Wallet last update date and time |
| `created_via_api` | `boolean` | no | Indicates if the wallet was created by API |

More fields and nested objects: `kobana schema banking.bank-billet-accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Collection wallet created with financial_account_uid |
| `422` | Invalid wallet - Beneficiary CPF |

## bank-billet-accounts get

Wallet Information

```text
kobana banking bank-billet-accounts get [ID] [OPTIONS]
```

`GET /v1/bank_billet_accounts/{id}`

Query specific information from a Wallet

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Wallet found |
| `404` | Wallet not found |

## bank-billet-accounts update

Update Wallet

```text
kobana banking bank-billet-accounts update [ID] [OPTIONS]
```

`PUT /v1/bank_billet_accounts/{id}`

Updates specific information of a Wallet.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `kind` | `string` | no | CNAB type: * `cnab400` CNAB 400 * `cnab240` CNAB 240 Values: `cnab400`, `cnab240`. |
| `bank_contract_slug` | `string` | no | Wallet slug. Active wallets will not be able to have the bank_contract_slug field changed. |
| `financial_account` | `object` | no | Financial account associated with the wallet |
| `payment_place` | `string` | no | Payment location |
| `custom_name` | `string` | no | Wallet name for identification within Kobana. |
| `allow_expiration_on_weekends` | `boolean` | no | Allow maturity on weekends and holidays |
| `beneficiary_name` | `string` | no | Beneficiary Name |
| `beneficiary_cnpj_cpf` | `string` | no | CNPJ/CPF of Beneficiary |
| `agency_number` | `string` | no | Agency. Automatically filled when financial_account_uid is provided. |
| `agency_digit` | `string` | no | Agency check digit. Optional: will be calculated automatically using Module 11 when the bank requires it and is not provided. Automatically filled when financial_account_uid is provided. |
| `account_number` | `string` | no | Account. Automatically filled when financial_account_uid is provided. |
| `account_digit` | `string` | no | Account check digit. Automatically filled when financial_account_uid is provided. |
| `extra1` | `string` | no | Extra field 1. |
| `extra1_digit` | `string` | no | Digit of extra field 1. |
| `extra2` | `string` | no | Extra field 2. |
| `extra2_digit` | `string` | no | Digit of extra field 2. |
| `contract` | `string` | no | Contract |
| `contract_type` | `string` | no | Contract type |
| `extra3` | `string` | no | Remittance code |
| `beneficiary_address_street` | `string` | no | Beneficiary street |
| `beneficiary_address_street_number` | `string` | no | Beneficiary street number |
| `beneficiary_address_complement` | `string` | no | Beneficiary address complement |
| `beneficiary_address_neighborhood` | `string` | no | Beneficiary neighborhood |
| `beneficiary_address_city` | `string` | no | Beneficiary city |
| `beneficiary_address_state` | `string` | no | Beneficiary state |
| `beneficiary_address_zipcode` | `string` | no | Beneficiary postal code (format 99999999) |
| `beneficiary_address` | `string` | no | Beneficiary complete address. |
| `bolepix_enabled` | `boolean` | no | Enable Bolepix (legacy — prefer the `bolepix` object) |
| `bolepix` | `object` | no | Hybrid boleto configuration |
| `bolepix_key` | `string` | no | Pix key that will be used in Bolepix |
| `default` | `boolean` | no | Default. The `default` field determines the collection portfolio that will be used when creating the boleto when no portfolio is informed. At the moment the first portfolio is approved (moves to `status` = `active`), it receives the value `default` = `true` |
| `remittance_disabled` | `boolean` | no | Disables remittance generation. |
| `next_our_number` | `integer` | no | Last our number |
| `next_remittance_number` | `integer` | no | Last remittance sequential Default: 1 |
| `bank_billet_account_next_our_number_attributes` | `object` | no |  |
| `bank_billet_account_next_remittance_number_attributes` | `object` | no |  |

More fields and nested objects: `kobana schema banking.bank-billet-accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Wallet updated |
| `404` | Wallet not found |
| `422` | Unprocessable Entity |

## bank-billet-accounts delete

Delete Collection Wallet

```text
kobana banking bank-billet-accounts delete [ID] [OPTIONS]
```

`DELETE /v1/bank_billet_accounts/{id}`

Deletes a Collection Wallet. Optionally, also deletes the associated Financial Account.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `delete_financial_account` | `boolean` | no | When `true`, also deletes the Financial Account associated with the Wallet. |

**Responses**

| HTTP | Description |
|---|---|
| `202` | Wallet and Financial Account deleted (delete_financial_account=true) |
| `204` | Wallet queued for deletion (delete_financial_account=false) |
| `404` | Wallet not found |

## bank-billet-accounts ask

Request Collection Wallet Approval

```text
kobana banking bank-billet-accounts ask [ID] [OPTIONS]
```

`GET /v1/bank_billet_accounts/{id}/ask`

Request Collection Wallet approval

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Approval request completed successfully |
| `404` | Wallet not found |
| `422` | Wallet already approved |

## bank-billet-accounts set-default

Change Default Collection Wallet

```text
kobana banking bank-billet-accounts set-default [ID] [OPTIONS]
```

`PUT /v1/bank_billet_accounts/{id}/set_default`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `default` | `boolean` | yes | Sets the default Wallet (Default: true) or remove from default (Default: false) |

More fields and nested objects: `kobana schema banking.bank-billet-accounts.set-default`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Wallet set as default successfully |
| `404` | Wallet not found |
| `422` | Unprocessable Entity |

## bank-billet-accounts validate

Validate Collection Wallet

```text
kobana banking bank-billet-accounts validate [ID] [OPTIONS]
```

`PUT /v1/bank_billet_accounts/{id}/validate`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `homologation_amount` | `string` | yes | Amount received by the billet (R$) Ex.: 1.87 |

More fields and nested objects: `kobana schema banking.bank-billet-accounts.validate`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Wallet approved successfully |
| `404` | Wallet not found |
| `422` | Unprocessable Entity |
