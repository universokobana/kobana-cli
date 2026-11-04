---
id: portal-user-mappings
title: "kobana dashboards portal-user-mappings"
sidebar_label: "portal-user-mappings"
description: "Get portal user mapping"
---

# kobana dashboards portal-user-mappings

Get portal user mapping

Commands of `kobana dashboards portal-user-mappings`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`portal-user-mappings`](#portal-user-mappings) | `GET /v1/portal_user_mappings` | Get portal user mapping |
| [`portal-user-mappings update`](#portal-user-mappings-update) | `PUT /v1/portal_user_mappings` | Map entity to portal users |
| [`portal-user-mappings delete`](#portal-user-mappings-delete) | `DELETE /v1/portal_user_mappings` | Remove portal user mapping |

## portal-user-mappings

Get portal user mapping

```text
kobana dashboards portal-user-mappings [OPTIONS] [COMMAND]
```

`GET /v1/portal_user_mappings`

The entity that feeds the workspace portal users (`mapping`, `null` when none, with the last sync outcome) and the entities that can be mapped, with their fields.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The mapping. |
| `404` | Resource not found in the current organization. |

## portal-user-mappings update

Map entity to portal users

```text
kobana dashboards portal-user-mappings update [OPTIONS]
```

`PUT /v1/portal_user_mappings`

Maps an entity: one portal user per e-mail of its records, with the tenant ID taken from a field. Replaces the previous mapping and syncs right away (returns `stats`); later syncs of the entity keep the users current.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `entity_type_id` | `string (uuid)` | yes | The entity whose records become portal users. |
| `tenant_id_field_key` | `string` | yes |  |
| `email_field_key` | `string` | yes |  |
| `name_field_key` | `string` | no |  |

More fields and nested objects: `kobana schema dashboards.portal-user-mappings.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The saved mapping and the sync outcome. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Invalid entity or field (e.g. an entity that syncs on access). |

## portal-user-mappings delete

Remove portal user mapping

```text
kobana dashboards portal-user-mappings delete [OPTIONS]
```

`DELETE /v1/portal_user_mappings`

Stops syncing; users already created stay.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Mapping removed (`deleted: false` when there was none). |
| `404` | Resource not found in the current organization. |
