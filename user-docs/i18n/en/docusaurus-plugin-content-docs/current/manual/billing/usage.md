---
id: usage
title: "kobana billing usage"
sidebar_label: "usage"
---

# kobana billing usage


Commands of `kobana billing usage`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`usage credits`](#usage-credits) | `GET /v1/usage/credits` | Get credit usage and balance |
| [`usage credits publish`](#usage-credits-publish) | `POST /v1/usage/credits/publish` | Publish credit usage |
| [`usage snapshots daily list`](#usage-snapshots-daily-list) | `GET /v1/usage/snapshots/daily` | List daily usage snapshots |
| [`usage snapshots daily create`](#usage-snapshots-daily-create) | `POST /v1/usage/snapshots/daily` | Create daily usage snapshots |
| [`usage snapshots monthly list`](#usage-snapshots-monthly-list) | `GET /v1/usage/snapshots/monthly` | List monthly usage snapshots |
| [`usage snapshots monthly create`](#usage-snapshots-monthly-create) | `POST /v1/usage/snapshots/monthly` | Create monthly usage snapshots |
| [`usage summary`](#usage-summary) | `GET /v1/usage/summary` | Usage summary for the period |
| [`usage list`](#usage-list) | `GET /v1/usage` | List usage records |
| [`usage create`](#usage-create) | `POST /v1/usage` | Report usage (metered) |

## usage credits

Get credit usage and balance

```text
kobana billing usage credits [OPTIONS] [COMMAND]
```

`GET /v1/usage/credits`

Returns prepaid credit usage for the period (totals and daily series) and the current balance. Without `billing_account_id`, sums every account in the organization. Without dates, uses the current month (São Paulo time zone).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string` | no | Filter by billing account. |
| `product_id` | `string` | no | Credits product. Default: the first active product. |
| `start_date` | `string` | no | Period start (`YYYY-MM-DD`). |
| `end_date` | `string` | no | Period end, inclusive (`YYYY-MM-DD`). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Period usage and balance. |
| `404` | Resource not found in the current organization. |

## usage credits publish

Publish credit usage

```text
kobana billing usage credits publish [OPTIONS]
```

`POST /v1/usage/credits/publish`

Publishes prepaid credit usage (a `credits` product) per billing account × product × day, as a single record **or** a batch `{ records: [...] }` of up to **100** items. The key (account, product, `usage_date`) is unique: publishing the same day again **overwrites** the record (`overwritten: true`), it never adds up. Batches with partial failures return **207 Multi-Status**.

**Request body**

The body is not a plain object (or has variants). See `kobana schema billing.usage.credits.publish`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Records processed (`207` when part of the batch failed). |
| `207` | Records processed (`207` when part of the batch failed). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## usage snapshots daily list

List daily usage snapshots

```text
kobana billing usage snapshots daily list [OPTIONS]
```

`GET /v1/usage/snapshots/daily`

Lists the organization's daily usage snapshots. Optional filters: `billing_account_id`, `product_id` and `start_date`/`end_date` window. Supports `limit`/`offset` pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string` | no | Filter snapshots by billing account UUID. |
| `product_id` | `string` | no | Filter snapshots by product UUID. |
| `start_date` | `string` | no | Filter start date/time — ISO 8601 format. |
| `end_date` | `string` | no | Filter end date/time — ISO 8601 format. |
| `limit` | `integer` | no | Items per page. Default: `100`. Maximum: `1000`. |
| `offset` | `integer,null` | no | Pagination offset. Default: `0`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Daily snapshots list with `meta` carrying `total`, `limit`, `offset`. |
| `404` | Resource not found in the current organization. |

## usage snapshots daily create

Create daily usage snapshots

```text
kobana billing usage snapshots daily create [OPTIONS]
```

`POST /v1/usage/snapshots/daily`

Creates daily usage snapshots, single record **or** `{ records: [...] }` batch up to **100** items. The natural key (organization, billing account, product, date) is unique: when present, the snapshot is overwritten (`overwritten: true`). Resolve the account by `billing_account_id` **or** `billing_account_public_id` and the product by `product_slug` **or** `product_id` within the supplied `product_group_slug`. Batches with partial failures return **207 Multi-Status**.

**Request body**

The body is not a plain object (or has variants). See `kobana schema billing.usage.snapshots.daily.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Processing result (single record or batch). |
| `207` | Processing result (single record or batch). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## usage snapshots monthly list

List monthly usage snapshots

```text
kobana billing usage snapshots monthly list [OPTIONS]
```

`GET /v1/usage/snapshots/monthly`

Lists the organization's monthly usage snapshots. Optional filters: `billing_account_id`, `product_id` and `start_month`/`end_month` window. Supports `limit`/`offset` pagination.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string` | no | Filter snapshots by billing account UUID. |
| `product_id` | `string` | no | Filter snapshots by product UUID. |
| `start_month` | `string` | no | Filter start month in `YYYY-MM-DD` format (first day of the month). |
| `end_month` | `string` | no | Filter end month in `YYYY-MM-DD` format (first day of the month). |
| `limit` | `integer` | no | Items per page. Default: `100`. Maximum: `1000`. |
| `offset` | `integer,null` | no | Pagination offset. Default: `0`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Monthly snapshots list with `meta` carrying `total`, `limit`, `offset`. |
| `404` | Resource not found in the current organization. |

## usage snapshots monthly create

Create monthly usage snapshots

```text
kobana billing usage snapshots monthly create [OPTIONS]
```

`POST /v1/usage/snapshots/monthly`

Creates monthly usage snapshots, single record **or** `{ records: [...] }` batch up to **100** items. The natural key (organization, billing account, product, month) is unique: when present, the snapshot is overwritten (`overwritten: true`). Resolve the account by `billing_account_id` **or** `billing_account_public_id` and the product by `product_slug` **or** `product_id` within `product_group_slug`. Batches with partial failures return **207 Multi-Status**.

**Request body**

The body is not a plain object (or has variants). See `kobana schema billing.usage.snapshots.monthly.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Processing result (single record or batch). |
| `207` | Processing result (single record or batch). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## usage summary

Usage summary for the period

```text
kobana billing usage summary [OPTIONS]
```

`GET /v1/usage/summary`

Returns the aggregated usage of a subscription item for the current billing period (or for an arbitrary period via `period_start`/`period_end`). Honors `set` records (which replace) and the sum of subsequent `increment` records. When the price is tiered or per-unit, computes an estimated cost in `estimated_cost` (float, up to 4 decimal places).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `subscription_item_id` | `string` | no | Subscription item UUID. Alternative to `product_slug` + `customer_id`. |
| `product_slug` | `string` | no | Product slug. Combine with `customer_id`. |
| `customer_id` | `string` | no | Customer external identifier. Combine with `product_slug`. |
| `period_start` | `string` | no | Period start (ISO 8601). Default: subscription `current_period_start`. |
| `period_end` | `string` | no | Period end (ISO 8601). Default: subscription `current_period_end`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Usage summary, period and estimated cost. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## usage list

List usage records

```text
kobana billing usage list [OPTIONS]
```

`GET /v1/usage`

Returns metered usage records for a subscription item. Identify the item by `subscription_item_id` **or** by the `product_slug` + `customer_id` combination. Supports date-range filtering and `limit`/`offset` windowing.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `subscription_item_id` | `string` | no | Subscription item UUID. Alternative to `product_slug` + `customer_id`. |
| `product_slug` | `string` | no | Product slug. Combine with `customer_id`. |
| `customer_id` | `string` | no | Customer external identifier. Combine with `product_slug`. |
| `start_date` | `string` | no | Filter start date/time — ISO 8601 format. |
| `end_date` | `string` | no | Filter end date/time — ISO 8601 format. |
| `limit` | `integer` | no | Items per page. Default: `100`. Maximum: `1000`. |
| `offset` | `integer,null` | no | Pagination offset. Default: `0`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Usage records list with `total_quantity` aggregation in `meta`. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## usage create

Report usage (metered)

```text
kobana billing usage create [OPTIONS]
```

`POST /v1/usage`

Reports usage for metered billing. Accepts a single record **or** a `{ records: [...] }` batch of up to **100** items. Each record must identify the subscription item by `subscription_item_id` **or** by `product_slug` + `customer_id` (recommended for external integrations). The price/product must be configured with `usage_type=metered` and the subscription must be `active` or `trialing`. Use `idempotency_key` to avoid duplicates — when a key was already seen the response is flagged with `duplicate: true`. Batches with partial failures return **207 Multi-Status**.

**Request body**

The body is not a plain object (or has variants). See `kobana schema billing.usage.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Processing result (single record or batch). |
| `207` | Processing result (single record or batch). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
