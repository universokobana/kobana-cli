---
id: sources
title: "kobana kadu sources"
sidebar_label: "sources"
---

# kobana kadu sources


Commands of `kobana kadu sources`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`sources list`](#sources-list) | `GET /v1/sources` | List source repositories |
| [`sources create`](#sources-create) | `POST /v1/sources` | Register a source repository |
| [`sources get`](#sources-get) | `GET /v1/sources/{id}` | Get a source repository |
| [`sources update`](#sources-update) | `PATCH /v1/sources/{id}` | Update a source repository |
| [`sources delete`](#sources-delete) | `DELETE /v1/sources/{id}` | Delete a source repository (soft delete) |
| [`sources check`](#sources-check) | `POST /v1/sources/{id}/check` | Check access to the repository |

## sources list

List source repositories

```text
kobana kadu sources list [OPTIONS]
```

`GET /v1/sources`

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

## sources create

Register a source repository

```text
kobana kadu sources create [OPTIONS]
```

`POST /v1/sources`

If the organization's GitHub App installation covers the repository, access is checked right away. Otherwise the response carries the deploy key to add on GitHub before checking access.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes |  |
| `repository` | `string` | yes | `owner/repo` or the repository URL on GitHub. |
| `ref` | `string` | no | Branch or tag; null = the default branch. |
| `label` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.sources.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — a live repository with this name already exists / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — invalid repository, ref or workspace |
| `503` | GITHUB_UNAVAILABLE — GitHub did not respond |

## sources get

Get a source repository

```text
kobana kadu sources get [ID] [OPTIONS]
```

`GET /v1/sources/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## sources update

Update a source repository

```text
kobana kadu sources update [ID] [OPTIONS]
```

`PATCH /v1/sources/{id}`

Name and repository do not change: skills reference the name and the deploy key belongs to the repository. Changing the ref resets the access check.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ref` | `string` | no | Branch or tag; null = the default branch. |
| `label` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema kadu.sources.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — invalid ref or workspace |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## sources delete

Delete a source repository (soft delete)

```text
kobana kadu sources delete [ID] [OPTIONS]
```

`DELETE /v1/sources/{id}`

Tasks that declare it fail on their next run. Refused while the `kadu.yaml` declares the repository.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declared in the `kadu.yaml` |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## sources check

Check access to the repository

```text
kobana kadu sources check [ID] [OPTIONS]
```

`POST /v1/sources/{id}/check`

Lists the ref on GitHub with the repository's credential, without cloning, and records the result. Missing access is not an HTTP error: it comes in `access_status` and `last_check_error`.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
