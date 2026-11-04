---
id: nfes
title: "kobana billing nfes"
sidebar_label: "nfes"
---

# kobana billing nfes


Commands of `kobana billing nfes`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`nfes bulk create`](#nfes-bulk-create) | `POST /v1/nfes/bulk` | Bulk update NF-e |
| [`nfes bulk delete`](#nfes-bulk-delete) | `DELETE /v1/nfes/bulk` | Bulk delete NF-e |
| [`nfes request-tries list`](#nfes-request-tries-list) | `GET /v1/nfes/request-tries` | List request retries (organization) |
| [`nfes request-tries get`](#nfes-request-tries-get) | `GET /v1/nfes/{id}/request-tries` | List NF-e request retries |
| [`nfes requests get`](#nfes-requests-get) | `GET /v1/nfes/{id}/requests/{request_id}` | Get external request |
| [`nfes requests list`](#nfes-requests-list) | `GET /v1/nfes/{id}/requests` | List NF-e external requests |
| [`nfes list`](#nfes-list) | `GET /v1/nfes` | List NF-e |
| [`nfes create`](#nfes-create) | `POST /v1/nfes` | Create draft NF-e |
| [`nfes update`](#nfes-update) | `PUT /v1/nfes` | Upsert NFe |
| [`nfes fetch-barueri`](#nfes-fetch-barueri) | `POST /v1/nfes/fetch-barueri` | Fetch NFS-e from Barueri |
| [`nfes fetch-from-sefaz`](#nfes-fetch-from-sefaz) | `POST /v1/nfes/fetch-from-sefaz` | Fetch NFS-e from ADN (SEFAZ) |
| [`nfes import-barueri`](#nfes-import-barueri) | `POST /v1/nfes/import-barueri` | Import Barueri NFS-e |
| [`nfes sync`](#nfes-sync) | `POST /v1/nfes/sync` | Sync organization NF-e |
| [`nfes get`](#nfes-get) | `GET /v1/nfes/{id}` | Get NF-e |
| [`nfes delete`](#nfes-delete) | `DELETE /v1/nfes/{id}` | Delete NF-e |
| [`nfes cancel`](#nfes-cancel) | `POST /v1/nfes/{id}/cancel` | Cancel NF-e |
| [`nfes issue`](#nfes-issue) | `POST /v1/nfes/{id}/issue` | Issue NF-e |
| [`nfes pdf`](#nfes-pdf) | `GET /v1/nfes/{id}/pdf` | Download NF-e PDF |
| [`nfes retry-cancel`](#nfes-retry-cancel) | `POST /v1/nfes/{id}/retry-cancel` | Retry NF-e cancellation |
| [`nfes validate-barueri`](#nfes-validate-barueri) | `POST /v1/nfes/{id}/validate-barueri` | Validate Barueri NFS-e |
| [`nfes xml`](#nfes-xml) | `GET /v1/nfes/{id}/xml` | Download NF-e XML |

## nfes bulk create

Bulk update NF-e

```text
kobana billing nfes bulk create [OPTIONS]
```

`POST /v1/nfes/bulk`

Updates multiple NF-e at once. Currently supports linking or unlinking an invoice (`invoice_id`) on up to 100 NF-e per request.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `nfe_ids` | `array<string (uuid)>` | yes | UUIDs of the NF-e to update. Minimum 1. |
| `invoice_id` | `string (uuid)` | yes | UUID of the invoice to link. Send `null` to unlink. |
| `invoice_withholding` | `string` | no | What to do when the NF-e retention differs from the invoice retention: `block` (default) refuses with 409 `NFE_WITHHOLDING_MISMATCH`; `update_invoice` updates the invoice with the NF-e retention. Values: `block`, `update_invoice`. |

More fields and nested objects: `kobana schema billing.nfes.bulk.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Number of NF-e updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | The NF-e retention differs from the linked invoice retention (`NFE_WITHHOLDING_MISMATCH`). `details` carries both sides; retry with `invoice_withholding: "update_invoice"` to update the invoice. |
| `422` | Request body failed validation. |

## nfes bulk delete

Bulk delete NF-e

```text
kobana billing nfes bulk delete [OPTIONS]
```

`DELETE /v1/nfes/bulk`

Deletes up to 100 NF-e at once. All must be in `draft` status and belong to the current organization. Requires the `delete` access level. Replaces `POST /v1/nfes/bulk-delete`, which was removed.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | UUIDs of the NF-e to delete. Minimum 1, maximum 100. |

More fields and nested objects: `kobana schema billing.nfes.bulk.delete`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Number of NF-e deleted. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes request-tries list

List request retries (organization)

```text
kobana billing nfes request-tries list [OPTIONS]
```

`GET /v1/nfes/request-tries`

Lists every NF-e provider request attempt for the organization, paginated and ordered by descending date.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of attempts. |
| `404` | Resource not found in the current organization. |

## nfes request-tries get

List NF-e request retries

```text
kobana billing nfes request-tries get [ID] [OPTIONS]
```

`GET /v1/nfes/{id}/request-tries`

Lists the attempts (`NfeRequestTry`) made to the provider for this NF-e — one entry per call, with endpoint, HTTP status, duration and error messages.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of attempts. |
| `404` | Resource not found in the current organization. |

## nfes requests get

Get external request

```text
kobana billing nfes requests get [ID] [REQUEST_ID] [OPTIONS]
```

`GET /v1/nfes/{id}/requests/{request_id}`

**Path arguments:** `id`, `request_id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | External request detail, including headers and bodies. |
| `404` | Resource not found in the current organization. |

## nfes requests list

List NF-e external requests

```text
kobana billing nfes requests list [ID] [OPTIONS]
```

`GET /v1/nfes/{id}/requests`

Lists captured HTTP requests for this NF-e (provider calls and incoming webhooks), paginated. Use `direction` to filter between `outbound` and `inbound`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `direction` | `string` | no | Filter requests by direction: `outbound` (outgoing calls) or `inbound` (incoming webhooks). Values: `outbound`, `inbound`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of requests. |
| `404` | Resource not found in the current organization. |

## nfes list

List NF-e

```text
kobana billing nfes list [OPTIONS]
```

`GET /v1/nfes`

Returns a paginated list of the current organization's NF-e (electronic invoices).

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
| `billing_account_id` | `string` | no | Filter by `billing_account_id`. |
| `company_id` | `string` | no | Filter by issuing `company_id`. |
| `invoice_id` | `string` | no | Filter by linked `invoice_id`. |
| `payment_id` | `string` | no | Filter by linked `payment_id`. |
| `transaction_id` | `string` | no | Filter by linked `transaction_id`. |
| `status` | `string` | no | Filter by status: `draft`, `pending`, `processing`, `issued`, `pending_cancel`, `canceling`, `canceled` or `error`. Values: `draft`, `pending`, `processing`, `issued`, `pending_cancel`, `canceling`, `canceled`, `error`. |
| `invoice_filter` | `string` | no | Filter by invoice presence: `with` (with invoice only) or `without` (one-off only). Values: `with`, `without`. |
| `nfe_number` | `integer,null` | no | Filter by NF-e number. |
| `nfe_series` | `string` | no | Filter by NF-e series. |
| `nfe_key` | `string` | no | Filter by NF-e access key. |
| `external_id` | `string` | no | Filter by NF-e external identifier. |
| `lc116_code` | `string` | no | Filter by LC 116 code. |
| `service_code` | `string` | no | Filter by municipal service code. |
| `amount_min` | `number,null` | no | Minimum NF-e amount in BRL (filters `amount`). |
| `amount_max` | `number,null` | no | Maximum NF-e amount in BRL (filters `amount`). |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `issue_date_from` | `string` | no | Filter by start `issue_date` (inclusive, YYYY-MM-DD format). |
| `issue_date_to` | `string` | no | Filter by end `issue_date` (inclusive, YYYY-MM-DD format). |
| `cancelled_after` | `string` | no | Filter by start `cancelled_at` (inclusive, YYYY-MM-DD format). |
| `cancelled_before` | `string` | no | Filter by end `cancelled_at` (inclusive, YYYY-MM-DD format). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of NF-e. |
| `404` | Resource not found in the current organization. |

## nfes create

Create draft NF-e

```text
kobana billing nfes create [OPTIONS]
```

`POST /v1/nfes`

Creates a draft (`draft`) NF-e for later issuance via `POST /nfes/{id}/issue`. Accepts taker data (`customer`) and optional tax rates.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | yes | Billing account (taker) UUID. **Required**. |
| `amount` | `number` | yes | NF-e amount in BRL (float, up to 4 decimal places). **Required**, must be positive. |
| `customer` | `object` | yes | Taker data at issuance time. **Required**. |
| `company_id` | `string (uuid)` | no | Issuing company (provider) UUID. Default: organization's default company. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `invoice_id` | `string (uuid)` | no | Linked invoice UUID (optional). |
| `transaction_id` | `string (uuid)` | no | Originating transaction UUID (optional). |
| `service_code` | `string` | no | Service code per the municipal table. |
| `service_description` | `string` | no | Free-form description of the service provided. |
| `lc116_code` | `string` | no | LC 116 code. |
| `cnae_code` | `string` | no | CNAE code. |
| `nbs_code` | `string` | no | NBS code. |
| `c_class_trib` | `string` | no | IBS/CBS Tax Classification (cClassTrib). |
| `ind_op` | `string` | no | IBS/CBS Operation Indicator (indOp). |
| `rps_number` | `integer` | no | RPS number (when numbering is managed by the application). |
| `rps_serial_number` | `string` | no | RPS series. |
| `taxes` | `object` | no | Tax rates (as decimals, e.g.: `0.05` for 5%). Falls back to company defaults when omitted. |
| `custom_metadata` | `object` | no | Custom fields (validated against the organization's schema). |

More fields and nested objects: `kobana schema billing.nfes.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Draft NF-e created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes update

Upsert NFe

```text
kobana billing nfes update [OPTIONS]
```

`PUT /v1/nfes`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | Billing account (taker) UUID. **Required**. |
| `company_id` | `string (uuid)` | no | Issuing company (provider) UUID. Default: organization's default company. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `invoice_id` | `string (uuid)` | no | Linked invoice UUID (optional). |
| `transaction_id` | `string (uuid)` | no | Originating transaction UUID (optional). |
| `amount` | `number` | no | NF-e amount in BRL (float, up to 4 decimal places). **Required**, must be positive. |
| `service_code` | `string` | no | Service code per the municipal table. |
| `service_description` | `string` | no | Free-form description of the service provided. |
| `lc116_code` | `string` | no | LC 116 code. |
| `cnae_code` | `string` | no | CNAE code. |
| `nbs_code` | `string` | no | NBS code. |
| `c_class_trib` | `string` | no | IBS/CBS Tax Classification (cClassTrib). |
| `ind_op` | `string` | no | IBS/CBS Operation Indicator (indOp). |
| `rps_number` | `integer` | no | RPS number (when numbering is managed by the application). |
| `rps_serial_number` | `string` | no | RPS series. |
| `customer` | `object` | no | Taker data at issuance time. **Required**. |
| `taxes` | `object` | no | Tax rates (as decimals, e.g.: `0.05` for 5%). Falls back to company defaults when omitted. |
| `custom_metadata` | `object` | no | Custom fields (validated against the organization's schema). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.nfes.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes fetch-barueri

Fetch NFS-e from Barueri

```text
kobana billing nfes fetch-barueri [OPTIONS]
```

`POST /v1/nfes/fetch-barueri`

Fetches an NFS-e from the Barueri municipal endpoint using the `codigo_autenticidade` printed on the DANFSE and the taker's CPF/CNPJ. Returns the raw XML and, when found, the NF-e already present in the organization.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `codigo_autenticidade` | `string` | yes | Authenticity code printed on the NFS-e DANFSE. |
| `cnpj_tomador` | `string` | yes | Taker's CPF or CNPJ (digits only or formatted). |
| `ambiente` | `string` | no | Municipal environment: `production` or `homologation`. Default: `production`. Values: `production`, `homologation`. |

More fields and nested objects: `kobana schema billing.nfes.fetch-barueri`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Fetch result with XML and reference to the existing NF-e, when present. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes fetch-from-sefaz

Fetch NFS-e from ADN (SEFAZ)

```text
kobana billing nfes fetch-from-sefaz [OPTIONS]
```

`POST /v1/nfes/fetch-from-sefaz`

Fetches an NFS-e from the Ambiente de Dados Nacional (ADN) using a 50-digit access key and a previously registered digital certificate (mTLS authentication).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `access_key` | `string` | yes | NFS-e access key (50 digits). |
| `certificate_id` | `string` | yes | UUID of the digital certificate registered in the organization (used for mTLS authentication). |
| `ambiente` | `anyOf` | no | SEFAZ environment: `1` (production) or `2` (homologation). Default: `2`. |

More fields and nested objects: `kobana schema billing.nfes.fetch-from-sefaz`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | ADN fetch result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes import-barueri

Import Barueri NFS-e

```text
kobana billing nfes import-barueri [OPTIONS]
```

`POST /v1/nfes/import-barueri`

Imports a Barueri NFS-e into the database. Accepts a raw XML (returned by `POST /nfes/fetch-barueri`) or the `codigo_autenticidade` + `cnpj_tomador` to re-fetch before importing. Creates the taker `BillingAccount` when it does not yet exist. Requires the `import` access level — `write` is not enough.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `codigo_autenticidade` | `string` | yes | Authenticity code printed on the NFS-e DANFSE. |
| `cnpj_tomador` | `string` | yes | Taker's CPF or CNPJ (digits only or formatted). |
| `xml` | `string` | no | Raw XML returned by `POST /nfes/fetch-barueri`. Optional — when omitted, the NFS-e is re-fetched with `codigo_autenticidade` + `cnpj_tomador`. |
| `ambiente` | `string` | no | Municipal environment: `production` or `homologation`. Default: `production`. Values: `production`, `homologation`. |

More fields and nested objects: `kobana schema billing.nfes.import-barueri`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Import result with `created`/`updated` flags and the persisted NF-e. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes sync

Sync organization NF-e

```text
kobana billing nfes sync [OPTIONS]
```

`POST /v1/nfes/sync`

Triggers a background job to sync every organization NF-e with the provider (updates status, downloads pending PDFs/XMLs). Returns the `job_id` for polling. Refuses if a sync is already running.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Sync queued. |
| `404` | Resource not found in the current organization. |

## nfes get

Get NF-e

```text
kobana billing nfes get [ID] [OPTIONS]
```

`GET /v1/nfes/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | NF-e detail. |
| `404` | Resource not found in the current organization. |

## nfes delete

Delete NF-e

```text
kobana billing nfes delete [ID] [OPTIONS]
```

`DELETE /v1/nfes/{id}`

Deletes an NF-e. Allowed only for `draft`, `pending`, `processing`, `error` or `canceled` statuses. `issued` NF-e must be canceled via `POST /nfes/{id}/cancel`.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | NF-e deleted. |
| `404` | Resource not found in the current organization. |

## nfes cancel

Cancel NF-e

```text
kobana billing nfes cancel [ID] [OPTIONS]
```

`POST /v1/nfes/{id}/cancel`

Cancels an issued NF-e. Enqueues the cancellation asynchronously with the provider. The `reason` must be at least 15 characters and is recorded in the audit log.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `reason` | `string` | yes | Cancellation reason. **Minimum 15 characters**. Recorded in the audit log and sent to the provider. |

More fields and nested objects: `kobana schema billing.nfes.cancel`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Cancellation queued. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes issue

Issue NF-e

```text
kobana billing nfes issue [ID] [OPTIONS]
```

`POST /v1/nfes/{id}/issue`

Issues a draft (`draft`) NF-e or retries one in `pending`/`error`. Sends the request to the provider configured for the organization.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | NF-e queued/issued. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes pdf

Download NF-e PDF

```text
kobana billing nfes pdf [ID] [OPTIONS]
```

`GET /v1/nfes/{id}/pdf`

Downloads the NF-e PDF (DANFE) as a binary attachment (`application/pdf`). Served from the database when available; otherwise fetched from the provider.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | NF-e PDF. |
| `404` | Resource not found in the current organization. |

## nfes retry-cancel

Retry NF-e cancellation

```text
kobana billing nfes retry-cancel [ID] [OPTIONS]
```

`POST /v1/nfes/{id}/retry-cancel`

Retries cancellation of an NF-e stuck in `pending_cancel`, `canceling` or `error`. Re-enqueues the cancellation job.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Cancellation retry queued. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## nfes validate-barueri

Validate Barueri NFS-e

```text
kobana billing nfes validate-barueri [ID] [OPTIONS]
```

`POST /v1/nfes/{id}/validate-barueri`

Re-fetches the NFS-e from the Barueri municipality and compares it to the local record, returning the diff list and the fresh XML for reapplication via `POST /nfes/import-barueri`.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Validation result with differences and current XML. |
| `404` | Resource not found in the current organization. |

## nfes xml

Download NF-e XML

```text
kobana billing nfes xml [ID] [OPTIONS]
```

`GET /v1/nfes/{id}/xml`

Downloads the NF-e XML as a binary attachment (`application/xml`). Served from the database when available; otherwise fetched from the provider.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | NF-e XML. |
| `404` | Resource not found in the current organization. |
