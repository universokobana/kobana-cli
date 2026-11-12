---
id: card-statements
title: "kobana finance card-statements"
sidebar_label: "card-statements"
---

# kobana finance card-statements


Comandos de `kobana finance card-statements`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`card-statements payments create`](#card-statements-payments-create) | `POST /v1/card-statements/{id}/payments` | Pagar uma fatura |
| [`card-statements payments delete`](#card-statements-payments-delete) | `DELETE /v1/card-statements/{id}/payments` | Desfazer o pagamento de uma fatura |
| [`card-statements get`](#card-statements-get) | `GET /v1/card-statements/{id}` | Buscar uma fatura do cartão pelo id |
| [`card-statements close`](#card-statements-close) | `POST /v1/card-statements/{id}/close` | Fechar uma fatura |
| [`card-statements cutoff`](#card-statements-cutoff) | `PATCH /v1/card-statements/{id}/cutoff` | Alterar o corte de uma fatura |
| [`card-statements events`](#card-statements-events) | `GET /v1/card-statements/{id}/events` | Listar o histórico de uma fatura |
| [`card-statements reopen`](#card-statements-reopen) | `POST /v1/card-statements/{id}/reopen` | Reabrir uma fatura |
| [`card-statements review`](#card-statements-review) | `POST /v1/card-statements/{id}/review` | Conferir uma fatura com o total do emissor |

## card-statements payments create

Pagar uma fatura

```text
kobana finance card-statements payments create [ID] [OPÇÕES]
```

`POST /v1/card-statements/{id}/payments`

Settles the payable of a `closed` or `reviewed` statement. The payment is a transfer from the paying account to the card's account, never an expense: the purchases already count through the card's launches. Paying the whole payable makes the statement `paid`; less than it makes it `partially_paid`, lowers the payable to the amount paid and carries the balance to the open statement as a `carryover` launch. Either way the launches become `reconciled`. Settling the payable through `PATCH /payables/{id}` or the bank's confirmation gives the same result. Records a `paid` event. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | não | Amount paid. Omitted pays the whole payable; less than it is a partial payment: the statement becomes `partially_paid` and the balance goes to the open statement as a `carryover` launch. |
| `payment_date` | `string` | não | Omitted = today. |
| `financial_account_id` | `string (uuid)` | não | Account the money leaves from. Omitted = the account on the payable, else the card's paying account. |

Outros campos e objetos aninhados: `kobana schema finance.card-statements.payments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The statement as it is now. |
| `400` | Malformed JSON body, or `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; the statement is not `closed` nor `reviewed` (`reason: statement_not_payable`) or has no payable (`reason: statement_without_payable`); the payable is not pending (`reason: payable_not_pending`) or is at the bank (`reason: payable_in_bank`) or still waits for approval or was rejected (`reason: payable_not_approved`); the amount is zero (`reason: amount_not_positive`) or above the payable (`reason: payment_exceeds_total`); the account is a card's (`reason: funding_account_is_card`) or of another company (`reason: funding_account_other_company`); an impossible `payment_date` (`reason: invalid_date`). |

## card-statements payments delete

Desfazer o pagamento de uma fatura

```text
kobana finance card-statements payments delete [ID] [OPÇÕES]
```

`DELETE /v1/card-statements/{id}/payments`

Undoes the payment of the statement's payable: the transfer is deleted, the payable goes back to pending with the statement's total, the statement returns to `reviewed` (or `closed`) and its launches to `actual`, and a carryover the payment created is removed. Refused while that carryover sits in a statement that already closed. Records a `payment_reverted` event. Requires scope `finance/credit_cards` with access level `delete`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The statement as it is now. |
| `404` | Card statement not found within the tenant. |
| `422` | The statement has no payable (`reason: statement_without_payable`), nothing was paid (`reason: statement_not_paid`), or its carryover is in a closed statement (`reason: carryover_statement_closed`). |

## card-statements get

Buscar uma fatura do cartão pelo id

```text
kobana finance card-statements get [ID] [OPÇÕES]
```

`GET /v1/card-statements/{id}`

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
| `200` | Get a card statement by id |
| `404` | Card statement not found within the tenant. |

## card-statements close

Fechar uma fatura

```text
kobana finance card-statements close [ID] [OPÇÕES]
```

`POST /v1/card-statements/{id}/close`

Closes an `open` statement — what the daily job does the day after its cutoff — and opens the next one. A credit card's statement creates its payable: the issuer as the person, `expected_total` as the amount, the statement's `due_date`, the card's payer account as the expected account and its payment method, no category, entering the approval queue when the workspace's approval is on. A prepaid or benefit card, or a statement with nothing to pay, closes without one. Closing again after a reopening updates the same payable to the new total, and a payment already made settles it again: `paid`, or `partially_paid` with the balance carried to the open statement. Records a `closed` event in the history. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The statement as it is now. |
| `400` | `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | The statement is not `open` (`reason: statement_not_open`). |

## card-statements cutoff

Alterar o corte de uma fatura

```text
kobana finance card-statements cutoff [ID] [OPÇÕES]
```

`PATCH /v1/card-statements/{id}/cutoff`

Sets the real cutoff of an `open` or `future` statement and marks it confirmed; the next statement starts the day after. Launches dated between the old and the new cutoff change statement: installments follow the first one, one per statement, and launches moved by hand (`statement_locked`) stay. With `dry_run=true` nothing is written and the answer lists the launches that would move — show it before applying. Applying records a `cutoff_changed` event in the statement's history. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `dry_run` | `string` | não | `true` previews the change without writing it. A preview ignores `X-Idempotency-Key`. Valores: `true`, `false`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cutoff_date` | `string` | sim | The real cutoff (inclusive). Sending the predicted date only confirms it. |

Outros campos e objetos aninhados: `kobana schema finance.card-statements.cutoff`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The statement and the next one as they are (or would be), and the launches that changed statement. |
| `400` | Malformed JSON body or `dry_run`. |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; a statement already closed, reviewed or paid (`reason: statement_closed` — reopen it first); or a cutoff before the statement's `period_start` (`reason: cutoff_before_period_start`) or not before the next statement's cutoff (`reason: cutoff_after_next_cutoff`); on a reopened statement whose next one already closed, any new date (`reason: next_statement_closed`). |

## card-statements events

Listar o histórico de uma fatura

```text
kobana finance card-statements events [ID] [OPÇÕES]
```

`GET /v1/card-statements/{id}/events`

Returns the statement's history, oldest first: cutoff changes (with the launches they moved), closing, review, reopening, payment and payment reversal, each with who did it and through which channel. Requires scope `finance/credit_cards` with access level `read`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `kind` | `string` | não | Filter by event kind. Accepts `[in]`. Valores: `cutoff_changed`, `closed`, `reviewed`, `reopened`, `paid`, `payment_reverted`. |
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
| `200` | Page of events. |
| `400` | Malformed query. |
| `404` | Card statement not found within the tenant. |

## card-statements reopen

Reabrir uma fatura

```text
kobana finance card-statements reopen [ID] [OPÇÕES]
```

`POST /v1/card-statements/{id}/reopen`

Takes a `closed`, `reviewed`, `paid` or `partially_paid` statement back to `open` and records who reopened it (`reopened_at`, `reopened_by_email`). Its payable and its payment stay; the balance a partial payment carried over is removed (the next closing carries it again) and the launches go back from `reconciled` to `actual`. Refused while that balance sits in a statement that already closed. The next statement goes back to `future`, so the card keeps one open statement, and the cutoff can be changed again. The next closing — by hand or by the daily job the day after the cutoff — recalculates the total. Records a `reopened` event. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The statement as it is now. |
| `400` | `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | The statement is not `closed`, `reviewed`, `paid` nor `partially_paid` (`reason: statement_not_reopenable`), or the balance it carried over is in a closed statement (`reason: carryover_statement_closed`). |

## card-statements review

Conferir uma fatura com o total do emissor

```text
kobana finance card-statements review [ID] [OPÇÕES]
```

`POST /v1/card-statements/{id}/review`

Stores the total (and the minimum payment) of the issuer's real statement on a `closed` or `reviewed` statement. When `issuer_total` equals `expected_total` the statement becomes `reviewed` and its payable, while still pending, takes the issuer's amount; otherwise it stays `closed` and the difference is `issuer_total − expected_total`. A match records a `reviewed` event. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `issuer_total` | `number` | sim | Total of the issuer's real statement. When it equals `expected_total` the statement becomes `reviewed`. |
| `minimum_payment` | `number` | não | Minimum payment printed on the statement. Omitted keeps the stored one; `null` clears it. |

Outros campos e objetos aninhados: `kobana schema finance.card-statements.review`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The statement as it is now. |
| `400` | Malformed JSON body. |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or the statement is not `closed` nor `reviewed` (`reason: statement_not_closed`). |
