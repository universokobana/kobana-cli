---
id: email-templates
title: "kobana dashboards email-templates"
sidebar_label: "email-templates"
---

# kobana dashboards email-templates


Comandos de `kobana dashboards email-templates`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`email-templates preview`](#email-templates-preview) | `GET /v1/email_templates/{id}/preview` | Previsualizar plantilla de correo |
| [`email-templates preview create`](#email-templates-preview-create) | `POST /v1/email_templates/{id}/preview` | Renderizar plantilla con variables |
| [`email-templates list`](#email-templates-list) | `GET /v1/email_templates` | Listar plantillas de correo |
| [`email-templates create`](#email-templates-create) | `POST /v1/email_templates` | Crear plantilla de correo |
| [`email-templates init-defaults`](#email-templates-init-defaults) | `POST /v1/email_templates/init_defaults` | Restaurar plantillas predeterminadas |
| [`email-templates get`](#email-templates-get) | `GET /v1/email_templates/{id}` | Consultar plantilla de correo |
| [`email-templates update`](#email-templates-update) | `PATCH /v1/email_templates/{id}` | Actualizar plantilla de correo |
| [`email-templates delete`](#email-templates-delete) | `DELETE /v1/email_templates/{id}` | Eliminar plantilla de correo |
| [`email-templates duplicate`](#email-templates-duplicate) | `POST /v1/email_templates/{id}/duplicate` | Duplicar plantilla de correo |

## email-templates preview

Previsualizar plantilla de correo

```text
kobana dashboards email-templates preview [ID] [OPCIONES] [COMANDO]
```

`GET /v1/email_templates/{id}/preview`

Renderiza la plantilla con datos de ejemplo, dentro del diseño de la organización. No se envía nada.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El correo renderizado. |
| `404` | Recurso no encontrado en la organización actual. |

## email-templates preview create

Renderizar plantilla con variables

```text
kobana dashboards email-templates preview create [ID] [OPCIONES]
```

`POST /v1/email_templates/{id}/preview`

Renderiza la plantilla con las `variables` enviadas, sin diseño. No se envía nada.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `variables` | `object` | no | Valores de las variables `{{nombre}}` de la plantilla. |

Otros campos y objetos anidados: `kobana schema dashboards.email-templates.preview.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El correo renderizado. |
| `400` | Solicitud inválida (campo obligatorio ausente, tipo o variables inválidos). |
| `404` | Recurso no encontrado en la organización actual. |

## email-templates list

Listar plantillas de correo

```text
kobana dashboards email-templates list [OPCIONES]
```

`GET /v1/email_templates`

Lista las plantillas de correo de la propia organización, ordenadas por tipo y nombre. Devuelve el conjunto completo, sin paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `type` | `string` | no | Solo plantillas de este tipo de notificación. Valores: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `is_active` | `string` | no | `true` para las plantillas activas, `false` para los borradores. Valores: `true`, `false`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plantillas de la organización. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## email-templates create

Crear plantilla de correo

```text
kobana dashboards email-templates create [OPCIONES]
```

`POST /v1/email_templates`

Crea una plantilla para un tipo de notificación. Queda como borrador, salvo que `is_active` sea `true` — entonces la plantilla activa anterior del tipo pasa a borrador.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la plantilla (obligatorio al crear). |
| `type` | `string` | sí | Tipo de notificación que la plantilla renderiza (obligatorio). Valores: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `subject` | `string` | sí | Asunto del correo, con variables `{{nombre}}` (obligatorio al crear). |
| `html_body` | `string` | sí | Cuerpo HTML, con variables `{{nombre}}` (obligatorio al crear). |
| `text_body` | `string` | no | Cuerpo alternativo en texto plano. `null` lo limpia. |
| `bcc` | `anyOf` | no | Lista (o una sola dirección) de correos en copia oculta. Direcciones inválidas devuelven 422; si se omite al actualizar, se mantiene la lista actual. |
| `variables` | `array<string>` | no | Nombres de las variables que usa la plantilla. |
| `is_active` | `boolean` | no | Hace de esta la plantilla activa del tipo; la activa anterior pasa a borrador. Por defecto `false` al crear. |

Otros campos y objetos anidados: `kobana schema dashboards.email-templates.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Plantilla creada. |
| `400` | Solicitud inválida (campo obligatorio ausente, tipo o variables inválidos). |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## email-templates init-defaults

Restaurar plantillas predeterminadas

```text
kobana dashboards email-templates init-defaults [OPCIONES]
```

`POST /v1/email_templates/init_defaults`

Crea o restaura la plantilla de cada tipo de notificación con el contenido predeterminado y la activa. Sobrescribe el contenido de las plantillas existentes de la organización.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Una fila por tipo de notificación. |
| `404` | Recurso no encontrado en la organización actual. |

## email-templates get

Consultar plantilla de correo

```text
kobana dashboards email-templates get [ID] [OPCIONES]
```

`GET /v1/email_templates/{id}`

Devuelve una plantilla de la organización o una predeterminada de la plataforma.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La plantilla. |
| `404` | Recurso no encontrado en la organización actual. |

## email-templates update

Actualizar plantilla de correo

```text
kobana dashboards email-templates update [ID] [OPCIONES]
```

`PATCH /v1/email_templates/{id}`

Actualiza una plantilla de la organización. Solo cambian los campos enviados. Las predeterminadas de la plataforma no se pueden modificar (404).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre de la plantilla (obligatorio al crear). |
| `subject` | `string` | no | Asunto del correo, con variables `{{nombre}}` (obligatorio al crear). |
| `html_body` | `string` | no | Cuerpo HTML, con variables `{{nombre}}` (obligatorio al crear). |
| `text_body` | `string` | no | Cuerpo alternativo en texto plano. `null` lo limpia. |
| `bcc` | `anyOf` | no | Lista (o una sola dirección) de correos en copia oculta. Direcciones inválidas devuelven 422; si se omite al actualizar, se mantiene la lista actual. |
| `variables` | `array<string>` | no | Nombres de las variables que usa la plantilla. |
| `is_active` | `boolean` | no | Hace de esta la plantilla activa del tipo; la activa anterior pasa a borrador. Por defecto `false` al crear. |
| `public_id` | `string (uuid)` | no | Nuevo identificador público (UUID), cuando la organización permite elegirlo. |

Otros campos y objetos anidados: `kobana schema dashboards.email-templates.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plantilla actualizada. |
| `400` | Solicitud inválida (campo obligatorio ausente, tipo o variables inválidos). |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | El `public_id` enviado ya está en uso. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## email-templates delete

Eliminar plantilla de correo

```text
kobana dashboards email-templates delete [ID] [OPCIONES]
```

`DELETE /v1/email_templates/{id}`

Elimina definitivamente una plantilla inactiva de la organización. La plantilla activa de un tipo no se puede eliminar (422): active otra antes.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plantilla eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## email-templates duplicate

Duplicar plantilla de correo

```text
kobana dashboards email-templates duplicate [ID] [OPCIONES]
```

`POST /v1/email_templates/{id}/duplicate`

Copia una plantilla como borrador (inactiva).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `new_name` | `string` | no | Nombre de la copia. Por defecto: "&lt;nombre> (Cópia)". |

Otros campos y objetos anidados: `kobana schema dashboards.email-templates.duplicate`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | La copia creada. |
| `400` | Solicitud inválida (campo obligatorio ausente, tipo o variables inválidos). |
| `404` | Recurso no encontrado en la organización actual. |
