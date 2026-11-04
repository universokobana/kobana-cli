---
id: collection-rules
title: "kobana dunning collection-rules"
sidebar_label: "collection-rules"
---

# kobana dunning collection-rules


Commands of `kobana dunning collection-rules`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`collection-rules steps`](#collection-rules-steps) | `GET /v1/collection-rules/{id}/steps` | List rule steps |
| [`collection-rules steps create`](#collection-rules-steps-create) | `POST /v1/collection-rules/{id}/steps` | Create step |
| [`collection-rules steps update`](#collection-rules-steps-update) | `PUT /v1/collection-rules/{id}/steps/{stepId}` | Update step |
| [`collection-rules steps delete`](#collection-rules-steps-delete) | `DELETE /v1/collection-rules/{id}/steps/{stepId}` | Delete step |
| [`collection-rules list`](#collection-rules-list) | `GET /v1/collection-rules` | List collection rules |
| [`collection-rules create`](#collection-rules-create) | `POST /v1/collection-rules` | Create collection rule |
| [`collection-rules get`](#collection-rules-get) | `GET /v1/collection-rules/{id}` | Get collection rule |
| [`collection-rules update`](#collection-rules-update) | `PUT /v1/collection-rules/{id}` | Update collection rule |
| [`collection-rules delete`](#collection-rules-delete) | `DELETE /v1/collection-rules/{id}` | Delete collection rule |

## collection-rules steps

List rule steps

```text
kobana dunning collection-rules steps [ID] [OPTIONS] [COMMAND]
```

`GET /v1/collection-rules/{id}/steps`

Lists the rule steps ordered by `position`, with the message template embedded.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Rule steps (`data` envelope, not paginated). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## collection-rules steps create

Create step

```text
kobana dunning collection-rules steps create [ID] [OPTIONS]
```

`POST /v1/collection-rules/{id}/steps`

Creates a step on the rule. Without `position` it goes last; with `position`, following steps shift down.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Step name. |
| `trigger_type` | `string` | yes | Trigger (`before_due` \| `on_due` \| `after_due`). The `after_issue`/`after_payment` triggers can only be set via the rule. Values: `before_due`, `on_due`, `after_due`. |
| `trigger_days` | `integer` | yes | Days relative to the trigger (0 = same day). |
| `action_type` | `string` | yes | Action (`notification` \| `negativation` \| `protest` \| `manual_task` \| `negotiation_offer` \| `judicial`). Values: `notification`, `negativation`, `protest`, `manual_task`, `negotiation_offer`, `judicial`. |
| `channels` | `array<string>` | yes | Delivery channels (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `message_template_id` | `string (uuid)` | no | Message template used by the step (must belong to the organization). |
| `position` | `integer` | no | Step position in the rule (starts at 1). |
| `tone` | `string` | no | Message tone (`friendly` \| `neutral` \| `firm` \| `urgent`). Values: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `preferred_time` | `string` | no | Preferred send time (e.g. `09:00`). |
| `activated_at` | `string (date-time)` | no | Activation timestamp (null = inactive). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |

More fields and nested objects: `kobana schema dunning.collection-rules.steps.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Step created (with message template). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## collection-rules steps update

Update step

```text
kobana dunning collection-rules steps update [ID] [STEPID] [OPTIONS]
```

`PUT /v1/collection-rules/{id}/steps/{stepId}`

Updates a step (partial). Changing `position` automatically reorders the other steps.

**Path arguments:** `id`, `stepId`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `message_template_id` | `string (uuid)` | no | Message template used by the step (must belong to the organization). |
| `position` | `integer` | no | Step position in the rule (starts at 1). |
| `name` | `string` | no | Step name. |
| `trigger_type` | `string` | no | Trigger (`before_due` \| `on_due` \| `after_due`). The `after_issue`/`after_payment` triggers can only be set via the rule. Values: `before_due`, `on_due`, `after_due`. |
| `trigger_days` | `integer` | no | Days relative to the trigger (0 = same day). |
| `action_type` | `string` | no | Action (`notification` \| `negativation` \| `protest` \| `manual_task` \| `negotiation_offer` \| `judicial`). Values: `notification`, `negativation`, `protest`, `manual_task`, `negotiation_offer`, `judicial`. |
| `channels` | `array<string>` | no | Delivery channels (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `tone` | `string` | no | Message tone (`friendly` \| `neutral` \| `firm` \| `urgent`). Values: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `preferred_time` | `string` | no | Preferred send time (e.g. `09:00`). |
| `activated_at` | `string (date-time)` | no | Activation timestamp (null = inactive). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.collection-rules.steps.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Step updated (with message template). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## collection-rules steps delete

Delete step

```text
kobana dunning collection-rules steps delete [ID] [STEPID] [OPTIONS]
```

`DELETE /v1/collection-rules/{id}/steps/{stepId}`

Soft-deletes a step and reorders the remaining ones. Fails with 409 if the step has associated notifications.

**Path arguments:** `id`, `stepId`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Step deleted. |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |

## collection-rules list

List collection rules

```text
kobana dunning collection-rules list [OPTIONS]
```

`GET /v1/collection-rules`

Lists the organization’s collection rules, ordered by priority. `_count.charges` carries the number of active charges on the rule.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page number (starts at 1). |
| `per_page` | `integer` | no | Items per page (max 100). |
| `classification_id` | `string` | no | Filter by associated classification (UUID). |
| `is_default` | `string` | no | Filter by default rule (`true` \| `false`). Values: `true`, `false`. |
| `activated_at` | `string` | no | Filter by activation: `true` = active, `false` = inactive. Values: `true`, `false`. |
| `include` | `string` | no | Comma-separated relations to embed. Supported: `steps`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated collection rule list. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## collection-rules create

Create collection rule

```text
kobana dunning collection-rules create [OPTIONS]
```

`POST /v1/collection-rules`

Creates a collection rule, optionally with steps. Setting `is_default` unsets the previous default rule.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Rule name. |
| `classification_id` | `string (uuid)` | no | Customer classification served by the rule (segmentation). |
| `description` | `string` | no | Rule description. |
| `is_default` | `boolean` | no | true when this is the organization’s default rule (only one). |
| `priority` | `integer` | no | Priority when selecting a rule (higher wins). |
| `activated_at` | `string (date-time)` | no | Activation timestamp (null = inactive). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |
| `steps` | `array<object>` | no | Rule steps ordered by `position` (present with `include=steps` and in create/update responses). |

More fields and nested objects: `kobana schema dunning.collection-rules.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Collection rule created (with classification and steps). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## collection-rules get

Get collection rule

```text
kobana dunning collection-rules get [ID] [OPTIONS]
```

`GET /v1/collection-rules/{id}`

Returns a collection rule by ID. Use `include=steps` to embed the steps.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `include` | `string` | no | Comma-separated relations to embed. Supported: `steps`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Collection rule found (`data` envelope). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## collection-rules update

Update collection rule

```text
kobana dunning collection-rules update [ID] [OPTIONS]
```

`PUT /v1/collection-rules/{id}`

Updates the rule data (partial). The `steps` array, when sent, upserts and removes missing steps.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `classification_id` | `string (uuid)` | no | Customer classification served by the rule (segmentation). |
| `name` | `string` | no | Rule name. |
| `description` | `string` | no | Rule description. |
| `is_default` | `boolean` | no | true when this is the organization’s default rule (only one). |
| `priority` | `integer` | no | Priority when selecting a rule (higher wins). |
| `activated_at` | `string (date-time)` | no | Activation timestamp (null = inactive). |
| `external_id` | `string` | no | Identifier in YOUR system (integration key). |
| `custom_data` | `object` | no | API-consumer custom metadata. Limits: 64 KB serialized and 16 nesting levels. |
| `custom_metadata` | `object` | no | User metadata (free-form JSON object, read-write). See `custom_data` for the legacy field on core resources. |
| `tags` | `array<string>` | no | Free-form tags. |
| `steps` | `array<object>` | no | Full step set (upsert): with `id` updates, without `id` creates, missing ones are removed. |
| `custom_metadata_mode` | `string` | no | How the `custom_metadata` sent is reconciled with the stored value: `replace` (default) replaces the whole object; `merge` deep-merges — nested objects merge, scalars and arrays replace the key, and a key sent as `null` is stored as `null` (not removed). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema dunning.collection-rules.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Collection rule updated (`data` envelope). |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## collection-rules delete

Delete collection rule

```text
kobana dunning collection-rules delete [ID] [OPTIONS]
```

`DELETE /v1/collection-rules/{id}`

Soft-deletes the rule and its steps. Fails with 409 if any charge uses the rule.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `204` | Collection rule deleted. |
| `404` | Resource not found or outside your organization. |
| `409` | Conflict with the current resource state (or reused idempotency key). |
| `413` | Request body larger than 2 MB. |
