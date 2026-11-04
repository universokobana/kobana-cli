---
id: withholdings
title: "kobana billing withholdings"
sidebar_label: "withholdings"
---

# kobana billing withholdings


Commands of `kobana billing withholdings`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`withholdings list`](#withholdings-list) | `GET /v1/withholdings` | List withholdings |
| [`withholdings create`](#withholdings-create) | `POST /v1/withholdings` | Create a withholding |
| [`withholdings update`](#withholdings-update) | `PUT /v1/withholdings` | Create or update a withholding |
| [`withholdings convert-to-credits`](#withholdings-convert-to-credits) | `POST /v1/withholdings/convert-to-credits` | Convert pending withholdings to credits (bulk) |
| [`withholdings get`](#withholdings-get) | `GET /v1/withholdings/{id}` | Get withholding |
| [`withholdings delete`](#withholdings-delete) | `DELETE /v1/withholdings/{id}` | Delete a withholding |
| [`withholdings convert-to-credit`](#withholdings-convert-to-credit) | `POST /v1/withholdings/{id}/convert-to-credit` | Convert withholding to credit |

## withholdings list

List withholdings

```text
kobana billing withholdings list [OPTIONS]
```

`GET /v1/withholdings`

Returns a paginated list of the current organization's tax withholdings. Supports filters by tax type, status, withholder, invoice, NF-e and competence period.

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
| `tax_type` | `string` | no | Filter by tax type: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs` or `ibs`. Values: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `status` | `string` | no | Filter by status: `retained`, `compensated` or `partially_compensated`. Values: `retained`, `compensated`, `partially_compensated`. |
| `withholder_id` | `string` | no | Filter by withholdings of a specific withholder (billing account). |
| `invoice_id` | `string` | no | Filter by withholdings associated with an invoice. |
| `nfe_id` | `string` | no | Filter by withholdings associated with an NF-e. |
| `competence_year` | `integer` | no | Filter by competence year (2000–3000). |
| `competence_month` | `integer` | no | Filter by competence month (1–12). |
| `amount_min` | `number,null` | no | Minimum withheld amount in BRL (e.g.: `10.5` for R$ 10.50). |
| `amount_max` | `number,null` | no | Maximum withheld amount in BRL (e.g.: `500.0` for R$ 500.00). |
| `base_min` | `number,null` | no | Minimum calculation base in BRL. |
| `base_max` | `number,null` | no | Maximum calculation base in BRL. |
| `compensated_min` | `number,null` | no | Minimum compensated amount in BRL. |
| `compensated_max` | `number,null` | no | Maximum compensated amount in BRL. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of withholdings. |
| `404` | Resource not found in the current organization. |

## withholdings create

Create a withholding

```text
kobana billing withholdings create [OPTIONS]
```

`POST /v1/withholdings`

Records a withholding at source for the given withholder. `base` and `amount` are values in BRL (float, up to 4 decimal places).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `tax_type` | `string` | yes | Withheld tax. Values: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `base` | `number` | yes | Calculation base, in BRL (float, up to 4 decimal places). |
| `rate` | `number` | yes | Applied rate, as a percentage. |
| `amount` | `number` | yes | Withheld amount, in BRL (float, up to 4 decimal places). |
| `withholder_id` | `string (uuid)` | yes | Billing account that performed the withholding. |
| `competence_month` | `integer` | yes | Competence month (1–12). |
| `competence_year` | `integer` | yes | Competence year. |
| `invoice_id` | `string (uuid)` | no | Originating invoice, when there is one. |
| `nfe_id` | `string (uuid)` | no | Related invoice document (NF-e), when there is one. |
| `metadata` | `object` | no |  |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema billing.withholdings.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Withholding created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## withholdings update

Create or update a withholding

```text
kobana billing withholdings update [OPTIONS]
```

`PUT /v1/withholdings`

Looks the withholding up by `id`, `public_id` or `external_id`. If it is not found, creates it from the full body and returns `201`. **If it is found, only `competence_month`, `competence_year`, `metadata`, `external_id` and `custom_metadata` are applied**: `base`, `rate`, `amount` and `withholder_id` are immutable once recorded and are ignored in the update body.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `tax_type` | `string` | no | Withheld tax. Values: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `base` | `number` | no | Calculation base, in BRL (float, up to 4 decimal places). |
| `rate` | `number` | no | Applied rate, as a percentage. |
| `amount` | `number` | no | Withheld amount, in BRL (float, up to 4 decimal places). |
| `withholder_id` | `string (uuid)` | no | Billing account that performed the withholding. |
| `competence_month` | `integer` | no | Competence month (1–12). |
| `competence_year` | `integer` | no | Competence year. |
| `invoice_id` | `string (uuid)` | no | Originating invoice, when there is one. |
| `nfe_id` | `string (uuid)` | no | Related invoice document (NF-e), when there is one. |
| `metadata` | `object` | no |  |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no |  |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.withholdings.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## withholdings convert-to-credits

Convert pending withholdings to credits (bulk)

```text
kobana billing withholdings convert-to-credits [OPTIONS]
```

`POST /v1/withholdings/convert-to-credits`

Converts all of the organization's pending withholdings — optionally narrowed by `tax_type` and/or competence period — into credits. Returns the list of generated credits, the total count converted and the total amount in BRL.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `tax_type` | `string` | no | Restricts conversion to a specific tax type (e.g.: `irrf`, `pis`, `cofins`). Values: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `competence_year` | `integer` | no | Restricts conversion to the given competence year. |
| `competence_month` | `integer` | no | Restricts conversion to the given competence month (1–12). |

More fields and nested objects: `kobana schema billing.withholdings.convert-to-credits`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Batch of withholdings converted to credits. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## withholdings get

Get withholding

```text
kobana billing withholdings get [ID] [OPTIONS]
```

`GET /v1/withholdings/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Withholding detail. |
| `404` | Resource not found in the current organization. |

## withholdings delete

Delete a withholding

```text
kobana billing withholdings delete [ID] [OPTIONS]
```

`DELETE /v1/withholdings/{id}`

Refuses the deletion when the withholding has already been compensated, in full or in part.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Withholding deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## withholdings convert-to-credit

Convert withholding to credit

```text
kobana billing withholdings convert-to-credit [ID] [OPTIONS]
```

`POST /v1/withholdings/{id}/convert-to-credit`

Converts a withholding (fully or partially) into a credit on the billing account derived from the associated invoice — or, when absent, from the withholder itself. When `amount` is omitted, converts the withholding's pending balance.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | no | Amount to convert in BRL (float, up to 4 decimal places). Default: pending balance of the withholding. |

More fields and nested objects: `kobana schema billing.withholdings.convert-to-credit`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Credit generated from the withholding. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
