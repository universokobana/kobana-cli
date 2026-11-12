---
id: iof-rates
title: "kobana finance iof-rates"
sidebar_label: "iof-rates"
description: "Listar alíquotas de IOF"
---

# kobana finance iof-rates

Listar alíquotas de IOF

Comandos de `kobana finance iof-rates`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`iof-rates`](#iof-rates) | `GET /v1/iof-rates` | Listar alíquotas de IOF |

## iof-rates

Listar alíquotas de IOF

```text
kobana finance iof-rates [OPÇÕES]
```

`GET /v1/iof-rates`

The IOF rates with their validity: the global ones kept by Kobana plus this workspace's overrides. A card launch copies the rate in force on its purchase date, so a later rate never changes it. Only `card_international_purchase` is applied to launches; `card_financing_fixed` and `card_financing_daily` are reference only — the financing IOF the issuer charges is recorded as a `fee` launch. Requires scope `finance/credit_cards` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `operation` | `string` | não | Filter by operation. Accepts `[in]`. Valores: `card_international_purchase`, `card_financing_fixed`, `card_financing_daily`. |
| `sort_by` | `string` | não | Field to order by. Default: by operation, newest validity first. Valores: `valid_from`. |
| `sort_order` | `string` | não | Order direction. Defaults to `asc`. Valores: `asc`, `desc`. |
| `limit` | `integer` | não | Page size. Default 25, max 100. |
| `cursor` | `string` | não | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Page of IOF rates. |
| `400` | Malformed query. |
