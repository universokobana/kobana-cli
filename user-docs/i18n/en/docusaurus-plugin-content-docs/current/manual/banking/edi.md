---
id: edi
title: "kobana banking edi"
sidebar_label: "edi"
description: "EDI (EDI boxes)"
---

# kobana banking edi

EDI (EDI boxes)

Commands of `kobana banking edi`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`edi boxes list`](#edi-boxes-list) | `GET /v2/edi/edi_boxes` | List Mailboxes |
| [`edi boxes create`](#edi-boxes-create) | `POST /v2/edi/edi_boxes` | Create Mailbox |
| [`edi boxes get`](#edi-boxes-get) | `GET /v2/edi/edi_boxes/{uid}` | View Mailbox |
| [`edi boxes update`](#edi-boxes-update) | `PUT /v2/edi/edi_boxes/{uid}` | Update Mailbox |

## edi boxes list

List Mailboxes

```text
kobana banking edi boxes list [OPTIONS]
```

`GET /v2/edi/edi_boxes`

Lists Mailboxes.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `resource_type` | `string` | no | Filter by Resource Type. Billing Portfolio (`charge.bank_billet_account`) or Financial Account (`financial.account`). |
| `per_page` | `integer` | no | Number of records per page. Maximum of 50. |
| `page` | `integer` | no | Page number. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Mailboxes found. |

## edi boxes create

Create Mailbox

```text
kobana banking edi boxes create [OPTIONS]
```

`POST /v2/edi/edi_boxes`

Creates a new Mailbox.

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.edi.boxes.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Mailbox created |
| `422` | Validation failure |

## edi boxes get

View Mailbox

```text
kobana banking edi boxes get [UID] [OPTIONS]
```

`GET /v2/edi/edi_boxes/{uid}`

Consults information of a specific Mailbox.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Mailbox found |
| `404` | Mailbox not found |

## edi boxes update

Update Mailbox

```text
kobana banking edi boxes update [UID] [OPTIONS]
```

`PUT /v2/edi/edi_boxes/{uid}`

Updates a specific Mailbox.

**Path arguments:** `uid`

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.edi.boxes.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Mailbox updated |
| `404` | Mailbox not found |
| `422` | Validation failure |
