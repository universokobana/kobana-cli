---
id: credit-cards
title: "kobana finance credit-cards"
sidebar_label: "credit-cards"
---

# kobana finance credit-cards


Comandos de `kobana finance credit-cards`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`credit-cards list`](#credit-cards-list) | `GET /v1/credit-cards` | Listar tarjetas de crédito |
| [`credit-cards create`](#credit-cards-create) | `POST /v1/credit-cards` | Crear una tarjeta de crédito |
| [`credit-cards get`](#credit-cards-get) | `GET /v1/credit-cards/{id}` | Obtener una tarjeta de crédito por id |
| [`credit-cards update`](#credit-cards-update) | `PATCH /v1/credit-cards/{id}` | Actualizar una tarjeta de crédito |
| [`credit-cards statements`](#credit-cards-statements) | `GET /v1/credit-cards/{id}/statements` | Listar los resúmenes de una tarjeta |
| [`credit-cards top-ups`](#credit-cards-top-ups) | `POST /v1/credit-cards/{id}/top-ups` | Recargar una tarjeta prepaga |

## credit-cards list

Listar tarjetas de crédito

```text
kobana finance credit-cards list [OPCIONES]
```

`GET /v1/credit-cards`

Returns a cursor-paginated page of the workspace's credit, prepaid and benefit cards, each with its limit used and the id of the statement in progress. Requires scope `finance/credit_cards` with access level `read`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string` | no | Filter by company. Accepts `[in]`. |
| `status` | `string` | no | Filter by status. Accepts `[in]`. Valores: `active`, `blocked`, `cancelled`. |
| `kind` | `string` | no | Filter by kind. Accepts `[in]`. Valores: `credit`, `prepaid`, `benefit`. |
| `sort_by` | `string` | no | Default: `nickname` ascending. Valores: `nickname`, `created_at`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`. Valores: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Page of credit cards. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## credit-cards create

Crear una tarjeta de crédito

```text
kobana finance credit-cards create [OPCIONES]
```

`POST /v1/credit-cards`

Creates a card and its statement cycle: the statement in progress plus 12 future ones. Pass `financial_account_id` to turn an existing `credit` account into the card; omit it and a new account is created. Only the last 4 digits are stored — any free-text field carrying a full card number is refused. Requires scope `finance/credit_cards` with access level `write`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string (uuid)` | sí |  |
| `start_date` | `string` | sí |  |
| `nickname` | `string` | sí |  |
| `issuer_bank_id` | `string (uuid)` | sí |  |
| `brand` | `string` | sí | Valores: `visa`, `mastercard`, `elo`, `amex`, `hipercard`, `other`. |
| `last4` | `string` | sí |  |
| `closing_day` | `integer` | sí |  |
| `due_day` | `integer` | sí |  |
| `payer_account_id` | `string (uuid)` | sí |  |
| `financial_account_id` | `string (uuid)` | no | An existing `credit` account to turn into this card. Omitted = a new account is created. |
| `kind` | `string` | no | Valores: `credit`, `prepaid`, `benefit`. |
| `issuer_person_id` | `string (uuid)` | no | Defaults to a supplier named after the issuer bank, created when missing. |
| `product_name` | `string` | no |  |
| `cutoff_offset_days` | `integer` | no |  |
| `due_non_business_rule` | `string` | no | Valores: `next`, `previous`, `keep`. |
| `credit_limit` | `number` | no |  |
| `limit_alert_pct` | `integer` | no |  |
| `payment_method_kind` | `string` | no | Valores: `direct_debit`, `bank_billet`, `pix`, `null`. |
| `default_category_id` | `string (uuid)` | no |  |
| `default_classification_center_id` | `string (uuid)` | no |  |
| `iof_rate_override` | `number` | no |  |
| `fx_spread_pct` | `number` | no |  |
| `status` | `string` | no | Valores: `active`, `blocked`, `cancelled`. |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema finance.credit-cards.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Card created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `404` | A referenced bank, account, person, category or classification center is not in the tenant. |
| `409` | The account already belongs to a card, or `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an account that cannot be used. |

## credit-cards get

Obtener una tarjeta de crédito por id

```text
kobana finance credit-cards get [ID] [OPCIONES]
```

`GET /v1/credit-cards/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/credit_cards` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Get a credit card by id |
| `404` | Credit card not found within the tenant. |

## credit-cards update

Actualizar una tarjeta de crédito

```text
kobana finance credit-cards update [ID] [OPCIONES]
```

`PATCH /v1/credit-cards/{id}`

Partially updates a card. Changing the closing or due day re-predicts only the future statements whose cutoff was not confirmed, and the launches of the future statements follow their new periods. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `nickname` | `string` | no |  |
| `issuer_bank_id` | `string (uuid)` | no |  |
| `issuer_person_id` | `string (uuid)` | no | Defaults to a supplier named after the issuer bank, created when missing. |
| `brand` | `string` | no | Valores: `visa`, `mastercard`, `elo`, `amex`, `hipercard`, `other`. |
| `product_name` | `string` | no |  |
| `last4` | `string` | no |  |
| `closing_day` | `integer` | no |  |
| `cutoff_offset_days` | `integer` | no |  |
| `due_day` | `integer` | no |  |
| `due_non_business_rule` | `string` | no | Valores: `next`, `previous`, `keep`. |
| `credit_limit` | `number` | no |  |
| `limit_alert_pct` | `integer` | no |  |
| `payer_account_id` | `string (uuid)` | no |  |
| `payment_method_kind` | `string` | no | Valores: `direct_debit`, `bank_billet`, `pix`, `null`. |
| `default_category_id` | `string (uuid)` | no |  |
| `default_classification_center_id` | `string (uuid)` | no |  |
| `iof_rate_override` | `number` | no |  |
| `fx_spread_pct` | `number` | no |  |
| `status` | `string` | no | Valores: `active`, `blocked`, `cancelled`. |
| `custom_metadata` | `object` | no |  |
| `custom_metadata_mode` | `string` | no | Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema finance.credit-cards.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Updated card. |
| `400` | Malformed JSON body. |
| `404` | Credit card not found within the tenant, or a referenced id is not in it. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## credit-cards statements

Listar los resúmenes de una tarjeta

```text
kobana finance credit-cards statements [ID] [OPCIONES]
```

`GET /v1/credit-cards/{id}/statements`

Returns the card's statements, oldest reference month first: the closed ones, the open one and the 12 future ones. Requires scope `finance/credit_cards` with access level `read`.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Filter by status. Accepts `[in]`. Valores: `future`, `open`, `closed`, `reviewed`, `paid`, `partially_paid`. |
| `reference_month` | `string` | no | First day of the due month (`YYYY-MM-DD`). Accepts `[gte]` / `[lte]`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Page of statements. |
| `400` | Malformed query. |
| `404` | Credit card not found within the tenant. |

## credit-cards top-ups

Recargar una tarjeta prepaga

```text
kobana finance credit-cards top-ups [ID] [OPCIONES]
```

`POST /v1/credit-cards/{id}/top-ups`

Loads a `prepaid` or `benefit` card: a transfer from a bank account to the card's account, and a negative `carryover` launch in the open statement linked to it. The card's available limit is its account's balance. Deleting the launch (`DELETE /card-transactions/{id}`) deletes the transfer too; it cannot be edited. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Amount loaded onto the prepaid or benefit card. |
| `date` | `string` | no | Omitted = today. |
| `from_account_id` | `string (uuid)` | no | Account the money leaves from. Omitted = the card's paying account. |
| `description` | `string` | no | Omitted = "Recarga". |

Otros campos y objetos anidados: `kobana schema finance.credit-cards.top-ups`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | The top-up launch. |
| `400` | Malformed JSON body, or `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Credit card not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; the card is a credit card (`reason: card_not_prepaid`) or cancelled (`reason: card_cancelled`); the amount is zero (`reason: amount_not_positive`); no open statement (`reason: no_open_statement`); the source account is a card's (`reason: funding_account_is_card`) or of another company (`reason: funding_account_other_company`); an impossible `date` (`reason: invalid_date`). |
