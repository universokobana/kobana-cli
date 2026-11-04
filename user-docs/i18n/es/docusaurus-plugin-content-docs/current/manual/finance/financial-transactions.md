---
id: financial-transactions
title: "kobana finance financial-transactions"
sidebar_label: "financial-transactions"
---

# kobana finance financial-transactions


Comandos de `kobana finance financial-transactions`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`financial-transactions aggregates`](#financial-transactions-aggregates) | `GET /v1/financial-transactions/aggregates` | Sumar totales de transacciones |
| [`financial-transactions ids`](#financial-transactions-ids) | `GET /v1/financial-transactions/ids` | Listar IDs de transacciones |
| [`financial-transactions list`](#financial-transactions-list) | `GET /v1/financial-transactions` | Listar transacciones |
| [`financial-transactions create`](#financial-transactions-create) | `POST /v1/financial-transactions` | Crear transacción |
| [`financial-transactions get`](#financial-transactions-get) | `GET /v1/financial-transactions/{id}` | Obtener transacción por ID |
| [`financial-transactions update`](#financial-transactions-update) | `PATCH /v1/financial-transactions/{id}` | Actualizar transacción |
| [`financial-transactions delete`](#financial-transactions-delete) | `DELETE /v1/financial-transactions/{id}` | Eliminar transacción |

## financial-transactions aggregates

Sumar totales de transacciones

```text
kobana finance financial-transactions aggregates [OPCIONES]
```

`GET /v1/financial-transactions/aggregates`

Sums realised transactions for the workspace tied to the token (forecasts excluded): `count`, `credit`, `debit`, `net`. Requires scope `finance/transactions` with access level `read`. Filter by `company_id` and a period; omit all filters for all-time.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string` | no | Scope to one company. Omit to aggregate across the whole tenant. |
| `month` | `integer` | no | Calendar month (1–12). Requires `year`. Mutually exclusive with `start_date`/`end_date`. |
| `year` | `integer,null` | no | Calendar year. Requires `month`. |
| `start_date` | `string` | no | Window start (ISO 8601). Requires `end_date`. Mutually exclusive with `month`/`year`. |
| `end_date` | `string` | no | Window end (ISO 8601). Requires `start_date`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Transaction totals for the scope. |
| `400` | Malformed query (unknown param, bad pairing, invalid date/uuid). |

## financial-transactions ids

Listar IDs de transacciones

```text
kobana finance financial-transactions ids [OPCIONES]
```

`GET /v1/financial-transactions/ids`

Every financial transaction id the token can see. Requires scope `finance/transactions` with access level `read`. Pairs with `?updated_at[gte]=` on the listing. The delta reports what changed; this reports what still exists. A delta can never report a disappearance — the row it would have to describe is gone — so an id that stops appearing here is the only signal that something left. Diff this set against your mirror and drop what is missing. Takes no filters: the sweep must cover exactly what the listing shows, and a filter applied here but not to your mirror would read as mass deletion. This is the largest resource in the API, which is what makes the pairing worth the trouble: re-paging the full history to find a handful of removals is exactly the cost the delta exists to avoid.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `string` | no | Page size. Default 1000, max 10000 — far above the listing's 100, because a row here is one UUID. |
| `cursor` | `string` | no | Opaque cursor from the previous page's `meta.next_cursor`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Live financial transaction ids. |
| `400` | Malformed query. This endpoint takes no filters or sort — passing one is a 400. |

## financial-transactions list

Listar transacciones

```text
kobana finance financial-transactions list [OPCIONES]
```

`GET /v1/financial-transactions`

Returns a cursor-paginated page of transactions for the workspace tied to the token's `sub` claim. Requires scope `finance/transactions` with access level `read`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string` | no | Filter by company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `type` | `string` | no | `debit` (money out) or `credit` (money in). Accepts `[in]=debit,credit`. Valores: `debit`, `credit`. |
| `financial_account_id` | `string` | no | Filter by financial account UUID. Accepts `[in]`. |
| `person_id` | `string` | no | Filter by person UUID. Accepts `[in]`. |
| `category_id` | `string` | no | Filter by category UUID. Accepts `[in]`. |
| `classification_center_id` | `string` | no | Filter by classification-center UUID. Accepts `[in]`. |
| `occurred_at` | `string` | no | ISO date filter on the movement date. Range operators supported: `[gte]`, `[lte]`, `[gt]`, `[lt]`. Example: `?occurred_at[gte]=2026-01-01&occurred_at[lte]=2026-12-31`. |
| `updated_at` | `string` | no | ISO timestamp filter on the last write. Range operators supported. This is the incremental-sync window: poll `?updated_at[gte]=<end of your last poll>&sort_by=updated_at` to fetch only what changed. Deletions do NOT appear here — a deleted transaction has no row to return; reconcile them against `GET /v1/financial-transactions/ids`. |
| `amount` | `string` | no | Decimal filter on amount (BRL). Range operators supported: `[gte]`, `[lte]`, `[gt]`, `[lt]`. A query parameter is always text, unlike the JSON body and the response, where money is a number. |
| `sort_by` | `string` | no | Field to order by. Default: `sort_by=occurred_at&sort_order=desc`. Valores: `amount`, `created_at`, `occurred_at`, `updated_at`. |
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
| `200` | Page of transactions plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## financial-transactions create

Crear transacción

```text
kobana finance financial-transactions create [OPCIONES]
```

`POST /v1/financial-transactions`

Creates a single transaction. Requires scope `finance/transactions` with access level `write`. `workspace_id` is always taken from the token, never from the body. `auto_created` and `transfer_id` are always server-controlled — values in the request are ignored. `{name}` accepted as alternative to `{id}` inside the nested `person` / `category` / `classification_center` object, for CSV-importer ergonomics; sending both inside the same object returns 400. Send `classification_centers[]` to split the amount across classification centers (rateio): effective amounts must sum to `amount` (1-cent tolerance) and each center's polarity must match `type` (`both` matches either). When present it supersedes the single `classification_center` field (which then mirrors rows[0]); the full split rides back on every read as `classification_center_allocations[]`. `categories[]` splits it across categories the same way (no polarity check) and rides back as `category_allocations[]`; it supersedes the single `category` field, which mirrors rows[0]. Pass an `X-Idempotency-Key` header on retries to avoid duplicate inserts.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string (uuid)` | sí | Owning company UUID. Must belong to the tenant and match the account's `company_id` (or the account must be workspace-level). |
| `financial_account_id` | `string (uuid)` | sí | Settlement account UUID. Must belong to the tenant. |
| `description` | `string` | sí |  |
| `amount` | `number` | sí | A number in BRL (up to 13 integer digits, 2 decimals). Always positive — direction is carried by `type`. |
| `occurred_at` | `string (date-time)` | sí | When the movement hit the account (ISO 8601). |
| `type` | `string` | sí | `debit` = money out, `credit` = money in. Drives the polarity for find-or-create on `{name}` fallbacks. Valores: `debit`, `credit`. |
| `bank_description` | `string` | no |  |
| `document` | `string` | no |  |
| `document_date` | `string (date-time)` | no | Issue date of the source document (ISO 8601). Pass `null` (PATCH) to clear. |
| `competence_date` | `string (date-time)` | no | Accounting period this movement belongs to. Defaults to `occurred_at` when omitted. |
| `category` | `object` | no | `{id}` for an existing category, or `{name}` to find-or-create one scoped to (workspace, company) with polarity derived from `type`. Sending both `id` and `name` returns 400. On update, sent without `categories` it replaces a category rateio with this single category. |
| `person` | `object` | no | `{id}` for an existing person, or `{name}` to find-or-create one with `customer` polarity for credits / `supplier` for debits. Never both. |
| `classification_center` | `object` | no | `{id}` for an existing classification center, or `{name}` to find-or-create one with `revenue` polarity for credits / `cost` for debits — a center typed `both` matches either; creates with the narrow polarity when no match. Never both. |
| `classification_centers` | `array<object>` | no | Rateio (split) across classification centers. Replace-all: the array becomes the row's set; an empty array clears it; omit to leave it alone. Effective amounts must sum to `amount` (1-cent tolerance). Each center's polarity must match the transaction `type` (`both` matches either). When present, the single `classification_center` field is ignored and the legacy FK mirrors rows[0]. |
| `categories` | `array<object>` | no | Category rateio. Replace-all set; an empty array clears it (and `category`); omit to leave it alone. Effective amounts must sum to `amount` (tolerance: the larger of 0.01 and 0.005 per row). When present, `category` is ignored and becomes rows[0]; every category must be of type the transaction's `type` (422 otherwise). The cents a percentage leaves go to the last slice sent without an `amount`. |
| `custom_classifications` | `array<object>` | no | Custom-classifier assignments. Replace-all: the array becomes the row's set; an empty array clears it; omit to leave it alone. One value per dimension for native single-select. |
| `payment_method_kind` | `string` | no | Valores: `bank_billet`, `ted`, `pix`, `credit_card`, `debit_card`, `cash`, `check`, `direct_debit`, `other`, `none`, `null`. |
| `classified` | `boolean` | no | Defaults to `false`. Setting `true` marks the row as already-categorised in the UI. |
| `notes` | `string` | no |  |
| `is_pending` | `boolean` | no | Defaults to `false`. When `true`, the row is treated as a forecast — excluded from realised-balance and dashboard income/expense aggregates. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Otros campos y objetos anidados: `kobana schema finance.financial-transactions.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Transaction created. The `Location` header points at the new row. `X-Idempotent-Replay: true` indicates a cached response from a previous identical request. |
| `400` | Malformed JSON body, or `id` and `name` sent together inside the same nested entity reference. |
| `409` | `X-Idempotency-Key` was reused with a different request body. |
| `422` | Schema validation failure, or domain consistency violation (company / account not in tenant, account does not match company, FK out of scope). |

## financial-transactions get

Obtener transacción por ID

```text
kobana finance financial-transactions get [ID] [OPCIONES]
```

`GET /v1/financial-transactions/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/transactions` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a transaction by id |
| `404` | Transaction not found within the tenant. |

## financial-transactions update

Actualizar transacción

```text
kobana finance financial-transactions update [ID] [OPCIONES]
```

`PATCH /v1/financial-transactions/{id}`

Partial update. Only fields present in the body are written. Requires scope `finance/transactions` with access level `write`. Read-only fields in the body (`id`, `workspace_id`, `company_id`, `financial_account_id`, `auto_created`, `transfer_id`, `created_at`) are silently dropped — the response is the new persisted state.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `data` | `object` | sí | Request body to update a financial transaction: send only the fields to change. |

Otros campos y objetos anidados: `kobana schema finance.financial-transactions.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Updated transaction. |
| `400` | Malformed JSON body, or `id` and `name` sent together inside the same nested entity reference. |
| `404` | Transaction not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure or FK out of scope. |

## financial-transactions delete

Eliminar transacción

```text
kobana finance financial-transactions delete [ID] [OPCIONES]
```

`DELETE /v1/financial-transactions/{id}`

Hard-deletes the row. Requires scope `finance/transactions` with access level `delete`. Deleting an auto-created row (one generated by paying / receiving a payable / receivable) returns 422 — cancel the source billing instead. Calling DELETE twice on the same id returns 200 then 404 (Stripe-style).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Transaction deleted. |
| `404` | Transaction not found within the tenant. |
| `422` | Auto-created transaction — cancel the parent payable / receivable instead. |
