---
id: comments
title: "kobana dashboards comments"
sidebar_label: "comments"
---

# kobana dashboards comments


Comandos de `kobana dashboards comments`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`comments list`](#comments-list) | `GET /v1/comments` | Listar comentarios |
| [`comments create`](#comments-create) | `POST /v1/comments` | Crear comentario |
| [`comments get`](#comments-get) | `GET /v1/comments/{id}` | Obtener comentario |
| [`comments delete`](#comments-delete) | `DELETE /v1/comments/{id}` | Eliminar comentario |

## comments list

Listar comentarios

```text
kobana dashboards comments list [OPCIONES]
```

`GET /v1/comments`

Los comentarios de la conversación de un tenant (`tenant_id` o `portal_user_id`) o, sin ninguno de los dos, de todas las conversaciones del workspace. Más recientes primero, paginado.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal dueños de la conversación — todos los usuarios del mismo tenant ven los mismos comentarios. Indique este o `portal_user_id`. |
| `portal_user_id` | `string` | no | Un usuario del portal (id o external_id): vale la conversación de su tenant (un usuario sin tenant tiene una conversación propia). Indique este o `tenant_id`. |
| `internal` | `string` | no | `true` = solo notas internas; `false` = solo lo que muestra el portal. Valores: `true`, `false`. |
| `author_type` | `string` | no | Solo comentarios escritos por `portal_user` (usuario del portal), `member` (miembro del equipo) o `api`. Valores: `portal_user`, `member`, `api`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comentarios. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## comments create

Crear comentario

```text
kobana dashboards comments create [OPCIONES]
```

`POST /v1/comments`

Agrega un comentario a la conversación de un tenant, en nombre del equipo. Todos los usuarios del portal de ese tenant lo ven en el bloque Comentarios — a menos que `internal` sea `true`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `body` | `string` | sí | El mensaje (hasta 5000 caracteres). |
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal dueños de la conversación — todos los usuarios del mismo tenant ven los mismos comentarios. Indique este o `portal_user_id`. |
| `portal_user_id` | `string` | no | Un usuario del portal (id o external_id): vale la conversación de su tenant (un usuario sin tenant tiene una conversación propia). Indique este o `tenant_id`. |
| `internal` | `boolean` | no | Nota interna: solo el equipo la ve (dashboard, API, MCP); el portal nunca la muestra. |
| `author_name` | `string` | no | Nombre mostrado junto al mensaje. Al crear, por defecto es el e-mail del operador del token o "Equipe". |

Otros campos y objetos anidados: `kobana schema dashboards.comments.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Comentario creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## comments get

Obtener comentario

```text
kobana dashboards comments get [ID] [OPCIONES]
```

`GET /v1/comments/{id}`

Un comentario.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El comentario. |
| `404` | Recurso no encontrado en la organización actual. |

## comments delete

Eliminar comentario

```text
kobana dashboards comments delete [ID] [OPCIONES]
```

`DELETE /v1/comments/{id}`

Quita el comentario de la conversación (eliminación lógica): desaparece del portal y del dashboard para todos.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comentario eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
