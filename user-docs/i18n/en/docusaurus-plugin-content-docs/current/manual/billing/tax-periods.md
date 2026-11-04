---
id: tax-periods
title: "kobana billing tax-periods"
sidebar_label: "tax-periods"
---

# kobana billing tax-periods


Commands of `kobana billing tax-periods`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`tax-periods list`](#tax-periods-list) | `GET /v1/tax_periods` | List tax periods |
| [`tax-periods create`](#tax-periods-create) | `POST /v1/tax_periods` | Process monthly tax period |
| [`tax-periods get`](#tax-periods-get) | `GET /v1/tax_periods/{id}` | Get tax period |
| [`tax-periods update`](#tax-periods-update) | `PATCH /v1/tax_periods/{id}` | Run action on tax period |
| [`tax-periods delete`](#tax-periods-delete) | `DELETE /v1/tax_periods/{id}` | Delete tax period |

## tax-periods list

List tax periods

```text
kobana billing tax-periods list [OPTIONS]
```

`GET /v1/tax_periods`

Returns a paginated list of the organization tax periods (monthly or quarterly). Supports filtering by company, period type, year and status.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `company_id` | `string` | no | Filter by issuing company. |
| `period_type` | `string` | no | Filter by period type: `monthly` or `quarterly`. Values: `monthly`, `quarterly`. |
| `year` | `string` | no | Filter by reference year (e.g.: `2026`). |
| `status` | `string` | no | Filter by status: `open`, `calculated` or `closed`. Values: `open`, `calculated`, `closed`. |
| `month` | `integer` | no | Filter by reference month (1 to 12). |
| `quarter` | `integer` | no | Filter by reference quarter (1 to 4). |
| `gross_revenue_min` | `number,null` | no | Minimum gross revenue for the period, in BRL (e.g.: `1000.00`). |
| `gross_revenue_max` | `number,null` | no | Maximum gross revenue for the period, in BRL (e.g.: `50000.00`). |
| `das_estimated_min` | `number,null` | no | Minimum estimated DAS for the period, in BRL (e.g.: `100.00`). |
| `das_estimated_max` | `number,null` | no | Maximum estimated DAS for the period, in BRL (e.g.: `5000.00`). |
| `closed_after` | `string` | no | Returns periods closed on or after this date (format `YYYY-MM-DD`). |
| `closed_before` | `string` | no | Returns periods closed on or before this date (format `YYYY-MM-DD`). |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of tax periods. |
| `404` | Resource not found in the current organization. |

## tax-periods create

Process monthly tax period

```text
kobana billing tax-periods create [OPTIONS]
```

`POST /v1/tax_periods`

Creates (or retrieves) the given monthly tax period and computes the taxes due (ISS, PIS, COFINS, IRPJ, CSLL, CBS, IBS) based on issued NFes and pending compensation. When `company_id` is omitted, uses the organization default company.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `year` | `integer` | yes | Reference year (e.g.: `2026`). |
| `month` | `integer` | yes | Reference month (1 to 12). |
| `company_id` | `string (uuid)` | no | UUID of the issuing company. Omit to use the organization default company. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.tax-periods.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Tax period processed. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## tax-periods get

Get tax period

```text
kobana billing tax-periods get [ID] [OPTIONS]
```

`GET /v1/tax_periods/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Tax period detail. |
| `404` | Resource not found in the current organization. |

## tax-periods update

Run action on tax period

```text
kobana billing tax-periods update [ID] [OPTIONS]
```

`PATCH /v1/tax_periods/{id}`

Applies an action to the period: `recalculate` recomputes taxes (monthly periods only), `close` closes the period and `reopen` reopens it.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `action` | `string` | yes | Action to execute: `recalculate`, `close` or `reopen`. Values: `recalculate`, `close`, `reopen`. |

More fields and nested objects: `kobana schema billing.tax-periods.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Tax period updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## tax-periods delete

Delete tax period

```text
kobana billing tax-periods delete [ID] [OPTIONS]
```

`DELETE /v1/tax_periods/{id}`

Permanently removes the tax period. Use with caution — closed periods usually should not be deleted.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Tax period removed. |
| `404` | Resource not found in the current organization. |
