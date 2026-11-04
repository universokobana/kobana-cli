---
id: workflow-runs
title: "kobana inbox workflow-runs"
sidebar_label: "workflow-runs"
---

# kobana inbox workflow-runs


Commands of `kobana inbox workflow-runs`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`workflow-runs list`](#workflow-runs-list) | `GET /v1/workflow-runs` | List workflow runs |
| [`workflow-runs get`](#workflow-runs-get) | `GET /v1/workflow-runs/{id}` | Get workflow run |

## workflow-runs list

List workflow runs

```text
kobana inbox workflow-runs list [OPTIONS]
```

`GET /v1/workflow-runs`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`. Steps are not embedded in the listing — `GET /v1/workflow-runs/{id}` returns them.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `updated_at`, `status`. Without it the order is `created_at` descending. Values: `created_at`, `updated_at`, `status`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `status` | `string` | no | Equality. Also `status[in]=a,b`. Values: `pending`, `running`, `success`, `failed`. |
| `email_id` | `string` | no | Equality. Also `email_id[in]=a,b`. |
| `inbox_id` | `string` | no | Equality. Also `inbox_id[in]=a,b`. |
| `agent_id` | `string` | no | Equality. Also `agent_id[in]=a,b`. |
| `agent_run_id` | `string` | no | Equality. Also `agent_run_id[in]=a,b`. |
| `created_at` | `string` | no | Equality. Also `created_at[in]=a,b` and `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `finished_at` | `string` | no | Equality. Also `finished_at[in]=a,b` and `finished_at[gt\|gte\|lt\|lte]`. |
| `finished_at[gte]` | `string` | no |  |
| `finished_at[lte]` | `string` | no |  |

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

## workflow-runs get

Get workflow run

```text
kobana inbox workflow-runs get [ID] [OPTIONS]
```

`GET /v1/workflow-runs/{id}`

The run with every step in execution order. Requests and responses are stored with secret values replaced by `[REDACTED]`; bodies are truncated to 4000 characters.

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
