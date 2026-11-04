---
id: pix-keys
title: "kobana billing pix-keys"
sidebar_label: "pix-keys"
---

# kobana billing pix-keys


Commands of `kobana billing pix-keys`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`pix-keys list`](#pix-keys-list) | `GET /v1/pix_keys` | List Pix keys |
| [`pix-keys create`](#pix-keys-create) | `POST /v1/pix_keys` | Create a Pix key |
| [`pix-keys get`](#pix-keys-get) | `GET /v1/pix_keys/{id}` | Get a Pix key |
| [`pix-keys update`](#pix-keys-update) | `PATCH /v1/pix_keys/{id}` | Update a Pix key |
| [`pix-keys delete`](#pix-keys-delete) | `DELETE /v1/pix_keys/{id}` | Delete a Pix key |

## pix-keys list

List Pix keys

```text
kobana billing pix-keys list [OPTIONS]
```

`GET /v1/pix_keys`

The organization Pix keys customers transfer to on a `bank_transfer` charge (`POST /invoices/{id}/pay`).

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `active` | `string` | no | Filter by active (`true`) or inactive (`false`) keys. Values: `true`, `false`. |
| `key_type` | `string` | no | Filter by key type. Values: `cpf`, `cnpj`, `email`, `phone`, `random`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of Pix keys. |
| `404` | Resource not found in the current organization. |

## pix-keys create

Create a Pix key

```text
kobana billing pix-keys create [OPTIONS]
```

`POST /v1/pix_keys`

The type is detected from the key itself (CPF, CNPJ, email, phone with +55 or random); `key_type`, when sent, must match. The key is stored unmasked and is unique in the organization.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | yes | The key, unmasked (documents as digits only, phone with `+55`, email and random key lowercased). |
| `key_type` | `string` | no | Key type: `cpf`, `cnpj`, `email`, `phone` or `random`. Values: `cpf`, `cnpj`, `email`, `phone`, `random`. |
| `description` | `string` | no | Free description for the operator (e.g. "Main Pix"). |
| `holder_name` | `string` | no | Holder shown to the customer. Empty uses the legal name of the invoice issuing company (or the organization). |
| `holder_document` | `string` | no | Holder CPF/CNPJ, digits only. Empty uses the document of the issuing company (or the organization). |
| `company_id` | `string (uuid)` | no | Issuing company it belongs to (optional). |
| `active` | `boolean` | no | Only active destinations can be chosen for a new charge. |

More fields and nested objects: `kobana schema billing.pix-keys.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Pix key created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## pix-keys get

Get a Pix key

```text
kobana billing pix-keys get [ID] [OPTIONS]
```

`GET /v1/pix_keys/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix key found. |
| `404` | Resource not found in the current organization. |

## pix-keys update

Update a Pix key

```text
kobana billing pix-keys update [ID] [OPTIONS]
```

`PATCH /v1/pix_keys/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | `string` | no | The key, unmasked (documents as digits only, phone with `+55`, email and random key lowercased). |
| `key_type` | `string` | no | Key type: `cpf`, `cnpj`, `email`, `phone` or `random`. Values: `cpf`, `cnpj`, `email`, `phone`, `random`. |
| `description` | `string` | no | Free description for the operator (e.g. "Main Pix"). |
| `holder_name` | `string` | no | Holder shown to the customer. Empty uses the legal name of the invoice issuing company (or the organization). |
| `holder_document` | `string` | no | Holder CPF/CNPJ, digits only. Empty uses the document of the issuing company (or the organization). |
| `company_id` | `string (uuid)` | no | Issuing company it belongs to (optional). |
| `active` | `boolean` | no | Only active destinations can be chosen for a new charge. |

More fields and nested objects: `kobana schema billing.pix-keys.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix key updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## pix-keys delete

Delete a Pix key

```text
kobana billing pix-keys delete [ID] [OPTIONS]
```

`DELETE /v1/pix_keys/{id}`

Soft delete. Charges already sent keep showing the details the customer received, and the key can be registered again.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pix key deleted. |
| `404` | Resource not found in the current organization. |
