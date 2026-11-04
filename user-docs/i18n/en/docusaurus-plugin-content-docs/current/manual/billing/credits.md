---
id: credits
title: "kobana billing credits"
sidebar_label: "credits"
---

# kobana billing credits


Commands of `kobana billing credits`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`credits list`](#credits-list) | `GET /v1/credits` | List credits |
| [`credits create`](#credits-create) | `POST /v1/credits` | Add credit |
| [`credits update`](#credits-update) | `PUT /v1/credits` | Upsert credit |
| [`credits get`](#credits-get) | `GET /v1/credits/{id}` | Get credit |
| [`credits delete`](#credits-delete) | `DELETE /v1/credits/{id}` | Delete credit |

## credits list

List credits

```text
kobana billing credits list [OPTIONS]
```

`GET /v1/credits`

Returns a paginated list of credits in the current organization. By default, expired credits are omitted — use `usage` or `include_expired` to include them.

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
| `billing_account_id` | `string` | no | Filter credits by billing account. |
| `company_id` | `string` | no | Filter credits by the linked `company`. |
| `type` | `string` | no | Filter by credit type: `promotional`, `adjustment`, `refund` or `manual`. Values: `promotional`, `adjustment`, `refund`, `manual`. |
| `usage` | `string` | no | Filter by usage state — `available` (remaining balance and not expired), `used` (fully consumed) or `expired` (past expiration). Values: `available`, `used`, `expired`. |
| `include_expired` | `string` | no | Includes expired credits when `true`. Ignored when `usage` is set. Values: `true`, `false`. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `amount_min` | `number,null` | no | Minimum credit amount in BRL (inclusive). |
| `amount_max` | `number,null` | no | Maximum credit amount in BRL (inclusive). |
| `amount_remaining_min` | `number,null` | no | Minimum remaining balance in BRL (inclusive). |
| `amount_remaining_max` | `number,null` | no | Maximum remaining balance in BRL (inclusive). |
| `expires_after` | `string` | no | Lower bound for `expires_at` (YYYY-MM-DD, inclusive). |
| `expires_before` | `string` | no | Upper bound for `expires_at` (YYYY-MM-DD, inclusive). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of credits. |
| `404` | Resource not found in the current organization. |

## credits create

Add credit

```text
kobana billing credits create [OPTIONS]
```

`POST /v1/credits`

Adds a credit to a billing account. The amount is credited to the `BillingAccount` `balance` and becomes available to settle future invoices. When `company_id` is provided, the credit can only be applied to invoices from the same `company`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | yes | UUID of the billing account that will receive the credit. |
| `type` | `string` | yes | Credit type: `promotional`, `adjustment`, `refund` or `manual`. Values: `promotional`, `adjustment`, `refund`, `manual`. |
| `amount` | `number` | yes | Credit amount in BRL (float, up to 4 decimal places). Must be a positive integer. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no | Optional UUID of the `company` the credit is tied to. When set, restricts credit application to invoices from the same `company`. |
| `description` | `string` | no | Free-form description (up to 500 characters) — shown in the statement and audit log. |
| `expires_at` | `string (date-time)` | no | Expiration date and time (ISO8601). After this date the remaining balance is discarded. |
| `custom_metadata` | `object` | no | Custom metadata validated against the organization schema. |

More fields and nested objects: `kobana schema billing.credits.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Credit created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## credits update

Upsert credit

```text
kobana billing credits update [OPTIONS]
```

`PUT /v1/credits`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | UUID of the billing account that will receive the credit. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no | Optional UUID of the `company` the credit is tied to. When set, restricts credit application to invoices from the same `company`. |
| `type` | `string` | no | Credit type: `promotional`, `adjustment`, `refund` or `manual`. Values: `promotional`, `adjustment`, `refund`, `manual`. |
| `amount` | `number` | no | Credit amount in BRL (float, up to 4 decimal places). Must be a positive integer. |
| `description` | `string` | no | Free-form description (up to 500 characters) — shown in the statement and audit log. |
| `expires_at` | `string (date-time)` | no | Expiration date and time (ISO8601). After this date the remaining balance is discarded. |
| `custom_metadata` | `object` | no | Custom metadata validated against the organization schema. |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.credits.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## credits get

Get credit

```text
kobana billing credits get [ID] [OPTIONS]
```

`GET /v1/credits/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Credit detail. |
| `404` | Resource not found in the current organization. |

## credits delete

Delete credit

```text
kobana billing credits delete [ID] [OPTIONS]
```

`DELETE /v1/credits/{id}`

Removes the credit and reverses the matching balance on the billing account. Only allowed if no cents of the credit have been consumed — otherwise the API responds with `400`.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Credit deleted. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
