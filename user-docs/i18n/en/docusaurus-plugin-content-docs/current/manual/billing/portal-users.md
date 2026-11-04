---
id: portal-users
title: "kobana billing portal-users"
sidebar_label: "portal-users"
description: "List portal users"
---

# kobana billing portal-users

List portal users

Commands of `kobana billing portal-users`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`portal-users`](#portal-users) | `GET /v1/portal_users` | List portal users |

## portal-users

List portal users

```text
kobana billing portal-users [OPTIONS]
```

`GET /v1/portal_users`

Returns the customer portal users in the authenticated organization, paginated. Sensitive fields (password, tokens) are never exposed.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Scope to a specific billing account. |
| `customer_id` | `string` | no | Scope to every billing account of a customer. Accepts the customer's `id` or its `external_id`. |
| `status` | `string` | no | Filter by status: `active`, `pending_invitation`, `pending_verification`, `suspended` or `locked`. Values: `active`, `pending_invitation`, `pending_verification`, `suspended`, `locked`. |
| `email` | `string` | no | Substring match on the email. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of portal users. |
| `404` | Resource not found in the current organization. |
