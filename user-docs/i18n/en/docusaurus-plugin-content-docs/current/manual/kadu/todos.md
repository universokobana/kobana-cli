---
id: todos
title: "kobana kadu todos"
sidebar_label: "todos"
---

# kobana kadu todos


Commands of `kobana kadu todos`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`todos update`](#todos-update) | `PATCH /v1/todos/{id}` | Resolve pending item |

## todos update

Resolve pending item

```text
kobana kadu todos update [ID] [OPTIONS]
```

`PATCH /v1/todos/{id}`

The only transition through the API: `status: resolved`.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | `string` | no | Values: `resolved`. |

More fields and nested objects: `kobana schema kadu.todos.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (or the route's CONFLICT) |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
