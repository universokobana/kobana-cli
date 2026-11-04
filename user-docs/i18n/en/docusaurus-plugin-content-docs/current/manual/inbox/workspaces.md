---
id: workspaces
title: "kobana inbox workspaces"
sidebar_label: "workspaces"
---

# kobana inbox workspaces


Commands of `kobana inbox workspaces`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | List workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Create workspace |
| [`workspaces update`](#workspaces-update) | `PUT /v1/workspaces` | Create or update workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Get workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Delete workspace |

## workspaces list

List workspaces

```text
kobana inbox workspaces list [OPTIONS]
```

`GET /v1/workspaces`

Scope: the organization of the token workspace — every live workspace of that organization is reachable; one outside it is a 404. A token whose workspace has no organization only sees itself. Keyset-paginated with typed filters.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `updated_at`, `label`, `slug`. Without it the order is `created_at` descending. Values: `created_at`, `updated_at`, `label`, `slug`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `source` | `string` | no | Equality. Also `source[in]=a,b`. |
| `slug` | `string` | no | Equality. Also `slug[in]=a,b`. |
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

## workspaces create

Create workspace

```text
kobana inbox workspaces create [OPTIONS]
```

`POST /v1/workspaces`

Requires a `src=system` token. The workspace is created inside the token organization. Scope: the organization of the token workspace — every live workspace of that organization is reachable; one outside it is a 404. A token whose workspace has no organization only sees itself.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | `string` | yes | Values: `kobana_app`, `kobana_billing`, `kobana_inbox`, `kobana_agent`, `kobana_finance`, `kobana_legal`, `kobana_team`. |
| `label` | `string` | yes |  |
| `external_id` | `string` | no | Optional. When omitted the server defaults `external_id` to the generated internal `id`. |
| `slug` | `string` | no | Optional. Lowercase dasherized slug, unique within the organization, used as the middle segment of inbox addresses. Capped at 20 characters so the composed local part fits SMTP's 64-character limit. When omitted, the server derives it from the dasherized `label`. |
| `monthly_cost_limit` | `number` | no |  |
| `monthly_emails_limit` | `integer` | no |  |
| `custom_metadata` | `object` | no | User metadata, free-form. Send `null` to clear it. Max 64 KB serialized, 16 levels deep. |

More fields and nested objects: `kobana schema inbox.workspaces.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## workspaces update

Create or update workspace

```text
kobana inbox workspaces update [OPTIONS]
```

`PUT /v1/workspaces`

Upsert. Takes the same body as POST plus the lookup keys `id` and `external_id`, matched in that order. A match is updated in place and returns 200 — fields absent from the body keep their stored value. No key, or no match, creates the workspace exactly as POST would and returns 201. Scope: the organization of the token workspace — every live workspace of that organization is reachable; one outside it is a 404. A token whose workspace has no organization only sees itself. Requires a `src=system` token; `external_id` cannot be changed on a matched workspace (409).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | `string` | yes | Values: `kobana_app`, `kobana_billing`, `kobana_inbox`, `kobana_agent`, `kobana_finance`, `kobana_legal`, `kobana_team`. |
| `label` | `string` | yes |  |
| `external_id` | `string` | no | Match on the source-system id. Also stored on the record when it is created. |
| `slug` | `string` | no | Optional. Lowercase dasherized slug, unique within the organization, used as the middle segment of inbox addresses. Capped at 20 characters so the composed local part fits SMTP's 64-character limit. When omitted, the server derives it from the dasherized `label`. |
| `monthly_cost_limit` | `number` | no |  |
| `monthly_emails_limit` | `integer` | no |  |
| `custom_metadata` | `object` | no | User metadata, free-form. Send `null` to clear it. Max 64 KB serialized, 16 levels deep. |
| `id` | `string (uuid)` | no | Match on the internal id. Checked before `external_id`. |

More fields and nested objects: `kobana schema inbox.workspaces.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## workspaces get

Get workspace

```text
kobana inbox workspaces get [ID] [OPTIONS]
```

`GET /v1/workspaces/{id}`

Accepts either the internal UUID or the resource `external_id`. Scope: the organization of the token workspace — every live workspace of that organization is reachable; one outside it is a 404. A token whose workspace has no organization only sees itself.

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

## workspaces delete

Delete workspace

```text
kobana inbox workspaces delete [ID] [OPTIONS]
```

`DELETE /v1/workspaces/{id}`

Accepts either the internal UUID or the resource `external_id`. Scope: the organization of the token workspace — every live workspace of that organization is reachable; one outside it is a 404. A token whose workspace has no organization only sees itself. Requires a `src=system` token.

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
