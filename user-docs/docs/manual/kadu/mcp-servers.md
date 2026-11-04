---
id: mcp-servers
title: "kobana kadu mcp-servers"
sidebar_label: "mcp-servers"
---

# kobana kadu mcp-servers


Comandos de `kobana kadu mcp-servers`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`mcp-servers list`](#mcp-servers-list) | `GET /v1/mcp_servers` | Lista servidores MCP |
| [`mcp-servers create`](#mcp-servers-create) | `POST /v1/mcp_servers` | Cadastra servidor MCP |
| [`mcp-servers get`](#mcp-servers-get) | `GET /v1/mcp_servers/{id}` | Detalha o servidor MCP |
| [`mcp-servers update`](#mcp-servers-update) | `PATCH /v1/mcp_servers/{id}` | Atualiza servidor MCP |
| [`mcp-servers delete`](#mcp-servers-delete) | `DELETE /v1/mcp_servers/{id}` | Remove servidor MCP (soft delete) |

## mcp-servers list

Lista servidores MCP

```text
kobana kadu mcp-servers list [OPÇÕES]
```

`GET /v1/mcp_servers`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cursor` | `string` | não |  |
| `limit` | `number` | não |  |
| `sort_by` | `string` | não | Campo em snake_case: created_at (padrão), updated_at ou um campo filtrável. |
| `sort_order` | `string` | não | Padrão: desc. Valores: `asc`, `desc`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## mcp-servers create

Cadastra servidor MCP

```text
kobana kadu mcp-servers create [OPÇÕES]
```

`POST /v1/mcp_servers`

A tarefa recebe o servidor quando o cita pelo nome em `task.mcp_servers` (depois do `.mcp.json` do plugin e antes do catálogo da Kobana). Escopo: `kadu/environments` (configuração da execução). A `message` das recusas sai no idioma do `Accept-Language` (pt-BR, en-US ou es-ES); `details.reason` é o motivo como código estável, com os parâmetros dele.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim |  |
| `config` | `object` | sim | Uma entrada de `mcpServers` no formato padrão do `.mcp.json`: remoto `{ "type": "http" \| "sse", "url": "https://…", "headers": { … } }` ou stdio `{ "command": "npx", "args": [ … ], "env": { … } }`. Credencial só como `${VAR}` (secret do ambiente, na `required_secrets` da tarefa): header ou env com nome de credencial e valor literal é recusado. |
| `description` | `string` | não |  |
| `enabled` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.mcp-servers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — já existe servidor vivo com este nome / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CONFIG — `config` fora do formato `mcpServers` ou com credencial literal |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## mcp-servers get

Detalha o servidor MCP

```text
kobana kadu mcp-servers get [ID] [OPÇÕES]
```

`GET /v1/mcp_servers/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## mcp-servers update

Atualiza servidor MCP

```text
kobana kadu mcp-servers update [ID] [OPÇÕES]
```

`PATCH /v1/mcp_servers/{id}`

`config` enviada substitui a entrada inteira.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `description` | `string` | não |  |
| `config` | `object` | não | Uma entrada de `mcpServers` no formato padrão do `.mcp.json`: remoto `{ "type": "http" \| "sse", "url": "https://…", "headers": { … } }` ou stdio `{ "command": "npx", "args": [ … ], "env": { … } }`. Credencial só como `${VAR}` (secret do ambiente, na `required_secrets` da tarefa): header ou env com nome de credencial e valor literal é recusado. |
| `enabled` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.mcp-servers.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — nome já usado / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CONFIG — `config` fora do formato `mcpServers` ou com credencial literal |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## mcp-servers delete

Remove servidor MCP (soft delete)

```text
kobana kadu mcp-servers delete [ID] [OPÇÕES]
```

`DELETE /v1/mcp_servers/{id}`

A tarefa que ainda o cita em `mcp_servers` passa a recebê-lo como não encontrado.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
