---
id: imports
title: "kobana banking imports"
sidebar_label: "imports"
---

# kobana banking imports


Commands of `kobana banking imports`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports/{collection_name}` | List Imports |
| [`imports create`](#imports-create) | `POST /v1/imports/{collection_name}` | Import |
| [`imports get`](#imports-get) | `GET /v1/imports/{collection_name}/{id}` | View Import |

## imports list

List Imports

```text
kobana banking imports list [COLLECTION_NAME] [OPTIONS]
```

`GET /v1/imports/{collection_name}`

Lists the imports performed for a specific type of resource (customers, subscriptions, billets or carnês).

**Path arguments:** `collection_name`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Imports found |

## imports create

Import

```text
kobana banking imports create [COLLECTION_NAME] [OPTIONS]
```

`POST /v1/imports/{collection_name}`

Imports resources (customers, subscriptions, billets or carnês) through a CSV file. Processing is done asynchronously.

> **Asynchronous Processing:** This endpoint is processed asynchronously.

The Import is done asynchronously, that is, when receiving the request,
our API saves the request parameters and processes the import in the background.
After the import is processed, the records will be associated with your Account.
You receive in this request response the ID of the Import created and can consult it at any time.

**Webhook Events**

When the request is made, an `import.created` event is triggered.
Our server then begins processing the import.
After processing is complete, the `import.processed` event is triggered.

**Path arguments:** `collection_name`

**Request body**

Sent as a `multipart/form-data` form: files go in `--file FIELD=PATH`.

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | `string (binary)` | yes | File for import. |

More fields and nested objects: `kobana schema banking.imports.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Import created |
| `422` | An import is already in progress or completed for this file |
| `503` | Could not check for duplicate imports right now — please try again |

## imports get

View Import

```text
kobana banking imports get [COLLECTION_NAME] [ID] [OPTIONS]
```

`GET /v1/imports/{collection_name}/{id}`

Query specific information from an Import.

**Path arguments:** `collection_name`, `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Import found |
| `404` | Account not found |
