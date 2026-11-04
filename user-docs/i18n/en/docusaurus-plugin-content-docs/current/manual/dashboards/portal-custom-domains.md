---
id: portal-custom-domains
title: "kobana dashboards portal-custom-domains"
sidebar_label: "portal-custom-domains"
---

# kobana dashboards portal-custom-domains


Commands of `kobana dashboards portal-custom-domains`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`portal-custom-domains list`](#portal-custom-domains-list) | `GET /v1/portal_custom_domains` | List custom domains |
| [`portal-custom-domains create`](#portal-custom-domains-create) | `POST /v1/portal_custom_domains` | Add custom domain |
| [`portal-custom-domains delete`](#portal-custom-domains-delete) | `DELETE /v1/portal_custom_domains/{id}` | Delete custom domain |
| [`portal-custom-domains dns-status`](#portal-custom-domains-dns-status) | `GET /v1/portal_custom_domains/{id}/dns_status` | Check custom domain DNS |
| [`portal-custom-domains verify`](#portal-custom-domains-verify) | `POST /v1/portal_custom_domains/{id}/verify` | Verify and provision custom domain |

## portal-custom-domains list

List custom domains

```text
kobana dashboards portal-custom-domains list [OPTIONS]
```

`GET /v1/portal_custom_domains`

Lists the custom domains of the workspace's portal (the whole set, unpaginated). Empty where custom domains are not supported.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The portal's custom domains. |
| `404` | Resource not found in the current organization. |

## portal-custom-domains create

Add custom domain

```text
kobana dashboards portal-custom-domains create [OPTIONS]
```

`POST /v1/portal_custom_domains`

Registers a custom domain as `pending_verification`, without touching AWS, and returns the DNS records to publish. Then call `POST /portal_custom_domains/{id}/verify`. Registering is only a claim: the domain goes to the first portal that publishes its verification TXT (the other unproven claims are released), and an unproven claim expires after 7 days. A portal holds at most 10 domains. Platform domains (`kobana.com.br`, `kobana.tech` and the shared portal domains) are refused.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `hostname` | `string` | yes | Fully qualified hostname, e.g. `portal.example.com`. |

More fields and nested objects: `kobana schema dashboards.portal-custom-domains.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | The custom domain. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | This domain is already registered (proven by another portal, or already claimed by this one). |
| `422` | Reserved platform domain (`hostname_reserved`) or the portal reached its domain limit (`domain_limit_reached`). |

## portal-custom-domains delete

Delete custom domain

```text
kobana dashboards portal-custom-domains delete [ID] [OPTIONS]
```

`DELETE /v1/portal_custom_domains/{id}`

Removes the custom domain and tears down its CloudFront tenant and certificate when provisioned. The portal stops answering on that hostname.

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

## portal-custom-domains dns-status

Check custom domain DNS

```text
kobana dashboards portal-custom-domains dns-status [ID] [OPTIONS]
```

`GET /v1/portal_custom_domains/{id}/dns_status`

Read-only lookup of the required CNAMEs, record by record, without changing anything on AWS.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | DNS status. |
| `404` | Resource not found in the current organization. |

## portal-custom-domains verify

Verify and provision custom domain

```text
kobana dashboards portal-custom-domains verify [ID] [OPTIONS]
```

`POST /v1/portal_custom_domains/{id}/verify`

While pending, checks the DNS and provisions the domain on AWS (certificate and CloudFront tenant); once provisioned, refreshes its status. Returns the domain with its new `status`.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on: the `id` or the `public_id` shown under Workspaces on the dashboard. Inside the token's organization, data lives in workspaces. It may be omitted while the organization has a single workspace; with two or more, leaving it out returns `400 workspace_required`. A workspace outside the organization returns `403 unknown_workspace`. With a person token, only the workspaces that person has access to are reachable (organization owners reach all of them); any other returns `403 workspace_access_denied`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The custom domain. |
| `404` | Resource not found in the current organization. |
