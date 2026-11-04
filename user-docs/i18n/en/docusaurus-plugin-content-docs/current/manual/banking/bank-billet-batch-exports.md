---
id: bank-billet-batch-exports
title: "kobana banking bank-billet-batch-exports"
sidebar_label: "bank-billet-batch-exports"
---

# kobana banking bank-billet-batch-exports


Commands of `kobana banking bank-billet-batch-exports`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`bank-billet-batch-exports get`](#bank-billet-batch-exports-get) | `GET /v1/bank_billet_batch_exports/{id}` | Batch Export File |

## bank-billet-batch-exports get

Batch Export File

```text
kobana banking bank-billet-batch-exports get [ID] [OPTIONS]
```

`GET /v1/bank_billet_batch_exports/{id}`

Batch export file.

If the file has not yet been processed the response will be 404. If the file is already ready for download the response will be a redirect to the file download.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `302` | Batch export generated successfully |
| `404` | Batch export not processed |
