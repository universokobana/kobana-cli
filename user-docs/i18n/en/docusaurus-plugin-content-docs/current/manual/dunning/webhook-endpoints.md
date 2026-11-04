---
id: webhook-endpoints
title: "kobana dunning webhook-endpoints"
sidebar_label: "webhook-endpoints"
---

# kobana dunning webhook-endpoints


Commands of `kobana dunning webhook-endpoints`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`webhook-endpoints deliveries retry`](#webhook-endpoints-deliveries-retry) | `POST /v1/webhook-endpoints/{id}/deliveries/{deliveryId}/retry` | Retry delivery |
| [`webhook-endpoints deliveries list`](#webhook-endpoints-deliveries-list) | `GET /v1/webhook-endpoints/{id}/deliveries` | List endpoint deliveries |
| [`webhook-endpoints list`](#webhook-endpoints-list) | `GET /v1/webhook-endpoints` | List webhook endpoints |
| [`webhook-endpoints create`](#webhook-endpoints-create) | `POST /v1/webhook-endpoints` | Create webhook endpoint |
| [`webhook-endpoints get`](#webhook-endpoints-get) | `GET /v1/webhook-endpoints/{id}` | Get webhook endpoint |
| [`webhook-endpoints update`](#webhook-endpoints-update) | `PATCH /v1/webhook-endpoints/{id}` | Update webhook endpoint |
| [`webhook-endpoints delete`](#webhook-endpoints-delete) | `DELETE /v1/webhook-endpoints/{id}` | Delete webhook endpoint |
| [`webhook-endpoints rotate-secret`](#webhook-endpoints-rotate-secret) | `POST /v1/webhook-endpoints/{id}/rotate-secret` | Rotate secret |
| [`webhook-endpoints test`](#webhook-endpoints-test) | `POST /v1/webhook-endpoints/{id}/test` | Send test event |

## webhook-endpoints deliveries retry

Retry delivery

```text
kobana dunning webhook-endpoints deliveries retry [ID] [DELIVERYID] [OPTIONS]
```

`POST /v1/webhook-endpoints/{id}/deliveries/{deliveryId}/retry`

Re-queues a delivery for an immediate retry (status resets to `pending`). Fails with 409 if the endpoint is inactive or deleted.

**Path arguments:** `id`, `deliveryId`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Delivery re-queued. |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## webhook-endpoints deliveries list

List endpoint deliveries

```text
kobana dunning webhook-endpoints deliveries list [ID] [OPTIONS]
```

`GET /v1/webhook-endpoints/{id}/deliveries`

Lists the endpoint’s deliveries (paginated, newest first) with the request/response capture for inspection and debugging.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `status` | `string` | no | Filter by delivery status. Values: `pending`, `success`, `failed`, `exhausted`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated delivery list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## webhook-endpoints list

List webhook endpoints

```text
kobana dunning webhook-endpoints list [OPTIONS]
```

`GET /v1/webhook-endpoints`

Lists the organization’s webhook endpoints with delivery counters by status. `secret` and `auth_config` never appear in the listing.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Endpoint list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## webhook-endpoints create

Create webhook endpoint

```text
kobana dunning webhook-endpoints create [OPTIONS]
```

`POST /v1/webhook-endpoints`

Creates a webhook endpoint. The URL requires HTTPS and is validated against internal addresses (SSRF). The `secret` is returned only in this response — store it safely.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `url` | `string (uri)` | yes | HTTPS destination URL for deliveries (validated against internal addresses — SSRF). |
| `description` | `string` | no | Free-form endpoint description. |
| `events` | `array<string>` | no | Subscribed event types. Empty list = all events. |
| `auth_type` | `string` | no | Request authentication against your endpoint (`none` \| `basic` \| `bearer` \| `header`), in addition to the HMAC signature (always sent). Values: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | no | Credentials for `auth_type`: `basic` → `{"username","password"}`; `bearer` → `{"token"}`; `header` → `{"key","value"}`. Encrypted at rest; never returned on reads. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |

More fields and nested objects: `kobana schema dunning.webhook-endpoints.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Endpoint created (includes `secret` — shown only once). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## webhook-endpoints get

Get webhook endpoint

```text
kobana dunning webhook-endpoints get [ID] [OPTIONS]
```

`GET /v1/webhook-endpoints/{id}`

Returns the endpoint with its latest 20 deliveries. The `secret` is never returned.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Endpoint with recent deliveries. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## webhook-endpoints update

Update webhook endpoint

```text
kobana dunning webhook-endpoints update [ID] [OPTIONS]
```

`PATCH /v1/webhook-endpoints/{id}`

Updates the endpoint (partial). The URL, when sent, is re-validated (HTTPS + SSRF); `auth_config` is encrypted and never returned.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `url` | `string (uri)` | no | HTTPS destination URL for deliveries (validated against internal addresses — SSRF). |
| `description` | `string` | no | Free-form endpoint description. |
| `events` | `array<string>` | no | Subscribed event types. Empty list = all events. |
| `is_active` | `boolean` | no | Active endpoints receive deliveries; inactive ones are skipped by the worker. |
| `auth_type` | `string` | no | Request authentication against your endpoint (`none` \| `basic` \| `bearer` \| `header`), in addition to the HMAC signature (always sent). Values: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | no | Credentials for `auth_type`: `basic` → `{"username","password"}`; `bearer` → `{"token"}`; `header` → `{"key","value"}`. Encrypted at rest; never returned on reads. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.webhook-endpoints.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Endpoint updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## webhook-endpoints delete

Delete webhook endpoint

```text
kobana dunning webhook-endpoints delete [ID] [OPTIONS]
```

`DELETE /v1/webhook-endpoints/{id}`

Soft-deletes the endpoint; pending deliveries stop being processed.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Endpoint deleted. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## webhook-endpoints rotate-secret

Rotate secret

```text
kobana dunning webhook-endpoints rotate-secret [ID] [OPTIONS]
```

`POST /v1/webhook-endpoints/{id}/rotate-secret`

Generates a new HMAC secret; the previous one stops validating immediately. The new secret is returned only in this response — store it safely.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | New secret (shown only once). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## webhook-endpoints test

Send test event

```text
kobana dunning webhook-endpoints test [ID] [OPTIONS]
```

`POST /v1/webhook-endpoints/{id}/test`

Delivers a REAL `webhook.test` event to the endpoint through the same pipeline as business events (queue, HMAC signature, request/response capture, retry) — validate your consumer end to end without waiting for a real event. Delivered only to the target endpoint, regardless of the subscribed event list. Fails with 409 if the endpoint is inactive.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `202` | Test event queued for delivery. |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |
