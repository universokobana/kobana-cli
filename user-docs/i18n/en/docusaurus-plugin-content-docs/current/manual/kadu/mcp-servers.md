---
id: mcp-servers
title: "kobana kadu mcp-servers"
sidebar_label: "mcp-servers"
---

# kobana kadu mcp-servers


Commands of `kobana kadu mcp-servers`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`mcp-servers list`](#mcp-servers-list) | `GET /v1/mcp_servers` | List MCP servers |
| [`mcp-servers create`](#mcp-servers-create) | `POST /v1/mcp_servers` | Register an MCP server |
| [`mcp-servers get`](#mcp-servers-get) | `GET /v1/mcp_servers/{id}` | Get the MCP server |
| [`mcp-servers update`](#mcp-servers-update) | `PATCH /v1/mcp_servers/{id}` | Update an MCP server |
| [`mcp-servers delete`](#mcp-servers-delete) | `DELETE /v1/mcp_servers/{id}` | Remove an MCP server (soft delete) |

## mcp-servers list

List MCP servers

```text
kobana kadu mcp-servers list [OPTIONS]
```

`GET /v1/mcp_servers`

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

## mcp-servers create

Register an MCP server

```text
kobana kadu mcp-servers create [OPTIONS]
```

`POST /v1/mcp_servers`

A task gets the server when it names it in `task.mcp_servers` (after the plugin `.mcp.json` and before the Kobana catalog). Scope: `kadu/environments` (run configuration). The `message` of a refusal follows `Accept-Language` (pt-BR, en-US or es-ES); `details.reason` is the reason as a stable code, with its parameters.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes |  |
| `config` | `object` | yes | One `mcpServers` entry in the standard `.mcp.json` format: remote `{ "type": "http" \| "sse", "url": "https://…", "headers": { … } }` or stdio `{ "command": "npx", "args": [ … ], "env": { … } }`. Credentials only as `${VAR}` (an environment secret in the task `required_secrets`): a header or env with a credential name and a literal value is refused. |
| `description` | `string` | no |  |
| `enabled` | `boolean` | no |  |

More fields and nested objects: `kobana schema kadu.mcp-servers.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — a live server already has this name / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CONFIG — `config` outside the `mcpServers` format or with a literal credential |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## mcp-servers get

Get the MCP server

```text
kobana kadu mcp-servers get [ID] [OPTIONS]
```

`GET /v1/mcp_servers/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## mcp-servers update

Update an MCP server

```text
kobana kadu mcp-servers update [ID] [OPTIONS]
```

`PATCH /v1/mcp_servers/{id}`

A `config` sent replaces the whole entry.

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no |  |
| `description` | `string` | no |  |
| `config` | `object` | no | One `mcpServers` entry in the standard `.mcp.json` format: remote `{ "type": "http" \| "sse", "url": "https://…", "headers": { … } }` or stdio `{ "command": "npx", "args": [ … ], "env": { … } }`. Credentials only as `${VAR}` (an environment secret in the task `required_secrets`): a header or env with a credential name and a literal value is refused. |
| `enabled` | `boolean` | no |  |

More fields and nested objects: `kobana schema kadu.mcp-servers.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — name already used / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CONFIG — `config` outside the `mcpServers` format or with a literal credential |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## mcp-servers delete

Remove an MCP server (soft delete)

```text
kobana kadu mcp-servers delete [ID] [OPTIONS]
```

`DELETE /v1/mcp_servers/{id}`

A task that still names it in `mcp_servers` gets it as not found.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
