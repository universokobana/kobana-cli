---
id: workspaces
title: "kobana inbox workspaces"
sidebar_label: "workspaces"
---

# kobana inbox workspaces


Comandos de `kobana inbox workspaces`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | Listar Workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Crear Workspace |
| [`workspaces update`](#workspaces-update) | `PUT /v1/workspaces` | Crear o actualizar workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Obtener Workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Eliminar Workspace |

## workspaces list

Listar Workspaces

```text
kobana inbox workspaces list [OPCIONES]
```

`GET /v1/workspaces`

Alcance: la organización del workspace del token — todo workspace vivo de esa organización es alcanzable; uno fuera de ella es 404. Un token cuyo workspace no tiene organización solo se ve a sí mismo. Paginación keyset con filtros tipados.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `updated_at`, `label`, `slug`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `updated_at`, `label`, `slug`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `source` | `string` | no | Igualdad. También `source[in]=a,b`. |
| `slug` | `string` | no | Igualdad. También `slug[in]=a,b`. |
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

## workspaces create

Crear Workspace

```text
kobana inbox workspaces create [OPCIONES]
```

`POST /v1/workspaces`

Requiere un token `src=system`. El workspace se crea dentro de la organización del token. Alcance: la organización del workspace del token — todo workspace vivo de esa organización es alcanzable; uno fuera de ella es 404. Un token cuyo workspace no tiene organización solo se ve a sí mismo.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `source` | `string` | sí | Valores: `kobana_app`, `kobana_billing`, `kobana_inbox`, `kobana_agent`, `kobana_finance`, `kobana_legal`, `kobana_team`. |
| `label` | `string` | sí |  |
| `external_id` | `string` | no | Opcional. Cuando se omite, el servidor usa como valor predeterminado de `external_id` el `id` interno generado. |
| `slug` | `string` | no | Opcional. Slug en minúsculas y dasherized, único dentro de la organización, usado como segmento intermedio de las direcciones de buzón. Limitado a 20 caracteres para que la parte local compuesta quepa en el límite SMTP de 64 caracteres. Si se omite, el servidor lo deriva del `label` dasherized. |
| `monthly_cost_limit` | `number` | no |  |
| `monthly_emails_limit` | `integer` | no |  |
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. Envía `null` para limpiarlos. Máx. 64 KB serializados, 16 niveles de profundidad. |

Otros campos y objetos anidados: `kobana schema inbox.workspaces.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## workspaces update

Crear o actualizar workspace

```text
kobana inbox workspaces update [OPCIONES]
```

`PUT /v1/workspaces`

Upsert. Recibe el mismo cuerpo que POST más las claves de búsqueda `id` y `external_id`, comparadas en ese orden. Una coincidencia se actualiza in situ y devuelve 200 — los campos ausentes del cuerpo conservan el valor almacenado. Sin clave, o sin coincidencia, crea el workspace exactamente como lo haría POST y devuelve 201. Alcance: la organización del workspace del token — todo workspace vivo de esa organización es alcanzable; uno fuera de ella es 404. Un token cuyo workspace no tiene organización solo se ve a sí mismo. Requiere un token `src=system`; `external_id` no puede cambiarse en un workspace coincidente (409).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `source` | `string` | sí | Valores: `kobana_app`, `kobana_billing`, `kobana_inbox`, `kobana_agent`, `kobana_finance`, `kobana_legal`, `kobana_team`. |
| `label` | `string` | sí |  |
| `external_id` | `string` | no | Coincide por el id del sistema de origen. También se guarda en el registro cuando se crea. |
| `slug` | `string` | no | Opcional. Slug en minúsculas y dasherized, único dentro de la organización, usado como segmento intermedio de las direcciones de buzón. Limitado a 20 caracteres para que la parte local compuesta quepa en el límite SMTP de 64 caracteres. Si se omite, el servidor lo deriva del `label` dasherized. |
| `monthly_cost_limit` | `number` | no |  |
| `monthly_emails_limit` | `integer` | no |  |
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. Envía `null` para limpiarlos. Máx. 64 KB serializados, 16 niveles de profundidad. |
| `id` | `string (uuid)` | no | Coincide por el id interno. Se comprueba antes que `external_id`. |

Otros campos y objetos anidados: `kobana schema inbox.workspaces.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## workspaces get

Obtener Workspace

```text
kobana inbox workspaces get [ID] [OPCIONES]
```

`GET /v1/workspaces/{id}`

Acepta el UUID interno o el `external_id` del recurso. Alcance: la organización del workspace del token — todo workspace vivo de esa organización es alcanzable; uno fuera de ella es 404. Un token cuyo workspace no tiene organización solo se ve a sí mismo.

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

## workspaces delete

Eliminar Workspace

```text
kobana inbox workspaces delete [ID] [OPCIONES]
```

`DELETE /v1/workspaces/{id}`

Acepta el UUID interno o el `external_id` del recurso. Alcance: la organización del workspace del token — todo workspace vivo de esa organización es alcanzable; uno fuera de ella es 404. Un token cuyo workspace no tiene organización solo se ve a sí mismo. Requiere un token `src=system`.

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
