---
id: tax-rules
title: "kobana billing tax-rules"
sidebar_label: "tax-rules"
---

# kobana billing tax-rules


Commands of `kobana billing tax-rules`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`tax-rules list`](#tax-rules-list) | `GET /v1/tax_rules` | List tax rules |
| [`tax-rules create`](#tax-rules-create) | `POST /v1/tax_rules` | Create tax rule |
| [`tax-rules update`](#tax-rules-update) | `PUT /v1/tax_rules` | Upsert tax rule |
| [`tax-rules get`](#tax-rules-get) | `GET /v1/tax_rules/{id}` | Get tax rule |
| [`tax-rules delete`](#tax-rules-delete) | `DELETE /v1/tax_rules/{id}` | Delete tax rule |

## tax-rules list

List tax rules

```text
kobana billing tax-rules list [OPTIONS]
```

`GET /v1/tax_rules`

Returns a paginated list of the current organization's tax rules. Use the `scope`, `company_id`, `service_item_id`, `billing_account_id` and `is_active` filters to narrow the result.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `scope` | `string` | no | Filter by scope: `global`, `organization`, `company`, `service`, `customer` or `municipality`. Values: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `company_id` | `string` | no | Filter by rules of a specific company. |
| `service_item_id` | `string` | no | Filter by rules of a specific service item. |
| `billing_account_id` | `string` | no | Filter by rules of a specific billing account. |
| `is_active` | `string` | no | When `true`, returns only active rules; when `false`, only inactive ones. Values: `true`, `false`. |
| `search` | `string` | no | Text search across `name` and `description`. |
| `customer_tax_type` | `string` | no | Filter by customer tax type: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` or `entidade_publica`. |
| `operation_type` | `string` | no | Filter by operation type: `interno` (same municipality) or `externo` (different municipality). |
| `municipality` | `string` | no | Filter by IBGE municipality code (7 digits). |
| `priority_from` | `integer,null` | no | Minimum priority (inclusive) to filter rules. |
| `priority_to` | `integer,null` | no | Maximum priority (inclusive) to filter rules. |
| `effective_from_from` | `string` | no | Returns rules whose effective-from date is on or after this value (ISO 8601). |
| `effective_from_to` | `string` | no | Returns rules whose effective-from date is on or before this value (ISO 8601). |
| `effective_to_from` | `string` | no | Returns rules whose effective-to date is on or after this value (ISO 8601). |
| `effective_to_to` | `string` | no | Returns rules whose effective-to date is on or before this value (ISO 8601). |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of tax rules. |
| `404` | Resource not found in the current organization. |

## tax-rules create

Create tax rule

```text
kobana billing tax-rules create [OPTIONS]
```

`POST /v1/tax_rules`

Creates a new tax rule along with its items (per-tax rates). The `scope` defines the applicability level and `priority` defines precedence — higher priority overrides lower. At least one entry in `items` is required.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Human-readable rule name. |
| `scope` | `string` | yes | Rule scope: `global`, `organization`, `company`, `service`, `customer` or `municipality`. Values: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `items` | `array<object>` | yes | List of items (per-tax rates). At least 1 item is required. |
| `description` | `string` | no | Optional rule description. |
| `priority` | `integer` | no | Rule priority. Higher value overrides lower-priority rules. Default: `0`. |
| `company_id` | `string (uuid)` | no | Company UUID when `scope = company`. The company must belong to the current organization. |
| `service_item_id` | `string (uuid)` | no | Service item UUID when `scope = service`. The item must belong to the current organization. |
| `billing_account_id` | `string (uuid)` | no | Billing account UUID when `scope = customer`. The account must belong to the current organization. |
| `customer_tax_type` | `string` | no | Customer tax type: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` or `entidade_publica`. Values: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante`, `entidade_publica`. |
| `municipality` | `string` | no | IBGE municipality code (7 digits) when `scope = municipality`. |
| `operation_type` | `string` | no | Operation type: `interno` (same municipality) or `externo` (different municipality). Values: `interno`, `externo`. |
| `effective_from` | `string (date-time)` | no | ISO 8601 date/time when the rule becomes effective. |
| `effective_to` | `string (date-time)` | no | ISO 8601 date/time when the rule stops being effective. `null` means open-ended. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.tax-rules.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Tax rule created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## tax-rules update

Upsert tax rule

```text
kobana billing tax-rules update [OPTIONS]
```

`PUT /v1/tax_rules`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Human-readable rule name. |
| `description` | `string` | no | Optional rule description. |
| `scope` | `string` | no | Rule scope: `global`, `organization`, `company`, `service`, `customer` or `municipality`. Values: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `priority` | `integer` | no | Rule priority. Higher value overrides lower-priority rules. Default: `0`. |
| `company_id` | `string (uuid)` | no | Company UUID when `scope = company`. The company must belong to the current organization. |
| `service_item_id` | `string (uuid)` | no | Service item UUID when `scope = service`. The item must belong to the current organization. |
| `billing_account_id` | `string (uuid)` | no | Billing account UUID when `scope = customer`. The account must belong to the current organization. |
| `customer_tax_type` | `string` | no | Customer tax type: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` or `entidade_publica`. Values: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante`, `entidade_publica`. |
| `municipality` | `string` | no | IBGE municipality code (7 digits) when `scope = municipality`. |
| `operation_type` | `string` | no | Operation type: `interno` (same municipality) or `externo` (different municipality). Values: `interno`, `externo`. |
| `effective_from` | `string (date-time)` | no | ISO 8601 date/time when the rule becomes effective. |
| `effective_to` | `string (date-time)` | no | ISO 8601 date/time when the rule stops being effective. `null` means open-ended. |
| `items` | `array<object>` | no | List of items (per-tax rates). At least 1 item is required. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.tax-rules.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## tax-rules get

Get tax rule

```text
kobana billing tax-rules get [ID] [OPTIONS]
```

`GET /v1/tax_rules/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Tax rule detail. |
| `404` | Resource not found in the current organization. |

## tax-rules delete

Delete tax rule

```text
kobana billing tax-rules delete [ID] [OPTIONS]
```

`DELETE /v1/tax_rules/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Tax rule removed. |
| `404` | Resource not found in the current organization. |
