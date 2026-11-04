---
id: email-templates
title: "kobana dashboards email-templates"
sidebar_label: "email-templates"
---

# kobana dashboards email-templates


Commands of `kobana dashboards email-templates`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`email-templates preview`](#email-templates-preview) | `GET /v1/email_templates/{id}/preview` | Preview email template |
| [`email-templates preview create`](#email-templates-preview-create) | `POST /v1/email_templates/{id}/preview` | Render template with variables |
| [`email-templates list`](#email-templates-list) | `GET /v1/email_templates` | List email templates |
| [`email-templates create`](#email-templates-create) | `POST /v1/email_templates` | Create email template |
| [`email-templates init-defaults`](#email-templates-init-defaults) | `POST /v1/email_templates/init_defaults` | Reset templates to defaults |
| [`email-templates get`](#email-templates-get) | `GET /v1/email_templates/{id}` | Get email template |
| [`email-templates update`](#email-templates-update) | `PATCH /v1/email_templates/{id}` | Update email template |
| [`email-templates delete`](#email-templates-delete) | `DELETE /v1/email_templates/{id}` | Delete email template |
| [`email-templates duplicate`](#email-templates-duplicate) | `POST /v1/email_templates/{id}/duplicate` | Duplicate email template |

## email-templates preview

Preview email template

```text
kobana dashboards email-templates preview [ID] [OPTIONS] [COMMAND]
```

`GET /v1/email_templates/{id}/preview`

Renders the template with sample data, inside the organization's layout. Nothing is sent.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The rendered email. |
| `404` | Resource not found in the current organization. |

## email-templates preview create

Render template with variables

```text
kobana dashboards email-templates preview create [ID] [OPTIONS]
```

`POST /v1/email_templates/{id}/preview`

Renders the template with the `variables` sent, without a layout. Nothing is sent.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `variables` | `object` | no | Values for the template `{{name}}` variables. |

More fields and nested objects: `kobana schema dashboards.email-templates.preview.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The rendered email. |
| `400` | Invalid request (missing required field, invalid type or variables). |
| `404` | Resource not found in the current organization. |

## email-templates list

List email templates

```text
kobana dashboards email-templates list [OPTIONS]
```

`GET /v1/email_templates`

Lists the organization's own email templates, ordered by type and name. Returns the whole set, unpaginated.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `type` | `string` | no | Only templates of this notification type. Values: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `is_active` | `string` | no | `true` for the active templates, `false` for the drafts. Values: `true`, `false`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The organization's templates. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## email-templates create

Create email template

```text
kobana dashboards email-templates create [OPTIONS]
```

`POST /v1/email_templates`

Creates a template for a notification type. It is a draft unless `is_active` is `true` — then the previous active template of the type becomes a draft.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Template name (required on create). |
| `type` | `string` | yes | Notification type the template renders (required). Values: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `subject` | `string` | yes | Email subject, with `{{name}}` variables (required on create). |
| `html_body` | `string` | yes | HTML body, with `{{name}}` variables (required on create). |
| `text_body` | `string` | no | Plain-text alternative body. `null` clears it. |
| `bcc` | `anyOf` | no | List (or a single address) of BCC emails. Invalid addresses return 422; omitted on update, the current list is kept. |
| `variables` | `array<string>` | no | Names of the variables the template uses. |
| `is_active` | `boolean` | no | Makes this the active template of its type; the previous active one becomes a draft. Defaults to `false` on create. |

More fields and nested objects: `kobana schema dashboards.email-templates.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Template created. |
| `400` | Invalid request (missing required field, invalid type or variables). |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## email-templates init-defaults

Reset templates to defaults

```text
kobana dashboards email-templates init-defaults [OPTIONS]
```

`POST /v1/email_templates/init_defaults`

Creates or resets the template of every notification type to the shipped default and activates it. Overwrites the content of the organization's existing templates.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | One row per notification type. |
| `404` | Resource not found in the current organization. |

## email-templates get

Get email template

```text
kobana dashboards email-templates get [ID] [OPTIONS]
```

`GET /v1/email_templates/{id}`

Returns a template of the organization or a platform default.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The template. |
| `404` | Resource not found in the current organization. |

## email-templates update

Update email template

```text
kobana dashboards email-templates update [ID] [OPTIONS]
```

`PATCH /v1/email_templates/{id}`

Updates a template of the organization. Only the fields sent change. Platform defaults cannot be changed (404).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Template name (required on create). |
| `subject` | `string` | no | Email subject, with `{{name}}` variables (required on create). |
| `html_body` | `string` | no | HTML body, with `{{name}}` variables (required on create). |
| `text_body` | `string` | no | Plain-text alternative body. `null` clears it. |
| `bcc` | `anyOf` | no | List (or a single address) of BCC emails. Invalid addresses return 422; omitted on update, the current list is kept. |
| `variables` | `array<string>` | no | Names of the variables the template uses. |
| `is_active` | `boolean` | no | Makes this the active template of its type; the previous active one becomes a draft. Defaults to `false` on create. |
| `public_id` | `string (uuid)` | no | New public identifier (UUID), when the organization allows choosing it. |

More fields and nested objects: `kobana schema dashboards.email-templates.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Template updated. |
| `400` | Invalid request (missing required field, invalid type or variables). |
| `404` | Resource not found in the current organization. |
| `409` | The `public_id` sent is already in use. |
| `422` | Request body failed validation. |

## email-templates delete

Delete email template

```text
kobana dashboards email-templates delete [ID] [OPTIONS]
```

`DELETE /v1/email_templates/{id}`

Permanently deletes an inactive template of the organization. The active template of a type cannot be deleted (422): activate another one first.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Template deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## email-templates duplicate

Duplicate email template

```text
kobana dashboards email-templates duplicate [ID] [OPTIONS]
```

`POST /v1/email_templates/{id}/duplicate`

Copies a template as an inactive draft.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `new_name` | `string` | no | Name of the copy. Default: "&lt;name> (Cópia)". |

More fields and nested objects: `kobana schema dashboards.email-templates.duplicate`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | The copy created. |
| `400` | Invalid request (missing required field, invalid type or variables). |
| `404` | Resource not found in the current organization. |
