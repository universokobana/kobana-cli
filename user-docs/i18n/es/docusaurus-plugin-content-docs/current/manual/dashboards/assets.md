---
id: assets
title: "kobana dashboards assets"
sidebar_label: "assets"
---

# kobana dashboards assets


Comandos de `kobana dashboards assets`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`assets list`](#assets-list) | `GET /v1/assets` | Listar archivos |
| [`assets create`](#assets-create) | `POST /v1/assets` | Agregar archivo |
| [`assets presign`](#assets-presign) | `POST /v1/assets/presign` | Generar URL de envío |
| [`assets get`](#assets-get) | `GET /v1/assets/{id}` | Consultar archivo |
| [`assets update`](#assets-update) | `PATCH /v1/assets/{id}` | Actualizar archivo |
| [`assets delete`](#assets-delete) | `DELETE /v1/assets/{id}` | Eliminar archivo |
| [`assets download`](#assets-download) | `GET /v1/assets/{id}/download` | Descargar archivo |

## assets list

Listar archivos

```text
kobana dashboards assets list [OPCIONES]
```

`GET /v1/assets`

La biblioteca de archivos del workspace — imágenes y documentos que el portal muestra u ofrece para descargar. Ordenable por `created_at` (por defecto), `name` o `size_bytes`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `kind` | `string` | no | Filtra por tipo: `image`, `document` u `other`. Valores: `image`, `document`, `other`. |
| `visibility` | `string` | no | Quién puede descargarlo: `public` (cualquiera con el enlace), `restricted` (usuarios con sesión en el portal del workspace) o `private` (solo miembros del panel). Los archivos nuevos empiezan `restricted`. Valores: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | no | Solo los archivos de este tenant. |
| `type_id` | `string` | no | Solo los archivos de este tipo. |
| `tag` | `string` | no | Solo los archivos con esta etiqueta. |
| `search` | `string` | no | Parte del nombre del archivo (sin distinguir mayúsculas). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de archivos. |
| `404` | Recurso no encontrado en la organización actual. |

## assets create

Agregar archivo

```text
kobana dashboards assets create [OPCIONES]
```

`POST /v1/assets`

Registra un archivo en la biblioteca, de una de dos formas: el `key` de un archivo ya enviado por la URL de `POST /assets/presign`, o el propio contenido en `content_base64`. El archivo empieza `restricted`, salvo que `visibility` diga otra cosa. Dispara el webhook `asset.created`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre visible, con la extensión (obligatorio). La extensión decide el tipo aceptado cuando no viene `content_type`. |
| `key` | `string` | no | El `key` devuelto por `POST /assets/presign`, después de enviar el archivo a la URL firmada. Envíe `key` o `content_base64`, nunca ambos. |
| `content_base64` | `string` | no | El contenido del archivo en base64, para archivos pequeños: el cuerpo de la solicitud tiene un límite de 2 MB (cerca de 1,5 MB de archivo). Para archivos mayores, use `POST /assets/presign`. |
| `content_type` | `string` | no | Tipo MIME. Si se omite, se deduce de la extensión. Aceptados: imágenes PNG, JPEG, GIF, WebP y AVIF; PDF, texto, CSV, Markdown, JSON, XML, Office y ZIP. SVG y HTML no se aceptan. |
| `alt_text` | `string` | no | Texto alternativo de una imagen (accesibilidad). |
| `visibility` | `string` | no | Quién puede descargarlo: `public` (cualquiera con el enlace), `restricted` (usuarios con sesión en el portal del workspace) o `private` (solo miembros del panel). Los archivos nuevos empiezan `restricted`. Valores: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal que ven el archivo — todos los usuarios del mismo tenant ven los mismos archivos, en los bloques Archivos del Usuario. `null` = archivo del workspace, sin tenant. |
| `type_id` | `string (uuid)` | no | Tipo del archivo (`GET /asset_types`: Propuestas, Contratos…). Un bloque Archivos del Usuario lista los tipos que elige; un archivo con tipo y sin tenant aparece para todos los usuarios en los bloques de ese tipo. `null` = sin tipo. |
| `tags` | `array<string>` | no | Etiquetas del archivo (hasta 20, cada una con hasta 50 caracteres; las repetidas se ignoran). Reemplaza las actuales. |

Otros campos y objetos anidados: `kobana schema dashboards.assets.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Archivo registrado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## assets presign

Generar URL de envío

```text
kobana dashboards assets presign [OPCIONES]
```

`POST /v1/assets/presign`

Una URL firmada y de corta duración para enviar el archivo directo al storage con `PUT` — el camino para archivos que superan el límite de 2 MB del cuerpo. Después del `PUT`, registre el archivo en `POST /assets` con el `key` devuelto.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `filename` | `string` | sí | Nombre del archivo con la extensión. |
| `size` | `integer` | sí | Tamaño exacto, en bytes, del archivo que se enviará (máximo 25 MB). |
| `content_type` | `string` | no | Tipo MIME. Si se omite, se deduce de la extensión. Aceptados: imágenes PNG, JPEG, GIF, WebP y AVIF; PDF, texto, CSV, Markdown, JSON, XML, Office y ZIP. SVG y HTML no se aceptan. |

Otros campos y objetos anidados: `kobana schema dashboards.assets.presign`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | URL de envío generada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## assets get

Consultar archivo

```text
kobana dashboards assets get [ID] [OPCIONES]
```

`GET /v1/assets/{id}`

Un archivo de la biblioteca, con su enlace, visibilidad, tipo y tamaño.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El archivo. |
| `404` | Recurso no encontrado en la organización actual. |

## assets update

Actualizar archivo

```text
kobana dashboards assets update [ID] [OPCIONES]
```

`PATCH /v1/assets/{id}`

Renombra el archivo, cambia el texto alternativo o la visibilidad. Hacerlo público abre la URL del storage a cualquiera; `restricted` o `private` la cierra.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre visible del archivo (por defecto, el nombre original, con la extensión). |
| `alt_text` | `string` | no | Texto alternativo de una imagen. Envíe `null` para limpiarlo. |
| `visibility` | `string` | no | Quién puede descargarlo: `public` (cualquiera con el enlace), `restricted` (usuarios con sesión en el portal del workspace) o `private` (solo miembros del panel). Los archivos nuevos empiezan `restricted`. Valores: `public`, `restricted`, `private`. |
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal que ven el archivo — todos los usuarios del mismo tenant ven los mismos archivos, en los bloques Archivos del Usuario. `null` = archivo del workspace, sin tenant. |
| `type_id` | `string (uuid)` | no | Tipo del archivo (`GET /asset_types`: Propuestas, Contratos…). Un bloque Archivos del Usuario lista los tipos que elige; un archivo con tipo y sin tenant aparece para todos los usuarios en los bloques de ese tipo. `null` = sin tipo. |
| `tags` | `array<string>` | no | Etiquetas del archivo (hasta 20, cada una con hasta 50 caracteres; las repetidas se ignoran). Reemplaza las actuales. |

Otros campos y objetos anidados: `kobana schema dashboards.assets.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivo actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## assets delete

Eliminar archivo

```text
kobana dashboards assets delete [ID] [OPCIONES]
```

`DELETE /v1/assets/{id}`

Quita el archivo de la biblioteca. El archivo guardado sigue en el storage, para que los enlaces ya publicados no se rompan. Dispara el webhook `asset.deleted`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivo eliminado. |
| `404` | Recurso no encontrado en la organización actual. |

## assets download

Descargar archivo

```text
kobana dashboards assets download [ID] [OPCIONES]
```

`GET /v1/assets/{id}/download`

El contenido del archivo, sea cual sea la visibilidad, con su propio `Content-Type`.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `inline` | `string` | no | Con `true`, imágenes y PDF vienen para mostrarse en el lugar (`Content-Disposition: inline`); los demás tipos vienen siempre como adjunto. Valores: `true`, `false`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El contenido del archivo. |
| `404` | Recurso no encontrado en la organización actual. |
