---
id: banks
title: "kobana finance banks"
sidebar_label: "banks"
description: "List banks"
---

# kobana finance banks

List banks

Commands of `kobana finance banks`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`banks`](#banks) | `GET /v1/banks` | List banks |

## banks

List banks

```text
kobana finance banks [OPTIONS]
```

`GET /v1/banks`

Returns a cursor-paginated page of the global bank catalog (COMPE/BACEN codes). Requires scope `finance/banks` with access level `read`. Tenant-less reference data — every token sees the same rows. Defaults to alphabetical sort by `name`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `code` | `string` | no | Filter by exact COMPE/BACEN code (unique). Accepts `[in]=001,237,341` for OR matching. |
| `name` | `string` | no | Filter by exact institution name. Accepts `[in]=...` for OR matching. |
| `sort_by` | `string` | no | Field to order by. Default: `sort_by=name&sort_order=asc`. Values: `code`, `name`. |
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
| `200` | Page of banks plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |
