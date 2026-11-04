---
id: settings
title: "kobana dashboards settings"
sidebar_label: "settings"
description: "Consultar configuración de la organización"
---

# kobana dashboards settings

Consultar configuración de la organización

Comandos de `kobana dashboards settings`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`settings`](#settings) | `GET /v1/settings` | Consultar configuración de la organización |
| [`settings update`](#settings-update) | `PUT /v1/settings` | Actualizar configuración de la organización |

## settings

Consultar configuración de la organización

```text
kobana dashboards settings [OPCIONES] [COMANDO]
```

`GET /v1/settings`

Nombre, correo de soporte, zona horaria y las opciones de registros de la organización. Recurso de la organización.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La configuración. |
| `404` | Recurso no encontrado en la organización actual. |

## settings update

Actualizar configuración de la organización

```text
kobana dashboards settings update [OPCIONES]
```

`PUT /v1/settings`

Cambia parcialmente la configuración: envíe solo lo que cambia. Los campos desconocidos se rechazan (400).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no |  |
| `support_email` | `string (email)` | no |  |
| `timezone` | `string` | no | Zona horaria IANA, p. ej. `America/Sao_Paulo`. |
| `crm_uppercase_names` | `boolean` | no | Guardar nombres en MAYÚSCULAS. |
| `crm_allow_edit_public_id` | `boolean` | no |  |
| `crm_allow_edit_document` | `boolean` | no |  |
| `crm_allow_create_customers` | `boolean` | no |  |
| `crm_allow_create_portal_users` | `boolean` | no | Permitir crear usuarios del portal desde el panel y por importación. |
| `crm_allow_edit_portal_user_email` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.settings.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La configuración. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
