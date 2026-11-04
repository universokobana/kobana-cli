---
id: permissions
title: "kobana dashboards permissions"
sidebar_label: "permissions"
description: "List permissions"
---

# kobana dashboards permissions

List permissions

Commands of `kobana dashboards permissions`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`permissions`](#permissions) | `GET /v1/permissions` | List permissions |

## permissions

List permissions

```text
kobana dashboards permissions [OPTIONS]
```

`GET /v1/permissions`

Catalog of the dashboard permissions a member or a team role can carry, grouped by resource — the vocabulary of `permissions` on `/dashboard_users` and `/team_roles`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Permission groups. |
| `404` | Resource not found in the current organization. |
