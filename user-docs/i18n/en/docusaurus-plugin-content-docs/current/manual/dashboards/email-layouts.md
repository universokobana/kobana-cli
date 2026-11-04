---
id: email-layouts
title: "kobana dashboards email-layouts"
sidebar_label: "email-layouts"
---

# kobana dashboards email-layouts


Commands of `kobana dashboards email-layouts`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`email-layouts list`](#email-layouts-list) | `GET /v1/email_layouts` | List email layouts |
| [`email-layouts create`](#email-layouts-create) | `POST /v1/email_layouts` | Create email layout |
| [`email-layouts init-defaults`](#email-layouts-init-defaults) | `POST /v1/email_layouts/init_defaults` | Create default layout |
| [`email-layouts get`](#email-layouts-get) | `GET /v1/email_layouts/{id}` | Get email layout |
| [`email-layouts update`](#email-layouts-update) | `PATCH /v1/email_layouts/{id}` | Update email layout |
| [`email-layouts delete`](#email-layouts-delete) | `DELETE /v1/email_layouts/{id}` | Delete email layout |

## email-layouts list

List email layouts

```text
kobana dashboards email-layouts list [OPTIONS]
```

`GET /v1/email_layouts`

Lists the organization's email layouts, default first. Returns the whole set, unpaginated.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The organization's layouts. |
| `404` | Resource not found in the current organization. |

## email-layouts create

Create email layout

```text
kobana dashboards email-layouts create [OPTIONS]
```

`POST /v1/email_layouts`

Creates a layout. The HTML must contain the `{{content}}` placeholder.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Layout name (required on create). |
| `html_body` | `string` | yes | HTML document containing the `{{content}}` placeholder (required on create; without it, 422). |
| `is_default` | `boolean` | no | Makes this the default layout; the previous default stops being one. |

More fields and nested objects: `kobana schema dashboards.email-layouts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Layout created. |
| `400` | Invalid request (missing required field, invalid type or variables). |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## email-layouts init-defaults

Create default layout

```text
kobana dashboards email-layouts init-defaults [OPTIONS]
```

`POST /v1/email_layouts/init_defaults`

Makes sure the organization has a default layout: returns the current one or creates the built-in one. Never overwrites a layout.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The default layout. |
| `404` | Resource not found in the current organization. |

## email-layouts get

Get email layout

```text
kobana dashboards email-layouts get [ID] [OPTIONS]
```

`GET /v1/email_layouts/{id}`

Returns a layout of the organization.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The layout. |
| `404` | Resource not found in the current organization. |

## email-layouts update

Update email layout

```text
kobana dashboards email-layouts update [ID] [OPTIONS]
```

`PATCH /v1/email_layouts/{id}`

Updates the name, HTML (which must keep `{{content}}`), default or active flag. Only the fields sent change.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Layout name (required on create). |
| `html_body` | `string` | no | HTML document containing the `{{content}}` placeholder (required on create; without it, 422). |
| `is_default` | `boolean` | no | Makes this the default layout; the previous default stops being one. |
| `is_active` | `boolean` | no | Activates or deactivates the layout. |

More fields and nested objects: `kobana schema dashboards.email-layouts.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Layout updated. |
| `400` | Invalid request (missing required field, invalid type or variables). |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## email-layouts delete

Delete email layout

```text
kobana dashboards email-layouts delete [ID] [OPTIONS]
```

`DELETE /v1/email_layouts/{id}`

Permanently deletes a layout. Emails fall back to the default layout (or the built-in one).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Layout deleted. |
| `404` | Resource not found in the current organization. |
