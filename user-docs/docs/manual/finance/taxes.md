---
id: taxes
title: "kobana finance taxes"
sidebar_label: "taxes"
---

# kobana finance taxes


Comandos de `kobana finance taxes`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`taxes list`](#taxes-list) | `GET /v1/taxes` | Listar impostos |
| [`taxes create`](#taxes-create) | `POST /v1/taxes` | Criar imposto |
| [`taxes get`](#taxes-get) | `GET /v1/taxes/{id}` | Obter imposto por ID |
| [`taxes update`](#taxes-update) | `PATCH /v1/taxes/{id}` | Atualizar imposto |
| [`taxes delete`](#taxes-delete) | `DELETE /v1/taxes/{id}` | Excluir imposto |

## taxes list

Listar impostos

```text
kobana finance taxes list [OPÇÕES]
```

`GET /v1/taxes`

Returns a cursor-paginated page of tax kinds for the workspace tied to the token's `sub` claim. Requires scope `finance/taxes` with access level `read`. Catalogal entity — defaults to alphabetical sort by `name`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `active` | `string` | não | Filter by active flag. `true` returns only active (default UX preference); omit to return both. Valores: `true`, `false`. |
| `default_withholding` | `string` | não | Filter by `default_withholding` flag (whether new charges default to imposto retido). Valores: `true`, `false`. |
| `name` | `string` | não | Filter by exact name (unique per workspace). Accepts `[in]=IRRF,INSS` for OR matching. |
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
| `200` | Page of tax kinds plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## taxes create

Criar imposto

```text
kobana finance taxes create [OPÇÕES]
```

`POST /v1/taxes`

Creates a tax kind. Requires scope `finance/taxes` with access level `write`. Workspace-scoped (no company). `name` is unique per workspace (duplicate → 409). `default_rate` is a percent string (0–100). Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Tax label. Unique per workspace. |
| `default_rate` | `number` | não | Suggested default rate as a percent (0–100, up to 2 decimals). `null`/omitted = no default. |
| `default_withholding` | `boolean` | não | Defaults to `true` (imposto retido). |
| `active` | `boolean` | não | Defaults to `true`. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.taxes.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Tax created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | A tax with the same `name` already exists, or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, or `default_rate` out of the 0–100 range. |

## taxes get

Obter imposto por ID

```text
kobana finance taxes get [ID] [OPÇÕES]
```

`GET /v1/taxes/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/taxes` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a tax kind by id |
| `404` | Tax not found within the tenant. |

## taxes update

Atualizar imposto

```text
kobana finance taxes update [ID] [OPÇÕES]
```

`PATCH /v1/taxes/{id}`

Partial update of `name`, `default_rate`, `default_withholding`, `active`. Requires scope `finance/taxes` with access level `write`. Send `default_rate: null` to clear it, or `active: false` to soft-deactivate.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Tax label. Unique per workspace. |
| `default_rate` | `number` | não | Suggested default rate as a percent (0–100, up to 2 decimals). `null`/omitted = no default. |
| `default_withholding` | `boolean` | não | Defaults to `true` (imposto retido). |
| `active` | `boolean` | não | Defaults to `true`. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.taxes.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated tax. |
| `400` | Malformed JSON body. |
| `404` | Tax not found within the tenant. |
| `409` | A tax with the same `name` already exists. |
| `422` | Schema validation failure, or `default_rate` out of range. |

## taxes delete

Excluir imposto

```text
kobana finance taxes delete [ID] [OPÇÕES]
```

`DELETE /v1/taxes/{id}`

Deletes a tax. Requires scope `finance/taxes` with access level `delete`. Returns 422 (`reason: in_use`, with the referencing payable/receivable ids) when the tax still has charges — soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Tax deleted. |
| `404` | Tax not found within the tenant. |
| `422` | The tax has charges (`reason: in_use`). |
