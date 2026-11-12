---
id: card-transactions
title: "kobana finance card-transactions"
sidebar_label: "card-transactions"
---

# kobana finance card-transactions


Comandos de `kobana finance card-transactions`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`card-transactions aggregates`](#card-transactions-aggregates) | `GET /v1/card-transactions/aggregates` | Totalizar os lançamentos do cartão |
| [`card-transactions list`](#card-transactions-list) | `GET /v1/card-transactions` | Listar os lançamentos do cartão |
| [`card-transactions create`](#card-transactions-create) | `POST /v1/card-transactions` | Criar um lançamento no cartão |
| [`card-transactions get`](#card-transactions-get) | `GET /v1/card-transactions/{id}` | Buscar um lançamento do cartão pelo id |
| [`card-transactions update`](#card-transactions-update) | `PATCH /v1/card-transactions/{id}` | Atualizar um lançamento do cartão |
| [`card-transactions delete`](#card-transactions-delete) | `DELETE /v1/card-transactions/{id}` | Excluir um lançamento do cartão |
| [`card-transactions anticipate`](#card-transactions-anticipate) | `POST /v1/card-transactions/{id}/anticipate` | Antecipar parcelas |
| [`card-transactions move`](#card-transactions-move) | `POST /v1/card-transactions/{id}/move` | Mover um lançamento para outra fatura |

## card-transactions aggregates

Totalizar os lançamentos do cartão

```text
kobana finance card-transactions aggregates [OPÇÕES]
```

`GET /v1/card-transactions/aggregates`

Sums the launches of the workspace's cards: `count`, `sum_total_amount` (IOF included), `sum_base_amount` and `sum_iof_amount`, in `totals` and, with `group_by`, per card, statement, category, supplier or month. Grouped by category, a launch split across categories (rateio) is apportioned by its slices, IOF included, the last slice taking the leftover cent so the groups add up to the totals. Without a `kind` filter, `carryover` launches (a statement balance rolled over, a prepaid top-up) are left out: they are not spending. Takes the listing's filters; pagination and sorting do not apply. Requires scope `finance/credit_cards` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `group_by` | `string` | não | Break the totals down by card, statement, category, supplier (`person`) or month of the purchase. Omit for the totals alone. Valores: `card`, `statement`, `category`, `person`, `month`. |
| `company_id` | `string` | não | Restrict to one company's cards. |
| `credit_card_id` | `string` | não | Filter by card. Accepts `[in]`. |
| `statement_id` | `string` | não | Filter by statement. Accepts `[in]`. |
| `recurrence_id` | `string` | não | The occurrences of one recurrence. Accepts `[in]`. |
| `kind` | `string` | não | Filter by kind. Accepts `[in]`. Valores: `purchase`, `installment`, `recurring`, `refund`, `fee`, `carryover`. |
| `status` | `string` | não | Filter by status. Accepts `[in]`. Omitted, it sums what a statement total sums: every status but `not_charged` and `cancelled`. For the IOF paid, ask for `actual,reconciled`. Valores: `expected`, `actual`, `reconciled`, `not_charged`, `cancelled`. |
| `purchase_date` | `string` | não | Purchase date (`YYYY-MM-DD`). Accepts `[gte]` / `[lte]`. |
| `person_id` | `string` | não | Filter by supplier. Accepts `[in]`. |
| `category_id` | `string` | não | Filter by category. A launch split across categories counts only its share in the chosen ones. Accepts `[in]`. |
| `is_international` | `string` | não | Valores: `true`, `false`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Totals, and the groups when `group_by` is given. |
| `400` | Malformed query (unknown parameter or operator, invalid date, uuid or `group_by`). |
| `404` | `company_id` is not a company of the tenant. |

## card-transactions list

Listar os lançamentos do cartão

```text
kobana finance card-transactions list [OPÇÕES]
```

`GET /v1/card-transactions`

Returns a cursor-paginated page of the launches of the workspace's cards — purchases, installments, refunds and fees. Requires scope `finance/credit_cards` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `credit_card_id` | `string` | não | Filter by card. Accepts `[in]`. |
| `statement_id` | `string` | não | Filter by statement. Accepts `[in]`. |
| `group_id` | `string` | não | The installments of one purchase. |
| `recurrence_id` | `string` | não | The occurrences of one recurrence. Accepts `[in]`. |
| `kind` | `string` | não | Filter by kind. Accepts `[in]`. Valores: `purchase`, `installment`, `recurring`, `refund`, `fee`, `carryover`. |
| `status` | `string` | não | Filter by status. Accepts `[in]`. Valores: `expected`, `actual`, `reconciled`, `not_charged`, `cancelled`. |
| `purchase_date` | `string` | não | Purchase date (`YYYY-MM-DD`). Accepts `[gte]` / `[lte]`. |
| `person_id` | `string` | não | Filter by supplier. Accepts `[in]`. |
| `category_id` | `string` | não | Filter by the scalar category. Accepts `[in]`. |
| `is_international` | `string` | não | Valores: `true`, `false`. |
| `sort_by` | `string` | não | Default: `purchase_date` ascending, installments in order. Valores: `purchase_date`, `total_amount`, `created_at`. |
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
| `200` | Page of card transactions. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## card-transactions create

Criar um lançamento no cartão

```text
kobana finance card-transactions create [OPÇÕES]
```

`POST /v1/card-transactions`

Launches a purchase, an installment purchase, a refund or a fee. The launch goes to the statement whose period holds `purchase_date` (the cutoff day is inclusive); when that statement already closed, it goes to the open one and the response carries a `STATEMENT_CLOSED` warning. An installment purchase creates one launch per statement sharing `group_id`, with the leftover cents on the first, and returns the first one. An international launch takes `fx_rate` or estimates it from PTAX, and its IOF uses the card's override or the rate in force on the purchase date. Every realized launch gets a read-only mirror in the card's financial account. Requires scope `finance/credit_cards` with access level `write`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `credit_card_id` | `string (uuid)` | sim |  |
| `kind` | `string` | sim | `purchase` (à vista), `installment`, `refund` (estorno) or `fee` (juros, multa, anuidade). Valores: `purchase`, `installment`, `refund`, `fee`. |
| `purchase_date` | `string` | sim |  |
| `description` | `string` | sim |  |
| `issuer_description` | `string` | não |  |
| `person_id` | `string (uuid)` | não |  |
| `category_id` | `string (uuid)` | não | Omitted on create = the card's default category. |
| `classification_center_id` | `string (uuid)` | não | Omitted on create = the card's default classification center. |
| `categories` | `array<object>` | não | Category rateio over \|total_amount\|. Replace-all; overrides `category_id`. |
| `classification_centers` | `array<object>` | não | Classification center rateio over \|total_amount\|. Replace-all. |
| `notes` | `string` | não |  |
| `ir_relevant` | `boolean` | não |  |
| `amount` | `number` | não | BRL without IOF, positive also for a refund. For `installment`, the purchase total. |
| `installments` | `integer` | não | Required for `installment`. Leftover cents go on the first installment. |
| `installment_amount` | `number` | não | Optional value of each installment; the first one absorbs the difference to the total. |
| `is_international` | `boolean` | não |  |
| `currency` | `string` | não |  |
| `foreign_amount` | `number` | não |  |
| `fx_rate` | `number` | não | Omitted = estimated from PTAX × (1 + the card's spread); 422 `fx_rate_required` when PTAX is unavailable. |
| `fx_rate_status` | `string` | não | Defaults to `actual` when `fx_rate` is sent. Valores: `estimated`, `actual`, `null`. |
| `iof_amount` | `number` | não | IOF typed from the real statement; prevails over the calculation. |
| `iof_refunded` | `boolean` | não | Refund of an international purchase: the issuer also returned the IOF. |
| `original_transaction_id` | `string (uuid)` | não | The launch a refund reverses; an international one lends its currency and rate. |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema finance.card-transactions.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Launch created (the first installment of a series). `warnings` lists placement notices. |
| `400` | Malformed JSON body. |
| `404` | The card or a referenced person, category, classification center or original launch is not in the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field or in `custom_metadata`, a date before the card's first statement, more than one year after today (`reason: purchase_date_too_far`) or impossible (`reason: invalid_date`), an `installment_amount` that does not fit the total (`reason: installments_do_not_fit`), an amount that rounds to zero (`reason: amount_not_positive`), or `reason: ptax_unavailable` when PTAX cannot estimate the rate (send `fx_rate`). |

## card-transactions get

Buscar um lançamento do cartão pelo id

```text
kobana finance card-transactions get [ID] [OPÇÕES]
```

`GET /v1/card-transactions/{id}`

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
| `200` | Get a card transaction by id |
| `404` | Card transaction not found within the tenant. |

## card-transactions update

Atualizar um lançamento do cartão

```text
kobana finance card-transactions update [ID] [OPÇÕES]
```

`PATCH /v1/card-transactions/{id}`

Partially updates a launch — or its series with `scope`. Description, supplier, classification and notes change in any statement. A launch whose statement already closed keeps its amount and date (`reason: statement_closed`). A new date re-places the launch, unless it was moved by hand. A recurring occurrence is confirmed (`status: actual`) or skipped (`status: not_charged`) here. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `scope` | `string` | não | Reach of the change inside a series: only this launch (default), this and the next ones, or all of them. On a recurring occurrence, a wider scope updates the recurrence and rebuilds its untouched forecasts; confirmed ones never change. Valores: `this`, `this_and_next`, `all`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `description` | `string` | não |  |
| `issuer_description` | `string` | não |  |
| `person_id` | `string (uuid)` | não |  |
| `category_id` | `string (uuid)` | não | Omitted on create = the card's default category. |
| `classification_center_id` | `string (uuid)` | não | Omitted on create = the card's default classification center. |
| `categories` | `array<object>` | não | Category rateio over \|total_amount\|. Replace-all; overrides `category_id`. |
| `classification_centers` | `array<object>` | não | Classification center rateio over \|total_amount\|. Replace-all. |
| `notes` | `string` | não |  |
| `ir_relevant` | `boolean` | não |  |
| `purchase_date` | `string` | não | Re-places the launch by the new date. An installment's date changes only with `scope=all`. |
| `amount` | `number` | não | BRL without IOF of a national launch. |
| `foreign_amount` | `number` | não |  |
| `fx_rate` | `number` | não |  |
| `fx_rate_status` | `string` | não | Valores: `estimated`, `actual`. |
| `iof_amount` | `number` | não | Typed IOF; `null` goes back to the calculated one. |
| `status` | `string` | não | Recurring occurrences only. `actual` confirms it against the real statement — with `amount`, or `foreign_amount` and `fx_rate`, when they differ from the forecast — and posts its mirror; `not_charged` takes it out of the statement total; `expected` takes a skipped one back to the forecast. Also accepted in a `closed` statement under review. Valores: `expected`, `actual`, `not_charged`. |
| `custom_metadata` | `object` | não |  |
| `custom_metadata_mode` | `string` | não | Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.card-transactions.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The launch as updated. |
| `400` | Malformed JSON body or `scope`. |
| `404` | Card transaction not found within the tenant, or a referenced id is not in it. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. A launch whose statement already closed keeps its amount and date (`reason: statement_closed`). A `status` on a launch that is not a recurring occurrence, or a transition it cannot make (a confirmed occurrence stays confirmed). A `purchase_date` more than one year after today (`reason: purchase_date_too_far`), a full card number in `custom_metadata`. |

## card-transactions delete

Excluir um lançamento do cartão

```text
kobana finance card-transactions delete [ID] [OPÇÕES]
```

`DELETE /v1/card-transactions/{id}`

Deletes a launch — or its series with `scope` — and its mirror in the card's financial account. A recurring occurrence is not deleted: mark it `not_charged` or end the recurrence. Deleting a prepaid top-up also deletes its transfer and needs the person's `card-statements.pay`, like creating it. A launch whose statement already closed keeps its amount and date (`reason: statement_closed`). Requires scope `finance/credit_cards` with access level `delete`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `scope` | `string` | não | Reach of the change inside a series: only this launch (default), this and the next ones, or all of them. On a recurring occurrence, a wider scope updates the recurrence and rebuilds its untouched forecasts; confirmed ones never change. Valores: `this`, `this_and_next`, `all`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Launches deleted. |
| `400` | Malformed `scope`. |
| `404` | Card transaction not found within the tenant. |
| `422` | A launch whose statement already closed keeps its amount and date (`reason: statement_closed`). Also for a recurring occurrence. |

## card-transactions anticipate

Antecipar parcelas

```text
kobana finance card-transactions anticipate [ID] [OPÇÕES]
```

`POST /v1/card-transactions/{id}/anticipate`

Moves the last installments still in future statements (all of them, or the last `count`) of the purchase this installment belongs to into the open statement. A `discount_amount` is launched there as a refund ("Desconto por antecipação"). Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `count` | `integer` | não | How many of the LAST installments still in future statements to anticipate. Omitted = all of them. |
| `discount_amount` | `number` | não | Discount granted by the issuer, launched as a refund in the open statement. |

Outros campos e objetos aninhados: `kobana schema finance.card-transactions.anticipate`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The installments moved, plus the discount when there is one. |
| `400` | Malformed JSON body. |
| `404` | Card transaction not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Not an installment, nothing left in future statements, no open statement, or a discount above the installments. |

## card-transactions move

Mover um lançamento para outra fatura

```text
kobana finance card-transactions move [ID] [OPÇÕES]
```

`POST /v1/card-transactions/{id}/move`

Moves one launch to another `open` or `future` statement of the same card. The launch stays there (`statement_locked`): a later cutoff change does not move it back. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `statement_id` | `string (uuid)` | sim | An `open` or `future` statement of the same card. |

Outros campos e objetos aninhados: `kobana schema finance.card-transactions.move`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The launch in its new statement. |
| `400` | Malformed JSON body. |
| `404` | Card transaction or statement not found within the card. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | The launch's statement or the target one already closed (`reason: statement_closed`). |
