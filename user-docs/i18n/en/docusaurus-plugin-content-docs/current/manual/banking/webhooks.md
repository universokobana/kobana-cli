---
id: webhooks
title: "kobana banking webhooks"
sidebar_label: "webhooks"
---

# kobana banking webhooks


Commands of `kobana banking webhooks`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | List Webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Create Webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Webhook Information |
| [`webhooks update`](#webhooks-update) | `PUT /v1/webhooks/{id}` | Update Webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Delete Webhook |

## webhooks list

List Webhooks

```text
kobana banking webhooks list [OPTIONS]
```

`GET /v1/webhooks`

List webhooks.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing webhooks |

## webhooks create

Create Webhook

```text
kobana banking webhooks create [OPTIONS]
```

`POST /v1/webhooks`

Creates a webhook to receive system event notifications via HTTP.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `url` | `string` | yes | Address where requests will be made |
| `events` | `array<string>` | yes | Events that should generate notifications for the webhook. Choose one or more events. |
| `id` | `integer` | no | Webhook ID |
| `name` | `string` | no | Webhook identification name |
| `content_type` | `string` | no | Content-type of the request that will be made. Valid values `application/json` (default) or `application/x-www-form-urlencoded`. |
| `active` | `boolean` | no | Defines whether the webhook will be active, receiving notifications. The default value is `true`. |
| `ssl_verification_enabled` | `boolean` | no | Defines whether SSL verification is enabled on the webhook. |
| `subaccounts` | `boolean` | no | Defines whether the webhook will be active for subaccount events. |
| `bank_billet_account_id` | `integer` | no | ID of the Billing Account. If provided, the webhook will only receive notifications for events related to the specified billing account. |
| `auth_kind` | `string` | no | Type of authentication configured. Returned in responses. To configure, send the `auth` object in the body. Values: `none`, `basic_auth`, `bearer`, `api_key`. |
| `auth` | `object` | no | Authentication configuration of the receiving endpoint. For creation/update only — not returned in responses. |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.webhooks.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Webhook created with Basic Auth |
| `422` | Invalid webhook |

## webhooks get

Webhook Information

```text
kobana banking webhooks get [ID] [OPTIONS]
```

`GET /v1/webhooks/{id}`

Query specific information from a Webhook.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Webhook found |
| `404` | Webhook not found |

## webhooks update

Update Webhook

```text
kobana banking webhooks update [ID] [OPTIONS]
```

`PUT /v1/webhooks/{id}`

Updates specific information of a webhook.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `url` | `string` | yes | Address where requests will be made |
| `events` | `array<string>` | yes | Events that should generate notifications for the webhook. Choose one or more events. |
| `id` | `integer` | no | Webhook ID |
| `name` | `string` | no | Webhook identification name |
| `content_type` | `string` | no | Content-type of the request that will be made. Valid values `application/json` (default) or `application/x-www-form-urlencoded`. |
| `active` | `boolean` | no | Defines whether the webhook will be active, receiving notifications. The default value is `true`. |
| `ssl_verification_enabled` | `boolean` | no | Defines whether SSL verification is enabled on the webhook. |
| `subaccounts` | `boolean` | no | Defines whether the webhook will be active for subaccount events. |
| `bank_billet_account_id` | `integer` | no | ID of the Billing Account. If provided, the webhook will only receive notifications for events related to the specified billing account. |
| `auth_kind` | `string` | no | Type of authentication configured. Returned in responses. To configure, send the `auth` object in the body. Values: `none`, `basic_auth`, `bearer`, `api_key`. |
| `auth` | `object` | no | Authentication configuration of the receiving endpoint. For creation/update only — not returned in responses. |
| `created_at` | `string (date-time)` | no | Date and time of record creation |
| `updated_at` | `string (date-time)` | no | Date and time of last record update |

More fields and nested objects: `kobana schema banking.webhooks.update`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Partial update of auth preserves existing secret fields |
| `404` | Webhook not found |

## webhooks delete

Delete Webhook

```text
kobana banking webhooks delete [ID] [OPTIONS]
```

`DELETE /v1/webhooks/{id}`

Deletes a specific webhook

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Webhook deleted |
| `404` | Webhook not found |
