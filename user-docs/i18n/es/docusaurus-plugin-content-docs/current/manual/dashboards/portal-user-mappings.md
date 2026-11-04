---
id: portal-user-mappings
title: "kobana dashboards portal-user-mappings"
sidebar_label: "portal-user-mappings"
description: "Consultar mapeo de usuarios del portal"
---

# kobana dashboards portal-user-mappings

Consultar mapeo de usuarios del portal

Comandos de `kobana dashboards portal-user-mappings`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`portal-user-mappings`](#portal-user-mappings) | `GET /v1/portal_user_mappings` | Consultar mapeo de usuarios del portal |
| [`portal-user-mappings update`](#portal-user-mappings-update) | `PUT /v1/portal_user_mappings` | Mapear entidad a usuarios del portal |
| [`portal-user-mappings delete`](#portal-user-mappings-delete) | `DELETE /v1/portal_user_mappings` | Eliminar mapeo de usuarios del portal |

## portal-user-mappings

Consultar mapeo de usuarios del portal

```text
kobana dashboards portal-user-mappings [OPCIONES] [COMANDO]
```

`GET /v1/portal_user_mappings`

La entidad que alimenta a los usuarios del portal del workspace (`mapping`, `null` si no hay, con el resultado de la última sincronización) y las entidades que pueden mapearse, con sus campos.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El mapeo. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-user-mappings update

Mapear entidad a usuarios del portal

```text
kobana dashboards portal-user-mappings update [OPCIONES]
```

`PUT /v1/portal_user_mappings`

Mapea una entidad: un usuario del portal por correo de sus registros, con el ID del titular tomado de un campo. Reemplaza el mapeo anterior y sincroniza en el acto (devuelve `stats`); las próximas sincronizaciones de la entidad mantienen a los usuarios al día.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `entity_type_id` | `string (uuid)` | sí | La entidad cuyos registros se convierten en usuarios del portal. |
| `tenant_id_field_key` | `string` | sí |  |
| `email_field_key` | `string` | sí |  |
| `name_field_key` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.portal-user-mappings.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El mapeo guardado y el resultado de la sincronización. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Entidad o campo inválido (p. ej., entidad que sincroniza al acceder). |

## portal-user-mappings delete

Eliminar mapeo de usuarios del portal

```text
kobana dashboards portal-user-mappings delete [OPCIONES]
```

`DELETE /v1/portal_user_mappings`

Deja de sincronizar; los usuarios ya creados permanecen.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Mapeo eliminado (`deleted: false` si no había). |
| `404` | Recurso no encontrado en la organización actual. |
