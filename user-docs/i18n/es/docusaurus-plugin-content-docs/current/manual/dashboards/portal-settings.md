---
id: portal-settings
title: "kobana dashboards portal-settings"
sidebar_label: "portal-settings"
description: "Obtener configuración del portal"
---

# kobana dashboards portal-settings

Obtener configuración del portal

Comandos de `kobana dashboards portal-settings`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`portal-settings`](#portal-settings) | `GET /v1/portal_settings` | Obtener configuración del portal |
| [`portal-settings domain check-slug`](#portal-settings-domain-check-slug) | `GET /v1/portal_settings/domain/check_slug` | Verificar disponibilidad de la dirección |
| [`portal-settings domain update`](#portal-settings-domain-update) | `PATCH /v1/portal_settings/domain` | Definir dirección del portal |
| [`portal-settings logo create`](#portal-settings-logo-create) | `POST /v1/portal_settings/logo` | Subir imagen del portal |
| [`portal-settings logo delete`](#portal-settings-logo-delete) | `DELETE /v1/portal_settings/logo` | Eliminar imagen del portal |
| [`portal-settings update`](#portal-settings-update) | `PATCH /v1/portal_settings` | Actualizar configuración del portal |
| [`portal-settings verify-sender`](#portal-settings-verify-sender) | `POST /v1/portal_settings/verify_sender` | Verificar el e-mail remitente de las notificaciones |

## portal-settings

Obtener configuración del portal

```text
kobana dashboards portal-settings [OPCIONES] [COMANDO]
```

`GET /v1/portal_settings`

Devuelve la configuración del portal del workspace: marca, idioma de la página, colores, dirección, opciones de inicio de sesión y SSO, notificaciones y remitente, y configuración del sitio (cabeceras de seguridad, SEO, rastreadores, chat y código propio).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Configuración del portal del workspace, con los valores predeterminados aplicados. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-settings domain check-slug

Verificar disponibilidad de la dirección

```text
kobana dashboards portal-settings domain check-slug [OPCIONES]
```

`GET /v1/portal_settings/domain/check_slug`

Indica si el `slug` está libre bajo `domain` para el portal de este workspace dentro de la organización (su dirección actual cuenta como libre).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `slug` | `string` | sí | Slug del workspace; la dirección es `{slug_de_la_organizacion}-{slug}.{domain}` (una sola etiqueta, hasta 63 caracteres): 3 a 50 letras minúsculas, números y guiones (sin guion en los extremos). `null` para eliminar. |
| `domain` | `string` | sí | Uno de los dominios de la plataforma (`available_domains`). `null` para eliminar. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la verificación. |
| `400` | Solicitud inválida (parámetro ausente o con formato incorrecto). |
| `404` | Recurso no encontrado en la organización actual. |

## portal-settings domain update

Definir dirección del portal

```text
kobana dashboards portal-settings domain update [OPCIONES]
```

`PATCH /v1/portal_settings/domain`

Define la dirección del portal en la plataforma, `https://{slug_de_la_organizacion}-{slug}.{domain}` (el slug de la organización es fijo aquí), o la elimina cuando ambos son `null`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `slug` | `string` | sí | Slug del workspace; la dirección es `{slug_de_la_organizacion}-{slug}.{domain}` (una sola etiqueta, hasta 63 caracteres): 3 a 50 letras minúsculas, números y guiones (sin guion en los extremos). `null` para eliminar. |
| `domain` | `string` | sí | Uno de los dominios de la plataforma (`available_domains`). `null` para eliminar. |

Otros campos y objetos anidados: `kobana schema dashboards.portal-settings.domain.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Dirección actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Rechazado por una regla de negocio (dirección en uso o reservada, archivo demasiado grande o de tipo no aceptado…). |

## portal-settings logo create

Subir imagen del portal

```text
kobana dashboards portal-settings logo create [OPCIONES]
```

`POST /v1/portal_settings/logo`

Sube el logo, el logo para fondos oscuros o el favicon del portal, en base64 en el cuerpo JSON. Reemplaza la imagen anterior.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `filename` | `string` | sí |  |
| `content_type` | `string` | sí | `image/png`, `image/jpeg` o `image/webp`; el favicon también acepta `image/x-icon`. SVG no se acepta. |
| `content_base64` | `string` | sí | Contenido del archivo en base64 (hasta 500 KB una vez decodificado). |
| `variant` | `string` | no | Qué imagen: `logo` (predeterminado), `logo_light` (logo para fondos oscuros) o `favicon`. Valores: `logo`, `logo_light`, `favicon`. |

Otros campos y objetos anidados: `kobana schema dashboards.portal-settings.logo.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Imagen guardada; `url` es su dirección pública. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Rechazado por una regla de negocio (dirección en uso o reservada, archivo demasiado grande o de tipo no aceptado…). |

## portal-settings logo delete

Eliminar imagen del portal

```text
kobana dashboards portal-settings logo delete [OPCIONES]
```

`DELETE /v1/portal_settings/logo`

Elimina el logo, el logo para fondos oscuros o el favicon; el portal vuelve al predeterminado.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `variant` | `string` | no | Qué imagen: `logo` (predeterminado), `logo_light` (logo para fondos oscuros) o `favicon`. Valores: `logo`, `logo_light`, `favicon`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Imagen eliminada. |
| `400` | Solicitud inválida (parámetro ausente o con formato incorrecto). |
| `404` | Recurso no encontrado en la organización actual. |

## portal-settings update

Actualizar configuración del portal

```text
kobana dashboards portal-settings update [OPCIONES]
```

`PATCH /v1/portal_settings`

Actualiza la configuración del portal del workspace. Parcial: solo cambian los campos enviados. Devuelve la configuración completa.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_name` | `string` | no |  |
| `title` | `string` | no |  |
| `subtitle` | `string` | no |  |
| `html_lang` | `string` | no | El idioma del portal: va en el `lang` del `<html>` de las páginas y traduce las pantallas de inicio de sesión. Valores: `pt-BR`, `en-US`, `es-ES`. |
| `light_colors` | `object` | no | Colores del tema, en HSL sin `hsl()` (ej.: `210 40% 98%`). Las claves omitidas usan el valor predeterminado. |
| `dark_colors` | `object` | no | Colores del tema, en HSL sin `hsl()` (ej.: `210 40% 98%`). Las claves omitidas usan el valor predeterminado. |
| `enable_login` | `boolean` | no |  |
| `enable_email_login` | `boolean` | no |  |
| `allow_password_change` | `boolean` | no |  |
| `sso_enabled` | `boolean` | no |  |
| `sso_public_key` | `string` | no |  |
| `sso_algorithm` | `string` | no | Valores: `RS256`, `RS384`, `RS512`, `ES256`, `ES384`, `ES512`, `PS256`, `PS384`, `PS512`. |
| `sso_issuer` | `string` | no |  |
| `notification_type_statuses` | `object` | no | Estado de cada notificación del portal: `ACTIVE` (se envía), `INACTIVE` (silenciada) o `DISABLED` (no disponible). |
| `notification_bcc_emails` | `array<object>` | no | Reemplaza la lista completa (hasta 20). |
| `notification_sender_name` | `string` | no |  |
| `notification_sender_email` | `string` | no | Una dirección nueva inicia su verificación en AWS SES, que le envía un enlace de confirmación. `""` vuelve al remitente de la plataforma. |
| `notification_reply_to_email` | `string` | no |  |
| `hsts_include_subdomains` | `boolean` | no |  |
| `hsts_preload` | `boolean` | no |  |
| `secure_frame_headers` | `boolean` | no |  |
| `seo_indexing_enabled` | `boolean` | no |  |
| `seo_robots_txt` | `string` | no |  |
| `seo_sitemap_enabled` | `boolean` | no |  |
| `seo_google_site_verification` | `string` | no | Solo el contenido de la meta etiqueta de verificación de Google. |
| `seo_canonical_url` | `string` | no |  |
| `tracking_integrations` | `object` | no | Rastreadores por proveedor (`google_tag_manager`, `google_analytics`, …): `{ proveedor: { enabled, ...ids } }`. Los proveedores y campos desconocidos se descartan. |
| `chat_integrations` | `object` | no | Widgets de chat por proveedor (`intercom`, `hubspot`, `zendesk`, …), con el mismo formato que `tracking_integrations`. |
| `custom_code` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.portal-settings.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Configuración del portal del workspace, con los valores predeterminados aplicados. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Rechazado por una regla de negocio (dirección en uso o reservada, archivo demasiado grande o de tipo no aceptado…). |

## portal-settings verify-sender

Verificar el e-mail remitente de las notificaciones

```text
kobana dashboards portal-settings verify-sender [OPCIONES]
```

`POST /v1/portal_settings/verify_sender`

Solicita o vuelve a comprobar la verificación en AWS SES del e-mail remitente de las notificaciones del portal. Cuando la dirección nunca se envió a SES o la verificación falló, pide la verificación (SES envía a la dirección un enlace); si no, consulta el estado actual. Devuelve `status` (`pending`, `active` o `failed`) y `verification_requested`. Sustituye a `POST /portal_settings/sender_identity`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Estado de la verificación. |
| `404` | Recurso no encontrado en la organización actual. |
