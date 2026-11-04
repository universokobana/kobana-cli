---
id: data-purges
title: "kobana dashboards data-purges"
sidebar_label: "data-purges"
---

# kobana dashboards data-purges


Commands of `kobana dashboards data-purges`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`data-purges list`](#data-purges-list) | `GET /v1/data_purges` | Count purgeable data |
| [`data-purges delete`](#data-purges-delete) | `DELETE /v1/data_purges` | Purge organization data |
| [`data-purges get`](#data-purges-get) | `GET /v1/data_purges/{id}` | Get purge job |

## data-purges list

Count purgeable data

```text
kobana dashboards data-purges list [OPTIONS]
```

`GET /v1/data_purges`

How many rows each purgeable resource (`portal_users`, `people`, `usage_history`) holds across every workspace. Requires `data.purge` (organization owners only by default) and access to every workspace.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The counts. |
| `404` | Resource not found in the current organization. |

## data-purges delete

Purge organization data

```text
kobana dashboards data-purges delete [OPTIONS]
```

`DELETE /v1/data_purges`

IRREVERSIBLE: permanently deletes the chosen resources in EVERY workspace of the organization. In the dashboard the operation is confirmed with a code sent by e-mail. That is why a token that names a person (a personal or OAuth token, or a call through the MCP) gets 403: the person purges from the dashboard. Only machine tokens can purge over the API, and only with the `delete` verb (or `all`) — `write` is not enough; with resource scopes, the token must also reach `dashboards/admin` (or `dashboards/all`); without that, 403 `insufficient_scope`. Their confirmation is `confirm`, which must repeat the organization id (from `GET /organization`) — a retried or mis-targeted call cannot pass by accident. Requires `data.purge` (organization owners only by default) and access to every workspace. Asynchronous: 202 with `job_id`. It is a `DELETE` with a JSON body (`resources`, `confirm`) — until 2026-10-29 it was a `POST`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `resources` | `array<string>` | yes | What to purge: `portal_users`, `people`, `usage_history`. |
| `confirm` | `string` | yes | Confirmation: the organization id, typed back. |

More fields and nested objects: `kobana schema dashboards.data-purges.delete`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | Purge queued; follow it on `GET /data_purges/{id}`. |
| `400` | Missing body, invalid JSON or incompatible fields. |
| `404` | Resource not found in the current organization. |
| `422` | `confirm` does not match the organization id — nothing was purged. |

## data-purges get

Get purge job

```text
kobana dashboards data-purges get [ID] [OPTIONS]
```

`GET /v1/data_purges/{id}`

Progress and result of a purge job. A job of another organization: 404.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The job. |
| `404` | Resource not found in the current organization. |
