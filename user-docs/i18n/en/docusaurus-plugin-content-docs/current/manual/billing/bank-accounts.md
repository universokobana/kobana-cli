---
id: bank-accounts
title: "kobana billing bank-accounts"
sidebar_label: "bank-accounts"
---

# kobana billing bank-accounts


Commands of `kobana billing bank-accounts`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`bank-accounts list`](#bank-accounts-list) | `GET /v1/bank_accounts` | List bank accounts |
| [`bank-accounts create`](#bank-accounts-create) | `POST /v1/bank_accounts` | Create a bank account |
| [`bank-accounts get`](#bank-accounts-get) | `GET /v1/bank_accounts/{id}` | Get a bank account |
| [`bank-accounts update`](#bank-accounts-update) | `PATCH /v1/bank_accounts/{id}` | Update a bank account |
| [`bank-accounts delete`](#bank-accounts-delete) | `DELETE /v1/bank_accounts/{id}` | Delete a bank account |

## bank-accounts list

List bank accounts

```text
kobana billing bank-accounts list [OPTIONS]
```

`GET /v1/bank_accounts`

The organization bank accounts customers transfer to on a `bank_transfer` charge (`POST /invoices/{id}/pay`).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `active` | `string` | no | Filter by active (`true`) or inactive (`false`) accounts. Values: `true`, `false`. |
| `bank_id` | `string` | no | Filter by bank (UUID from `/banks`). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of bank accounts. |
| `404` | Resource not found in the current organization. |

## bank-accounts create

Create a bank account

```text
kobana billing bank-accounts create [OPTIONS]
```

`POST /v1/bank_accounts`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Account nickname, for the operator. |
| `bank_id` | `string (uuid)` | yes | Bank UUID (see `/banks`). |
| `agency` | `string` | yes | Branch number, digits only. |
| `account_number` | `string` | yes | Account number, digits only. |
| `type` | `string` | no | `checking` or `savings`. Values: `checking`, `savings`. |
| `agency_digit` | `string` | no | Branch check digit, when there is one. |
| `account_digit` | `string` | no | Account check digit, when there is one. |
| `holder_name` | `string` | no | Holder shown to the customer. Empty uses the legal name of the invoice issuing company (or the organization). |
| `holder_document` | `string` | no | Holder CPF/CNPJ, digits only. Empty uses the document of the issuing company (or the organization). |
| `company_id` | `string (uuid)` | no | Issuing company it belongs to (optional). |
| `active` | `boolean` | no | Only active destinations can be chosen for a new charge. |

More fields and nested objects: `kobana schema billing.bank-accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Bank account created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## bank-accounts get

Get a bank account

```text
kobana billing bank-accounts get [ID] [OPTIONS]
```

`GET /v1/bank_accounts/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bank account found. |
| `404` | Resource not found in the current organization. |

## bank-accounts update

Update a bank account

```text
kobana billing bank-accounts update [ID] [OPTIONS]
```

`PATCH /v1/bank_accounts/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Account nickname, for the operator. |
| `bank_id` | `string (uuid)` | no | Bank UUID (see `/banks`). |
| `type` | `string` | no | `checking` or `savings`. Values: `checking`, `savings`. |
| `agency` | `string` | no | Branch number, digits only. |
| `agency_digit` | `string` | no | Branch check digit, when there is one. |
| `account_number` | `string` | no | Account number, digits only. |
| `account_digit` | `string` | no | Account check digit, when there is one. |
| `holder_name` | `string` | no | Holder shown to the customer. Empty uses the legal name of the invoice issuing company (or the organization). |
| `holder_document` | `string` | no | Holder CPF/CNPJ, digits only. Empty uses the document of the issuing company (or the organization). |
| `company_id` | `string (uuid)` | no | Issuing company it belongs to (optional). |
| `active` | `boolean` | no | Only active destinations can be chosen for a new charge. |

More fields and nested objects: `kobana schema billing.bank-accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bank account updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## bank-accounts delete

Delete a bank account

```text
kobana billing bank-accounts delete [ID] [OPTIONS]
```

`DELETE /v1/bank_accounts/{id}`

Soft delete. Charges already sent keep showing the details the customer received.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bank account deleted. |
| `404` | Resource not found in the current organization. |
