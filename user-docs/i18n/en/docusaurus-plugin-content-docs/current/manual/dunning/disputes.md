---
id: disputes
title: "kobana dunning disputes"
sidebar_label: "disputes"
---

# kobana dunning disputes


Commands of `kobana dunning disputes`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`disputes list`](#disputes-list) | `GET /v1/disputes` | List disputes |
| [`disputes create`](#disputes-create) | `POST /v1/disputes` | Open dispute |
| [`disputes get`](#disputes-get) | `GET /v1/disputes/{id}` | Get dispute |
| [`disputes update`](#disputes-update) | `PATCH /v1/disputes/{id}` | Run dispute action |

## disputes list

List disputes

```text
kobana dunning disputes list [OPTIONS]
```

`GET /v1/disputes`

Lists the organization’s disputes, ordered by opening date (newest first). Custom sorting is not supported.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `status` | `string` | no | Filter by status (`open` \| `under_review` \| `resolved_valid` \| `resolved_invalid` \| `canceled`). Values: `open`, `under_review`, `resolved_valid`, `resolved_invalid`, `canceled`. |
| `charge_id` | `string` | no | Filter by charge (UUID). |
| `person_id` | `string` | no | Filter by customer (UUID). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated dispute list. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## disputes create

Open dispute

```text
kobana dunning disputes create [OPTIONS]
```

`POST /v1/disputes`

Opens a dispute against a charge and immediately pauses the collection rule. If the charge already has an open or under-review dispute, the existing one is returned (no duplicates). Supports `Idempotency-Key`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `charge_id` | `string (uuid)` | yes | Disputed charge. |
| `type` | `string` | yes | Dispute type (`already_paid` \| `not_recognized` \| `wrong_amount` \| `service_not_provided` \| `incorrect_invoice` \| `wrong_recipient` \| `other`). Values: `already_paid`, `not_recognized`, `wrong_amount`, `service_not_provided`, `incorrect_invoice`, `wrong_recipient`, `other`. |
| `reason` | `string` | no | Debtor/operator account of the issue. |
| `disputed_amount` | `number` | no | Disputed amount (number). Omit to dispute the full amount. |
| `documents` | `array<object>` | no | Evidence attachments/references. |
| `assigned_to_id` | `string (uuid)` | no | User assigned to the review (must belong to the organization). |
| `sla_days` | `integer` | no | SLA days to resolve (max 60; system default when omitted). |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |

More fields and nested objects: `kobana schema dunning.disputes.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Dispute opened (or the charge’s existing open dispute). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## disputes get

Get dispute

```text
kobana dunning disputes get [ID] [OPTIONS]
```

`GET /v1/disputes/{id}`

Returns a dispute by ID, with the full charge, customer and assignee.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Dispute found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## disputes update

Run dispute action

```text
kobana dunning disputes update [ID] [OPTIONS]
```

`PATCH /v1/disputes/{id}`

Runs a workflow action on the dispute: `start_review`, `resolve` or `cancel`. Invalid transitions (e.g., `start_review` when the status is not `open`) return 409.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

The body is not a plain object (or has variants). See `kobana schema dunning.disputes.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Dispute updated by the action. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |
