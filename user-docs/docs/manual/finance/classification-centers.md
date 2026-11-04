---
id: classification-centers
title: "kobana finance classification-centers"
sidebar_label: "classification-centers"
---

# kobana finance classification-centers


Comandos de `kobana finance classification-centers`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`classification-centers list`](#classification-centers-list) | `GET /v1/classification-centers` | Listar centros de classificação |
| [`classification-centers create`](#classification-centers-create) | `POST /v1/classification-centers` | Criar centro de classificação |
| [`classification-centers get`](#classification-centers-get) | `GET /v1/classification-centers/{id}` | Obter centro de classificação por ID |
| [`classification-centers update`](#classification-centers-update) | `PATCH /v1/classification-centers/{id}` | Atualizar centro de classificação |
| [`classification-centers delete`](#classification-centers-delete) | `DELETE /v1/classification-centers/{id}` | Excluir centro de classificação |

## classification-centers list

Listar centros de classificação

```text
kobana finance classification-centers list [OPÇÕES]
```

`GET /v1/classification-centers`

Returns a cursor-paginated page of classification centers for the workspace tied to the token's `sub` claim. Company-scoped within the workspace — pass `?company_id=…` to narrow. Requires scope `finance/cost_centers` with access level `read`. **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `active` | `string` | não | Filter by active flag. `true` returns only active; omit to return both. Valores: `true`, `false`. |
| `type` | `string` | não | `cost` (deducts), `revenue` (adds) or `both` (usable on either side). Accepts `[in]=cost,revenue,both`. Valores: `cost`, `revenue`, `both`. |
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
| `200` | Page of classification centers plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## classification-centers create

Criar centro de classificação

```text
kobana finance classification-centers create [OPÇÕES]
```

`POST /v1/classification-centers`

Creates a classification center. Requires scope `finance/cost_centers` with access level `write`. `workspace_id` is taken from the token. `type` (`cost` / `revenue`) drives the rateio polarity. Pass an `X-Idempotency-Key` header on retries. **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Owning company UUID. Must belong to the tenant. |
| `name` | `string` | sim |  |
| `type` | `string` | sim | `cost` (deducts from gross result), `revenue` (adds), or `both` (usable on either side). Valores: `cost`, `revenue`, `both`. |
| `description` | `string` | não |  |
| `active` | `boolean` | não | Defaults to `true`. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.classification-centers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Classification center created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or `company_id` not in the tenant. |

## classification-centers get

Obter centro de classificação por ID

```text
kobana finance classification-centers get [ID] [OPÇÕES]
```

`GET /v1/classification-centers/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/cost_centers` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish. **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

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
| `200` | Get a classification center by id |
| `404` | Classification center not found within the tenant. |

## classification-centers update

Atualizar centro de classificação

```text
kobana finance classification-centers update [ID] [OPÇÕES]
```

`PATCH /v1/classification-centers/{id}`

Partial update of `name`, `type`, `description`, `active`. Requires scope `finance/cost_centers` with access level `write`. Send `description: null` to clear it, or `active: false` to soft-deactivate (existing references are preserved). **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `type` | `string` | não | Valores: `cost`, `revenue`, `both`. |
| `description` | `string` | não |  |
| `active` | `boolean` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.classification-centers.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated classification center. |
| `400` | Malformed JSON body. |
| `404` | Classification center not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## classification-centers delete

Excluir centro de classificação

```text
kobana finance classification-centers delete [ID] [OPÇÕES]
```

`DELETE /v1/classification-centers/{id}`

Deletes a classification center. Requires scope `finance/cost_centers` with access level `delete`. Returns 422 (`reason: in_use`, with a `usage` breakdown) when the classification center is referenced by a rateio allocation, billing or transaction — soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404. **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Classification center deleted. |
| `404` | Classification center not found within the tenant. |
| `422` | The classification center is in use (`reason: in_use`). |
