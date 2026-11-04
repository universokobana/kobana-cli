---
id: organization
title: "kobana inbox organization"
sidebar_label: "organization"
description: "Get organization"
---

# kobana inbox organization

Get organization

Commands of `kobana inbox organization`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Get organization |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Update organization |

## organization

Get organization

```text
kobana inbox organization [OPTIONS] [COMMAND]
```

`GET /v1/organization`

The organization comes from the token alone (its `org` claim, as in `GET /v1/me`) — there is no list and no `{id}`, and no header, query or body picks another one. Organization-level: `X-Workspace-Id` is not needed. Needs the `settings.show` permission (every system role has it).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |

## organization update

Update organization

```text
kobana inbox organization update [OPTIONS]
```

`PATCH /v1/organization`

The organization comes from the token alone (its `org` claim, as in `GET /v1/me`) — there is no list and no `{id}`, and no header, query or body picks another one. Organization-level: `X-Workspace-Id` is not needed. Changes the profile shared by every workspace: `custom_metadata`. Unknown keys, the identity fields (`id`, `public_id`, `external_id`, `slug`, `document_type`, `document_number`) and the mirrors of the Kobana account (`name`, `email`, `phone` — kept in sync from it, so they are not editable here) are ignored. Naming one of Kobana's commercial settings (courtesy, plan bypass, limits, partner, entitlements) is refused with 403 `FORBIDDEN` (`error.details.fields`). Needs the `settings.update` permission (owner and admin). Every change is recorded in the organization audit log.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Shared secret between CloudFront and the origin. Required in production; ignored in development unless `CF_ORIGIN_SHARED_SECRET` is set. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `custom_metadata` | `object` | no | User metadata, free-form. `null` keeps the stored value. Max 64 KB serialized, 16 levels deep. |
| `custom_metadata_mode` | `string` | no | How the submitted `custom_metadata` is reconciled with the stored value. `replace` (default) overwrites the whole object; `merge` deep-merges over the current value — nested objects merge recursively, scalars and arrays overwrite the key, and `null` keys are kept (they do not delete the key). Update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema inbox.organization.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success. |
| `400` | Body failed Zod validation. `error.details` carries `formErrors`/`fieldErrors`. |
