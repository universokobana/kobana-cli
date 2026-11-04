---
id: comments
title: "kobana dashboards comments"
sidebar_label: "comments"
---

# kobana dashboards comments


Commands of `kobana dashboards comments`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`comments list`](#comments-list) | `GET /v1/comments` | List comments |
| [`comments create`](#comments-create) | `POST /v1/comments` | Create comment |
| [`comments get`](#comments-get) | `GET /v1/comments/{id}` | Get comment |
| [`comments delete`](#comments-delete) | `DELETE /v1/comments/{id}` | Delete comment |

## comments list

List comments

```text
kobana dashboards comments list [OPTIONS]
```

`GET /v1/comments`

The comments of a tenant's conversation (`tenant_id` or `portal_user_id`) or, without either, of every conversation of the workspace. Newest first, paginated.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `tenant_id` | `string` | no | Tenant ID of the portal users who own the conversation — every user of the same tenant sees the same comments. Give this or `portal_user_id`. |
| `portal_user_id` | `string` | no | A portal user (id or external_id): the conversation of their tenant is the one meant (a user without a tenant has a conversation of their own). Give this or `tenant_id`. |
| `internal` | `string` | no | `true` = internal notes only; `false` = only what the portal shows. Values: `true`, `false`. |
| `author_type` | `string` | no | Only comments written by `portal_user` (portal user), `member` (team member) or `api`. Values: `portal_user`, `member`, `api`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Comments. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## comments create

Create comment

```text
kobana dashboards comments create [OPTIONS]
```

`POST /v1/comments`

Adds a comment to a tenant's conversation, on behalf of the team. Every portal user of that tenant sees it in the Comments block — unless `internal` is `true`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `body` | `string` | yes | The message (up to 5000 characters). |
| `tenant_id` | `string` | no | Tenant ID of the portal users who own the conversation — every user of the same tenant sees the same comments. Give this or `portal_user_id`. |
| `portal_user_id` | `string` | no | A portal user (id or external_id): the conversation of their tenant is the one meant (a user without a tenant has a conversation of their own). Give this or `tenant_id`. |
| `internal` | `boolean` | no | Internal note: only the team sees it (dashboard, API, MCP); the portal never shows it. |
| `author_name` | `string` | no | Name shown next to the message. On create, it defaults to the token operator's e-mail or "Equipe". |

More fields and nested objects: `kobana schema dashboards.comments.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Comment created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## comments get

Get comment

```text
kobana dashboards comments get [ID] [OPTIONS]
```

`GET /v1/comments/{id}`

One comment.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The comment. |
| `404` | Resource not found in the current organization. |

## comments delete

Delete comment

```text
kobana dashboards comments delete [ID] [OPTIONS]
```

`DELETE /v1/comments/{id}`

Removes the comment from the conversation (soft delete): it leaves the portal and the dashboard for everyone.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Comment deleted. |
| `404` | Resource not found in the current organization. |
