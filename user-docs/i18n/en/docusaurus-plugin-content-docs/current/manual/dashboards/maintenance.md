---
id: maintenance
title: "kobana dashboards maintenance"
sidebar_label: "maintenance"
---

# kobana dashboards maintenance


Commands of `kobana dashboards maintenance`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`maintenance uppercase-names create`](#maintenance-uppercase-names-create) | `POST /v1/maintenance/uppercase_names` | Upper-case contact names |

## maintenance uppercase-names create

Upper-case contact names

```text
kobana dashboards maintenance uppercase-names create [OPTIONS]
```

`POST /v1/maintenance/uppercase_names`

Upper-cases the name of every contact linked to a portal user of the workspace. Not reversible (the previous spelling is not kept).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | How many names changed. |
| `404` | Resource not found in the current organization. |
