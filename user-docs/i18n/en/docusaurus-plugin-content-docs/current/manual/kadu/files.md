---
id: files
title: "kobana kadu files"
sidebar_label: "files"
---

# kobana kadu files


Commands of `kobana kadu files`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`files list`](#files-list) | `GET /v1/files` | List files |
| [`files create`](#files-create) | `POST /v1/files` | Start an input file upload |
| [`files get`](#files-get) | `GET /v1/files/{id}` | Get file details |
| [`files confirm`](#files-confirm) | `POST /v1/files/{id}/confirm` | Confirm the upload (pending → uploaded) |
| [`files download`](#files-download) | `GET /v1/files/{id}/download` | Temporary download URL |

## files list

List files

```text
kobana kadu files list [OPTIONS]
```

`GET /v1/files`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Field in snake_case: created_at (default), updated_at or a filterable field. |
| `sort_order` | `string` | no | Default: desc. Values: `asc`, `desc`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## files create

Start an input file upload

```text
kobana kadu files create [OPTIONS]
```

`POST /v1/files`

Three-step flow: (1) this POST creates the `pending` file and returns `upload_url`; (2) `PUT` the bytes to `upload_url` with the same `Content-Type`; (3) `POST /files/{id}/confirm`. The id goes in `input_file_ids` of `POST /tasks/{id}/run` (the Task must accept input files). Maximum size: 100 MB.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `task_id` | `string (uuid)` | yes |  |
| `filename` | `string` | yes |  |
| `size_bytes` | `integer` | yes |  |
| `content_type` | `string` | no |  |

More fields and nested objects: `kobana schema kadu.files.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | TASK_DOES_NOT_ACCEPT_FILES / VALIDATION_ERROR — Task outside the organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## files get

Get file details

```text
kobana kadu files get [ID] [OPTIONS]
```

`GET /v1/files/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## files confirm

Confirm the upload (pending → uploaded)

```text
kobana kadu files confirm [ID] [OPTIONS]
```

`POST /v1/files/{id}/confirm`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — not a pending input file / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## files download

Temporary download URL

```text
kobana kadu files download [ID] [OPTIONS]
```

`GET /v1/files/{id}/download`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | FILE_NOT_UPLOADED |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
