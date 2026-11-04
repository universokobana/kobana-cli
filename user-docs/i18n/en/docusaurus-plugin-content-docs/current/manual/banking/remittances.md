---
id: remittances
title: "kobana banking remittances"
sidebar_label: "remittances"
---

# kobana banking remittances


Commands of `kobana banking remittances`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`remittances list`](#remittances-list) | `GET /v1/remittances` | List CNABs |
| [`remittances create`](#remittances-create) | `POST /v1/remittances` | Create CNAB |
| [`remittances bulk`](#remittances-bulk) | `POST /v1/remittances/bulk` | Create CNABs in Batch |
| [`remittances get`](#remittances-get) | `GET /v1/remittances/{id}` | CNAB Information |
| [`remittances delete`](#remittances-delete) | `DELETE /v1/remittances/{id}` | Delete CNAB |
| [`remittances raw`](#remittances-raw) | `GET /v1/remittances/{id}/raw` | Raw(text/plain) of CNAB |

## remittances list

List CNABs

```text
kobana banking remittances list [OPTIONS]
```

`GET /v1/remittances`

List CNABs.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `expire_from` | `string` | no | From Billet Due Date. |
| `expire_to` | `string` | no | To Billet Due Date. |
| `created_from` | `string` | no | From Remittance Creation Date. |
| `created_to` | `string` | no | To Remittance Creation Date. |
| `bank_billet_account_id` | `string` | no | Filter by Wallet ID. |
| `our_code` | `string` | no | Filter by Our Number. |
| `tags` | `string` | no | Filter by tags (separated by comma). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing CNABs |

## remittances create

Create CNAB

```text
kobana banking remittances create [OPTIONS]
```

`POST /v1/remittances`

Create CNAB.

A Remittance request will be made. After Remittance creation, it will be processed and at the end of processing notifications will be issued through Webhooks.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `bank_billet_account_id` | `integer` | no | ID of the Billing Account. |
| `bank_billet_remittance_ids` | `array<integer>` | no | IDs of Remittance Records. |
| `external_id` | `string` | no | External ID. |
| `custom_data` | `object` | no | Custom data. |
| `tags` | `array<string>` | no | Tags. |

More fields and nested objects: `kobana schema banking.remittances.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | CNAB created with custom_data and tags |
| `404` | Remittance not found |

## remittances bulk

Create CNABs in Batch

```text
kobana banking remittances bulk [OPTIONS]
```

`POST /v1/remittances/bulk`

Create CNABs in Batch.

Remittance requests will be made according to occurrence IDs. A Remittance request will be created for each wallet associated with the occurrences. After Remittance creation, they will be processed and at the end of processing notifications will be issued through Webhooks.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `bank_billet_remittance_ids` | `array<integer>` | no | IDs of Remittance Records. |

More fields and nested objects: `kobana schema banking.remittances.bulk`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | CNABs created |
| `404` | Occurrence not found |

## remittances get

CNAB Information

```text
kobana banking remittances get [ID] [OPTIONS]
```

`GET /v1/remittances/{id}`

Query specific information from a CNAB.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | CNAB found |
| `404` | CNAB not found |

## remittances delete

Delete CNAB

```text
kobana banking remittances delete [ID] [OPTIONS]
```

`DELETE /v1/remittances/{id}`

Delete CNAB.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | CNAB deleted |
| `404` | CNAB not found |

## remittances raw

Raw(text/plain) of CNAB

```text
kobana banking remittances raw [ID] [OPTIONS]
```

`GET /v1/remittances/{id}/raw`

Raw(text/plain) of CNAB.

The remittance.downloaded event will be triggered.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | CNAB Raw |
| `404` | CNAB not found |
| `422` | Remittance awaiting processing |
