---
id: emails
title: "kobana inbox emails"
sidebar_label: "emails"
---

# kobana inbox emails


Commands of `kobana inbox emails`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`emails list`](#emails-list) | `GET /v1/emails` | List emails |
| [`emails create`](#emails-create) | `POST /v1/emails` | Submit a raw .eml |
| [`emails get`](#emails-get) | `GET /v1/emails/{id}` | Get email |
| [`emails delete`](#emails-delete) | `DELETE /v1/emails/{id}` | Delete email |
| [`emails eml`](#emails-eml) | `GET /v1/emails/{id}/eml` | Download original .eml |
| [`emails reprocess`](#emails-reprocess) | `POST /v1/emails/{id}/reprocess` | Reprocess email |

## emails list

List emails

```text
kobana inbox emails list [OPTIONS]
```

`GET /v1/emails`

Keyset-paginated. Filters are typed query params (`field=value`, `field[op]=value`); unknown filters, unsortable fields and malformed values are rejected with 400 `VALIDATION_ERROR`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Opaque cursor returned in `meta.next_cursor` of the previous page. |
| `sort_by` | `string` | no | Field to order by. Allowed: `created_at`, `updated_at`, `from`, `status`. Without it the order is `created_at` descending. Values: `created_at`, `updated_at`, `from`, `status`. |
| `sort_order` | `string` | no | `asc` (default) or `desc`. Needs `sort_by`. Values: `asc`, `desc`. |
| `status` | `string` | no | Equality. Also `status[in]=a,b`. Values: `received`, `approved`, `rejected`, `processing`, `processed`, `failed`. |
| `inbox_id` | `string` | no | Equality. Also `inbox_id[in]=a,b`. |
| `agent_id` | `string` | no | Equality. Also `agent_id[in]=a,b`. |
| `message_id` | `string` | no | Equality. Also `message_id[in]=a,b`. |
| `from` | `string` | no | Equality. Also `from[in]=a,b`. |
| `to` | `string` | no | Rows whose `to` contains the value. |
| `created_at` | `string` | no | Equality. Also `created_at[in]=a,b` and `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `updated_at` | `string` | no | Equality. Also `updated_at[in]=a,b` and `updated_at[gt\|gte\|lt\|lte]`. |
| `updated_at[gte]` | `string` | no |  |
| `updated_at[lte]` | `string` | no |  |
| `received_at` | `string` | no | Equality. Also `received_at[in]=a,b` and `received_at[gt\|gte\|lt\|lte]`. |
| `received_at[gte]` | `string` | no |  |
| `received_at[lte]` | `string` | no |  |
| `processed_at` | `string` | no | Equality. Also `processed_at[in]=a,b` and `processed_at[gt\|gte\|lt\|lte]`. |
| `processed_at[gte]` | `string` | no |  |
| `processed_at[lte]` | `string` | no |  |

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

## emails create

Submit a raw .eml

```text
kobana inbox emails create [OPTIONS]
```

`POST /v1/emails`

Stores an RFC 5322 message exactly as SES inbound would: raw object in the raw bucket, `Email` in `received`, and a `process-email` job — filters, agent and webhooks run as for real mail. Send the message either as `multipart/form-data` (field `file`, optional `inbox_id`) or as the request body with `Content-Type: message/rfc822` (optional `?inbox_id=`). Without `inbox_id` the inbox is resolved from the `To:` header among the caller workspace inboxes. A `Message-ID` already stored in the same inbox is rejected with 409 (`error.details.email_id` points at the existing email); the same `Message-ID` in another inbox is a different email.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `inbox_id` | `string` | no | Target inbox. Overridden by the multipart `inbox_id` field when both are sent. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

The body is not a plain object (or has variants). See `kobana schema inbox.emails.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Stored and queued for processing. |
| `400` | Empty body or missing multipart `file` field. |
| `404` | `inbox_id` does not exist in the caller workspace (`INBOX_NOT_FOUND`). |
| `409` | `DUPLICATE_MESSAGE_ID`: the `Message-ID` is already stored in the target inbox. |
| `413` | `PAYLOAD_TOO_LARGE`: message above 20 MB. |
| `415` | `UNSUPPORTED_MEDIA_TYPE`: body is neither multipart nor `message/rfc822`. |
| `422` | `NO_INBOX_MATCHES_TO_HEADER` (no workspace inbox matches the `To:` recipients), `MISSING_RECIPIENT` (no usable `To:` header; pass `inbox_id`) or `INVALID_MESSAGE_ID` (control characters or over 998 characters). |

## emails get

Get email

```text
kobana inbox emails get [ID] [OPTIONS]
```

`GET /v1/emails/{id}`

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

## emails delete

Delete email

```text
kobana inbox emails delete [ID] [OPTIONS]
```

`DELETE /v1/emails/{id}`

Soft-deletes the email (sets `deleted_at`). It stops appearing in lists and detail; the raw object is purged by the retention job.

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

## emails eml

Download original .eml

```text
kobana inbox emails eml [ID] [OPTIONS]
```

`GET /v1/emails/{id}/eml`

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Raw RFC 5322 message stream. |
| `404` | Resource not found or filtered out by soft-delete. |

## emails reprocess

Reprocess email

```text
kobana inbox emails reprocess [ID] [OPTIONS]
```

`POST /v1/emails/{id}/reprocess`

Sends the email back through processing (filters, agent, webhooks). Only an email in a terminal status — `processed`, `rejected` or `failed` — is eligible; one still `received`, `approved` or `processing` is refused with 409 `CONFLICT`, so an agent (and its workflow) never runs twice for the same email. Running the agent needs an active plan: without one the call is refused with 403 `ENTITLEMENT_REQUIRED` (`error.details.reason` = `no_plan`) and nothing is queued.

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
| `409` | `CONFLICT`: the email is still `received`, `approved` or `processing`. |
