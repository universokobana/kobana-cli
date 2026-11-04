---
id: todos
title: "kobana kadu todos"
sidebar_label: "todos"
---

# kobana kadu todos


Comandos de `kobana kadu todos`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`todos update`](#todos-update) | `PATCH /v1/todos/{id}` | Resuelve pendiente |

## todos update

Resuelve pendiente

```text
kobana kadu todos update [ID] [OPCIONES]
```

`PATCH /v1/todos/{id}`

La única transición por la API: `status: resolved`.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Valores: `resolved`. |

Otros campos y objetos anidados: `kobana schema kadu.todos.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
