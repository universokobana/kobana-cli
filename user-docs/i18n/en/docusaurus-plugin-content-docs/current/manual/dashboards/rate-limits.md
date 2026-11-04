---
id: rate-limits
title: "kobana dashboards rate-limits"
sidebar_label: "rate-limits"
description: "Get rate limits"
---

# kobana dashboards rate-limits

Get rate limits

Commands of `kobana dashboards rate-limits`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`rate-limits`](#rate-limits) | `GET /v1/rate_limits` | Get rate limits |

## rate-limits

Get rate limits

```text
kobana dashboards rate-limits [OPTIONS]
```

`GET /v1/rate_limits`

The rate limits in force for the organization, whether they are custom (set by the Kobana team) and the platform defaults. Read-only.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The limits. |
| `404` | Resource not found in the current organization. |
