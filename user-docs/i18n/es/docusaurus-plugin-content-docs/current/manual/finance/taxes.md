---
id: taxes
title: "kobana finance taxes"
sidebar_label: "taxes"
---

# kobana finance taxes


Comandos de `kobana finance taxes`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`taxes list`](#taxes-list) | `GET /v1/taxes` | Listar impuestos |
| [`taxes create`](#taxes-create) | `POST /v1/taxes` | Crear impuesto |
| [`taxes get`](#taxes-get) | `GET /v1/taxes/{id}` | Obtener impuesto por ID |
| [`taxes update`](#taxes-update) | `PATCH /v1/taxes/{id}` | Actualizar impuesto |
| [`taxes delete`](#taxes-delete) | `DELETE /v1/taxes/{id}` | Eliminar impuesto |

## taxes list

Listar impuestos

```text
kobana finance taxes list [OPCIONES]
```

`GET /v1/taxes`

Returns a cursor-paginated page of tax kinds for the workspace tied to the token's `sub` claim. Requires scope `finance/taxes` with access level `read`. Catalogal entity — defaults to alphabetical sort by `name`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `active` | `string` | no | Filter by active flag. `true` returns only active (default UX preference); omit to return both. Valores: `true`, `false`. |
| `default_withholding` | `string` | no | Filter by `default_withholding` flag (whether new charges default to imposto retido). Valores: `true`, `false`. |
| `name` | `string` | no | Filter by exact name (unique per workspace). Accepts `[in]=IRRF,INSS` for OR matching. |
| `sort_by` | `string` | no | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `created_at`, `name`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`; ignored without `sort_by`. Valores: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Page of tax kinds plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## taxes create

Crear impuesto

```text
kobana finance taxes create [OPCIONES]
```

`POST /v1/taxes`

Creates a tax kind. Requires scope `finance/taxes` with access level `write`. Workspace-scoped (no company). `name` is unique per workspace (duplicate → 409). `default_rate` is a percent string (0–100). Pass an `X-Idempotency-Key` header on retries.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Tax label. Unique per workspace. |
| `default_rate` | `number` | no | Suggested default rate as a percent (0–100, up to 2 decimals). `null`/omitted = no default. |
| `default_withholding` | `boolean` | no | Defaults to `true` (imposto retido). |
| `active` | `boolean` | no | Defaults to `true`. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Otros campos y objetos anidados: `kobana schema finance.taxes.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Tax created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | A tax with the same `name` already exists, or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, or `default_rate` out of the 0–100 range. |

## taxes get

Obtener impuesto por ID

```text
kobana finance taxes get [ID] [OPCIONES]
```

`GET /v1/taxes/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/taxes` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Get a tax kind by id |
| `404` | Tax not found within the tenant. |

## taxes update

Actualizar impuesto

```text
kobana finance taxes update [ID] [OPCIONES]
```

`PATCH /v1/taxes/{id}`

Partial update of `name`, `default_rate`, `default_withholding`, `active`. Requires scope `finance/taxes` with access level `write`. Send `default_rate: null` to clear it, or `active: false` to soft-deactivate.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Tax label. Unique per workspace. |
| `default_rate` | `number` | no | Suggested default rate as a percent (0–100, up to 2 decimals). `null`/omitted = no default. |
| `default_withholding` | `boolean` | no | Defaults to `true` (imposto retido). |
| `active` | `boolean` | no | Defaults to `true`. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Otros campos y objetos anidados: `kobana schema finance.taxes.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Updated tax. |
| `400` | Malformed JSON body. |
| `404` | Tax not found within the tenant. |
| `409` | A tax with the same `name` already exists. |
| `422` | Schema validation failure, or `default_rate` out of range. |

## taxes delete

Eliminar impuesto

```text
kobana finance taxes delete [ID] [OPCIONES]
```

`DELETE /v1/taxes/{id}`

Deletes a tax. Requires scope `finance/taxes` with access level `delete`. Returns 422 (`reason: in_use`, with the referencing payable/receivable ids) when the tax still has charges — soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Tax deleted. |
| `404` | Tax not found within the tenant. |
| `422` | The tax has charges (`reason: in_use`). |
