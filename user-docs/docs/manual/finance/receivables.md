---
id: receivables
title: "kobana finance receivables"
sidebar_label: "receivables"
---

# kobana finance receivables


Comandos de `kobana finance receivables`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`receivables aggregates`](#receivables-aggregates) | `GET /v1/receivables/aggregates` | Somar totais de contas a receber |
| [`receivables charge cancel`](#receivables-charge-cancel) | `POST /v1/receivables/{id}/charge/cancel` | Cancelar a cobrança de uma conta a receber |
| [`receivables charge create`](#receivables-charge-create) | `POST /v1/receivables/{id}/charge` | Gerar cobrança de uma conta a receber |
| [`receivables ids`](#receivables-ids) | `GET /v1/receivables/ids` | Listar IDs de contas a receber |
| [`receivables list`](#receivables-list) | `GET /v1/receivables` | Listar contas a receber |
| [`receivables create`](#receivables-create) | `POST /v1/receivables` | Criar conta a receber |
| [`receivables get`](#receivables-get) | `GET /v1/receivables/{id}` | Obter conta a receber por ID |
| [`receivables update`](#receivables-update) | `PATCH /v1/receivables/{id}` | Atualizar ou liquidar conta a receber |
| [`receivables delete`](#receivables-delete) | `DELETE /v1/receivables/{id}` | Excluir conta a receber |

## receivables aggregates

Somar totais de contas a receber

```text
kobana finance receivables aggregates [OPÇÕES]
```

`GET /v1/receivables/aggregates`

Sums receivables for the workspace tied to the token: `count`, `total`, `pending`, `received`, `overdue_count`. Requires scope `finance/receivables` with access level `read`. Filter by `company_id` and a period; omit all filters for all-time.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Scope to one company. Omit to aggregate across the whole tenant. |
| `month` | `integer` | não | Calendar month (1–12). Requires `year`. Mutually exclusive with `start_date`/`end_date`. |
| `year` | `integer,null` | não | Calendar year. Requires `month`. |
| `start_date` | `string` | não | Window start (ISO 8601). Requires `end_date`. Mutually exclusive with `month`/`year`. |
| `end_date` | `string` | não | Window end (ISO 8601). Requires `start_date`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Receivable totals for the scope. |
| `400` | Malformed query (unknown param, bad pairing, invalid date/uuid). |

## receivables charge cancel

Cancelar a cobrança de uma conta a receber

```text
kobana finance receivables charge cancel [ID] [OPÇÕES]
```

`POST /v1/receivables/{id}/charge/cancel`

Cancels the charge on the gateway. Same guards as issuing it — Requires scope `finance/receivable_charges` with access level `write`, and an `X-Idempotency-Key` header is required.

⚠️ **A 200 does not mean the charge is dead.** A bank slip stops being payable at once, but a Pix cancellation is a *command* the financial institution confirms later — and until it does, your customer can still pay it. Read `charge.state` on the response: `canceled` is done; anything else means the cancellation is still in flight, and the background poll will keep the mirror moving.

Cancelling is also how you unlock the bill: while a charge is live, `PATCH /receivables/{id}` refuses the fields that describe it (amount, due date, payer) and `DELETE` refuses the row — editing here would not re-issue anything, it would only make the record disagree with what the customer holds.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cancellation accepted. The body is the refreshed receivable detail — check `charge.state`. |
| `400` | Missing / empty `X-Idempotency-Key`. |
| `404` | Receivable not found within the tenant. |
| `409` | Nothing to cancel: the receivable has no charge, or the charge already reached a closed state (paid, cancelled, failed). Also returned when `X-Idempotency-Key` is reused with a different body. |

## receivables charge create

Gerar cobrança de uma conta a receber

```text
kobana finance receivables charge create [ID] [OPÇÕES]
```

`POST /v1/receivables/{id}/charge`

Issues a **bank slip** or a **Pix charge** on the gateway so the customer has something to pay, and mirrors it onto the receivable. Requires scope `finance/receivable_charges` with access level `write` — the money resource is its own, so a token reaching only `finance/receivables` cannot charge, and `finance/all` with only `read` cannot either. An `X-Idempotency-Key` header is **required**: a duplicate charge is a second debt your customer receives.

⚠️ **The charge in the response is not deliverable yet.** Registration at the institution is asynchronous, so it comes back `charge.state: "registering"` — a bank slip does not exist at the bank yet and a Pix has no `charge.pix_qrcode`. Poll `GET /receivables/{id}` until `charge.state` is `registered` before sending anything to the payer. There is no refresh endpoint on purpose: the mirror is kept current by a background poll and by the gateway's webhook, and a per-call re-read would burn the same hourly request budget the rest of the integration shares.

The two rails differ in what they require of the payer — a bank slip needs their **full address**, a Pix only name and document — and in what must be registered on the gateway account (an agreement vs. a Pix charging account). A missing piece comes back as 422 naming it; a state that forbids charging comes back as 409.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `kind` | `string` | sim | Which rail to issue on. They are not the same charge with a flag: a bank slip carries the payer inline and needs their **full address**, while a Pix needs only name and document — but a Pix charging account must exist on the gateway account, and a bank slip needs a registered agreement. Valores: `bank_billet`, `pix`. |

Outros campos e objetos aninhados: `kobana schema finance.receivables.charge.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Charge issued. The body is the refreshed receivable detail with the charge mirror populated (`charge.state` will read `registering`). |
| `400` | Malformed JSON, or a missing / empty `X-Idempotency-Key`. |
| `404` | Receivable not found within the tenant. |
| `409` | The receivable's state forbids charging: it already has a charge, or it was already received or cancelled. Also returned when `X-Idempotency-Key` is reused with a different body. |
| `422` | Something required is missing, and the message names it: no person linked, no CPF/CNPJ, no due date, no full address (bank slip only), or no charging account / agreement registered on the gateway account. |

## receivables ids

Listar IDs de contas a receber

```text
kobana finance receivables ids [OPÇÕES]
```

`GET /v1/receivables/ids`

Every receivable id the token can see. Requires scope `finance/receivables` with access level `read`. Pairs with `?updated_at[gte]=` on the listing. The delta reports what changed; this reports what still exists. A delta can never report a disappearance — the row it would have to describe is gone — so an id that stops appearing here is the only signal that something left. Diff this set against your mirror and drop what is missing. Takes no filters: the sweep must cover exactly what the listing shows, and a filter applied here but not to your mirror would read as mass deletion.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `string` | não | Page size. Default 1000, max 10000 — far above the listing's 100, because a row here is one UUID. |
| `cursor` | `string` | não | Opaque cursor from the previous page's `meta.next_cursor`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Live receivable ids. |
| `400` | Malformed query. This endpoint takes no filters or sort — passing one is a 400. |

## receivables list

Listar contas a receber

```text
kobana finance receivables list [OPÇÕES]
```

`GET /v1/receivables`

Returns a cursor-paginated page of receivables (accounts receivable) for the workspace tied to the token's `sub` claim. Mirror of `/payables` with the receive-side settlement amounts (`interest_amount`, `discount_amount`, `ticket_amount`) and a derived `nominal_amount = amount + interest - discount`. Status is the raw DB enum (`pending` / `received` / `cancelled`); derive overdue client-side. Every row also carries the **charge mirror** (`charge`) — whether a bank slip / Pix was issued for it and where that charge stands. `null` when never charged. Branch on `charge.state`, not on `charge.status`: the commercial axis alone calls a charge `opened` while its registration at the institution is still pending, and such a charge cannot be paid. Default sort is `due_date` ascending. Requires scope `finance/receivables` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]`. |
| `status` | `string` | não | Raw status. To find overdue receivables, combine `status=pending&due_date[lt]=<today ISO>`. Accepts `[in]`. Valores: `pending`, `received`, `cancelled`. |
| `type` | `string` | não | Billing kind. Accepts `[in]`. Valores: `normal`, `recurring`, `installment`. |
| `person_id` | `string` | não | Filter by person UUID. Accepts `[in]`. |
| `category_id` | `string` | não | Filter by category UUID. Accepts `[in]`. |
| `classification_center_id` | `string` | não | Filter by primary classification-center UUID. Accepts `[in]`. |
| `financial_account_id` | `string` | não | Filter by settlement account UUID. Accepts `[in]`. |
| `payment_method_kind` | `string` | não | Payment method enum. Accepts `[in]`. Valores: `bank_billet`, `ted`, `pix`, `credit_card`, `debit_card`, `cash`, `check`, `direct_debit`, `other`, `none`. |
| `due_date` | `string` | não | ISO date filter. Range operators: `[gte]`, `[lte]`, `[gt]`, `[lt]`. |
| `payment_date` | `string` | não | ISO date filter on the actual receive date (column is `payment_date` to match the DB schema). Range operators supported. |
| `competence_date` | `string` | não | ISO date filter on the accounting period. Range operators supported. |
| `updated_at` | `string` | não | ISO timestamp filter on the last write. Range operators supported. This is the incremental-sync window: poll `?updated_at[gte]=<end of your last poll>&sort_by=updated_at` to fetch only what changed. Deletions do NOT appear here — a deleted receivable has no row to return; reconcile them against `GET /v1/receivables/ids`. |
| `amount` | `string` | não | Decimal filter on amount (BRL). Range operators supported. |
| `document_number` | `string` | não | Filter by exact document number. Accepts `[in]`. |
| `ir_relevant` | `string` | não | Filter by tax-relevance flag. Valores: `true`, `false`. |
| `group_id` | `string` | não | Filter siblings of a recurring/installment series. |
| `kobana_charge_kind` | `string` | não | Which charge rail the receivable was issued on. Accepts `[in]`. Valores: `bank_billet`, `pix`. |
| `kobana_status` | `string` | não | Charge mirror, commercial axis (`opened`, `paid`, `overdue`, `canceled`, …). Free string — not validated against a fixed list, because the gateway owns the vocabulary. Accepts `[in]`. |
| `kobana_registration_status` | `string` | não | Charge mirror, registration axis (`pending`, `confirmed`, `skipped`, `rejected`, `failed`). Free string. This is how you list the charges that can actually be delivered: `?kobana_registration_status=confirmed`. Accepts `[in]`. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=due_date&sort_order=asc`. Valores: `amount`, `competence_date`, `created_at`, `due_date`, `payment_date`, `updated_at`. |
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
| `200` | Page of receivables plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## receivables create

Criar conta a receber

```text
kobana finance receivables create [OPÇÕES]
```

`POST /v1/receivables`

Creates a single receivable (or an installment/recurring series). Requires scope `finance/receivables` with access level `write`. `workspace_id` is always taken from the token. A person is required — send `person: {id}` or `person: {name}`. `{name}` (person / category / classification_center) is find-or-create (customer/credit/revenue polarity); sending both `id` and `name` inside the same nested object returns 400. `classification_centers[]` (rateio) and `tax_charges[]` are accepted. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Owning company UUID. Must belong to the tenant. |
| `description` | `string` | sim |  |
| `amount` | `number` | sim | A number in BRL (up to 13 integer digits, 2 decimals). Always positive. |
| `due_date` | `string (date-time)` | sim |  |
| `competence_date` | `string (date-time)` | não | Accounting period (ISO 8601). Defaults to `due_date` when omitted. |
| `type` | `string` | não | Defaults to `normal`. `recurring` requires `recurrence_period` and stores the series ahead up to the workspace's provisioning horizon (default 12 months) as rows sharing `group_id`; `installment` requires `total_installments` (2–120). A title that belongs to a series keeps its `type`; a standalone `normal` title may become `recurring`. Valores: `normal`, `recurring`, `installment`. |
| `person` | `object` | não | `{id}` for an existing person, or `{name}` to find-or-create one with `customer` polarity, scoped to (workspace, company). Never both. |
| `category` | `object` | não | `{id}` for an existing category, or `{name}` to find-or-create one with `credit` polarity. Never both. On update, sent without `categories` it replaces a category rateio with this single category. |
| `classification_center` | `object` | não | `{id}` for an existing classification center, or `{name}` to find-or-create one with `revenue` polarity — matches a center typed `revenue` or `both`; creates as `revenue` when no match. Never both. |
| `payment_method_kind` | `string` | não | Valores: `bank_billet`, `ted`, `pix`, `credit_card`, `debit_card`, `cash`, `check`, `direct_debit`, `other`, `none`, `null`. |
| `financial_account_id` | `string (uuid)` | não | Optional pre-set receipt account. Must belong to the tenant and match `company_id`. |
| `expected_account_id` | `string (uuid)` | não | Projected receipt account (forecast). Independent of `financial_account_id`. |
| `document` | `string` | não | Free-form reference of the source document (e.g. `NF-12345`, `BOL-001`); what the `search` / `documentContains` filters match. Distinct from `document_number`. |
| `document_date` | `string (date-time)` | não |  |
| `document_number` | `string` | não | Structured document number — the bare identifier (e.g. `12345`). Mirrors the "Nº de documento" field in the app UI; the canonical field for manually-entered document numbers. Distinct from the free-form `document`. |
| `notes` | `string` | não |  |
| `ir_relevant` | `boolean` | não |  |
| `recurrence_period` | `string` | não | Required for `type=recurring` and rejected for any other type (`weekly` \| `biweekly` \| `monthly` \| `quarterly` \| `yearly`). |
| `total_installments` | `integer` | não | Required for `type=installment` and rejected for any other type (2–120). A title that belongs to a series keeps its `type` and `total_installments`. |
| `classification_centers` | `array<object>` | não | Rateio. Replace-all set; Σ validated against `amount` (0.01 tolerance). |
| `categories` | `array<object>` | não | Category rateio. Replace-all set; an empty array clears it (and `category`); omit to leave it alone. Effective amounts must sum to `amount` (tolerance: the larger of 0.01 and 0.005 per row). When present, `category` is ignored and becomes rows[0]; every category must be of type `credit` (422 otherwise). The cents a percentage leaves go to the last slice sent without an `amount`. |
| `custom_classifications` | `array<object>` | não | Custom-classifier assignments. Replace-all set; one value per dimension for native single-select. Replicated onto every installment. |
| `tax_charges` | `array<object>` | não | Withholding/tax charges. Replace-all set. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.receivables.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Receivable created. The `Location` header points at the new row; the body is the full detail shape. `X-Idempotent-Replay: true` marks a replayed response. |
| `400` | Malformed JSON, or `id` and `name` sent together inside the same nested entity reference. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or domain violation: missing person, company / account not in tenant, account does not match company, or rateio Σ ≠ amount. |

## receivables get

Obter conta a receber por ID

```text
kobana finance receivables get [ID] [OPÇÕES]
```

`GET /v1/receivables/{id}`

Mirror of the payable detail — denorm names + counts + `nominal_amount`, plus the expanded `classification_center_allocations[]` / `category_allocations[]` / `tax_charges[]` arrays and the `reconciliation` group (`null` when not reconciled). Returns the same shape as the rows from the list endpoint. Requires scope `finance/receivables` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish. The detail's `charge` additionally carries what is handed to the payer when a charge was issued — `url`, `barcode`, `digitable_line`, `pix_qrcode` — while the list row's `charge` keeps only the state. All of them are `null` until the charge exists, and the Pix payload stays `null` until `state` reaches `registered`.

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
| `200` | Get a receivable by id |
| `404` | Receivable not found within the tenant. |

## receivables update

Atualizar ou liquidar conta a receber

```text
kobana finance receivables update [ID] [OPÇÕES]
```

`PATCH /v1/receivables/{id}`

Partial update. Only fields present in the body are written; unknown / response-only fields are silently dropped. Requires scope `finance/receivables` with access level `write`. **Settle (receive)**: send `status:"received"` with `payment_date` and `financial_account_id` (or a previously-set account) — an auto credit transaction is generated; the receive-time `interest_amount` / `discount_amount` / `ticket_amount` are persisted. Set `status` back to `pending` to revert, or `cancelled` to cancel. `draft` is never accepted. Settling generates a receipt transaction, so pass an `X-Idempotency-Key` header on retries — without it a retried settle is a second movement in the ledger.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `description` | `string` | não |  |
| `amount` | `number` | não | A number in BRL (up to 13 integer digits, 2 decimals). Always positive. |
| `due_date` | `string (date-time)` | não |  |
| `competence_date` | `string (date-time)` | não | Accounting period (ISO 8601). Defaults to `due_date` when omitted. |
| `type` | `string` | não | Defaults to `normal`. `recurring` requires `recurrence_period` and stores the series ahead up to the workspace's provisioning horizon (default 12 months) as rows sharing `group_id`; `installment` requires `total_installments` (2–120). A title that belongs to a series keeps its `type`; a standalone `normal` title may become `recurring`. Valores: `normal`, `recurring`, `installment`. |
| `person` | `object` | não | `{id}` for an existing person, or `{name}` to find-or-create one with `customer` polarity, scoped to (workspace, company). Never both. |
| `category` | `object` | não | `{id}` for an existing category, or `{name}` to find-or-create one with `credit` polarity. Never both. On update, sent without `categories` it replaces a category rateio with this single category. |
| `classification_center` | `object` | não | `{id}` for an existing classification center, or `{name}` to find-or-create one with `revenue` polarity — matches a center typed `revenue` or `both`; creates as `revenue` when no match. Never both. |
| `payment_method_kind` | `string` | não | Valores: `bank_billet`, `ted`, `pix`, `credit_card`, `debit_card`, `cash`, `check`, `direct_debit`, `other`, `none`, `null`. |
| `financial_account_id` | `string (uuid)` | não | Optional pre-set receipt account. Must belong to the tenant and match `company_id`. |
| `expected_account_id` | `string (uuid)` | não | Projected receipt account (forecast). Independent of `financial_account_id`. |
| `document` | `string` | não | Free-form reference of the source document (e.g. `NF-12345`, `BOL-001`); what the `search` / `documentContains` filters match. Distinct from `document_number`. |
| `document_date` | `string (date-time)` | não |  |
| `document_number` | `string` | não | Structured document number — the bare identifier (e.g. `12345`). Mirrors the "Nº de documento" field in the app UI; the canonical field for manually-entered document numbers. Distinct from the free-form `document`. |
| `notes` | `string` | não |  |
| `ir_relevant` | `boolean` | não |  |
| `recurrence_period` | `string` | não | Required for `type=recurring` and rejected for any other type (`weekly` \| `biweekly` \| `monthly` \| `quarterly` \| `yearly`). |
| `total_installments` | `integer` | não | Required for `type=installment` and rejected for any other type (2–120). A title that belongs to a series keeps its `type` and `total_installments`. |
| `classification_centers` | `array<object>` | não | Rateio. Replace-all set; Σ validated against `amount` (0.01 tolerance). |
| `categories` | `array<object>` | não | Category rateio. Replace-all set; an empty array clears it (and `category`); omit to leave it alone. Effective amounts must sum to `amount` (tolerance: the larger of 0.01 and 0.005 per row). When present, `category` is ignored and becomes rows[0]; every category must be of type `credit` (422 otherwise). The cents a percentage leaves go to the last slice sent without an `amount`. |
| `custom_classifications` | `array<object>` | não | Custom-classifier assignments. Replace-all set; one value per dimension for native single-select. Replicated onto every installment. |
| `tax_charges` | `array<object>` | não | Withholding/tax charges. Replace-all set. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `status` | `string` | não | Settle (`received`), revert (`pending`) or cancel (`cancelled`). `draft` is never accepted. Valores: `pending`, `received`, `cancelled`. |
| `payment_date` | `string (date-time)` | não | Receipt date (ISO 8601). Required when moving to `status=received`. |
| `interest_amount` | `number` | não | Late-payment interest collected at receive time (BRL), in BRL. |
| `discount_amount` | `number` | não | Discount granted at receive time (BRL), in BRL. |
| `ticket_amount` | `number` | não | "Valor liquidado" — the actual amount that hit the bank (BRL), in BRL. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.receivables.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated receivable (full detail shape). |
| `400` | Malformed JSON, or `id` and `name` sent together inside the same nested entity reference. |
| `404` | Receivable not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or domain violation: receiving without an account, FK out of scope, or rateio Σ ≠ amount. |

## receivables delete

Excluir conta a receber

```text
kobana finance receivables delete [ID] [OPÇÕES]
```

`DELETE /v1/receivables/{id}`

Hard-deletes the receivable. Requires scope `finance/receivables` with access level `delete`. Dissolves any reconciliation and removes the auto-created receipt transaction (manually-linked transactions are preserved). Calling DELETE twice returns 200 then 404 (Stripe-style). Single-row — series-aware delete is not yet exposed.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Receivable deleted. |
| `404` | Receivable not found within the tenant. |
