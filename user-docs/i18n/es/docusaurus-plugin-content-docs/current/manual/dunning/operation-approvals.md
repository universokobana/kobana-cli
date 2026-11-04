---
id: operation-approvals
title: "kobana dunning operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana dunning operation-approvals


Comandos de `kobana dunning operation-approvals`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation-approvals` | Listar mis aprobaciones |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation-approvals/{id}` | Consultar una aprobación |

## operation-approvals list

Listar mis aprobaciones

```text
kobana dunning operation-approvals list [OPCIONES]
```

`GET /v1/operation-approvals`

Lista las aprobaciones por enlace de la persona del token, en la organización del token y el workspace de la llamada, de la más reciente a la más antigua. Solo lectura: nada aquí aprueba, cancela ni reabre una aprobación. No exige permiso de rol ni alcance de recurso propio — el dato es de la propia persona; basta el nivel de lectura.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `status` | `string` | no | Solo esta situación. `expired` incluye la pendiente vencida; `pending` solo la que aún vale. Valores: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | no | Pedidas a partir de este instante (ISO 8601, inclusive). |
| `created_before` | `string` | no | Pedidas antes de este instante (ISO 8601, exclusivo). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Aprobaciones de la persona. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## operation-approvals get

Consultar una aprobación

```text
kobana dunning operation-approvals get [ID] [OPCIONES]
```

`GET /v1/operation-approvals/{id}`

Situación y resultado de una aprobación por enlace, por el `approval.id` que la tool destructiva devolvió en `approval_required`. Llámela después de que la persona diga que decidió en la pantalla — no en bucle. Aprobación de otra persona, de otra organización o id inexistente: el mismo 404. El `result` sale con los secretos enmascarados.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La aprobación. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
