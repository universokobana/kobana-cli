---
id: templates
title: "kobana dunning templates"
sidebar_label: "templates"
---

# kobana dunning templates


Comandos de `kobana dunning templates`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`templates list`](#templates-list) | `GET /v1/templates` | Listar plantillas |
| [`templates create`](#templates-create) | `POST /v1/templates` | Crear plantilla |
| [`templates get`](#templates-get) | `GET /v1/templates/{id}` | Consultar plantilla |
| [`templates update`](#templates-update) | `PUT /v1/templates/{id}` | Actualizar plantilla |
| [`templates delete`](#templates-delete) | `DELETE /v1/templates/{id}` | Eliminar plantilla |
| [`templates duplicate`](#templates-duplicate) | `POST /v1/templates/{id}/duplicate` | Duplicar plantilla |

## templates list

Listar plantillas

```text
kobana dunning templates list [OPCIONES]
```

`GET /v1/templates`

Lista las plantillas de mensaje de la organización con paginación, búsqueda y filtros.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `search` | `string` | no | Búsqueda por nombre, asunto o cuerpo. |
| `channel` | `string` | no | Filtra por canal. Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | no | Filtra por categoría. Valores: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `tone` | `string` | no | Filtra por tono. Valores: `friendly`, `neutral`, `firm`, `urgent`. |
| `is_active` | `string` | no | Filtra por activación: `true` = activas, `false` = inactivas. Valores: `true`, `false`. |
| `sort_by` | `string` | no | Campo de ordenación (nombre público en snake_case, ej.: `due_date`). Valores: `name`, `channel`, `category`, `createdAt`, `updatedAt`. |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de plantillas. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## templates create

Crear plantilla

```text
kobana dunning templates create [OPCIONES]
```

`POST /v1/templates`

Crea una plantilla de mensaje. Falla con 409 si ya existe una plantilla con el mismo `name` o `external_id`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la plantilla (único en la organización). |
| `channel` | `string` | sí | Canal de envío (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | sí | Categoría (`reminder` \| `overdue` \| `negotiation` \| `negativation_warning` \| `protest_warning` \| `payment_confirmation`). Valores: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `body` | `string` | sí | Cuerpo del mensaje, con variables en formato `{{variable}}`. |
| `tone` | `string` | no | Tono del mensaje (`friendly` \| `neutral` \| `firm` \| `urgent`). Valores: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `subject` | `string` | no | Asunto (usado en correo). |
| `variables` | `array<string>` | no | Variables disponibles en el cuerpo/asunto. |
| `activated_at` | `string (date-time)` | no | Momento de activación (null = inactiva). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones; único en la organización). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.templates.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Plantilla creada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## templates get

Consultar plantilla

```text
kobana dunning templates get [ID] [OPCIONES]
```

`GET /v1/templates/{id}`

Devuelve una plantilla por ID.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plantilla encontrada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## templates update

Actualizar plantilla

```text
kobana dunning templates update [ID] [OPCIONES]
```

`PUT /v1/templates/{id}`

Actualiza una plantilla (parcial). Falla con 409 si el nuevo `name` o `external_id` ya está en uso.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre de la plantilla (único en la organización). |
| `channel` | `string` | no | Canal de envío (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `category` | `string` | no | Categoría (`reminder` \| `overdue` \| `negotiation` \| `negativation_warning` \| `protest_warning` \| `payment_confirmation`). Valores: `reminder`, `overdue`, `negotiation`, `negativation_warning`, `protest_warning`, `payment_confirmation`. |
| `tone` | `string` | no | Tono del mensaje (`friendly` \| `neutral` \| `firm` \| `urgent`). Valores: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `subject` | `string` | no | Asunto (usado en correo). |
| `body` | `string` | no | Cuerpo del mensaje, con variables en formato `{{variable}}`. |
| `variables` | `array<string>` | no | Variables disponibles en el cuerpo/asunto. |
| `activated_at` | `string (date-time)` | no | Momento de activación (null = inactiva). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones; único en la organización). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.templates.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plantilla actualizada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## templates delete

Eliminar plantilla

```text
kobana dunning templates delete [ID] [OPCIONES]
```

`DELETE /v1/templates/{id}`

Elimina (borrado lógico) una plantilla. Falla con 409 si está en uso por etapas de regla.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Plantilla eliminada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## templates duplicate

Duplicar plantilla

```text
kobana dunning templates duplicate [ID] [OPCIONES]
```

`POST /v1/templates/{id}/duplicate`

Crea una copia de la plantilla con el nombre indicado. La copia nace inactiva (`activated_at` null) y sin `external_id`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la plantilla (único en la organización). |

Otros campos y objetos anidados: `kobana schema dunning.templates.duplicate`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Plantilla duplicada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
