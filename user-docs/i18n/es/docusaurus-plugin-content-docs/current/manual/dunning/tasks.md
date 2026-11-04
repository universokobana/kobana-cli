---
id: tasks
title: "kobana dunning tasks"
sidebar_label: "tasks"
---

# kobana dunning tasks


Comandos de `kobana dunning tasks`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`tasks list`](#tasks-list) | `GET /v1/tasks` | Listar tareas |
| [`tasks create`](#tasks-create) | `POST /v1/tasks` | Crear tarea |
| [`tasks bulk-assign`](#tasks-bulk-assign) | `POST /v1/tasks/bulk-assign` | Asignar responsable en masa |
| [`tasks bulk-complete`](#tasks-bulk-complete) | `POST /v1/tasks/bulk-complete` | Completar tareas en masa |
| [`tasks get`](#tasks-get) | `GET /v1/tasks/{id}` | Consultar tarea |
| [`tasks update`](#tasks-update) | `PUT /v1/tasks/{id}` | Actualizar tarea |
| [`tasks complete`](#tasks-complete) | `POST /v1/tasks/{id}/complete` | Completar tarea |

## tasks list

Listar tareas

```text
kobana dunning tasks list [OPCIONES]
```

`GET /v1/tasks`

Lista las tareas de la organización con paginación y filtros. Ordenable vía `sort_by` (`title` | `priority` | `status` | `due_at` | `created_at` | `updated_at`).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `sort_by` | `string` | no | Campo de ordenación (nombre público en snake_case, ej.: `due_date`). |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra por estado (`pending` \| `in_progress` \| `completed` \| `cancelled`). Valores: `pending`, `in_progress`, `completed`, `cancelled`. |
| `priority` | `string` | no | Filtra por prioridad (`low` \| `medium` \| `high` \| `urgent`). Valores: `low`, `medium`, `high`, `urgent`. |
| `task_type` | `string` | no | Filtra por tipo (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Valores: `call`, `email`, `visit`, `review`, `follow_up`. |
| `assigned_to_id` | `string` | no | Filtra por responsable (UUID). |
| `person_id` | `string` | no | Filtra por cliente (UUID). |
| `charge_id` | `string` | no | Filtra por cobro (UUID). |
| `include` | `string` | no | Relaciones a incluir, separadas por comas (`person`, `assigned_to`, `charge`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de tareas. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## tasks create

Crear tarea

```text
kobana dunning tasks create [OPCIONES]
```

`POST /v1/tasks`

Crea una tarea, opcionalmente vinculada a un cliente, un cobro y un responsable (todos de la propia organización). Admite `Idempotency-Key`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `title` | `string` | sí | Título de la tarea. |
| `task_type` | `string` | sí | Tipo (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Valores: `call`, `email`, `visit`, `review`, `follow_up`. |
| `person_id` | `string (uuid)` | no | Cliente vinculado a la tarea. |
| `charge_id` | `string (uuid)` | no | Cobro vinculado a la tarea. |
| `assigned_to_id` | `string (uuid)` | no | Usuario responsable de la tarea. |
| `description` | `string` | no | Descripción detallada. |
| `priority` | `string` | no | Prioridad inicial (`low` \| `medium` \| `high` \| `urgent`; por defecto `medium`). Valores: `low`, `medium`, `high`, `urgent`. |
| `status` | `string` | no | Estado inicial (por defecto `pending`). Valores: `pending`, `in_progress`, `completed`, `cancelled`. |
| `due_at` | `string (date-time)` | no | Plazo de la tarea (ISO 8601). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.tasks.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Tarea creada (incluye los resúmenes de cliente, responsable y cobro). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## tasks bulk-assign

Asignar responsable en masa

```text
kobana dunning tasks bulk-assign [OPCIONES]
```

`POST /v1/tasks/bulk-assign`

Asigna (o quita, con `assignedToId: null`) el responsable de hasta 100 tareas. El usuario objetivo debe pertenecer a la organización. Una tarea cancelada vuelve en `failed` con `invalid_status`. Un resultado parcial no es error.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | IDs de los registros objetivo (1 a 100 por llamada). Los duplicados se ignoran. |
| `assigned_to_id` | `string (uuid)` | sí | ID del usuario responsable, o `null` para quitar la asignación. |

Otros campos y objetos anidados: `kobana schema dunning.tasks.bulk-assign`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado por ítem de la asignación masiva. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## tasks bulk-complete

Completar tareas en masa

```text
kobana dunning tasks bulk-complete [OPCIONES]
```

`POST /v1/tasks/bulk-complete`

Completa hasta 100 tareas en una llamada. Una tarea ya completada vuelve en `failed` con `already_completed`; una cancelada, con `invalid_status`. Un resultado parcial no es error.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | IDs de los registros objetivo (1 a 100 por llamada). Los duplicados se ignoran. |

Otros campos y objetos anidados: `kobana schema dunning.tasks.bulk-complete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado por ítem de la finalización masiva. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## tasks get

Consultar tarea

```text
kobana dunning tasks get [ID] [OPCIONES]
```

`GET /v1/tasks/{id}`

Devuelve una tarea por ID, con resúmenes de cliente, responsable y cobro.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Tarea encontrada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## tasks update

Actualizar tarea

```text
kobana dunning tasks update [ID] [OPCIONES]
```

`PUT /v1/tasks/{id}`

Actualiza los datos de una tarea (parcial).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_id` | `string (uuid)` | no | Cliente vinculado a la tarea. |
| `charge_id` | `string (uuid)` | no | Cobro vinculado a la tarea. |
| `assigned_to_id` | `string (uuid)` | no | Usuario responsable de la tarea. |
| `title` | `string` | no | Título de la tarea. |
| `description` | `string` | no | Descripción detallada. |
| `task_type` | `string` | no | Tipo (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Valores: `call`, `email`, `visit`, `review`, `follow_up`. |
| `priority` | `string` | no | Prioridad (`low` \| `medium` \| `high` \| `urgent`). Valores: `low`, `medium`, `high`, `urgent`. |
| `status` | `string` | no | Nuevo estado; `completed` completa `completed_at` automáticamente. Valores: `pending`, `in_progress`, `completed`, `cancelled`. |
| `due_at` | `string (date-time)` | no | Plazo de la tarea (ISO 8601). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_data_mode` | `string` | no | Cómo se aplica `custom_data` en el update: `replace` (default) sobrescribe el objeto entero; `merge` fusiona (deep) el payload sobre el valor almacenado — los objetos anidados se fusionan, escalares/arrays/null sobrescriben la clave. Valores: `replace`, `merge`. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.tasks.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Tarea actualizada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## tasks complete

Completar tarea

```text
kobana dunning tasks complete [ID] [OPCIONES]
```

`POST /v1/tasks/{id}/complete`

Marca la tarea como completada y completa `completed_at`. Devuelve 409 si la tarea ya está completada o cancelada.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Tarea completada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
