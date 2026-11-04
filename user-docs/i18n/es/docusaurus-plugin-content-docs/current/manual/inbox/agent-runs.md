---
id: agent-runs
title: "kobana inbox agent-runs"
sidebar_label: "agent-runs"
---

# kobana inbox agent-runs


Comandos de `kobana inbox agent-runs`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`agent-runs list`](#agent-runs-list) | `GET /v1/agent-runs` | Listar AgentRuns |
| [`agent-runs get`](#agent-runs-get) | `GET /v1/agent-runs/{id}` | Obtener AgentRun |

## agent-runs list

Listar AgentRuns

```text
kobana inbox agent-runs list [OPCIONES]
```

`GET /v1/agent-runs`

Paginación keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`); los filtros desconocidos, los campos no ordenables y los valores malformados se rechazan con 400 `VALIDATION_ERROR`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `updated_at`, `total_cost`, `status`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `updated_at`, `total_cost`, `status`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Igualdad. También `status[in]=a,b`. Valores: `running`, `success`, `failed`. |
| `agent_id` | `string` | no | Igualdad. También `agent_id[in]=a,b`. |
| `email_id` | `string` | no | Igualdad. También `email_id[in]=a,b`. |
| `inbox_id` | `string` | no | Igualdad. También `inbox_id[in]=a,b`. |
| `llm_model` | `string` | no | Igualdad. También `llm_model[in]=a,b`. |
| `created_at` | `string` | no | Igualdad. También `created_at[in]=a,b` y `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `finished_at` | `string` | no | Igualdad. También `finished_at[in]=a,b` y `finished_at[gt\|gte\|lt\|lte]`. |
| `finished_at[gte]` | `string` | no |  |
| `finished_at[lte]` | `string` | no |  |
| `total_cost` | `number` | no | Igualdad. También `total_cost[in]=a,b` y `total_cost[gt\|gte\|lt\|lte]`. |
| `total_cost[gte]` | `number` | no |  |
| `total_cost[lte]` | `number` | no |  |

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

## agent-runs get

Obtener AgentRun

```text
kobana inbox agent-runs get [ID] [OPCIONES]
```

`GET /v1/agent-runs/{id}`

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
