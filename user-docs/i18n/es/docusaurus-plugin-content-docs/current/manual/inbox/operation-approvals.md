---
id: operation-approvals
title: "kobana inbox operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana inbox operation-approvals


Comandos de `kobana inbox operation-approvals`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation-approvals` | Listar aprobaciones de operación |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation-approvals/{id}` | Consultar aprobación de operación |

## operation-approvals list

Listar aprobaciones de operación

```text
kobana inbox operation-approvals list [OPCIONES]
```

`GET /v1/operation-approvals`

Las aprobaciones de operación que pidió una tool MCP destructiva, de la más reciente a la más antigua, paginadas por página. Solo las aprobaciones de quien llama — la persona del token — en la organización del token y en el workspace de la llamada. No exige alcance de recurso, como `/v1/me`: el dato es de la propia persona. Un token sin persona no tiene ninguna. El enlace de la aprobación, su token y la tarjeta guardada nunca se devuelven.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Solo las aprobaciones en este estado; una pendiente vencida cuenta como `expired`. Valores: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | no | Pedidas en este instante ISO 8601 o después. |
| `created_before` | `string` | no | Pedidas en este instante ISO 8601 o antes. |
| `page` | `integer` | no | Número de página, por defecto 1. |
| `per_page` | `integer` | no | Tamaño de página, por defecto 25, máximo 100. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de aprobaciones. |
| `400` | Un parámetro de consulta no es válido. |

## operation-approvals get

Consultar aprobación de operación

```text
kobana inbox operation-approvals get [ID] [OPCIONES]
```

`GET /v1/operation-approvals/{id}`

Una aprobación de operación: estado, título, fechas y resultado. La aprobación de otra persona, de otra organización o un id inexistente responden el mismo `404 NOT_FOUND`. Solo las aprobaciones de quien llama — la persona del token — en la organización del token y en el workspace de la llamada. No exige alcance de recurso, como `/v1/me`: el dato es de la propia persona. Un token sin persona no tiene ninguna. El enlace de la aprobación, su token y la tarjeta guardada nunca se devuelven.

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
