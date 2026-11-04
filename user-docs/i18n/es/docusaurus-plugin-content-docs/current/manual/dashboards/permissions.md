---
id: permissions
title: "kobana dashboards permissions"
sidebar_label: "permissions"
description: "Listar permisos"
---

# kobana dashboards permissions

Listar permisos

Comandos de `kobana dashboards permissions`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`permissions`](#permissions) | `GET /v1/permissions` | Listar permisos |

## permissions

Listar permisos

```text
kobana dashboards permissions [OPCIONES]
```

`GET /v1/permissions`

Catálogo de los permisos del dashboard que un miembro o un rol del equipo puede tener, agrupados por recurso — el vocabulario de `permissions` en `/dashboard_users` y `/team_roles`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Grupos de permisos. |
| `404` | Recurso no encontrado en la organización actual. |
