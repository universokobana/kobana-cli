---
id: webhooks
title: "kobana dashboards webhooks"
sidebar_label: "webhooks"
---

# kobana dashboards webhooks


Commands of `kobana dashboards webhooks`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`webhooks deliveries retry`](#webhooks-deliveries-retry) | `POST /v1/webhooks/{id}/deliveries/{delivery_id}/retry` | Retry a webhook delivery |
| [`webhooks deliveries list`](#webhooks-deliveries-list) | `GET /v1/webhooks/{id}/deliveries` | List the deliveries of a webhook |
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | List webhook endpoints |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Create a webhook endpoint |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Get a webhook endpoint |
| [`webhooks update`](#webhooks-update) | `PATCH /v1/webhooks/{id}` | Update a webhook endpoint |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Delete a webhook endpoint |
| [`webhooks rotate-secret`](#webhooks-rotate-secret) | `POST /v1/webhooks/{id}/rotate_secret` | Rotate the webhook secret |
| [`webhooks test`](#webhooks-test) | `POST /v1/webhooks/{id}/test` | Send a test webhook |

## webhooks deliveries retry

Retry a webhook delivery

```text
kobana dashboards webhooks deliveries retry [ID] [DELIVERY_ID] [OPTIONS]
```

`POST /v1/webhooks/{id}/deliveries/{delivery_id}/retry`

Puts the delivery back to `pending` and queues it to be sent again. Sending is asynchronous: follow the outcome in the delivery log.

**Path arguments:** `id`, `delivery_id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `202` | Delivery queued to be sent again. |
| `404` | Resource not found in the current organization. |

## webhooks deliveries list

List the deliveries of a webhook

```text
kobana dashboards webhooks deliveries list [ID] [OPTIONS]
```

`GET /v1/webhooks/{id}/deliveries`

Delivery log of one endpoint, newest first.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `status` | `string` | no | `pending`, `delivered` or `failed`. Values: `pending`, `delivered`, `failed`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | A page of deliveries. |
| `404` | Resource not found in the current organization. |

## webhooks list

List webhook endpoints

```text
kobana dashboards webhooks list [OPTIONS]
```

`GET /v1/webhooks`

Lists the webhook endpoints of the workspace, newest first, each with its delivery count.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `status` | `string` | no | `active` or `inactive`. Values: `active`, `inactive`. |
| `search` | `string` | no | Substring of the URL or the description, case-insensitive. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | A page of webhook endpoints. |
| `404` | Resource not found in the current organization. |

## webhooks create

Create a webhook endpoint

```text
kobana dashboards webhooks create [OPTIONS]
```

`POST /v1/webhooks`

Creates an active endpoint in the workspace. The response carries the signing `secret` — it only appears here (and on rotation).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `url` | `string` | yes | Endpoint URL (required on creation). Must be a valid URL. |
| `events` | `array<string>` | yes | Event types to receive. At least one. |
| `description` | `string` | no | Free-form description. `null` clears it. |
| `auth_type` | `string` | no | Delivery authentication (default `none`). When changing it, send the matching `auth_config` too. Values: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | no | Credentials for `auth_type`: basic → `{ username, password }`, bearer → `{ token }`, header → `{ key, value }`. Stored encrypted and never returned. |

More fields and nested objects: `kobana schema dashboards.webhooks.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Endpoint created, with its signing secret. |
| `400` | Bad request: malformed body, missing required field or a filter with an invalid value. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## webhooks get

Get a webhook endpoint

```text
kobana dashboards webhooks get [ID] [OPTIONS]
```

`GET /v1/webhooks/{id}`

Returns one webhook endpoint of the workspace. Its deliveries are at `GET /webhooks/{id}/deliveries`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The webhook endpoint. |
| `404` | Resource not found in the current organization. |

## webhooks update

Update a webhook endpoint

```text
kobana dashboards webhooks update [ID] [OPTIONS]
```

`PATCH /v1/webhooks/{id}`

Changes the URL, description, subscribed events, `active` or the authentication. Absent fields are left as they are. When `url` moves to another origin, path or query string (parameter order aside) and `auth_type` is not sent in the same request, the stored credentials are dropped (`auth_type` becomes `none`): they were given for the previous receiver. The signing secret is kept — call `rotate_secret` if the endpoint changed hands.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `url` | `string` | no | Endpoint URL (required on creation). Must be a valid URL. |
| `events` | `array<string>` | no | Event types to receive. At least one. |
| `description` | `string` | no | Free-form description. `null` clears it. |
| `active` | `boolean` | no | `false` pauses the endpoint; `true` resumes it. |
| `auth_type` | `string` | no | Delivery authentication (default `none`). When changing it, send the matching `auth_config` too. Values: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | no | Credentials for `auth_type`: basic → `{ username, password }`, bearer → `{ token }`, header → `{ key, value }`. Stored encrypted and never returned. |

More fields and nested objects: `kobana schema dashboards.webhooks.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Endpoint updated. |
| `400` | Bad request: malformed body, missing required field or a filter with an invalid value. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## webhooks delete

Delete a webhook endpoint

```text
kobana dashboards webhooks delete [ID] [OPTIONS]
```

`DELETE /v1/webhooks/{id}`

Deletes the endpoint for good, together with its delivery log. Events stop being sent immediately.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Endpoint deleted. |
| `404` | Resource not found in the current organization. |

## webhooks rotate-secret

Rotate the webhook secret

```text
kobana dashboards webhooks rotate-secret [ID] [OPTIONS]
```

`POST /v1/webhooks/{id}/rotate_secret`

Generates a new signing secret. The previous one stops working immediately; the new `secret` only appears in this response.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Endpoint with its new secret. |
| `404` | Resource not found in the current organization. |

## webhooks test

Send a test webhook

```text
kobana dashboards webhooks test [ID] [OPTIONS]
```

`POST /v1/webhooks/{id}/test`

Sends a signed `test.webhook` event to the endpoint URL now and records it in the delivery log. Answers 200 with `success: false` when the endpoint answers non-2xx or cannot be reached, and 422 when the URL points to a blocked (private network) address.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Outcome of the attempt. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
