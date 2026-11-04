---
id: system-events
title: "kobana inbox system-events"
sidebar_label: "system-events"
---

# kobana inbox system-events


Comandos de `kobana inbox system-events`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`system-events list`](#system-events-list) | `GET /v1/system-events` | Listar SystemEvents |
| [`system-events get`](#system-events-get) | `GET /v1/system-events/{id}` | Obtener SystemEvent |

## system-events list

Listar SystemEvents

```text
kobana inbox system-events list [OPCIONES]
```

`GET /v1/system-events`

Paginación keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`); los filtros desconocidos, los campos no ordenables y los valores malformados se rechazan con 400 `VALIDATION_ERROR`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `slug`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `slug`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `slug` | `string` | no | Igualdad. También `slug[in]=a,b`. |
| `inbox_id` | `string` | no | Igualdad. También `inbox_id[in]=a,b`. |
| `email_id` | `string` | no | Igualdad. También `email_id[in]=a,b`. |
| `created_at` | `string` | no | Igualdad. También `created_at[in]=a,b` y `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |

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

## system-events get

Obtener SystemEvent

```text
kobana inbox system-events get [ID] [OPCIONES]
```

`GET /v1/system-events/{id}`

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
