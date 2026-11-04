---
id: agents
title: "kobana inbox agents"
sidebar_label: "agents"
---

# kobana inbox agents


Commands of `kobana inbox agents`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`agents list`](#agents-list) | `GET /v1/agents` | List agents |
| [`agents create`](#agents-create) | `POST /v1/agents` | Create agent |
| [`agents update`](#agents-update) | `PUT /v1/agents` | Create or update agent |
| [`agents get`](#agents-get) | `GET /v1/agents/{id}` | Get agent |
| [`agents delete`](#agents-delete) | `DELETE /v1/agents/{id}` | Delete agent |
| [`agents versions`](#agents-versions) | `GET /v1/agents/{id}/versions` | List agent prompt versions |

## agents list

List agents

```text
kobana inbox agents list [OPTIONS]
```

`GET /v1/agents`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `updated_at`, `label`, `version`. Without it the order is `created_at` descending. Values: `created_at`, `updated_at`, `label`, `version`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `llm_model` | `string` | no | Equality. Also `llm_model[in]=a,b`. |
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

## agents create

Create agent

```text
kobana inbox agents create [OPTIONS]
```

`POST /v1/agents`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `llm_model` | `string` | yes |  |
| `system_message` | `string` | yes |  |
| `external_id` | `string` | no | Optional. Defaults to the generated internal `id`. |
| `skills_repo` | `string (uri)` | no |  |
| `workflow_allowed_hosts` | `array<string>` | no | Hosts (exact, e.g. `api.example.com`) a workflow returned by this agent may send `${{secrets.*}}`, `${{email.attachments.*}}` or a step output derived from them to. Empty (the default) blocks every destination for those values; steps without them are unaffected. |
| `custom_metadata` | `object` | no | User metadata, free-form. Send `null` to clear it. Max 64 KB serialized, 16 levels deep. |

More fields and nested objects: `kobana schema inbox.agents.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## agents update

Create or update agent

```text
kobana inbox agents update [OPTIONS]
```

`PUT /v1/agents`

Upsert. Takes the same body as POST plus the lookup keys `id` and `external_id`, matched in that order. A match is updated in place and returns 200 — fields absent from the body keep their stored value. No key, or no match, creates the agent exactly as POST would and returns 201.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `llm_model` | `string` | yes |  |
| `system_message` | `string` | yes |  |
| `external_id` | `string` | no | Match on the source-system id. Also stored on the record when it is created. |
| `skills_repo` | `string (uri)` | no |  |
| `workflow_allowed_hosts` | `array<string>` | no | Hosts (exact, e.g. `api.example.com`) a workflow returned by this agent may send `${{secrets.*}}`, `${{email.attachments.*}}` or a step output derived from them to. Empty (the default) blocks every destination for those values; steps without them are unaffected. |
| `custom_metadata` | `object` | no | User metadata, free-form. Send `null` to clear it. Max 64 KB serialized, 16 levels deep. |
| `id` | `string (uuid)` | no | Match on the internal id. Checked before `external_id`. |

More fields and nested objects: `kobana schema inbox.agents.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## agents get

Get agent

```text
kobana inbox agents get [ID] [OPTIONS]
```

`GET /v1/agents/{id}`

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

## agents delete

Delete agent

```text
kobana inbox agents delete [ID] [OPTIONS]
```

`DELETE /v1/agents/{id}`

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

## agents versions

List agent prompt versions

```text
kobana inbox agents versions [ID] [OPTIONS]
```

`GET /v1/agents/{id}/versions`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `version`. Without it the order is `version` descending. Values: `created_at`, `version`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `version` | `integer` | no | Equality. Also `version[in]=a,b` and `version[gt\|gte\|lt\|lte]`. |
| `version[gte]` | `integer` | no |  |
| `version[lte]` | `integer` | no |  |
| `created_at` | `string` | no | Equality. Also `created_at[in]=a,b` and `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |

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
