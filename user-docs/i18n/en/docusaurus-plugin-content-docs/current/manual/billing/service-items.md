---
id: service-items
title: "kobana billing service-items"
sidebar_label: "service-items"
---

# kobana billing service-items


Commands of `kobana billing service-items`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`service-items list`](#service-items-list) | `GET /v1/service_items` | List service items |
| [`service-items create`](#service-items-create) | `POST /v1/service_items` | Create service item |
| [`service-items update`](#service-items-update) | `PUT /v1/service_items` | Upsert service item |
| [`service-items get`](#service-items-get) | `GET /v1/service_items/{id}` | Get service item |
| [`service-items delete`](#service-items-delete) | `DELETE /v1/service_items/{id}` | Delete service item |

## service-items list

List service items

```text
kobana billing service-items list [OPTIONS]
```

`GET /v1/service_items`

Returns a paginated list of the current organization's service items (LC 116/2003 catalog). Use the filters to narrow by company, LC 116 code, NBS, tax classification, operation indicator, or Simples annex.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `company_id` | `string` | no | Filter by UUID of the issuing company associated with the item. |
| `search` | `string` | no | Free-text search across `name`, `internal_code` and `description`. |
| `is_active` | `string` | no | Filter by status: `true` returns only active items; `false`, only inactive. Values: `true`, `false`. |
| `lc116_code` | `string` | no | Filter by LC 116/2003 code (e.g. `01.05`). |
| `nbs_code` | `string` | no | Filter by NBS code (Brazilian Service Nomenclature). |
| `c_class_trib` | `string` | no | Filter by IBS/CBS tax classification (`c_class_trib`). |
| `indop_code` | `string` | no | Filter by Operation Indicator (`indop_code`) used in SNNFSE NFS-e. |
| `simples_annex` | `string` | no | Filter by applicable Simples Nacional annex: `annex_iii`, `annex_iv` or `annex_v`. Values: `annex_iii`, `annex_iv`, `annex_v`. |
| `cnae_code` | `string` | no | Filter by CNAE code related to the service. |
| `municipal_code` | `string` | no | Filter by municipal service code. |
| `internal_code` | `string` | no | Filter by internal item code. |
| `cbs_ibs_category` | `string` | no | Filter by CBS/IBS category for the item per the tax reform. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of service items. |
| `404` | Resource not found in the current organization. |

## service-items create

Create service item

```text
kobana billing service-items create [OPTIONS]
```

`POST /v1/service_items`

Creates a service item in the LC 116/2003 catalog. `internal_code` is the internal identifier and `lc116_code` must follow the Complementary Law 116/2003 catalog (e.g. `01.05`). When `company_id` is provided, the company must belong to the authenticated organization.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Service item name. |
| `internal_code` | `string` | yes | Internal item code — immutable after creation. |
| `lc116_code` | `string` | yes | LC 116/2003 service code (e.g. `01.05`). |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no | UUID of the associated issuing company. Omit or send `null` for an item applicable to all companies. |
| `description` | `string` | no | Optional description. |
| `municipal_code` | `string` | no | Municipal service code, when applicable. |
| `nbs_code` | `string` | no | NBS code. |
| `cnae_code` | `string` | no | Related CNAE code. |
| `simples_annex` | `string` | no | Applicable Simples Nacional annex: `annex_iii`, `annex_iv` or `annex_v`. Values: `annex_iii`, `annex_iv`, `annex_v`. |
| `default_iss_rate` | `number` | no | Default ISS rate in percentage points (e.g. `2.5`). |
| `c_class_trib` | `string` | no | IBS/CBS CST tax classification. |
| `cbs_ibs_category` | `string` | no | CBS/IBS category for the item. |
| `indop_code` | `string` | no | SNNFSE NFS-e Operation Indicator (`indOp`). |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.service-items.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Service item created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## service-items update

Upsert service item

```text
kobana billing service-items update [OPTIONS]
```

`PUT /v1/service_items`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no | UUID of the associated issuing company. Omit or send `null` for an item applicable to all companies. |
| `name` | `string` | no | Service item name. |
| `description` | `string` | no | Optional description. |
| `internal_code` | `string` | no | Internal item code — immutable after creation. |
| `lc116_code` | `string` | no | LC 116/2003 service code (e.g. `01.05`). |
| `municipal_code` | `string` | no | Municipal service code, when applicable. |
| `nbs_code` | `string` | no | NBS code. |
| `cnae_code` | `string` | no | Related CNAE code. |
| `simples_annex` | `string` | no | Applicable Simples Nacional annex: `annex_iii`, `annex_iv` or `annex_v`. Values: `annex_iii`, `annex_iv`, `annex_v`. |
| `default_iss_rate` | `number` | no | Default ISS rate in percentage points (e.g. `2.5`). |
| `c_class_trib` | `string` | no | IBS/CBS CST tax classification. |
| `cbs_ibs_category` | `string` | no | CBS/IBS category for the item. |
| `indop_code` | `string` | no | SNNFSE NFS-e Operation Indicator (`indOp`). |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.service-items.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## service-items get

Get service item

```text
kobana billing service-items get [ID] [OPTIONS]
```

`GET /v1/service_items/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Service item detail. |
| `404` | Resource not found in the current organization. |

## service-items delete

Delete service item

```text
kobana billing service-items delete [ID] [OPTIONS]
```

`DELETE /v1/service_items/{id}`

Deletes the service item (soft delete: `deleted_at` — it disappears from every read; tax rules, products and issued NFS-e that cite it keep the reference). Requires the `delete` access level. To just stop using it, deactivate it (`PATCH` with `is_active=false`). Until October 2026 this route deleted the item for good, or only deactivated it when a tax rule used it.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Service item deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
