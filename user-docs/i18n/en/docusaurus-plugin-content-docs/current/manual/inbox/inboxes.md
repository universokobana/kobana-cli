---
id: inboxes
title: "kobana inbox inboxes"
sidebar_label: "inboxes"
---

# kobana inbox inboxes


Commands of `kobana inbox inboxes`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`inboxes recipients remove`](#inboxes-recipients-remove) | `POST /v1/inboxes/{id}/recipients/remove` | Remove allowed recipient |
| [`inboxes recipients create`](#inboxes-recipients-create) | `POST /v1/inboxes/{id}/recipients` | Add allowed recipient |
| [`inboxes list`](#inboxes-list) | `GET /v1/inboxes` | List inboxes |
| [`inboxes create`](#inboxes-create) | `POST /v1/inboxes` | Create inbox |
| [`inboxes update`](#inboxes-update) | `PUT /v1/inboxes` | Create or update inbox |
| [`inboxes get`](#inboxes-get) | `GET /v1/inboxes/{id}` | Get inbox |
| [`inboxes delete`](#inboxes-delete) | `DELETE /v1/inboxes/{id}` | Delete inbox |

## inboxes recipients remove

Remove allowed recipient

```text
kobana inbox inboxes recipients remove [ID] [OPTIONS]
```

`POST /v1/inboxes/{id}/recipients/remove`

Accepts either the internal UUID or the resource `external_id`. Removes an email from `allowed_recipients`. 404 when the address is not in the list. Replaces `DELETE /v1/inboxes/{id}/recipients`: removing an address is an update of the inbox (`write`), not a deletion.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string (email)` | yes |  |

More fields and nested objects: `kobana schema inbox.inboxes.recipients.remove`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Recipient removed. |
| `400` | `email` missing or not a valid address. |
| `404` | Resource not found or filtered out by soft-delete. |

## inboxes recipients create

Add allowed recipient

```text
kobana inbox inboxes recipients create [ID] [OPTIONS]
```

`POST /v1/inboxes/{id}/recipients`

Accepts either the internal UUID or the resource `external_id`. Adds an email to `allowed_recipients`. Idempotent — returns 200 with the current list when already present.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string (email)` | yes |  |

More fields and nested objects: `kobana schema inbox.inboxes.recipients.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Recipient already present (no-op). |
| `201` | Recipient added. |
| `404` | Resource not found or filtered out by soft-delete. |

## inboxes list

List inboxes

```text
kobana inbox inboxes list [OPTIONS]
```

`GET /v1/inboxes`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `updated_at`, `label`, `slug`. Without it the order is `created_at` descending. Values: `created_at`, `updated_at`, `label`, `slug`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `status` | `string` | no | Equality. Also `status[in]=a,b`. Values: `active`, `inactive`. |
| `agent_id` | `string` | no | Equality. Also `agent_id[in]=a,b`. |
| `slug` | `string` | no | Equality. Also `slug[in]=a,b`. |
| `email` | `string` | no | Equality. Also `email[in]=a,b`. |
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

## inboxes create

Create inbox

```text
kobana inbox inboxes create [OPTIONS]
```

`POST /v1/inboxes`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `external_id` | `string` | no | Optional. Defaults to the generated internal `id`. |
| `email` | `string (email)` | no | Optional. Must use the configured inbox domain (`SES_DOMAIN`) and a local part that starts with "\{organization-slug\}.\{workspace-slug\}." of the caller; otherwise returns 422. Duplicate addresses return 409. When omitted the server composes it as "\{organization-slug\}.\{workspace-slug\}.\{inbox-slug\}@&lt;domain>". |
| `slug` | `string` | no | Optional. Lowercase dasherized slug, unique within the workspace, used as the last segment of the address. Returns 409 when it collides with another inbox in the same workspace, and 422 when the composed local part exceeds 64 characters. When omitted, the server derives it from the dasherized `label`. |
| `agent_id` | `string (uuid)` | no |  |
| `allowed_recipients` | `array<string (email)>` | no |  |
| `filters` | `array<object>` | no |  |
| `attachments_allow` | `boolean` | no |  |
| `attachments_max_file_number` | `integer` | no |  |
| `attachments_max_file_size` | `integer` | no |  |
| `attachments_max_total_size` | `integer` | no |  |
| `attachments_allow_rules` | `array<object>` | no | Allow list of attachments. An attachment is processed when it matches at least one rule; inside a rule every condition given must match. `null` or `[]` processes every attachment. An attachment outside the list is ignored, never a reason to reject the e-mail: the agent does not see it, the attachment limits do not count it, and a workflow step cannot reference it. On update, omit to keep, send `null` to clear. |
| `custom_metadata` | `object` | no | User metadata, free-form. Send `null` to clear it. Max 64 KB serialized, 16 levels deep. |

More fields and nested objects: `kobana schema inbox.inboxes.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## inboxes update

Create or update inbox

```text
kobana inbox inboxes update [OPTIONS]
```

`PUT /v1/inboxes`

Upsert. Takes the same body as POST plus the lookup keys `id` and `external_id`, matched in that order, then by `email`. A match is updated in place and returns 200 — fields absent from the body keep their stored value. No key, or no match, creates the inbox exactly as POST would and returns 201.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | yes |  |
| `external_id` | `string` | no | Match on the source-system id. Also stored on the record when it is created. |
| `email` | `string (email)` | no | Optional. Must use the configured inbox domain (`SES_DOMAIN`) and a local part that starts with "\{organization-slug\}.\{workspace-slug\}." of the caller; otherwise returns 422. Duplicate addresses return 409. When omitted the server composes it as "\{organization-slug\}.\{workspace-slug\}.\{inbox-slug\}@&lt;domain>". |
| `slug` | `string` | no | Optional. Lowercase dasherized slug, unique within the workspace, used as the last segment of the address. Returns 409 when it collides with another inbox in the same workspace, and 422 when the composed local part exceeds 64 characters. When omitted, the server derives it from the dasherized `label`. |
| `agent_id` | `string (uuid)` | no |  |
| `allowed_recipients` | `array<string (email)>` | no |  |
| `filters` | `array<object>` | no |  |
| `attachments_allow` | `boolean` | no |  |
| `attachments_max_file_number` | `integer` | no |  |
| `attachments_max_file_size` | `integer` | no |  |
| `attachments_max_total_size` | `integer` | no |  |
| `attachments_allow_rules` | `array<object>` | no | Allow list of attachments. An attachment is processed when it matches at least one rule; inside a rule every condition given must match. `null` or `[]` processes every attachment. An attachment outside the list is ignored, never a reason to reject the e-mail: the agent does not see it, the attachment limits do not count it, and a workflow step cannot reference it. On update, omit to keep, send `null` to clear. |
| `custom_metadata` | `object` | no | User metadata, free-form. Send `null` to clear it. Max 64 KB serialized, 16 levels deep. |
| `id` | `string (uuid)` | no | Match on the internal id. Checked before `external_id`. |

More fields and nested objects: `kobana schema inbox.inboxes.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `201` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
| `404` | Resource not found or filtered out by soft-delete. |
| `409` | Unique constraint violated (e.g. `external_id` or inbox `email` already in use). |
| `422` | Body validation that requires server-side context (e.g. inbox domain mismatch). |

## inboxes get

Get inbox

```text
kobana inbox inboxes get [ID] [OPTIONS]
```

`GET /v1/inboxes/{id}`

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

## inboxes delete

Delete inbox

```text
kobana inbox inboxes delete [ID] [OPTIONS]
```

`DELETE /v1/inboxes/{id}`

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
