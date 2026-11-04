---
id: billing-accounts
title: "kobana billing billing-accounts"
sidebar_label: "billing-accounts"
---

# kobana billing billing-accounts


Commands of `kobana billing billing-accounts`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`billing-accounts list`](#billing-accounts-list) | `GET /v1/billing_accounts` | List billing accounts |
| [`billing-accounts create`](#billing-accounts-create) | `POST /v1/billing_accounts` | Create billing account |
| [`billing-accounts update`](#billing-accounts-update) | `PUT /v1/billing_accounts` | Upsert billing account |
| [`billing-accounts get`](#billing-accounts-get) | `GET /v1/billing_accounts/{id}` | Get billing account |
| [`billing-accounts delete`](#billing-accounts-delete) | `DELETE /v1/billing_accounts/{id}` | Delete billing account |
| [`billing-accounts close`](#billing-accounts-close) | `POST /v1/billing_accounts/{id}/close` | Close billing account |
| [`billing-accounts credits`](#billing-accounts-credits) | `GET /v1/billing_accounts/{id}/credits` | List billing account credits |
| [`billing-accounts entitlements`](#billing-accounts-entitlements) | `GET /v1/billing_accounts/{id}/entitlements` | List billing account entitlements |
| [`billing-accounts invoices`](#billing-accounts-invoices) | `GET /v1/billing_accounts/{id}/invoices` | List billing account invoices |
| [`billing-accounts reactivate`](#billing-accounts-reactivate) | `POST /v1/billing_accounts/{id}/reactivate` | Reactivate billing account |
| [`billing-accounts subscriptions`](#billing-accounts-subscriptions) | `GET /v1/billing_accounts/{id}/subscriptions` | List billing account subscriptions |
| [`billing-accounts suspend`](#billing-accounts-suspend) | `POST /v1/billing_accounts/{id}/suspend` | Suspend billing account |

## billing-accounts list

List billing accounts

```text
kobana billing billing-accounts list [OPTIONS]
```

`GET /v1/billing_accounts`

Returns a paginated list of the current organization's billing accounts.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `status` | `string` | no | Filter by billing account status. Values: `active`, `suspended`, `closed`. |
| `search` | `string` | no | Free-text search. |
| `external_id` | `string` | no | Filter by the external identifier of the account. |
| `currency` | `string` | no | Filter by ISO 4217 currency (e.g.: `BRL`). |
| `auto_collection` | `string` | no | Filter by automatic collection enabled (`true`) or disabled (`false`). Values: `true`, `false`. |
| `tax_exempt` | `string` | no | Filter tax-exempt accounts (`true`) or non-exempt (`false`). Values: `true`, `false`. |
| `customer_tax_type` | `string` | no | Filter by customer tax type. |
| `balance_min` | `number,null` | no | Minimum account balance in BRL (float, up to 4 decimal places). |
| `balance_max` | `number,null` | no | Maximum account balance in BRL (float, up to 4 decimal places). |
| `mrr_min` | `number,null` | no | Minimum MRR in BRL (float, up to 4 decimal places). |
| `mrr_max` | `number,null` | no | Maximum MRR in BRL (float, up to 4 decimal places). |
| `allowed_negative_balance_min` | `number,null` | no | Minimum allowed negative balance in BRL (float, up to 4 decimal places). |
| `allowed_negative_balance_max` | `number,null` | no | Maximum allowed negative balance in BRL (float, up to 4 decimal places). |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `updated_after` | `string` | no | Start of update date range (YYYY-MM-DD). |
| `updated_before` | `string` | no | End of update date range (YYYY-MM-DD). |
| `suspended_after` | `string` | no | Filter accounts suspended from this date onwards (format `YYYY-MM-DD`). |
| `suspended_before` | `string` | no | Filter accounts suspended up to this date (format `YYYY-MM-DD`). |
| `person_id` | `string` | no | Filter by associated person UUID. |
| `person_document_number` | `string` | no | Filter by person CPF/CNPJ (exact match). |
| `person_name_contains` | `string` | no | Filter by person name (partial search, case-insensitive). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of billing accounts. |
| `404` | Resource not found in the current organization. |

## billing-accounts create

Create billing account

```text
kobana billing billing-accounts create [OPTIONS]
```

`POST /v1/billing_accounts`

Creates a billing account associated with a customer. The `customer` block locates an existing customer by `id`, `external_id` or `document_number`; when not found, creates a new one (requires `name` and `document_number`).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `customer` | `object` | yes |  |
| `external_id` | `string` | no | External identifier for the account in the client's system. |
| `billing_email` | `string (email)` | no | Billing email. Default: `customer.email`. |
| `payment_terms_days` | `integer` | no | Invoice due period in days (1–90). Default: the organization's configured default payment terms (10 when not configured). |
| `auto_collection` | `boolean` | no | Enables automatic collection. Default: `true`. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.billing-accounts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Billing account created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## billing-accounts update

Upsert billing account

```text
kobana billing billing-accounts update [OPTIONS]
```

`PUT /v1/billing_accounts`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `customer` | `object` | no |  |
| `external_id` | `string` | no | External identifier for the account in the client's system. |
| `billing_email` | `string (email)` | no | Billing email. Default: `customer.email`. |
| `payment_terms_days` | `integer` | no | Invoice due period in days (1–90). Default: the organization's configured default payment terms (10 when not configured). |
| `auto_collection` | `boolean` | no | Enables automatic collection. Default: `true`. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.billing-accounts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## billing-accounts get

Get billing account

```text
kobana billing billing-accounts get [ID] [OPTIONS]
```

`GET /v1/billing_accounts/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Billing account detail. |
| `404` | Resource not found in the current organization. |

## billing-accounts delete

Delete billing account

```text
kobana billing billing-accounts delete [ID] [OPTIONS]
```

`DELETE /v1/billing_accounts/{id}`

Deletes the billing account (soft delete: `deleted_at`), together with its payment methods, cards, portal users and files — all of them disappear from every read; issued invoices, payments and NFS-e stay in the history. Requires the `delete` access level. Answers 422 while a subscription is not canceled, an invoice is draft or open, or a payment is pending. Until October 2026 this route closed the account (and took `?reason=`) — closing is `POST /v1/billing_accounts/{id}/close`, reversible.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Billing account deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## billing-accounts close

Close billing account

```text
kobana billing billing-accounts close [ID] [OPTIONS]
```

`POST /v1/billing_accounts/{id}/close`

Closes the billing account. The `reason` is recorded in the audit log.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `reason` | `string` | no | Operation reason (recorded in the audit log). |

More fields and nested objects: `kobana schema billing.billing-accounts.close`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Billing account closed. |
| `404` | Resource not found in the current organization. |

## billing-accounts credits

List billing account credits

```text
kobana billing billing-accounts credits [ID] [OPTIONS]
```

`GET /v1/billing_accounts/{id}/credits`

Returns a paginated list of the billing account's credits.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `include_expired` | `string` | no | Includes expired credits when `true`. Values: `true`, `false`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of credits. |
| `404` | Resource not found in the current organization. |

## billing-accounts entitlements

List billing account entitlements

```text
kobana billing billing-accounts entitlements [ID] [OPTIONS]
```

`GET /v1/billing_accounts/{id}/entitlements`

Returns a lean list of the products a billing account is entitled to, based on subscriptions that are trialing, active, or past due (dunning grace). Only parent products are returned; child (add-on) products are rolled up under their parent and omitted. Ideal for gating a product's usage in an external app.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `product_group_id` | `string` | no | Filters contracted products by family (product group), by ID. |
| `product_group_slug` | `string` | no | Filters contracted products by family (product group), by slug. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of contracted products. |
| `404` | Resource not found in the current organization. |

## billing-accounts invoices

List billing account invoices

```text
kobana billing billing-accounts invoices [ID] [OPTIONS]
```

`GET /v1/billing_accounts/{id}/invoices`

Returns a paginated list of the billing account's invoices.

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
| `200` | Paginated list of invoices. |
| `404` | Resource not found in the current organization. |

## billing-accounts reactivate

Reactivate billing account

```text
kobana billing billing-accounts reactivate [ID] [OPTIONS]
```

`POST /v1/billing_accounts/{id}/reactivate`

Reactivates a suspended billing account.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Billing account reactivated. |
| `404` | Resource not found in the current organization. |

## billing-accounts subscriptions

List billing account subscriptions

```text
kobana billing billing-accounts subscriptions [ID] [OPTIONS]
```

`GET /v1/billing_accounts/{id}/subscriptions`

Returns a paginated list of the billing account's subscriptions.

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
| `200` | Paginated list of subscriptions. |
| `404` | Resource not found in the current organization. |

## billing-accounts suspend

Suspend billing account

```text
kobana billing billing-accounts suspend [ID] [OPTIONS]
```

`POST /v1/billing_accounts/{id}/suspend`

Suspends the account. The `reason` is recorded in the audit log.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `reason` | `string` | no | Operation reason (recorded in the audit log). |

More fields and nested objects: `kobana schema billing.billing-accounts.suspend`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Billing account suspended. |
| `404` | Resource not found in the current organization. |
