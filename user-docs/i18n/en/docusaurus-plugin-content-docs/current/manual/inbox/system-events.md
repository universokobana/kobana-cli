---
id: system-events
title: "kobana inbox system-events"
sidebar_label: "system-events"
---

# kobana inbox system-events


Commands of `kobana inbox system-events`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`system-events list`](#system-events-list) | `GET /v1/system-events` | List system events |
| [`system-events get`](#system-events-get) | `GET /v1/system-events/{id}` | Get system event |

## system-events list

List system events

```text
kobana inbox system-events list [OPTIONS]
```

`GET /v1/system-events`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `slug`. Without it the order is `created_at` descending. Values: `created_at`, `slug`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `slug` | `string` | no | Equality. Also `slug[in]=a,b`. |
| `inbox_id` | `string` | no | Equality. Also `inbox_id[in]=a,b`. |
| `email_id` | `string` | no | Equality. Also `email_id[in]=a,b`. |
| `created_at` | `string` | no | Equality. Also `created_at[in]=a,b` and `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |

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

## system-events get

Get system event

```text
kobana inbox system-events get [ID] [OPTIONS]
```

`GET /v1/system-events/{id}`

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
