---
id: taxes
title: "kobana finance taxes"
sidebar_label: "taxes"
---

# kobana finance taxes


Commands of `kobana finance taxes`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`taxes list`](#taxes-list) | `GET /v1/taxes` | List tax kinds |
| [`taxes create`](#taxes-create) | `POST /v1/taxes` | Create a tax kind |
| [`taxes get`](#taxes-get) | `GET /v1/taxes/{id}` | Get a tax kind by id |
| [`taxes update`](#taxes-update) | `PATCH /v1/taxes/{id}` | Update a tax kind |
| [`taxes delete`](#taxes-delete) | `DELETE /v1/taxes/{id}` | Delete a tax kind |

## taxes list

List tax kinds

```text
kobana finance taxes list [OPTIONS]
```

`GET /v1/taxes`

Returns a cursor-paginated page of tax kinds for the workspace tied to the token's `sub` claim. Requires scope `finance/taxes` with access level `read`. Catalogal entity — defaults to alphabetical sort by `name`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `active` | `string` | no | Filter by active flag. `true` returns only active (default UX preference); omit to return both. Values: `true`, `false`. |
| `default_withholding` | `string` | no | Filter by `default_withholding` flag (whether new charges default to imposto retido). Values: `true`, `false`. |
| `name` | `string` | no | Filter by exact name (unique per workspace). Accepts `[in]=IRRF,INSS` for OR matching. |
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
| `200` | Page of tax kinds plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## taxes create

Create a tax kind

```text
kobana finance taxes create [OPTIONS]
```

`POST /v1/taxes`

Creates a tax kind. Requires scope `finance/taxes` with access level `write`. Workspace-scoped (no company). `name` is unique per workspace (duplicate → 409). `default_rate` is a percent string (0–100). Pass an `X-Idempotency-Key` header on retries.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Tax label. Unique per workspace. |
| `default_rate` | `number` | no | Suggested default rate as a percent (0–100, up to 2 decimals). `null`/omitted = no default. |
| `default_withholding` | `boolean` | no | Defaults to `true` (imposto retido). |
| `active` | `boolean` | no | Defaults to `true`. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

More fields and nested objects: `kobana schema finance.taxes.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Tax created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | A tax with the same `name` already exists, or `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, or `default_rate` out of the 0–100 range. |

## taxes get

Get a tax kind by id

```text
kobana finance taxes get [ID] [OPTIONS]
```

`GET /v1/taxes/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/taxes` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a tax kind by id |
| `404` | Tax not found within the tenant. |

## taxes update

Update a tax kind

```text
kobana finance taxes update [ID] [OPTIONS]
```

`PATCH /v1/taxes/{id}`

Partial update of `name`, `default_rate`, `default_withholding`, `active`. Requires scope `finance/taxes` with access level `write`. Send `default_rate: null` to clear it, or `active: false` to soft-deactivate.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Tax label. Unique per workspace. |
| `default_rate` | `number` | no | Suggested default rate as a percent (0–100, up to 2 decimals). `null`/omitted = no default. |
| `default_withholding` | `boolean` | no | Defaults to `true` (imposto retido). |
| `active` | `boolean` | no | Defaults to `true`. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

More fields and nested objects: `kobana schema finance.taxes.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Updated tax. |
| `400` | Malformed JSON body. |
| `404` | Tax not found within the tenant. |
| `409` | A tax with the same `name` already exists. |
| `422` | Schema validation failure, or `default_rate` out of range. |

## taxes delete

Delete a tax kind

```text
kobana finance taxes delete [ID] [OPTIONS]
```

`DELETE /v1/taxes/{id}`

Deletes a tax. Requires scope `finance/taxes` with access level `delete`. Returns 422 (`reason: in_use`, with the referencing payable/receivable ids) when the tax still has charges — soft-deactivate via `PATCH { active: false }` instead. Calling DELETE twice returns 200 then 404.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Tax deleted. |
| `404` | Tax not found within the tenant. |
| `422` | The tax has charges (`reason: in_use`). |
