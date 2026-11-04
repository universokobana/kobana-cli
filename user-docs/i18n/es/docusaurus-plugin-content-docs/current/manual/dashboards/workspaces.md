---
id: workspaces
title: "kobana dashboards workspaces"
sidebar_label: "workspaces"
---

# kobana dashboards workspaces


Comandos de `kobana dashboards workspaces`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | Listar workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Crear workspace |
| [`workspaces import`](#workspaces-import) | `POST /v1/workspaces/import` | Importar workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Consultar workspace |
| [`workspaces update`](#workspaces-update) | `PATCH /v1/workspaces/{id}` | Renombrar workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Eliminar workspace |
| [`workspaces export`](#workspaces-export) | `GET /v1/workspaces/{id}/export` | Exportar workspace |
| [`workspaces usage`](#workspaces-usage) | `GET /v1/workspaces/{id}/usage` | Contar lo que ocultaría eliminar el workspace |

## workspaces list

Listar workspaces

```text
kobana dashboards workspaces list [OPCIONES]
```

`GET /v1/workspaces`

Lista los workspaces de la organización que usted puede abrir, con su rol en cada uno — los mismos del selector del dashboard. Un token de máquina ve todos los workspaces de la organización. No requiere `X-Workspace-Id`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | no | Fragmento del nombre del workspace, sin distinguir mayúsculas. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de workspaces. |
| `404` | Recurso no encontrado en la organización actual. |

## workspaces create

Crear workspace

```text
kobana dashboards workspaces create [OPCIONES]
```

`POST /v1/workspaces`

Crea un workspace vacío en la organización. La persona del token pasa a ser su dueña. Creado por un token de máquina, el workspace queda sin dueño: los dueños de la organización lo reciben en su próximo inicio de sesión, y `PUT /dashboard_users/{id}/workspaces` lo concede a otros miembros.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del workspace, de 1 a 100 caracteres. |

Otros campos y objetos anidados: `kobana schema dashboards.workspaces.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Workspace creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## workspaces import

Importar workspace

```text
kobana dashboards workspaces import [OPCIONES]
```

`POST /v1/workspaces/import`

Restaura un archivo de `GET /workspaces/{id}/export` en un workspace NUEVO, cuyo dueño es la persona del token. Responde lo que no se pudo traer de vuelta (`warnings`) para terminarlo a mano. Requiere un token de persona (un token de máquina recibe 403) con los permisos de crear workspaces, configurar el portal y crear fuentes de datos. Las entidades del archivo pasan por las mismas validaciones que al crearlas una a una (claves, expresiones, límite de campos del workspace). El cuerpo está limitado a 2 MB.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `data` | `object` | sí | El archivo exportado, tal como lo devuelve la exportación. |
| `name` | `string` | no | Nombre del nuevo workspace; por defecto, el del archivo. |

Otros campos y objetos anidados: `kobana schema dashboards.workspaces.import`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Workspace importado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El archivo no es una exportación de workspace válida (`INVALID_WORKSPACE_FILE`). |

## workspaces get

Consultar workspace

```text
kobana dashboards workspaces get [ID] [OPCIONES]
```

`GET /v1/workspaces/{id}`

Devuelve un workspace que usted puede abrir. Un workspace sin acceso concedido responde 404.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Workspace encontrado. |
| `404` | Recurso no encontrado en la organización actual. |

## workspaces update

Renombrar workspace

```text
kobana dashboards workspaces update [ID] [OPCIONES]
```

`PATCH /v1/workspaces/{id}`

Renombra el workspace. Solo los dueños del workspace pueden (403 para un miembro común).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del workspace, de 1 a 100 caracteres. |

Otros campos y objetos anidados: `kobana schema dashboards.workspaces.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Workspace renombrado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## workspaces delete

Eliminar workspace

```text
kobana dashboards workspaces delete [ID] [OPCIONES]
```

`DELETE /v1/workspaces/{id}`

Elimina el workspace (eliminación lógica), solo para sus dueños: el portal deja de estar disponible, los conectores dejan de sincronizar y los miembros pierden el acceso. El último workspace de la organización no puede eliminarse.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Workspace eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Es el último workspace de la organización (`WORKSPACE_LAST`). |

## workspaces export

Exportar workspace

```text
kobana dashboards workspaces export [ID] [OPCIONES]
```

`GET /v1/workspaces/{id}/export`

Descarga la configuración del workspace (constructor del portal, acceso y ajustes del portal, mapeo de usuarios, conectores y entidades) como un archivo JSON, solo para sus dueños. Los secretos y los datos quedan fuera. La respuesta es el propio archivo, sin envoltorio `data`; envíelo como `data` a `POST /workspaces/import`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivo de exportación (`Content-Disposition: attachment`). |
| `404` | Recurso no encontrado en la organización actual. |

## workspaces usage

Contar lo que ocultaría eliminar el workspace

```text
kobana dashboards workspaces usage [ID] [OPCIONES]
```

`GET /v1/workspaces/{id}/usage`

Cuenta lo que la eliminación del workspace ocultaría — el mismo resumen del diálogo de eliminación del dashboard.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Conteos del workspace. |
| `404` | Recurso no encontrado en la organización actual. |
