---
id: operation-approvals
title: "kobana billing operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana billing operation-approvals


Comandos de `kobana billing operation-approvals`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation_approvals` | Listar tus aprobaciones de operación |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation_approvals/{id}` | Consultar una aprobación de operación |

## operation-approvals list

Listar tus aprobaciones de operación

```text
kobana billing operation-approvals list [OPCIONES]
```

`GET /v1/operation_approvals`

Las aprobaciones por enlace de la persona dueña del token, en la organización del token, de la más reciente a la más antigua: cada tool destructiva del MCP que esperó la confirmación en la pantalla de aprobación, con lo que pasó después. Cualquier token con persona consulta sus propias aprobaciones, sin scope de recurso; un token sin persona recibe la lista vacía. Lo que devolvió cada operación (`result`, `error`, el `amount` leído del registro) solo viene cuando el token lee el recurso de esa aprobación y la persona aún tiene permiso de lectura sobre él; si no, viene `null` con `result_hidden: true`. Solo lectura: nada aquí aprueba, cancela ni reabre una aprobación. El `title` viene en el idioma del `Accept-Language` (pt-BR por defecto).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `status` | `string` | no | Solo aprobaciones en este estado. `pending` deja fuera la pendiente vencida, que es `expired`. Valores: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | no | Solo aprobaciones pedidas a partir de este instante (ISO 8601). |
| `created_before` | `string` | no | Solo aprobaciones pedidas hasta este instante (ISO 8601). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de aprobaciones. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## operation-approvals get

Consultar una aprobación de operación

```text
kobana billing operation-approvals get [ID] [OPCIONES]
```

`GET /v1/operation_approvals/{id}`

Una aprobación de la persona dueña del token, en la organización del token — el `approval.id` que la tool del MCP devolvió con `approval_required`. La aprobación de otra persona, de otra organización o un id inexistente responden el mismo 404. Nunca devuelve el enlace, el token del enlace, los argumentos ni la captura de la pantalla.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La aprobación. |
| `404` | Recurso no encontrado en la organización actual. |
