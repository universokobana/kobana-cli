---
id: custom-classifiers
title: "kobana finance custom-classifiers"
sidebar_label: "custom-classifiers"
---

# kobana finance custom-classifiers


Comandos de `kobana finance custom-classifiers`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`custom-classifiers values update`](#custom-classifiers-values-update) | `PATCH /v1/custom-classifiers/{id}/values/{valueId}` | Actualizar valor de clasificador personalizado |
| [`custom-classifiers values delete`](#custom-classifiers-values-delete) | `DELETE /v1/custom-classifiers/{id}/values/{valueId}` | Eliminar valor de clasificador personalizado |
| [`custom-classifiers values create`](#custom-classifiers-values-create) | `POST /v1/custom-classifiers/{id}/values` | Agregar valor a un clasificador personalizado |
| [`custom-classifiers list`](#custom-classifiers-list) | `GET /v1/custom-classifiers` | Listar clasificadores personalizados |
| [`custom-classifiers create`](#custom-classifiers-create) | `POST /v1/custom-classifiers` | Crear clasificador personalizado |
| [`custom-classifiers get`](#custom-classifiers-get) | `GET /v1/custom-classifiers/{id}` | Obtener clasificador personalizado por ID |
| [`custom-classifiers update`](#custom-classifiers-update) | `PATCH /v1/custom-classifiers/{id}` | Actualizar clasificador personalizado |
| [`custom-classifiers delete`](#custom-classifiers-delete) | `DELETE /v1/custom-classifiers/{id}` | Eliminar clasificador personalizado |

## custom-classifiers values update

Actualizar valor de clasificador personalizado

```text
kobana finance custom-classifiers values update [ID] [VALUEID] [OPCIONES]
```

`PATCH /v1/custom-classifiers/{id}/values/{valueId}`

Partial update of a value's `value`, `color`, `active`. Requires scope `finance/custom_classifiers` with access level `write`. Send `active: false` to soft-deactivate, or `color: null` to clear the swatch.

**Argumentos de la ruta:** `id`, `valueId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `value` | `string` | no |  |
| `color` | `string` | no | Hex swatch, or `null` to clear it and fall back to the palette. |
| `active` | `boolean` | no |  |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | no | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema finance.custom-classifiers.values.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Updated value. |
| `400` | Malformed JSON body. |
| `404` | Value (or its parent dimension) not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## custom-classifiers values delete

Eliminar valor de clasificador personalizado

```text
kobana finance custom-classifiers values delete [ID] [VALUEID] [OPCIONES]
```

`DELETE /v1/custom-classifiers/{id}/values/{valueId}`

Deletes a value. Requires scope `finance/custom_classifiers` with access level `delete`. Returns 422 (`reason: in_use`) when the value is assigned to a payable / receivable / transaction — soft-deactivate via `PATCH { active: false }` instead.

**Argumentos de la ruta:** `id`, `valueId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Value deleted. |
| `404` | Value (or its parent dimension) not found within the tenant. |
| `422` | The value is in use (`reason: in_use`). |

## custom-classifiers values create

Agregar valor a un clasificador personalizado

```text
kobana finance custom-classifiers values create [ID] [OPCIONES]
```

`POST /v1/custom-classifiers/{id}/values`

Adds a value to the dimension. Requires scope `finance/custom_classifiers` with access level `write`. Pass an `X-Idempotency-Key` header on retries.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `value` | `string` | sí |  |
| `color` | `string` | no | Hex swatch (`#rrggbb` or `#rrggbbaa`). Omit for an auto-assigned palette color. |
| `active` | `boolean` | no | Defaults to `true`. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Otros campos y objetos anidados: `kobana schema finance.custom-classifiers.values.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Value created. The `Location` header points at the new value. |
| `400` | Malformed JSON body. |
| `404` | Parent custom classifier not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## custom-classifiers list

Listar clasificadores personalizados

```text
kobana finance custom-classifiers list [OPCIONES]
```

`GET /v1/custom-classifiers`

Returns a cursor-paginated page of custom classifiers (user-defined classification dimensions) for the workspace tied to the token's `sub` claim. Each dimension carries its `values[]` inline. Company-scoped — pass `?company_id=…` to narrow. Requires scope `finance/custom_classifiers` with access level `read`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string` | no | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `active` | `string` | no | Filter by active flag. `true` returns only active; omit to return both. Valores: `true`, `false`. |
| `name` | `string` | no | Filter by exact name. Accepts `[in]`. |
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
| `200` | Page of custom classifiers plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## custom-classifiers create

Crear clasificador personalizado

```text
kobana finance custom-classifiers create [OPCIONES]
```

`POST /v1/custom-classifiers`

Creates a custom classifier dimension. Requires scope `finance/custom_classifiers` with access level `write`. `workspace_id` is taken from the token. Add values with `POST /custom-classifiers/{id}/values`. Pass an `X-Idempotency-Key` header on retries.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string (uuid)` | sí | Owning company UUID. Must belong to the tenant. |
| `name` | `string` | sí |  |
| `field_type` | `string` | no | Defaults to `dropdown`. Valores: `dropdown`, `radio`, `multi_select`, `number`, `text`, `phone`, `url`, `date`, `datetime`. |
| `number_format` | `string` | no | Only meaningful for `number` (defaults to `formatted`); ignored otherwise. Valores: `formatted`, `unformatted`, `currency`, `percent`, `null`. |
| `option_style` | `string` | no | Only meaningful for list types (defaults to `default`); ignored otherwise. Valores: `default`, `dot`, `null`. |
| `active` | `boolean` | no | Defaults to `true`. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Otros campos y objetos anidados: `kobana schema finance.custom-classifiers.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Custom classifier created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or `company_id` not in the tenant. |

## custom-classifiers get

Obtener clasificador personalizado por ID

```text
kobana finance custom-classifiers get [ID] [OPCIONES]
```

`GET /v1/custom-classifiers/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/custom_classifiers` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a custom classifier by id |
| `404` | Custom classifier not found within the tenant. |

## custom-classifiers update

Actualizar clasificador personalizado

```text
kobana finance custom-classifiers update [ID] [OPCIONES]
```

`PATCH /v1/custom-classifiers/{id}`

Partial update of `name`, `field_type`, `number_format`, `option_style`, `active`. Requires scope `finance/custom_classifiers` with access level `write`. Send `active: false` to soft-deactivate (existing assignments are preserved).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no |  |
| `field_type` | `string` | no | Valores: `dropdown`, `radio`, `multi_select`, `number`, `text`, `phone`, `url`, `date`, `datetime`. |
| `number_format` | `string` | no | Valores: `formatted`, `unformatted`, `currency`, `percent`, `null`. |
| `option_style` | `string` | no | Valores: `default`, `dot`, `null`. |
| `active` | `boolean` | no |  |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | no | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema finance.custom-classifiers.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Updated custom classifier. |
| `400` | Malformed JSON body. |
| `404` | Custom classifier not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## custom-classifiers delete

Eliminar clasificador personalizado

```text
kobana finance custom-classifiers delete [ID] [OPCIONES]
```

`DELETE /v1/custom-classifiers/{id}`

Deletes a custom classifier and its values. Requires scope `finance/custom_classifiers` with access level `delete`. Returns 422 (`reason: in_use`, with a `usage` breakdown) when any value is assigned to a payable / receivable / transaction — soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Custom classifier deleted. |
| `404` | Custom classifier not found within the tenant. |
| `422` | The custom classifier is in use (`reason: in_use`). |
