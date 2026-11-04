---
id: memories
title: "kobana kadu memories"
sidebar_label: "memories"
---

# kobana kadu memories


Comandos de `kobana kadu memories`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`memories list`](#memories-list) | `GET /v1/memories` | Lista memorias |
| [`memories create`](#memories-create) | `POST /v1/memories` | Crea memoria |
| [`memories consolidate`](#memories-consolidate) | `POST /v1/memories/consolidate` | Consolida las memorias de un período |
| [`memories get`](#memories-get) | `GET /v1/memories/{id}` | Detalla la memoria |
| [`memories update`](#memories-update) | `PATCH /v1/memories/{id}` | Actualiza memoria |
| [`memories delete`](#memories-delete) | `DELETE /v1/memories/{id}` | Elimina memoria (soft delete) |
| [`memories rollback`](#memories-rollback) | `POST /v1/memories/{id}/rollback` | Vuelve la memoria a una versión |
| [`memories versions`](#memories-versions) | `GET /v1/memories/{id}/versions` | Lista las versiones de la memoria |

## memories list

Lista memorias

```text
kobana kadu memories list [OPCIONES]
```

`GET /v1/memories`

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

## memories create

Crea memoria

```text
kobana kadu memories create [OPCIONES]
```

`POST /v1/memories`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `content` | `string` | sí |  |
| `workspace_id` | `string (uuid)` | no |  |
| `activity_id` | `string (uuid)` | no |  |
| `entity_type` | `string` | no |  |
| `entity_value` | `string` | no |  |
| `key` | `string` | no |  |
| `summary` | `string` | no |  |
| `status` | `string` | no |  |
| `external_id` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.memories.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT (external_id duplicado en la organización) / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — workspace/activity fuera de la organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## memories consolidate

Consolida las memorias de un período

```text
kobana kadu memories consolidate [OPCIONES]
```

`POST /v1/memories/consolidate`

Lee las ejecuciones terminadas (`success`/`failed`) del período que aún no se consolidaron — incluso de Task con memorias desactivadas — y guarda los hechos duraderos como memorias, en el alcance de cada Task. Cada ejecución es una llamada de modelo cobrada a la organización. `dry_run: true` devuelve solo la vista previa. Asíncrono: las memorias aparecen a medida que corren los jobs.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `from` | `string` | sí | Primer día (YYYY-MM-DD, UTC), inclusive. |
| `to` | `string` | sí | Último día (YYYY-MM-DD, UTC), inclusive. Hasta 366 días. |
| `task_id` | `string (uuid)` | no | Solo las ejecuciones de esta Task. |
| `dry_run` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.memories.consolidate`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | vista previa (dry_run) |
| `202` | en cola |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | VALIDATION_ERROR — período o Task inválidos |
| `503` | SERVICE_UNAVAILABLE — cola no disponible |

## memories get

Detalla la memoria

```text
kobana kadu memories get [ID] [OPCIONES]
```

`GET /v1/memories/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## memories update

Actualiza memoria

```text
kobana kadu memories update [ID] [OPCIONES]
```

`PATCH /v1/memories/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `content` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `activity_id` | `string (uuid)` | no |  |
| `entity_type` | `string` | no |  |
| `entity_value` | `string` | no |  |
| `key` | `string` | no |  |
| `summary` | `string` | no |  |
| `status` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.memories.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | VALIDATION_ERROR — workspace/activity fuera de la organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## memories delete

Elimina memoria (soft delete)

```text
kobana kadu memories delete [ID] [OPCIONES]
```

`DELETE /v1/memories/{id}`

Exige el nivel `delete` (o `all`): `write` no elimina.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## memories rollback

Vuelve la memoria a una versión

```text
kobana kadu memories rollback [ID] [OPCIONES]
```

`POST /v1/memories/{id}/rollback`

Guarda una versión NUEVA con el contenido de la elegida, que pasa a ser la vigente; el historial queda completo (`rollback_from`).

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `version` | `integer` | sí |  |

Otros campos y objetos anidados: `kobana schema kadu.memories.rollback`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND (memoria o versión) |
| `409` | CONFLICT — ya es la versión vigente / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — el workspace/activity de la versión ya no es válido |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## memories versions

Lista las versiones de la memoria

```text
kobana kadu memories versions [ID] [OPCIONES]
```

`GET /v1/memories/{id}/versions`

Más nueva primero. Todo cambio de contenido (por la API, el agente, la consolidación o la consola) guarda una versión; `current: true` marca la vigente.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
