---
id: interactions
title: "kobana dunning interactions"
sidebar_label: "interactions"
---

# kobana dunning interactions


Commands of `kobana dunning interactions`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`interactions list`](#interactions-list) | `GET /v1/interactions` | List interactions |
| [`interactions create`](#interactions-create) | `POST /v1/interactions` | Log interaction |
| [`interactions get`](#interactions-get) | `GET /v1/interactions/{id}` | Get interaction |
| [`interactions update`](#interactions-update) | `PUT /v1/interactions/{id}` | Update interaction |
| [`interactions delete`](#interactions-delete) | `DELETE /v1/interactions/{id}` | Delete interaction |

## interactions list

List interactions

```text
kobana dunning interactions list [OPTIONS]
```

`GET /v1/interactions`

Lists the organization’s interactions with pagination and filters. Sortable via `sort_by` (`contacted_at` | `interaction_type` | `created_at` | `updated_at`; default `contacted_at` desc).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `sort_by` | `string` | no | Sort field (public snake_case name, e.g. `due_date`). |
| `sort_order` | `string` | no | Sort direction (`asc` \| `desc`). Values: `asc`, `desc`. |
| `interaction_type` | `string` | no | Filter by type (`call` \| `email` \| `whatsapp` \| `meeting` \| `note`). Values: `call`, `email`, `whatsapp`, `meeting`, `note`. |
| `direction` | `string` | no | Filter by direction (`inbound` \| `outbound`). Values: `inbound`, `outbound`. |
| `outcome` | `string` | no | Filter by outcome (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Values: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`. |
| `person_id` | `string` | no | Filter by customer (UUID). |
| `charge_id` | `string` | no | Filter by charge (UUID). |
| `include` | `string` | no | Relations to include, comma-separated (`person`, `user`, `charge`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated interaction list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## interactions create

Log interaction

```text
kobana dunning interactions create [OPTIONS]
```

`POST /v1/interactions`

Logs an interaction with a customer, optionally linked to a charge. Supports `Idempotency-Key`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `person_id` | `string (uuid)` | yes | Customer of the interaction. |
| `interaction_type` | `string` | yes | Type (`call` \| `email` \| `whatsapp` \| `meeting` \| `note`). Values: `call`, `email`, `whatsapp`, `meeting`, `note`. |
| `direction` | `string` | yes | Direction (`inbound` = customer reached out, `outbound` = we contacted the customer). Values: `inbound`, `outbound`. |
| `charge_id` | `string (uuid)` | no | Charge related to the interaction. |
| `summary` | `string` | no | Interaction summary/notes. |
| `outcome` | `string` | no | Outcome (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Values: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`, `null`. |
| `promise_date` | `string (date-time)` | no | Promised payment date. |
| `promised_amount` | `number` | no | Promised amount (positive number). |
| `contacted_at` | `string (date-time)` | no | Contact time (ISO 8601; defaults to now). |
| `duration_seconds` | `integer` | no | Contact duration in seconds. |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.interactions.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Interaction logged (includes customer, user and charge summaries). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## interactions get

Get interaction

```text
kobana dunning interactions get [ID] [OPTIONS]
```

`GET /v1/interactions/{id}`

Returns an interaction by ID, with customer, user and charge summaries.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Interaction found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## interactions update

Update interaction

```text
kobana dunning interactions update [ID] [OPTIONS]
```

`PUT /v1/interactions/{id}`

Updates an interaction’s editable fields (partial).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `summary` | `string` | no | Interaction summary/notes. |
| `notes` | `string` | no | Alias for `summary` (stored in the same field; when both are sent, `notes` wins). |
| `outcome` | `string` | no | Outcome (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Values: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`, `null`. |
| `promise_date` | `string (date-time)` | no | Promised payment date. |
| `promised_amount` | `number` | no | Promised amount (positive number). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_data_mode` | `string` | no | How `custom_data` is applied on update: `replace` (default) overwrites the whole object; `merge` deep-merges the payload over the stored value — nested objects merge, scalars/arrays/null overwrite the key. Values: `replace`, `merge`. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.interactions.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Interaction updated. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## interactions delete

Delete interaction

```text
kobana dunning interactions delete [ID] [OPTIONS]
```

`DELETE /v1/interactions/{id}`

Soft-deletes an interaction; the record remains for the audit trail.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Interaction deleted. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
