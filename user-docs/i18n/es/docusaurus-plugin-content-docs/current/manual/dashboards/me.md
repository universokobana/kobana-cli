---
id: me
title: "kobana dashboards me"
sidebar_label: "me"
description: "Identidad del token actual"
---

# kobana dashboards me

Identidad del token actual

Comandos de `kobana dashboards me`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidad del token actual |

## me

Identidad del token actual

```text
kobana dashboards me [OPCIONES]
```

`GET /v1/me`

Devuelve quién está detrás del token Bearer: la persona (`user`, `null` para un token de máquina), la organización y los scopes, el nivel de acceso y la expiración del propio token. Sirve para validar la credencial configurada en una integración. Nunca devuelve el token, hashes ni claves.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Identidad del token autenticado. |
| `404` | Recurso no encontrado en la organización actual. |
