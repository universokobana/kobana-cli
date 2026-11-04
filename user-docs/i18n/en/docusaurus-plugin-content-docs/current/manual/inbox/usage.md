---
id: usage
title: "kobana inbox usage"
sidebar_label: "usage"
---

# kobana inbox usage


Commands of `kobana inbox usage`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`usage cost`](#usage-cost) | `GET /v1/usage/cost` | Cost by model |
| [`usage summary`](#usage-summary) | `GET /v1/usage/summary` | Usage summary |

## usage cost

Cost by model

```text
kobana inbox usage cost [OPTIONS]
```

`GET /v1/usage/cost`

Cost and tokens per LLM model over the range: totals per model and a per-day series.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `from` | `string` | no | First UTC day, `YYYY-MM-DD`. Defaults to 29 days before `to`. |
| `to` | `string` | no | Last UTC day (inclusive), `YYYY-MM-DD`. Defaults to today. `from`–`to` may span at most 366 days; a wider range is 400 `VALIDATION_ERROR`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |

## usage summary

Usage summary

```text
kobana inbox usage summary [OPTIONS]
```

`GET /v1/usage/summary`

Emails processed, tokens and cost per UTC day over the range (successful agent runs only), plus the running monthly counters against the workspace limits. Cost is USD as billed by the AI Gateway.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `from` | `string` | no | First UTC day, `YYYY-MM-DD`. Defaults to 29 days before `to`. |
| `to` | `string` | no | Last UTC day (inclusive), `YYYY-MM-DD`. Defaults to today. `from`–`to` may span at most 366 days; a wider range is 400 `VALIDATION_ERROR`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
