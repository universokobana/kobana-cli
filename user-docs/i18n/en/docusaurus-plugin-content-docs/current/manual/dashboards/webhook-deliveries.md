---
id: webhook-deliveries
title: "kobana dashboards webhook-deliveries"
sidebar_label: "webhook-deliveries"
---

# kobana dashboards webhook-deliveries


Commands of `kobana dashboards webhook-deliveries`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`webhook-deliveries list`](#webhook-deliveries-list) | `GET /v1/webhook_deliveries` | List webhook deliveries |
| [`webhook-deliveries get`](#webhook-deliveries-get) | `GET /v1/webhook_deliveries/{id}` | Get a webhook delivery |

## webhook-deliveries list

List webhook deliveries

```text
kobana dashboards webhook-deliveries list [OPTIONS]
```

`GET /v1/webhook_deliveries`

Deliveries of every endpoint of the workspace, newest first, with the event type and an endpoint summary.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `status` | `string` | no | `pending`, `delivered` or `failed`. Values: `pending`, `delivered`, `failed`. |
| `webhook_id` | `string` | no | Only deliveries of this webhook endpoint. |
| `date_from` | `string` | no | Lower bound of `created_at` (ISO 8601 date or date-time). |
| `date_to` | `string` | no | Upper bound of `created_at` (ISO 8601 date or date-time). A bare date covers the whole day (UTC). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | A page of deliveries. |
| `400` | Bad request: malformed body, missing required field or a filter with an invalid value. |
| `404` | Resource not found in the current organization. |

## webhook-deliveries get

Get a webhook delivery

```text
kobana dashboards webhook-deliveries get [ID] [OPTIONS]
```

`GET /v1/webhook_deliveries/{id}`

Returns one delivery: the (redacted) request, the response, attempts and the target endpoint.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The delivery. |
| `404` | Resource not found in the current organization. |
