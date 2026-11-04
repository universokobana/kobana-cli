---
id: notification-channels
title: "kobana dunning notification-channels"
sidebar_label: "notification-channels"
---

# kobana dunning notification-channels


Comandos de `kobana dunning notification-channels`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`notification-channels list`](#notification-channels-list) | `GET /v1/notification-channels` | Listar canales de notificación |
| [`notification-channels create`](#notification-channels-create) | `POST /v1/notification-channels` | Crear canal de notificación |
| [`notification-channels get`](#notification-channels-get) | `GET /v1/notification-channels/{id}` | Consultar canal de notificación |
| [`notification-channels update`](#notification-channels-update) | `PATCH /v1/notification-channels/{id}` | Actualizar canal de notificación |
| [`notification-channels delete`](#notification-channels-delete) | `DELETE /v1/notification-channels/{id}` | Eliminar canal de notificación |

## notification-channels list

Listar canales de notificación

```text
kobana dunning notification-channels list [OPCIONES]
```

`GET /v1/notification-channels`

Devuelve los canales de la organización. Las credenciales nunca se incluyen.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `channel_type` | `string` | no | Filtra por tipo de canal. Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | no | Filtra por proveedor. |
| `activated_at` | `string` | no | Filtra por activación: `true` (activos) \| `false`/`null` (inactivos). Valores: `true`, `false`, `null`. |
| `sort` | `string` | no | Campo de ordenación: `channel_type` \| `provider` \| `created_at` \| `updated_at`. Valores: `channelType`, `provider`, `createdAt`, `updatedAt`. |
| `order` | `string` | no | Dirección de la ordenación: `asc` \| `desc`. Valores: `asc`, `desc`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de canales. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notification-channels create

Crear canal de notificación

```text
kobana dunning notification-channels create [OPCIONES]
```

`POST /v1/notification-channels`

Crea un canal. Las credenciales se cifran en reposo antes de guardar. Un canal por combinación de tipo + proveedor.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `channel_type` | `string` | sí | Tipo de canal: `email` \| `sms` \| `whatsapp` \| `voice` \| `manual`. Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | sí | Proveedor del canal (ej.: `sendgrid`, `twilio`, `meta`). |
| `credentials` | `object` | sí | Credenciales del proveedor (ej.: `api_key`/`api_secret`). Cifradas en reposo al escribir y NUNCA devueltas en la lectura. Solo escritura. |
| `settings` | `object` | no | Configuraciones no sensibles del canal (JSON libre). |
| `cost_per_message` | `number` | no | Costo por mensaje enviado (cadena decimal). |
| `activated_at` | `string (date-time)` | no | Fecha de activación; `null` = canal inactivo. |
| `external_id` | `string` | no | Identificador del canal en el sistema externo del integrador. |
| `custom_data` | `object` | no | Datos personalizados (JSON) definidos por el integrador. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres para organización. |

Otros campos y objetos anidados: `kobana schema dunning.notification-channels.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Canal creado (sin credenciales). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notification-channels get

Consultar canal de notificación

```text
kobana dunning notification-channels get [ID] [OPCIONES]
```

`GET /v1/notification-channels/{id}`

Devuelve un canal por ID. Las credenciales nunca se incluyen.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal encontrado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notification-channels update

Actualizar canal de notificación

```text
kobana dunning notification-channels update [ID] [OPCIONES]
```

`PATCH /v1/notification-channels/{id}`

Actualiza un canal (parcial). Si se envía `credentials`, se vuelve a cifrar; omitir mantiene el secreto actual.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `channel_type` | `string` | no | Tipo de canal: `email` \| `sms` \| `whatsapp` \| `voice` \| `manual`. Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | no | Proveedor del canal (ej.: `sendgrid`, `twilio`, `meta`). |
| `credentials` | `object` | no | Credenciales del proveedor (ej.: `api_key`/`api_secret`). Cifradas en reposo al escribir y NUNCA devueltas en la lectura. Solo escritura. |
| `settings` | `object` | no | Configuraciones no sensibles del canal (JSON libre). |
| `cost_per_message` | `number` | no | Costo por mensaje enviado (cadena decimal). |
| `activated_at` | `string (date-time)` | no | Fecha de activación; `null` = canal inactivo. |
| `external_id` | `string` | no | Identificador del canal en el sistema externo del integrador. |
| `custom_data` | `object` | no | Datos personalizados (JSON) definidos por el integrador. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres para organización. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.notification-channels.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Canal actualizado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notification-channels delete

Eliminar canal de notificación

```text
kobana dunning notification-channels delete [ID] [OPCIONES]
```

`DELETE /v1/notification-channels/{id}`

Elimina el canal (soft delete).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Canal eliminado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
