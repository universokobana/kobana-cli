---
id: subscription-item-changes
title: "kobana billing subscription-item-changes"
sidebar_label: "subscription-item-changes"
---

# kobana billing subscription-item-changes


Commands of `kobana billing subscription-item-changes`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`subscription-item-changes list`](#subscription-item-changes-list) | `GET /v1/subscription_item_changes` | List subscription item changes |
| [`subscription-item-changes get`](#subscription-item-changes-get) | `GET /v1/subscription_item_changes/{id}` | Get subscription item change |
| [`subscription-item-changes cancel`](#subscription-item-changes-cancel) | `POST /v1/subscription_item_changes/{id}/cancel` | Cancel a scheduled change |

## subscription-item-changes list

List subscription item changes

```text
kobana billing subscription-item-changes list [OPTIONS]
```

`GET /v1/subscription_item_changes`

Returns a paginated list of changes applied (or scheduled) to the organization's subscription items — additions, removals, quantity changes and price swaps. Each record includes the operations snapshot, proration amounts in BRL (float, up to 4 decimal places) and pointers to the resulting invoice/credit.

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
| `subscription_id` | `string` | no | Filter by changes of a specific subscription. |
| `billing_account_id` | `string` | no | Filter by changes of a specific billing account. |
| `status` | `string` | no | Filter by status: `pending`, `scheduled`, `completed`, `canceled` or `failed`. Values: `pending`, `scheduled`, `completed`, `canceled`, `failed`. |
| `timing` | `string` | no | Filter by the timing of the change application: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `initiated_by_role` | `string` | no | Filter by the origin of the change: `dashboard`, `portal` or `api`. Values: `dashboard`, `portal`, `api`. |
| `proration_charge_min` | `number,null` | no | Minimum proration charge amount in BRL (R$). Internally stored in BRL (float, up to 4 decimal places). |
| `proration_charge_max` | `number,null` | no | Maximum proration charge amount in BRL (R$). Internally stored in BRL (float, up to 4 decimal places). |
| `proration_credit_min` | `number,null` | no | Minimum proration credit amount in BRL (R$). Internally stored in BRL (float, up to 4 decimal places). |
| `proration_credit_max` | `number,null` | no | Maximum proration credit amount in BRL (R$). Internally stored in BRL (float, up to 4 decimal places). |
| `effective_date_from` | `string` | no | Minimum effective date of the change (inclusive), in `YYYY-MM-DD` format. |
| `effective_date_to` | `string` | no | Maximum effective date of the change (inclusive), in `YYYY-MM-DD` format. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of item changes. |
| `404` | Resource not found in the current organization. |

## subscription-item-changes get

Get subscription item change

```text
kobana billing subscription-item-changes get [ID] [OPTIONS]
```

`GET /v1/subscription_item_changes/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Item change detail. |
| `404` | Resource not found in the current organization. |

## subscription-item-changes cancel

Cancel a scheduled change

```text
kobana billing subscription-item-changes cancel [ID] [OPTIONS]
```

`POST /v1/subscription_item_changes/{id}/cancel`

Cancels an item change that is still scheduled (`timing=end_of_period`). Changes already applied cannot be canceled. Returns the record with its new status. Requires the `write` access level. Replaces `DELETE /v1/subscription_item_changes/{id}`, which was removed.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Change canceled. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
