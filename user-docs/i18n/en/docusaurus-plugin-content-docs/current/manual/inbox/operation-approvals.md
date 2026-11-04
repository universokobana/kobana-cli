---
id: operation-approvals
title: "kobana inbox operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana inbox operation-approvals


Commands of `kobana inbox operation-approvals`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation-approvals` | List operation approvals |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation-approvals/{id}` | Get operation approval |

## operation-approvals list

List operation approvals

```text
kobana inbox operation-approvals list [OPTIONS]
```

`GET /v1/operation-approvals`

The operation approvals a destructive MCP tool asked for, newest first, page-based. Only the caller's own approvals — the person behind the token — in the token's organization and the call's workspace. Requires no resource scope, like `/v1/me`: the data is the person's own. A token without a person has none. The approval link, its token and the stored card are never returned.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Only approvals in this status; a lapsed pending approval counts as `expired`. Values: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | no | Requested at or after this ISO 8601 timestamp. |
| `created_before` | `string` | no | Requested at or before this ISO 8601 timestamp. |
| `page` | `integer` | no | Page number, default 1. |
| `per_page` | `integer` | no | Page size, default 25, max 100. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of approvals. |
| `400` | A query parameter is invalid. |

## operation-approvals get

Get operation approval

```text
kobana inbox operation-approvals get [ID] [OPTIONS]
```

`GET /v1/operation-approvals/{id}`

One operation approval: its status, title, dates and outcome. Another person's, another organization's or an unknown id answers the same `404 NOT_FOUND`. Only the caller's own approvals — the person behind the token — in the token's organization and the call's workspace. Requires no resource scope, like `/v1/me`: the data is the person's own. A token without a person has none. The approval link, its token and the stored card are never returned.

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
