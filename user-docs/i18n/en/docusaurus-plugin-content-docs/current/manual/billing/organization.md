---
id: organization
title: "kobana billing organization"
sidebar_label: "organization"
description: "Get current organization"
---

# kobana billing organization

Get current organization

Commands of `kobana billing organization`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Get current organization |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Update current organization |

## organization

Get current organization

```text
kobana billing organization [OPTIONS] [COMMAND]
```

`GET /v1/organization`

Returns the data of the organization the authenticated token belongs to. There is no `:id` parameter — the organization is always resolved from the API key.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Current organization detail. |
| `404` | Resource not found in the current organization. |

## organization update

Update current organization

```text
kobana billing organization update [OPTIONS]
```

`PATCH /v1/organization`

Updates the profile the authenticated organization controls: `addresses` and `custom_metadata` (`null` keeps the current value). Identity (`id`, `external_id`, `status`, document) and the fields mirrored from the Kobana Account (`name`, `email`, `phone`) cannot be changed through this endpoint: if sent, they are ignored — to correct them, change the Kobana Account. Commercial fields (such as `courtesy` and `bypass_subscription`) are refused with `403`.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `addresses` | `array<object>` | no | List of organization addresses. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). `null` is ignored and keeps the current value. |

More fields and nested objects: `kobana schema billing.organization.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Organization updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
