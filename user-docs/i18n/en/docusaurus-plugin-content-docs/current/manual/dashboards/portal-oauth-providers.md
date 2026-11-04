---
id: portal-oauth-providers
title: "kobana dashboards portal-oauth-providers"
sidebar_label: "portal-oauth-providers"
---

# kobana dashboards portal-oauth-providers


Commands of `kobana dashboards portal-oauth-providers`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`portal-oauth-providers list`](#portal-oauth-providers-list) | `GET /v1/portal_oauth_providers` | List OIDC providers |
| [`portal-oauth-providers create`](#portal-oauth-providers-create) | `POST /v1/portal_oauth_providers` | Add OIDC provider |
| [`portal-oauth-providers test`](#portal-oauth-providers-test) | `POST /v1/portal_oauth_providers/test` | Test OIDC issuer |
| [`portal-oauth-providers update`](#portal-oauth-providers-update) | `PATCH /v1/portal_oauth_providers/{id}` | Update OIDC provider |
| [`portal-oauth-providers delete`](#portal-oauth-providers-delete) | `DELETE /v1/portal_oauth_providers/{id}` | Delete OIDC provider |

## portal-oauth-providers list

List OIDC providers

```text
kobana dashboards portal-oauth-providers list [OPTIONS]
```

`GET /v1/portal_oauth_providers`

Lists the OpenID Connect providers portal users can sign in with (the whole set, unpaginated).

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The portal's OIDC providers. |
| `404` | Resource not found in the current organization. |

## portal-oauth-providers create

Add OIDC provider

```text
kobana dashboards portal-oauth-providers create [OPTIONS]
```

`POST /v1/portal_oauth_providers`

Adds an OpenID Connect provider to the portal login.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes |  |
| `issuer_url` | `string (uri)` | yes | OpenID Connect issuer URL (https in production). |
| `client_id` | `string` | yes |  |
| `client_secret` | `string` | yes | Write-only: stored encrypted and never returned. |
| `button_label` | `string` | no | Text of the login button; the name when empty. |
| `scopes` | `string` | no | Space-separated scopes. Default: `openid email profile`. |
| `email_claim` | `string` | no | Token claim that carries the e-mail. Default: `email`. |
| `enabled` | `boolean` | no |  |

More fields and nested objects: `kobana schema dashboards.portal-oauth-providers.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | The provider (without the secret). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## portal-oauth-providers test

Test OIDC issuer

```text
kobana dashboards portal-oauth-providers test [OPTIONS]
```

`POST /v1/portal_oauth_providers/test`

Fetches the issuer's discovery document and reports its endpoints, to validate the URL before saving. A discovery failure is a 200 with `ok: false`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `issuer_url` | `string (uri)` | yes | OpenID Connect issuer URL (https in production). |

More fields and nested objects: `kobana schema dashboards.portal-oauth-providers.test`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Result of the test. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## portal-oauth-providers update

Update OIDC provider

```text
kobana dashboards portal-oauth-providers update [ID] [OPTIONS]
```

`PATCH /v1/portal_oauth_providers/{id}`

Updates an OpenID Connect provider. Partial: only the fields sent change.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no |  |
| `button_label` | `string` | no |  |
| `issuer_url` | `string (uri)` | no |  |
| `client_id` | `string` | no |  |
| `client_secret` | `string` | no | Omitted or empty keeps the current secret; a value replaces it. Never returned. Required when `issuer_url` changes (another host, port or path): without it the update is refused with 422 `CLIENT_SECRET_REQUIRED_FOR_ISSUER_CHANGE`, so the stored secret is never sent to another provider. |
| `scopes` | `string` | no |  |
| `email_claim` | `string` | no |  |
| `enabled` | `boolean` | no |  |

More fields and nested objects: `kobana schema dashboards.portal-oauth-providers.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | The provider (without the secret). |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## portal-oauth-providers delete

Delete OIDC provider

```text
kobana dashboards portal-oauth-providers delete [ID] [OPTIONS]
```

`DELETE /v1/portal_oauth_providers/{id}`

Removes the provider; its button leaves the login page and nobody can sign in through it anymore.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Removed. |
| `404` | Resource not found in the current organization. |
