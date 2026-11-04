---
id: api-keys
title: "kobana dashboards api-keys"
sidebar_label: "api-keys"
---

# kobana dashboards api-keys


Commands of `kobana dashboards api-keys`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`api-keys list`](#api-keys-list) | `GET /v1/api_keys` | List API keys |
| [`api-keys create`](#api-keys-create) | `POST /v1/api_keys` | Create API key |
| [`api-keys get`](#api-keys-get) | `GET /v1/api_keys/{id}` | Get API key |
| [`api-keys update`](#api-keys-update) | `PATCH /v1/api_keys/{id}` | Update API key |
| [`api-keys delete`](#api-keys-delete) | `DELETE /v1/api_keys/{id}` | Remove revoked API key |
| [`api-keys revoke`](#api-keys-revoke) | `POST /v1/api_keys/{id}/revoke` | Revoke API key |
| [`api-keys rotate`](#api-keys-rotate) | `POST /v1/api_keys/{id}/rotate` | Rotate API key |

## api-keys list

List API keys

```text
kobana dashboards api-keys list [OPTIONS]
```

`GET /v1/api_keys`

Lists the organization API keys (`kb_…`), without the secret. These keys no longer authenticate the API v1 nor the MCP — both take access tokens issued by the Kobana authorization server; the records remain the organization key inventory (lifecycle, expiry reminders and `api_key.*` webhooks). Organization-level: ignores `X-Workspace-Id`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `status` | `string` | no | Filter by status: `active`, `disabled` or `revoked`. Values: `active`, `disabled`, `revoked`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of API keys. |
| `404` | Resource not found in the current organization. |

## api-keys create

Create API key

```text
kobana dashboards api-keys create [OPTIONS]
```

`POST /v1/api_keys`

Creates an active key, valid for one year unless `expires_at` says otherwise. The secret (`key`) is returned ONCE in this response — only its hash is stored. Requires a person who reaches every workspace (organization owner or granted on each one), as in the dashboard.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Key name (required). |
| `description` | `string` | no |  |
| `source` | `string` | no | Where the key will be used. Default `dashboard`. Values: `api`, `dashboard`, `manage`. |
| `notification_emails` | `array<string>` | no | E-mails that get the expiry reminders. |
| `expires_at` | `string (date-time)` | no | Expiry (ISO 8601 date-time in the future). Default: one year from now. |

More fields and nested objects: `kobana schema dashboards.api-keys.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Key created, with the secret in `key` (shown only now). |
| `400` | Missing body, invalid JSON or incompatible fields. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## api-keys get

Get API key

```text
kobana dashboards api-keys get [ID] [OPTIONS]
```

`GET /v1/api_keys/{id}`

Returns one organization key — metadata and lifecycle stamps, never the secret.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The API key. |
| `404` | Resource not found in the current organization. |

## api-keys update

Update API key

```text
kobana dashboards api-keys update [ID] [OPTIONS]
```

`PATCH /v1/api_keys/{id}`

Edits name, description, source, notification e-mails and expiry, and/or enables ⇄ disables the key with `enabled`. A revoked key is terminal (409).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no |  |
| `description` | `string` | no |  |
| `source` | `string` | no | Values: `api`, `dashboard`, `manage`. |
| `notification_emails` | `array<string>` | no | E-mails that get the expiry reminders. |
| `expires_at` | `string (date-time)` | no | New expiry (ISO 8601); `null` removes the expiry. |
| `enabled` | `boolean` | no | `false` disables the key; `true` re-enables it. |

More fields and nested objects: `kobana schema dashboards.api-keys.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The API key. |
| `400` | Missing body, invalid JSON or incompatible fields. |
| `404` | Resource not found in the current organization. |
| `409` | The current state of the resource does not allow this operation. |
| `422` | Request body failed validation. |

## api-keys delete

Remove revoked API key

```text
kobana dashboards api-keys delete [ID] [OPTIONS]
```

`DELETE /v1/api_keys/{id}`

Removes an ALREADY REVOKED key from the list (soft delete; history is kept). An active or disabled key must be revoked first (409).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Removed. |
| `404` | Resource not found in the current organization. |
| `409` | The current state of the resource does not allow this operation. |

## api-keys revoke

Revoke API key

```text
kobana dashboards api-keys revoke [ID] [OPTIONS]
```

`POST /v1/api_keys/{id}/revoke`

Permanently revokes the key — it cannot be re-enabled. 409 when it is already revoked.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The API key. |
| `404` | Resource not found in the current organization. |
| `409` | The current state of the resource does not allow this operation. |

## api-keys rotate

Rotate API key

```text
kobana dashboards api-keys rotate [ID] [OPTIONS]
```

`POST /v1/api_keys/{id}/rotate`

Issues a successor key (same name and settings) and keeps the previous one working for `grace_days` days (default 7, max 30). Only an active key rotates (409). The new secret is returned ONCE.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `grace_days` | `integer` | no | Days the previous key keeps working (0 to 30, default 7). |

More fields and nested objects: `kobana schema dashboards.api-keys.rotate`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The successor key with its secret in `key`, and `previous_key` with the grace period of the old one. |
| `404` | Resource not found in the current organization. |
| `409` | The current state of the resource does not allow this operation. |
| `422` | Request body failed validation. |
