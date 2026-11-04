---
id: events
title: "kobana dashboards events"
sidebar_label: "events"
---

# kobana dashboards events


Commands of `kobana dashboards events`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`events list`](#events-list) | `GET /v1/events` | List events |
| [`events get`](#events-get) | `GET /v1/events/{id}` | Get an event |

## events list

List events

```text
kobana dashboards events list [OPTIONS]
```

`GET /v1/events`

Event log of the workspace and of the organization, newest first. Events are evidence: they cannot be changed or deleted.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `type` | `string` | no | Exact event type (e.g. `api_key.rotated`). |
| `resource_type` | `string` | no | Exact resource type (e.g. `api_key`). |
| `resource_id` | `string` | no | Only events about this record. |
| `date_from` | `string` | no | Lower bound of `created_at` (ISO 8601 date or date-time). |
| `date_to` | `string` | no | Upper bound of `created_at` (ISO 8601 date or date-time). A bare date covers the whole day (UTC). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | A page of events. |
| `400` | Bad request: malformed body, missing required field or a filter with an invalid value. |
| `404` | Resource not found in the current organization. |

## events get

Get an event

```text
kobana dashboards events get [ID] [OPTIONS]
```

`GET /v1/events/{id}`

Returns one event with its full payload.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The event. |
| `404` | Resource not found in the current organization. |
