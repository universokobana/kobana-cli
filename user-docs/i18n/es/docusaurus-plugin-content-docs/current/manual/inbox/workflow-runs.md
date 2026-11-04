---
id: workflow-runs
title: "kobana inbox workflow-runs"
sidebar_label: "workflow-runs"
---

# kobana inbox workflow-runs


Comandos de `kobana inbox workflow-runs`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`workflow-runs list`](#workflow-runs-list) | `GET /v1/workflow-runs` | Listar ejecuciones de workflow |
| [`workflow-runs get`](#workflow-runs-get) | `GET /v1/workflow-runs/{id}` | Obtener ejecución de workflow |

## workflow-runs list

Listar ejecuciones de workflow

```text
kobana inbox workflow-runs list [OPCIONES]
```

`GET /v1/workflow-runs`

Paginación por keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`). Los pasos no vienen incluidos en el listado — `GET /v1/workflow-runs/{id}` los devuelve.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `updated_at`, `status`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `updated_at`, `status`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Igualdad. También `status[in]=a,b`. Valores: `pending`, `running`, `success`, `failed`. |
| `email_id` | `string` | no | Igualdad. También `email_id[in]=a,b`. |
| `inbox_id` | `string` | no | Igualdad. También `inbox_id[in]=a,b`. |
| `agent_id` | `string` | no | Igualdad. También `agent_id[in]=a,b`. |
| `agent_run_id` | `string` | no | Equality. Also `agent_run_id[in]=a,b`. |
| `created_at` | `string` | no | Igualdad. También `created_at[in]=a,b` y `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `finished_at` | `string` | no | Igualdad. También `finished_at[in]=a,b` y `finished_at[gt\|gte\|lt\|lte]`. |
| `finished_at[gte]` | `string` | no |  |
| `finished_at[lte]` | `string` | no |  |

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

## workflow-runs get

Obtener ejecución de workflow

```text
kobana inbox workflow-runs get [ID] [OPCIONES]
```

`GET /v1/workflow-runs/{id}`

La ejecución con todos sus pasos en orden. Peticiones y respuestas se guardan con los valores de secrets reemplazados por `[REDACTED]`; los cuerpos se truncan a 4000 caracteres.

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
