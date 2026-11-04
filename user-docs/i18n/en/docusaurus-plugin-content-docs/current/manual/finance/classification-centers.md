---
id: classification-centers
title: "kobana finance classification-centers"
sidebar_label: "classification-centers"
---

# kobana finance classification-centers


Commands of `kobana finance classification-centers`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`classification-centers list`](#classification-centers-list) | `GET /v1/classification-centers` | List classification centers |
| [`classification-centers create`](#classification-centers-create) | `POST /v1/classification-centers` | Create a classification center |
| [`classification-centers get`](#classification-centers-get) | `GET /v1/classification-centers/{id}` | Get a classification center by id |
| [`classification-centers update`](#classification-centers-update) | `PATCH /v1/classification-centers/{id}` | Update a classification center |
| [`classification-centers delete`](#classification-centers-delete) | `DELETE /v1/classification-centers/{id}` | Delete a classification center |

## classification-centers list

List classification centers

```text
kobana finance classification-centers list [OPTIONS]
```

`GET /v1/classification-centers`

Returns a cursor-paginated page of classification centers for the workspace tied to the token's `sub` claim. Company-scoped within the workspace — pass `?company_id=…` to narrow. Requires scope `finance/cost_centers` with access level `read`. **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string` | no | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `active` | `string` | no | Filter by active flag. `true` returns only active; omit to return both. Values: `true`, `false`. |
| `type` | `string` | no | `cost` (deducts), `revenue` (adds) or `both` (usable on either side). Accepts `[in]=cost,revenue,both`. Values: `cost`, `revenue`, `both`. |
| `name` | `string` | no | Filter by exact name. Accepts `[in]`. |
| `sort_by` | `string` | no | Field to order by. Default: `sort_by=name&sort_order=asc`. Values: `created_at`, `name`. |
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
| `200` | Page of classification centers plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## classification-centers create

Create a classification center

```text
kobana finance classification-centers create [OPTIONS]
```

`POST /v1/classification-centers`

Creates a classification center. Requires scope `finance/cost_centers` with access level `write`. `workspace_id` is taken from the token. `type` (`cost` / `revenue`) drives the rateio polarity. Pass an `X-Idempotency-Key` header on retries. **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string (uuid)` | yes | Owning company UUID. Must belong to the tenant. |
| `name` | `string` | yes |  |
| `type` | `string` | yes | `cost` (deducts from gross result), `revenue` (adds), or `both` (usable on either side). Values: `cost`, `revenue`, `both`. |
| `description` | `string` | no |  |
| `active` | `boolean` | no | Defaults to `true`. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

More fields and nested objects: `kobana schema finance.classification-centers.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Classification center created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, or `company_id` not in the tenant. |

## classification-centers get

Get a classification center by id

```text
kobana finance classification-centers get [ID] [OPTIONS]
```

`GET /v1/classification-centers/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/cost_centers` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish. **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

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
| `200` | Get a classification center by id |
| `404` | Classification center not found within the tenant. |

## classification-centers update

Update a classification center

```text
kobana finance classification-centers update [ID] [OPTIONS]
```

`PATCH /v1/classification-centers/{id}`

Partial update of `name`, `type`, `description`, `active`. Requires scope `finance/cost_centers` with access level `write`. Send `description: null` to clear it, or `active: false` to soft-deactivate (existing references are preserved). **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no |  |
| `type` | `string` | no | Values: `cost`, `revenue`, `both`. |
| `description` | `string` | no |  |
| `active` | `boolean` | no |  |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | no | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema finance.classification-centers.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Updated classification center. |
| `400` | Malformed JSON body. |
| `404` | Classification center not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure. |

## classification-centers delete

Delete a classification center

```text
kobana finance classification-centers delete [ID] [OPTIONS]
```

`DELETE /v1/classification-centers/{id}`

Deletes a classification center. Requires scope `finance/cost_centers` with access level `delete`. Returns 422 (`reason: in_use`, with a `usage` breakdown) when the classification center is referenced by a rateio allocation, billing or transaction — soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404. **Naming:** the path is hyphenated like every other v1 resource (`/classification-centers`), but the scope is still `finance/cost_centers` — the table behind it is `cost_centers`, and the scope names the grant, not the URL.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Classification center deleted. |
| `404` | Classification center not found within the tenant. |
| `422` | The classification center is in use (`reason: in_use`). |
