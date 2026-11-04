---
id: companies
title: "kobana billing companies"
sidebar_label: "companies"
---

# kobana billing companies


Commands of `kobana billing companies`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`companies fiscal`](#companies-fiscal) | `GET /v1/companies/{id}/fiscal` | Get fiscal profile |
| [`companies fiscal update`](#companies-fiscal-update) | `PATCH /v1/companies/{id}/fiscal` | Update fiscal profile |
| [`companies fiscal-profile`](#companies-fiscal-profile) | `GET /v1/companies/{id}/fiscal-profile` | Get fiscal profile (alias) |
| [`companies fiscal-profile update`](#companies-fiscal-profile-update) | `PATCH /v1/companies/{id}/fiscal-profile` | Update fiscal profile (alias) |
| [`companies matriz`](#companies-matriz) | `GET /v1/companies/matriz` | List head office companies |
| [`companies list`](#companies-list) | `GET /v1/companies` | List companies |
| [`companies create`](#companies-create) | `POST /v1/companies` | Create company |
| [`companies update`](#companies-update) | `PUT /v1/companies` | Upsert company |
| [`companies get`](#companies-get) | `GET /v1/companies/{id}` | Get company |
| [`companies delete`](#companies-delete) | `DELETE /v1/companies/{id}` | Delete company |
| [`companies deactivate`](#companies-deactivate) | `POST /v1/companies/{id}/deactivate` | Deactivate company |
| [`companies reactivate`](#companies-reactivate) | `POST /v1/companies/{id}/reactivate` | Reactivate company |
| [`companies set-default`](#companies-set-default) | `POST /v1/companies/{id}/set-default` | Set default company |

## companies fiscal

Get fiscal profile

```text
kobana billing companies fiscal [ID] [OPTIONS] [COMMAND]
```

`GET /v1/companies/{id}/fiscal`

Returns the company's fiscal profile, or `null` when it has not been configured yet.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company fiscal profile (or `null`). |
| `404` | Resource not found in the current organization. |

## companies fiscal update

Update fiscal profile

```text
kobana billing companies fiscal update [ID] [OPTIONS]
```

`PATCH /v1/companies/{id}/fiscal`

Creates or updates (upsert) the company fiscal profile. Includes taxation regime, rates (ISS, PIS, COFINS, CSLL, IRPJ) and CBS/IBS settings.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `taxation_regime` | `string` | yes | Taxation regime: `simple_national`, `presumed_profit`, `real_profit` or `mei`. Values: `simple_national`, `presumed_profit`, `real_profit`, `mei`. |
| `simples_annex` | `string` | no | Simples Nacional annex: `annex_iii`, `annex_iv` or `annex_v`. Values: `annex_iii`, `annex_iv`, `annex_v`. |
| `rbt12` | `number` | no | Gross revenue accumulated over the last 12 months, in BRL (float, up to 4 decimal places). |
| `factor_r` | `number` | no | Factor R (payroll ÷ revenue), used for Simples Nacional bracket classification. |
| `cpp_inside_das` | `boolean` | no | Whether CPP (employer social security contribution) is bundled into DAS. |
| `presumption_rate` | `number` | no | Presumption rate (presumed profit), as decimal (e.g.: 0.32). |
| `iss_municipality` | `string` | no | Municipality where ISS is levied. |
| `iss_rate` | `number` | no | ISS rate, as decimal. |
| `pis_rate` | `number` | no | PIS rate, as decimal. |
| `cofins_rate` | `number` | no | COFINS rate, as decimal. |
| `csll_rate` | `number` | no | CSLL rate, as decimal. |
| `irpj_rate` | `number` | no | IRPJ rate, as decimal. |
| `cbs_ibs_enabled` | `boolean` | no | Enables the CBS/IBS fields (Brazilian tax reform). |
| `cbs_rate` | `number` | no | CBS rate, as decimal. |
| `ibs_rate` | `number` | no | IBS rate, as decimal. |
| `cbs_ibs_mode` | `string` | no | CBS/IBS calculation mode: `informative` (display only) or `compositive` (composes the price). Values: `informative`, `compositive`. |

More fields and nested objects: `kobana schema billing.companies.fiscal.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Fiscal profile updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## companies fiscal-profile

Get fiscal profile (alias)

```text
kobana billing companies fiscal-profile [ID] [OPTIONS] [COMMAND]
```

`GET /v1/companies/{id}/fiscal-profile`

Alias of `GET /companies/{id}/fiscal` — same representation and behaviour.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company fiscal profile (or `null`). |
| `404` | Resource not found in the current organization. |

## companies fiscal-profile update

Update fiscal profile (alias)

```text
kobana billing companies fiscal-profile update [ID] [OPTIONS]
```

`PATCH /v1/companies/{id}/fiscal-profile`

Alias of `PATCH /companies/{id}/fiscal` — same representation and behaviour.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `taxation_regime` | `string` | yes | Taxation regime: `simple_national`, `presumed_profit`, `real_profit` or `mei`. Values: `simple_national`, `presumed_profit`, `real_profit`, `mei`. |
| `simples_annex` | `string` | no | Simples Nacional annex: `annex_iii`, `annex_iv` or `annex_v`. Values: `annex_iii`, `annex_iv`, `annex_v`. |
| `rbt12` | `number` | no | Gross revenue accumulated over the last 12 months, in BRL (float, up to 4 decimal places). |
| `factor_r` | `number` | no | Factor R (payroll ÷ revenue), used for Simples Nacional bracket classification. |
| `cpp_inside_das` | `boolean` | no | Whether CPP (employer social security contribution) is bundled into DAS. |
| `presumption_rate` | `number` | no | Presumption rate (presumed profit), as decimal (e.g.: 0.32). |
| `iss_municipality` | `string` | no | Municipality where ISS is levied. |
| `iss_rate` | `number` | no | ISS rate, as decimal. |
| `pis_rate` | `number` | no | PIS rate, as decimal. |
| `cofins_rate` | `number` | no | COFINS rate, as decimal. |
| `csll_rate` | `number` | no | CSLL rate, as decimal. |
| `irpj_rate` | `number` | no | IRPJ rate, as decimal. |
| `cbs_ibs_enabled` | `boolean` | no | Enables the CBS/IBS fields (Brazilian tax reform). |
| `cbs_rate` | `number` | no | CBS rate, as decimal. |
| `ibs_rate` | `number` | no | IBS rate, as decimal. |
| `cbs_ibs_mode` | `string` | no | CBS/IBS calculation mode: `informative` (display only) or `compositive` (composes the price). Values: `informative`, `compositive`. |

More fields and nested objects: `kobana schema billing.companies.fiscal-profile.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Fiscal profile updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## companies matriz

List head office companies

```text
kobana billing companies matriz [OPTIONS]
```

`GET /v1/companies/matriz`

Lists the active head office companies (without `parent_company_id`) of the organization. Useful when selecting the parent company before creating a branch. Only CNPJ companies appear here.

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of head office companies. |
| `404` | Resource not found in the current organization. |

## companies list

List companies

```text
kobana billing companies list [OPTIONS]
```

`GET /v1/companies`

Returns a paginated list of the current organization's companies (head offices and branches).

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
| `status` | `string` | no | Filter by status: `active` or `inactive`. Values: `active`, `inactive`. |
| `search` | `string` | no | Free-text search by name, legal name, email or document number. |
| `is_default` | `string` | no | Filter only the company flagged as the organization default. Values: `true`, `false`. |
| `parent_company_id` | `string` | no | Filter branches by head office. Use the head office UUID to list its branches. |
| `email_layout_id` | `string` | no | Filter companies by the UUID of the associated email layout. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of companies. |
| `404` | Resource not found in the current organization. |

## companies create

Create company

```text
kobana billing companies create [OPTIONS]
```

`POST /v1/companies`

Creates a company (head office or branch). To create a branch, provide `parent_company_id` — the CNPJ must share the same prefix as the head office. Only CNPJ companies can have branches. The first company created in the organization is flagged as `is_default` automatically.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Trade name of the company. |
| `document_type` | `string` | yes | Document type: `cpf` or `cnpj`. Only `cnpj` can have branches. Values: `cpf`, `cnpj`. |
| `document_number` | `string` | yes | CPF or CNPJ (with or without mask — will be normalized to digits only). |
| `email` | `string (email)` | yes | Primary company email. |
| `parent_company_id` | `string (uuid)` | no | Head office UUID when creating a branch. Omit to create a head office. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `legal_name` | `string` | no | Legal name. |
| `phone` | `string` | no | Primary company phone. When provided, it is added to the `phones` array. |
| `addresses` | `array<object>` | no | List of company addresses. |
| `emails` | `array<object>` | no | List of additional emails. If omitted, the primary `email` is used. |
| `phones` | `array<object>` | no | List of company phones. |
| `websites` | `array<object>` | no | List of company websites. |
| `is_default` | `boolean` | no | When `true`, sets this company as the organization default. Only head offices can be default. |
| `custom_metadata` | `object` | no | Custom metadata (free-form key/value). |

More fields and nested objects: `kobana schema billing.companies.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Company created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## companies update

Upsert company

```text
kobana billing companies update [OPTIONS]
```

`PUT /v1/companies`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `parent_company_id` | `string (uuid)` | no | Head office UUID when creating a branch. Omit to create a head office. |
| `name` | `string` | no | Trade name of the company. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `legal_name` | `string` | no | Legal name. |
| `document_type` | `string` | no | Document type: `cpf` or `cnpj`. Only `cnpj` can have branches. Values: `cpf`, `cnpj`. |
| `document_number` | `string` | no | CPF or CNPJ (with or without mask — will be normalized to digits only). |
| `email` | `string (email)` | no | Primary company email. |
| `phone` | `string` | no | Primary company phone. When provided, it is added to the `phones` array. |
| `addresses` | `array<object>` | no | List of company addresses. |
| `emails` | `array<object>` | no | List of additional emails. If omitted, the primary `email` is used. |
| `phones` | `array<object>` | no | List of company phones. |
| `websites` | `array<object>` | no | List of company websites. |
| `is_default` | `boolean` | no | When `true`, sets this company as the organization default. Only head offices can be default. |
| `custom_metadata` | `object` | no | Custom metadata (free-form key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.companies.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## companies get

Get company

```text
kobana billing companies get [ID] [OPTIONS]
```

`GET /v1/companies/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company detail. |
| `404` | Resource not found in the current organization. |

## companies delete

Delete company

```text
kobana billing companies delete [ID] [OPTIONS]
```

`DELETE /v1/companies/{id}`

Deletes the company (soft delete: `deleted_at` — it disappears from every read; issued invoices and NFS-e keep citing it). Requires the `delete` access level. The only active company, or one with active or trialing subscriptions, cannot be deleted. Until October 2026 this route only deactivated the company — deactivating is `POST /v1/companies/{id}/deactivate`, reversible.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## companies deactivate

Deactivate company

```text
kobana billing companies deactivate [ID] [OPTIONS]
```

`POST /v1/companies/{id}/deactivate`

Deactivates the company. Cannot deactivate the only active company, nor a company that has active or trialing subscriptions. If the deactivated company was the default, another active company is promoted automatically.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company deactivated. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## companies reactivate

Reactivate company

```text
kobana billing companies reactivate [ID] [OPTIONS]
```

`POST /v1/companies/{id}/reactivate`

Reactivates a previously deactivated company. Returns **422** when the company is already active.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company reactivated. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## companies set-default

Set default company

```text
kobana billing companies set-default [ID] [OPTIONS]
```

`POST /v1/companies/{id}/set-default`

Marks the company as the organization's default. The previous default is unset in the same transaction.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Company set as default. |
| `404` | Resource not found in the current organization. |
