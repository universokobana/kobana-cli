---
id: settings
title: "kobana dashboards settings"
sidebar_label: "settings"
description: "Get organization settings"
---

# kobana dashboards settings

Get organization settings

Commands of `kobana dashboards settings`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`settings`](#settings) | `GET /v1/settings` | Get organization settings |
| [`settings update`](#settings-update) | `PUT /v1/settings` | Update organization settings |

## settings

Get organization settings

```text
kobana dashboards settings [OPTIONS] [COMMAND]
```

`GET /v1/settings`

Name, support e-mail, timezone and the records options of the organization. Organization-level.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The settings. |
| `404` | Resource not found in the current organization. |

## settings update

Update organization settings

```text
kobana dashboards settings update [OPTIONS]
```

`PUT /v1/settings`

Partially changes the settings: send only what changes. Unknown fields are refused (400).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no |  |
| `support_email` | `string (email)` | no |  |
| `timezone` | `string` | no | IANA timezone, e.g. `America/Sao_Paulo`. |
| `crm_uppercase_names` | `boolean` | no | Save names in UPPERCASE. |
| `crm_allow_edit_public_id` | `boolean` | no |  |
| `crm_allow_edit_document` | `boolean` | no |  |
| `crm_allow_create_customers` | `boolean` | no |  |
| `crm_allow_create_portal_users` | `boolean` | no | Allow creating portal users from the dashboard and by import. |
| `crm_allow_edit_portal_user_email` | `boolean` | no |  |

More fields and nested objects: `kobana schema dashboards.settings.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The settings. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
