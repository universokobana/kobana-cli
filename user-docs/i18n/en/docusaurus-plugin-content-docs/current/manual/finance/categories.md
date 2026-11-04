---
id: categories
title: "kobana finance categories"
sidebar_label: "categories"
---

# kobana finance categories


Commands of `kobana finance categories`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`categories list`](#categories-list) | `GET /v1/categories` | List categories |
| [`categories create`](#categories-create) | `POST /v1/categories` | Create a category |
| [`categories get`](#categories-get) | `GET /v1/categories/{id}` | Get a category by id |
| [`categories update`](#categories-update) | `PATCH /v1/categories/{id}` | Update a category |
| [`categories delete`](#categories-delete) | `DELETE /v1/categories/{id}` | Delete a category |

## categories list

List categories

```text
kobana finance categories list [OPTIONS]
```

`GET /v1/categories`

Returns a cursor-paginated page of categories for the workspace tied to the token's `sub` claim. Company-scoped within the workspace; categories form a tree via `parent_id` but are returned flat — build the tree client-side using `parent_id` and `level`. Requires scope `finance/categories` with access level `read`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string` | no | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `parent_id` | `string` | no | Filter by direct parent UUID. To list root-level categories, use `?level=1` instead — the URL grammar does not support `parent_id=null`. |
| `type` | `string` | no | `debit` (cost) or `credit` (revenue). Accepts `[in]=debit,credit`. Values: `debit`, `credit`. |
| `level` | `integer` | no | Filter by depth. `1` returns root-level categories. Range operators supported: `[gte]`, `[lte]`. Example: `?level[lte]=2` for roots + their direct children. |
| `name` | `string` | no | Filter by exact name. Accepts `[in]`. |
| `sort_by` | `string` | no | Field to order by. Default: `sort_by=name&sort_order=asc`. Values: `created_at`, `level`, `name`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`; ignored without `sort_by`. Values: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of categories plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## categories create

Create a category

```text
kobana finance categories create [OPTIONS]
```

`POST /v1/categories`

Creates a category. Requires scope `finance/categories` with access level `write`. `workspace_id` is taken from the token. Omit `parent_id` for a root (`level` 1); when set, the server derives `level = parent.level + 1` (max depth 3) and requires `type` to match the parent. The response is flat (`parent_id` + `level`). Pass an `X-Idempotency-Key` header on retries.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string (uuid)` | yes | Owning company UUID. Must belong to the tenant. |
| `name` | `string` | yes |  |
| `type` | `string` | yes | `debit` (cost) or `credit` (revenue). A subcategory must match its parent's type. Values: `debit`, `credit`. |
| `parent_id` | `string (uuid)` | no | Parent category UUID. Omit (or `null`) for a root (`level` 1). When set, the server computes `level = parent.level + 1` (max 3) and requires `type` to match the parent. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

More fields and nested objects: `kobana schema finance.categories.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Category created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; `company_id` or `parent_id` not in the tenant; subcategory `type` not matching the parent; or tree depth > 3. |

## categories get

Get a category by id

```text
kobana finance categories get [ID] [OPTIONS]
```

`GET /v1/categories/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/categories` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a category by id |
| `404` | Category not found within the tenant. |

## categories update

Update a category

```text
kobana finance categories update [ID] [OPTIONS]
```

`PATCH /v1/categories/{id}`

Partial update of `name` and `type` only. Requires scope `finance/categories` with access level `write`. Re-parenting is not supported via v1. Changing a parent's `type` cascades to all descendants (internal behavior).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no |  |
| `type` | `string` | no | Changing a parent's type cascades to all descendants. Values: `debit`, `credit`. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | no | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema finance.categories.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Updated category. |
| `400` | Malformed JSON body. |
| `404` | Category not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## categories delete

Delete a category

```text
kobana finance categories delete [ID] [OPTIONS]
```

`DELETE /v1/categories/{id}`

Deletes a category. Requires scope `finance/categories` with access level `delete`. Returns 422 (`reason: has_children`) while the category still has subcategories — delete the children first — and 422 (`reason: in_rateio`) while a transaction / payable / receivable splits part of its value into it. For a leaf, references on transactions / payables / receivables are set to null and the row is removed. Calling DELETE twice returns 200 then 404.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Category deleted. |
| `404` | Category not found within the tenant. |
| `422` | The category still has subcategories (`reason: has_children`) or is used in a rateio (`reason: in_rateio`). |
