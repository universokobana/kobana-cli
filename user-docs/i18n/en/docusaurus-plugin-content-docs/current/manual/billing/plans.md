---
id: plans
title: "kobana billing plans"
sidebar_label: "plans"
---

# kobana billing plans


Commands of `kobana billing plans`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`plans bulk delete`](#plans-bulk-delete) | `DELETE /v1/plans/bulk` | Bulk delete plans |
| [`plans items`](#plans-items) | `GET /v1/plans/{id}/items` | List plan items |
| [`plans items update`](#plans-items-update) | `PUT /v1/plans/{id}/items` | Replace plan items |
| [`plans list`](#plans-list) | `GET /v1/plans` | List plans |
| [`plans create`](#plans-create) | `POST /v1/plans` | Create plan |
| [`plans update`](#plans-update) | `PUT /v1/plans` | Upsert plan |
| [`plans bulk-archive`](#plans-bulk-archive) | `POST /v1/plans/bulk-archive` | Bulk archive plans |
| [`plans bulk-move-group`](#plans-bulk-move-group) | `POST /v1/plans/bulk-move-group` | Bulk move plans to another group |
| [`plans get`](#plans-get) | `GET /v1/plans/{id}` | Get plan |
| [`plans delete`](#plans-delete) | `DELETE /v1/plans/{id}` | Delete plan |
| [`plans archive`](#plans-archive) | `POST /v1/plans/{id}/archive` | Archive plan |
| [`plans duplicate`](#plans-duplicate) | `POST /v1/plans/{id}/duplicate` | Duplicate plan |

## plans bulk delete

Bulk delete plans

```text
kobana billing plans bulk delete [OPTIONS]
```

`DELETE /v1/plans/bulk`

Deletes the given plans (soft delete: `deleted_at`, the same effect as `DELETE /v1/plans/{id}` on each). Plans with a subscription that is not canceled are skipped and returned in `skipped_ids`. Requires the `delete` access level. Replaces `POST /v1/plans/bulk-delete`, which was removed.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | List of plan UUIDs. Minimum 1. |

More fields and nested objects: `kobana schema billing.plans.bulk.delete`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk delete summary. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plans items

List plan items

```text
kobana billing plans items [ID] [OPTIONS] [COMMAND]
```

`GET /v1/plans/{id}/items`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of plan items. |
| `404` | Resource not found in the current organization. |

## plans items update

Replace plan items

```text
kobana billing plans items update [ID] [OPTIONS]
```

`PUT /v1/plans/{id}/items`

Atomically replaces all plan items. Only root products are accepted and each `product_id` must be unique in the list.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `items` | `array<object>` | yes |  |

More fields and nested objects: `kobana schema billing.plans.items.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Plan items replaced. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plans list

List plans

```text
kobana billing plans list [OPTIONS]
```

`GET /v1/plans`

Returns a paginated list of the current organization's plans.

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
| `status` | `string` | no | Filter by plan status. Values: `draft`, `active`, `archived`. |
| `plan_type` | `string` | no | Filter by plan type. Values: `standard`, `custom`, `promotional`. |
| `visibility` | `string` | no | Filter by visibility. Values: `public`, `private`. |
| `plan_group_id` | `string` | no | Filter by plan group. |
| `company_id` | `string` | no | Filter by issuing company. |
| `billing_account_id` | `string` | no | Filter custom plans associated with a billing account. |
| `scoped_billing_account_id` | `string` | no | Returns plans available for the given billing account (standard + account-specific custom plans). |
| `exclude_custom_plans` | `string` | no | When `true`, excludes `custom` plan type from the results. Values: `true`, `false`. |
| `default_billing_cycle` | `string` | no | Filter by the plan default billing cycle. Values: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `external_id` | `string` | no | Filter by the plan external identifier. |
| `slug` | `string` | no | Filter by the plan slug. |
| `search` | `string` | no | Free-text search. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `updated_after` | `string` | no | Start of update date range (YYYY-MM-DD). |
| `updated_before` | `string` | no | End of update date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of plans. |
| `404` | Resource not found in the current organization. |

## plans create

Create plan

```text
kobana billing plans create [OPTIONS]
```

`POST /v1/plans`

Creates a plan for the authenticated organization.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes |  |
| `slug` | `string` | yes | Lowercase letters, digits and hyphens only. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no |  |
| `plan_group_id` | `string (uuid)` | no |  |
| `description` | `string` | no |  |
| `status` | `string` | no | Values: `draft`, `active`, `archived`. |
| `plan_type` | `string` | no | Default: `standard`. Values: `standard`, `custom`, `promotional`. |
| `billing_account_id` | `string (uuid)` | no |  |
| `visibility` | `string` | no | Default: `public`. Values: `public`, `private`. |
| `features` | `array<string>` | no |  |
| `limits` | `object` | no |  |
| `trial_period_days` | `integer` | no |  |
| `default_billing_cycle` | `string` | no | Values: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `available_billing_cycles` | `array<string>` | no |  |
| `items` | `array<object>` | no |  |
| `custom_metadata` | `object` | no | Free-form custom metadata (`object`). |

More fields and nested objects: `kobana schema billing.plans.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Plan created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plans update

Upsert plan

```text
kobana billing plans update [OPTIONS]
```

`PUT /v1/plans`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `company_id` | `string (uuid)` | no |  |
| `plan_group_id` | `string (uuid)` | no |  |
| `name` | `string` | no |  |
| `slug` | `string` | no | Lowercase letters, digits and hyphens only. |
| `description` | `string` | no |  |
| `status` | `string` | no | Values: `draft`, `active`, `archived`. |
| `plan_type` | `string` | no | Default: `standard`. Values: `standard`, `custom`, `promotional`. |
| `billing_account_id` | `string (uuid)` | no |  |
| `visibility` | `string` | no | Default: `public`. Values: `public`, `private`. |
| `features` | `array<string>` | no |  |
| `limits` | `object` | no |  |
| `trial_period_days` | `integer` | no |  |
| `default_billing_cycle` | `string` | no | Values: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `available_billing_cycles` | `array<string>` | no |  |
| `items` | `array<object>` | no |  |
| `custom_metadata` | `object` | no | Free-form custom metadata (`object`). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.plans.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plans bulk-archive

Bulk archive plans

```text
kobana billing plans bulk-archive [OPTIONS]
```

`POST /v1/plans/bulk-archive`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | List of plan UUIDs. Minimum 1. |

More fields and nested objects: `kobana schema billing.plans.bulk-archive`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk archive summary. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plans bulk-move-group

Bulk move plans to another group

```text
kobana billing plans bulk-move-group [OPTIONS]
```

`POST /v1/plans/bulk-move-group`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes |  |
| `plan_group_id` | `string (uuid)` | yes | Target group UUID. Use `null` to remove the grouping. |

More fields and nested objects: `kobana schema billing.plans.bulk-move-group`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk move summary. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plans get

Get plan

```text
kobana billing plans get [ID] [OPTIONS]
```

`GET /v1/plans/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Plan detail. |
| `404` | Resource not found in the current organization. |

## plans delete

Delete plan

```text
kobana billing plans delete [ID] [OPTIONS]
```

`DELETE /v1/plans/{id}`

Deletes the plan (soft delete: `deleted_at` — it disappears from every read; plan-change rules that cite it are removed). Requires the `delete` access level. Answers 422 while a subscription on the plan is not canceled. Until October 2026 this route archived the plan, or deleted a draft with `?hard=true` (the parameter is gone) — archiving is `POST /v1/plans/{id}/archive`, reversible.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Plan deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plans archive

Archive plan

```text
kobana billing plans archive [ID] [OPTIONS]
```

`POST /v1/plans/{id}/archive`

Archives the plan: it stays listed and can be restored, but takes no new subscriptions; existing ones keep billing. Answers 422 while active subscriptions are on the plan. Requires the `write` access level.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Plan archived. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plans duplicate

Duplicate plan

```text
kobana billing plans duplicate [ID] [OPTIONS]
```

`POST /v1/plans/{id}/duplicate`

Creates a copy of the plan (including its items) in draft state.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `201` | Plan duplicated. |
| `404` | Resource not found in the current organization. |
