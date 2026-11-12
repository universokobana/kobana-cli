---
id: financial-accounts
title: "kobana finance financial-accounts"
sidebar_label: "financial-accounts"
---

# kobana finance financial-accounts


Comandos de `kobana finance financial-accounts`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`financial-accounts balances latest`](#financial-accounts-balances-latest) | `GET /v1/financial-accounts/{id}/balances/latest` | Último saldo de cada origem |
| [`financial-accounts balances list`](#financial-accounts-balances-list) | `GET /v1/financial-accounts/{id}/balances` | Listar saldos registrados da conta |
| [`financial-accounts balances create`](#financial-accounts-balances-create) | `POST /v1/financial-accounts/{id}/balances` | Registrar um saldo |
| [`financial-accounts balances delete`](#financial-accounts-balances-delete) | `DELETE /v1/financial-accounts/{id}/balances/{balanceId}` | Excluir saldo registrado |
| [`financial-accounts list`](#financial-accounts-list) | `GET /v1/financial-accounts` | Listar contas financeiras |
| [`financial-accounts create`](#financial-accounts-create) | `POST /v1/financial-accounts` | Criar conta financeira |
| [`financial-accounts get`](#financial-accounts-get) | `GET /v1/financial-accounts/{id}` | Obter conta financeira por ID |
| [`financial-accounts update`](#financial-accounts-update) | `PATCH /v1/financial-accounts/{id}` | Atualizar conta financeira |
| [`financial-accounts delete`](#financial-accounts-delete) | `DELETE /v1/financial-accounts/{id}` | Excluir conta financeira |
| [`financial-accounts adjust-balance`](#financial-accounts-adjust-balance) | `POST /v1/financial-accounts/{id}/adjust-balance` | Ajustar saldo da conta |

## financial-accounts balances latest

Último saldo de cada origem

```text
kobana finance financial-accounts balances latest [ID] [OPÇÕES]
```

`GET /v1/financial-accounts/{id}/balances/latest`

Returns the most recent observation of each source in one call — what a source picker needs without one request per source. A source that never reported comes back with `amount: null`, which is deliberately different from a source that reported zero. `calculated` is always recomputed from the posted transactions rather than read from its stored snapshots, so it can never disagree with `balance` on the account detail. Requires scope `finance/accounts` with access level `read`.

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
| `200` | One entry per source. |
| `404` | Account not found within the tenant. |

## financial-accounts balances list

Listar saldos registrados da conta

```text
kobana finance financial-accounts balances list [ID] [OPÇÕES]
```

`GET /v1/financial-accounts/{id}/balances`

Returns a cursor-paginated page of the account's recorded balances, newest `capture_date` first. **These are observations, not the computed balance** — the transaction-derived number is `balance` on `GET /financial-accounts/{id}`. Each row carries the `source` that produced it, so a balance the bank reported and one a person typed stay distinguishable. Requires scope `finance/accounts` with access level `read`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `source` | `string` | não | Filter by the channel that produced the observation. Accepts `[in]=banking,manual`. Valores: `calculated`, `banking`, `open_finance`, `manual`, `imported`, `api`, `mcp`, `cli`. |
| `capture_date` | `string` | não | Filter on the observation instant. Accepts the range operators `[gte]` / `[lte]` / `[gt]` / `[lt]`. |
| `external_id` | `string` | não | Filter by your own id for the observation. Accepts `[in]`. |
| `sort_by` | `string` | não | Default `capture_date`. Valores: `capture_date`, `created_at`, `amount`. |
| `sort_order` | `string` | não | Default `desc` — the newest observation first. Valores: `asc`, `desc`. |
| `limit` | `integer` | não | Default 25. |
| `cursor` | `string` | não | Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Page of recorded balances plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |
| `404` | Account not found within the tenant. |

## financial-accounts balances create

Registrar um saldo

```text
kobana finance financial-accounts balances create [ID] [OPÇÕES]
```

`POST /v1/financial-accounts/{id}/balances`

Records one observed balance for the account. This does NOT move money and creates no transaction — use `POST /financial-accounts/{id}/adjust-balance` when you want the ledger corrected. `source` defaults to `api`; narrow it to `mcp` or `cli` so the audit trail names the real channel. The sources the product produces itself (`manual`, `banking`, `open_finance`, `calculated`, `imported`) are refused (422). Re-sending the same `external_id` for the same account and source returns 409. The new row's id is the `balanceId` used by the delete. Requires scope `finance/accounts` with access level `write`; pass an `X-Idempotency-Key` header on retries.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Balance in BRL. Signed — an overdrawn account is negative. |
| `capture_date` | `string (date-time)` | não | Instant the balance refers to. Defaults to now. |
| `source` | `string` | não | Which client is recording this. Defaults to `api`; send `mcp` or `cli` so the audit trail names the real channel. `manual`, `banking`, `calculated` and `imported` are refused — each has a single producer inside the product and must not be forgeable from a request. Valores: `api`, `mcp`, `cli`. |
| `blocked_amount` | `number` | não |  |
| `automatically_invested_amount` | `number` | não |  |
| `external_id` | `string` | não | Your own id for this observation, unique per account and source. |
| `tags` | `array<string>` | não |  |
| `notes` | `string` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema finance.financial-accounts.balances.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Balance recorded. `Location` points at the new row. |
| `400` | Malformed JSON body. |
| `404` | Account not found within the tenant. |
| `409` | `X-Idempotency-Key` reuse with a different body, or an `external_id` already used for this account and source. |
| `422` | Validation failure, including a `source` the product produces itself. |

## financial-accounts balances delete

Excluir saldo registrado

```text
kobana finance financial-accounts balances delete [ID] [BALANCEID] [OPÇÕES]
```

`DELETE /v1/financial-accounts/{id}/balances/{balanceId}`

Removes one recorded balance. A `banking` or `calculated` row is refused (422): both are reproducible from their producer, so the delete would be undone by the next import or snapshot. Requires scope `finance/accounts` with access level `delete`.

**Argumentos do caminho:** `id`, `balanceId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Balance removed. |
| `404` | Balance not found within the tenant. |
| `422` | The row's source is reproducible and cannot be deleted. |

## financial-accounts list

Listar contas financeiras

```text
kobana finance financial-accounts list [OPÇÕES]
```

`GET /v1/financial-accounts`

Returns a cursor-paginated page of financial accounts for the workspace tied to the token's `sub` claim. Accounts may belong to a specific company or be workspace-level (`company_id` nullable). **The current balance is NOT returned by this listing** — the stored `balance` column is a write-only seed; the real balance is derived from the transactions ledger and will be available on the detail endpoint. Requires scope `finance/accounts` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2`. Workspace-level accounts (`company_id IS NULL`) are returned only when this filter is omitted. |
| `type` | `string` | não | Account kind. Accepts `[in]=checking,savings`. Valores: `checking`, `savings`, `cash`, `investment`, `credit`. |
| `bank_id` | `string` | não | Filter by bank UUID. Accepts `[in]` for multi-bank lookups. |
| `active` | `string` | não | Filter by active flag. Valores: `true`, `false`. |
| `name` | `string` | não | Filter by exact account name. Accepts `[in]`. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `created_at`, `name`, `type`. |
| `sort_order` | `string` | não | Order direction. Defaults to `asc`; ignored without `sort_by`. Valores: `asc`, `desc`. |
| `limit` | `integer` | não | Page size. Default 25, max 100. |
| `cursor` | `string` | não | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Page of accounts plus pagination metadata. No `balance` field by design. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## financial-accounts create

Criar conta financeira

```text
kobana finance financial-accounts create [OPÇÕES]
```

`POST /v1/financial-accounts`

Creates a financial account. Requires scope `finance/accounts` with access level `write`. `workspace_id` is taken from the token. `company_id` is required (accounts must belong to a company; workspace-level accounts exist from imports but aren't creatable via v1). `balance` is an opening seed (default `0.00`) — it is NOT the live balance (that is computed from transactions and returned on the detail endpoint) and cannot be updated. The bank may be given as `bank_id` (UUID) or `bank_code` (public COMPE/BACEN, e.g. `341`) — not both (400); an unknown code → 422. The response echoes the resolved bank as `bank: { code, name }`. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Owning company UUID — required (financial accounts must belong to a company). Must belong to the tenant. Workspace-level accounts (`company_id: null`) exist from imports but are not creatable via v1. |
| `name` | `string` | sim |  |
| `type` | `string` | sim | Valores: `checking`, `savings`, `cash`, `investment`, `credit`. |
| `bank_id` | `string (uuid)` | não | Reference to a `banks` row. `null` for cash / non-bank accounts. |
| `bank_code` | `string` | não | Alternative to `bank_id`: the bank's public COMPE/BACEN code (e.g. `341`), resolved to the catalog bank server-side. Mutually exclusive with `bank_id` — sending both returns 400; an unknown code returns 422. |
| `agency` | `string` | não |  |
| `agency_digit` | `string` | não |  |
| `account_number` | `string` | não |  |
| `account_digit` | `string` | não |  |
| `balance` | `number` | não | Opening balance seed (signed number, up to 13 integer digits). Defaults to `0.00`. NOT the live balance — that is computed from transactions. |
| `disable_reconcile` | `boolean` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.financial-accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Account created. The body is the detail shape (includes the computed `balance`). |
| `400` | Malformed JSON body, or `bank_id` and `bank_code` sent together. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, `company_id` not in the tenant, or unknown `bank_code`. |

## financial-accounts get

Obter conta financeira por ID

```text
kobana finance financial-accounts get [ID] [OPÇÕES]
```

`GET /v1/financial-accounts/{id}`

Returns the account plus the **computed `balance`** (live, derived from posted transactions — not the creation seed; #13). The listing still omits balance. Returns the same shape as the rows from the list endpoint. Requires scope `finance/accounts` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a financial account by id |
| `404` | Account not found within the tenant. |

## financial-accounts update

Atualizar conta financeira

```text
kobana finance financial-accounts update [ID] [OPÇÕES]
```

`PATCH /v1/financial-accounts/{id}`

Partial update. Requires scope `finance/accounts` with access level `write`. `balance` is never settable (the live balance is computed). Set `active: false` to soft-deactivate (existing transactions stay valid). The bank accepts `bank_id` or `bank_code` (not both → 400; unknown code → 422); the response echoes `bank: { code, name }`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | não | Owning company UUID — required (financial accounts must belong to a company). Must belong to the tenant. Workspace-level accounts (`company_id: null`) exist from imports but are not creatable via v1. |
| `name` | `string` | não |  |
| `type` | `string` | não | Valores: `checking`, `savings`, `cash`, `investment`, `credit`. |
| `bank_id` | `string (uuid)` | não | Reference to a `banks` row. `null` for cash / non-bank accounts. |
| `bank_code` | `string` | não | Alternative to `bank_id`: the bank's public COMPE/BACEN code (e.g. `341`), resolved to the catalog bank server-side. Mutually exclusive with `bank_id` — sending both returns 400; an unknown code returns 422. |
| `agency` | `string` | não |  |
| `agency_digit` | `string` | não |  |
| `account_number` | `string` | não |  |
| `account_digit` | `string` | não |  |
| `disable_reconcile` | `boolean` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `active` | `boolean` | não | Set `false` to soft-deactivate (existing transactions stay valid). |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.financial-accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated account (detail shape with computed balance). |
| `400` | Malformed JSON body, or `bank_id` and `bank_code` sent together. |
| `404` | Account not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, `company_id` not in the tenant, or unknown `bank_code`. |

## financial-accounts delete

Excluir conta financeira

```text
kobana finance financial-accounts delete [ID] [OPÇÕES]
```

`DELETE /v1/financial-accounts/{id}`

Deletes an account. Requires scope `finance/accounts` with access level `delete`. Returns 422 (`reason: in_use`, with a `usage` breakdown) when the account has any transaction, payable or receivable — deleting would cascade-delete its transactions, so soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Account deleted. |
| `404` | Account not found within the tenant. |
| `422` | The account is in use (`reason: in_use`), is a credit card's own account (`reason: account_has_credit_card`) or pays a credit card (`reason: account_pays_credit_card`). |

## financial-accounts adjust-balance

Ajustar saldo da conta

```text
kobana finance financial-accounts adjust-balance [ID] [OPÇÕES]
```

`POST /v1/financial-accounts/{id}/adjust-balance`

Corrects the account's calculated balance to `target_balance` by inserting an auto-created adjustment transaction for the signed diff. Requires scope `finance/accounts` with access level `write`. The `category_id` type must match the diff direction (credit for a positive diff, debit for negative) and its company must match the account's. A target equal to the current balance is rejected (422). Pass an `X-Idempotency-Key` header on retries.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `target_balance` | `number` | sim | The balance the account should have after the adjustment. Signed number (up to 13 integer digits, 2 decimals). |
| `category_id` | `string (uuid)` | sim | Category for the adjustment transaction. Its type must match the diff direction (credit for a positive diff, debit for negative) and its company must match the account's. |
| `occurred_at` | `string (date-time)` | não | When the adjustment transaction is dated (ISO 8601). Defaults to now. |
| `description` | `string` | não | Overrides the auto-created transaction description (default "Ajuste de saldo"). |

Outros campos e objetos aninhados: `kobana schema finance.financial-accounts.adjust-balance`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Adjustment created. The `Location` header points at the auto-created adjustment transaction (`GET /financial-transactions/{transaction_id}`). |
| `400` | Malformed JSON body. |
| `404` | Account or category not found within the tenant. |
| `409` | `X-Idempotency-Key` reuse with a different body. |
| `422` | Validation failure: target equals current balance, account without a company, or a category of the wrong type/company. |
