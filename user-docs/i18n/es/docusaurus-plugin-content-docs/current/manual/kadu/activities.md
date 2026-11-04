---
id: activities
title: "kobana kadu activities"
sidebar_label: "activities"
---

# kobana kadu activities


Comandos de `kobana kadu activities`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`activities list`](#activities-list) | `GET /v1/activities` | Lista actividades |
| [`activities create`](#activities-create) | `POST /v1/activities` | Crea activity |
| [`activities get`](#activities-get) | `GET /v1/activities/{id}` | Detalla la actividad |
| [`activities update`](#activities-update) | `PATCH /v1/activities/{id}` | Actualiza activity |
| [`activities delete`](#activities-delete) | `DELETE /v1/activities/{id}` | Elimina activities (soft delete) |

## activities list

Lista actividades

```text
kobana kadu activities list [OPCIONES]
```

`GET /v1/activities`

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

## activities create

Crea activity

```text
kobana kadu activities create [OPCIONES]
```

`POST /v1/activities`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `workspace_id` | `string (uuid)` | sí |  |
| `external_id` | `string` | no | Id del registro en el sistema de origen (texto libre). |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.activities.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | VALIDATION_ERROR — workspace fuera de la organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## activities get

Detalla la actividad

```text
kobana kadu activities get [ID] [OPCIONES]
```

`GET /v1/activities/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## activities update

Actualiza activity

```text
kobana kadu activities update [ID] [OPCIONES]
```

`PATCH /v1/activities/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.activities.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## activities delete

Elimina activities (soft delete)

```text
kobana kadu activities delete [ID] [OPCIONES]
```

`DELETE /v1/activities/{id}`

Exige el nivel `delete` (o `all`): `write` no elimina. Cascada: tasks, ejecuciones y memorias de la activity; cancela programaciones y desactiva el endpoint de las tasks.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
