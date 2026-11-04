---
id: cash-flow
title: "kobana finance cash-flow"
sidebar_label: "cash-flow"
description: "Get the cash-flow projection"
---

# kobana finance cash-flow

Get the cash-flow projection

Commands of `kobana finance cash-flow`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`cash-flow`](#cash-flow) | `GET /v1/cash-flow` | Get the cash-flow projection |

## cash-flow

Get the cash-flow projection

```text
kobana finance cash-flow [OPTIONS]
```

`GET /v1/cash-flow`

Returns the cash-flow projection for the workspace tied to the token: a series of time buckets (realised + projected income/expenses, running balance) plus `current_balance`, the `today_bucket` seam and `retroactive_overdue`. Requires scope `finance/cash_flow` with access level `read`. Read-only.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `from` | `string` | yes | Window start (ISO 8601). Required. |
| `to` | `string` | yes | Window end (ISO 8601). Required. |
| `granularity` | `string` | no | Bucket size. Default `month`. A range over the per-granularity cap is clipped (`truncated: true`). Values: `day`, `week`, `month`, `year`. |
| `company_id` | `string` | no | Scope to one company. Omit for the consolidated (all-company) projection. |
| `account_id` | `string` | no | Scope to one financial account. Omit for all accounts. |
| `tz` | `string` | no | IANA time zone that anchors the `today` seam (e.g. `America/Sao_Paulo`, the default). Only the seam is tz-aware; bucketing stays UTC. An unrecognized value falls back to the default. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The cash-flow projection. |
| `400` | Malformed query (missing/invalid `from`/`to`, bad granularity, non-uuid scope id, unknown param). |
| `404` | The `company_id` or `account_id` is not in the tenant. |
