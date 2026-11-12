---
id: card-recurrences
title: "kobana finance card-recurrences"
sidebar_label: "card-recurrences"
---

# kobana finance card-recurrences


Commands of `kobana finance card-recurrences`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`card-recurrences list`](#card-recurrences-list) | `GET /v1/card-recurrences` | List card recurrences |
| [`card-recurrences create`](#card-recurrences-create) | `POST /v1/card-recurrences` | Create a card recurrence |
| [`card-recurrences get`](#card-recurrences-get) | `GET /v1/card-recurrences/{id}` | Get a card recurrence by id |
| [`card-recurrences update`](#card-recurrences-update) | `PATCH /v1/card-recurrences/{id}` | Update a card recurrence |
| [`card-recurrences delete`](#card-recurrences-delete) | `DELETE /v1/card-recurrences/{id}` | Delete a card recurrence |

## card-recurrences list

List card recurrences

```text
kobana finance card-recurrences list [OPTIONS]
```

`GET /v1/card-recurrences`

Returns a cursor-paginated page of the recurring charges of the workspace's cards (subscriptions), each with its next charge, the latest confirmed occurrence, its deviation from the forecast and, after 3 periods in a row more than 5% off, the amount to update it to (`suggested_amount`). Requires scope `finance/credit_cards` with access level `read`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `credit_card_id` | `string` | no | Filter by card. Accepts `[in]`. |
| `active` | `string` | no | Values: `true`, `false`. |
| `is_international` | `string` | no | Values: `true`, `false`. |
| `frequency` | `string` | no | Accepts `[in]`. Values: `monthly`, `quarterly`, `yearly`. |
| `person_id` | `string` | no | Filter by supplier. Accepts `[in]`. |
| `category_id` | `string` | no | Accepts `[in]`. |
| `sort_by` | `string` | no | Default: `description` ascending. Values: `description`, `charge_day`, `created_at`. |
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
| `200` | Page of card recurrences. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## card-recurrences create

Create a card recurrence

```text
kobana finance card-recurrences create [OPTIONS]
```

`POST /v1/card-recurrences`

Creates a recurring charge and its forecasts: one `recurring` launch with status `expected` in each open or future statement inside the validity, dated on the charge day. Forecasts have no mirror in the ledger and are confirmed through `PATCH /card-transactions/{id}` (`status: actual`). An international recurrence is kept in its currency; its forecasts use the estimated rate and the IOF in force. Requires scope `finance/credit_cards` with access level `write`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `credit_card_id` | `string (uuid)` | yes |  |
| `description` | `string` | yes |  |
| `expected_amount` | `number` | yes | In `currency`. |
| `person_id` | `string (uuid)` | no |  |
| `category_id` | `string (uuid)` | no | Omitted on create = the card's default category. |
| `classification_center_id` | `string (uuid)` | no | Omitted on create = the card's default classification center. |
| `currency` | `string` | no | Defaults to BRL. Another currency makes the recurrence international. |
| `is_international` | `boolean` | no | Derived from `currency` when omitted. |
| `charge_day` | `integer` | no | Defaults to the day of `start_date`. |
| `frequency` | `string` | no | Defaults to `monthly`. Values: `monthly`, `quarterly`, `yearly`. |
| `start_date` | `string` | no | Defaults to today. |
| `end_date` | `string` | no | Ends the recurrence: forecasts after it are cancelled. `null` reopens it. |
| `issuer_match_pattern` | `string` | no |  |
| `active` | `boolean` | no |  |
| `ir_relevant` | `boolean` | no |  |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema finance.card-recurrences.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Recurrence created. `warnings` says when international forecasts wait for a PTAX rate. |
| `400` | Malformed JSON body. |
| `404` | The card or a referenced person, category or classification center is not in the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an end date before the start date. |

## card-recurrences get

Get a card recurrence by id

```text
kobana finance card-recurrences get [ID] [OPTIONS]
```

`GET /v1/card-recurrences/{id}`

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
| `200` | Get a card recurrence by id |
| `404` | Card recurrence not found within the tenant. |

## card-recurrences update

Update a card recurrence

```text
kobana finance card-recurrences update [ID] [OPTIONS]
```

`PATCH /v1/card-recurrences/{id}`

Partially updates a recurrence. Its forecasts nobody touched are rebuilt from it; confirmed, skipped and individually edited occurrences stay as they are. An `end_date` ends the recurrence and cancels the forecasts after it; `active: false` cancels the open and future ones. Requires scope `finance/credit_cards` with access level `write`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `description` | `string` | no |  |
| `expected_amount` | `number` | no |  |
| `person_id` | `string (uuid)` | no |  |
| `category_id` | `string (uuid)` | no | Omitted on create = the card's default category. |
| `classification_center_id` | `string (uuid)` | no | Omitted on create = the card's default classification center. |
| `currency` | `string` | no | Defaults to BRL. Another currency makes the recurrence international. |
| `is_international` | `boolean` | no | Derived from `currency` when omitted. |
| `charge_day` | `integer` | no | Defaults to the day of `start_date`. |
| `frequency` | `string` | no | Defaults to `monthly`. Values: `monthly`, `quarterly`, `yearly`. |
| `start_date` | `string` | no | Defaults to today. |
| `end_date` | `string` | no | Ends the recurrence: forecasts after it are cancelled. `null` reopens it. |
| `issuer_match_pattern` | `string` | no |  |
| `active` | `boolean` | no |  |
| `ir_relevant` | `boolean` | no |  |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |
| `custom_metadata_mode` | `string` | no | Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema finance.card-recurrences.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The recurrence as updated. |
| `400` | Malformed JSON body. |
| `404` | Card recurrence not found within the tenant, or a referenced id is not in it. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an end date before the start date. |

## card-recurrences delete

Delete a card recurrence

```text
kobana finance card-recurrences delete [ID] [OPTIONS]
```

`DELETE /v1/card-recurrences/{id}`

Deletes a recurrence and its forecasts while none was confirmed, skipped or billed in a closed statement; after that it is ended with `end_date` instead (`reason: recurrence_has_history`). Requires scope `finance/credit_cards` with access level `delete`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Recurrence deleted. |
| `404` | Card recurrence not found within the tenant. |
| `422` | An occurrence was already confirmed, skipped or billed in a closed statement (`reason: recurrence_has_history`). |
