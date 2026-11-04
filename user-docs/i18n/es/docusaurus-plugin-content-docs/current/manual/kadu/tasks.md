---
id: tasks
title: "kobana kadu tasks"
sidebar_label: "tasks"
---

# kobana kadu tasks


Comandos de `kobana kadu tasks`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`tasks endpoint`](#tasks-endpoint) | `GET /v1/tasks/{id}/endpoint` | Endpoint HTTP de lanzamiento de la Task |
| [`tasks endpoint update`](#tasks-endpoint-update) | `PUT /v1/tasks/{id}/endpoint` | Configura el endpoint HTTP de la Task |
| [`tasks endpoint delete`](#tasks-endpoint-delete) | `DELETE /v1/tasks/{id}/endpoint` | Deshabilita el endpoint HTTP de la Task |
| [`tasks list`](#tasks-list) | `GET /v1/tasks` | Lista tareas |
| [`tasks create`](#tasks-create) | `POST /v1/tasks` | Crea task |
| [`tasks get`](#tasks-get) | `GET /v1/tasks/{id}` | Detalla la tarea |
| [`tasks update`](#tasks-update) | `PATCH /v1/tasks/{id}` | Actualiza task |
| [`tasks delete`](#tasks-delete) | `DELETE /v1/tasks/{id}` | Elimina tasks (soft delete) |
| [`tasks readiness`](#tasks-readiness) | `GET /v1/tasks/{id}/readiness` | Verificación previa del lanzamiento |
| [`tasks run`](#tasks-run) | `POST /v1/tasks/{id}/run` | Lanza ejecución |

## tasks endpoint

Endpoint HTTP de lanzamiento de la Task

```text
kobana kadu tasks endpoint [ID] [OPCIONES] [COMANDO]
```

`GET /v1/tasks/{id}/endpoint`

Configuración del endpoint público `POST /api/run-task/{task_id}`. Las credenciales nunca se devuelven: solo los flags `*_set`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## tasks endpoint update

Configura el endpoint HTTP de la Task

```text
kobana kadu tasks endpoint update [ID] [OPCIONES]
```

`PUT /v1/tasks/{id}/endpoint`

Crea o actualiza. Una credencial omitida mantiene la guardada mientras el tipo de auth no cambie. Evita `auth: params`: el secreto va en la URL y queda en los logs de acceso de proxies y CDNs; prefiere `bearer` o `header`.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `enabled` | `boolean` | sí |  |
| `auth` | `string` | sí | Valores: `basic`, `bearer`, `header`, `params`. |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |
| `auth_param_key` | `string` | no |  |
| `auth_param_value` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.tasks.endpoint.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | INCOMPLETE_AUTH — falta la credencial para el tipo de auth elegido |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## tasks endpoint delete

Deshabilita el endpoint HTTP de la Task

```text
kobana kadu tasks endpoint delete [ID] [OPCIONES]
```

`DELETE /v1/tasks/{id}/endpoint`

Mantiene la configuración; un PUT con `enabled: true` reactiva.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## tasks list

Lista tareas

```text
kobana kadu tasks list [OPCIONES]
```

`GET /v1/tasks`

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

## tasks create

Crea task

```text
kobana kadu tasks create [OPCIONES]
```

`POST /v1/tasks`

La task nace en el tier de precio predeterminado; el tier y el modelo de IA los define Kobana (`model` en el cuerpo se rechaza).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `workspace_id` | `string (uuid)` | sí |  |
| `activity_id` | `string (uuid)` | sí |  |
| `skill_slug` | `string` | sí | `<plugin>:<skill>` del repositorio de la organización: plugin del workspace (cuando tiene uno), skill presente en el catálogo sincronizado (cuando hay catálogo), nunca `kadu:*`. |
| `parameters_schema` | `object` | no |  |
| `parameters_sensitive` | `boolean` | no |  |
| `external_id` | `string` | no | Id del registro en el sistema de origen (texto libre). |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.tasks.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | VALIDATION_ERROR — workspace/activity fuera de la organización o `skill_slug` no permitido |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## tasks get

Detalla la tarea

```text
kobana kadu tasks get [ID] [OPCIONES]
```

`GET /v1/tasks/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## tasks update

Actualiza task

```text
kobana kadu tasks update [ID] [OPCIONES]
```

`PATCH /v1/tasks/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no |  |
| `skill_slug` | `string` | no | `<plugin>:<skill>` del repositorio de la organización: plugin del workspace (cuando tiene uno), skill presente en el catálogo sincronizado (cuando hay catálogo), nunca `kadu:*`. |
| `parameters_schema` | `object` | no |  |
| `parameters_sensitive` | `boolean` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.tasks.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | VALIDATION_ERROR — `skill_slug` no permitido |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## tasks delete

Elimina tasks (soft delete)

```text
kobana kadu tasks delete [ID] [OPCIONES]
```

`DELETE /v1/tasks/{id}`

Exige el nivel `delete` (o `all`): `write` no elimina. Cascada: ejecuciones y archivos de la task; cancela programaciones y desactiva el endpoint.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## tasks readiness

Verificación previa del lanzamiento

```text
kobana kadu tasks readiness [ID] [OPCIONES]
```

`GET /v1/tasks/{id}/readiness`

Si la Task puede correr en el entorno: entorno válido y valor para todos los secrets exigidos (en el workspace de la Task o globales). Trae también la cola de la Task. El lanzamiento revalida todo.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `environment_id` | `string` | no |  |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## tasks run

Lanza ejecución

```text
kobana kadu tasks run [ID] [OPCIONES]
```

`POST /v1/tasks/{id}/run`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `reference_date` | `string` | sí |  |
| `environment_id` | `string (uuid)` | sí |  |
| `parameters` | `object` | no |  |
| `locale` | `string` | no |  |
| `additional_instructions` | `string` | no |  |
| `input_file_ids` | `array<string (uuid)>` | no | Archivos enviados por `POST /files` (confirmados y aún no usados). |
| `mode` | `string` | no | `resolve_pendencies` solo resuelve los pendientes de las ejecuciones anteriores de la Task (consola: "Resolver pendientes"); `parameters` se ignora. Valores: `default`, `resolve_pendencies`. |

Otros campos y objetos anidados: `kobana schema kadu.tasks.run`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | COST_LIMIT_REACHED / INVALID_PARAMETERS / ENVIRONMENT_REQUIRED / MISSING_SECRETS (details.keys) / INVALID_INPUT_FILES |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
