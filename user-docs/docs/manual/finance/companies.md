---
id: companies
title: "kobana finance companies"
sidebar_label: "companies"
---

# kobana finance companies


Comandos de `kobana finance companies`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`companies list`](#companies-list) | `GET /v1/companies` | Listar empresas |
| [`companies create`](#companies-create) | `POST /v1/companies` | Criar empresa |
| [`companies get`](#companies-get) | `GET /v1/companies/{id}` | Obter empresa por ID |
| [`companies update`](#companies-update) | `PATCH /v1/companies/{id}` | Atualizar empresa |
| [`companies delete`](#companies-delete) | `DELETE /v1/companies/{id}` | Excluir empresa |

## companies list

Listar empresas

```text
kobana finance companies list [OPÇÕES]
```

`GET /v1/companies`

Returns a cursor-paginated page of companies for the workspace tied to the token's `sub` claim. Requires scope `finance/companies` with access level `read`. Catalogal entity — defaults to alphabetical sort by `name`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `active` | `string` | não | Filter by active flag. `true` returns only active; omit to return both. Valores: `true`, `false`. |
| `type` | `string` | não | `matriz` (head office) or `filial` (branch). Accepts `[in]=matriz,filial`. Valores: `matriz`, `filial`. |
| `cnpj` | `string` | não | Filter by CNPJ (14-digit numeric string, no formatting). Accepts `[in]=cnpj1,cnpj2`. |
| `name` | `string` | não | Filter by exact name. Accepts `[in]`. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `created_at`, `name`. |
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
| `200` | Page of companies plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## companies create

Criar empresa

```text
kobana finance companies create [OPÇÕES]
```

`POST /v1/companies`

Creates a company. Requires scope `finance/companies` with access level `write`. **`cnpj` is required and validated** with the standard BR check-digit algorithm — a missing, invalid, or document-less value returns 422 (there is no pessoa-física / document-less company in this API; a migrating client whose source row has no CNPJ must supply one). The CNPJ must be unique among active companies in the workspace (duplicate → 409). `type` (matriz/filial) is derived server-side from the CNPJ branch digits and is never read from the request. `active` starts `true`. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Legal name as registered. |
| `cnpj` | `string` | sim | CNPJ — 14 characters. Formatting (dots / slash / dash) is accepted and stripped; letters are uppercased. Supports both numeric-only (legacy) and alphanumeric (new RFB format) CNPJs. Validated with the standard BR check-digit algorithm. **Required** — a missing, invalid, or document-less value returns 422. Must be unique among active companies in the workspace (duplicate → 409). `type` (matriz/filial) is derived server-side. |
| `trade_name` | `string` | não |  |
| `municipal_registration` | `string` | não |  |
| `tax_regime` | `string` | não | Valores: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`. |
| `address` | `object` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.companies.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Company created. The `Location` header points at the new row; `type` is derived from the CNPJ. |
| `400` | Malformed JSON body. |
| `409` | A company with the same CNPJ already exists (active), or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, or a missing / invalid / document-less `cnpj`. |

## companies get

Obter empresa por ID

```text
kobana finance companies get [ID] [OPÇÕES]
```

`GET /v1/companies/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/companies` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a company by id |
| `404` | Company not found within the tenant. |

## companies update

Atualizar empresa

```text
kobana finance companies update [ID] [OPÇÕES]
```

`PATCH /v1/companies/{id}`

Partial update of `name` and `active`. Requires scope `finance/companies` with access level `write`. The CNPJ (and the `type` it derives) is immutable via the public API — send `active: false` to soft-deactivate.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `active` | `boolean` | não | Set `false` to soft-deactivate the company (existing references are preserved). |
| `trade_name` | `string` | não |  |
| `municipal_registration` | `string` | não |  |
| `tax_regime` | `string` | não | Valores: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`. |
| `address` | `object` | não | Replaces the fiscal address wholesale. `null` clears it; omitting it leaves it untouched. It is never merged field by field — a partial merge would leave a company registered under half of one address and half of another. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.companies.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated company. |
| `400` | Malformed JSON body. |
| `404` | Company not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## companies delete

Excluir empresa

```text
kobana finance companies delete [ID] [OPÇÕES]
```

`DELETE /v1/companies/{id}`

Hard-deletes a company. Requires scope `finance/companies` with access level `delete`. Blocked with 409 (`details` carries the per-table counts `{ transactions, payables, receivables, people }`) when the company still has financial records or active people — remove or reassign those first, or soft-deactivate via `PATCH { active: false }`. Config tables (categories, classification centers, financial accounts, soft-deleted people) cascade automatically. Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Company deleted. |
| `404` | Company not found within the tenant. |
| `409` | The company has financial records or active people (`details` carries the counts). |
