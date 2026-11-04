---
id: events
title: "kobana dashboards events"
sidebar_label: "events"
---

# kobana dashboards events


Comandos de `kobana dashboards events`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`events list`](#events-list) | `GET /v1/events` | Listar eventos |
| [`events get`](#events-get) | `GET /v1/events/{id}` | Consultar evento |

## events list

Listar eventos

```text
kobana dashboards events list [OPCIONES]
```

`GET /v1/events`

Log de eventos del workspace y de la organización, del más reciente al más antiguo. Los eventos son evidencia: no se pueden cambiar ni eliminar.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `type` | `string` | no | Tipo exacto del evento (ej.: `api_key.rotated`). |
| `resource_type` | `string` | no | Tipo exacto del recurso (ej.: `api_key`). |
| `resource_id` | `string` | no | Solo eventos de este registro. |
| `date_from` | `string` | no | Límite inferior de `created_at` (fecha o fecha-hora ISO 8601). |
| `date_to` | `string` | no | Límite superior de `created_at` (fecha o fecha-hora ISO 8601). Una fecha sin hora cubre el día entero (UTC). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de eventos. |
| `400` | Solicitud inválida: cuerpo malformado, campo obligatorio ausente o filtro con valor inválido. |
| `404` | Recurso no encontrado en la organización actual. |

## events get

Consultar evento

```text
kobana dashboards events get [ID] [OPCIONES]
```

`GET /v1/events/{id}`

Devuelve un evento con el payload completo.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El evento. |
| `404` | Recurso no encontrado en la organización actual. |
