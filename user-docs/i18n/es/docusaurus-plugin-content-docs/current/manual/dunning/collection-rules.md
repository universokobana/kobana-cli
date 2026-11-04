---
id: collection-rules
title: "kobana dunning collection-rules"
sidebar_label: "collection-rules"
---

# kobana dunning collection-rules


Comandos de `kobana dunning collection-rules`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`collection-rules steps`](#collection-rules-steps) | `GET /v1/collection-rules/{id}/steps` | Listar etapas de la regla |
| [`collection-rules steps create`](#collection-rules-steps-create) | `POST /v1/collection-rules/{id}/steps` | Crear etapa |
| [`collection-rules steps update`](#collection-rules-steps-update) | `PUT /v1/collection-rules/{id}/steps/{stepId}` | Actualizar etapa |
| [`collection-rules steps delete`](#collection-rules-steps-delete) | `DELETE /v1/collection-rules/{id}/steps/{stepId}` | Eliminar etapa |
| [`collection-rules list`](#collection-rules-list) | `GET /v1/collection-rules` | Listar reglas |
| [`collection-rules create`](#collection-rules-create) | `POST /v1/collection-rules` | Crear regla |
| [`collection-rules get`](#collection-rules-get) | `GET /v1/collection-rules/{id}` | Consultar regla |
| [`collection-rules update`](#collection-rules-update) | `PUT /v1/collection-rules/{id}` | Actualizar regla |
| [`collection-rules delete`](#collection-rules-delete) | `DELETE /v1/collection-rules/{id}` | Eliminar regla |

## collection-rules steps

Listar etapas de la regla

```text
kobana dunning collection-rules steps [ID] [OPCIONES] [COMANDO]
```

`GET /v1/collection-rules/{id}/steps`

Lista las etapas de la regla ordenadas por `position`, con la plantilla de mensaje incluida.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Etapas de la regla (envelope `data`, sin paginación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## collection-rules steps create

Crear etapa

```text
kobana dunning collection-rules steps create [ID] [OPCIONES]
```

`POST /v1/collection-rules/{id}/steps`

Crea una etapa en la regla. Sin `position` entra al final; con `position`, desplaza las etapas siguientes.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la etapa. |
| `trigger_type` | `string` | sí | Disparador (`before_due` \| `on_due` \| `after_due`). Los disparadores `after_issue`/`after_payment` solo se definen vía la regla. Valores: `before_due`, `on_due`, `after_due`. |
| `trigger_days` | `integer` | sí | Días relativos al disparador (0 = el mismo día). |
| `action_type` | `string` | sí | Acción (`notification` \| `negativation` \| `protest` \| `manual_task` \| `negotiation_offer` \| `judicial`). Valores: `notification`, `negativation`, `protest`, `manual_task`, `negotiation_offer`, `judicial`. |
| `channels` | `array<string>` | sí | Canales de envío (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `message_template_id` | `string (uuid)` | no | Plantilla de mensaje usada por la etapa (debe pertenecer a la organización). |
| `position` | `integer` | no | Posición de la etapa en la regla (desde 1). |
| `tone` | `string` | no | Tono del mensaje (`friendly` \| `neutral` \| `firm` \| `urgent`). Valores: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `preferred_time` | `string` | no | Horario preferido de envío (ej.: `09:00`). |
| `activated_at` | `string (date-time)` | no | Momento de activación (null = inactiva). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.collection-rules.steps.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Etapa creada (con plantilla de mensaje). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## collection-rules steps update

Actualizar etapa

```text
kobana dunning collection-rules steps update [ID] [STEPID] [OPCIONES]
```

`PUT /v1/collection-rules/{id}/steps/{stepId}`

Actualiza una etapa (parcial). Cambiar `position` reordena automáticamente las demás etapas.

**Argumentos de la ruta:** `id`, `stepId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `message_template_id` | `string (uuid)` | no | Plantilla de mensaje usada por la etapa (debe pertenecer a la organización). |
| `position` | `integer` | no | Posición de la etapa en la regla (desde 1). |
| `name` | `string` | no | Nombre de la etapa. |
| `trigger_type` | `string` | no | Disparador (`before_due` \| `on_due` \| `after_due`). Los disparadores `after_issue`/`after_payment` solo se definen vía la regla. Valores: `before_due`, `on_due`, `after_due`. |
| `trigger_days` | `integer` | no | Días relativos al disparador (0 = el mismo día). |
| `action_type` | `string` | no | Acción (`notification` \| `negativation` \| `protest` \| `manual_task` \| `negotiation_offer` \| `judicial`). Valores: `notification`, `negativation`, `protest`, `manual_task`, `negotiation_offer`, `judicial`. |
| `channels` | `array<string>` | no | Canales de envío (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `tone` | `string` | no | Tono del mensaje (`friendly` \| `neutral` \| `firm` \| `urgent`). Valores: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `preferred_time` | `string` | no | Horario preferido de envío (ej.: `09:00`). |
| `activated_at` | `string (date-time)` | no | Momento de activación (null = inactiva). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.collection-rules.steps.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Etapa actualizada (con plantilla de mensaje). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## collection-rules steps delete

Eliminar etapa

```text
kobana dunning collection-rules steps delete [ID] [STEPID] [OPCIONES]
```

`DELETE /v1/collection-rules/{id}/steps/{stepId}`

Elimina (borrado lógico) una etapa y reordena las restantes. Falla con 409 si la etapa tiene notificaciones asociadas.

**Argumentos de la ruta:** `id`, `stepId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Etapa eliminada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## collection-rules list

Listar reglas

```text
kobana dunning collection-rules list [OPCIONES]
```

`GET /v1/collection-rules`

Lista las reglas de cobranza de la organización, ordenadas por prioridad. `_count.charges` trae el total de cobros activos en la regla.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `classification_id` | `string` | no | Filtra por la clasificación asociada (UUID). |
| `is_default` | `string` | no | Filtra por la regla predeterminada (`true` \| `false`). Valores: `true`, `false`. |
| `activated_at` | `string` | no | Filtra por activación: `true` = activas, `false` = inactivas. Valores: `true`, `false`. |
| `include` | `string` | no | Relaciones a incluir, separadas por coma. Soportado: `steps`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de reglas. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## collection-rules create

Crear regla

```text
kobana dunning collection-rules create [OPCIONES]
```

`POST /v1/collection-rules`

Crea una regla de cobranza, opcionalmente ya con etapas. Marcar `is_default` desmarca la regla predeterminada anterior.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la regla. |
| `classification_id` | `string (uuid)` | no | Clasificación de clientes atendida por la regla (segmentación). |
| `description` | `string` | no | Descripción de la regla. |
| `is_default` | `boolean` | no | true cuando es la regla predeterminada de la organización (única). |
| `priority` | `integer` | no | Prioridad en la selección de regla (mayor gana). |
| `activated_at` | `string (date-time)` | no | Momento de activación (null = inactiva). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `steps` | `array<object>` | no | Etapas de la regla ordenadas por `position` (presente con `include=steps` y en las respuestas de creación/actualización). |

Otros campos y objetos anidados: `kobana schema dunning.collection-rules.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Regla creada (con clasificación y etapas). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## collection-rules get

Consultar regla

```text
kobana dunning collection-rules get [ID] [OPCIONES]
```

`GET /v1/collection-rules/{id}`

Devuelve una regla por ID. Usa `include=steps` para incluir las etapas.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `include` | `string` | no | Relaciones a incluir, separadas por coma. Soportado: `steps`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Regla encontrada (envelope `data`). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## collection-rules update

Actualizar regla

```text
kobana dunning collection-rules update [ID] [OPCIONES]
```

`PUT /v1/collection-rules/{id}`

Actualiza los datos de la regla (parcial). El array `steps`, cuando se envía, hace upsert y elimina las etapas ausentes.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `classification_id` | `string (uuid)` | no | Clasificación de clientes atendida por la regla (segmentación). |
| `name` | `string` | no | Nombre de la regla. |
| `description` | `string` | no | Descripción de la regla. |
| `is_default` | `boolean` | no | true cuando es la regla predeterminada de la organización (única). |
| `priority` | `integer` | no | Prioridad en la selección de regla (mayor gana). |
| `activated_at` | `string (date-time)` | no | Momento de activación (null = inactiva). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `steps` | `array<object>` | no | Conjunto completo de etapas (upsert): con `id` actualiza, sin `id` crea, las ausentes se eliminan. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.collection-rules.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Regla actualizada (envelope `data`). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## collection-rules delete

Eliminar regla

```text
kobana dunning collection-rules delete [ID] [OPCIONES]
```

`DELETE /v1/collection-rules/{id}`

Elimina (borrado lógico) la regla y sus etapas. Falla con 409 si hay cobros usando la regla.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Regla eliminada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
