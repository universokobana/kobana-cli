---
id: credit-purchases
title: "kobana billing credit-purchases"
sidebar_label: "credit-purchases"
---

# kobana billing credit-purchases


Commands of `kobana billing credit-purchases`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`credit-purchases create`](#credit-purchases-create) | `POST /v1/credit_purchases` | Buy credits |

## credit-purchases create

Buy credits

```text
kobana billing credit-purchases create [OPTIONS]
```

`POST /v1/credit_purchases`

Creates and finalizes a standalone invoice (`billing_reason = credit_purchase`) for a quantity of a `credits` product, independent of any subscription. The balance becomes available once the invoice is paid: the `invoice.paid` webhook carries `line_items[].product_type` and `quantity`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `product_id` | `string (uuid)` | yes | A `credits` product. |
| `quantity` | `integer` | yes | Number of credits (integer; a multiple of the product increment). |
| `billing_account_id` | `string (uuid)` | no | Billing account ID. Send this **or** `billing_account_public_id`. |
| `billing_account_public_id` | `string (uuid)` | no | Public identifier of the billing account. |
| `price_id` | `string (uuid)` | no | Price to charge. Default: the product active default price. |
| `collection_method` | `string` | no | `manual_charge` (default): the customer pays the invoice. `charge_automatically`: charges the default card on finalization. Values: `charge_automatically`, `manual_charge`. |
| `due_date` | `string` | no | Due date. Default: the account payment terms. |

More fields and nested objects: `kobana schema billing.credit-purchases.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Purchase invoice, with its line items. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
