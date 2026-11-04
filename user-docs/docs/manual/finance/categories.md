---
id: categories
title: "kobana finance categories"
sidebar_label: "categories"
---

# kobana finance categories


Comandos de `kobana finance categories`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`categories list`](#categories-list) | `GET /v1/categories` | Listar categorias |
| [`categories create`](#categories-create) | `POST /v1/categories` | Criar categoria |
| [`categories get`](#categories-get) | `GET /v1/categories/{id}` | Obter categoria por ID |
| [`categories update`](#categories-update) | `PATCH /v1/categories/{id}` | Atualizar categoria |
| [`categories delete`](#categories-delete) | `DELETE /v1/categories/{id}` | Excluir categoria |

## categories list

Listar categorias

```text
kobana finance categories list [OPÇÕES]
```

`GET /v1/categories`

Returns a cursor-paginated page of categories for the workspace tied to the token's `sub` claim. Company-scoped within the workspace; categories form a tree via `parent_id` but are returned flat — build the tree client-side using `parent_id` and `level`. Requires scope `finance/categories` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `parent_id` | `string` | não | Filter by direct parent UUID. To list root-level categories, use `?level=1` instead — the URL grammar does not support `parent_id=null`. |
| `type` | `string` | não | `debit` (cost) or `credit` (revenue). Accepts `[in]=debit,credit`. Valores: `debit`, `credit`. |
| `level` | `integer` | não | Filter by depth. `1` returns root-level categories. Range operators supported: `[gte]`, `[lte]`. Example: `?level[lte]=2` for roots + their direct children. |
| `name` | `string` | não | Filter by exact name. Accepts `[in]`. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `created_at`, `level`, `name`. |
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
| `200` | Page of categories plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## categories create

Criar categoria

```text
kobana finance categories create [OPÇÕES]
```

`POST /v1/categories`

Creates a category. Requires scope `finance/categories` with access level `write`. `workspace_id` is taken from the token. Omit `parent_id` for a root (`level` 1); when set, the server derives `level = parent.level + 1` (max depth 3) and requires `type` to match the parent. The response is flat (`parent_id` + `level`). Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Owning company UUID. Must belong to the tenant. |
| `name` | `string` | sim |  |
| `type` | `string` | sim | `debit` (cost) or `credit` (revenue). A subcategory must match its parent's type. Valores: `debit`, `credit`. |
| `parent_id` | `string (uuid)` | não | Parent category UUID. Omit (or `null`) for a root (`level` 1). When set, the server computes `level = parent.level + 1` (max 3) and requires `type` to match the parent. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.categories.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Category created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; `company_id` or `parent_id` not in the tenant; subcategory `type` not matching the parent; or tree depth > 3. |

## categories get

Obter categoria por ID

```text
kobana finance categories get [ID] [OPÇÕES]
```

`GET /v1/categories/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/categories` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a category by id |
| `404` | Category not found within the tenant. |

## categories update

Atualizar categoria

```text
kobana finance categories update [ID] [OPÇÕES]
```

`PATCH /v1/categories/{id}`

Partial update of `name` and `type` only. Requires scope `finance/categories` with access level `write`. Re-parenting is not supported via v1. Changing a parent's `type` cascades to all descendants (internal behavior).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `type` | `string` | não | Changing a parent's type cascades to all descendants. Valores: `debit`, `credit`. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.categories.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated category. |
| `400` | Malformed JSON body. |
| `404` | Category not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## categories delete

Excluir categoria

```text
kobana finance categories delete [ID] [OPÇÕES]
```

`DELETE /v1/categories/{id}`

Deletes a category. Requires scope `finance/categories` with access level `delete`. Returns 422 (`reason: has_children`) while the category still has subcategories — delete the children first — and 422 (`reason: in_rateio`) while a transaction / payable / receivable splits part of its value into it. For a leaf, references on transactions / payables / receivables are set to null and the row is removed. Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Category deleted. |
| `404` | Category not found within the tenant. |
| `422` | The category still has subcategories (`reason: has_children`) or is used in a rateio (`reason: in_rateio`). |
