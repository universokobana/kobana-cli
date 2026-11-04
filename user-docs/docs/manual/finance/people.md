---
id: people
title: "kobana finance people"
sidebar_label: "people"
---

# kobana finance people


Comandos de `kobana finance people`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`people ids`](#people-ids) | `GET /v1/people/ids` | Listar IDs de pessoas |
| [`people list`](#people-list) | `GET /v1/people` | Listar pessoas (clientes / fornecedores) |
| [`people create`](#people-create) | `POST /v1/people` | Criar pessoa (cliente / fornecedor) |
| [`people get`](#people-get) | `GET /v1/people/{id}` | Obter pessoa por ID |
| [`people update`](#people-update) | `PATCH /v1/people/{id}` | Atualizar pessoa |
| [`people delete`](#people-delete) | `DELETE /v1/people/{id}` | Excluir pessoa |

## people ids

Listar IDs de pessoas

```text
kobana finance people ids [OPÇÕES]
```

`GET /v1/people/ids`

Every person id the token can see. Requires scope `finance/people` with access level `read`. Pairs with `?updated_at[gte]=` on the listing. The delta reports what changed; this reports what still exists. A delta can never report a disappearance — the row it would have to describe is gone — so an id that stops appearing here is the only signal that something left. Diff this set against your mirror and drop what is missing. Takes no filters: the sweep must cover exactly what the listing shows, and a filter applied here but not to your mirror would read as mass deletion. People are soft-deleted, so a removed person's row survives internally — but it is filtered out of every v1 read, including this one, which makes its absence here the deletion signal.

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
| `200` | Live person ids. |
| `400` | Malformed query. This endpoint takes no filters or sort — passing one is a 400. |

## people list

Listar pessoas (clientes / fornecedores)

```text
kobana finance people list [OPÇÕES]
```

`GET /v1/people`

Returns a cursor-paginated page of people for the workspace tied to the token's `sub` claim. Soft-deleted rows are never returned. Company-scoped within the workspace — `?company_id=…` narrows. Sensitive fields (bank/PIX info, addresses, custom_data, birthday) are intentionally excluded from this endpoint. Requires scope `finance/people` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `kind` | `string` | não | `natural` (pessoa física) or `juridical` (pessoa jurídica). Accepts `[in]=natural,juridical`. Valores: `natural`, `juridical`. |
| `is_customer` | `string` | não | Filter by `is_customer` flag. Valores: `true`, `false`. |
| `is_supplier` | `string` | não | Filter by `is_supplier` flag. Valores: `true`, `false`. |
| `is_bank_manager` | `string` | não | Filter by `is_bank_manager` flag. Valores: `true`, `false`. |
| `is_team` | `string` | não | Filter by `is_team` flag. Valores: `true`, `false`. |
| `active` | `string` | não | Filter by active flag. Valores: `true`, `false`. |
| `document_number` | `string` | não | Filter by CPF/CNPJ (numeric-only string, no formatting). Accepts `[in]=11111111111,22222222222` for batch lookup. |
| `name` | `string` | não | Filter by exact name. Accepts `[in]`. |
| `updated_at` | `string` | não | ISO timestamp filter on the last write. Range operators supported. This is the incremental-sync window: poll `?updated_at[gte]=<end of your last poll>&sort_by=updated_at` to fetch only what changed. Deletions do NOT appear here — a deleted person is filtered out of every v1 read; reconcile them against `GET /v1/people/ids`. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `created_at`, `name`, `updated_at`. |
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
| `200` | Page of people plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## people create

Criar pessoa (cliente / fornecedor)

```text
kobana finance people create [OPÇÕES]
```

`POST /v1/people`

Creates a person. Requires scope `finance/people` with access level `write`. `workspace_id` is always taken from the token. Set `is_customer` / `is_supplier` to make the row referenceable by receivables / payables. `document_number` (CPF 11 digits / CNPJ 14 digits) is validated and stored numeric-only; `document_type` is inferred from the length when omitted. Contact details (`emails` / `phones`), addresses, and bank/PIX coordinates are not yet writable via v1. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Owning company UUID. Must belong to the tenant. |
| `name` | `string` | sim |  |
| `legal_name` | `string` | não | Razão social (juridical persons). |
| `nickname` | `string` | não |  |
| `kind` | `string` | não | `natural` (pessoa física) or `juridical` (pessoa jurídica). Valores: `natural`, `juridical`, `null`. |
| `document_type` | `string` | não | Document type for `document_number`. Inferred from the number length when omitted. Valores: `cpf`, `cnpj`, `null`. |
| `document_number` | `string` | não | CPF (11 digits) or CNPJ (14 digits). Formatting is stripped; check digits are validated. |
| `is_customer` | `boolean` | não | Defaults to `false`. True ⇒ can receive a receivable. |
| `is_supplier` | `boolean` | não | Defaults to `false`. True ⇒ can receive a payable. |
| `is_bank_manager` | `boolean` | não | Defaults to `false`. |
| `is_team` | `boolean` | não | Defaults to `false`. |
| `active` | `boolean` | não | Defaults to `true`. |
| `notes` | `string` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.people.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Person created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | A person with the same `document_number` already exists for the company, or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, invalid CPF/CNPJ check digits, or `company_id` not in the tenant. |

## people get

Obter pessoa por ID

```text
kobana finance people get [ID] [OPÇÕES]
```

`GET /v1/people/{id}`

Soft-deleted rows return 404 — same `deleted_at IS NULL` filter as the listing. Returns the same shape as the rows from the list endpoint. Requires scope `finance/people` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a person by id |
| `404` | Person not found within the tenant (or soft-deleted). |

## people update

Atualizar pessoa

```text
kobana finance people update [ID] [OPÇÕES]
```

`PATCH /v1/people/{id}`

Partial update. Only fields present in the body are written; non-writable fields (`emails`, `phones`, bank/PIX, `id`, `workspace_id`, `company_id`, `created_at`) are dropped. Requires scope `finance/people` with access level `write`. Sending `null` for an optional string field is a no-op (clearing to null is not supported via v1). The CPF/CNPJ is re-validated only when `document_number` / `document_type` actually changes — re-sending the stored value verbatim (even a legacy/invalid one) is accepted so unrelated edits aren't blocked. A `document_type` sent on its own is rejected with 422 when it contradicts the stored number's digit count (e.g. tagging an 11-digit number as `cnpj`).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `legal_name` | `string` | não | Razão social (juridical persons). |
| `nickname` | `string` | não |  |
| `kind` | `string` | não | `natural` (pessoa física) or `juridical` (pessoa jurídica). Valores: `natural`, `juridical`, `null`. |
| `document_type` | `string` | não | Document type for `document_number`. Inferred from the number length when omitted. Valores: `cpf`, `cnpj`, `null`. |
| `document_number` | `string` | não | CPF (11 digits) or CNPJ (14 digits). Formatting is stripped; check digits are validated. |
| `is_customer` | `boolean` | não | Defaults to `false`. True ⇒ can receive a receivable. |
| `is_supplier` | `boolean` | não | Defaults to `false`. True ⇒ can receive a payable. |
| `is_bank_manager` | `boolean` | não | Defaults to `false`. |
| `is_team` | `boolean` | não | Defaults to `false`. |
| `active` | `boolean` | não | Defaults to `true`. |
| `notes` | `string` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.people.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated person. |
| `400` | Malformed JSON body. |
| `404` | Person not found within the tenant (or soft-deleted). |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, invalid CPF/CNPJ check digits when the document is changed, or a type-only change that contradicts the stored number's digit count. |

## people delete

Excluir pessoa

```text
kobana finance people delete [ID] [OPÇÕES]
```

`DELETE /v1/people/{id}`

Soft-deletes the person (sets `deleted_at`). Requires scope `finance/people` with access level `delete`. Existing payables / receivables / transactions referencing the person stay valid — the row just stops surfacing in listings and lookups. Calling DELETE twice returns 200 then 404 (Stripe-style). Refused with 409 while a classification rule still references the person (as match criterion or set action) — edit or remove the rule first.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Person soft-deleted. |
| `404` | Person not found within the tenant (or already deleted). |
| `409` | The person is still referenced by a classification rule (match or set) — edit/remove the rule before deleting. |
