---
id: notification-preferences
title: "kobana dunning notification-preferences"
sidebar_label: "notification-preferences"
---

# kobana dunning notification-preferences


Commands of `kobana dunning notification-preferences`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`notification-preferences list`](#notification-preferences-list) | `GET /v1/notification-preferences` | Get notification preferences |
| [`notification-preferences update`](#notification-preferences-update) | `PUT /v1/notification-preferences` | Update notification preferences |

## notification-preferences list

Get notification preferences

```text
kobana dunning notification-preferences list [OPTIONS]
```

`GET /v1/notification-preferences`

Returns the effective gate per message type — types without a saved record appear with all channels enabled.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Effective preferences per type. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## notification-preferences update

Update notification preferences

```text
kobana dunning notification-preferences update [OPTIONS]
```

`PUT /v1/notification-preferences`

Upserts the submitted preferences (partial: only the included types are changed).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `preferences` | `array<object>` | yes |  |

More fields and nested objects: `kobana schema dunning.notification-preferences.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Preferences saved. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
