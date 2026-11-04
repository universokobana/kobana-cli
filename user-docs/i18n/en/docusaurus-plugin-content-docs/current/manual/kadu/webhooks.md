---
id: webhooks
title: "kobana kadu webhooks"
sidebar_label: "webhooks"
---

# kobana kadu webhooks


Commands of `kobana kadu webhooks`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | List webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Create webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Get webhook details |
| [`webhooks update`](#webhooks-update) | `PATCH /v1/webhooks/{id}` | Update webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Delete webhooks (soft delete) |
| [`webhooks rotate-secret`](#webhooks-rotate-secret) | `POST /v1/webhooks/{id}/rotate-secret` | Rotate the signing secret |

## webhooks list

List webhooks

```text
kobana kadu webhooks list [OPTIONS]
```

`GET /v1/webhooks`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Field in snake_case: created_at (default), updated_at or a filterable field. |
| `sort_order` | `string` | no | Default: desc. Values: `asc`, `desc`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## webhooks create

Create webhook

```text
kobana kadu webhooks create [OPTIONS]
```

`POST /v1/webhooks`

The URL is checked against the internal network. `signing_secret` is returned only in this response (an idempotency replay returns `null`).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `url` | `string (uri)` | yes |  |
| `auth` | `string` | yes | Values: `basic`, `bearer`, `header`. |
| `workspace_id` | `string (uuid)` | no |  |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |
| `subscribed_events` | `array<string>` | no |  |
| `emails` | `array<object>` | no |  |

More fields and nested objects: `kobana schema kadu.webhooks.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | VALIDATION_ERROR — workspace outside the organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## webhooks get

Get webhook details

```text
kobana kadu webhooks get [ID] [OPTIONS]
```

`GET /v1/webhooks/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## webhooks update

Update webhook

```text
kobana kadu webhooks update [ID] [OPTIONS]
```

`PATCH /v1/webhooks/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no |  |
| `url` | `string (uri)` | no |  |
| `subscribed_events` | `array<string>` | no |  |
| `status` | `string` | no | Values: `active`, `inactive`. |
| `emails` | `array<object>` | no |  |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |

More fields and nested objects: `kobana schema kadu.webhooks.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## webhooks delete

Delete webhooks (soft delete)

```text
kobana kadu webhooks delete [ID] [OPTIONS]
```

`DELETE /v1/webhooks/{id}`

Requires the `delete` level (or `all`): `write` does not delete. Requires the `webhooks.delete` permission. A webhook declared in `kadu.yaml` (`repo_managed`) responds 409 REPO_MANAGED: remove it from there.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## webhooks rotate-secret

Rotate the signing secret

```text
kobana kadu webhooks rotate-secret [ID] [OPTIONS]
```

`POST /v1/webhooks/{id}/rotate-secret`

Returns the new secret only in this response; the previous one keeps signing for 24h (an idempotency replay returns `signing_secret: null`).

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
