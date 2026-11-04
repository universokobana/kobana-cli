---
id: notifications
title: "kobana dunning notifications"
sidebar_label: "notifications"
---

# kobana dunning notifications


Comandos de `kobana dunning notifications`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`notifications list`](#notifications-list) | `GET /v1/notifications` | Listar notificaciones |
| [`notifications create`](#notifications-create) | `POST /v1/notifications` | Crear notificación |
| [`notifications get`](#notifications-get) | `GET /v1/notifications/{id}` | Consultar notificación |
| [`notifications update`](#notifications-update) | `PUT /v1/notifications/{id}` | Actualizar notificación |
| [`notifications delete`](#notifications-delete) | `DELETE /v1/notifications/{id}` | Eliminar notificación |
| [`notifications resend`](#notifications-resend) | `POST /v1/notifications/{id}/resend` | Reenviar notificación |

## notifications list

Listar notificaciones

```text
kobana dunning notifications list [OPCIONES]
```

`GET /v1/notifications`

Lista las notificaciones de la organización con paginación y filtros. Ordenable vía `sort_by` (`created_at` | `updated_at` | `sent_at` | `delivered_at` | `channel` | `status`).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `sort_by` | `string` | no | Campo de ordenación (nombre público en snake_case, ej.: `due_date`). |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `channel` | `string` | no | Filtra por canal; acepta un valor único o una lista separada por comas (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `status` | `string` | no | Filtra por estado; acepta un valor único o una lista separada por comas (`pending` \| `queued` \| `sent` \| `delivered` \| `read` \| `failed` \| `bounced`). |
| `person_id` | `string` | no | Filtra por cliente (UUID). |
| `charge_id` | `string` | no | Filtra por cobro (UUID). |
| `include` | `string` | no | Relaciones a incluir, separadas por comas (`person`, `charge`, `step`; `template` equivale a `step`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de notificaciones. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notifications create

Crear notificación

```text
kobana dunning notifications create [OPCIONES]
```

`POST /v1/notifications`

Crea una notificación manual para un cliente, opcionalmente vinculada a un cobro y a un paso de la regla. El destinatario se valida según el canal.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_id` | `string (uuid)` | sí | Cliente destinatario. |
| `channel` | `string` | sí | Canal (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `recipient` | `string` | sí | Destinatario; validado según el canal (correo para `email`, teléfono para `sms`/`whatsapp`/`voice`). |
| `body` | `string` | sí | Cuerpo del mensaje. |
| `charge_id` | `string (uuid)` | no | Cobro relacionado con la notificación. |
| `collection_rule_step_id` | `string (uuid)` | no | Paso de la regla de cobro que originó la notificación. |
| `subject` | `string` | no | Asunto (para canales que lo admiten, p. ej., correo). |
| `external_id` | `string` | no | Identificador en TU sistema o en el proveedor de envío. |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.notifications.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Notificación creada con estado `pending`. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notifications get

Consultar notificación

```text
kobana dunning notifications get [ID] [OPCIONES]
```

`GET /v1/notifications/{id}`

Devuelve una notificación por ID; usa `include` para anexar relaciones.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `include` | `string` | no | Relaciones a incluir, separadas por comas (`person`, `charge`, `step`; `template` equivale a `step`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Notificación encontrada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notifications update

Actualizar notificación

```text
kobana dunning notifications update [ID] [OPCIONES]
```

`PUT /v1/notifications/{id}`

Actualiza el estado, los timestamps de entrega y los metadatos de una notificación (parcial). Útil para integraciones que confirman envío/entrega.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Nuevo estado; completa el timestamp correspondiente (sentAt/deliveredAt/readAt/failedAt) cuando no se informa. Valores: `pending`, `queued`, `sent`, `delivered`, `read`, `failed`, `bounced`. |
| `sent_at` | `string` | no | Timestamp ISO 8601 o fecha `YYYY-MM-DD`; null limpia el campo. |
| `delivered_at` | `string` | no | Timestamp ISO 8601 o fecha `YYYY-MM-DD`; null limpia el campo. |
| `read_at` | `string` | no | Timestamp ISO 8601 o fecha `YYYY-MM-DD`; null limpia el campo. |
| `failed_at` | `string` | no | Timestamp ISO 8601 o fecha `YYYY-MM-DD`; null limpia el campo. |
| `failure_reason` | `string` | no | Motivo del fallo reportado por el proveedor. |
| `external_id` | `string` | no | Identificador en TU sistema o en el proveedor de envío. |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `cost` | `number` | no | Costo del envío (número, mínimo 0). |

Otros campos y objetos anidados: `kobana schema dunning.notifications.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Notificación actualizada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notifications delete

Eliminar notificación

```text
kobana dunning notifications delete [ID] [OPCIONES]
```

`DELETE /v1/notifications/{id}`

Elimina una notificación DEFINITIVAMENTE (borrado físico — el modelo no tiene borrado lógico). Solo la notificación aún no entregada: la entregada es evidencia probatoria y responde `409`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Notificación eliminada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | La notificación ya fue entregada — es evidencia probatoria y no puede eliminarse. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notifications resend

Reenviar notificación

```text
kobana dunning notifications resend [ID] [OPCIONES]
```

`POST /v1/notifications/{id}/resend`

Crea una NUEVA notificación `pending` copiando la original (el `metadata` de la nueva referencia la original en `resend_of`). Solo se permite cuando la original está `failed` o `bounced`; en caso contrario devuelve 400. Admite `Idempotency-Key`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Nueva notificación creada a partir de la original. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
