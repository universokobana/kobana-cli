---
id: bank-billet-batches
title: "kobana banking bank-billet-batches"
sidebar_label: "bank-billet-batches"
---

# kobana banking bank-billet-batches


Commands of `kobana banking bank-billet-batches`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`bank-billet-batches list`](#bank-billet-batches-list) | `GET /v1/bank_billet_batches` | List Batches |
| [`bank-billet-batches create`](#bank-billet-batches-create) | `POST /v1/bank_billet_batches` | Create Batch |
| [`bank-billet-batches get`](#bank-billet-batches-get) | `GET /v1/bank_billet_batches/{id}/` | Batch Information |
| [`bank-billet-batches delete`](#bank-billet-batches-delete) | `DELETE /v1/bank_billet_batches/{id}/` | Delete the batch |
| [`bank-billet-batches add-bank-billets`](#bank-billet-batches-add-bank-billets) | `PUT /v1/bank_billet_batches/{id}/add_bank_billets` | Add Billets to Batch |
| [`bank-billet-batches pdf`](#bank-billet-batches-pdf) | `POST /v1/bank_billet_batches/{id}/pdf` | Export Batch in .pdf Format |
| [`bank-billet-batches remove-bank-billet`](#bank-billet-batches-remove-bank-billet) | `DELETE /v1/bank_billet_batches/{id}/remove_bank_billet` | Remove Billet from Batch |
| [`bank-billet-batches zip`](#bank-billet-batches-zip) | `POST /v1/bank_billet_batches/{id}/zip` | Export Batch in .zip Format |

## bank-billet-batches list

List Batches

```text
kobana banking bank-billet-batches list [OPTIONS]
```

`GET /v1/bank_billet_batches`

List Batches.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing batches |

## bank-billet-batches create

Create Batch

```text
kobana banking bank-billet-batches create [OPTIONS]
```

`POST /v1/bank_billet_batches`

Create Batch.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Batch name. |
| `bank_billet_ids` | `string` | no | IDs of Billets separated by comma. |

More fields and nested objects: `kobana schema banking.bank-billet-batches.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Batch created successfully |
| `422` | Invalid data |

## bank-billet-batches get

Batch Information

```text
kobana banking bank-billet-batches get [ID] [OPTIONS]
```

`GET /v1/bank_billet_batches/{id}/`

Query specific information from a Batch.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Batch found |
| `404` | Batch not found |

## bank-billet-batches delete

Delete the batch

```text
kobana banking bank-billet-batches delete [ID] [OPTIONS]
```

`DELETE /v1/bank_billet_batches/{id}/`

Delete batch.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Batch deleted |
| `404` | Batch not found |

## bank-billet-batches add-bank-billets

Add Billets to Batch

```text
kobana banking bank-billet-batches add-bank-billets [ID] [OPTIONS]
```

`PUT /v1/bank_billet_batches/{id}/add_bank_billets`

Add billets to batch.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `bank_billet_ids` | `string` | yes | IDs of Billets separated by comma. |

More fields and nested objects: `kobana schema banking.bank-billet-batches.add-bank-billets`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Billets added successfully |
| `404` | Batch not found |
| `422` | Invalid data |

## bank-billet-batches pdf

Export Batch in .pdf Format

```text
kobana banking bank-billet-batches pdf [ID] [OPTIONS]
```

`POST /v1/bank_billet_batches/{id}/pdf`

Export batch in .pdf format.

After the request the export enters a queue to generate the batch file. The batch file URL will be returned by the Batch Export endpoint.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `201` | Batch export request created successfully |
| `404` | Batch not found |

## bank-billet-batches remove-bank-billet

Remove Billet from Batch

```text
kobana banking bank-billet-batches remove-bank-billet [ID] [OPTIONS]
```

`DELETE /v1/bank_billet_batches/{id}/remove_bank_billet`

Remove billet from batch.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `bank_billet_id` | `integer` | yes | ID of Billet separated by comma. |

More fields and nested objects: `kobana schema banking.bank-billet-batches.remove-bank-billet`.

**Responses**

| HTTP | Description |
|---|---|
| `204` | Billet removed successfully |
| `404` | Batch not found |

## bank-billet-batches zip

Export Batch in .zip Format

```text
kobana banking bank-billet-batches zip [ID] [OPTIONS]
```

`POST /v1/bank_billet_batches/{id}/zip`

Export batch in .zip format.

After the request the export enters a queue to generate the batch file. The batch file URL will be returned by the Batch Export endpoint.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `201` | Batch export request created successfully |
| `404` | Batch not found |
