---
id: credit-cards
title: "kobana finance credit-cards"
sidebar_label: "credit-cards"
---

# kobana finance credit-cards


Commands of `kobana finance credit-cards`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`credit-cards list`](#credit-cards-list) | `GET /v1/credit-cards` | List credit cards |
| [`credit-cards create`](#credit-cards-create) | `POST /v1/credit-cards` | Create a credit card |
| [`credit-cards get`](#credit-cards-get) | `GET /v1/credit-cards/{id}` | Get a credit card by id |
| [`credit-cards update`](#credit-cards-update) | `PATCH /v1/credit-cards/{id}` | Update a credit card |
| [`credit-cards statements`](#credit-cards-statements) | `GET /v1/credit-cards/{id}/statements` | List a card's statements |
| [`credit-cards top-ups`](#credit-cards-top-ups) | `POST /v1/credit-cards/{id}/top-ups` | Top up a prepaid card |

## credit-cards list

List credit cards

```text
kobana finance credit-cards list [OPTIONS]
```

`GET /v1/credit-cards`

Returns a cursor-paginated page of the workspace's credit, prepaid and benefit cards, each with its limit used and the id of the statement in progress. Requires scope `finance/credit_cards` with access level `read`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string` | no | Filter by company. Accepts `[in]`. |
| `status` | `string` | no | Filter by status. Accepts `[in]`. Values: `active`, `blocked`, `cancelled`. |
| `kind` | `string` | no | Filter by kind. Accepts `[in]`. Values: `credit`, `prepaid`, `benefit`. |
| `sort_by` | `string` | no | Default: `nickname` ascending. Values: `nickname`, `created_at`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`. Values: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of credit cards. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## credit-cards create

Create a credit card

```text
kobana finance credit-cards create [OPTIONS]
```

`POST /v1/credit-cards`

Creates a card and its statement cycle: the statement in progress plus 12 future ones. Pass `financial_account_id` to turn an existing `credit` account into the card; omit it and a new account is created. Only the last 4 digits are stored — any free-text field carrying a full card number is refused. Requires scope `finance/credit_cards` with access level `write`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string (uuid)` | yes |  |
| `start_date` | `string` | yes |  |
| `nickname` | `string` | yes |  |
| `issuer_bank_id` | `string (uuid)` | yes |  |
| `brand` | `string` | yes | Values: `visa`, `mastercard`, `elo`, `amex`, `hipercard`, `other`. |
| `last4` | `string` | yes |  |
| `closing_day` | `integer` | yes |  |
| `due_day` | `integer` | yes |  |
| `payer_account_id` | `string (uuid)` | yes |  |
| `financial_account_id` | `string (uuid)` | no | An existing `credit` account to turn into this card. Omitted = a new account is created. |
| `kind` | `string` | no | Values: `credit`, `prepaid`, `benefit`. |
| `issuer_person_id` | `string (uuid)` | no | Defaults to a supplier named after the issuer bank, created when missing. |
| `product_name` | `string` | no |  |
| `cutoff_offset_days` | `integer` | no |  |
| `due_non_business_rule` | `string` | no | Values: `next`, `previous`, `keep`. |
| `credit_limit` | `number` | no |  |
| `limit_alert_pct` | `integer` | no |  |
| `payment_method_kind` | `string` | no | Values: `direct_debit`, `bank_billet`, `pix`, `null`. |
| `default_category_id` | `string (uuid)` | no |  |
| `default_classification_center_id` | `string (uuid)` | no |  |
| `iof_rate_override` | `number` | no |  |
| `fx_spread_pct` | `number` | no |  |
| `status` | `string` | no | Values: `active`, `blocked`, `cancelled`. |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema finance.credit-cards.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Card created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `404` | A referenced bank, account, person, category or classification center is not in the tenant. |
| `409` | The account already belongs to a card, or `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an account that cannot be used. |

## credit-cards get

Get a credit card by id

```text
kobana finance credit-cards get [ID] [OPTIONS]
```

`GET /v1/credit-cards/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/credit_cards` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Get a credit card by id |
| `404` | Credit card not found within the tenant. |

## credit-cards update

Update a credit card

```text
kobana finance credit-cards update [ID] [OPTIONS]
```

`PATCH /v1/credit-cards/{id}`

Partially updates a card. Changing the closing or due day re-predicts only the future statements whose cutoff was not confirmed, and the launches of the future statements follow their new periods. Requires scope `finance/credit_cards` with access level `write`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `nickname` | `string` | no |  |
| `issuer_bank_id` | `string (uuid)` | no |  |
| `issuer_person_id` | `string (uuid)` | no | Defaults to a supplier named after the issuer bank, created when missing. |
| `brand` | `string` | no | Values: `visa`, `mastercard`, `elo`, `amex`, `hipercard`, `other`. |
| `product_name` | `string` | no |  |
| `last4` | `string` | no |  |
| `closing_day` | `integer` | no |  |
| `cutoff_offset_days` | `integer` | no |  |
| `due_day` | `integer` | no |  |
| `due_non_business_rule` | `string` | no | Values: `next`, `previous`, `keep`. |
| `credit_limit` | `number` | no |  |
| `limit_alert_pct` | `integer` | no |  |
| `payer_account_id` | `string (uuid)` | no |  |
| `payment_method_kind` | `string` | no | Values: `direct_debit`, `bank_billet`, `pix`, `null`. |
| `default_category_id` | `string (uuid)` | no |  |
| `default_classification_center_id` | `string (uuid)` | no |  |
| `iof_rate_override` | `number` | no |  |
| `fx_spread_pct` | `number` | no |  |
| `status` | `string` | no | Values: `active`, `blocked`, `cancelled`. |
| `custom_metadata` | `object` | no |  |
| `custom_metadata_mode` | `string` | no | Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema finance.credit-cards.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Updated card. |
| `400` | Malformed JSON body. |
| `404` | Credit card not found within the tenant, or a referenced id is not in it. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## credit-cards statements

List a card's statements

```text
kobana finance credit-cards statements [ID] [OPTIONS]
```

`GET /v1/credit-cards/{id}/statements`

Returns the card's statements, oldest reference month first: the closed ones, the open one and the 12 future ones. Requires scope `finance/credit_cards` with access level `read`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Filter by status. Accepts `[in]`. Values: `future`, `open`, `closed`, `reviewed`, `paid`, `partially_paid`. |
| `reference_month` | `string` | no | First day of the due month (`YYYY-MM-DD`). Accepts `[gte]` / `[lte]`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of statements. |
| `400` | Malformed query. |
| `404` | Credit card not found within the tenant. |

## credit-cards top-ups

Top up a prepaid card

```text
kobana finance credit-cards top-ups [ID] [OPTIONS]
```

`POST /v1/credit-cards/{id}/top-ups`

Loads a `prepaid` or `benefit` card: a transfer from a bank account to the card's account, and a negative `carryover` launch in the open statement linked to it. The card's available limit is its account's balance. Deleting the launch (`DELETE /card-transactions/{id}`) deletes the transfer too; it cannot be edited. Requires scope `finance/credit_cards` with access level `write`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | yes | Amount loaded onto the prepaid or benefit card. |
| `date` | `string` | no | Omitted = today. |
| `from_account_id` | `string (uuid)` | no | Account the money leaves from. Omitted = the card's paying account. |
| `description` | `string` | no | Omitted = "Recarga". |

More fields and nested objects: `kobana schema finance.credit-cards.top-ups`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | The top-up launch. |
| `400` | Malformed JSON body, or `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Credit card not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; the card is a credit card (`reason: card_not_prepaid`) or cancelled (`reason: card_cancelled`); the amount is zero (`reason: amount_not_positive`); no open statement (`reason: no_open_statement`); the source account is a card's (`reason: funding_account_is_card`) or of another company (`reason: funding_account_other_company`); an impossible `date` (`reason: invalid_date`). |
