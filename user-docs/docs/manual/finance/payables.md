---
id: payables
title: "kobana finance payables"
sidebar_label: "payables"
---

# kobana finance payables


Comandos de `kobana finance payables`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`payables aggregates`](#payables-aggregates) | `GET /v1/payables/aggregates` | Somar totais de contas a pagar |
| [`payables ids`](#payables-ids) | `GET /v1/payables/ids` | Listar IDs de contas a pagar |
| [`payables list`](#payables-list) | `GET /v1/payables` | Listar contas a pagar |
| [`payables create`](#payables-create) | `POST /v1/payables` | Criar conta a pagar |
| [`payables get`](#payables-get) | `GET /v1/payables/{id}` | Obter conta a pagar por ID |
| [`payables update`](#payables-update) | `PATCH /v1/payables/{id}` | Atualizar ou liquidar conta a pagar |
| [`payables delete`](#payables-delete) | `DELETE /v1/payables/{id}` | Excluir conta a pagar |
| [`payables cancel-dispatch`](#payables-cancel-dispatch) | `POST /v1/payables/{id}/cancel-dispatch` | Cancelar o envio ao banco |
| [`payables send-to-bank`](#payables-send-to-bank) | `POST /v1/payables/{id}/send-to-bank` | Enviar conta a pagar ao banco |

## payables aggregates

Somar totais de contas a pagar

```text
kobana finance payables aggregates [OPÇÕES]
```

`GET /v1/payables/aggregates`

Sums payables for the workspace tied to the token: `count`, `total`, `pending`, `paid`, `overdue_count`. Requires scope `finance/payables` with access level `read`. Filter by `company_id` and a period; omit all filters for all-time.

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
| `200` | Payable totals for the scope. |
| `400` | Malformed query (unknown param, bad pairing, invalid date/uuid). |

## payables ids

Listar IDs de contas a pagar

```text
kobana finance payables ids [OPÇÕES]
```

`GET /v1/payables/ids`

Every payable id the token can see. Requires scope `finance/payables` with access level `read`. Pairs with `?updated_at[gte]=` on the listing. The delta reports what changed; this reports what still exists. A delta can never report a disappearance — the row it would have to describe is gone — so an id that stops appearing here is the only signal that something left. Diff this set against your mirror and drop what is missing. Takes no filters: the sweep must cover exactly what the listing shows, and a filter applied here but not to your mirror would read as mass deletion. Note that deletion is not the only exit: a payable whose payment approval is rejected leaves the visible set while its row stays put, and it disappears from this sweep exactly as a deleted one would.

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
| `200` | Live payable ids. |
| `400` | Malformed query. This endpoint takes no filters or sort — passing one is a 400. |

## payables list

Listar contas a pagar

```text
kobana finance payables list [OPÇÕES]
```

`GET /v1/payables`

Returns a cursor-paginated page of payables (accounts payable) for the workspace tied to the token's `sub` claim. Each row includes nested entity references (`person`, `category`, `classification_center`, each `{id, name}`) and aggregate counts (`tax_charges_count`, `classification_center_allocations_count`, `category_allocations_count`, `attachments_count`) — full child collections live in the per-resource detail endpoints. Each row also carries the settlement amounts (`interest_amount`, `discount_amount`, `ticket_amount`) and a derived `nominal_amount = amount + interest - discount`. Status is the raw DB enum (`pending` / `paid` / `cancelled`); derive overdue client-side via `status === "pending" AND due_date < today`. Default sort is `due_date` ascending. Payables awaiting human approval (see the create endpoint) are **omitted** from this list until an operator approves them in the app; a bill is still retrievable by id via `GET /payables/{id}`. To **list the approval queue** instead, filter it explicitly — `?payment_approval=pending_approval` (read-only; there is no approve/reject action on this API). Each row carries its `payment_approval` state. Requires scope `finance/payables` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]`. |
| `status` | `string` | não | Raw status. To find overdue payables, combine `status=pending&due_date[lt]=<today ISO>`. Accepts `[in]=pending,cancelled`. Valores: `pending`, `paid`, `cancelled`. |
| `type` | `string` | não | Billing kind. Accepts `[in]`. Valores: `normal`, `recurring`, `installment`. |
| `person_id` | `string` | não | Filter by person UUID. Accepts `[in]`. |
| `category_id` | `string` | não | Filter by category UUID. Accepts `[in]`. |
| `classification_center_id` | `string` | não | Filter by primary classification-center UUID (not the rateio). Accepts `[in]`. |
| `financial_account_id` | `string` | não | Filter by settlement account UUID. Accepts `[in]`. |
| `payment_method_kind` | `string` | não | Payment method enum (`utility`/`tribute` are payable-only). Accepts `[in]`. Valores: `bank_billet`, `utility`, `tribute`, `ted`, `pix`, `credit_card`, `debit_card`, `cash`, `check`, `direct_debit`, `other`, `none`. |
| `payment_approval` | `string` | não | Read-only view of the human payment-approval gate. By default the list hides gated bills; filter this explicitly to **list the approval queue** — e.g. `?payment_approval=pending_approval`. Accepts `[in]`. There is no approve/reject action on this API; that stays a human action in the app. Valores: `pending_approval`, `partially_approved`, `creation_approved`, `creation_rejected`. |
| `due_date` | `string` | não | ISO date filter. Range operators: `[gte]`, `[lte]`, `[gt]`, `[lt]`. Example: `?due_date[gte]=2026-01-01&due_date[lte]=2026-12-31`. |
| `payment_date` | `string` | não | ISO date filter on the actual payment date. Range operators supported. |
| `competence_date` | `string` | não | ISO date filter on the accounting period. Range operators supported. |
| `updated_at` | `string` | não | ISO timestamp filter on the last write. Range operators supported. This is the incremental-sync window: poll `?updated_at[gte]=<end of your last poll>&sort_by=updated_at` to fetch only what changed. Deletions do NOT appear here — a deleted payable has no row to return; reconcile them against `GET /v1/payables/ids`. |
| `amount` | `string` | não | Decimal filter on amount (BRL). Range operators supported. |
| `document_number` | `string` | não | Filter by exact document number. Accepts `[in]`. |
| `ir_relevant` | `string` | não | Filter by tax-relevance flag. Valores: `true`, `false`. |
| `group_id` | `string` | não | Filter siblings of a recurring/installment series. |
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
| `200` | Page of payables plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## payables create

Criar conta a pagar

```text
kobana finance payables create [OPÇÕES]
```

`POST /v1/payables`

Creates a single payable (or an installment/recurring series). Requires scope `finance/payables` with access level `write`. `workspace_id` is always taken from the token. A person is required — send `person: {id}` or `person: {name}`. `{name}` (person / category / classification_center) is find-or-create; sending both `id` and `name` inside the same nested object returns 400. `classification_centers[]` (rateio) and `tax_charges[]` are accepted; the rateio Σ is validated against `amount`. When payment approval is enabled for the workspace (a per-workspace setting, on by default), the created payable enters an **approval queue**: it is returned here (with its id) but is **hidden from `GET /payables` until an operator approves it in the app** — fetch it by id meanwhile. With the setting off, the payable appears on `GET /payables` immediately. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Owning company UUID. Must belong to the tenant. |
| `description` | `string` | sim |  |
| `amount` | `number` | sim | A number in BRL (up to 13 integer digits, 2 decimals). Always positive — cost polarity is implicit. |
| `due_date` | `string (date-time)` | sim | When this payable is due (ISO 8601). |
| `competence_date` | `string (date-time)` | não | Accounting period (ISO 8601). Defaults to `due_date` when omitted. |
| `type` | `string` | não | Defaults to `normal`. `recurring` requires `recurrence_period` and stores the series ahead up to the workspace's provisioning horizon (default 12 months) as rows sharing `group_id`; `installment` requires `total_installments` (2–120). A title that belongs to a series keeps its `type`; a standalone `normal` title may become `recurring`. Valores: `normal`, `recurring`, `installment`. |
| `person` | `object` | não | `{id}` for an existing person, or `{name}` to find-or-create one with `supplier` polarity, scoped to (workspace, company). Never both. |
| `category` | `object` | não | `{id}` for an existing category, or `{name}` to find-or-create one with `debit` polarity. Never both. On update, sent without `categories` it replaces a category rateio with this single category. |
| `classification_center` | `object` | não | `{id}` for an existing classification center, or `{name}` to find-or-create one with `cost` polarity — matches a center typed `cost` or `both`; creates as `cost` when no match. Never both. |
| `payment_method_kind` | `string` | não | Valores: `bank_billet`, `utility`, `tribute`, `ted`, `pix`, `credit_card`, `debit_card`, `cash`, `check`, `direct_debit`, `other`, `none`, `null`. |
| `operation_kind` | `string` | não | Valores: `payment`, `transfer`, `null`. |
| `financial_account_id` | `string (uuid)` | não | Optional pre-set settlement account. Must belong to the tenant and match `company_id`. |
| `expected_account_id` | `string (uuid)` | não | Projected settlement account (forecast). Independent of `financial_account_id`. |
| `document` | `string` | não | Free-form reference of the source document (e.g. `NF-12345`, `BOL-001`); what the `search` / `documentContains` filters match. Distinct from `document_number`. |
| `document_date` | `string (date-time)` | não |  |
| `document_number` | `string` | não | Structured document number — the bare identifier (e.g. `12345`). Mirrors the "Nº de documento" field in the app UI; the canonical field for manually-entered document numbers. Distinct from the free-form `document`. |
| `notes` | `string` | não |  |
| `ir_relevant` | `boolean` | não |  |
| `recurrence_period` | `string` | não | Required for `type=recurring` and rejected for any other type (`weekly` \| `biweekly` \| `monthly` \| `quarterly` \| `yearly`). |
| `total_installments` | `integer` | não | Required for `type=installment` and rejected for any other type — number of parcelas (2–120). A title that belongs to a series keeps its `type` and `total_installments`. |
| `classification_centers` | `array<object>` | não | Rateio. Replace-all set; the server validates Σ against `amount` (0.01 tolerance). Spread across installments. |
| `categories` | `array<object>` | não | Category rateio. Replace-all set; an empty array clears it (and `category`); omit to leave it alone. Effective amounts must sum to `amount` (tolerance: the larger of 0.01 and 0.005 per row). When present, `category` is ignored and becomes rows[0]; every category must be of type `debit` (422 otherwise). The cents a percentage leaves go to the last slice sent without an `amount`. |
| `custom_classifications` | `array<object>` | não | Custom-classifier assignments. Replace-all set; one value per dimension for native single-select. Replicated onto every installment. |
| `tax_charges` | `array<object>` | não | Withholding/tax charges. Replace-all set. Attached to the first record of a series. |
| `payment_source` | `string` | não | Origin of the payable shown on the approval screen: `ai_agent` when an AI agent created it, `manual` otherwise. Defaults to `manual`. Valores: `manual`, `ai_agent`. |
| `barcode` | `string` | não |  |
| `bank` | `object` | não |  |
| `bank_account` | `object` | não |  |
| `pix` | `object` | não |  |
| `transfer_purpose` | `string` | não |  |
| `judicial_deposit_number` | `string` | não | Judicial deposit identifier (digits only, up to 20). Set with `operation_kind = transfer` + `payment_method_kind = ted`; `bank.code` is the depositary bank and no `bank_account` is needed. |
| `tax_kind` | `string` | não | Tribute kind. Set with `payment_method_kind = tribute`; the barcode goes in `barcode`. Valores: `itbi`, `icms`, `iss`, `iptu`, `fgts`, `dare`, `darf`, `gru`, `gps`, `null`. |
| `tax_data` | `object` | não |  |
| `beneficiary` | `object` | não |  |
| `scheduled_to` | `string (date-time)` | não | Scheduled payment date (ISO 8601). Suggested default is `due_date`. |
| `receipt_emails` | `array<object>` | não | Replace-all list of receipt (comprovante) recipients for this payable — they receive the receipt as an attachment once one is attached. Omit to leave the current list untouched; send `[]` or `null` to clear it. Additional to the workspace-wide recipient list, never a replacement for it. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.payables.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Payable created. The `Location` header points at the new row. The body is the full detail shape (arrays + reconciliation). `X-Idempotent-Replay: true` marks a replayed response. |
| `400` | Malformed JSON, or `id` and `name` sent together inside the same nested entity reference. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or domain violation: missing person, company / account not in tenant, account does not match company, a credit card's account (`reason: card_account_not_allowed` — launch it on the card), a Pix transfer whose `beneficiary.document` is not the `person`'s (`reason: pix_beneficiary_not_person`), or rateio Σ ≠ amount. |

## payables get

Obter conta a pagar por ID

```text
kobana finance payables get [ID] [OPÇÕES]
```

`GET /v1/payables/{id}`

List-row shape plus the expanded child arrays the list only counts: `classification_center_allocations[]` and `category_allocations[]` (rateio) and `tax_charges[]`, plus the `reconciliation` group (`null` when not reconciled). Returns the same shape as the rows from the list endpoint. Requires scope `finance/payables` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a payable by id |
| `404` | Payable not found within the tenant. |

## payables update

Atualizar ou liquidar conta a pagar

```text
kobana finance payables update [ID] [OPÇÕES]
```

`PATCH /v1/payables/{id}`

Partial update. Only fields present in the body are written; unknown / response-only fields (`id`, `created_at`, `transaction_id`, denorm names, counts, `is_reconciled`) are silently dropped. Requires scope `finance/payables` with access level `write`. **Settle**: send `status:"paid"` with `payment_date` and `financial_account_id` (or rely on a previously-set account) — a debit payment transaction is generated automatically. The settlement amounts (`interest_amount` / `discount_amount` / `ticket_amount`) are persisted on the row. Set `status` back to `pending` to revert, or `cancelled` to cancel. `draft` is never accepted. Settling generates a payment transaction, so pass an `X-Idempotency-Key` header on retries — without it a retried settle is a second movement in the ledger.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `description` | `string` | não |  |
| `amount` | `number` | não | A number in BRL (up to 13 integer digits, 2 decimals). Always positive — cost polarity is implicit. |
| `due_date` | `string (date-time)` | não | When this payable is due (ISO 8601). |
| `competence_date` | `string (date-time)` | não | Accounting period (ISO 8601). Defaults to `due_date` when omitted. |
| `type` | `string` | não | Defaults to `normal`. `recurring` requires `recurrence_period` and stores the series ahead up to the workspace's provisioning horizon (default 12 months) as rows sharing `group_id`; `installment` requires `total_installments` (2–120). A title that belongs to a series keeps its `type`; a standalone `normal` title may become `recurring`. Valores: `normal`, `recurring`, `installment`. |
| `person` | `object` | não | `{id}` for an existing person, or `{name}` to find-or-create one with `supplier` polarity, scoped to (workspace, company). Never both. |
| `category` | `object` | não | `{id}` for an existing category, or `{name}` to find-or-create one with `debit` polarity. Never both. On update, sent without `categories` it replaces a category rateio with this single category. |
| `classification_center` | `object` | não | `{id}` for an existing classification center, or `{name}` to find-or-create one with `cost` polarity — matches a center typed `cost` or `both`; creates as `cost` when no match. Never both. |
| `payment_method_kind` | `string` | não | Valores: `bank_billet`, `utility`, `tribute`, `ted`, `pix`, `credit_card`, `debit_card`, `cash`, `check`, `direct_debit`, `other`, `none`, `null`. |
| `operation_kind` | `string` | não | Valores: `payment`, `transfer`, `null`. |
| `financial_account_id` | `string (uuid)` | não | Optional pre-set settlement account. Must belong to the tenant and match `company_id`. |
| `expected_account_id` | `string (uuid)` | não | Projected settlement account (forecast). Independent of `financial_account_id`. |
| `document` | `string` | não | Free-form reference of the source document (e.g. `NF-12345`, `BOL-001`); what the `search` / `documentContains` filters match. Distinct from `document_number`. |
| `document_date` | `string (date-time)` | não |  |
| `document_number` | `string` | não | Structured document number — the bare identifier (e.g. `12345`). Mirrors the "Nº de documento" field in the app UI; the canonical field for manually-entered document numbers. Distinct from the free-form `document`. |
| `notes` | `string` | não |  |
| `ir_relevant` | `boolean` | não |  |
| `recurrence_period` | `string` | não | Required for `type=recurring` and rejected for any other type (`weekly` \| `biweekly` \| `monthly` \| `quarterly` \| `yearly`). |
| `total_installments` | `integer` | não | Required for `type=installment` and rejected for any other type — number of parcelas (2–120). A title that belongs to a series keeps its `type` and `total_installments`. |
| `classification_centers` | `array<object>` | não | Rateio. Replace-all set; the server validates Σ against `amount` (0.01 tolerance). Spread across installments. |
| `categories` | `array<object>` | não | Category rateio. Replace-all set; an empty array clears it (and `category`); omit to leave it alone. Effective amounts must sum to `amount` (tolerance: the larger of 0.01 and 0.005 per row). When present, `category` is ignored and becomes rows[0]; every category must be of type `debit` (422 otherwise). The cents a percentage leaves go to the last slice sent without an `amount`. |
| `custom_classifications` | `array<object>` | não | Custom-classifier assignments. Replace-all set; one value per dimension for native single-select. Replicated onto every installment. |
| `tax_charges` | `array<object>` | não | Withholding/tax charges. Replace-all set. Attached to the first record of a series. |
| `barcode` | `string` | não |  |
| `bank` | `object` | não |  |
| `bank_account` | `object` | não |  |
| `pix` | `object` | não |  |
| `transfer_purpose` | `string` | não |  |
| `judicial_deposit_number` | `string` | não | Judicial deposit identifier (digits only, up to 20). Set with `operation_kind = transfer` + `payment_method_kind = ted`; `bank.code` is the depositary bank and no `bank_account` is needed. |
| `tax_kind` | `string` | não | Tribute kind. Set with `payment_method_kind = tribute`; the barcode goes in `barcode`. Valores: `itbi`, `icms`, `iss`, `iptu`, `fgts`, `dare`, `darf`, `gru`, `gps`, `null`. |
| `tax_data` | `object` | não |  |
| `beneficiary` | `object` | não |  |
| `scheduled_to` | `string (date-time)` | não | Scheduled payment date (ISO 8601). Suggested default is `due_date`. |
| `receipt_emails` | `array<object>` | não | Replace-all list of receipt (comprovante) recipients for this payable — they receive the receipt as an attachment once one is attached. Omit to leave the current list untouched; send `[]` or `null` to clear it. Additional to the workspace-wide recipient list, never a replacement for it. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `status` | `string` | não | Settle (`paid`), revert (`pending`) or cancel (`cancelled`). `draft` is never accepted. Valores: `pending`, `paid`, `cancelled`. |
| `payment_date` | `string (date-time)` | não | Settlement date (ISO 8601). Required when moving to `status=paid`. |
| `interest_amount` | `number` | não | Late-payment interest paid at settlement (BRL), in BRL. |
| `discount_amount` | `number` | não | Discount obtained at settlement (BRL), in BRL. |
| `ticket_amount` | `number` | não | "Valor liquidado" — the actual amount that left the bank (BRL), in BRL. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.payables.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated payable (full detail shape). |
| `400` | Malformed JSON, or `id` and `name` sent together inside the same nested entity reference. |
| `404` | Payable not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or domain violation: settling without an account, FK out of scope, rateio Σ ≠ amount, a credit card's account (`reason: card_account_not_allowed`), a Pix transfer whose `beneficiary.document` is not the `person`'s (`reason: pix_beneficiary_not_person`), or a new `amount` or `status: cancelled` on the payable of a card statement (`reason: card_statement_managed` — it follows the statement's review and payment). |

## payables delete

Excluir conta a pagar

```text
kobana finance payables delete [ID] [OPÇÕES]
```

`DELETE /v1/payables/{id}`

Hard-deletes the payable. Requires scope `finance/payables` with access level `delete`. Dissolves any reconciliation the payable is part of and removes the auto-created payment transaction (manually-linked transactions are preserved). Calling DELETE twice on the same id returns 200 then 404 (Stripe-style). Deletes a single row — series-aware delete is not yet exposed. A payable already sent to the bank cannot be deleted (422) while its payment is in flight or after it was paid; it can again once the bank rejected or the payment was canceled.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Payable deleted. |
| `404` | Payable not found within the tenant. |
| `422` | The payable was already sent to the bank (payment in flight or paid), or it is the payable of a card statement (`reason: card_statement_managed`), and cannot be deleted. |

## payables cancel-dispatch

Cancelar o envio ao banco

```text
kobana finance payables cancel-dispatch [ID] [OPÇÕES]
```

`POST /v1/payables/{id}/cancel-dispatch`

Undoes a payable's bank dispatch on the Kobana side and mirrors the payment as `canceled` (the `kobana_*` mirror is kept, not cleared — the bill returns to the manual rail because `canceled` is terminal). Requires scope `finance/payable_dispatches` with access level `write` and a **required** `X-Idempotency-Key`. A bill that was never dispatched, or whose execution already reached a terminal state (`confirmed` / `canceled` / …), returns 422. Returns the refreshed payable detail.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Dispatch cancelled. Body is the refreshed payable detail (`kobana_status` = `canceled`). |
| `400` | Missing / blank `X-Idempotency-Key`. |
| `404` | Payable not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Not cancellable: the bill was never dispatched, or its execution already reached a terminal state. |

## payables send-to-bank

Enviar conta a pagar ao banco

```text
kobana finance payables send-to-bank [ID] [OPÇÕES]
```

`POST /v1/payables/{id}/send-to-bank`

Dispatches an approved payable to the bank through the Kobana gateway (creates the payment, then envelopes and approves it on the gateway side). Requires scope `finance/payable_dispatches` with access level `write` — the money resource is deliberately **separate** from `finance/payables` (dispatching money is not editing a bill), while a token with no `finance/…` scope needs just the `write` access level. `X-Idempotency-Key` is **required**. Only an **official** bill dispatches — one whose `payment_approval` is `null` or `creation_approved`; a bill still awaiting approval (`pending_approval` / `partially_approved`) or `creation_rejected` returns 422. The payable must also carry the payment coordinates for its method and a Kobana-mirrored source account. Returns the refreshed payable detail with the execution mirror (`kobana_status`, `kobana_payment_uid`) populated. **Note:** the Kobana sandbox `example_bank` does not execute outbound payments, so the full `confirmed` cycle is only observable against an environment that actually settles.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Dispatched. Body is the refreshed payable detail with the Kobana mirror populated. |
| `400` | Missing / blank `X-Idempotency-Key`. |
| `404` | Payable not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Not dispatchable: bill not official (awaiting approval / rejected), missing payment coordinates, source account not mirrored in Kobana, or the Kobana integration is not connected. |
