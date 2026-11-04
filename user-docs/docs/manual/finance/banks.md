---
id: banks
title: "kobana finance banks"
sidebar_label: "banks"
description: "Listar bancos"
---

# kobana finance banks

Listar bancos

Comandos de `kobana finance banks`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`banks`](#banks) | `GET /v1/banks` | Listar bancos |

## banks

Listar bancos

```text
kobana finance banks [OPÇÕES]
```

`GET /v1/banks`

Returns a cursor-paginated page of the global bank catalog (COMPE/BACEN codes). Requires scope `finance/banks` with access level `read`. Tenant-less reference data — every token sees the same rows. Defaults to alphabetical sort by `name`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `code` | `string` | não | Filter by exact COMPE/BACEN code (unique). Accepts `[in]=001,237,341` for OR matching. |
| `name` | `string` | não | Filter by exact institution name. Accepts `[in]=...` for OR matching. |
| `sort_by` | `string` | não | Field to order by. Default: `sort_by=name&sort_order=asc`. Valores: `code`, `name`. |
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
| `200` | Page of banks plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |
