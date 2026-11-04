---
id: schedules
title: "kobana kadu schedules"
sidebar_label: "schedules"
---

# kobana kadu schedules


Comandos de `kobana kadu schedules`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`schedules list`](#schedules-list) | `GET /v1/schedules` | Lista programaciones |
| [`schedules create`](#schedules-create) | `POST /v1/schedules` | Programa una Task en un entorno |
| [`schedules get`](#schedules-get) | `GET /v1/schedules/{id}` | Detalla la programación |
| [`schedules update`](#schedules-update) | `PATCH /v1/schedules/{id}` | Modifica, pausa o reanuda una programación |
| [`schedules delete`](#schedules-delete) | `DELETE /v1/schedules/{id}` | Cancela la programación (soft delete) |

## schedules list

Lista programaciones

```text
kobana kadu schedules list [OPCIONES]
```

`GET /v1/schedules`

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

## schedules create

Programa una Task en un entorno

```text
kobana kadu schedules create [OPCIONES]
```

`POST /v1/schedules`

Una programación por Task y entorno. La programación no se apila: el lanzamiento se omite si la Task ya tiene una ejecución activa o en cola.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `task_id` | `string (uuid)` | sí |  |
| `environment_id` | `string (uuid)` | sí |  |
| `cron` | `string` | sí |  |
| `timezone` | `string` | no |  |
| `paused` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.schedules.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `409` | CONFLICT — la Task ya tiene una programación en el entorno (`details.schedule_id`) / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CRON / VALIDATION_ERROR — Task o entorno fuera de la organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## schedules get

Detalla la programación

```text
kobana kadu schedules get [ID] [OPCIONES]
```

`GET /v1/schedules/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## schedules update

Modifica, pausa o reanuda una programación

```text
kobana kadu schedules update [ID] [OPCIONES]
```

`PATCH /v1/schedules/{id}`

`paused: true` mantiene el cron y hace que el dispatcher omita la programación.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cron` | `string` | no |  |
| `timezone` | `string` | no |  |
| `paused` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.schedules.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | INVALID_CRON |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## schedules delete

Cancela la programación (soft delete)

```text
kobana kadu schedules delete [ID] [OPCIONES]
```

`DELETE /v1/schedules/{id}`

Un nuevo POST para la misma Task y entorno reactiva la programación.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declarado en el `schedules` del frontmatter de la skill: quítalo de allí |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
