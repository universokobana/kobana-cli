---
id: iof-rates
title: "kobana finance iof-rates"
sidebar_label: "iof-rates"
description: "List IOF rates"
---

# kobana finance iof-rates

List IOF rates

Commands of `kobana finance iof-rates`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`iof-rates`](#iof-rates) | `GET /v1/iof-rates` | List IOF rates |

## iof-rates

List IOF rates

```text
kobana finance iof-rates [OPTIONS]
```

`GET /v1/iof-rates`

The IOF rates with their validity: the global ones kept by Kobana plus this workspace's overrides. A card launch copies the rate in force on its purchase date, so a later rate never changes it. Only `card_international_purchase` is applied to launches; `card_financing_fixed` and `card_financing_daily` are reference only — the financing IOF the issuer charges is recorded as a `fee` launch. Requires scope `finance/credit_cards` with access level `read`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `operation` | `string` | no | Filter by operation. Accepts `[in]`. Values: `card_international_purchase`, `card_financing_fixed`, `card_financing_daily`. |
| `sort_by` | `string` | no | Field to order by. Default: by operation, newest validity first. Values: `valid_from`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`. Values: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of IOF rates. |
| `400` | Malformed query. |
