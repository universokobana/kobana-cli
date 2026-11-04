---
id: subscription-changes
title: "kobana billing subscription-changes"
sidebar_label: "subscription-changes"
---

# kobana billing subscription-changes


Commands of `kobana billing subscription-changes`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`subscription-changes config list`](#subscription-changes-config-list) | `GET /v1/subscription_changes/config` | Get subscription change settings |
| [`subscription-changes config update`](#subscription-changes-config-update) | `PUT /v1/subscription_changes/config` | Update subscription change settings |

## subscription-changes config list

Get subscription change settings

```text
kobana billing subscription-changes config list [OPTIONS]
```

`GET /v1/subscription_changes/config`

Returns the organization settings for subscription item changes — allowed operations (add/remove/quantity change/price swap), proration method per operation, timing (`immediate` or `end_of_period`), refund/credit on remove and portal self-service. If not yet present, defaults are created on first read.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Organization settings. |
| `404` | Resource not found in the current organization. |

## subscription-changes config update

Update subscription change settings

```text
kobana billing subscription-changes config update [OPTIONS]
```

`PUT /v1/subscription_changes/config`

Partially updates the settings. Only supplied fields are changed. `refund_on_remove` requires `credit_on_remove` to also be enabled — refund is a variant of credit.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `allow_item_add` | `boolean` | no | Allows adding new items to an existing subscription. |
| `allow_item_remove` | `boolean` | no | Allows removing items from an existing subscription. |
| `allow_quantity_change` | `boolean` | no | Allows changing the quantity of items already on the subscription. |
| `allow_price_swap` | `boolean` | no | Allows swapping the price (variant) of an existing item without removing it. |
| `add_proration` | `string` | no | Proration method when adding items: `full_proration`, `no_proration` or `partial_proration`. Values: `full_proration`, `no_proration`, `partial_proration`. |
| `remove_proration` | `string` | no | Proration method when removing items: `full_proration`, `no_proration` or `partial_proration`. Values: `full_proration`, `no_proration`, `partial_proration`. |
| `quantity_up_proration` | `string` | no | Proration method when increasing an item quantity. Values: `full_proration`, `no_proration`, `partial_proration`. |
| `quantity_down_proration` | `string` | no | Proration method when decreasing an item quantity. Values: `full_proration`, `no_proration`, `partial_proration`. |
| `add_timing` | `string` | no | Timing when adding items: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `remove_timing` | `string` | no | Timing when removing items: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `quantity_up_timing` | `string` | no | Timing when increasing quantity: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `quantity_down_timing` | `string` | no | Timing when decreasing quantity: `immediate` or `end_of_period`. Values: `immediate`, `end_of_period`. |
| `refund_on_remove` | `boolean` | no | When `true`, issues a refund to the customer when items are removed. Requires `credit_on_remove` to be enabled. |
| `credit_on_remove` | `boolean` | no | When `true`, issues a credit in favor of the customer when items are removed. |
| `portal_self_service` | `boolean` | no | When `true`, lets customers change their own subscription from the portal. |
| `portal_require_confirmation` | `boolean` | no | When `true`, requires explicit customer confirmation before applying the change in the portal. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.subscription-changes.config.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Configuration updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
