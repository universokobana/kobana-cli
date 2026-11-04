---
id: email-layouts
title: "kobana dashboards email-layouts"
sidebar_label: "email-layouts"
---

# kobana dashboards email-layouts


Comandos de `kobana dashboards email-layouts`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`email-layouts list`](#email-layouts-list) | `GET /v1/email_layouts` | Listar diseños de correo |
| [`email-layouts create`](#email-layouts-create) | `POST /v1/email_layouts` | Crear diseño de correo |
| [`email-layouts init-defaults`](#email-layouts-init-defaults) | `POST /v1/email_layouts/init_defaults` | Crear diseño predeterminado |
| [`email-layouts get`](#email-layouts-get) | `GET /v1/email_layouts/{id}` | Consultar diseño de correo |
| [`email-layouts update`](#email-layouts-update) | `PATCH /v1/email_layouts/{id}` | Actualizar diseño de correo |
| [`email-layouts delete`](#email-layouts-delete) | `DELETE /v1/email_layouts/{id}` | Eliminar diseño de correo |

## email-layouts list

Listar diseños de correo

```text
kobana dashboards email-layouts list [OPCIONES]
```

`GET /v1/email_layouts`

Lista los diseños de correo de la organización, el predeterminado primero. Devuelve el conjunto completo, sin paginación.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Diseños de la organización. |
| `404` | Recurso no encontrado en la organización actual. |

## email-layouts create

Crear diseño de correo

```text
kobana dashboards email-layouts create [OPCIONES]
```

`POST /v1/email_layouts`

Crea un diseño. El HTML debe contener el marcador `{{content}}`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del diseño (obligatorio al crear). |
| `html_body` | `string` | sí | Documento HTML con el marcador `{{content}}` (obligatorio al crear; sin el marcador, 422). |
| `is_default` | `boolean` | no | Hace de este el diseño predeterminado; el anterior deja de serlo. |

Otros campos y objetos anidados: `kobana schema dashboards.email-layouts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Diseño creado. |
| `400` | Solicitud inválida (campo obligatorio ausente, tipo o variables inválidos). |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## email-layouts init-defaults

Crear diseño predeterminado

```text
kobana dashboards email-layouts init-defaults [OPCIONES]
```

`POST /v1/email_layouts/init_defaults`

Garantiza que la organización tenga un diseño predeterminado: devuelve el actual o crea el incorporado. Nunca sobrescribe un diseño.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El diseño predeterminado. |
| `404` | Recurso no encontrado en la organización actual. |

## email-layouts get

Consultar diseño de correo

```text
kobana dashboards email-layouts get [ID] [OPCIONES]
```

`GET /v1/email_layouts/{id}`

Devuelve un diseño de la organización.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El diseño. |
| `404` | Recurso no encontrado en la organización actual. |

## email-layouts update

Actualizar diseño de correo

```text
kobana dashboards email-layouts update [ID] [OPCIONES]
```

`PATCH /v1/email_layouts/{id}`

Actualiza nombre, HTML (que debe mantener `{{content}}`), predeterminado o activo. Solo cambian los campos enviados.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre del diseño (obligatorio al crear). |
| `html_body` | `string` | no | Documento HTML con el marcador `{{content}}` (obligatorio al crear; sin el marcador, 422). |
| `is_default` | `boolean` | no | Hace de este el diseño predeterminado; el anterior deja de serlo. |
| `is_active` | `boolean` | no | Activa o desactiva el diseño. |

Otros campos y objetos anidados: `kobana schema dashboards.email-layouts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Diseño actualizado. |
| `400` | Solicitud inválida (campo obligatorio ausente, tipo o variables inválidos). |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## email-layouts delete

Eliminar diseño de correo

```text
kobana dashboards email-layouts delete [ID] [OPCIONES]
```

`DELETE /v1/email_layouts/{id}`

Elimina definitivamente un diseño. Los correos pasan a usar el diseño predeterminado (o el incorporado).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Diseño eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
