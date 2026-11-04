---
id: payment-methods
title: "kobana billing payment-methods"
sidebar_label: "payment-methods"
---

# kobana billing payment-methods


Commands of `kobana billing payment-methods`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`payment-methods list`](#payment-methods-list) | `GET /v1/payment_methods` | List payment methods |
| [`payment-methods create`](#payment-methods-create) | `POST /v1/payment_methods` | Create payment method |
| [`payment-methods update`](#payment-methods-update) | `PUT /v1/payment_methods` | Upsert payment method |
| [`payment-methods get`](#payment-methods-get) | `GET /v1/payment_methods/{id}` | Get payment method |
| [`payment-methods delete`](#payment-methods-delete) | `DELETE /v1/payment_methods/{id}` | Delete payment method |
| [`payment-methods set-default`](#payment-methods-set-default) | `POST /v1/payment_methods/{id}/set-default` | Set payment method as default |

## payment-methods list

List payment methods

```text
kobana billing payment-methods list [OPTIONS]
```

`GET /v1/payment_methods`

Returns a paginated list of the current organization's payment methods. Optional filters: `billing_account_id`, `type`, `status`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Filter by billing account UUID. |
| `type` | `string` | no | Filter by type: `card`, `bank_slip`, `pix` or `bank_transfer`. Values: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `status` | `string` | no | Filter by status: `active`, `expired` or `failed`. Values: `active`, `expired`, `failed`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of payment methods. |
| `404` | Resource not found in the current organization. |

## payment-methods create

Create payment method

```text
kobana billing payment-methods create [OPTIONS]
```

`POST /v1/payment_methods`

Creates a payment method tied to a billing account. For `type=card`, send the `card` block with the `card_token` produced by client-side tokenization. For `type=bank_transfer`, send the `bank_account` block. For `bank_slip` and `pix`, only the `type` is required — the default gateway is resolved from the organization.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | yes | UUID of the billing account that owns the payment method. |
| `type` | `string` | yes | Method type: `card`, `bank_slip`, `pix` or `bank_transfer`. Values: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `is_default` | `boolean` | no | When `true`, marks this method as the account default (unsets previous defaults). The first method created for an account also becomes the default automatically. |
| `card` | `object` | no |  |
| `bank_account` | `object` | no |  |
| `billing_details` | `object` | no |  |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Free-form custom metadata (`object`). |

More fields and nested objects: `kobana schema billing.payment-methods.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Payment method created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payment-methods update

Upsert payment method

```text
kobana billing payment-methods update [OPTIONS]
```

`PUT /v1/payment_methods`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | UUID of the billing account that owns the payment method. |
| `type` | `string` | no | Method type: `card`, `bank_slip`, `pix` or `bank_transfer`. Values: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `is_default` | `boolean` | no | When `true`, marks this method as the account default (unsets previous defaults). The first method created for an account also becomes the default automatically. |
| `card` | `object` | no |  |
| `bank_account` | `object` | no |  |
| `billing_details` | `object` | no |  |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Free-form custom metadata (`object`). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.payment-methods.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payment-methods get

Get payment method

```text
kobana billing payment-methods get [ID] [OPTIONS]
```

`GET /v1/payment_methods/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment method detail. |
| `404` | Resource not found in the current organization. |

## payment-methods delete

Delete payment method

```text
kobana billing payment-methods delete [ID] [OPTIONS]
```

`DELETE /v1/payment_methods/{id}`

Deletes the payment method (soft delete: `deleted_at`, and the status becomes `removed` wherever it still shows through a relation — on the payments it paid). Requires the `delete` access level. Fails while payments are pending or processing. If it was the default, another active method is promoted automatically.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment method deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## payment-methods set-default

Set payment method as default

```text
kobana billing payment-methods set-default [ID] [OPTIONS]
```

`POST /v1/payment_methods/{id}/set-default`

Marks the method as the billing account's default and unsets previous defaults. Fails when the method is not `active`.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Payment method set as default. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
