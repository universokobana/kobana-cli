---
id: repository
title: "kobana kadu repository"
sidebar_label: "repository"
description: "Organization Kadu Playbook"
---

# kobana kadu repository

Organization Kadu Playbook

Commands of `kobana kadu repository`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`repository`](#repository) | `GET /v1/repository` | Organization Kadu Playbook |
| [`repository sync`](#repository-sync) | `POST /v1/repository/sync` | Syncs the Kadu Playbook |

## repository

Organization Kadu Playbook

```text
kobana kadu repository [OPTIONS] [COMMAND]
```

`GET /v1/repository`

Connecting the repository means installing the GitHub App, done in the console (OAuth flow in the browser).

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## repository sync

Syncs the Kadu Playbook

```text
kobana kadu repository sync [OPTIONS]
```

`POST /v1/repository/sync`

Queues the sync (the worker clones the repository and updates the Task catalog). Follow `synced_at` and `sync_error` in `GET /repository`.

**Responses**

| HTTP | Description |
|---|---|
| `202` | accepted |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `422` | NO_PLUGIN_REPO / NO_DEPLOY_KEY |
| `503` | QUEUE_UNAVAILABLE / AS_UNAVAILABLE |
