---
id: card-recurrences
title: "kobana finance card-recurrences"
sidebar_label: "card-recurrences"
---

# kobana finance card-recurrences


Comandos de `kobana finance card-recurrences`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`card-recurrences list`](#card-recurrences-list) | `GET /v1/card-recurrences` | Listar as recorrências do cartão |
| [`card-recurrences create`](#card-recurrences-create) | `POST /v1/card-recurrences` | Criar uma recorrência no cartão |
| [`card-recurrences get`](#card-recurrences-get) | `GET /v1/card-recurrences/{id}` | Buscar uma recorrência do cartão pelo id |
| [`card-recurrences update`](#card-recurrences-update) | `PATCH /v1/card-recurrences/{id}` | Atualizar uma recorrência do cartão |
| [`card-recurrences delete`](#card-recurrences-delete) | `DELETE /v1/card-recurrences/{id}` | Excluir uma recorrência do cartão |

## card-recurrences list

Listar as recorrências do cartão

```text
kobana finance card-recurrences list [OPÇÕES]
```

`GET /v1/card-recurrences`

Returns a cursor-paginated page of the recurring charges of the workspace's cards (subscriptions), each with its next charge, the latest confirmed occurrence, its deviation from the forecast and, after 3 periods in a row more than 5% off, the amount to update it to (`suggested_amount`). Requires scope `finance/credit_cards` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `credit_card_id` | `string` | não | Filter by card. Accepts `[in]`. |
| `active` | `string` | não | Valores: `true`, `false`. |
| `is_international` | `string` | não | Valores: `true`, `false`. |
| `frequency` | `string` | não | Accepts `[in]`. Valores: `monthly`, `quarterly`, `yearly`. |
| `person_id` | `string` | não | Filter by supplier. Accepts `[in]`. |
| `category_id` | `string` | não | Accepts `[in]`. |
| `sort_by` | `string` | não | Default: `description` ascending. Valores: `description`, `charge_day`, `created_at`. |
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
| `200` | Page of card recurrences. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## card-recurrences create

Criar uma recorrência no cartão

```text
kobana finance card-recurrences create [OPÇÕES]
```

`POST /v1/card-recurrences`

Creates a recurring charge and its forecasts: one `recurring` launch with status `expected` in each open or future statement inside the validity, dated on the charge day. Forecasts have no mirror in the ledger and are confirmed through `PATCH /card-transactions/{id}` (`status: actual`). An international recurrence is kept in its currency; its forecasts use the estimated rate and the IOF in force. Requires scope `finance/credit_cards` with access level `write`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `credit_card_id` | `string (uuid)` | sim |  |
| `description` | `string` | sim |  |
| `expected_amount` | `number` | sim | In `currency`. |
| `person_id` | `string (uuid)` | não |  |
| `category_id` | `string (uuid)` | não | Omitted on create = the card's default category. |
| `classification_center_id` | `string (uuid)` | não | Omitted on create = the card's default classification center. |
| `currency` | `string` | não | Defaults to BRL. Another currency makes the recurrence international. |
| `is_international` | `boolean` | não | Derived from `currency` when omitted. |
| `charge_day` | `integer` | não | Defaults to the day of `start_date`. |
| `frequency` | `string` | não | Defaults to `monthly`. Valores: `monthly`, `quarterly`, `yearly`. |
| `start_date` | `string` | não | Defaults to today. |
| `end_date` | `string` | não | Ends the recurrence: forecasts after it are cancelled. `null` reopens it. |
| `issuer_match_pattern` | `string` | não |  |
| `active` | `boolean` | não |  |
| `ir_relevant` | `boolean` | não |  |
| `notes` | `string` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema finance.card-recurrences.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Recurrence created. `warnings` says when international forecasts wait for a PTAX rate. |
| `400` | Malformed JSON body. |
| `404` | The card or a referenced person, category or classification center is not in the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an end date before the start date. |

## card-recurrences get

Buscar uma recorrência do cartão pelo id

```text
kobana finance card-recurrences get [ID] [OPÇÕES]
```

`GET /v1/card-recurrences/{id}`

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
| `200` | Get a card recurrence by id |
| `404` | Card recurrence not found within the tenant. |

## card-recurrences update

Atualizar uma recorrência do cartão

```text
kobana finance card-recurrences update [ID] [OPÇÕES]
```

`PATCH /v1/card-recurrences/{id}`

Partially updates a recurrence. Its forecasts nobody touched are rebuilt from it; confirmed, skipped and individually edited occurrences stay as they are. An `end_date` ends the recurrence and cancels the forecasts after it; `active: false` cancels the open and future ones. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `description` | `string` | não |  |
| `expected_amount` | `number` | não |  |
| `person_id` | `string (uuid)` | não |  |
| `category_id` | `string (uuid)` | não | Omitted on create = the card's default category. |
| `classification_center_id` | `string (uuid)` | não | Omitted on create = the card's default classification center. |
| `currency` | `string` | não | Defaults to BRL. Another currency makes the recurrence international. |
| `is_international` | `boolean` | não | Derived from `currency` when omitted. |
| `charge_day` | `integer` | não | Defaults to the day of `start_date`. |
| `frequency` | `string` | não | Defaults to `monthly`. Valores: `monthly`, `quarterly`, `yearly`. |
| `start_date` | `string` | não | Defaults to today. |
| `end_date` | `string` | não | Ends the recurrence: forecasts after it are cancelled. `null` reopens it. |
| `issuer_match_pattern` | `string` | não |  |
| `active` | `boolean` | não |  |
| `ir_relevant` | `boolean` | não |  |
| `notes` | `string` | não |  |
| `custom_metadata` | `object` | não |  |
| `custom_metadata_mode` | `string` | não | Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.card-recurrences.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The recurrence as updated. |
| `400` | Malformed JSON body. |
| `404` | Card recurrence not found within the tenant, or a referenced id is not in it. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an end date before the start date. |

## card-recurrences delete

Excluir uma recorrência do cartão

```text
kobana finance card-recurrences delete [ID] [OPÇÕES]
```

`DELETE /v1/card-recurrences/{id}`

Deletes a recurrence and its forecasts while none was confirmed, skipped or billed in a closed statement; after that it is ended with `end_date` instead (`reason: recurrence_has_history`). Requires scope `finance/credit_cards` with access level `delete`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Recurrence deleted. |
| `404` | Card recurrence not found within the tenant. |
| `422` | An occurrence was already confirmed, skipped or billed in a closed statement (`reason: recurrence_has_history`). |
