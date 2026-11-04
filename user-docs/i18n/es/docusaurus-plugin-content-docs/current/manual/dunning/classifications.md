---
id: classifications
title: "kobana dunning classifications"
sidebar_label: "classifications"
---

# kobana dunning classifications


Comandos de `kobana dunning classifications`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`classifications list`](#classifications-list) | `GET /v1/classifications` | Listar clasificaciones |
| [`classifications create`](#classifications-create) | `POST /v1/classifications` | Crear clasificación |
| [`classifications get`](#classifications-get) | `GET /v1/classifications/{id}` | Consultar clasificación |
| [`classifications update`](#classifications-update) | `PUT /v1/classifications/{id}` | Actualizar clasificación |
| [`classifications delete`](#classifications-delete) | `DELETE /v1/classifications/{id}` | Eliminar clasificación |

## classifications list

Listar clasificaciones

```text
kobana dunning classifications list [OPCIONES]
```

`GET /v1/classifications`

Lista las clasificaciones de la organización, por defecto ordenadas por `priority` y `name`. Usa `include=_count` para los contadores.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `is_default` | `string` | no | Filtra por la clasificación predeterminada (`true` \| `false`). Valores: `true`, `false`. |
| `auto_assign` | `string` | no | Filtra por asignación automática (`true` \| `false`). Valores: `true`, `false`. |
| `activated_at` | `string` | no | Filtra por activación: `true` = activas, `false` o `null` = inactivas. Valores: `true`, `false`, `null`. |
| `include` | `string` | no | Relaciones a incluir. Soportado: `_count` (clientes y reglas). |
| `sort` | `string` | no | Campo de ordenación (`name` \| `code` \| `priority` \| `created_at` \| `updated_at`). Valores: `name`, `code`, `priority`, `createdAt`, `updatedAt`. |
| `order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de clasificaciones. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## classifications create

Crear clasificación

```text
kobana dunning classifications create [OPCIONES]
```

`POST /v1/classifications`

Crea una clasificación. Marcar `is_default` desmarca la predeterminada anterior; un `code` duplicado falla con 409.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la clasificación. |
| `code` | `string` | sí | Código estable (minúsculas, números y guion bajo; único en la organización). |
| `description` | `string` | no | Descripción de la clasificación. |
| `color` | `string` | no | Color hexadecimal mostrado en la interfaz (ej.: `#22c55e`). |
| `priority` | `integer` | no | Prioridad de evaluación (menor se evalúa primero). |
| `is_default` | `boolean` | no | true cuando es la clasificación predeterminada de la organización (única). |
| `auto_assign` | `boolean` | no | true cuando los clientes se asignan automáticamente por los criterios. |
| `criteria` | `object` | no | Criterios de asignación automática (comportamiento de pago). |
| `activated_at` | `string (date-time)` | no | Momento de activación (null = inactiva). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.classifications.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Clasificación creada (con contadores). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## classifications get

Consultar clasificación

```text
kobana dunning classifications get [ID] [OPCIONES]
```

`GET /v1/classifications/{id}`

Devuelve una clasificación por ID. Usa `include=_count` para los contadores.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `include` | `string` | no | Relaciones a incluir. Soportado: `_count` (clientes y reglas). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Clasificación encontrada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## classifications update

Actualizar clasificación

```text
kobana dunning classifications update [ID] [OPCIONES]
```

`PUT /v1/classifications/{id}`

Actualiza una clasificación (parcial). Un `code` duplicado falla con 409.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre de la clasificación. |
| `code` | `string` | no | Código estable (minúsculas, números y guion bajo; único en la organización). |
| `description` | `string` | no | Descripción de la clasificación. |
| `color` | `string` | no | Color hexadecimal mostrado en la interfaz (ej.: `#22c55e`). |
| `priority` | `integer` | no | Prioridad de evaluación (menor se evalúa primero). |
| `is_default` | `boolean` | no | true cuando es la clasificación predeterminada de la organización (única). |
| `auto_assign` | `boolean` | no | true cuando los clientes se asignan automáticamente por los criterios. |
| `criteria` | `object` | no | Criterios de asignación automática (comportamiento de pago). |
| `activated_at` | `string (date-time)` | no | Momento de activación (null = inactiva). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.classifications.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Clasificación actualizada (con contadores). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## classifications delete

Eliminar clasificación

```text
kobana dunning classifications delete [ID] [OPCIONES]
```

`DELETE /v1/classifications/{id}`

Elimina (borrado lógico) una clasificación. Falla con 409 si hay clientes o reglas asociados. El `code` sigue reservado.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Clasificación eliminada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
