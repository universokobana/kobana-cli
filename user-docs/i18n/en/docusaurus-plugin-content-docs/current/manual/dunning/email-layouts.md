---
id: email-layouts
title: "kobana dunning email-layouts"
sidebar_label: "email-layouts"
description: "List email layouts"
---

# kobana dunning email-layouts

List email layouts

Commands of `kobana dunning email-layouts`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`email-layouts`](#email-layouts) | `GET /v1/email-layouts` | List email layouts |
| [`email-layouts create`](#email-layouts-create) | `POST /v1/email-layouts` | Create email layout |

## email-layouts

List email layouts

```text
kobana dunning email-layouts [OPTIONS] [COMMAND]
```

`GET /v1/email-layouts`

Lists the organization’s email layouts (default first) and the built-in reference layout.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Layout list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## email-layouts create

Create email layout

```text
kobana dunning email-layouts create [OPTIONS]
```

`POST /v1/email-layouts`

Creates an email layout. The `html_body` must contain the `{{content}}` placeholder (otherwise, 400 `MISSING_CONTENT_PLACEHOLDER`). `isDefault: true` unsets the previous default.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Layout name. |
| `html_body` | `string` | yes | Full layout HTML; must contain the `{{content}}` placeholder. |
| `is_default` | `boolean` | no | Organization default layout (only one; setting a new default unsets the previous one). |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |

More fields and nested objects: `kobana schema dunning.email-layouts.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Layout created. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
