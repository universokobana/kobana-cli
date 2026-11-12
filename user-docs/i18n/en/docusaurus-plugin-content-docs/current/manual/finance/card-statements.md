---
id: card-statements
title: "kobana finance card-statements"
sidebar_label: "card-statements"
---

# kobana finance card-statements


Commands of `kobana finance card-statements`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`card-statements payments create`](#card-statements-payments-create) | `POST /v1/card-statements/{id}/payments` | Pay a statement |
| [`card-statements payments delete`](#card-statements-payments-delete) | `DELETE /v1/card-statements/{id}/payments` | Undo a statement's payment |
| [`card-statements get`](#card-statements-get) | `GET /v1/card-statements/{id}` | Get a card statement by id |
| [`card-statements close`](#card-statements-close) | `POST /v1/card-statements/{id}/close` | Close a statement |
| [`card-statements cutoff`](#card-statements-cutoff) | `PATCH /v1/card-statements/{id}/cutoff` | Change a statement's cutoff |
| [`card-statements events`](#card-statements-events) | `GET /v1/card-statements/{id}/events` | List a statement's history |
| [`card-statements reopen`](#card-statements-reopen) | `POST /v1/card-statements/{id}/reopen` | Reopen a statement |
| [`card-statements review`](#card-statements-review) | `POST /v1/card-statements/{id}/review` | Review a statement against the issuer's total |

## card-statements payments create

Pay a statement

```text
kobana finance card-statements payments create [ID] [OPTIONS]
```

`POST /v1/card-statements/{id}/payments`

Settles the payable of a `closed` or `reviewed` statement. The payment is a transfer from the paying account to the card's account, never an expense: the purchases already count through the card's launches. Paying the whole payable makes the statement `paid`; less than it makes it `partially_paid`, lowers the payable to the amount paid and carries the balance to the open statement as a `carryover` launch. Either way the launches become `reconciled`. Settling the payable through `PATCH /payables/{id}` or the bank's confirmation gives the same result. Records a `paid` event. Requires scope `finance/credit_cards` with access level `write`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `amount` | `number` | no | Amount paid. Omitted pays the whole payable; less than it is a partial payment: the statement becomes `partially_paid` and the balance goes to the open statement as a `carryover` launch. |
| `payment_date` | `string` | no | Omitted = today. |
| `financial_account_id` | `string (uuid)` | no | Account the money leaves from. Omitted = the account on the payable, else the card's paying account. |

More fields and nested objects: `kobana schema finance.card-statements.payments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The statement as it is now. |
| `400` | Malformed JSON body, or `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; the statement is not `closed` nor `reviewed` (`reason: statement_not_payable`) or has no payable (`reason: statement_without_payable`); the payable is not pending (`reason: payable_not_pending`) or is at the bank (`reason: payable_in_bank`) or still waits for approval or was rejected (`reason: payable_not_approved`); the amount is zero (`reason: amount_not_positive`) or above the payable (`reason: payment_exceeds_total`); the account is a card's (`reason: funding_account_is_card`) or of another company (`reason: funding_account_other_company`); an impossible `payment_date` (`reason: invalid_date`). |

## card-statements payments delete

Undo a statement's payment

```text
kobana finance card-statements payments delete [ID] [OPTIONS]
```

`DELETE /v1/card-statements/{id}/payments`

Undoes the payment of the statement's payable: the transfer is deleted, the payable goes back to pending with the statement's total, the statement returns to `reviewed` (or `closed`) and its launches to `actual`, and a carryover the payment created is removed. Refused while that carryover sits in a statement that already closed. Records a `payment_reverted` event. Requires scope `finance/credit_cards` with access level `delete`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The statement as it is now. |
| `404` | Card statement not found within the tenant. |
| `422` | The statement has no payable (`reason: statement_without_payable`), nothing was paid (`reason: statement_not_paid`), or its carryover is in a closed statement (`reason: carryover_statement_closed`). |

## card-statements get

Get a card statement by id

```text
kobana finance card-statements get [ID] [OPTIONS]
```

`GET /v1/card-statements/{id}`

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
| `200` | Get a card statement by id |
| `404` | Card statement not found within the tenant. |

## card-statements close

Close a statement

```text
kobana finance card-statements close [ID] [OPTIONS]
```

`POST /v1/card-statements/{id}/close`

Closes an `open` statement — what the daily job does the day after its cutoff — and opens the next one. A credit card's statement creates its payable: the issuer as the person, `expected_total` as the amount, the statement's `due_date`, the card's payer account as the expected account and its payment method, no category, entering the approval queue when the workspace's approval is on. A prepaid or benefit card, or a statement with nothing to pay, closes without one. Closing again after a reopening updates the same payable to the new total, and a payment already made settles it again: `paid`, or `partially_paid` with the balance carried to the open statement. Records a `closed` event in the history. Requires scope `finance/credit_cards` with access level `write`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The statement as it is now. |
| `400` | `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | The statement is not `open` (`reason: statement_not_open`). |

## card-statements cutoff

Change a statement's cutoff

```text
kobana finance card-statements cutoff [ID] [OPTIONS]
```

`PATCH /v1/card-statements/{id}/cutoff`

Sets the real cutoff of an `open` or `future` statement and marks it confirmed; the next statement starts the day after. Launches dated between the old and the new cutoff change statement: installments follow the first one, one per statement, and launches moved by hand (`statement_locked`) stay. With `dry_run=true` nothing is written and the answer lists the launches that would move — show it before applying. Applying records a `cutoff_changed` event in the statement's history. Requires scope `finance/credit_cards` with access level `write`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `dry_run` | `string` | no | `true` previews the change without writing it. A preview ignores `X-Idempotency-Key`. Values: `true`, `false`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `cutoff_date` | `string` | yes | The real cutoff (inclusive). Sending the predicted date only confirms it. |

More fields and nested objects: `kobana schema finance.card-statements.cutoff`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The statement and the next one as they are (or would be), and the launches that changed statement. |
| `400` | Malformed JSON body or `dry_run`. |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; a statement already closed, reviewed or paid (`reason: statement_closed` — reopen it first); or a cutoff before the statement's `period_start` (`reason: cutoff_before_period_start`) or not before the next statement's cutoff (`reason: cutoff_after_next_cutoff`); on a reopened statement whose next one already closed, any new date (`reason: next_statement_closed`). |

## card-statements events

List a statement's history

```text
kobana finance card-statements events [ID] [OPTIONS]
```

`GET /v1/card-statements/{id}/events`

Returns the statement's history, oldest first: cutoff changes (with the launches they moved), closing, review, reopening, payment and payment reversal, each with who did it and through which channel. Requires scope `finance/credit_cards` with access level `read`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `kind` | `string` | no | Filter by event kind. Accepts `[in]`. Values: `cutoff_changed`, `closed`, `reviewed`, `reopened`, `paid`, `payment_reverted`. |
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
| `200` | Page of events. |
| `400` | Malformed query. |
| `404` | Card statement not found within the tenant. |

## card-statements reopen

Reopen a statement

```text
kobana finance card-statements reopen [ID] [OPTIONS]
```

`POST /v1/card-statements/{id}/reopen`

Takes a `closed`, `reviewed`, `paid` or `partially_paid` statement back to `open` and records who reopened it (`reopened_at`, `reopened_by_email`). Its payable and its payment stay; the balance a partial payment carried over is removed (the next closing carries it again) and the launches go back from `reconciled` to `actual`. Refused while that balance sits in a statement that already closed. The next statement goes back to `future`, so the card keeps one open statement, and the cutoff can be changed again. The next closing — by hand or by the daily job the day after the cutoff — recalculates the total. Records a `reopened` event. Requires scope `finance/credit_cards` with access level `write`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The statement as it is now. |
| `400` | `X-Idempotency-Key` empty or longer than 255 characters (`reason: invalid_idempotency_key`). |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | The statement is not `closed`, `reviewed`, `paid` nor `partially_paid` (`reason: statement_not_reopenable`), or the balance it carried over is in a closed statement (`reason: carryover_statement_closed`). |

## card-statements review

Review a statement against the issuer's total

```text
kobana finance card-statements review [ID] [OPTIONS]
```

`POST /v1/card-statements/{id}/review`

Stores the total (and the minimum payment) of the issuer's real statement on a `closed` or `reviewed` statement. When `issuer_total` equals `expected_total` the statement becomes `reviewed` and its payable, while still pending, takes the issuer's amount; otherwise it stays `closed` and the difference is `issuer_total − expected_total`. A match records a `reviewed` event. Requires scope `finance/credit_cards` with access level `write`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `issuer_total` | `number` | yes | Total of the issuer's real statement. When it equals `expected_total` the statement becomes `reviewed`. |
| `minimum_payment` | `number` | no | Minimum payment printed on the statement. Omitted keeps the stored one; `null` clears it. |

More fields and nested objects: `kobana schema finance.card-statements.review`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The statement as it is now. |
| `400` | Malformed JSON body. |
| `404` | Card statement not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or the statement is not `closed` nor `reviewed` (`reason: statement_not_closed`). |
