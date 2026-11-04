---
id: usage
title: "kobana dashboards usage"
sidebar_label: "usage"
description: "Get platform usage"
---

# kobana dashboards usage

Get platform usage

Commands of `kobana dashboards usage`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`usage`](#usage) | `GET /v1/usage` | Get platform usage |

## usage

Get platform usage

```text
kobana dashboards usage [OPTIONS]
```

`GET /v1/usage`

For each metric (members, customers, portal users, integrations…), the daily series in the range, the latest value and the peak. Default: current month; at most 12 months back. Organization-level.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `start_date` | `string` | no | First day (YYYY-MM-DD). Default: day 1 of the current month. |
| `end_date` | `string` | no | Last day (YYYY-MM-DD). Default: today. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The usage. |
| `404` | Resource not found in the current organization. |
