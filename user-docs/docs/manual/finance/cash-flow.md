---
id: cash-flow
title: "kobana finance cash-flow"
sidebar_label: "cash-flow"
description: "Obter a projeção de fluxo de caixa"
---

# kobana finance cash-flow

Obter a projeção de fluxo de caixa

Comandos de `kobana finance cash-flow`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`cash-flow`](#cash-flow) | `GET /v1/cash-flow` | Obter a projeção de fluxo de caixa |

## cash-flow

Obter a projeção de fluxo de caixa

```text
kobana finance cash-flow [OPÇÕES]
```

`GET /v1/cash-flow`

Returns the cash-flow projection for the workspace tied to the token: a series of time buckets (realised + projected income/expenses, running balance) plus `current_balance`, the `today_bucket` seam and `retroactive_overdue`. A credit card's account (see `/credit-cards`) stays out of the cash: its purchases never enter, the realised cash is the statement payment leaving the paying account on the payment date, and an open or future statement with no payable yet is projected by its expected total on its due date (once closed, its payable takes over). Requires scope `finance/cash_flow` with access level `read`. Read-only.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `from` | `string` | sim | Window start (ISO 8601). Required. |
| `to` | `string` | sim | Window end (ISO 8601). Required. |
| `granularity` | `string` | não | Bucket size. Default `month`. A range over the per-granularity cap is clipped (`truncated: true`). Valores: `day`, `week`, `month`, `year`. |
| `company_id` | `string` | não | Scope to one company. Omit for the consolidated (all-company) projection. |
| `account_id` | `string` | não | Scope to one financial account. Omit for all accounts. A credit card's account is not cash, so scoped to it every bucket and `current_balance` are zero — its statements count on the paying account. |
| `tz` | `string` | não | IANA time zone that anchors the `today` seam (e.g. `America/Sao_Paulo`, the default). Only the seam is tz-aware; bucketing stays UTC. An unrecognized value falls back to the default. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The cash-flow projection. |
| `400` | Malformed query (missing/invalid `from`/`to`, bad granularity, non-uuid scope id, unknown param). |
| `404` | The `company_id` or `account_id` is not in the tenant. |
