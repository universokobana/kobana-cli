---
id: secrets
title: "kobana inbox secrets"
sidebar_label: "secrets"
---

# kobana inbox secrets


Commands of `kobana inbox secrets`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`secrets list`](#secrets-list) | `GET /v1/secrets` | List secrets |
| [`secrets create`](#secrets-create) | `POST /v1/secrets` | Create secret |
| [`secrets update`](#secrets-update) | `PUT /v1/secrets` | Create or update secret |
| [`secrets get`](#secrets-get) | `GET /v1/secrets/{id}` | Get secret |
| [`secrets delete`](#secrets-delete) | `DELETE /v1/secrets/{id}` | Delete secret |

## secrets list

List secrets

```text
kobana inbox secrets list [OPTIONS]
```

`GET /v1/secrets`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`. The value is write-only: it is stored encrypted and is never returned by any endpoint. To rotate it, PATCH the row (or PUT by `key`) with a new `value`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `updated_at`, `key`. Without it the order is `created_at` descending. Values: `created_at`, `updated_at`, `key`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `key` | `string` | no | Equality. Also `key[in]=a,b`. |
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

## secrets create

Create secret

```text
kobana inbox secrets create [OPTIONS]
```

`POST /v1/secrets`

Store a secret an agent workflow can reference as `${{secrets.KEY}}`. A duplicate `key` in the workspace is refused with 409 `CONFLICT`. The value is write-only: it is stored encrypted and is never returned by any endpoint. To rotate it, PATCH the row (or PUT by `key`) with a new `value`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | yes |  |
| `value` | `string` | yes | Stored encrypted; never returned. |
| `notes` | `string` | no |  |

More fields and nested objects: `kobana schema inbox.secrets.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## secrets update

Create or update secret

```text
kobana inbox secrets update [OPTIONS]
```

`PUT /v1/secrets`

Upsert matched by `key` within the caller workspace — the rotation path, no id lookup needed. Creates with 201 when the key is new, replaces the value with 200 when it exists. The value is write-only: it is stored encrypted and is never returned by any endpoint. To rotate it, PATCH the row (or PUT by `key`) with a new `value`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | yes | Lookup key of the upsert. |
| `value` | `string` | yes |  |
| `notes` | `string` | no |  |

More fields and nested objects: `kobana schema inbox.secrets.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## secrets get

Get secret

```text
kobana inbox secrets get [ID] [OPTIONS]
```

`GET /v1/secrets/{id}`

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

## secrets delete

Delete secret

```text
kobana inbox secrets delete [ID] [OPTIONS]
```

`DELETE /v1/secrets/{id}`

Soft-delete. Workflow steps referencing the key start failing with `secret not defined` from the next run on.

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
