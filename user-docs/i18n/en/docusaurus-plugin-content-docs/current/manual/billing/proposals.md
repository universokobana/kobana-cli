---
id: proposals
title: "kobana billing proposals"
sidebar_label: "proposals"
---

# kobana billing proposals


Commands of `kobana billing proposals`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`proposals bulk delete`](#proposals-bulk-delete) | `DELETE /v1/proposals/bulk` | Bulk delete proposals |
| [`proposals items list`](#proposals-items-list) | `GET /v1/proposals/{id}/items` | List proposal items |
| [`proposals items create`](#proposals-items-create) | `POST /v1/proposals/{id}/items` | Add item to proposal |
| [`proposals items get`](#proposals-items-get) | `GET /v1/proposals/{id}/items/{item_id}` | Get proposal item |
| [`proposals items update`](#proposals-items-update) | `PATCH /v1/proposals/{id}/items/{item_id}` | Update proposal item |
| [`proposals items delete`](#proposals-items-delete) | `DELETE /v1/proposals/{id}/items/{item_id}` | Delete proposal item |
| [`proposals stats`](#proposals-stats) | `GET /v1/proposals/stats` | Get proposal statistics |
| [`proposals list`](#proposals-list) | `GET /v1/proposals` | List proposals |
| [`proposals create`](#proposals-create) | `POST /v1/proposals` | Create proposal |
| [`proposals update`](#proposals-update) | `PUT /v1/proposals` | Upsert proposal |
| [`proposals bulk-cancel`](#proposals-bulk-cancel) | `POST /v1/proposals/bulk-cancel` | Bulk cancel proposals |
| [`proposals bulk-send`](#proposals-bulk-send) | `POST /v1/proposals/bulk-send` | Bulk send proposals |
| [`proposals get`](#proposals-get) | `GET /v1/proposals/{id}` | Get proposal |
| [`proposals delete`](#proposals-delete) | `DELETE /v1/proposals/{id}` | Delete proposal |
| [`proposals accept`](#proposals-accept) | `POST /v1/proposals/{id}/accept` | Accept proposal |
| [`proposals cancel`](#proposals-cancel) | `POST /v1/proposals/{id}/cancel` | Cancel proposal |
| [`proposals duplicate`](#proposals-duplicate) | `POST /v1/proposals/{id}/duplicate` | Duplicate proposal |
| [`proposals generate-pdf`](#proposals-generate-pdf) | `POST /v1/proposals/{id}/generate-pdf` | Generate proposal PDF |
| [`proposals json`](#proposals-json) | `GET /v1/proposals/{id}/json` | Get proposal JSON snapshot |
| [`proposals pricing`](#proposals-pricing) | `GET /v1/proposals/{id}/pricing` | Get proposal pricing breakdown |
| [`proposals publish`](#proposals-publish) | `POST /v1/proposals/{id}/publish` | Publish proposal |
| [`proposals send`](#proposals-send) | `POST /v1/proposals/{id}/send` | Send proposal email |
| [`proposals undo-accept`](#proposals-undo-accept) | `POST /v1/proposals/{id}/undo-accept` | Undo proposal acceptance |
| [`proposals unpublish`](#proposals-unpublish) | `POST /v1/proposals/{id}/unpublish` | Unpublish proposal |

## proposals bulk delete

Bulk delete proposals

```text
kobana billing proposals bulk delete [OPTIONS]
```

`DELETE /v1/proposals/bulk`

Deletes multiple proposals at once (up to 100). Returns success/failure counts and errors per id. Requires the `delete` access level. Replaces `POST /v1/proposals/bulk-delete`, which was removed.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | List of target proposal UUIDs. Minimum 1, maximum 100. |

More fields and nested objects: `kobana schema billing.proposals.bulk.delete`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk delete result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals items list

List proposal items

```text
kobana billing proposals items list [ID] [OPTIONS]
```

`GET /v1/proposals/{id}/items`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of items. |
| `404` | Resource not found in the current organization. |

## proposals items create

Add item to proposal

```text
kobana billing proposals items create [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/items`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `product_id` | `string (uuid)` | yes | Product UUID. **Required**. |
| `price_id` | `string (uuid)` | yes | Price UUID. **Required**. |
| `unit_amount` | `number` | yes | Unit amount in sub-cents (float, up to 4 decimal places). |
| `quantity` | `number` | no | Quantity. Default: 1. |
| `metadata` | `object` | no | Arbitrary item metadata. |
| `custom_metadata` | `object` | no | Item custom fields. |

More fields and nested objects: `kobana schema billing.proposals.items.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Item created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals items get

Get proposal item

```text
kobana billing proposals items get [ID] [ITEM_ID] [OPTIONS]
```

`GET /v1/proposals/{id}/items/{item_id}`

**Path arguments:** `id`, `item_id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Item detail. |
| `404` | Resource not found in the current organization. |

## proposals items update

Update proposal item

```text
kobana billing proposals items update [ID] [ITEM_ID] [OPTIONS]
```

`PATCH /v1/proposals/{id}/items/{item_id}`

Updates an item. Only allowed on draft (`status = draft`) proposals.

**Path arguments:** `id`, `item_id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `quantity` | `integer` | no | Update quantity. |
| `unit_amount` | `number` | no | Update unit amount in sub-cents (float, up to 4 decimal places). |
| `setup_amount` | `number` | no | Update unit setup amount in sub-cents (float, up to 4 decimal places). |
| `custom_metadata` | `object` | no | proposals.items.update.body.custom_metadata |

More fields and nested objects: `kobana schema billing.proposals.items.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Item updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals items delete

Delete proposal item

```text
kobana billing proposals items delete [ID] [ITEM_ID] [OPTIONS]
```

`DELETE /v1/proposals/{id}/items/{item_id}`

Removes an item. Only allowed on draft (`status = draft`) proposals.

**Path arguments:** `id`, `item_id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Item deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals stats

Get proposal statistics

```text
kobana billing proposals stats [OPTIONS]
```

`GET /v1/proposals/stats`

Returns proposal counts by status (`draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired`, `canceled`) for the current organization.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal statistics. |
| `404` | Resource not found in the current organization. |

## proposals list

List proposals

```text
kobana billing proposals list [OPTIONS]
```

`GET /v1/proposals`

Returns a paginated list of the current organization's commercial proposals.

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
| `status` | `string` | no | Filter by status: `draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired` or `canceled`. Values: `draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired`, `canceled`. |
| `billing_account_id` | `string` | no | Filter by linked `billing_account_id`. |
| `prospect_id` | `string` | no | Filter by `prospect_id`. |
| `contact_id` | `string` | no | Filter by linked `contact_id`. |
| `coupon_id` | `string` | no | Filter by applied `coupon_id`. |
| `company_id` | `string` | no | Filter by issuing `company_id`. |
| `template_id` | `string` | no | Filter by `template_id` used to generate the PDF. |
| `code` | `string` | no | Filter by proposal short code (e.g.: `PROP-0001`). |
| `external_id` | `string` | no | Filter by external identifier. |
| `search` | `string` | no | Free-text search. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `expires_after` | `string` | no | Filter by start `expires_at` (ISO 8601, inclusive). |
| `expires_before` | `string` | no | Filter by end `expires_at` (ISO 8601, inclusive). |
| `accepted_after` | `string` | no | Filter by start `accepted_at` (ISO 8601, inclusive). |
| `accepted_before` | `string` | no | Filter by end `accepted_at` (ISO 8601, inclusive). |
| `sent_after` | `string` | no | Filter by start `sent_at` (ISO 8601, inclusive). |
| `sent_before` | `string` | no | Filter by end `sent_at` (ISO 8601, inclusive). |
| `viewed_after` | `string` | no | Filter by start `viewed_at` (ISO 8601, inclusive). |
| `viewed_before` | `string` | no | Filter by end `viewed_at` (ISO 8601, inclusive). |
| `rejected_after` | `string` | no | Filter by start `rejected_at` (ISO 8601, inclusive). |
| `rejected_before` | `string` | no | Filter by end `rejected_at` (ISO 8601, inclusive). |
| `setup_amount_min` | `number,null` | no | Filter by minimum `setup_amount` (in BRL). |
| `setup_amount_max` | `number,null` | no | Filter by maximum `setup_amount` (in BRL). |
| `monthly_amount_min` | `number,null` | no | Filter by minimum `monthly_amount` (in BRL). |
| `monthly_amount_max` | `number,null` | no | Filter by maximum `monthly_amount` (in BRL). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of proposals. |
| `404` | Resource not found in the current organization. |

## proposals create

Create proposal

```text
kobana billing proposals create [OPTIONS]
```

`POST /v1/proposals`

Creates a commercial proposal. You must provide `billing_account_id`, `prospect_id` or `prospect_data` (inline prospect creation).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string (uuid)` | no | Issuing company UUID. Use `null` to clear. |
| `title` | `string` | no | Proposal title. Default: "Proposta Comercial". |
| `description` | `string` | no | Free-form proposal description. |
| `billing_account_id` | `string (uuid)` | no | Target billing account UUID (existing customer). Required if `prospect_id` and `prospect_data` are not provided. |
| `prospect_id` | `string (uuid)` | no | UUID of an existing prospect. Alternative to `billing_account_id`/`prospect_data`. |
| `contact_id` | `string (uuid)` | no | UUID of an existing contact linked to the prospect. |
| `prospect_data` | `object` | no | Data to create a prospect inline (juridical person). Alternative to `prospect_id`/`billing_account_id`. |
| `contact_data` | `object` | no | Data to create a contact inline (natural person in charge). |
| `coupon_id` | `string (uuid)` | no | Coupon UUID to apply. |
| `template_id` | `string (uuid)` | no | Template UUID for PDF generation. |
| `discount_percent` | `number` | no | Negotiated discount percent (0–100), applied to the monthly amount; the cycle discount applies over the discounted value. Do not send together with `discount_amount`. |
| `discount_amount` | `number` | no | Negotiated fixed per-month discount in BRL (float, up to 4 decimal places), applied to the monthly amount; the cycle discount applies over the discounted value. Do not send together with `discount_percent`. |
| `commitment_months` | `integer` | no | Commitment period in months (1–120). Use `null` to clear. Copied to the subscription when the proposal is accepted. |
| `expires_at` | `string` | no | Expiration date. |
| `notes` | `string` | no | Internal notes. |
| `items` | `array<object>` | no | Proposal items. Minimum 1. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Custom fields (validated against the organization's schema). |

More fields and nested objects: `kobana schema billing.proposals.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Proposal created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals update

Upsert proposal

```text
kobana billing proposals update [OPTIONS]
```

`PUT /v1/proposals`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string (uuid)` | no | Issuing company UUID. Use `null` to clear. |
| `title` | `string` | no | Proposal title. Default: "Proposta Comercial". |
| `description` | `string` | no | Free-form proposal description. |
| `billing_account_id` | `string (uuid)` | no | Target billing account UUID (existing customer). Required if `prospect_id` and `prospect_data` are not provided. |
| `prospect_id` | `string (uuid)` | no | UUID of an existing prospect. Alternative to `billing_account_id`/`prospect_data`. |
| `contact_id` | `string (uuid)` | no | UUID of an existing contact linked to the prospect. |
| `prospect_data` | `object` | no | Data to create a prospect inline (juridical person). Alternative to `prospect_id`/`billing_account_id`. |
| `contact_data` | `object` | no | Data to create a contact inline (natural person in charge). |
| `coupon_id` | `string (uuid)` | no | Coupon UUID to apply. |
| `template_id` | `string (uuid)` | no | Template UUID for PDF generation. |
| `discount_percent` | `number` | no | Negotiated discount percent (0–100), applied to the monthly amount; the cycle discount applies over the discounted value. Do not send together with `discount_amount`. |
| `discount_amount` | `number` | no | Negotiated fixed per-month discount in BRL (float, up to 4 decimal places), applied to the monthly amount; the cycle discount applies over the discounted value. Do not send together with `discount_percent`. |
| `commitment_months` | `integer` | no | Commitment period in months (1–120). Use `null` to clear. Copied to the subscription when the proposal is accepted. |
| `expires_at` | `string` | no | Expiration date. |
| `notes` | `string` | no | Internal notes. |
| `items` | `array<object>` | no | Proposal items. Minimum 1. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Custom fields (validated against the organization's schema). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.proposals.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals bulk-cancel

Bulk cancel proposals

```text
kobana billing proposals bulk-cancel [OPTIONS]
```

`POST /v1/proposals/bulk-cancel`

Cancels multiple proposals at once (up to 100). Returns success/failure counts and errors per id.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | List of target proposal UUIDs. Minimum 1, maximum 100. |

More fields and nested objects: `kobana schema billing.proposals.bulk-cancel`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk cancel result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals bulk-send

Bulk send proposals

```text
kobana billing proposals bulk-send [OPTIONS]
```

`POST /v1/proposals/bulk-send`

Sends multiple proposals at once (up to 100). Returns success/failure counts and errors per id.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | List of target proposal UUIDs. Minimum 1, maximum 100. |

More fields and nested objects: `kobana schema billing.proposals.bulk-send`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk send result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals get

Get proposal

```text
kobana billing proposals get [ID] [OPTIONS]
```

`GET /v1/proposals/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal detail. |
| `404` | Resource not found in the current organization. |

## proposals delete

Delete proposal

```text
kobana billing proposals delete [ID] [OPTIONS]
```

`DELETE /v1/proposals/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal deleted. |
| `404` | Resource not found in the current organization. |

## proposals accept

Accept proposal

```text
kobana billing proposals accept [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/accept`

Accepts the proposal and creates (or confirms) the corresponding subscription using the given cycle.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `cycle` | `string` | yes | Billing cycle of the subscription created on acceptance: `monthly`, `quarterly`, `semiannual` or `annual`. Values: `monthly`, `quarterly`, `semiannual`, `annual`. |

More fields and nested objects: `kobana schema billing.proposals.accept`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal accepted — returns the proposal, `subscription_id` and `invoice_id` when applicable. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals cancel

Cancel proposal

```text
kobana billing proposals cancel [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/cancel`

Cancels the proposal (`status → canceled`).

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal canceled. |
| `404` | Resource not found in the current organization. |

## proposals duplicate

Duplicate proposal

```text
kobana billing proposals duplicate [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/duplicate`

Creates a new proposal from the original (as draft), preserving items and configuration.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `201` | Proposal duplicated. |
| `404` | Resource not found in the current organization. |

## proposals generate-pdf

Generate proposal PDF

```text
kobana billing proposals generate-pdf [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/generate-pdf`

Generates (or regenerates) the proposal PDF via Google App Scripts. Requires a `template` with `app_script_url` configured.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | PDF generated — returns the updated proposal and document/PDF URLs. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals json

Get proposal JSON snapshot

```text
kobana billing proposals json [ID] [OPTIONS]
```

`GET /v1/proposals/{id}/json`

Returns the full data snapshot used to render the PDF template (mirrors what AppScript receives).

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal JSON snapshot. |
| `404` | Resource not found in the current organization. |

## proposals pricing

Get proposal pricing breakdown

```text
kobana billing proposals pricing [ID] [OPTIONS]
```

`GET /v1/proposals/{id}/pricing`

Returns the pricing breakdown per billing cycle (monthly, quarterly, semiannual, annual), with setup and discounts applied.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pricing breakdown. |
| `404` | Resource not found in the current organization. |

## proposals publish

Publish proposal

```text
kobana billing proposals publish [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/publish`

Publishes the proposal (`draft → sent`) without sending the email, making it available on the portal.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal published. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals send

Send proposal email

```text
kobana billing proposals send [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/send`

Sends (or resends) the proposal email to the recipient. Allowed statuses: `draft`, `sent` or `viewed`. Optionally customize `subject` and `message` (HTML).

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `subject` | `string` | no | Custom email subject. Default: `Proposta {code}`. |
| `message` | `string` | no | Custom HTML message for the email body. |

More fields and nested objects: `kobana schema billing.proposals.send`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal sent. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## proposals undo-accept

Undo proposal acceptance

```text
kobana billing proposals undo-accept [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/undo-accept`

Reverts the acceptance: removes the linked subscription and returns status to `sent`.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Acceptance undone. |
| `404` | Resource not found in the current organization. |

## proposals unpublish

Unpublish proposal

```text
kobana billing proposals unpublish [ID] [OPTIONS]
```

`POST /v1/proposals/{id}/unpublish`

Reverts the proposal to draft (`sent`/`viewed` → `draft`).

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Proposal unpublished. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
