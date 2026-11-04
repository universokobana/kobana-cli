---
id: inboxes
title: "kobana inbox inboxes"
sidebar_label: "inboxes"
---

# kobana inbox inboxes


Comandos de `kobana inbox inboxes`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`inboxes recipients remove`](#inboxes-recipients-remove) | `POST /v1/inboxes/{id}/recipients/remove` | Eliminar destinatario permitido |
| [`inboxes recipients create`](#inboxes-recipients-create) | `POST /v1/inboxes/{id}/recipients` | Agregar destinatario permitido |
| [`inboxes list`](#inboxes-list) | `GET /v1/inboxes` | Listar Inboxes |
| [`inboxes create`](#inboxes-create) | `POST /v1/inboxes` | Crear Inbox |
| [`inboxes update`](#inboxes-update) | `PUT /v1/inboxes` | Crear o actualizar buzón |
| [`inboxes get`](#inboxes-get) | `GET /v1/inboxes/{id}` | Obtener Inbox |
| [`inboxes delete`](#inboxes-delete) | `DELETE /v1/inboxes/{id}` | Eliminar Inbox |

## inboxes recipients remove

Eliminar destinatario permitido

```text
kobana inbox inboxes recipients remove [ID] [OPCIONES]
```

`POST /v1/inboxes/{id}/recipients/remove`

Acepta el UUID interno o el `external_id` del recurso. Quita un email de `allowed_recipients`. 404 cuando la dirección no está en la lista. Reemplaza `DELETE /v1/inboxes/{id}/recipients`: quitar una dirección es una modificación de la inbox (`write`), no una eliminación.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string (email)` | sí |  |

Otros campos y objetos anidados: `kobana schema inbox.inboxes.recipients.remove`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Destinatario eliminado. |
| `400` | `email` ausente o no es una dirección válida. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## inboxes recipients create

Agregar destinatario permitido

```text
kobana inbox inboxes recipients create [ID] [OPCIONES]
```

`POST /v1/inboxes/{id}/recipients`

Acepta el UUID interno o el `external_id` del recurso. Agrega un email a `allowed_recipients`. Es idempotente: devuelve 200 con la lista actual cuando ya está presente.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string (email)` | sí |  |

Otros campos y objetos anidados: `kobana schema inbox.inboxes.recipients.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El destinatario ya estaba presente (no-op). |
| `201` | Destinatario agregado. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## inboxes list

Listar Inboxes

```text
kobana inbox inboxes list [OPCIONES]
```

`GET /v1/inboxes`

Paginación keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`); los filtros desconocidos, los campos no ordenables y los valores malformados se rechazan con 400 `VALIDATION_ERROR`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `updated_at`, `label`, `slug`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `updated_at`, `label`, `slug`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Igualdad. También `status[in]=a,b`. Valores: `active`, `inactive`. |
| `agent_id` | `string` | no | Igualdad. También `agent_id[in]=a,b`. |
| `slug` | `string` | no | Igualdad. También `slug[in]=a,b`. |
| `email` | `string` | no | Igualdad. También `email[in]=a,b`. |
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

## inboxes create

Crear Inbox

```text
kobana inbox inboxes create [OPCIONES]
```

`POST /v1/inboxes`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `external_id` | `string` | no | Opcional. Por defecto usa el `id` interno generado. |
| `email` | `string (email)` | no | Opcional. Debe usar el dominio de inbox configurado (`SES_DOMAIN`) y una parte local que empiece con "\{organization-slug\}.\{workspace-slug\}." de quien llama; de lo contrario devuelve 422. Las direcciones duplicadas devuelven 409. Cuando se omite, el servidor la compone como "\{organization-slug\}.\{workspace-slug\}.\{inbox-slug\}@&lt;domain>". |
| `slug` | `string` | no | Opcional. Slug en minúsculas y dasherized, único dentro del workspace, usado como último segmento de la dirección. Devuelve 409 cuando colisiona con otro buzón del mismo workspace y 422 cuando la parte local compuesta supera los 64 caracteres. Si se omite, el servidor lo deriva del `label` dasherized. |
| `agent_id` | `string (uuid)` | no |  |
| `allowed_recipients` | `array<string (email)>` | no |  |
| `filters` | `array<object>` | no |  |
| `attachments_allow` | `boolean` | no |  |
| `attachments_max_file_number` | `integer` | no |  |
| `attachments_max_file_size` | `integer` | no |  |
| `attachments_max_total_size` | `integer` | no |  |
| `attachments_allow_rules` | `array<object>` | no | Lista de adjuntos permitidos. Un adjunto se procesa cuando cumple al menos una regla; dentro de una regla, todas las condiciones indicadas deben cumplirse. `null` o `[]` procesa todos los adjuntos. Un adjunto fuera de la lista se ignora, nunca es motivo para rechazar el correo: el agente no lo ve, los límites de adjuntos no lo cuentan y un paso de workflow no puede referenciarlo. En la actualización, omítalo para mantener y envíe `null` para limpiar. |
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. Envía `null` para limpiarlos. Máx. 64 KB serializados, 16 niveles de profundidad. |

Otros campos y objetos anidados: `kobana schema inbox.inboxes.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## inboxes update

Crear o actualizar buzón

```text
kobana inbox inboxes update [OPCIONES]
```

`PUT /v1/inboxes`

Upsert. Recibe el mismo cuerpo que POST más las claves de búsqueda `id` y `external_id`, comparadas en ese orden, luego por `email`. Una coincidencia se actualiza in situ y devuelve 200 — los campos ausentes del cuerpo conservan el valor almacenado. Sin clave, o sin coincidencia, crea el buzón exactamente como lo haría POST y devuelve 201.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `external_id` | `string` | no | Coincide por el id del sistema de origen. También se guarda en el registro cuando se crea. |
| `email` | `string (email)` | no | Opcional. Debe usar el dominio de inbox configurado (`SES_DOMAIN`) y una parte local que empiece con "\{organization-slug\}.\{workspace-slug\}." de quien llama; de lo contrario devuelve 422. Las direcciones duplicadas devuelven 409. Cuando se omite, el servidor la compone como "\{organization-slug\}.\{workspace-slug\}.\{inbox-slug\}@&lt;domain>". |
| `slug` | `string` | no | Opcional. Slug en minúsculas y dasherized, único dentro del workspace, usado como último segmento de la dirección. Devuelve 409 cuando colisiona con otro buzón del mismo workspace y 422 cuando la parte local compuesta supera los 64 caracteres. Si se omite, el servidor lo deriva del `label` dasherized. |
| `agent_id` | `string (uuid)` | no |  |
| `allowed_recipients` | `array<string (email)>` | no |  |
| `filters` | `array<object>` | no |  |
| `attachments_allow` | `boolean` | no |  |
| `attachments_max_file_number` | `integer` | no |  |
| `attachments_max_file_size` | `integer` | no |  |
| `attachments_max_total_size` | `integer` | no |  |
| `attachments_allow_rules` | `array<object>` | no | Lista de adjuntos permitidos. Un adjunto se procesa cuando cumple al menos una regla; dentro de una regla, todas las condiciones indicadas deben cumplirse. `null` o `[]` procesa todos los adjuntos. Un adjunto fuera de la lista se ignora, nunca es motivo para rechazar el correo: el agente no lo ve, los límites de adjuntos no lo cuentan y un paso de workflow no puede referenciarlo. En la actualización, omítalo para mantener y envíe `null` para limpiar. |
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. Envía `null` para limpiarlos. Máx. 64 KB serializados, 16 niveles de profundidad. |
| `id` | `string (uuid)` | no | Coincide por el id interno. Se comprueba antes que `external_id`. |

Otros campos y objetos anidados: `kobana schema inbox.inboxes.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## inboxes get

Obtener Inbox

```text
kobana inbox inboxes get [ID] [OPCIONES]
```

`GET /v1/inboxes/{id}`

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

## inboxes delete

Eliminar Inbox

```text
kobana inbox inboxes delete [ID] [OPCIONES]
```

`DELETE /v1/inboxes/{id}`

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
