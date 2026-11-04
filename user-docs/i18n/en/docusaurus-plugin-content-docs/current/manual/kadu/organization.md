---
id: organization
title: "kobana kadu organization"
sidebar_label: "organization"
description: "Token's organization"
---

# kobana kadu organization

Token's organization

Commands of `kobana kadu organization`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Token's organization |
| [`organization slack-installations list`](#organization-slack-installations-list) | `GET /v1/organization/slack_installations` | Slack bot installations of the organization |
| [`organization slack-installations update`](#organization-slack-installations-update) | `PATCH /v1/organization/slack_installations/{installationId}` | Configures a Slack installation |
| [`organization slack-installations delete`](#organization-slack-installations-delete) | `DELETE /v1/organization/slack_installations/{installationId}` | Removes a Slack installation |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Update the token's organization |

## organization

Token's organization

```text
kobana kadu organization [OPTIONS] [COMMAND]
```

`GET /v1/organization`

Returns **only the token's organization** (the one in the `org` claim). It takes no id and does not list: there is no way to read another organization through this API.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## organization slack-installations list

Slack bot installations of the organization

```text
kobana kadu organization slack-installations list [OPTIONS]
```

`GET /v1/organization/slack_installations`

Slack workspaces linked to the token's organization (always the token's; no id).

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## organization slack-installations update

Configures a Slack installation

```text
kobana kadu organization slack-installations update [INSTALLATIONID] [OPTIONS]
```

`PATCH /v1/organization/slack_installations/{installationId}`

Where the bot answers (`allowed_channel_types`), output files as a link or attachment (`files_mode`, `max_upload_bytes` up to 100 MB) and detail in the thread (`log_detail`).

**Path arguments:** `installationId`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `allowed_channel_types` | `array<string>` | no |  |
| `files_mode` | `string` | no | Values: `link`, `upload`. |
| `max_upload_bytes` | `integer` | no |  |
| `log_detail` | `string` | no | Values: `milestones`, `verbose`. |

More fields and nested objects: `kobana schema kadu.organization.slack-installations.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## organization slack-installations delete

Removes a Slack installation

```text
kobana kadu organization slack-installations delete [INSTALLATIONID] [OPTIONS]
```

`DELETE /v1/organization/slack_installations/{installationId}`

Requires the `delete` level (or `all`). Revokes the bot token in Slack and undoes the account connections.

**Path arguments:** `installationId`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## organization update

Update the token's organization

```text
kobana kadu organization update [OPTIONS]
```

`PATCH /v1/organization`

Changes **only the token's organization** (the one in the `org` claim); takes no id. `monthly_cost_limit` and `bypass_subscription` are set by Kobana and rejected (403).

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no |  |
| `timezone` | `string` | no |  |
| `default_locale` | `string` | no | Values: `pt-BR`, `en-US`, `es-ES`. |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.organization.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
