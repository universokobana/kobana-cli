---
id: webhooks
title: "kobana inbox webhooks"
sidebar_label: "webhooks"
---

# kobana inbox webhooks


Comandos de `kobana inbox webhooks`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`webhooks deliveries get`](#webhooks-deliveries-get) | `GET /v1/webhooks/{id}/deliveries/{deliveryId}` | Obtener entrega del Webhook |
| [`webhooks deliveries replay`](#webhooks-deliveries-replay) | `POST /v1/webhooks/{id}/deliveries/{deliveryId}/replay` | Reproducir una entrega del Webhook |
| [`webhooks deliveries list`](#webhooks-deliveries-list) | `GET /v1/webhooks/{id}/deliveries` | Listar entregas del Webhook |
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | Listar Webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Crear Webhook |
| [`webhooks update`](#webhooks-update) | `PUT /v1/webhooks` | Crear o actualizar webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Obtener Webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Eliminar Webhook |
| [`webhooks regenerate-secret`](#webhooks-regenerate-secret) | `POST /v1/webhooks/{id}/regenerate-secret` | Rotar el signing secret |
| [`webhooks test`](#webhooks-test) | `POST /v1/webhooks/{id}/test` | Enviar un evento webhook.ping |

## webhooks deliveries get

Obtener entrega del Webhook

```text
kobana inbox webhooks deliveries get [ID] [DELIVERYID] [OPCIONES]
```

`GET /v1/webhooks/{id}/deliveries/{deliveryId}`

Acepta el UUID interno o el `external_id` del recurso.

**Argumentos de la ruta:** `id`, `deliveryId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## webhooks deliveries replay

Reproducir una entrega del Webhook

```text
kobana inbox webhooks deliveries replay [ID] [DELIVERYID] [OPCIONES]
```

`POST /v1/webhooks/{id}/deliveries/{deliveryId}/replay`

Acepta el UUID interno o el `external_id` del recurso.

**Argumentos de la ruta:** `id`, `deliveryId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Aceptado y encolado. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## webhooks deliveries list

Listar entregas del Webhook

```text
kobana inbox webhooks deliveries list [ID] [OPCIONES]
```

`GET /v1/webhooks/{id}/deliveries`

Acepta el UUID interno o el `external_id` del recurso.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `attempt`, `status`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `attempt`, `status`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Igualdad. También `status[in]=a,b`. Valores: `pending`, `success`, `failed`. |
| `attempt` | `integer` | no | Igualdad. También `attempt[in]=a,b` y `attempt[gt\|gte\|lt\|lte]`. |
| `attempt[gte]` | `integer` | no |  |
| `attempt[lte]` | `integer` | no |  |
| `response_status` | `integer` | no | Igualdad. También `response_status[in]=a,b` y `response_status[gt\|gte\|lt\|lte]`. |
| `response_status[gte]` | `integer` | no |  |
| `response_status[lte]` | `integer` | no |  |
| `system_event_id` | `string` | no | Igualdad. También `system_event_id[in]=a,b`. |
| `created_at` | `string` | no | Igualdad. También `created_at[in]=a,b` y `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `delivered_at` | `string` | no | Igualdad. También `delivered_at[in]=a,b` y `delivered_at[gt\|gte\|lt\|lte]`. |
| `delivered_at[gte]` | `string` | no |  |
| `delivered_at[lte]` | `string` | no |  |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de resultados. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## webhooks list

Listar Webhooks

```text
kobana inbox webhooks list [OPCIONES]
```

`GET /v1/webhooks`

Paginación keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`); los filtros desconocidos, los campos no ordenables y los valores malformados se rechazan con 400 `VALIDATION_ERROR`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `updated_at`, `label`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `updated_at`, `label`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Igualdad. También `status[in]=a,b`. Valores: `active`, `inactive`. |
| `inbox_id` | `string` | no | Igualdad. También `inbox_id[in]=a,b`. |
| `external_id` | `string` | no | Igualdad. También `external_id[in]=a,b`. |
| `created_at` | `string` | no | Igualdad. También `created_at[in]=a,b` y `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `updated_at` | `string` | no | Igualdad. También `updated_at[in]=a,b` y `updated_at[gt\|gte\|lt\|lte]`. |
| `updated_at[gte]` | `string` | no |  |
| `updated_at[lte]` | `string` | no |  |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de resultados. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## webhooks create

Crear Webhook

```text
kobana inbox webhooks create [OPCIONES]
```

`POST /v1/webhooks`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `url` | `string (uri)` | sí |  |
| `external_id` | `string` | no | Opcional. Por defecto usa el `id` interno generado. |
| `inbox_id` | `string (uuid)` | no |  |
| `subscribed_events` | `array<string>` | no |  |
| `auth` | `string` | no | Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |
| `signing_secret` | `string` | no | Opcional. Trae tu propio signing secret: `whsec_` + base64 estándar de 24-64 bytes aleatorios. Se genera (32 bytes) si se omite. Se devuelve en texto claro solo en esta respuesta. |
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. Envía `null` para limpiarlos. Máx. 64 KB serializados, 16 niveles de profundidad. |

Otros campos y objetos anidados: `kobana schema inbox.webhooks.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## webhooks update

Crear o actualizar webhook

```text
kobana inbox webhooks update [OPCIONES]
```

`PUT /v1/webhooks`

Upsert. Recibe el mismo cuerpo que POST más las claves de búsqueda `id` y `external_id`, comparadas en ese orden. Una coincidencia se actualiza in situ y devuelve 200 — los campos ausentes del cuerpo conservan el valor almacenado. Sin clave, o sin coincidencia, crea el webhook exactamente como lo haría POST y devuelve 201.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `url` | `string (uri)` | sí |  |
| `external_id` | `string` | no | Coincide por el id del sistema de origen. También se guarda en el registro cuando se crea. |
| `inbox_id` | `string (uuid)` | no |  |
| `subscribed_events` | `array<string>` | no |  |
| `auth` | `string` | no | Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |
| `signing_secret` | `string` | no | Opcional. Trae tu propio signing secret: `whsec_` + base64 estándar de 24-64 bytes aleatorios. Se genera (32 bytes) si se omite. Se devuelve en texto claro solo en esta respuesta. |
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. Envía `null` para limpiarlos. Máx. 64 KB serializados, 16 niveles de profundidad. |
| `id` | `string (uuid)` | no | Coincide por el id interno. Se comprueba antes que `external_id`. |

Otros campos y objetos anidados: `kobana schema inbox.webhooks.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## webhooks get

Obtener Webhook

```text
kobana inbox webhooks get [ID] [OPCIONES]
```

`GET /v1/webhooks/{id}`

Acepta el UUID interno o el `external_id` del recurso.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## webhooks delete

Eliminar Webhook

```text
kobana inbox webhooks delete [ID] [OPCIONES]
```

`DELETE /v1/webhooks/{id}`

Acepta el UUID interno o el `external_id` del recurso.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Eliminado (soft delete). |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## webhooks regenerate-secret

Rotar el signing secret

```text
kobana inbox webhooks regenerate-secret [ID] [OPCIONES]
```

`POST /v1/webhooks/{id}/regenerate-secret`

Reemplaza el signing secret y devuelve el nuevo valor **una sola vez**. Por defecto el secret anterior sigue firmando entregas durante 24 horas (`previous_secret_grace_hours`), periodo en el que cada entrega lleva una firma por secret, para que el receptor cambie de clave sin perder eventos. Envía `0` para revocar el secret anterior de inmediato (secret filtrado). Acepta tanto el UUID interno como el `external_id` del recurso.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `signing_secret` | `string` | no | Opcional. Trae tu propio signing secret: `whsec_` + base64 estándar de 24-64 bytes aleatorios. Se genera (32 bytes) si se omite. Se devuelve en texto claro solo en esta respuesta. |
| `previous_secret_grace_hours` | `integer` | no | Horas que el secret anterior sigue siendo válido tras la rotación. `0` lo revoca de inmediato. |

Otros campos y objetos anidados: `kobana schema inbox.webhooks.regenerate-secret`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Nuevo signing secret (se muestra una sola vez). |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## webhooks test

Enviar un evento webhook.ping

```text
kobana inbox webhooks test [ID] [OPCIONES]
```

`POST /v1/webhooks/{id}/test`

Acepta el UUID interno o el `external_id` del recurso.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Aceptado y encolado. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
