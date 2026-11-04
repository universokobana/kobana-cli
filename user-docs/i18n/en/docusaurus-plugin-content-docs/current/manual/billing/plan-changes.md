---
id: plan-changes
title: "kobana billing plan-changes"
sidebar_label: "plan-changes"
---

# kobana billing plan-changes


Commands of `kobana billing plan-changes`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`plan-changes config`](#plan-changes-config) | `GET /v1/plan_changes/config` | Get plan change config |
| [`plan-changes config update`](#plan-changes-config-update) | `PATCH /v1/plan_changes/config` | Update plan change config |
| [`plan-changes rules list`](#plan-changes-rules-list) | `GET /v1/plan_changes/rules` | List plan change rules |
| [`plan-changes rules create`](#plan-changes-rules-create) | `POST /v1/plan_changes/rules` | Create plan change rule |
| [`plan-changes rules get`](#plan-changes-rules-get) | `GET /v1/plan_changes/rules/{id}` | Get plan change rule |
| [`plan-changes rules update`](#plan-changes-rules-update) | `PATCH /v1/plan_changes/rules/{id}` | Update plan change rule |
| [`plan-changes rules delete`](#plan-changes-rules-delete) | `DELETE /v1/plan_changes/rules/{id}` | Delete plan change rule |
| [`plan-changes stats`](#plan-changes-stats) | `GET /v1/plan_changes/stats` | Plan change stats |
| [`plan-changes list`](#plan-changes-list) | `GET /v1/plan_changes` | List plan changes |
| [`plan-changes create`](#plan-changes-create) | `POST /v1/plan_changes` | Execute plan change |
| [`plan-changes get`](#plan-changes-get) | `GET /v1/plan_changes/{id}` | Get plan change |

## plan-changes config

Get plan change config

```text
kobana billing plan-changes config [OPTIONS] [COMMAND]
```

`GET /v1/plan_changes/config`

Returns the organization's default configuration for upgrades and downgrades — proration behavior, timing and credit/refund generation.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Current configuration. |
| `404` | Resource not found in the current organization. |

## plan-changes config update

Update plan change config

```text
kobana billing plan-changes config update [OPTIONS]
```

`PATCH /v1/plan_changes/config`

Partially updates (PATCH) the organization's plan change configuration. All fields are optional.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `allow_upgrade` | `boolean` | no | Enables plan upgrades for the organization. |
| `allow_downgrade` | `boolean` | no | Enables plan downgrades for the organization. |
| `upgrade_proration` | `string` | no | Default proration method for upgrades: `full_proration`, `no_proration` or `partial_proration`. Values: `full_proration`, `no_proration`, `partial_proration`. |
| `downgrade_proration` | `string` | no | Default proration method for downgrades. Values: `full_proration`, `no_proration`, `partial_proration`. |
| `upgrade_timing` | `string` | no | When to apply upgrades: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `downgrade_timing` | `string` | no | When to apply downgrades: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `refund_on_downgrade` | `boolean` | no | Refunds the difference when downgrading. |
| `credit_on_downgrade` | `boolean` | no | Generates credit (instead of refund) when downgrading. |
| `apply_discount_on_change` | `boolean` | no | Keeps active discounts when changing plans. |

More fields and nested objects: `kobana schema billing.plan-changes.config.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Configuration updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plan-changes rules list

List plan change rules

```text
kobana billing plan-changes rules list [OPTIONS]
```

`GET /v1/plan_changes/rules`

Lists custom rules that override the default behavior for specific plan-to-plan transitions.

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of plan change rules. |
| `404` | Resource not found in the current organization. |

## plan-changes rules create

Create plan change rule

```text
kobana billing plan-changes rules create [OPTIONS]
```

`POST /v1/plan_changes/rules`

Creates a new rule for a plan-to-plan transition. Lets you block/allow the transition, set timing, proration, discounts and bonus days.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `from_plan_id` | `string (uuid)` | no | UUID of the source plan. When omitted, the rule covers any source plan. |
| `to_plan_id` | `string (uuid)` | no | UUID of the target plan. When omitted, the rule covers any target plan. |
| `change_type` | `string` | no | Change type: `upgrade`, `downgrade` or `lateral`. Values: `upgrade`, `downgrade`, `lateral`. |
| `allowed` | `boolean` | no | When `false`, explicitly blocks the transition. |
| `timing` | `string` | no | Overrides the change timing: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `proration_method` | `string` | no | Overrides the proration method: `full_proration`, `no_proration` or `partial_proration`. Values: `full_proration`, `no_proration`, `partial_proration`. |
| `discount_percent` | `number` | no | Percent discount applied when this rule matches (0 to 100). |
| `bonus_days` | `integer` | no | Bonus days added to the current period when this rule is applied. |
| `message` | `string` | no | Free-form message shown in the portal/checkout (e.g.: block reason). |
| `priority` | `integer` | no | Priority for conflict resolution when multiple rules match (higher wins). |

More fields and nested objects: `kobana schema billing.plan-changes.rules.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Rule created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plan-changes rules get

Get plan change rule

```text
kobana billing plan-changes rules get [ID] [OPTIONS]
```

`GET /v1/plan_changes/rules/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Rule detail. |
| `404` | Resource not found in the current organization. |

## plan-changes rules update

Update plan change rule

```text
kobana billing plan-changes rules update [ID] [OPTIONS]
```

`PATCH /v1/plan_changes/rules/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `allowed` | `boolean` | no | When `false`, explicitly blocks the transition. |
| `timing` | `string` | no | Overrides the change timing: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `proration_method` | `string` | no | Overrides the proration method: `full_proration`, `no_proration` or `partial_proration`. Values: `full_proration`, `no_proration`, `partial_proration`. |
| `discount_percent` | `number` | no | Percent discount applied when this rule matches (0 to 100). |
| `bonus_days` | `integer` | no | Bonus days added to the current period when this rule is applied. |
| `message` | `string` | no | Free-form message shown in the portal/checkout (e.g.: block reason). |
| `priority` | `integer` | no | Priority for conflict resolution when multiple rules match (higher wins). |
| `is_active` | `boolean` | no | Deactivates the rule without removing it. |

More fields and nested objects: `kobana schema billing.plan-changes.rules.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Rule updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plan-changes rules delete

Delete plan change rule

```text
kobana billing plan-changes rules delete [ID] [OPTIONS]
```

`DELETE /v1/plan_changes/rules/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Rule removed. |
| `404` | Resource not found in the current organization. |

## plan-changes stats

Plan change stats

```text
kobana billing plan-changes stats [OPTIONS]
```

`GET /v1/plan_changes/stats`

Returns plan change aggregates (totals by type, by status, proration credits and charges) for a window in days.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `days` | `integer` | no | Window in days for the aggregates (default: `30`). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Aggregated plan change stats. |
| `404` | Resource not found in the current organization. |

## plan-changes list

List plan changes

```text
kobana billing plan-changes list [OPTIONS]
```

`GET /v1/plan_changes`

Returns the paginated history of plan changes (upgrades, downgrades and lateral) for the current organization.

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
| `subscription_id` | `string` | no | Filter by changes of a subscription. |
| `billing_account_id` | `string` | no | Filter by changes of a billing account. |
| `from_plan_id` | `string` | no | Filter by source plan (UUID). |
| `to_plan_id` | `string` | no | Filter by target plan (UUID). |
| `change_type` | `string` | no | Filter by type: `upgrade`, `downgrade` or `lateral`. Values: `upgrade`, `downgrade`, `lateral`. |
| `status` | `string` | no | Filter by status: `pending`, `scheduled`, `completed`, `canceled` or `failed`. Values: `pending`, `scheduled`, `completed`, `canceled`, `failed`. |
| `timing` | `string` | no | Filter by timing: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `proration_method` | `string` | no | Filter by proration method (e.g. `full_proration`, `no_proration`, `partial_proration`). |
| `price_difference_min` | `number,null` | no | Minimum price difference in R$ (sub-cents ÷10000). E.g. `10.50`. |
| `price_difference_max` | `number,null` | no | Maximum price difference in R$ (sub-cents ÷10000). E.g. `500.00`. |
| `proration_charge_min` | `number,null` | no | Minimum proration charge in R$ (sub-cents ÷10000). |
| `proration_charge_max` | `number,null` | no | Maximum proration charge in R$ (sub-cents ÷10000). |
| `proration_credit_min` | `number,null` | no | Minimum proration credit in R$ (sub-cents ÷10000). |
| `proration_credit_max` | `number,null` | no | Maximum proration credit in R$ (sub-cents ÷10000). |
| `effective_date_from` | `string` | no | Effective date from (inclusive). Format: `YYYY-MM-DD`. |
| `effective_date_to` | `string` | no | Effective date to (inclusive). Format: `YYYY-MM-DD`. |
| `scheduled_date_from` | `string` | no | Scheduled date from (inclusive). Format: `YYYY-MM-DD`. |
| `scheduled_date_to` | `string` | no | Scheduled date to (inclusive). Format: `YYYY-MM-DD`. |
| `completed_after` | `string` | no | Completed from (inclusive). Format: `YYYY-MM-DD`. |
| `completed_before` | `string` | no | Completed to (inclusive). Format: `YYYY-MM-DD`. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of plan changes. |
| `404` | Resource not found in the current organization. |

## plan-changes create

Execute plan change

```text
kobana billing plan-changes create [OPTIONS]
```

`POST /v1/plan_changes`

Executes a plan change for an existing subscription. `timing` controls whether the change applies immediately (with proration) or at the end of the current period. On success, returns the UUIDs of the `plan_change`, the proration invoice (if any) and the generated credit (if any).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `subscription_id` | `string (uuid)` | yes | UUID of the subscription that will have its plan changed. |
| `to_plan_id` | `string (uuid)` | yes | UUID of the target plan. |
| `timing` | `string` | no | When to apply the change: `immediate` (default for upgrades) or `end_of_period` (default for downgrades). Values: `immediate`, `end_of_period`. |
| `reason` | `string` | no | Reason for the change (recorded in the audit log). |

More fields and nested objects: `kobana schema billing.plan-changes.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Plan change executed. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plan-changes get

Get plan change

```text
kobana billing plan-changes get [ID] [OPTIONS]
```

`GET /v1/plan_changes/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Plan change detail. |
| `404` | Resource not found in the current organization. |
