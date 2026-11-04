---
id: data-purges
title: "kobana dashboards data-purges"
sidebar_label: "data-purges"
---

# kobana dashboards data-purges


Comandos de `kobana dashboards data-purges`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`data-purges list`](#data-purges-list) | `GET /v1/data_purges` | Contar datos eliminables |
| [`data-purges delete`](#data-purges-delete) | `DELETE /v1/data_purges` | Eliminar datos de la organización |
| [`data-purges get`](#data-purges-get) | `GET /v1/data_purges/{id}` | Seguir limpieza |

## data-purges list

Contar datos eliminables

```text
kobana dashboards data-purges list [OPCIONES]
```

`GET /v1/data_purges`

Cuántos registros tiene cada recurso eliminable (`portal_users`, `people`, `usage_history`) sumando todos los workspaces. Exige `data.purge` (por defecto solo dueños de la organización) y acceso a todos los workspaces.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Los conteos. |
| `404` | Recurso no encontrado en la organización actual. |

## data-purges delete

Eliminar datos de la organización

```text
kobana dashboards data-purges delete [OPCIONES]
```

`DELETE /v1/data_purges`

IRREVERSIBLE: elimina definitivamente los recursos elegidos en TODOS los workspaces de la organización. En el panel la operación se confirma con un código enviado por correo. Por eso, un token que identifica a una persona (token personal, OAuth o llamada por el MCP) recibe 403: la persona hace la limpieza desde el panel. Solo los tokens de máquina pueden limpiar por la API, y solo con el verbo `delete` (o `all`) — `write` no basta; con scopes de recurso, el token también debe alcanzar `dashboards/admin` (o `dashboards/all`); sin eso, 403 `insufficient_scope`. Su confirmación es `confirm`, que debe repetir el ID de la organización (de `GET /organization`) — una llamada repetida o dirigida a la organización equivocada no pasa por error. Exige `data.purge` (por defecto solo dueños de la organización) y acceso a todos los workspaces. Asíncrono: 202 con `job_id`. Es un `DELETE` con cuerpo JSON (`resources`, `confirm`) — hasta 2026-10-29 era `POST`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `resources` | `array<string>` | sí | Qué eliminar: `portal_users`, `people`, `usage_history`. |
| `confirm` | `string` | sí | Confirmación: el ID de la organización, escrito de nuevo. |

Otros campos y objetos anidados: `kobana schema dashboards.data-purges.delete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Limpieza programada; sígala en `GET /data_purges/{id}`. |
| `400` | Cuerpo ausente, JSON inválido o campos incompatibles. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | `confirm` no coincide con el ID de la organización — no se eliminó nada. |

## data-purges get

Seguir limpieza

```text
kobana dashboards data-purges get [ID] [OPCIONES]
```

`GET /v1/data_purges/{id}`

Progreso y resultado de un job de limpieza. Job de otra organización: 404.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El job. |
| `404` | Recurso no encontrado en la organización actual. |
