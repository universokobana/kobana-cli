---
id: asset-types
title: "kobana dashboards asset-types"
sidebar_label: "asset-types"
---

# kobana dashboards asset-types


Comandos de `kobana dashboards asset-types`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`asset-types list`](#asset-types-list) | `GET /v1/asset_types` | Listar tipos de archivo |
| [`asset-types create`](#asset-types-create) | `POST /v1/asset_types` | Crear tipo de archivo |
| [`asset-types get`](#asset-types-get) | `GET /v1/asset_types/{id}` | Obtener tipo de archivo |
| [`asset-types update`](#asset-types-update) | `PATCH /v1/asset_types/{id}` | Renombrar tipo de archivo |
| [`asset-types delete`](#asset-types-delete) | `DELETE /v1/asset_types/{id}` | Eliminar tipo de archivo |

## asset-types list

Listar tipos de archivo

```text
kobana dashboards asset-types list [OPCIONES]
```

`GET /v1/asset_types`

Los tipos de archivo del workspace, por nombre. Un bloque Archivos del Usuario del portal lista los archivos de los tipos que elige.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Los tipos de archivo. |
| `404` | Recurso no encontrado en la organización actual. |

## asset-types create

Crear tipo de archivo

```text
kobana dashboards asset-types create [OPCIONES]
```

`POST /v1/asset_types`

Crea un tipo de archivo. Después, use el `id` en el `type_id` de los archivos.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del tipo (Propuestas, Contratos…), único en el workspace sin distinguir mayúsculas; hasta 100 caracteres. |

Otros campos y objetos anidados: `kobana schema dashboards.asset-types.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | El tipo creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## asset-types get

Obtener tipo de archivo

```text
kobana dashboards asset-types get [ID] [OPCIONES]
```

`GET /v1/asset_types/{id}`

Un tipo de archivo.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El tipo de archivo. |
| `404` | Recurso no encontrado en la organización actual. |

## asset-types update

Renombrar tipo de archivo

```text
kobana dashboards asset-types update [ID] [OPCIONES]
```

`PATCH /v1/asset_types/{id}`

Cambia el nombre de un tipo de archivo.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del tipo (Propuestas, Contratos…), único en el workspace sin distinguir mayúsculas; hasta 100 caracteres. |

Otros campos y objetos anidados: `kobana schema dashboards.asset-types.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El tipo actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## asset-types delete

Eliminar tipo de archivo

```text
kobana dashboards asset-types delete [ID] [OPCIONES]
```

`DELETE /v1/asset_types/{id}`

Elimina el tipo. Sus archivos siguen, sin tipo — y salen de los bloques Archivos del Usuario que listaban ese tipo.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El tipo fue eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
