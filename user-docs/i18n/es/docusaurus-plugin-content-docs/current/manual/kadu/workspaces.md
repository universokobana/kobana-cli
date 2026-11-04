---
id: workspaces
title: "kobana kadu workspaces"
sidebar_label: "workspaces"
---

# kobana kadu workspaces


Comandos de `kobana kadu workspaces`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | Lista workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Crea workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Detalla el workspace |
| [`workspaces update`](#workspaces-update) | `PATCH /v1/workspaces/{id}` | Actualiza workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Elimina workspaces (soft delete) |

## workspaces list

Lista workspaces

```text
kobana kadu workspaces list [OPCIONES]
```

`GET /v1/workspaces`

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

## workspaces create

Crea workspace

```text
kobana kadu workspaces create [OPCIONES]
```

`POST /v1/workspaces`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `external_id` | `string` | no | Id del registro en el sistema de origen (texto libre). |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.workspaces.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — registro duplicado / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## workspaces get

Detalla el workspace

```text
kobana kadu workspaces get [ID] [OPCIONES]
```

`GET /v1/workspaces/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## workspaces update

Actualiza workspace

```text
kobana kadu workspaces update [ID] [OPCIONES]
```

`PATCH /v1/workspaces/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.workspaces.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## workspaces delete

Elimina workspaces (soft delete)

```text
kobana kadu workspaces delete [ID] [OPCIONES]
```

`DELETE /v1/workspaces/{id}`

Exige el nivel `delete` (o `all`): `write` no elimina. Cascada: activities, tasks, ejecuciones, webhooks, memorias y secrets del workspace; cancela programaciones y desactiva el endpoint de las tasks.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
