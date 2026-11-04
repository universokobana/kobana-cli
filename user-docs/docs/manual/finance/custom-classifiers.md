---
id: custom-classifiers
title: "kobana finance custom-classifiers"
sidebar_label: "custom-classifiers"
---

# kobana finance custom-classifiers


Comandos de `kobana finance custom-classifiers`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`custom-classifiers values update`](#custom-classifiers-values-update) | `PATCH /v1/custom-classifiers/{id}/values/{valueId}` | Atualizar valor de classificador personalizado |
| [`custom-classifiers values delete`](#custom-classifiers-values-delete) | `DELETE /v1/custom-classifiers/{id}/values/{valueId}` | Excluir valor de classificador personalizado |
| [`custom-classifiers values create`](#custom-classifiers-values-create) | `POST /v1/custom-classifiers/{id}/values` | Adicionar valor a um classificador personalizado |
| [`custom-classifiers list`](#custom-classifiers-list) | `GET /v1/custom-classifiers` | Listar classificadores personalizados |
| [`custom-classifiers create`](#custom-classifiers-create) | `POST /v1/custom-classifiers` | Criar classificador personalizado |
| [`custom-classifiers get`](#custom-classifiers-get) | `GET /v1/custom-classifiers/{id}` | Obter classificador personalizado por ID |
| [`custom-classifiers update`](#custom-classifiers-update) | `PATCH /v1/custom-classifiers/{id}` | Atualizar classificador personalizado |
| [`custom-classifiers delete`](#custom-classifiers-delete) | `DELETE /v1/custom-classifiers/{id}` | Excluir classificador personalizado |

## custom-classifiers values update

Atualizar valor de classificador personalizado

```text
kobana finance custom-classifiers values update [ID] [VALUEID] [OPÇÕES]
```

`PATCH /v1/custom-classifiers/{id}/values/{valueId}`

Partial update of a value's `value`, `color`, `active`. Requires scope `finance/custom_classifiers` with access level `write`. Send `active: false` to soft-deactivate, or `color: null` to clear the swatch.

**Argumentos do caminho:** `id`, `valueId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `value` | `string` | não |  |
| `color` | `string` | não | Hex swatch, or `null` to clear it and fall back to the palette. |
| `active` | `boolean` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.custom-classifiers.values.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated value. |
| `400` | Malformed JSON body. |
| `404` | Value (or its parent dimension) not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## custom-classifiers values delete

Excluir valor de classificador personalizado

```text
kobana finance custom-classifiers values delete [ID] [VALUEID] [OPÇÕES]
```

`DELETE /v1/custom-classifiers/{id}/values/{valueId}`

Deletes a value. Requires scope `finance/custom_classifiers` with access level `delete`. Returns 422 (`reason: in_use`) when the value is assigned to a payable / receivable / transaction — soft-deactivate via `PATCH { active: false }` instead.

**Argumentos do caminho:** `id`, `valueId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Value deleted. |
| `404` | Value (or its parent dimension) not found within the tenant. |
| `422` | The value is in use (`reason: in_use`). |

## custom-classifiers values create

Adicionar valor a um classificador personalizado

```text
kobana finance custom-classifiers values create [ID] [OPÇÕES]
```

`POST /v1/custom-classifiers/{id}/values`

Adds a value to the dimension. Requires scope `finance/custom_classifiers` with access level `write`. Pass an `X-Idempotency-Key` header on retries.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `value` | `string` | sim |  |
| `color` | `string` | não | Hex swatch (`#rrggbb` or `#rrggbbaa`). Omit for an auto-assigned palette color. |
| `active` | `boolean` | não | Defaults to `true`. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.custom-classifiers.values.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Value created. The `Location` header points at the new value. |
| `400` | Malformed JSON body. |
| `404` | Parent custom classifier not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## custom-classifiers list

Listar classificadores personalizados

```text
kobana finance custom-classifiers list [OPÇÕES]
```

`GET /v1/custom-classifiers`

Returns a cursor-paginated page of custom classifiers (user-defined classification dimensions) for the workspace tied to the token's `sub` claim. Each dimension carries its `values[]` inline. Company-scoped — pass `?company_id=…` to narrow. Requires scope `finance/custom_classifiers` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `active` | `string` | não | Filter by active flag. `true` returns only active; omit to return both. Valores: `true`, `false`. |
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
| `200` | Page of custom classifiers plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## custom-classifiers create

Criar classificador personalizado

```text
kobana finance custom-classifiers create [OPÇÕES]
```

`POST /v1/custom-classifiers`

Creates a custom classifier dimension. Requires scope `finance/custom_classifiers` with access level `write`. `workspace_id` is taken from the token. Add values with `POST /custom-classifiers/{id}/values`. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Owning company UUID. Must belong to the tenant. |
| `name` | `string` | sim |  |
| `field_type` | `string` | não | Defaults to `dropdown`. Valores: `dropdown`, `radio`, `multi_select`, `number`, `text`, `phone`, `url`, `date`, `datetime`. |
| `number_format` | `string` | não | Only meaningful for `number` (defaults to `formatted`); ignored otherwise. Valores: `formatted`, `unformatted`, `currency`, `percent`, `null`. |
| `option_style` | `string` | não | Only meaningful for list types (defaults to `default`); ignored otherwise. Valores: `default`, `dot`, `null`. |
| `active` | `boolean` | não | Defaults to `true`. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.custom-classifiers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Custom classifier created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or `company_id` not in the tenant. |

## custom-classifiers get

Obter classificador personalizado por ID

```text
kobana finance custom-classifiers get [ID] [OPÇÕES]
```

`GET /v1/custom-classifiers/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/custom_classifiers` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a custom classifier by id |
| `404` | Custom classifier not found within the tenant. |

## custom-classifiers update

Atualizar classificador personalizado

```text
kobana finance custom-classifiers update [ID] [OPÇÕES]
```

`PATCH /v1/custom-classifiers/{id}`

Partial update of `name`, `field_type`, `number_format`, `option_style`, `active`. Requires scope `finance/custom_classifiers` with access level `write`. Send `active: false` to soft-deactivate (existing assignments are preserved).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `field_type` | `string` | não | Valores: `dropdown`, `radio`, `multi_select`, `number`, `text`, `phone`, `url`, `date`, `datetime`. |
| `number_format` | `string` | não | Valores: `formatted`, `unformatted`, `currency`, `percent`, `null`. |
| `option_style` | `string` | não | Valores: `default`, `dot`, `null`. |
| `active` | `boolean` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.custom-classifiers.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated custom classifier. |
| `400` | Malformed JSON body. |
| `404` | Custom classifier not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## custom-classifiers delete

Excluir classificador personalizado

```text
kobana finance custom-classifiers delete [ID] [OPÇÕES]
```

`DELETE /v1/custom-classifiers/{id}`

Deletes a custom classifier and its values. Requires scope `finance/custom_classifiers` with access level `delete`. Returns 422 (`reason: in_use`, with a `usage` breakdown) when any value is assigned to a payable / receivable / transaction — soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Custom classifier deleted. |
| `404` | Custom classifier not found within the tenant. |
| `422` | The custom classifier is in use (`reason: in_use`). |
