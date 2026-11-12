---
id: receipts
title: "kobana finance receipts"
sidebar_label: "receipts"
---

# kobana finance receipts


Comandos de `kobana finance receipts`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`receipts list`](#receipts-list) | `GET /v1/receipts` | Listar comprovantes de pagamento |
| [`receipts get`](#receipts-get) | `GET /v1/receipts/{id}` | Obter comprovante de pagamento por ID |

## receipts list

Listar comprovantes de pagamento

```text
kobana finance receipts list [OPÇÕES]
```

`GET /v1/receipts`

Returns a cursor-paginated page of payment receipts (comprovantes) for the workspace tied to the token's `sub` claim — the proof that a payment left. A receipt is either **captured** (`source: "kobana"`, produced in the background when a payment dispatched to the bank was confirmed) or **uploaded** (`source: "upload"`, a file someone attached for a payment made outside the system). Only an uploaded one can be missing a `payable_id`. Default sort is `-paid_at`. Requires scope `finance/receipts` with access level `read`.

⚠️ `download_url` points at the **attachments** resource, so fetching the bytes needs a token that also reaches `finance/attachments` (or `finance/all` / `all`). And a row can legitimately have no binary at all: when the origin offered only a format we do not store, the receipt is `available` and reachable through `document_url` instead.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]`. |
| `payable_id` | `string` | não | Filter by the bill the receipt proves. Accepts `[in]`. |
| `financial_account_id` | `string` | não | Filter by the account the money left from. Accepts `[in]`. |
| `source` | `string` | não | `kobana` (captured automatically) or `upload` (sent by a person). Accepts `[in]`. Valores: `kobana`, `upload`. |
| `status` | `string` | não | Capture state. `?status=failed` is the queue of receipts a human should look at; `?status=available` is everything ready to fetch. Accepts `[in]`. Valores: `pending`, `available`, `failed`. |
| `payment_method_kind` | `string` | não | How the payment settled. Accepts `[in]`. Valores: `bank_billet`, `utility`, `tribute`, `ted`, `pix`, `credit_card`, `debit_card`, `cash`, `check`, `direct_debit`, `other`, `none`. |
| `external_uid` | `string` | não | Filter by the gateway payment `uid` the receipt was captured from — how you go from your own gateway record to the proof here. Accepts `[in]`. |
| `paid_at` | `string` | não | ISO date filter on the payment date. Range operators: `[gte]`, `[lte]`, `[gt]`, `[lt]`. |
| `created_at` | `string` | não | ISO timestamp filter on when the receipt row appeared. Range operators supported. |
| `updated_at` | `string` | não | ISO timestamp filter on the last write. Range operators supported. This is the incremental-sync window: poll `?updated_at[gte]=<end of your last poll>&sort_by=updated_at` to pick up captures that landed since. Deletions do NOT appear here. |
| `amount` | `string` | não | Decimal filter on the amount paid (BRL). Range operators supported. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=paid_at&sort_order=desc`. Valores: `amount`, `created_at`, `paid_at`, `updated_at`. |
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
| `200` | Page of receipts plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## receipts get

Obter comprovante de pagamento por ID

```text
kobana finance receipts get [ID] [OPÇÕES]
```

`GET /v1/receipts/{id}`

A single payment receipt. Returns the same shape as the rows from the list endpoint. Requires scope `finance/receipts` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish. The bytes are not served here — follow `download_url`, which lives on the attachments resource and needs `finance/attachments`.

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
| `200` | Get a payment receipt by id |
| `404` | Receipt not found within the tenant. |
