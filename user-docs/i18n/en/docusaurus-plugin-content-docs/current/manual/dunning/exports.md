---
id: exports
title: "kobana dunning exports"
sidebar_label: "exports"
---

# kobana dunning exports


Commands of `kobana dunning exports`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`exports charges`](#exports-charges) | `GET /v1/exports/charges` | Export charges (synchronous CSV) |
| [`exports jobs list`](#exports-jobs-list) | `GET /v1/exports/jobs` | List asynchronous exports |
| [`exports jobs create`](#exports-jobs-create) | `POST /v1/exports/jobs` | Create asynchronous export |
| [`exports jobs download`](#exports-jobs-download) | `GET /v1/exports/jobs/{id}/download` | Download export CSV |
| [`exports people`](#exports-people) | `GET /v1/exports/people` | Export customers (synchronous CSV) |

## exports charges

Export charges (synchronous CSV)

```text
kobana dunning exports charges [OPTIONS]
```

`GET /v1/exports/charges`

Generates and returns the charges CSV inline (`text/csv; charset=utf-8` with BOM, `Content-Disposition: attachment`), with optional status and due-date filters. Columns: cliente, documento_cliente, numero_documento, descricao, valor_original, valor_atual, vencimento, dias_atraso, status, origem, external_id. Capped at 10k rows; beyond that use the asynchronous export (POST /exports/jobs). Requires the `export` access level (or `all`) — `read` and `write` are not enough.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Filter charges by status (e.g. `pending`, `overdue`, `paid`). |
| `from` | `string` | no | Due date from (ISO date `YYYY-MM-DD`). |
| `to` | `string` | no | Due date until (ISO date `YYYY-MM-DD`). |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | CSV file (`text/csv; charset=utf-8` with BOM so Excel renders accents correctly; `Content-Disposition: attachment`). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## exports jobs list

List asynchronous exports

```text
kobana dunning exports jobs list [OPTIONS]
```

`GET /v1/exports/jobs`

History of the organization’s 50 most recent asynchronous exports (no pagination). Use this route to track job status; the CSV itself is not included in the listing. A read (`read`): the file itself requires `export`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of the most recent exports. |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## exports jobs create

Create asynchronous export

```text
kobana dunning exports jobs create [OPTIONS]
```

`POST /v1/exports/jobs`

Schedules CSV generation on a worker, without the 10k-row cap of the synchronous mode and without blocking the request. Track status via GET /exports/jobs and download the file via GET /exports/jobs/\{id\}/download before `expires_at`. Responds 202. Requires the `export` access level (or `all`) — `read` and `write` are not enough.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `resource_type` | `string` | yes | Exported resource type (`people` = customers \| `charges` = charges). Values: `people`, `charges`. |
| `file_name` | `string` | no | Generated file name (default: `{resourceType}.csv`). |
| `filters` | `object` | no | Filters applied to the export (only for `charges`); stored on the job for auditing. |

More fields and nested objects: `kobana schema dunning.exports.jobs.create`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | Export accepted and queued for processing. |
| `400` | Invalid request (validation error). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |

## exports jobs download

Download export CSV

```text
kobana dunning exports jobs download [ID] [OPTIONS]
```

`GET /v1/exports/jobs/{id}/download`

Serves the generated CSV of an asynchronous export (`text/csv; charset=utf-8` with BOM). Only available once the job is `completed` and before `expires_at`. Requires the `export` access level (or `all`) — `read` and `write` are not enough.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | CSV file (`text/csv; charset=utf-8` with BOM so Excel renders accents correctly; `Content-Disposition: attachment`). |
| `404` | Resource not found or outside your organization. |
| `409` | Export not finished yet (`NOT_READY` code; the `status` field reports the current state). |
| `410` | Export expired — create a new one (`EXPIRED` code). |
| `413` | Request body larger than 2 MB. |

## exports people

Export customers (synchronous CSV)

```text
kobana dunning exports people [OPTIONS]
```

`GET /v1/exports/people`

Generates and returns the customers CSV inline (`text/csv; charset=utf-8` with BOM, `Content-Disposition: attachment`). Columns: nome, documento, tipo_documento, email, telefone, classificacao, external_id, tags, criado_em. Capped at 10k rows; beyond that use the asynchronous export (POST /exports/jobs). Requires the `export` access level (or `all`) — `read` and `write` are not enough.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace the call acts on (`id` or `public_id`, listed by `GET /v1/workspaces`). Optional while the organization has a single workspace; with two or more, omitting it is `400 WORKSPACE_REQUIRED`. A workspace of another organization is `403 UNKNOWN_WORKSPACE`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | CSV file (`text/csv; charset=utf-8` with BOM so Excel renders accents correctly; `Content-Disposition: attachment`). |
| `404` | Resource not found or outside your organization. |
| `413` | Request body larger than 2 MB. |
