---
id: agent-runs
title: "kobana inbox agent-runs"
sidebar_label: "agent-runs"
---

# kobana inbox agent-runs


Commands of `kobana inbox agent-runs`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`agent-runs list`](#agent-runs-list) | `GET /v1/agent-runs` | List agent runs |
| [`agent-runs get`](#agent-runs-get) | `GET /v1/agent-runs/{id}` | Get agent run |

## agent-runs list

List agent runs

```text
kobana inbox agent-runs list [OPTIONS]
```

`GET /v1/agent-runs`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `updated_at`, `total_cost`, `status`. Without it the order is `created_at` descending. Values: `created_at`, `updated_at`, `total_cost`, `status`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `status` | `string` | no | Equality. Also `status[in]=a,b`. Values: `running`, `success`, `failed`. |
| `agent_id` | `string` | no | Equality. Also `agent_id[in]=a,b`. |
| `email_id` | `string` | no | Equality. Also `email_id[in]=a,b`. |
| `inbox_id` | `string` | no | Equality. Also `inbox_id[in]=a,b`. |
| `llm_model` | `string` | no | Equality. Also `llm_model[in]=a,b`. |
| `created_at` | `string` | no | Equality. Also `created_at[in]=a,b` and `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `finished_at` | `string` | no | Equality. Also `finished_at[in]=a,b` and `finished_at[gt\|gte\|lt\|lte]`. |
| `finished_at[gte]` | `string` | no |  |
| `finished_at[lte]` | `string` | no |  |
| `total_cost` | `number` | no | Equality. Also `total_cost[in]=a,b` and `total_cost[gt\|gte\|lt\|lte]`. |
| `total_cost[gte]` | `number` | no |  |
| `total_cost[lte]` | `number` | no |  |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of results. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |

## agent-runs get

Get agent run

```text
kobana inbox agent-runs get [ID] [OPTIONS]
```

`GET /v1/agent-runs/{id}`

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `404` | Resource not found or filtered out by soft-delete. |
