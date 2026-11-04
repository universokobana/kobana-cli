---
id: builder
title: "kobana dashboards builder"
sidebar_label: "builder"
description: "Consultar el constructor del portal"
---

# kobana dashboards builder

Consultar el constructor del portal

Comandos de `kobana dashboards builder`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`builder`](#builder) | `GET /v1/builder` | Consultar el constructor del portal |
| [`builder draft update`](#builder-draft-update) | `PUT /v1/builder/draft` | Guardar el borrador del portal |
| [`builder theme update`](#builder-theme-update) | `PATCH /v1/builder/theme` | Modificar los colores del tema del portal |
| [`builder tools create-page`](#builder-tools-create-page) | `POST /v1/builder/tools/create_page` | Crear una página |
| [`builder tools delete-page`](#builder-tools-delete-page) | `POST /v1/builder/tools/delete_page` | Eliminar una página |
| [`builder tools describe-block-type`](#builder-tools-describe-block-type) | `GET /v1/builder/tools/describe_block_type` | Describir un tipo de bloque |
| [`builder tools duplicate-block`](#builder-tools-duplicate-block) | `POST /v1/builder/tools/duplicate_block` | Duplicar un bloque |
| [`builder tools get-page`](#builder-tools-get-page) | `GET /v1/builder/tools/get_page` | Consultar una página del borrador |
| [`builder tools get-theme`](#builder-tools-get-theme) | `GET /v1/builder/tools/get_theme` | Consultar los colores del tema |
| [`builder tools insert-block`](#builder-tools-insert-block) | `POST /v1/builder/tools/insert_block` | Insertar un bloque |
| [`builder tools list-actions`](#builder-tools-list-actions) | `GET /v1/builder/tools/list_actions` | Listar las acciones del botón |
| [`builder tools list-block-types`](#builder-tools-list-block-types) | `GET /v1/builder/tools/list_block_types` | Listar los tipos de bloque |
| [`builder tools list-pages`](#builder-tools-list-pages) | `GET /v1/builder/tools/list_pages` | Listar las páginas del borrador |
| [`builder tools move-block`](#builder-tools-move-block) | `POST /v1/builder/tools/move_block` | Mover un bloque |
| [`builder tools move-page`](#builder-tools-move-page) | `POST /v1/builder/tools/move_page` | Mover una página en el menú |
| [`builder tools preview-portal-user-mapping`](#builder-tools-preview-portal-user-mapping) | `GET /v1/builder/tools/preview_portal_user_mapping` | Previsualizar el mapeo de usuarios del portal |
| [`builder tools read-asset`](#builder-tools-read-asset) | `GET /v1/builder/tools/read_asset` | Leer el contenido de un archivo |
| [`builder tools register-entity`](#builder-tools-register-entity) | `POST /v1/builder/tools/register_entity` | Registrar una entidad desde una fuente de datos |
| [`builder tools remove-block`](#builder-tools-remove-block) | `POST /v1/builder/tools/remove_block` | Quitar un bloque |
| [`builder tools rename-entity-fields`](#builder-tools-rename-entity-fields) | `POST /v1/builder/tools/rename_entity_fields` | Renombrar los campos de una entidad |
| [`builder tools save-computed-field`](#builder-tools-save-computed-field) | `POST /v1/builder/tools/save_computed_field` | Guardar un campo calculado |
| [`builder tools update-block`](#builder-tools-update-block) | `POST /v1/builder/tools/update_block` | Modificar un bloque |
| [`builder tools update-page`](#builder-tools-update-page) | `POST /v1/builder/tools/update_page` | Modificar una página |
| [`builder versions get`](#builder-versions-get) | `GET /v1/builder/versions/{number}` | Consultar una versión publicada |
| [`builder versions restore`](#builder-versions-restore) | `POST /v1/builder/versions/{number}/restore` | Restaurar una versión en el borrador |
| [`builder versions rollback`](#builder-versions-rollback) | `POST /v1/builder/versions/{number}/rollback` | Volver a poner una versión en el aire |
| [`builder encrypt-secret`](#builder-encrypt-secret) | `POST /v1/builder/encrypt_secret` | Cifrar un secreto de bloque |
| [`builder publish`](#builder-publish) | `POST /v1/builder/publish` | Publicar el portal |
| [`builder test-action`](#builder-test-action) | `POST /v1/builder/test_action` | Probar un botón de acción del borrador |
| [`builder test-form`](#builder-test-form) | `POST /v1/builder/test_form` | Probar un formulario del borrador |
| [`builder upload-og-image`](#builder-upload-og-image) | `POST /v1/builder/upload_og_image` | Subir la og:image del portal |

## builder

Consultar el constructor del portal

```text
kobana dashboards builder [OPCIONES] [COMANDO]
```

`GET /v1/builder`

Devuelve el portal del workspace como lo ve el constructor: identidad del portal, borrador (páginas en el orden del menú, cada una con su marca respecto a la versión publicada, `revision` y `dirty`), número de la versión publicada, versiones publicadas, tema y SEO predeterminado. Lea la `revision` aquí antes de guardar el borrador.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El constructor del portal. |
| `404` | Recurso no encontrado en la organización actual. |

## builder draft update

Guardar el borrador del portal

```text
kobana dashboards builder draft update [OPCIONES]
```

`PUT /v1/builder/draft`

Sustituye el borrador completo — páginas en el orden del menú, estado del editor y, opcionalmente, tema y SEO predeterminado — protegido por la última `revision` leída. Una `revision` desactualizada responde 409 y no cambia nada. Nada se publica: para publicar, use `POST /builder/publish`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `revision` | `integer` | sí | La última `revision` del borrador leída (`0` cuando aún no hay borrador). |
| `pages` | `array<object>` | sí | Todas las páginas del borrador, en el orden del menú (hasta 200). Las páginas omitidas salen del borrador. |
| `editor_state` | `object` | no | Estado del editor; devuelva lo que leyó. Omitido = `{}`. |
| `theme` | `object` | no |  |
| `seo_defaults` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.builder.draft.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Borrador guardado: nueva `revision`, si difiere de la versión publicada y la marca de cada página. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |

## builder theme update

Modificar los colores del tema del portal

```text
kobana dashboards builder theme update [OPCIONES]
```

`PATCH /v1/builder/theme`

Cambia colores del tema de las páginas del portal (la pestaña Visual del constructor) en el borrador, por modo, en hex. Solo cambian los colores enviados; el portal publicado cambia cuando se publique el borrador. Para leer los colores actuales, y para qué sirve cada uno, usa `GET /builder/tools/get_theme`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `light` | `object` | no |  |
| `dark` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.builder.theme.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Tema del borrador: nueva `revision`, si el tema cambió y cuántos colores se enviaron. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## builder tools create-page

Crear una página

```text
kobana dashboards builder tools create-page [OPCIONES]
```

`POST /v1/builder/tools/create_page`

Crea una página en el borrador y la agrega al menú del portal. template: blank (solo el título), list (título + KPIs + tabla de `entity`), detail (título + detalle del registro + documentos). El título es un bloque PageHeader: la respuesta lista los bloques creados — modifícalos con update_block en lugar de insertarlos de nuevo.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Menu label. |
| `path` | `string` | no | Route, e.g. /contratos. Omitted = derived from the name. |
| `title` | `string` | no | Browser title; omitted = the name. |
| `template` | `string` | no | Valores: `blank`, `list`, `detail`. |
| `entity` | `string` | no | Entity the list/detail template binds to. |
| `show_in_menu` | `boolean` | no | Listed in the portal menu (default true); false for a detail page reached by links. |
| `position` | `integer` | no | Place in the menu; omitted = last. |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.create-page`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools delete-page

Eliminar una página

```text
kobana dashboards builder tools delete-page [OPCIONES]
```

`POST /v1/builder/tools/delete_page`

Elimina una página (y sus bloques) del borrador y del menú. Solo cuando el operador lo pida.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page_id` | `string` | sí | Page id (list_pages). |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.delete-page`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools describe-block-type

Describir un tipo de bloque

```text
kobana dashboards builder tools describe-block-type [OPCIONES]
```

`GET /v1/builder/tools/describe_block_type`

Las props de un tipo de bloque: JSON Schema, valores por defecto y slots. Léelo antes de insertar o modificar un bloque que todavía no usaste.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `type` | `string` | sí | Block type, e.g. DataTable. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que leyó la herramienta, en `data`. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools duplicate-block

Duplicar un bloque

```text
kobana dashboards builder tools duplicate-block [OPCIONES]
```

`POST /v1/builder/tools/duplicate_block`

Copia un bloque (con los bloques dentro de él) justo después de sí mismo, con ids nuevos.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `block_id` | `string` | sí | Block id (get_page). |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.duplicate-block`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools get-page

Consultar una página del borrador

```text
kobana dashboards builder tools get-page [OPCIONES]
```

`GET /v1/builder/tools/get_page`

Una página del borrador: su configuración (nombre, ruta, título, SEO) y su árbol de bloques, cada bloque con id, tipo, props y slots.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page_id` | `string` | sí | Page id (list_pages). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que leyó la herramienta, en `data`. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools get-theme

Consultar los colores del tema

```text
kobana dashboards builder tools get-theme [OPCIONES]
```

`GET /v1/builder/tools/get_theme`

Los colores de las páginas del portal (la pestaña Visual del borrador), en hex, en los modos claro y oscuro: para qué sirve cada color y su valor actual.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que leyó la herramienta, en `data`. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools insert-block

Insertar un bloque

```text
kobana dashboards builder tools insert-block [OPCIONES]
```

`POST /v1/builder/tools/insert_block`

Inserta un bloque del catálogo en una página: en la raíz de la página o dentro de un slot de un bloque Columns/Tabs (parent_block_id + slot). Las props no enviadas usan el valor por defecto.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page_id` | `string` | sí | Page id (list_pages). |
| `type` | `string` | sí | Block type (list_block_types). |
| `props` | `object` | no | Block props (describe_block_type). Never include id or slot props. |
| `parent_block_id` | `string` | no | Columns or Tabs block that receives the new block. |
| `slot` | `string` | no | Slot of the parent: col1..col3 (Columns) or content (Tabs). Omitted = the first. |
| `position` | `integer` | no | 0-based index; omitted = at the end. |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.insert-block`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools list-actions

Listar las acciones del botón

```text
kobana dashboards builder tools list-actions [OPCIONES]
```

`GET /v1/builder/tools/list_actions`

Lo que un ActionButton puede hacer (su actionType): abrir un enlace (una página del portal o una URL) o disparar un webhook, y las \{\{variables\}\} que la URL, las cabeceras y el cuerpo del webhook pueden usar.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que leyó la herramienta, en `data`. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools list-block-types

Listar los tipos de bloque

```text
kobana dashboards builder tools list-block-types [OPCIONES]
```

`GET /v1/builder/tools/list_block_types`

Lista el catálogo de bloques: todo tipo de bloque que el portal sabe renderizar, con para qué sirve. Solo existen esos tipos.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que leyó la herramienta, en `data`. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools list-pages

Listar las páginas del borrador

```text
kobana dashboards builder tools list-pages [OPCIONES]
```

`GET /v1/builder/tools/list_pages`

Lista las páginas del borrador en el orden del menú: id, nombre, ruta, título y cantidad de bloques.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que leyó la herramienta, en `data`. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools move-block

Mover un bloque

```text
kobana dashboards builder tools move-block [OPCIONES]
```

`POST /v1/builder/tools/move_block`

Mueve un bloque dentro de su página: a otra posición en la raíz (parent_block_id null) o a un slot de un bloque Columns/Tabs.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `block_id` | `string` | sí | Block id (get_page). |
| `position` | `integer` | sí | 0-based index in the target. |
| `parent_block_id` | `string` | no | Columns/Tabs block to move into; null or omitted = the page root. |
| `slot` | `string` | no |  |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.move-block`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools move-page

Mover una página en el menú

```text
kobana dashboards builder tools move-page [OPCIONES]
```

`POST /v1/builder/tools/move_page`

Mueve una página a otro lugar del menú del portal.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page_id` | `string` | sí | Page id (list_pages). |
| `position` | `integer` | sí | 0-based index in the menu. |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.move-page`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools preview-portal-user-mapping

Previsualizar el mapeo de usuarios del portal

```text
kobana dashboards builder tools preview-portal-user-mapping [OPCIONES]
```

`GET /v1/builder/tools/preview_portal_user_mapping`

Comprueba un mapeo de usuarios del portal antes de guardarlo con `dashboards_portal_user_mappings_update`: la entidad cuyos registros se convertirían en los usuarios del portal, sus campos de ID del tenant, correo y nombre, cuántos usuarios serían y cómo los IDs de tenant coinciden con los titulares de los registros de las otras entidades. No guarda nada.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `entity` | `string` | sí | Entity key. |
| `tenant_id_field` | `string` | sí | The value the other entities' owners hold (the owner field of the entity, or its identifier). |
| `email_field` | `string` | sí | The field with the customer's e-mail. |
| `name_field` | `string` | no | The field with the customer's name, if any. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que leyó la herramienta, en `data`. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools read-asset

Leer el contenido de un archivo

```text
kobana dashboards builder tools read-asset [OPCIONES]
```

`GET /v1/builder/tools/read_asset`

Abre un archivo de la biblioteca de Archivos para ver su contenido: una imagen o un PDF vienen como imagen, un archivo de texto (txt, csv, json, md, xml) o una planilla .xlsx vienen como texto. Word/PowerPoint/zip no se pueden leer (solo sus detalles).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `asset_id` | `string` | sí | Asset id (list_assets, or the asset_id of a file sent in the chat). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que leyó la herramienta, en `data`. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools register-entity

Registrar una entidad desde una fuente de datos

```text
kobana dashboards builder tools register-entity [OPCIONES]
```

`POST /v1/builder/tools/register_entity`

Registra una entidad (los bloques de datos se vinculan a ella) a partir de una operación de una fuente de datos conectada (describe_data_source), con los campos inferidos de una muestra de sus registros, e inicia su primera sincronización. Úsalo cuando el operador quiere datos que ninguna entidad tiene todavía. Tiene efecto en el workspace de inmediato (no es parte del borrador); la nueva entidad ya puede usarse en bloques en la misma respuesta. El workspace limita cuántos campos guarda una entidad nueva: cuando la fuente tiene más, el rechazo lista todos los campos con un valor de ejemplo — llama de nuevo con `fields` con los que el portal necesita (lo que muestra, filtra o suma, y lo que identifica al cliente dueño del registro).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `data_source_id` | `string (uuid)` | sí |  |
| `operation_key` | `string` | sí | Operation key (describe_data_source); prefer a "list" operation. |
| `params` | `object` | no | Fixed parameters the operation needs, if any. |
| `key` | `string` | no | Entity key; omitted = inferred from the operation. |
| `name` | `string` | no | Singular name in Portuguese, e.g. "Contrato". |
| `name_plural` | `string` | no | Plural name, e.g. "Contratos". |
| `fields` | `array<string>` | no | Field keys to keep, from the list a refusal gives; omitted = every visible field the sample shows. |
| `field_names` | `object` | no | Labels in Brazilian Portuguese for the kept fields, by key (\{ "amount": "Valor", "expire_at": "Vencimento" \}): they are the column headers the customers read. |
| `owner_field` | `string` | no | The field saying which customer the record belongs to (the titular), when the inferred one is wrong or missing. On a CUSTOMERS entity it is its own id (each customer owns their own record); on a data entity, the customer id it carries (e.g. customer_id). When the portal is keyed by CPF/CNPJ, the document field instead. |
| `owner_match` | `string` | no | How the owner is compared to the users' tenant ID: external_id = as text (an id); document_number = as a CPF/CNPJ, digits only on both sides, so "32.013.964/0001-60" matches "32013964000160". Use document_number whenever the owner field holds a CPF/CNPJ. Valores: `external_id`, `document_number`. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.register-entity`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools remove-block

Quitar un bloque

```text
kobana dashboards builder tools remove-block [OPCIONES]
```

`POST /v1/builder/tools/remove_block`

Quita un bloque (y los bloques dentro de él) de su página.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `block_id` | `string` | sí | Block id (get_page). |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.remove-block`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools rename-entity-fields

Renombrar los campos de una entidad

```text
kobana dashboards builder tools rename-entity-fields [OPCIONES]
```

`POST /v1/builder/tools/rename_entity_fields`

Modifica una entidad existente: renombra sus campos — las etiquetas que muestran los bloques (encabezados de tabla, etiquetas de tarjetas) — y, opcionalmente, la propia entidad, y puede definir su campo titular (de quién es el registro). Úsalo cuando las etiquetas vinieron de la fuente en inglés o como claves crudas ("Expire at", "customer_cnpj_cpf"): los clientes las leen, así que van en portugués de Brasil; y cuando el titular está mal o falta (una entidad de clientes es titular de sus propios registros por su id). Las claves se mantienen; un titular nuevo vuelve a mapear los registros guardados. Tiene efecto en el workspace de inmediato, no en el borrador.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `entity` | `string` | sí | Entity key (list_entities). |
| `names` | `object` | no | New label by field key, e.g. \{ "amount": "Valor", "expire_at": "Vencimento" \}. |
| `name` | `string` | no | New singular name of the entity. |
| `name_plural` | `string` | no | New plural name of the entity. |
| `owner_field` | `string` | no | New owner field (a field key of the entity, a computed one included). |
| `owner_match` | `string` | no | How the owner is compared to the users' tenant ID: external_id = as text (an id); document_number = as a CPF/CNPJ, digits only on both sides, so "32.013.964/0001-60" matches "32013964000160". Use document_number whenever the owner field holds a CPF/CNPJ. Valores: `external_id`, `document_number`. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.rename-entity-fields`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools save-computed-field

Guardar un campo calculado

```text
kobana dashboards builder tools save-computed-field [OPCIONES]
```

`POST /v1/builder/tools/save_computed_field`

Agrega un campo calculado a una entidad — un valor derivado de los otros campos por una expresión (al estilo de n8n) — o modifica la expresión de un campo calculado que ya tiene (misma clave). Úsalo cuando el operador quiere un valor que la fuente no entrega tal cual: parte de un texto, la unión de campos, una fecha o un importe con formato, una etiqueta a partir de una condición. La expresión se prueba primero en los registros guardados (una expresión que falla en todos se rechaza y no se guarda nada), luego se guarda en la entidad en el workspace de inmediato (no es parte del borrador) y se calcula en cada registro guardado con los datos que ya tiene (sin volver a consultar la fuente). Los bloques y sample_records ya la ven en la misma respuesta.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `entity` | `string` | sí | Entity key (list_entities). |
| `key` | `string` | sí | Field key, snake_case. Must not be a field the source maps. |
| `name` | `string` | sí | Field label in Portuguese, e.g. "Descrição curta". |
| `expression` | `string` | sí | Template with \{\{ \}\}, e.g. "\{\{ $resource.description.split('\\r\\n')[0] \}\}". |
| `data_type` | `string` | no | What the expression gives: string, int, decimal, money, bool, date, datetime… Valores: `string`, `int`, `decimal`, `money`, `bool`, `date`, `datetime`, `email`, `enum`, `document`, `phone`, `url`, `file`, `array`, `json`. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.save-computed-field`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools update-block

Modificar un bloque

```text
kobana dashboards builder tools update-block [OPCIONES]
```

`POST /v1/builder/tools/update_block`

Modifica props de un bloque. Solo cambian las props enviadas; el resultado se valida contra el tipo del bloque.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `block_id` | `string` | sí | Block id (get_page). |
| `props` | `object` | sí | Block props (describe_block_type). Never include id or slot props. |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.update-block`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder tools update-page

Modificar una página

```text
kobana dashboards builder tools update-page [OPCIONES]
```

`POST /v1/builder/tools/update_page`

Modifica la configuración de una página: nombre en el menú, ruta, título en el navegador, si aparece en el menú del portal y las etiquetas de SEO / Open Graph. Solo cambian los campos enviados.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page_id` | `string` | sí | Page id (list_pages). |
| `name` | `string` | no |  |
| `path` | `string` | no |  |
| `title` | `string` | no |  |
| `show_in_menu` | `boolean` | no | false = the page stays reachable by links (e.g. a record detail page) but is not in the header menu. |
| `seo` | `object` | no |  |
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.tools.update-page`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lo que hizo la herramienta, en `data`, y la nueva `revision` del borrador (la misma si nada cambió). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | La herramienta rechazó el paso (`BUILDER_OPERATION_REJECTED`): tipo de bloque inexistente, props que no pasan el esquema del bloque, ruta en uso, entidad desconocida… El mensaje dice el motivo y no se guarda nada. |

## builder versions get

Consultar una versión publicada

```text
kobana dashboards builder versions get [NUMBER] [OPCIONES]
```

`GET /v1/builder/versions/{number}`

Devuelve una versión publicada del portal, por número, con las páginas que congeló.

**Argumentos de la ruta:** `number`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La versión y sus páginas. |
| `404` | Recurso no encontrado en la organización actual. |

## builder versions restore

Restaurar una versión en el borrador

```text
kobana dashboards builder versions restore [NUMBER] [OPCIONES]
```

`POST /v1/builder/versions/{number}/restore`

Copia una versión publicada al borrador, reemplazando sus páginas — para seguir editando desde una versión antigua. Nada se publica: el portal solo cambia cuando se publique el borrador. (Poner una versión antigua en línea es `.../rollback`.) `revision` es opcional: enviada y desactualizada, responde 409.

**Argumentos de la ruta:** `number`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `revision` | `integer` | no | La `revision` del borrador leída por último. Opcional: si se envía y está desactualizada, responde 409 y no guarda nada. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.versions.restore`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Borrador con las páginas de la versión: nueva `revision`, número restaurado y cuántas páginas. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |

## builder versions rollback

Volver a poner una versión en el aire

```text
kobana dashboards builder versions rollback [NUMBER] [OPCIONES]
```

`POST /v1/builder/versions/{number}/rollback`

Vuelve a publicar esa versión como el próximo número — los visitantes del portal la ven de inmediato. Devuelve la nueva versión y el constructor tal como quedó.

**Argumentos de la ruta:** `number`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Nueva versión publicada y el constructor actualizado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Esa versión ya está publicada (`ALREADY_LIVE`). |

## builder encrypt-secret

Cifrar un secreto de bloque

```text
kobana dashboards builder encrypt-secret [OPCIONES]
```

`POST /v1/builder/encrypt_secret`

Cifra un valor que un bloque guarda en el JSON de la página (una cabecera de webhook como `Authorization`). Devuelve `secret:<cifrado>`, vinculado a este workspace y al origen de `url` (obligatoria), que solo el servidor abre para enviar la solicitud a ese origen. Nada se almacena: guarde el valor devuelto en el bloque junto con el borrador. Sustituye a `POST /builder/secrets`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `value` | `string` | sí | El valor en texto plano a cifrar. Nunca se devuelve. |
| `url` | `string` | sí | La URL del webhook a la que se destina el valor (obligatoria): el secreto solo se abre para una solicitud al mismo origen (esquema, dominio y puerto). El dominio debe ser fijo — sin `{{variables}}`. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.encrypt-secret`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El valor cifrado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | La `url` no tiene un origen http(s) fijo — por ejemplo, una `{{variable}}` en el dominio (`ACTION_SECRET_URL_REQUIRED`). |

## builder publish

Publicar el portal

```text
kobana dashboards builder publish [OPCIONES]
```

`POST /v1/builder/publish`

Guarda el borrador tal como se envía (mismo cuerpo que el guardado, más una `note` opcional) y lo congela como la próxima versión, en una sola transacción: los visitantes del portal la ven de inmediato. Un campo de formulario cuya expresión regular puede bloquear la validación responde 422 `UNSAFE_FIELD_PATTERN`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `revision` | `integer` | sí | La última `revision` del borrador leída (`0` cuando aún no hay borrador). |
| `pages` | `array<object>` | sí | Todas las páginas del borrador, en el orden del menú (hasta 200). Las páginas omitidas salen del borrador. |
| `editor_state` | `object` | no | Estado del editor; devuelva lo que leyó. Omitido = `{}`. |
| `theme` | `object` | no |  |
| `seo_defaults` | `object` | no |  |
| `note` | `string` | no | Lo que cambió, mostrado en el historial de versiones. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.publish`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Versión publicada, con la nueva `revision` del borrador. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La `revision` enviada está desactualizada: otra persona o pestaña guardó el borrador. Léalo de nuevo y reenvíe. |
| `422` | Una expresión regular de campo de formulario puede bloquear la validación (`UNSAFE_FIELD_PATTERN`); `details` indica página, bloque y campo. |

## builder test-action

Probar un botón de acción del borrador

```text
kobana dashboards builder test-action [OPCIONES]
```

`POST /v1/builder/test_action`

Ejecuta el webhook de un Botón de acción como la vista previa del constructor: la solicitud sale de verdad, marcada como `preview: true`, y la respuesta trae el estado HTTP devuelto por el destino. Las cabeceras pueden llevar valores `secret:` creados por `POST /builder/encrypt_secret`, que solo se abren para el origen para el que fueron creados. Limitado a 20 ejecuciones por minuto por operador, 60 por minuto y 500 por día por organización. Sustituye a `POST /builder/action_runs`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `block_id` | `string` | sí | Id del bloque Botón de acción. |
| `request` | `object` | sí |  |
| `label` | `string` | no |  |
| `page_path` | `string` | no | Ruta de la página donde está el bloque. Predeterminado `/`. |
| `submitter` | `object` | no | Cliente que representa la vista previa, para `{{customer_name}}` / `{{customer_email}}`. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.test-action`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la ejecución. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Solicitud mal configurada (URL sin https, cabecera inválida, secreto de otro workspace…). |

## builder test-form

Probar un formulario del borrador

```text
kobana dashboards builder test-form [OPCIONES]
```

`POST /v1/builder/test_form`

Ejecuta la acción de un bloque Formulario como la vista previa del constructor: el e-mail configurado sale de verdad (o el webhook elegido se dispara), marcado como `preview: true`. La configuración viene en el cuerpo (el borrador de quien llama). El e-mail de prueba va solo al operador que el token nombra. Limitado a 20 ejecuciones por minuto por operador, 60 por minuto y 500 por día por organización. Sustituye a `POST /builder/form_submissions`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `form_id` | `string` | sí | Id del bloque Formulario. |
| `fields` | `array<object>` | sí |  |
| `action` | `object` | sí |  |
| `form_title` | `string` | no |  |
| `page_path` | `string` | no | Ruta de la página donde está el bloque. Predeterminado `/`. |
| `values` | `object` | no | Valores completados, por nombre del campo. |
| `submitter` | `object` | no | Cliente que representa la vista previa, para `{{customer_name}}` / `{{customer_email}}`. |

Otros campos y objetos anidados: `kobana schema dashboards.builder.test-form`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Envío registrado: destinatarios, notificaciones o la entrega del webhook. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Valores que no cumplen las reglas del formulario, expresión regular peligrosa o acción mal configurada; `field_errors` lista los campos. |

## builder upload-og-image

Subir la og:image del portal

```text
kobana dashboards builder upload-og-image [OPCIONES]
```

`POST /v1/builder/upload_og_image`

Sube la og:image de una página (o la predeterminada del portal) — la imagen que muestran las vistas previas de enlaces — en base64 (PNG, JPEG o WebP, hasta ~1,5 MB) y devuelve la URL pública, para guardar en `seo.og_image_url` (o `seo_defaults.og_image_url`) en el próximo guardado del borrador. Sustituye a `POST /builder/og_image`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `filename` | `string` | sí | Nombre original del archivo. |
| `content_type` | `string` | sí | Valores: `image/png`, `image/jpeg`, `image/webp`. |
| `content_base64` | `string` | sí | Los bytes de la imagen en base64 (se acepta un prefijo `data:…;base64,`). |

Otros campos y objetos anidados: `kobana schema dashboards.builder.upload-og-image`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Imagen subida, con su URL pública. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `413` | El cuerpo de la solicitud supera los 2 MB. |
| `422` | Tipo no aceptado, base64 inválido, imagen por encima del límite o contenido que no corresponde al `content_type`. |
