---
id: volumes
title: "kobana kadu volumes"
sidebar_label: "volumes"
---

# kobana kadu volumes


Commands of `kobana kadu volumes`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`volumes list`](#volumes-list) | `GET /v1/volumes` | List volumes |
| [`volumes create`](#volumes-create) | `POST /v1/volumes` | Create volume |
| [`volumes get`](#volumes-get) | `GET /v1/volumes/{id}` | Get the volume |
| [`volumes update`](#volumes-update) | `PATCH /v1/volumes/{id}` | Update volume |
| [`volumes delete`](#volumes-delete) | `DELETE /v1/volumes/{id}` | Delete volume (soft delete) |
| [`volumes files`](#volumes-files) | `GET /v1/volumes/{id}/files` | List the files of the volume |
| [`volumes syncs`](#volumes-syncs) | `GET /v1/volumes/{id}/syncs` | List the syncs of the volume |

## volumes list

List volumes

```text
kobana kadu volumes list [OPTIONS]
```

`GET /v1/volumes`

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

## volumes create

Create volume

```text
kobana kadu volumes create [OPTIONS]
```

`POST /v1/volumes`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes |  |
| `environment_id` | `string (uuid)` | no |  |
| `label` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `quota_mb` | `integer` | no | Default: 1024. |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.volumes.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — a live volume with this name already exists in the environment / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — environment or workspace of another organization |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## volumes get

Get the volume

```text
kobana kadu volumes get [ID] [OPTIONS]
```

`GET /v1/volumes/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## volumes update

Update volume

```text
kobana kadu volumes update [ID] [OPTIONS]
```

`PATCH /v1/volumes/{id}`

Name and environment do not change: skills and the `kadu.yaml` reference the volume by them.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `label` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `quota_mb` | `integer` | no | Default: 1024. |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.volumes.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## volumes delete

Delete volume (soft delete)

```text
kobana kadu volumes delete [ID] [OPTIONS]
```

`DELETE /v1/volumes/{id}`

The content is kept for 30 days before the purge. Refused while the `kadu.yaml` declares the volume or a run is writing to it.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declared in the `kadu.yaml` / VOLUME_IN_USE — a run is writing to it |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## volumes files

List the files of the volume

```text
kobana kadu volumes files [ID] [OPTIONS]
```

`GET /v1/volumes/{id}/files`

In path order. Metadata only: the content is never returned.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `path_prefix` | `string` | no |  |
| `cursor` | `string` | no |  |
| `limit` | `number` | no | Default: 200. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — invalid cursor |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## volumes syncs

List the syncs of the volume

```text
kobana kadu volumes syncs [ID] [OPTIONS]
```

`GET /v1/volumes/{id}/syncs`

The runs that mounted the volume, newest first, with the outcome of each sync.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | `number` | no |  |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
