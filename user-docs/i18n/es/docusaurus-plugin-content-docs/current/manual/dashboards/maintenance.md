---
id: maintenance
title: "kobana dashboards maintenance"
sidebar_label: "maintenance"
---

# kobana dashboards maintenance


Comandos de `kobana dashboards maintenance`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`maintenance uppercase-names create`](#maintenance-uppercase-names-create) | `POST /v1/maintenance/uppercase_names` | Poner nombres en mayúsculas |

## maintenance uppercase-names create

Poner nombres en mayúsculas

```text
kobana dashboards maintenance uppercase-names create [OPCIONES]
```

`POST /v1/maintenance/uppercase_names`

Convierte a mayúsculas el nombre de todo contacto vinculado a un usuario del portal del workspace. No es reversible (la grafía anterior no se guarda).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuántos nombres cambiaron. |
| `404` | Recurso no encontrado en la organización actual. |
