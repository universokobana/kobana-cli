---
id: agents
title: "kobana inbox agents"
sidebar_label: "agents"
---

# kobana inbox agents


Comandos de `kobana inbox agents`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`agents list`](#agents-list) | `GET /v1/agents` | Listar Agents |
| [`agents create`](#agents-create) | `POST /v1/agents` | Crear Agent |
| [`agents update`](#agents-update) | `PUT /v1/agents` | Crear o actualizar agente |
| [`agents get`](#agents-get) | `GET /v1/agents/{id}` | Obtener Agent |
| [`agents delete`](#agents-delete) | `DELETE /v1/agents/{id}` | Eliminar Agent |
| [`agents versions`](#agents-versions) | `GET /v1/agents/{id}/versions` | Listar versiones del prompt del agente |

## agents list

Listar Agents

```text
kobana inbox agents list [OPCIONES]
```

`GET /v1/agents`

Paginación keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`); los filtros desconocidos, los campos no ordenables y los valores malformados se rechazan con 400 `VALIDATION_ERROR`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `updated_at`, `label`, `version`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `updated_at`, `label`, `version`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `llm_model` | `string` | no | Igualdad. También `llm_model[in]=a,b`. |
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

## agents create

Crear Agent

```text
kobana inbox agents create [OPCIONES]
```

`POST /v1/agents`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `llm_model` | `string` | sí |  |
| `system_message` | `string` | sí |  |
| `external_id` | `string` | no | Opcional. Por defecto usa el `id` interno generado. |
| `skills_repo` | `string (uri)` | no |  |
| `workflow_allowed_hosts` | `array<string>` | no | Hosts (exactos, p. ej. `api.example.com`) a los que un workflow devuelto por este agente puede enviar `${{secrets.*}}`, `${{email.attachments.*}}` o la salida de un paso derivada de ellos. Vacío (el valor por defecto) bloquea todos los destinos para esos valores; los pasos sin ellos no se ven afectados. |
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. Envía `null` para limpiarlos. Máx. 64 KB serializados, 16 niveles de profundidad. |

Otros campos y objetos anidados: `kobana schema inbox.agents.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## agents update

Crear o actualizar agente

```text
kobana inbox agents update [OPCIONES]
```

`PUT /v1/agents`

Upsert. Recibe el mismo cuerpo que POST más las claves de búsqueda `id` y `external_id`, comparadas en ese orden. Una coincidencia se actualiza in situ y devuelve 200 — los campos ausentes del cuerpo conservan el valor almacenado. Sin clave, o sin coincidencia, crea el agente exactamente como lo haría POST y devuelve 201.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `llm_model` | `string` | sí |  |
| `system_message` | `string` | sí |  |
| `external_id` | `string` | no | Coincide por el id del sistema de origen. También se guarda en el registro cuando se crea. |
| `skills_repo` | `string (uri)` | no |  |
| `workflow_allowed_hosts` | `array<string>` | no | Hosts (exactos, p. ej. `api.example.com`) a los que un workflow devuelto por este agente puede enviar `${{secrets.*}}`, `${{email.attachments.*}}` o la salida de un paso derivada de ellos. Vacío (el valor por defecto) bloquea todos los destinos para esos valores; los pasos sin ellos no se ven afectados. |
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. Envía `null` para limpiarlos. Máx. 64 KB serializados, 16 niveles de profundidad. |
| `id` | `string (uuid)` | no | Coincide por el id interno. Se comprueba antes que `external_id`. |

Otros campos y objetos anidados: `kobana schema inbox.agents.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## agents get

Obtener Agent

```text
kobana inbox agents get [ID] [OPCIONES]
```

`GET /v1/agents/{id}`

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

## agents delete

Eliminar Agent

```text
kobana inbox agents delete [ID] [OPCIONES]
```

`DELETE /v1/agents/{id}`

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

## agents versions

Listar versiones del prompt del agente

```text
kobana inbox agents versions [ID] [OPCIONES]
```

`GET /v1/agents/{id}/versions`

Paginación keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`); los filtros desconocidos, los campos no ordenables y los valores malformados se rechazan con 400 `VALIDATION_ERROR`.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `version`. Sin él, el orden es `version` descendente. Valores: `created_at`, `version`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `version` | `integer` | no | Igualdad. También `version[in]=a,b` y `version[gt\|gte\|lt\|lte]`. |
| `version[gte]` | `integer` | no |  |
| `version[lte]` | `integer` | no |  |
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
