---
id: financial-accounts
title: "kobana finance financial-accounts"
sidebar_label: "financial-accounts"
---

# kobana finance financial-accounts


Comandos de `kobana finance financial-accounts`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`financial-accounts balances latest`](#financial-accounts-balances-latest) | `GET /v1/financial-accounts/{id}/balances/latest` | Último saldo de cada origen |
| [`financial-accounts balances list`](#financial-accounts-balances-list) | `GET /v1/financial-accounts/{id}/balances` | Listar saldos registrados de la cuenta |
| [`financial-accounts balances create`](#financial-accounts-balances-create) | `POST /v1/financial-accounts/{id}/balances` | Registrar un saldo |
| [`financial-accounts balances delete`](#financial-accounts-balances-delete) | `DELETE /v1/financial-accounts/{id}/balances/{balanceId}` | Eliminar saldo registrado |
| [`financial-accounts list`](#financial-accounts-list) | `GET /v1/financial-accounts` | Listar cuentas financieras |
| [`financial-accounts create`](#financial-accounts-create) | `POST /v1/financial-accounts` | Crear cuenta financiera |
| [`financial-accounts get`](#financial-accounts-get) | `GET /v1/financial-accounts/{id}` | Obtener cuenta financiera por ID |
| [`financial-accounts update`](#financial-accounts-update) | `PATCH /v1/financial-accounts/{id}` | Actualizar cuenta financiera |
| [`financial-accounts delete`](#financial-accounts-delete) | `DELETE /v1/financial-accounts/{id}` | Eliminar cuenta financiera |
| [`financial-accounts adjust-balance`](#financial-accounts-adjust-balance) | `POST /v1/financial-accounts/{id}/adjust-balance` | Ajustar saldo de la cuenta |

## financial-accounts balances latest

Último saldo de cada origen

```text
kobana finance financial-accounts balances latest [ID] [OPCIONES]
```

`GET /v1/financial-accounts/{id}/balances/latest`

Returns the most recent observation of each source in one call — what a source picker needs without one request per source. A source that never reported comes back with `amount: null`, which is deliberately different from a source that reported zero. `calculated` is always recomputed from the posted transactions rather than read from its stored snapshots, so it can never disagree with `balance` on the account detail. Requires scope `finance/accounts` with access level `read`.

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
| `200` | One entry per source. |
| `404` | Account not found within the tenant. |

## financial-accounts balances list

Listar saldos registrados de la cuenta

```text
kobana finance financial-accounts balances list [ID] [OPCIONES]
```

`GET /v1/financial-accounts/{id}/balances`

Returns a cursor-paginated page of the account's recorded balances, newest `capture_date` first. **These are observations, not the computed balance** — the transaction-derived number is `balance` on `GET /financial-accounts/{id}`. Each row carries the `source` that produced it, so a balance the bank reported and one a person typed stay distinguishable. Requires scope `finance/accounts` with access level `read`.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `source` | `string` | no | Filter by the channel that produced the observation. Accepts `[in]=banking,manual`. Valores: `calculated`, `banking`, `open_finance`, `manual`, `imported`, `api`, `mcp`, `cli`. |
| `capture_date` | `string` | no | Filter on the observation instant. Accepts the range operators `[gte]` / `[lte]` / `[gt]` / `[lt]`. |
| `external_id` | `string` | no | Filter by your own id for the observation. Accepts `[in]`. |
| `sort_by` | `string` | no | Default `capture_date`. Valores: `capture_date`, `created_at`, `amount`. |
| `sort_order` | `string` | no | Default `desc` — the newest observation first. Valores: `asc`, `desc`. |
| `limit` | `integer` | no | Default 25. |
| `cursor` | `string` | no | Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Page of recorded balances plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |
| `404` | Account not found within the tenant. |

## financial-accounts balances create

Registrar un saldo

```text
kobana finance financial-accounts balances create [ID] [OPCIONES]
```

`POST /v1/financial-accounts/{id}/balances`

Records one observed balance for the account. This does NOT move money and creates no transaction — use `POST /financial-accounts/{id}/adjust-balance` when you want the ledger corrected. `source` defaults to `api`; narrow it to `mcp` or `cli` so the audit trail names the real channel. The sources the product produces itself (`manual`, `banking`, `open_finance`, `calculated`, `imported`) are refused (422). Re-sending the same `external_id` for the same account and source returns 409. The new row's id is the `balanceId` used by the delete. Requires scope `finance/accounts` with access level `write`; pass an `X-Idempotency-Key` header on retries.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Balance in BRL. Signed — an overdrawn account is negative. |
| `capture_date` | `string (date-time)` | no | Instant the balance refers to. Defaults to now. |
| `source` | `string` | no | Which client is recording this. Defaults to `api`; send `mcp` or `cli` so the audit trail names the real channel. `manual`, `banking`, `calculated` and `imported` are refused — each has a single producer inside the product and must not be forgeable from a request. Valores: `api`, `mcp`, `cli`. |
| `blocked_amount` | `number` | no |  |
| `automatically_invested_amount` | `number` | no |  |
| `external_id` | `string` | no | Your own id for this observation, unique per account and source. |
| `tags` | `array<string>` | no |  |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema finance.financial-accounts.balances.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Balance recorded. `Location` points at the new row. |
| `400` | Malformed JSON body. |
| `404` | Account not found within the tenant. |
| `409` | `X-Idempotency-Key` reuse with a different body, or an `external_id` already used for this account and source. |
| `422` | Validation failure, including a `source` the product produces itself. |

## financial-accounts balances delete

Eliminar saldo registrado

```text
kobana finance financial-accounts balances delete [ID] [BALANCEID] [OPCIONES]
```

`DELETE /v1/financial-accounts/{id}/balances/{balanceId}`

Removes one recorded balance. A `banking` or `calculated` row is refused (422): both are reproducible from their producer, so the delete would be undone by the next import or snapshot. Requires scope `finance/accounts` with access level `delete`.

**Argumentos de la ruta:** `id`, `balanceId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Balance removed. |
| `404` | Balance not found within the tenant. |
| `422` | The row's source is reproducible and cannot be deleted. |

## financial-accounts list

Listar cuentas financieras

```text
kobana finance financial-accounts list [OPCIONES]
```

`GET /v1/financial-accounts`

Returns a cursor-paginated page of financial accounts for the workspace tied to the token's `sub` claim. Accounts may belong to a specific company or be workspace-level (`company_id` nullable). **The current balance is NOT returned by this listing** — the stored `balance` column is a write-only seed; the real balance is derived from the transactions ledger and will be available on the detail endpoint. Requires scope `finance/accounts` with access level `read`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string` | no | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2`. Workspace-level accounts (`company_id IS NULL`) are returned only when this filter is omitted. |
| `type` | `string` | no | Account kind. Accepts `[in]=checking,savings`. Valores: `checking`, `savings`, `cash`, `investment`, `credit`. |
| `bank_id` | `string` | no | Filter by bank UUID. Accepts `[in]` for multi-bank lookups. |
| `active` | `string` | no | Filter by active flag. Valores: `true`, `false`. |
| `name` | `string` | no | Filter by exact account name. Accepts `[in]`. |
| `sort_by` | `string` | no | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `created_at`, `name`, `type`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`; ignored without `sort_by`. Valores: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Page of accounts plus pagination metadata. No `balance` field by design. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## financial-accounts create

Crear cuenta financiera

```text
kobana finance financial-accounts create [OPCIONES]
```

`POST /v1/financial-accounts`

Creates a financial account. Requires scope `finance/accounts` with access level `write`. `workspace_id` is taken from the token. `company_id` is required (accounts must belong to a company; workspace-level accounts exist from imports but aren't creatable via v1). `balance` is an opening seed (default `0.00`) — it is NOT the live balance (that is computed from transactions and returned on the detail endpoint) and cannot be updated. The bank may be given as `bank_id` (UUID) or `bank_code` (public COMPE/BACEN, e.g. `341`) — not both (400); an unknown code → 422. The response echoes the resolved bank as `bank: { code, name }`. Pass an `X-Idempotency-Key` header on retries.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string (uuid)` | sí | Owning company UUID — required (financial accounts must belong to a company). Must belong to the tenant. Workspace-level accounts (`company_id: null`) exist from imports but are not creatable via v1. |
| `name` | `string` | sí |  |
| `type` | `string` | sí | Valores: `checking`, `savings`, `cash`, `investment`, `credit`. |
| `bank_id` | `string (uuid)` | no | Reference to a `banks` row. `null` for cash / non-bank accounts. |
| `bank_code` | `string` | no | Alternative to `bank_id`: the bank's public COMPE/BACEN code (e.g. `341`), resolved to the catalog bank server-side. Mutually exclusive with `bank_id` — sending both returns 400; an unknown code returns 422. |
| `agency` | `string` | no |  |
| `agency_digit` | `string` | no |  |
| `account_number` | `string` | no |  |
| `account_digit` | `string` | no |  |
| `balance` | `number` | no | Opening balance seed (signed number, up to 13 integer digits). Defaults to `0.00`. NOT the live balance — that is computed from transactions. |
| `disable_reconcile` | `boolean` | no |  |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Otros campos y objetos anidados: `kobana schema finance.financial-accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Account created. The body is the detail shape (includes the computed `balance`). |
| `400` | Malformed JSON body, or `bank_id` and `bank_code` sent together. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, `company_id` not in the tenant, or unknown `bank_code`. |

## financial-accounts get

Obtener cuenta financiera por ID

```text
kobana finance financial-accounts get [ID] [OPCIONES]
```

`GET /v1/financial-accounts/{id}`

Returns the account plus the **computed `balance`** (live, derived from posted transactions — not the creation seed; #13). The listing still omits balance. Returns the same shape as the rows from the list endpoint. Requires scope `finance/accounts` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a financial account by id |
| `404` | Account not found within the tenant. |

## financial-accounts update

Actualizar cuenta financiera

```text
kobana finance financial-accounts update [ID] [OPCIONES]
```

`PATCH /v1/financial-accounts/{id}`

Partial update. Requires scope `finance/accounts` with access level `write`. `balance` is never settable (the live balance is computed). Set `active: false` to soft-deactivate (existing transactions stay valid). The bank accepts `bank_id` or `bank_code` (not both → 400; unknown code → 422); the response echoes `bank: { code, name }`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string (uuid)` | no | Owning company UUID — required (financial accounts must belong to a company). Must belong to the tenant. Workspace-level accounts (`company_id: null`) exist from imports but are not creatable via v1. |
| `name` | `string` | no |  |
| `type` | `string` | no | Valores: `checking`, `savings`, `cash`, `investment`, `credit`. |
| `bank_id` | `string (uuid)` | no | Reference to a `banks` row. `null` for cash / non-bank accounts. |
| `bank_code` | `string` | no | Alternative to `bank_id`: the bank's public COMPE/BACEN code (e.g. `341`), resolved to the catalog bank server-side. Mutually exclusive with `bank_id` — sending both returns 400; an unknown code returns 422. |
| `agency` | `string` | no |  |
| `agency_digit` | `string` | no |  |
| `account_number` | `string` | no |  |
| `account_digit` | `string` | no |  |
| `disable_reconcile` | `boolean` | no |  |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `active` | `boolean` | no | Set `false` to soft-deactivate (existing transactions stay valid). |
| `custom_metadata_mode` | `string` | no | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema finance.financial-accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Updated account (detail shape with computed balance). |
| `400` | Malformed JSON body, or `bank_id` and `bank_code` sent together. |
| `404` | Account not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, `company_id` not in the tenant, or unknown `bank_code`. |

## financial-accounts delete

Eliminar cuenta financiera

```text
kobana finance financial-accounts delete [ID] [OPCIONES]
```

`DELETE /v1/financial-accounts/{id}`

Deletes an account. Requires scope `finance/accounts` with access level `delete`. Returns 422 (`reason: in_use`, with a `usage` breakdown) when the account has any transaction, payable or receivable — deleting would cascade-delete its transactions, so soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Account deleted. |
| `404` | Account not found within the tenant. |
| `422` | The account is in use (`reason: in_use`), is a credit card's own account (`reason: account_has_credit_card`) or pays a credit card (`reason: account_pays_credit_card`). |

## financial-accounts adjust-balance

Ajustar saldo de la cuenta

```text
kobana finance financial-accounts adjust-balance [ID] [OPCIONES]
```

`POST /v1/financial-accounts/{id}/adjust-balance`

Corrects the account's calculated balance to `target_balance` by inserting an auto-created adjustment transaction for the signed diff. Requires scope `finance/accounts` with access level `write`. The `category_id` type must match the diff direction (credit for a positive diff, debit for negative) and its company must match the account's. A target equal to the current balance is rejected (422). Pass an `X-Idempotency-Key` header on retries.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `target_balance` | `number` | sí | The balance the account should have after the adjustment. Signed number (up to 13 integer digits, 2 decimals). |
| `category_id` | `string (uuid)` | sí | Category for the adjustment transaction. Its type must match the diff direction (credit for a positive diff, debit for negative) and its company must match the account's. |
| `occurred_at` | `string (date-time)` | no | When the adjustment transaction is dated (ISO 8601). Defaults to now. |
| `description` | `string` | no | Overrides the auto-created transaction description (default "Ajuste de saldo"). |

Otros campos y objetos anidados: `kobana schema finance.financial-accounts.adjust-balance`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Adjustment created. The `Location` header points at the auto-created adjustment transaction (`GET /financial-transactions/{transaction_id}`). |
| `400` | Malformed JSON body. |
| `404` | Account or category not found within the tenant. |
| `409` | `X-Idempotency-Key` reuse with a different body. |
| `422` | Validation failure: target equals current balance, account without a company, or a category of the wrong type/company. |
