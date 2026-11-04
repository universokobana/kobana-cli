---
id: portal-settings
title: "kobana dashboards portal-settings"
sidebar_label: "portal-settings"
description: "Get portal settings"
---

# kobana dashboards portal-settings

Get portal settings

Commands of `kobana dashboards portal-settings`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`portal-settings`](#portal-settings) | `GET /v1/portal_settings` | Get portal settings |
| [`portal-settings domain check-slug`](#portal-settings-domain-check-slug) | `GET /v1/portal_settings/domain/check_slug` | Check address availability |
| [`portal-settings domain update`](#portal-settings-domain-update) | `PATCH /v1/portal_settings/domain` | Set portal address |
| [`portal-settings logo create`](#portal-settings-logo-create) | `POST /v1/portal_settings/logo` | Upload portal image |
| [`portal-settings logo delete`](#portal-settings-logo-delete) | `DELETE /v1/portal_settings/logo` | Delete portal image |
| [`portal-settings update`](#portal-settings-update) | `PATCH /v1/portal_settings` | Update portal settings |
| [`portal-settings verify-sender`](#portal-settings-verify-sender) | `POST /v1/portal_settings/verify_sender` | Verify the notifications sender e-mail |

## portal-settings

Get portal settings

```text
kobana dashboards portal-settings [OPTIONS] [COMMAND]
```

`GET /v1/portal_settings`

Returns the workspace portal's configuration: branding, page language, colors, address, login and SSO options, notifications and sender, and site settings (security headers, SEO, trackers, chat and custom code).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The workspace portal's configuration, with defaults applied. |
| `404` | Resource not found in the current organization. |

## portal-settings domain check-slug

Check address availability

```text
kobana dashboards portal-settings domain check-slug [OPTIONS]
```

`GET /v1/portal_settings/domain/check_slug`

Tells whether `slug` is free under `domain` for this workspace's portal within the organization (its own current address counts as free).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `slug` | `string` | yes | Workspace slug; the address is `{organization_slug}-{slug}.{domain}` (a single label, up to 63 characters): 3 to 50 lowercase letters, digits and hyphens (no hyphen at either end). `null` to remove. |
| `domain` | `string` | yes | One of the platform domains (`available_domains`). `null` to remove. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Result of the check. |
| `400` | Invalid request (missing or malformed parameter). |
| `404` | Resource not found in the current organization. |

## portal-settings domain update

Set portal address

```text
kobana dashboards portal-settings domain update [OPTIONS]
```

`PATCH /v1/portal_settings/domain`

Sets the portal's platform address, `https://{organization_slug}-{slug}.{domain}` (the organization slug is fixed here), or removes it when both are `null`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `slug` | `string` | yes | Workspace slug; the address is `{organization_slug}-{slug}.{domain}` (a single label, up to 63 characters): 3 to 50 lowercase letters, digits and hyphens (no hyphen at either end). `null` to remove. |
| `domain` | `string` | yes | One of the platform domains (`available_domains`). `null` to remove. |

More fields and nested objects: `kobana schema dashboards.portal-settings.domain.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Address updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Refused by a business rule (address taken or reserved, file too large or of an unaccepted type…). |

## portal-settings logo create

Upload portal image

```text
kobana dashboards portal-settings logo create [OPTIONS]
```

`POST /v1/portal_settings/logo`

Uploads the portal's logo, dark-background logo or favicon, as base64 in the JSON body. Replaces the previous image.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `filename` | `string` | yes |  |
| `content_type` | `string` | yes | `image/png`, `image/jpeg` or `image/webp`; the favicon also accepts `image/x-icon`. SVG is not accepted. |
| `content_base64` | `string` | yes | File content in base64 (up to 500 KB once decoded). |
| `variant` | `string` | no | Which image: `logo` (default), `logo_light` (logo for dark backgrounds) or `favicon`. Values: `logo`, `logo_light`, `favicon`. |

More fields and nested objects: `kobana schema dashboards.portal-settings.logo.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Image saved; `url` is its public address. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Refused by a business rule (address taken or reserved, file too large or of an unaccepted type…). |

## portal-settings logo delete

Delete portal image

```text
kobana dashboards portal-settings logo delete [OPTIONS]
```

`DELETE /v1/portal_settings/logo`

Removes the logo, dark-background logo or favicon; the portal falls back to its default.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `variant` | `string` | no | Which image: `logo` (default), `logo_light` (logo for dark backgrounds) or `favicon`. Values: `logo`, `logo_light`, `favicon`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Image removed. |
| `400` | Invalid request (missing or malformed parameter). |
| `404` | Resource not found in the current organization. |

## portal-settings update

Update portal settings

```text
kobana dashboards portal-settings update [OPTIONS]
```

`PATCH /v1/portal_settings`

Updates the workspace portal's configuration. Partial: only the fields sent change. Returns the full configuration.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_name` | `string` | no |  |
| `title` | `string` | no |  |
| `subtitle` | `string` | no |  |
| `html_lang` | `string` | no | The portal language: it goes in the `lang` of the pages' `<html>` and translates the login screens. Values: `pt-BR`, `en-US`, `es-ES`. |
| `light_colors` | `object` | no | Theme colors, in HSL without the `hsl()` wrapper (e.g. `210 40% 98%`). Omitted keys use the default. |
| `dark_colors` | `object` | no | Theme colors, in HSL without the `hsl()` wrapper (e.g. `210 40% 98%`). Omitted keys use the default. |
| `enable_login` | `boolean` | no |  |
| `enable_email_login` | `boolean` | no |  |
| `allow_password_change` | `boolean` | no |  |
| `sso_enabled` | `boolean` | no |  |
| `sso_public_key` | `string` | no |  |
| `sso_algorithm` | `string` | no | Values: `RS256`, `RS384`, `RS512`, `ES256`, `ES384`, `ES512`, `PS256`, `PS384`, `PS512`. |
| `sso_issuer` | `string` | no |  |
| `notification_type_statuses` | `object` | no | Status of each portal notification: `ACTIVE` (sent), `INACTIVE` (silenced) or `DISABLED` (unavailable). |
| `notification_bcc_emails` | `array<object>` | no | Replaces the whole list (up to 20). |
| `notification_sender_name` | `string` | no |  |
| `notification_sender_email` | `string` | no | A new address starts its AWS SES verification, which e-mails it a confirmation link. `""` reverts to the platform sender. |
| `notification_reply_to_email` | `string` | no |  |
| `hsts_include_subdomains` | `boolean` | no |  |
| `hsts_preload` | `boolean` | no |  |
| `secure_frame_headers` | `boolean` | no |  |
| `seo_indexing_enabled` | `boolean` | no |  |
| `seo_robots_txt` | `string` | no |  |
| `seo_sitemap_enabled` | `boolean` | no |  |
| `seo_google_site_verification` | `string` | no | Only the content of the Google verification meta tag. |
| `seo_canonical_url` | `string` | no |  |
| `tracking_integrations` | `object` | no | Trackers per provider (`google_tag_manager`, `google_analytics`, …): `{ provider: { enabled, ...ids } }`. Unknown providers and fields are dropped. |
| `chat_integrations` | `object` | no | Chat widgets per provider (`intercom`, `hubspot`, `zendesk`, …), same shape as `tracking_integrations`. |
| `custom_code` | `string` | no |  |

More fields and nested objects: `kobana schema dashboards.portal-settings.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The workspace portal's configuration, with defaults applied. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Refused by a business rule (address taken or reserved, file too large or of an unaccepted type…). |

## portal-settings verify-sender

Verify the notifications sender e-mail

```text
kobana dashboards portal-settings verify-sender [OPTIONS]
```

`POST /v1/portal_settings/verify_sender`

Requests or re-checks the AWS SES verification of the portal's notification sender e-mail. When the address was never submitted to SES or its verification failed, it asks SES to verify it (SES e-mails the address a link); otherwise it polls the current state. Returns `status` (`pending`, `active` or `failed`) and `verification_requested`. Replaces `POST /portal_settings/sender_identity`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Verification status. |
| `404` | Resource not found in the current organization. |
