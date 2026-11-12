---
id: credit-cards
title: "kobana finance credit-cards"
sidebar_label: "credit-cards"
---

# kobana finance credit-cards


Comandos de `kobana finance credit-cards`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`credit-cards list`](#credit-cards-list) | `GET /v1/credit-cards` | Listar cartões de crédito |
| [`credit-cards create`](#credit-cards-create) | `POST /v1/credit-cards` | Criar um cartão de crédito |
| [`credit-cards get`](#credit-cards-get) | `GET /v1/credit-cards/{id}` | Buscar um cartão de crédito pelo id |
| [`credit-cards update`](#credit-cards-update) | `PATCH /v1/credit-cards/{id}` | Atualizar um cartão de crédito |
| [`credit-cards statements`](#credit-cards-statements) | `GET /v1/credit-cards/{id}/statements` | Listar as faturas de um cartão |
| [`credit-cards top-ups`](#credit-cards-top-ups) | `POST /v1/credit-cards/{id}/top-ups` | Recarregar um cartão pré-pago |

## credit-cards list

Listar cartões de crédito

```text
kobana finance credit-cards list [OPÇÕES]
```

`GET /v1/credit-cards`

Returns a cursor-paginated page of the workspace's credit, prepaid and benefit cards, each with its limit used and the id of the statement in progress. Requires scope `finance/credit_cards` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by company. Accepts `[in]`. |
| `status` | `string` | não | Filter by status. Accepts `[in]`. Valores: `active`, `blocked`, `cancelled`. |
| `kind` | `string` | não | Filter by kind. Accepts `[in]`. Valores: `credit`, `prepaid`, `benefit`. |
| `sort_by` | `string` | não | Default: `nickname` ascending. Valores: `nickname`, `created_at`. |
| `sort_order` | `string` | não | Order direction. Defaults to `asc`. Valores: `asc`, `desc`. |
| `limit` | `integer` | não | Page size. Default 25, max 100. |
| `cursor` | `string` | não | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Page of credit cards. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## credit-cards create

Criar um cartão de crédito

```text
kobana finance credit-cards create [OPÇÕES]
```

`POST /v1/credit-cards`

Creates a card and its statement cycle: the statement in progress plus 12 future ones. Pass `financial_account_id` to turn an existing `credit` account into the card; omit it and a new account is created. Only the last 4 digits are stored — any free-text field carrying a full card number is refused. Requires scope `finance/credit_cards` with access level `write`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim |  |
| `start_date` | `string` | sim |  |
| `nickname` | `string` | sim |  |
| `issuer_bank_id` | `string (uuid)` | sim |  |
| `brand` | `string` | sim | Valores: `visa`, `mastercard`, `elo`, `amex`, `hipercard`, `other`. |
| `last4` | `string` | sim |  |
| `closing_day` | `integer` | sim |  |
| `due_day` | `integer` | sim |  |
| `payer_account_id` | `string (uuid)` | sim |  |
| `financial_account_id` | `string (uuid)` | não | An existing `credit` account to turn into this card. Omitted = a new account is created. |
| `kind` | `string` | não | Valores: `credit`, `prepaid`, `benefit`. |
| `issuer_person_id` | `string (uuid)` | não | Defaults to a supplier named after the issuer bank, created when missing. |
| `product_name` | `string` | não |  |
| `cutoff_offset_days` | `integer` | não |  |
| `due_non_business_rule` | `string` | não | Valores: `next`, `previous`, `keep`. |
| `credit_limit` | `number` | não |  |
| `limit_alert_pct` | `integer` | não |  |
| `payment_method_kind` | `string` | não | Valores: `direct_debit`, `bank_billet`, `pix`, `null`. |
| `default_category_id` | `string (uuid)` | não |  |
| `default_classification_center_id` | `string (uuid)` | não |  |
| `iof_rate_override` | `number` | não |  |
| `fx_spread_pct` | `number` | não |  |
| `status` | `string` | não | Valores: `active`, `blocked`, `cancelled`. |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema finance.credit-cards.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Card created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `404` | A referenced bank, account, person, category or classification center is not in the tenant. |
| `409` | The account already belongs to a card, or `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an account that cannot be used. |

## credit-cards get

Buscar um cartão de crédito pelo id

```text
kobana finance credit-cards get [ID] [OPÇÕES]
```

`GET /v1/credit-cards/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/credit_cards` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Get a credit card by id |
| `404` | Credit card not found within the tenant. |

## credit-cards update

Atualizar um cartão de crédito

```text
kobana finance credit-cards update [ID] [OPÇÕES]
```

`PATCH /v1/credit-cards/{id}`

Partially updates a card. Changing the closing or due day re-predicts only the future statements whose cutoff was not confirmed, and the launches of the future statements follow their new periods. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `nickname` | `string` | não |  |
| `issuer_bank_id` | `string (uuid)` | não |  |
| `issuer_person_id` | `string (uuid)` | não | Defaults to a supplier named after the issuer bank, created when missing. |
| `brand` | `string` | não | Valores: `visa`, `mastercard`, `elo`, `amex`, `hipercard`, `other`. |
| `product_name` | `string` | não |  |
| `last4` | `string` | não |  |
| `closing_day` | `integer` | não |  |
| `cutoff_offset_days` | `integer` | não |  |
| `due_day` | `integer` | não |  |
| `due_non_business_rule` | `string` | não | Valores: `next`, `previous`, `keep`. |
| `credit_limit` | `number` | não |  |
| `limit_alert_pct` | `integer` | não |  |
| `payer_account_id` | `string (uuid)` | não |  |
| `payment_method_kind` | `string` | não | Valores: `direct_debit`, `bank_billet`, `pix`, `null`. |
| `default_category_id` | `string (uuid)` | não |  |
| `default_classification_center_id` | `string (uuid)` | não |  |
| `iof_rate_override` | `number` | não |  |
| `fx_spread_pct` | `number` | não |  |
| `status` | `string` | não | Valores: `active`, `blocked`, `cancelled`. |
| `custom_metadata` | `object` | não |  |
| `custom_metadata_mode` | `string` | não | Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.credit-cards.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated card. |
| `400` | Malformed JSON body. |
| `404` | Credit card not found within the tenant, or a referenced id is not in it. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## credit-cards statements

Listar as faturas de um cartão

```text
kobana finance credit-cards statements [ID] [OPÇÕES]
```

`GET /v1/credit-cards/{id}/statements`

Returns the card's statements, oldest reference month first: the closed ones, the open one and the 12 future ones. Requires scope `finance/credit_cards` with access level `read`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Filter by status. Accepts `[in]`. Valores: `future`, `open`, `closed`, `reviewed`, `paid`, `partially_paid`. |
| `reference_month` | `string` | não | First day of the due month (`YYYY-MM-DD`). Accepts `[gte]` / `[lte]`. |
| `limit` | `integer` | não | Page size. Default 25, max 100. |
| `cursor` | `string` | não | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Page of statements. |
| `400` | Malformed query. |
| `404` | Credit card not found within the tenant. |

## credit-cards top-ups

Recarregar um cartão pré-pago

```text
kobana finance credit-cards top-ups [ID] [OPÇÕES]
```

`POST /v1/credit-cards/{id}/top-ups`

Loads a `prepaid` or `benefit` card: a transfer from a bank account to the card's account, and a negative `carryover` launch in the open statement linked to it. The card's available limit is its account's balance. Deleting the launch (`DELETE /card-transactions/{id}`) deletes the transfer too; it cannot be edited. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Amount loaded onto the prepaid or benefit card. |
| `date` | `string` | não | Omitted = today. |
| `from_account_id` | `string (uuid)` | não | Account the money leaves from. Omitted = the card's paying account. |
| `description` | `string` | não | Omitted = "Recarga". |

Outros campos e objetos aninhados: `kobana schema finance.credit-cards.top-ups`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | The top-up launch. |
| `400` | Malformed JSON body, or `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Credit card not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; the card is a credit card (`reason: card_not_prepaid`) or cancelled (`reason: card_cancelled`); the amount is zero (`reason: amount_not_positive`); no open statement (`reason: no_open_statement`); the source account is a card's (`reason: funding_account_is_card`) or of another company (`reason: funding_account_other_company`); an impossible `date` (`reason: invalid_date`). |
