---
id: imports
title: "kobana dunning imports"
sidebar_label: "imports"
---

# kobana dunning imports


Commands of `kobana dunning imports`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports` | List imports |
| [`imports create`](#imports-create) | `POST /v1/imports` | Create import |
| [`imports get`](#imports-get) | `GET /v1/imports/{id}` | Get import |

## imports list

List imports

```text
kobana dunning imports list [OPTIONS]
```

`GET /v1/imports`

History of the organization’s 50 most recent imports (no pagination). File content and per-row errors are omitted; use GET /imports/\{id\} for details.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of the most recent imports. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## imports create

Create import

```text
kobana dunning imports create [OPTIONS]
```

`POST /v1/imports`

Uploads a CSV file of customers or charges and queues asynchronous processing. The body is JSON with the raw CSV text in the `content` field (max 2 MB). Invalid rows do not abort the batch: they are recorded in `errors` (`[{ line, error }]`) while the rest is processed. Track progress via GET /imports/\{id\}. Responds 202. Requires the `import` access level (or `all`) — `write` is not enough.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `resource_type` | `string` | yes | Imported resource type (`people` = customers \| `charges` = charges). Values: `people`, `charges`. |
| `content` | `string` | yes | Raw CSV content as plain text (UTF-8), including the header row. Max 2 MB. |
| `file_name` | `string` | no | Uploaded file name. |

More fields and nested objects: `kobana schema dunning.imports.create`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | Import accepted and queued for processing. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | File larger than 2 MB — split the file into smaller parts. |

## imports get

Get import

```text
kobana dunning imports get [ID] [OPTIONS]
```

`GET /v1/imports/{id}`

Returns an import’s status with progress counters and per-row rejection errors (`errors: [{ line, error }]`; line 1 is the header, data starts at line 2).

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Import found. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
