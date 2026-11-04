---
id: mcp-servers
title: "kobana kadu mcp-servers"
sidebar_label: "mcp-servers"
---

# kobana kadu mcp-servers


Comandos de `kobana kadu mcp-servers`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`mcp-servers list`](#mcp-servers-list) | `GET /v1/mcp_servers` | Lista servidores MCP |
| [`mcp-servers create`](#mcp-servers-create) | `POST /v1/mcp_servers` | Registra un servidor MCP |
| [`mcp-servers get`](#mcp-servers-get) | `GET /v1/mcp_servers/{id}` | Detalla el servidor MCP |
| [`mcp-servers update`](#mcp-servers-update) | `PATCH /v1/mcp_servers/{id}` | Actualiza un servidor MCP |
| [`mcp-servers delete`](#mcp-servers-delete) | `DELETE /v1/mcp_servers/{id}` | Elimina un servidor MCP (soft delete) |

## mcp-servers list

Lista servidores MCP

```text
kobana kadu mcp-servers list [OPCIONES]
```

`GET /v1/mcp_servers`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Campo en snake_case: created_at (predeterminado), updated_at o un campo filtrable. |
| `sort_order` | `string` | no | Predeterminado: desc. Valores: `asc`, `desc`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## mcp-servers create

Registra un servidor MCP

```text
kobana kadu mcp-servers create [OPCIONES]
```

`POST /v1/mcp_servers`

La tarea recibe el servidor cuando lo cita por nombre en `task.mcp_servers` (después del `.mcp.json` del plugin y antes del catálogo de Kobana). Alcance: `kadu/environments` (configuración de la ejecución). El `message` de los rechazos sale en el idioma del `Accept-Language` (pt-BR, en-US o es-ES); `details.reason` es el motivo como código estable, con sus parámetros.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí |  |
| `config` | `object` | sí | Una entrada de `mcpServers` en el formato estándar del `.mcp.json`: remoto `{ "type": "http" \| "sse", "url": "https://…", "headers": { … } }` o stdio `{ "command": "npx", "args": [ … ], "env": { … } }`. Credencial solo como `${VAR}` (secret del entorno, en la `required_secrets` de la tarea): un header o env con nombre de credencial y valor literal se rechaza. |
| `description` | `string` | no |  |
| `enabled` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.mcp-servers.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — ya existe un servidor vivo con este nombre / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CONFIG — `config` fuera del formato `mcpServers` o con credencial literal |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## mcp-servers get

Detalla el servidor MCP

```text
kobana kadu mcp-servers get [ID] [OPCIONES]
```

`GET /v1/mcp_servers/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## mcp-servers update

Actualiza un servidor MCP

```text
kobana kadu mcp-servers update [ID] [OPCIONES]
```

`PATCH /v1/mcp_servers/{id}`

La `config` enviada reemplaza la entrada entera.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no |  |
| `description` | `string` | no |  |
| `config` | `object` | no | Una entrada de `mcpServers` en el formato estándar del `.mcp.json`: remoto `{ "type": "http" \| "sse", "url": "https://…", "headers": { … } }` o stdio `{ "command": "npx", "args": [ … ], "env": { … } }`. Credencial solo como `${VAR}` (secret del entorno, en la `required_secrets` de la tarea): un header o env con nombre de credencial y valor literal se rechaza. |
| `enabled` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.mcp-servers.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — nombre ya usado / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CONFIG — `config` fuera del formato `mcpServers` o con credencial literal |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## mcp-servers delete

Elimina un servidor MCP (soft delete)

```text
kobana kadu mcp-servers delete [ID] [OPCIONES]
```

`DELETE /v1/mcp_servers/{id}`

La tarea que aún lo cita en `mcp_servers` pasa a recibirlo como no encontrado.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
