---
id: coupons
title: "kobana billing coupons"
sidebar_label: "coupons"
---

# kobana billing coupons


Commands of `kobana billing coupons`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`coupons list`](#coupons-list) | `GET /v1/coupons` | List coupons |
| [`coupons create`](#coupons-create) | `POST /v1/coupons` | Create coupon |
| [`coupons update`](#coupons-update) | `PUT /v1/coupons` | Upsert coupon |
| [`coupons get`](#coupons-get) | `GET /v1/coupons/{id}` | Get coupon |
| [`coupons delete`](#coupons-delete) | `DELETE /v1/coupons/{id}` | Delete coupon |
| [`coupons deactivate`](#coupons-deactivate) | `POST /v1/coupons/{id}/deactivate` | Deactivate coupon |
| [`coupons redemptions`](#coupons-redemptions) | `GET /v1/coupons/{id}/redemptions` | List coupon redemptions |

## coupons list

List coupons

```text
kobana billing coupons list [OPTIONS]
```

`GET /v1/coupons`

Returns a paginated list of the current organization's coupons.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `is_active` | `string` | no | Filter by status: `true` returns only active coupons; `false`, only inactive. Values: `true`, `false`. |
| `search` | `string` | no | Free-text search across `code` and `name`. |
| `discount_type` | `string` | no | Filter by discount type: `percentage` or `fixed_amount`. Values: `percentage`, `fixed_amount`. |
| `code` | `string` | no | Filter by exact coupon code. |
| `min_purchase_min` | `number,null` | no | Minimum value of `min_purchase` (in BRL) to filter coupons. |
| `min_purchase_max` | `number,null` | no | Maximum value of `min_purchase` (in BRL) to filter coupons. |
| `max_discount_min` | `number,null` | no | Minimum value of `max_discount` (in BRL) to filter coupons. |
| `max_discount_max` | `number,null` | no | Maximum value of `max_discount` (in BRL) to filter coupons. |
| `valid_from_from` | `string` | no | Returns coupons whose validity start date is on or after this date (YYYY-MM-DD). |
| `valid_from_to` | `string` | no | Returns coupons whose validity start date is on or before this date (YYYY-MM-DD). |
| `valid_until_from` | `string` | no | Returns coupons whose expiration date is on or after this date (YYYY-MM-DD). |
| `valid_until_to` | `string` | no | Returns coupons whose expiration date is on or before this date (YYYY-MM-DD). |
| `created_after` | `string` | no | Returns coupons created on or after this date (YYYY-MM-DD). |
| `created_before` | `string` | no | Returns coupons created on or before this date (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of coupons. |
| `404` | Resource not found in the current organization. |

## coupons create

Create coupon

```text
kobana billing coupons create [OPTIONS]
```

`POST /v1/coupons`

Creates a discount coupon. The `code` must be unique per organization. Use `discount_type=percentage` with `discount_value` between `0` and `100`, or `discount_type=fixed_amount` with `discount_value` in BRL (float, up to 4 decimal places).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `code` | `string` | yes | Public coupon code — unique per organization. |
| `name` | `string` | yes | Internal coupon name. |
| `discount_type` | `string` | yes | Discount type: `percentage` or `fixed_amount`. Values: `percentage`, `fixed_amount`. |
| `discount_value` | `number` | yes | Discount value. For `percentage`, integer between `0` and `100`. For `fixed_amount`, amount in BRL (float, up to 4 decimal places). |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `description` | `string` | no | Optional description. |
| `currency` | `string` | no | ISO 4217 currency. Default: `BRL`. |
| `min_purchase` | `number` | no | Minimum invoice amount, in BRL (float, up to 4 decimal places), for the coupon to be applicable. |
| `max_discount` | `number` | no | Absolute discount cap, in BRL (float, up to 4 decimal places). |
| `usage_limit` | `integer` | no | Maximum total redemptions across the organization. Omit for unlimited. |
| `usage_limit_per_user` | `integer` | no | Maximum redemptions per billing account. Omit for unlimited. |
| `valid_from` | `string` | no | Start of the validity window (ISO 8601 — date or date-time). |
| `valid_until` | `string` | no | End of the validity window (ISO 8601 — date or date-time). |
| `applies_to` | `object` | no |  |
| `excludes` | `object` | no |  |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.coupons.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Coupon created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## coupons update

Upsert coupon

```text
kobana billing coupons update [OPTIONS]
```

`PUT /v1/coupons`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `code` | `string` | no | Public coupon code — unique per organization. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `name` | `string` | no | Internal coupon name. |
| `description` | `string` | no | Optional description. |
| `discount_type` | `string` | no | Discount type: `percentage` or `fixed_amount`. Values: `percentage`, `fixed_amount`. |
| `discount_value` | `number` | no | Discount value. For `percentage`, integer between `0` and `100`. For `fixed_amount`, amount in BRL (float, up to 4 decimal places). |
| `currency` | `string` | no | ISO 4217 currency. Default: `BRL`. |
| `min_purchase` | `number` | no | Minimum invoice amount, in BRL (float, up to 4 decimal places), for the coupon to be applicable. |
| `max_discount` | `number` | no | Absolute discount cap, in BRL (float, up to 4 decimal places). |
| `usage_limit` | `integer` | no | Maximum total redemptions across the organization. Omit for unlimited. |
| `usage_limit_per_user` | `integer` | no | Maximum redemptions per billing account. Omit for unlimited. |
| `valid_from` | `string` | no | Start of the validity window (ISO 8601 — date or date-time). |
| `valid_until` | `string` | no | End of the validity window (ISO 8601 — date or date-time). |
| `applies_to` | `object` | no |  |
| `excludes` | `object` | no |  |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.coupons.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## coupons get

Get coupon

```text
kobana billing coupons get [ID] [OPTIONS]
```

`GET /v1/coupons/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Coupon detail. |
| `404` | Resource not found in the current organization. |

## coupons delete

Delete coupon

```text
kobana billing coupons delete [ID] [OPTIONS]
```

`DELETE /v1/coupons/{id}`

Deletes the coupon permanently. Allowed only while the coupon **has no recorded redemptions** — otherwise use `POST /coupons/{id}/deactivate` to preserve history.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Coupon deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## coupons deactivate

Deactivate coupon

```text
kobana billing coupons deactivate [ID] [OPTIONS]
```

`POST /v1/coupons/{id}/deactivate`

Deactivates the coupon: new redemptions are blocked, but previous redemptions are preserved.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Coupon deactivated. |
| `404` | Resource not found in the current organization. |

## coupons redemptions

List coupon redemptions

```text
kobana billing coupons redemptions [ID] [OPTIONS]
```

`GET /v1/coupons/{id}/redemptions`

Returns a paginated list of the coupon's recorded redemptions.

**Path arguments:** `id`

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
| `200` | Paginated list of coupon redemptions. |
| `404` | Resource not found in the current organization. |
