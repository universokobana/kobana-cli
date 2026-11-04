---
id: organization
title: "kobana dashboards organization"
sidebar_label: "organization"
description: "Get current organization"
---

# kobana dashboards organization

Get current organization

Commands of `kobana dashboards organization`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Get current organization |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Update current organization |

## organization

Get current organization

```text
kobana dashboards organization [OPTIONS] [COMMAND]
```

`GET /v1/organization`

Returns the data of the organization the authenticated token belongs to. There is no `:id` parameter — the organization is always resolved from the API key.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Current organization detail. |
| `404` | Resource not found in the current organization. |

## organization update

Update current organization

```text
kobana dashboards organization update [OPTIONS]
```

`PATCH /v1/organization`

Updates the profile of the authenticated organization. Only `addresses` and `custom_metadata` are editable (`custom_metadata: null` keeps the current value). `name`, `email` and `phone` mirror the Kobana Account: only the sync with the Kobana Account writes them, and a body that carries them answers 200 without changing them. Identity (`id`, `public_id`, `external_id`, `status`, `slug`) is ignored as well. Commercial fields (such as `bypass_subscription`, `courtesy` and `monthly_cost_limit`) are set by Kobana and answer 403.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `addresses` | `array<object>` | no | List of organization addresses. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema dashboards.organization.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Organization updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
