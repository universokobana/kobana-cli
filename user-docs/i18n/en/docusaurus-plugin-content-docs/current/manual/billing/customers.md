---
id: customers
title: "kobana billing customers"
sidebar_label: "customers"
---

# kobana billing customers


Commands of `kobana billing customers`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`customers statement sync`](#customers-statement-sync) | `POST /v1/customers/{id}/statement/sync` | Rebuild the statement from history |
| [`customers statement list`](#customers-statement-list) | `GET /v1/customers/{id}/statement` | List customer financial statement |
| [`customers users list`](#customers-users-list) | `GET /v1/customers/{id}/users` | List customer portal users |
| [`customers users create`](#customers-users-create) | `POST /v1/customers/{id}/users` | Invite portal user |
| [`customers users invite`](#customers-users-invite) | `POST /v1/customers/{id}/users/invite` | Invite portal user (verb) |
| [`customers users get`](#customers-users-get) | `GET /v1/customers/{id}/users/{user_id}` | Get portal user |
| [`customers users update`](#customers-users-update) | `PATCH /v1/customers/{id}/users/{user_id}` | Update portal user |
| [`customers users delete`](#customers-users-delete) | `DELETE /v1/customers/{id}/users/{user_id}` | Delete portal user |
| [`customers users email-preview`](#customers-users-email-preview) | `GET /v1/customers/{id}/users/{user_id}/email-preview` | Preview portal user email |
| [`customers users resend`](#customers-users-resend) | `POST /v1/customers/{id}/users/{user_id}/resend` | Resend invitation or verification email |
| [`customers list`](#customers-list) | `GET /v1/customers` | List customers |
| [`customers create`](#customers-create) | `POST /v1/customers` | Create customer |
| [`customers update`](#customers-update) | `PUT /v1/customers` | Upsert customer |
| [`customers get`](#customers-get) | `GET /v1/customers/{id}` | Get customer |
| [`customers delete`](#customers-delete) | `DELETE /v1/customers/{id}` | Remove customer (soft delete) |
| [`customers recalculate-mrr`](#customers-recalculate-mrr) | `POST /v1/customers/{id}/recalculate-mrr` | Recalculate customer MRR |

## customers statement sync

Rebuild the statement from history

```text
kobana billing customers statement sync [ID] [OPTIONS]
```

`POST /v1/customers/{id}/statement/sync`

Recreates statement items from existing invoices, payments and credit applications. Returns a summary of what was created.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | Required when the customer has multiple billing accounts. |

More fields and nested objects: `kobana schema billing.customers.statement.sync`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Statement rebuilt. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## customers statement list

List customer financial statement

```text
kobana billing customers statement list [ID] [OPTIONS]
```

`GET /v1/customers/{id}/statement`

Returns statement entries (invoices, payments, credit applications) for the customer's billing account. When the customer has multiple accounts, provide `billing_account_id`.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Required when the customer has multiple billing accounts. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of statement items. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## customers users list

List customer portal users

```text
kobana billing customers users list [ID] [OPTIONS]
```

`GET /v1/customers/{id}/users`

Returns portal users linked to the customer's billing account.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `status` | `string` | no | Filter portal users by status. Values: `active`, `pending_verification`, `pending_invitation`, `suspended`, `locked`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of portal users. |
| `404` | Resource not found in the current organization. |

## customers users create

Invite portal user

```text
kobana billing customers users create [ID] [OPTIONS]
```

`POST /v1/customers/{id}/users`

Creates a portal access invitation for the given email. The invitation expires in 7 days.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string` | yes | Invitee's email (required). |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.customers.users.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Invitation created and email queued for sending. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `409` | A user or invitation already exists for this email. |

## customers users invite

Invite portal user (verb)

```text
kobana billing customers users invite [ID] [OPTIONS]
```

`POST /v1/customers/{id}/users/invite`

Equivalent to `POST /customers/{id}/users`. Available as a verb route for SDKs that prefer descriptive URLs.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string` | yes | Invitee's email (required). |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.customers.users.invite`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Invitation created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## customers users get

Get portal user

```text
kobana billing customers users get [ID] [USER_ID] [OPTIONS]
```

`GET /v1/customers/{id}/users/{user_id}`

The `{user_id}` parameter accepts the user's UUID or `external_id`.

**Path arguments:** `id`, `user_id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Portal user detail. |
| `404` | Resource not found in the current organization. |

## customers users update

Update portal user

```text
kobana billing customers users update [ID] [USER_ID] [OPTIONS]
```

`PATCH /v1/customers/{id}/users/{user_id}`

Updates `email`, `external_id`, `sso_only` and `status`. Allowed status transitions: `active` ↔ `suspended`.

**Path arguments:** `id`, `user_id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `email` | `string` | no |  |
| `external_id` | `string` | no |  |
| `sso_only` | `boolean` | no |  |
| `status` | `string` | no | Allowed transitions: `active` ↔ `suspended`. Values: `active`, `suspended`. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.customers.users.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | User updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## customers users delete

Delete portal user

```text
kobana billing customers users delete [ID] [USER_ID] [OPTIONS]
```

`DELETE /v1/customers/{id}/users/{user_id}`

Deletes the portal user (soft delete: `deleted_at`) — a pending invitation and an active user alike: access is lost at once, sessions and pending links stop working, and the email can be invited again. Requires the `delete` access level. Until October 2026 this route erased a pending invitation and only suspended an active user.

**Path arguments:** `id`, `user_id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Portal user deleted. |
| `404` | Resource not found in the current organization. |

## customers users email-preview

Preview portal user email

```text
kobana billing customers users email-preview [ID] [USER_ID] [OPTIONS]
```

`GET /v1/customers/{id}/users/{user_id}/email-preview`

Returns the subject and body of the invitation or verification email that would be sent to the user, without actually sending it. Available only for `pending_invitation` or `pending_verification` users.

**Path arguments:** `id`, `user_id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Email preview. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## customers users resend

Resend invitation or verification email

```text
kobana billing customers users resend [ID] [USER_ID] [OPTIONS]
```

`POST /v1/customers/{id}/users/{user_id}/resend`

Resends the invitation email (for `pending_invitation`, rotating the token) or verification email (for `pending_verification`). Other statuses return 400.

**Path arguments:** `id`, `user_id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Email resent. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## customers list

List customers

```text
kobana billing customers list [OPTIONS]
```

`GET /v1/customers`

Returns a paginated list of the current organization's customers.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `search` | `string` | no | Free-text search by name, document or email. |
| `kind` | `string` | no | Filter by person kind: `natural` (individual) or `juridical` (business). Values: `natural`, `juridical`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of customers. |
| `404` | Resource not found in the current organization. |

## customers create

Create customer

```text
kobana billing customers create [OPTIONS]
```

`POST /v1/customers`

Creates a customer and, by default, an associated billing account. Use `create_billing_account=false` to create only the customer.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Customer name (required). |
| `public_id` | `string (uuid)` | no | Public UUID of the customer. Generated when omitted; supply one only to carry over an identifier from your system. Must be unique. |
| `external_id` | `string` | no | External customer identifier in the source system. |
| `nickname` | `string` | no |  |
| `legal_name` | `string` | no | Legal name (for businesses). |
| `document_number` | `string` | no | CPF or CNPJ. The `document_type` is inferred from the length (≤11 → cpf, otherwise cnpj). |
| `kind` | `string` | no | Values: `natural`, `juridical`. |
| `birthday` | `string` | no | Birth date (ISO 8601). |
| `email` | `string` | no | Primary contact email. |
| `phone` | `string` | no |  |
| `notes` | `string` | no |  |
| `addresses` | `array<object>` | no |  |
| `phones` | `array<object>` | no |  |
| `emails` | `array<object>` | no |  |
| `websites` | `array<object>` | no |  |
| `tags` | `array<string>` | no |  |
| `custom_metadata` | `object` | no |  |
| `create_billing_account` | `boolean` | no | When `false`, doesn't automatically create a billing account. Default: `true`. |
| `billing_email` | `string` | no | Email for sending invoices. Default: uses `email` when omitted. |
| `payment_terms_days` | `integer` | no | Invoice due period in days. Default: the organization's configured default payment terms (10 when not configured). |
| `auto_collection` | `boolean` | no | Automatic collection on the created account. Default: `true`. |

More fields and nested objects: `kobana schema billing.customers.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Customer created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## customers update

Upsert customer

```text
kobana billing customers update [OPTIONS]
```

`PUT /v1/customers`

Takes the same body as POST and dedups on `id`, `public_id`, `external_id` and `document_number`, in that order, stopping at the first key that finds a customer. A match is updated and returns 200; otherwise the customer is created and returns 201. Updates are partial: fields absent from the body keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Customer name. Required only when no customer is found by the dedup keys. |
| `public_id` | `string (uuid)` | no | Public UUID of the customer. Locates the record and, when none is found, is written onto the created customer. |
| `external_id` | `string` | no | External customer identifier in the source system. |
| `nickname` | `string` | no |  |
| `legal_name` | `string` | no | Legal name (for businesses). |
| `document_number` | `string` | no | CPF or CNPJ. The `document_type` is inferred from the length (≤11 → cpf, otherwise cnpj). |
| `kind` | `string` | no | Values: `natural`, `juridical`. |
| `birthday` | `string` | no | Birth date (ISO 8601). |
| `email` | `string` | no | Primary contact email. |
| `phone` | `string` | no |  |
| `notes` | `string` | no |  |
| `addresses` | `array<object>` | no |  |
| `phones` | `array<object>` | no |  |
| `emails` | `array<object>` | no |  |
| `websites` | `array<object>` | no |  |
| `tags` | `array<string>` | no |  |
| `custom_metadata` | `object` | no |  |
| `create_billing_account` | `boolean` | no | When `false`, doesn't automatically create a billing account. Default: `true`. |
| `billing_email` | `string` | no | Email for sending invoices. Default: uses `email` when omitted. |
| `payment_terms_days` | `integer` | no | Invoice due period in days. Default: the organization's configured default payment terms (10 when not configured). |
| `auto_collection` | `boolean` | no | Automatic collection on the created account. Default: `true`. |
| `id` | `string (uuid)` | no | Customer UUID. Used only to locate the record — never written. |
| `custom_metadata_mode` | `string` | no | How the submitted `custom_metadata` is reconciled with the stored value. `replace` (default) overwrites the whole object; `merge` deep-merges over the current value — nested objects merge recursively, scalars and arrays overwrite the key, and `null` keys are kept (they do not delete the key). Accepted on update only. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema billing.customers.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Customer updated. |
| `201` | Customer created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |

## customers get

Get customer

```text
kobana billing customers get [ID] [OPTIONS]
```

`GET /v1/customers/{id}`

The `{id}` parameter accepts the customer UUID or its `external_id`.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Customer detail. |
| `404` | Resource not found in the current organization. |

## customers delete

Remove customer (soft delete)

```text
kobana billing customers delete [ID] [OPTIONS]
```

`DELETE /v1/customers/{id}`

Marks the customer as removed without deleting historical records.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Soft delete confirmation. |
| `404` | Resource not found in the current organization. |

## customers recalculate-mrr

Recalculate customer MRR

```text
kobana billing customers recalculate-mrr [ID] [OPTIONS]
```

`POST /v1/customers/{id}/recalculate-mrr`

Recalculates the MRR (Monthly Recurring Revenue) of the customer's billing accounts from active subscriptions.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | MRR recalculated. |
| `404` | Resource not found in the current organization. |
