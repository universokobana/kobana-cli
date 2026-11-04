---
id: operation-approvals
title: "kobana kadu operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana kadu operation-approvals


Comandos de `kobana kadu operation-approvals`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation_approvals` | Aprobaciones por enlace de la persona del token |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation_approvals/{id}` | Detalla una aprobación por enlace |

## operation-approvals list

Aprobaciones por enlace de la persona del token

```text
kobana kadu operation-approvals list [OPCIONES]
```

`GET /v1/operation_approvals`

Operaciones que un asistente solicitó por MCP y que solo se ejecutan después de que la persona las confirma por enlace en la consola. Lista solo las aprobaciones de la persona del token, en la organización del token, las más recientes primero. Solo lectura: Continuar y Cancelar ocurren en la consola. Alcance: `kadu/system_events`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Valores: `pending`, `approved`, `executed`, `declined`, `failed`, `expired`. |
| `resource` | `string` | no |  |
| `created_at[gte]` | `string` | no | ISO 8601 (fecha o fecha y hora). |
| `created_at[lte]` | `string` | no | ISO 8601 (fecha o fecha y hora). |
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — filtro o cursor inválido |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## operation-approvals get

Detalla una aprobación por enlace

```text
kobana kadu operation-approvals get [ID] [OPCIONES]
```

`GET /v1/operation_approvals/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
