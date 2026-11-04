---
id: companies
title: "kobana finance companies"
sidebar_label: "companies"
---

# kobana finance companies


Comandos de `kobana finance companies`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`companies list`](#companies-list) | `GET /v1/companies` | Listar empresas |
| [`companies create`](#companies-create) | `POST /v1/companies` | Crear empresa |
| [`companies get`](#companies-get) | `GET /v1/companies/{id}` | Obtener empresa por ID |
| [`companies update`](#companies-update) | `PATCH /v1/companies/{id}` | Actualizar empresa |
| [`companies delete`](#companies-delete) | `DELETE /v1/companies/{id}` | Eliminar empresa |

## companies list

Listar empresas

```text
kobana finance companies list [OPCIONES]
```

`GET /v1/companies`

Returns a cursor-paginated page of companies for the workspace tied to the token's `sub` claim. Requires scope `finance/companies` with access level `read`. Catalogal entity — defaults to alphabetical sort by `name`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `active` | `string` | no | Filter by active flag. `true` returns only active; omit to return both. Valores: `true`, `false`. |
| `type` | `string` | no | `matriz` (head office) or `filial` (branch). Accepts `[in]=matriz,filial`. Valores: `matriz`, `filial`. |
| `cnpj` | `string` | no | Filter by CNPJ (14-digit numeric string, no formatting). Accepts `[in]=cnpj1,cnpj2`. |
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
| `200` | Page of companies plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## companies create

Crear empresa

```text
kobana finance companies create [OPCIONES]
```

`POST /v1/companies`

Creates a company. Requires scope `finance/companies` with access level `write`. **`cnpj` is required and validated** with the standard BR check-digit algorithm — a missing, invalid, or document-less value returns 422 (there is no pessoa-física / document-less company in this API; a migrating client whose source row has no CNPJ must supply one). The CNPJ must be unique among active companies in the workspace (duplicate → 409). `type` (matriz/filial) is derived server-side from the CNPJ branch digits and is never read from the request. `active` starts `true`. Pass an `X-Idempotency-Key` header on retries.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Legal name as registered. |
| `cnpj` | `string` | sí | CNPJ — 14 characters. Formatting (dots / slash / dash) is accepted and stripped; letters are uppercased. Supports both numeric-only (legacy) and alphanumeric (new RFB format) CNPJs. Validated with the standard BR check-digit algorithm. **Required** — a missing, invalid, or document-less value returns 422. Must be unique among active companies in the workspace (duplicate → 409). `type` (matriz/filial) is derived server-side. |
| `trade_name` | `string` | no |  |
| `municipal_registration` | `string` | no |  |
| `tax_regime` | `string` | no | Valores: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`. |
| `address` | `object` | no |  |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Otros campos y objetos anidados: `kobana schema finance.companies.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Company created. The `Location` header points at the new row; `type` is derived from the CNPJ. |
| `400` | Malformed JSON body. |
| `409` | A company with the same CNPJ already exists (active), or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, or a missing / invalid / document-less `cnpj`. |

## companies get

Obtener empresa por ID

```text
kobana finance companies get [ID] [OPCIONES]
```

`GET /v1/companies/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/companies` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a company by id |
| `404` | Company not found within the tenant. |

## companies update

Actualizar empresa

```text
kobana finance companies update [ID] [OPCIONES]
```

`PATCH /v1/companies/{id}`

Partial update of `name` and `active`. Requires scope `finance/companies` with access level `write`. The CNPJ (and the `type` it derives) is immutable via the public API — send `active: false` to soft-deactivate.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no |  |
| `active` | `boolean` | no | Set `false` to soft-deactivate the company (existing references are preserved). |
| `trade_name` | `string` | no |  |
| `municipal_registration` | `string` | no |  |
| `tax_regime` | `string` | no | Valores: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`. |
| `address` | `object` | no | Replaces the fiscal address wholesale. `null` clears it; omitting it leaves it untouched. It is never merged field by field — a partial merge would leave a company registered under half of one address and half of another. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | no | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema finance.companies.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Updated company. |
| `400` | Malformed JSON body. |
| `404` | Company not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## companies delete

Eliminar empresa

```text
kobana finance companies delete [ID] [OPCIONES]
```

`DELETE /v1/companies/{id}`

Hard-deletes a company. Requires scope `finance/companies` with access level `delete`. Blocked with 409 (`details` carries the per-table counts `{ transactions, payables, receivables, people }`) when the company still has financial records or active people — remove or reassign those first, or soft-deactivate via `PATCH { active: false }`. Config tables (categories, classification centers, financial accounts, soft-deleted people) cascade automatically. Calling DELETE twice returns 200 then 404.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Company deleted. |
| `404` | Company not found within the tenant. |
| `409` | The company has financial records or active people (`details` carries the counts). |
