---
id: me
title: "kobana dashboards me"
sidebar_label: "me"
description: "Current token identity"
---

# kobana dashboards me

Current token identity

Commands of `kobana dashboards me`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Current token identity |

## me

Current token identity

```text
kobana dashboards me [OPTIONS]
```

`GET /v1/me`

Returns who is behind the Bearer token: the person (`user`, `null` for a machine token), the organization, and the token's own scopes, access level and expiry. Use it to check that the credential configured in an integration is valid. It never returns the token, hashes or keys.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Identity of the authenticated token. |
| `404` | Resource not found in the current organization. |
