---
id: ai-usage
title: "kobana dashboards ai-usage"
sidebar_label: "ai-usage"
description: "Get AI usage"
---

# kobana dashboards ai-usage

Get AI usage

Commands of `kobana dashboards ai-usage`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`ai-usage`](#ai-usage) | `GET /v1/ai_usage` | Get AI usage |

## ai-usage

Get AI usage

```text
kobana dashboards ai-usage [OPTIONS]
```

`GET /v1/ai_usage`

AI consumption, price only (cost and margin never leave): totals, daily breakdown and latest turns of the workspace of the call, plus the month to date, grants, limits, credit balance and whether the next turn is allowed — those for the whole organization, as the limit and the credit are shared. Default period: current month (America/Sao_Paulo), at most 366 days.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `start_date` | `string` | no | First day (YYYY-MM-DD). |
| `end_date` | `string` | no | Last day (YYYY-MM-DD). |
| `days` | `integer` | no | The last N days, instead of a date range. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The AI usage. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
