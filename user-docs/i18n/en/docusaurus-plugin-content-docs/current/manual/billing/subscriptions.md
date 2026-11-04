---
id: subscriptions
title: "kobana billing subscriptions"
sidebar_label: "subscriptions"
---

# kobana billing subscriptions


Commands of `kobana billing subscriptions`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`subscriptions items`](#subscriptions-items) | `GET /v1/subscriptions/{id}/items` | List subscription items |
| [`subscriptions items create`](#subscriptions-items-create) | `POST /v1/subscriptions/{id}/items` | Apply changes to the subscription items |
| [`subscriptions list`](#subscriptions-list) | `GET /v1/subscriptions` | List subscriptions |
| [`subscriptions create`](#subscriptions-create) | `POST /v1/subscriptions` | Create subscription |
| [`subscriptions update`](#subscriptions-update) | `PUT /v1/subscriptions` | Upsert subscription |
| [`subscriptions get`](#subscriptions-get) | `GET /v1/subscriptions/{id}` | Get subscription |
| [`subscriptions delete`](#subscriptions-delete) | `DELETE /v1/subscriptions/{id}` | Cancel and remove subscription (soft delete) |
| [`subscriptions activate`](#subscriptions-activate) | `POST /v1/subscriptions/{id}/activate` | Activate draft/confirmed subscription |
| [`subscriptions cancel`](#subscriptions-cancel) | `POST /v1/subscriptions/{id}/cancel` | Cancel subscription |
| [`subscriptions change-anniversary`](#subscriptions-change-anniversary) | `POST /v1/subscriptions/{id}/change-anniversary` | Change the subscription anniversary (cycle start day) |
| [`subscriptions change-plan`](#subscriptions-change-plan) | `POST /v1/subscriptions/{id}/change-plan` | Change subscription plan |
| [`subscriptions create-invoice`](#subscriptions-create-invoice) | `POST /v1/subscriptions/{id}/create-invoice` | Generate an ad-hoc invoice for the subscription |
| [`subscriptions pause`](#subscriptions-pause) | `POST /v1/subscriptions/{id}/pause` | Pause subscription |
| [`subscriptions resume`](#subscriptions-resume) | `POST /v1/subscriptions/{id}/resume` | Resume paused subscription |
| [`subscriptions revert-to-confirmed`](#subscriptions-revert-to-confirmed) | `POST /v1/subscriptions/{id}/revert-to-confirmed` | Revert subscription to confirmed state |
| [`subscriptions sync-plan-items`](#subscriptions-sync-plan-items) | `POST /v1/subscriptions/{id}/sync-plan-items` | Resync items with the current plan |

## subscriptions items

List subscription items

```text
kobana billing subscriptions items [ID] [OPTIONS] [COMMAND]
```

`GET /v1/subscriptions/{id}/items`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of items. |
| `404` | Resource not found in the current organization. |

## subscriptions items create

Apply changes to the subscription items

```text
kobana billing subscriptions items create [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/items`

Runs a batch of up to 50 operations over the items: add, remove, change quantity, swap price or set an ad-hoc unit amount. The operations are applied together and produce a single change record, along with the resulting proration invoice or credit.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `changes` | `array<oneOf>` | yes | Operations to apply, from 1 to 50. Each item is discriminated by `op`. |
| `timing` | `string` | no | When to apply: `immediate` (default) or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `reason` | `string` | no | Free-form reason, stored alongside the change record. |
| `idempotency_key` | `string` | no | Idempotency key. The `Idempotency-Key` header takes precedence over this field. |

More fields and nested objects: `kobana schema billing.subscriptions.items.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Changes applied. `item_change_id` points to the record created; `invoice_id` and `credit_id` are filled in when the proration produced an invoice or a credit. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## subscriptions list

List subscriptions

```text
kobana billing subscriptions list [OPTIONS]
```

`GET /v1/subscriptions`

Returns a paginated list of the current organization's subscriptions.

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
| `status` | `string` | no | Filter by subscription status. Values: `draft`, `trialing`, `active`, `past_due`, `canceled`, `paused`, `suspended`. |
| `plan_id` | `string` | no | Filter by plan. |
| `billing_cycle` | `string` | no | Filter by billing cycle. Values: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `billing_account_id` | `string` | no | Filter by billing account. |
| `search` | `string` | no | Free-text search. |
| `company_id` | `string` | no | Filter by company (UUID). |
| `collection_method` | `string` | no | Filter by collection method: `charge_automatically`, `manual_charge` or `manual_invoice`. Values: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `cancel_at_period_end` | `string` | no | Filter subscriptions scheduled to cancel at period end (`true`) or not (`false`). Values: `true`, `false`. |
| `external_id` | `string` | no | Filter by external identifier. |
| `product_group_id` | `string` | no | Filter by product group (UUID). |
| `proposal_id` | `string` | no | Filter by proposal (UUID). |
| `recurring_amount_min` | `number,null` | no | Minimum recurring amount in BRL (e.g. `50.00`). |
| `recurring_amount_max` | `number,null` | no | Maximum recurring amount in BRL (e.g. `500.00`). |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `current_period_start_from` | `string` | no | Current period start on or after this date (YYYY-MM-DD). |
| `current_period_start_to` | `string` | no | Current period start on or before this date (YYYY-MM-DD). |
| `current_period_end_from` | `string` | no | Current period end on or after this date (YYYY-MM-DD). |
| `current_period_end_to` | `string` | no | Current period end on or before this date (YYYY-MM-DD). |
| `trial_end_from` | `string` | no | Trial end on or after this date (YYYY-MM-DD). |
| `trial_end_to` | `string` | no | Trial end on or before this date (YYYY-MM-DD). |
| `canceled_after` | `string` | no | Canceled on or after this date (YYYY-MM-DD). |
| `canceled_before` | `string` | no | Canceled on or before this date (YYYY-MM-DD). |
| `activated_after` | `string` | no | Activated on or after this date (YYYY-MM-DD). |
| `activated_before` | `string` | no | Activated on or before this date (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of subscriptions. |
| `404` | Resource not found in the current organization. |

## subscriptions create

Create subscription

```text
kobana billing subscriptions create [OPTIONS]
```

`POST /v1/subscriptions`

Creates a subscription. Either `plan_id` or `items` is required.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | yes | UUID of the billing account that will receive this subscription. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no |  |
| `plan_id` | `string (uuid)` | no | Plan UUID. Required when `items` is not provided. |
| `product_group_id` | `string (uuid)` | no |  |
| `billing_cycle` | `string` | no | Values: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `collection_method` | `string` | no | Values: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `trial_end` | `string (date-time)` | no | Trial end date (ISO 8601). |
| `trial_period_days` | `integer` | no | Alternative to `trial_end`. Trial days from creation. |
| `proration_behavior` | `string` | no | Values: `create_prorations`, `none`, `always_invoice`. |
| `nfe_issuance_policy` | `string` | no | Values: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `items` | `array<object>` | no |  |
| `commitment_months` | `integer` | no | Commitment (loyalty) period: number of contracted months (copied from the accepted proposal). Inside the commitment window — subscription start + months — the customer cannot cancel through the portal. Use `null` to clear. |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema billing.subscriptions.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Subscription created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## subscriptions update

Upsert subscription

```text
kobana billing subscriptions update [OPTIONS]
```

`PUT /v1/subscriptions`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | UUID of the billing account that will receive this subscription. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no |  |
| `plan_id` | `string (uuid)` | no | Plan UUID. Required when `items` is not provided. |
| `product_group_id` | `string (uuid)` | no |  |
| `billing_cycle` | `string` | no | Values: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `collection_method` | `string` | no | Values: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `trial_end` | `string (date-time)` | no | Trial end date (ISO 8601). |
| `trial_period_days` | `integer` | no | Alternative to `trial_end`. Trial days from creation. |
| `proration_behavior` | `string` | no | Values: `create_prorations`, `none`, `always_invoice`. |
| `nfe_issuance_policy` | `string` | no | Values: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `items` | `array<object>` | no |  |
| `commitment_months` | `integer` | no | Commitment (loyalty) period: number of contracted months (copied from the accepted proposal). Inside the commitment window — subscription start + months — the customer cannot cancel through the portal. Use `null` to clear. |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.subscriptions.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## subscriptions get

Get subscription

```text
kobana billing subscriptions get [ID] [OPTIONS]
```

`GET /v1/subscriptions/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription detail. |
| `404` | Resource not found in the current organization. |

## subscriptions delete

Cancel and remove subscription (soft delete)

```text
kobana billing subscriptions delete [ID] [OPTIONS]
```

`DELETE /v1/subscriptions/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Soft delete confirmation. |
| `404` | Resource not found in the current organization. |

## subscriptions activate

Activate draft/confirmed subscription

```text
kobana billing subscriptions activate [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/activate`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription activated. |
| `404` | Resource not found in the current organization. |

## subscriptions cancel

Cancel subscription

```text
kobana billing subscriptions cancel [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/cancel`

Cancels immediately, or schedules for end of period when `cancel_at_period_end=true`.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `cancel_at_period_end` | `boolean` | no | When `true`, schedules cancellation for end of current period. Default: `false` (cancels immediately). |
| `reason` | `string` | no |  |

More fields and nested objects: `kobana schema billing.subscriptions.cancel`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription canceled. |
| `404` | Resource not found in the current organization. |

## subscriptions change-anniversary

Change the subscription anniversary (cycle start day)

```text
kobana billing subscriptions change-anniversary [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/change-anniversary`

Moves the day of the month on which cycles start. The cycle in progress keeps its start and is extended to the day before the new day; the added days are billed pro rata (cycle amount × days ÷ cycle days) in a `subscription_update` invoice. The change always moves the date forward, never back. Use `dry_run` to compute without applying.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `anniversary_day` | `integer` | yes | New day of the month (1 to 28) on which cycles start. Use 1 for calendar months. |
| `charge_proration` | `boolean` | no | Bill the added days. Default: `true`, unless the subscription `proration_behavior` is `none`. |
| `dry_run` | `boolean` | no | When `true`, only computes and returns the result, changing nothing. |
| `reason` | `string` | no | Change reason (optional, recorded in audit log). |

More fields and nested objects: `kobana schema billing.subscriptions.change-anniversary`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Change result (or the simulation, with `dry_run`). |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## subscriptions change-plan

Change subscription plan

```text
kobana billing subscriptions change-plan [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/change-plan`

Upgrade or downgrade. Proration follows the subscription's `proration_behavior`.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `plan_id` | `string (uuid)` | yes | UUID of the new plan. |
| `timing` | `string` | no | When the plan change takes effect. Default: `immediate`. Values: `immediate`, `end_of_period`. |
| `reason` | `string` | no | Change reason (optional, recorded in audit log). |

More fields and nested objects: `kobana schema billing.subscriptions.change-plan`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription updated. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## subscriptions create-invoice

Generate an ad-hoc invoice for the subscription

```text
kobana billing subscriptions create-invoice [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/create-invoice`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `201` | Invoice generated. |
| `404` | Resource not found in the current organization. |

## subscriptions pause

Pause subscription

```text
kobana billing subscriptions pause [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/pause`

Suspends charging while keeping the subscription open. `behavior` controls how invoices generated during the pause are handled.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `behavior` | `string` | no | How to treat invoices generated during the pause. Default: `mark_uncollectible`. Values: `mark_uncollectible`, `keep_as_draft`, `void`. |
| `resumes_at` | `string (date-time)` | no | Auto-resume date. Omit for manual resume. |

More fields and nested objects: `kobana schema billing.subscriptions.pause`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription paused. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## subscriptions resume

Resume paused subscription

```text
kobana billing subscriptions resume [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/resume`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription resumed. |
| `404` | Resource not found in the current organization. |

## subscriptions revert-to-confirmed

Revert subscription to confirmed state

```text
kobana billing subscriptions revert-to-confirmed [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/revert-to-confirmed`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription reverted. |
| `404` | Resource not found in the current organization. |

## subscriptions sync-plan-items

Resync items with the current plan

```text
kobana billing subscriptions sync-plan-items [ID] [OPTIONS]
```

`POST /v1/subscriptions/{id}/sync-plan-items`

Realigns subscription items with the plan-items template (manual reconciliation).

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Subscription synced. |
| `404` | Resource not found in the current organization. |
