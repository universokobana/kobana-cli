---
id: webhooks
title: "kobana inbox webhooks"
sidebar_label: "webhooks"
---

# kobana inbox webhooks


Commands of `kobana inbox webhooks`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`webhooks deliveries get`](#webhooks-deliveries-get) | `GET /v1/webhooks/{id}/deliveries/{deliveryId}` | Get webhook delivery |
| [`webhooks deliveries replay`](#webhooks-deliveries-replay) | `POST /v1/webhooks/{id}/deliveries/{deliveryId}/replay` | Replay a webhook delivery |
| [`webhooks deliveries list`](#webhooks-deliveries-list) | `GET /v1/webhooks/{id}/deliveries` | List webhook deliveries |
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | List webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Create webhook |
| [`webhooks update`](#webhooks-update) | `PUT /v1/webhooks` | Create or update webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Get webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Delete webhook |
| [`webhooks regenerate-secret`](#webhooks-regenerate-secret) | `POST /v1/webhooks/{id}/regenerate-secret` | Rotate the signing secret |
| [`webhooks test`](#webhooks-test) | `POST /v1/webhooks/{id}/test` | Send a webhook.ping event |

## webhooks deliveries get

Get webhook delivery

```text
kobana inbox webhooks deliveries get [ID] [DELIVERYID] [OPTIONS]
```

`GET /v1/webhooks/{id}/deliveries/{deliveryId}`

Accepts either the internal UUID or the resource `external_id`.

**Path arguments:** `id`, `deliveryId`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `404` | Resource not found or filtered out by soft-delete. |

## webhooks deliveries replay

Replay a webhook delivery

```text
kobana inbox webhooks deliveries replay [ID] [DELIVERYID] [OPTIONS]
```

`POST /v1/webhooks/{id}/deliveries/{deliveryId}/replay`

Accepts either the internal UUID or the resource `external_id`.

**Path arguments:** `id`, `deliveryId`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `202` | Accepted and queued. |
| `404` | Resource not found or filtered out by soft-delete. |

## webhooks deliveries list

List webhook deliveries

```text
kobana inbox webhooks deliveries list [ID] [OPTIONS]
```

`GET /v1/webhooks/{id}/deliveries`

Accepts either the internal UUID or the resource `external_id`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `attempt`, `status`. Without it the order is `created_at` descending. Values: `created_at`, `attempt`, `status`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `status` | `string` | no | Equality. Also `status[in]=a,b`. Values: `pending`, `success`, `failed`. |
| `attempt` | `integer` | no | Equality. Also `attempt[in]=a,b` and `attempt[gt\|gte\|lt\|lte]`. |
| `attempt[gte]` | `integer` | no |  |
| `attempt[lte]` | `integer` | no |  |
| `response_status` | `integer` | no | Equality. Also `response_status[in]=a,b` and `response_status[gt\|gte\|lt\|lte]`. |
| `response_status[gte]` | `integer` | no |  |
| `response_status[lte]` | `integer` | no |  |
| `system_event_id` | `string` | no | Equality. Also `system_event_id[in]=a,b`. |
| `created_at` | `string` | no | Equality. Also `created_at[in]=a,b` and `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `delivered_at` | `string` | no | Equality. Also `delivered_at[in]=a,b` and `delivered_at[gt\|gte\|lt\|lte]`. |
| `delivered_at[gte]` | `string` | no |  |
| `delivered_at[lte]` | `string` | no |  |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of results. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |

## webhooks list

List webhooks

```text
kobana inbox webhooks list [OPTIONS]
```

`GET /v1/webhooks`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `updated_at`, `label`. Without it the order is `created_at` descending. Values: `created_at`, `updated_at`, `label`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `status` | `string` | no | Equality. Also `status[in]=a,b`. Values: `active`, `inactive`. |
| `inbox_id` | `string` | no | Equality. Also `inbox_id[in]=a,b`. |
| `external_id` | `string` | no | Equality. Also `external_id[in]=a,b`. |
| `created_at` | `string` | no | Equality. Also `created_at[in]=a,b` and `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `updated_at` | `string` | no | Equality. Also `updated_at[in]=a,b` and `updated_at[gt\|gte\|lt\|lte]`. |
| `updated_at[gte]` | `string` | no |  |
| `updated_at[lte]` | `string` | no |  |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of results. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |

## webhooks create

Create webhook

```text
kobana inbox webhooks create [OPTIONS]
```

`POST /v1/webhooks`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `url` | `string (uri)` | yes |  |
| `external_id` | `string` | no | Optional. Defaults to the generated internal `id`. |
| `inbox_id` | `string (uuid)` | no |  |
| `subscribed_events` | `array<string>` | no |  |
| `auth` | `string` | no | Values: `none`, `basic`, `bearer`, `header`. |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |
| `signing_secret` | `string` | no | Optional. Bring your own signing secret: `whsec_` + standard base64 of 24-64 random bytes. Generated (32 bytes) when omitted. Returned in plaintext only in this response. |
| `custom_metadata` | `object` | no | User metadata, free-form. Send `null` to clear it. Max 64 KB serialized, 16 levels deep. |

More fields and nested objects: `kobana schema inbox.webhooks.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## webhooks update

Create or update webhook

```text
kobana inbox webhooks update [OPTIONS]
```

`PUT /v1/webhooks`

Upsert. Takes the same body as POST plus the lookup keys `id` and `external_id`, matched in that order. A match is updated in place and returns 200 — fields absent from the body keep their stored value. No key, or no match, creates the webhook exactly as POST would and returns 201.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `url` | `string (uri)` | yes |  |
| `external_id` | `string` | no | Match on the source-system id. Also stored on the record when it is created. |
| `inbox_id` | `string (uuid)` | no |  |
| `subscribed_events` | `array<string>` | no |  |
| `auth` | `string` | no | Values: `none`, `basic`, `bearer`, `header`. |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |
| `signing_secret` | `string` | no | Optional. Bring your own signing secret: `whsec_` + standard base64 of 24-64 random bytes. Generated (32 bytes) when omitted. Returned in plaintext only in this response. |
| `custom_metadata` | `object` | no | User metadata, free-form. Send `null` to clear it. Max 64 KB serialized, 16 levels deep. |
| `id` | `string (uuid)` | no | Match on the internal id. Checked before `external_id`. |

More fields and nested objects: `kobana schema inbox.webhooks.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## webhooks get

Get webhook

```text
kobana inbox webhooks get [ID] [OPTIONS]
```

`GET /v1/webhooks/{id}`

Accepts either the internal UUID or the resource `external_id`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `404` | Resource not found or filtered out by soft-delete. |

## webhooks delete

Delete webhook

```text
kobana inbox webhooks delete [ID] [OPTIONS]
```

`DELETE /v1/webhooks/{id}`

Accepts either the internal UUID or the resource `external_id`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Deleted (soft delete). |
| `404` | Resource not found or filtered out by soft-delete. |

## webhooks regenerate-secret

Rotate the signing secret

```text
kobana inbox webhooks regenerate-secret [ID] [OPTIONS]
```

`POST /v1/webhooks/{id}/regenerate-secret`

Replaces the signing secret and returns the new value **once**. By default the previous secret keeps signing deliveries for 24 hours (`previous_secret_grace_hours`), during which every delivery carries one signature per secret so the receiver can switch keys without dropping events. Pass `0` to revoke the previous secret immediately (leaked secret). Accepts either the internal UUID or the resource `external_id`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `signing_secret` | `string` | no | Optional. Bring your own signing secret: `whsec_` + standard base64 of 24-64 random bytes. Generated (32 bytes) when omitted. Returned in plaintext only in this response. |
| `previous_secret_grace_hours` | `integer` | no | Hours the previous secret remains valid after the rotation. `0` revokes it immediately. |

More fields and nested objects: `kobana schema inbox.webhooks.regenerate-secret`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | New signing secret (shown once). |
| `404` | Resource not found or filtered out by soft-delete. |

## webhooks test

Send a webhook.ping event

```text
kobana inbox webhooks test [ID] [OPTIONS]
```

`POST /v1/webhooks/{id}/test`

Accepts either the internal UUID or the resource `external_id`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `202` | Accepted and queued. |
| `404` | Resource not found or filtered out by soft-delete. |
