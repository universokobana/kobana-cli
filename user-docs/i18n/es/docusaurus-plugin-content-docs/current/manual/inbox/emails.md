---
id: emails
title: "kobana inbox emails"
sidebar_label: "emails"
---

# kobana inbox emails


Comandos de `kobana inbox emails`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`emails list`](#emails-list) | `GET /v1/emails` | Listar Emails |
| [`emails create`](#emails-create) | `POST /v1/emails` | Enviar un .eml en bruto |
| [`emails get`](#emails-get) | `GET /v1/emails/{id}` | Obtener Email |
| [`emails delete`](#emails-delete) | `DELETE /v1/emails/{id}` | Eliminar correo |
| [`emails eml`](#emails-eml) | `GET /v1/emails/{id}/eml` | Descargar el .eml original |
| [`emails reprocess`](#emails-reprocess) | `POST /v1/emails/{id}/reprocess` | Reprocesar Email |

## emails list

Listar Emails

```text
kobana inbox emails list [OPCIONES]
```

`GET /v1/emails`

Paginación keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`); los filtros desconocidos, los campos no ordenables y los valores malformados se rechazan con 400 `VALIDATION_ERROR`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `updated_at`, `from`, `status`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `updated_at`, `from`, `status`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Igualdad. También `status[in]=a,b`. Valores: `received`, `approved`, `rejected`, `processing`, `processed`, `failed`. |
| `inbox_id` | `string` | no | Igualdad. También `inbox_id[in]=a,b`. |
| `agent_id` | `string` | no | Igualdad. También `agent_id[in]=a,b`. |
| `message_id` | `string` | no | Igualdad. También `message_id[in]=a,b`. |
| `from` | `string` | no | Igualdad. También `from[in]=a,b`. |
| `to` | `string` | no | Filas cuyo `to` contiene el valor. |
| `created_at` | `string` | no | Igualdad. También `created_at[in]=a,b` y `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `updated_at` | `string` | no | Igualdad. También `updated_at[in]=a,b` y `updated_at[gt\|gte\|lt\|lte]`. |
| `updated_at[gte]` | `string` | no |  |
| `updated_at[lte]` | `string` | no |  |
| `received_at` | `string` | no | Igualdad. También `received_at[in]=a,b` y `received_at[gt\|gte\|lt\|lte]`. |
| `received_at[gte]` | `string` | no |  |
| `received_at[lte]` | `string` | no |  |
| `processed_at` | `string` | no | Igualdad. También `processed_at[in]=a,b` y `processed_at[gt\|gte\|lt\|lte]`. |
| `processed_at[gte]` | `string` | no |  |
| `processed_at[lte]` | `string` | no |  |

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

## emails create

Enviar un .eml en bruto

```text
kobana inbox emails create [OPCIONES]
```

`POST /v1/emails`

Almacena un mensaje RFC 5322 exactamente como lo haría la ingesta de SES: objeto en bruto en el bucket raw, `Email` en `received` y un job `process-email` — filtros, agente y webhooks se ejecutan como para un correo real. Envía el mensaje como `multipart/form-data` (campo `file`, `inbox_id` opcional) o como cuerpo de la solicitud con `Content-Type: message/rfc822` (`?inbox_id=` opcional). Sin `inbox_id`, el buzón se resuelve a partir del encabezado `To:` entre los buzones del workspace del token. Un `Message-ID` ya almacenado en el mismo buzón se rechaza con 409 (`error.details.email_id` apunta al correo existente); el mismo `Message-ID` en otro buzón es otro correo.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `inbox_id` | `string` | no | Buzón de destino. Se sobrescribe con el campo multipart `inbox_id` cuando se envían ambos. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema inbox.emails.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Almacenado y encolado para procesamiento. |
| `400` | Cuerpo vacío o campo multipart `file` ausente. |
| `404` | `inbox_id` no existe en el workspace del token (`INBOX_NOT_FOUND`). |
| `409` | `DUPLICATE_MESSAGE_ID`: el `Message-ID` ya está almacenado en el buzón de destino. |
| `413` | `PAYLOAD_TOO_LARGE`: mensaje de más de 20 MB. |
| `415` | `UNSUPPORTED_MEDIA_TYPE`: el cuerpo no es multipart ni `message/rfc822`. |
| `422` | `NO_INBOX_MATCHES_TO_HEADER` (ningún buzón del workspace coincide con los destinatarios de `To:`), `MISSING_RECIPIENT` (sin encabezado `To:` utilizable; envía `inbox_id`) o `INVALID_MESSAGE_ID` (caracteres de control o más de 998 caracteres). |

## emails get

Obtener Email

```text
kobana inbox emails get [ID] [OPCIONES]
```

`GET /v1/emails/{id}`

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

## emails delete

Eliminar correo

```text
kobana inbox emails delete [ID] [OPCIONES]
```

`DELETE /v1/emails/{id}`

Soft-delete del correo (establece `deleted_at`). Deja de aparecer en listados y detalle; el objeto en bruto lo purga el job de retención.

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

## emails eml

Descargar el .eml original

```text
kobana inbox emails eml [ID] [OPCIONES]
```

`GET /v1/emails/{id}/eml`

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Flujo de mensaje RFC 5322 sin procesar. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## emails reprocess

Reprocesar Email

```text
kobana inbox emails reprocess [ID] [OPCIONES]
```

`POST /v1/emails/{id}/reprocess`

Devuelve el correo al procesamiento (filtros, agente, webhooks). Solo es elegible un correo en estado terminal — `processed`, `rejected` o `failed`; uno aún en `received`, `approved` o `processing` se rechaza con 409 `CONFLICT`, para que un agente (y su workflow) nunca corra dos veces para el mismo correo. Ejecutar el agente requiere un plan activo: sin él la llamada se rechaza con 403 `ENTITLEMENT_REQUIRED` (`error.details.reason` = `no_plan`) y no se encola nada.

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
| `409` | `CONFLICT`: el correo aún está en `received`, `approved` o `processing`. |
