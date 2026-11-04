---
id: late-fee-policies
title: "kobana billing late-fee-policies"
sidebar_label: "late-fee-policies"
---

# kobana billing late-fee-policies


Commands of `kobana billing late-fee-policies`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`late-fee-policies list`](#late-fee-policies-list) | `GET /v1/late_fee_policies` | List late fee policies |
| [`late-fee-policies create`](#late-fee-policies-create) | `POST /v1/late_fee_policies` | Create a late fee policy |
| [`late-fee-policies simulate`](#late-fee-policies-simulate) | `POST /v1/late_fee_policies/simulate` | Simulate late charges |
| [`late-fee-policies get`](#late-fee-policies-get) | `GET /v1/late_fee_policies/{id}` | Get a late fee policy |
| [`late-fee-policies update`](#late-fee-policies-update) | `PATCH /v1/late_fee_policies/{id}` | Update a late fee policy |
| [`late-fee-policies delete`](#late-fee-policies-delete) | `DELETE /v1/late_fee_policies/{id}` | Delete a late fee policy |
| [`late-fee-policies default`](#late-fee-policies-default) | `POST /v1/late_fee_policies/{id}/default` | Set as the default policy |

## late-fee-policies list

List late fee policies

```text
kobana billing late-fee-policies list [OPTIONS]
```

`GET /v1/late_fee_policies`

Returns the organization late fee policies. The policy that applies to an invoice is resolved in cascade: invoice → subscription → billing account → organization default. A null level means inherit, never "no charges" — charging nothing is an explicit policy with both modes set to `none`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `include_deleted` | `string` | no | Includes soft-deleted policies. Values: `true`, `false`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of late fee policies. |
| `404` | Resource not found in the current organization. |

## late-fee-policies create

Create a late fee policy

```text
kobana billing late-fee-policies create [OPTIONS]
```

`POST /v1/late_fee_policies`

Creates a policy. Fine and interest above the legal ceilings (2% fine and 1% monthly interest, Brazilian Consumer Code art. 52) are refused unless `enforce_legal_caps` is `false` — which is audited separately, being a legal-risk decision.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Policy name, unique within the organization. |
| `is_default` | `boolean` | no | Whether this is the organization default policy. |
| `fine_mode` | `string` | no | How the fine is computed. Charged once. `percentage` uses `fine_percent`, `fixed` uses `fine_amount`. Values: `none`, `percentage`, `fixed`. |
| `fine_percent` | `number` | no | Fine as a percentage of the outstanding balance, 0 to 100. Populated only when `fine_mode` is `percentage`. |
| `fine_amount` | `number` | no | Flat fine amount, in BRL. Populated only when `fine_mode` is `fixed`. |
| `fine_days` | `integer` | no | Days past the due date when the fine starts applying. Default is 1 — the day after the due date. |
| `interest_mode` | `string` | no | How interest accrues. The percentage modes use `interest_percent`; `daily_amount` uses `interest_amount`. Interest is charged on the principal alone, never on principal plus fine. Values: `none`, `daily_amount`, `daily_percentage`, `monthly_percentage`, `yearly_percentage`. |
| `interest_percent` | `number` | no | Interest percentage per day, month or year, per `interest_mode`. The monthly rate is divided pro rata die by 30; the yearly one by 360. |
| `interest_amount` | `number` | no | Flat interest amount per day, in BRL. Populated only when `interest_mode` is `daily_amount`. |
| `interest_days` | `integer` | no | Days past the due date when interest starts accruing. Default is 1. |
| `day_count` | `string` | no | Day-count basis for the delay. Only `calendar` is accepted for now: the local accrual has no holiday calendar yet, and counting business days without one would overstate the charge against what the bank collects. Values: `calendar`, `business`. |
| `max_interest_months` | `integer` | no | Ceiling on accumulated months of interest. Null means uncapped. |
| `max_total_charge_percent` | `number` | no | Ceiling on the total charge (fine plus interest) as a percentage of the outstanding balance. The fine is kept whole and the interest is trimmed when the cap bites. |
| `enforce_legal_caps` | `boolean` | no | When true, refuses a fine above 2% and interest above 1% per month. Turning it off is a deliberate choice for contracts outside consumer relations, and is audited. |
| `partial_allocation` | `string` | no | Imputation order when a payment covers neither principal nor charges. `charges_first` follows art. 354 of the Brazilian Civil Code: interest before capital. Values: `charges_first`, `principal_first`. |
| `register_on_charge` | `boolean` | no | Registers the fine and interest instruction with the bank when issuing a bank slip or Pix, so the charge itself collects them after the due date. |
| `apply_on_dynamic_charge` | `boolean` | no | Recomputes the charged amount on card charges and re-issues, where no bank instruction can collect the charge. |
| `charge_instructions` | `string` | no | Free text printed in the bank slip instructions. |

More fields and nested objects: `kobana schema billing.late-fee-policies.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Policy created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## late-fee-policies simulate

Simulate late charges

```text
kobana billing late-fee-policies simulate [OPTIONS]
```

`POST /v1/late_fee_policies/simulate`

Computes what an outstanding balance owes in fine and interest on a given date, creating nothing. Pass either `policy_id` or an inline `policy`. Use it to reconcile the arithmetic before an invoice actually goes late.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `principal` | `number` | yes | Outstanding balance to accrue the charges on, in BRL. |
| `due_date` | `string (date-time)` | yes | Due date the delay is counted from. |
| `reference_date` | `string (date-time)` | no | Date to accrue at. Defaults to now. Days are counted on the civil date in America/Sao_Paulo. |
| `policy_id` | `string (uuid)` | no | Existing policy to use. Alternative to `policy`. |
| `policy` | `object` | no |  |

More fields and nested objects: `kobana schema billing.late-fee-policies.simulate`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Charges accrued at the given date. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## late-fee-policies get

Get a late fee policy

```text
kobana billing late-fee-policies get [ID] [OPTIONS]
```

`GET /v1/late_fee_policies/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Policy found. |
| `404` | Resource not found in the current organization. |

## late-fee-policies update

Update a late fee policy

```text
kobana billing late-fee-policies update [ID] [OPTIONS]
```

`PATCH /v1/late_fee_policies/{id}`

Updates the policy. Invoices already finalized are unaffected: each invoice freezes the policy in force at finalization, so a renegotiation cannot retroactively change what was already issued.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Policy name, unique within the organization. |
| `is_default` | `boolean` | no | Whether this is the organization default policy. |
| `fine_mode` | `string` | no | How the fine is computed. Charged once. `percentage` uses `fine_percent`, `fixed` uses `fine_amount`. Values: `none`, `percentage`, `fixed`. |
| `fine_percent` | `number` | no | Fine as a percentage of the outstanding balance, 0 to 100. Populated only when `fine_mode` is `percentage`. |
| `fine_amount` | `number` | no | Flat fine amount, in BRL. Populated only when `fine_mode` is `fixed`. |
| `fine_days` | `integer` | no | Days past the due date when the fine starts applying. Default is 1 — the day after the due date. |
| `interest_mode` | `string` | no | How interest accrues. The percentage modes use `interest_percent`; `daily_amount` uses `interest_amount`. Interest is charged on the principal alone, never on principal plus fine. Values: `none`, `daily_amount`, `daily_percentage`, `monthly_percentage`, `yearly_percentage`. |
| `interest_percent` | `number` | no | Interest percentage per day, month or year, per `interest_mode`. The monthly rate is divided pro rata die by 30; the yearly one by 360. |
| `interest_amount` | `number` | no | Flat interest amount per day, in BRL. Populated only when `interest_mode` is `daily_amount`. |
| `interest_days` | `integer` | no | Days past the due date when interest starts accruing. Default is 1. |
| `day_count` | `string` | no | Day-count basis for the delay. Only `calendar` is accepted for now: the local accrual has no holiday calendar yet, and counting business days without one would overstate the charge against what the bank collects. Values: `calendar`, `business`. |
| `max_interest_months` | `integer` | no | Ceiling on accumulated months of interest. Null means uncapped. |
| `max_total_charge_percent` | `number` | no | Ceiling on the total charge (fine plus interest) as a percentage of the outstanding balance. The fine is kept whole and the interest is trimmed when the cap bites. |
| `enforce_legal_caps` | `boolean` | no | When true, refuses a fine above 2% and interest above 1% per month. Turning it off is a deliberate choice for contracts outside consumer relations, and is audited. |
| `partial_allocation` | `string` | no | Imputation order when a payment covers neither principal nor charges. `charges_first` follows art. 354 of the Brazilian Civil Code: interest before capital. Values: `charges_first`, `principal_first`. |
| `register_on_charge` | `boolean` | no | Registers the fine and interest instruction with the bank when issuing a bank slip or Pix, so the charge itself collects them after the due date. |
| `apply_on_dynamic_charge` | `boolean` | no | Recomputes the charged amount on card charges and re-issues, where no bank instruction can collect the charge. |
| `charge_instructions` | `string` | no | Free text printed in the bank slip instructions. |

More fields and nested objects: `kobana schema billing.late-fee-policies.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Policy updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## late-fee-policies delete

Delete a late fee policy

```text
kobana billing late-fee-policies delete [ID] [OPTIONS]
```

`DELETE /v1/late_fee_policies/{id}`

Soft delete. Invoices already issued keep working from the snapshot they carry. The default policy cannot be deleted — set another as default first.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Policy deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## late-fee-policies default

Set as the default policy

```text
kobana billing late-fee-policies default [ID] [OPTIONS]
```

`POST /v1/late_fee_policies/{id}/default`

Makes this the organization default — what an invoice inherits when neither it, its subscription nor its billing account names one. Exactly one policy holds the flag.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Policy set as default. |
| `404` | Resource not found in the current organization. |
