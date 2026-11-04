---
id: notifications
title: "kobana dashboards notifications"
sidebar_label: "notifications"
---

# kobana dashboards notifications


Comandos de `kobana dashboards notifications`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`notifications stats`](#notifications-stats) | `GET /v1/notifications/stats` | Estadísticas de notificaciones |
| [`notifications list`](#notifications-list) | `GET /v1/notifications` | Listar notificaciones |
| [`notifications create`](#notifications-create) | `POST /v1/notifications` | Crear notificación |
| [`notifications send`](#notifications-send) | `POST /v1/notifications/{id}/send` | Enviar notificación |

## notifications stats

Estadísticas de notificaciones

```text
kobana dashboards notifications stats [OPCIONES]
```

`GET /v1/notifications/stats`

Conteos por estado, tipo y canal de las notificaciones creadas en los últimos `days` días, con las tasas de entrega y de lectura.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `days` | `integer` | no | Ventana en días, de 1 a 365. Por defecto 30. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Las estadísticas. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## notifications list

Listar notificaciones

```text
kobana dashboards notifications list [OPCIONES]
```

`GET /v1/notifications`

Lista las notificaciones del workspace y las de toda la organización, de las más nuevas a las más antiguas.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `type` | `string` | no | Solo este tipo de notificación. Valores: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `channel` | `string` | no | Solo este canal. Valores: `email`, `in_app`, `webhook`. |
| `status` | `string` | no | Solo este estado. Valores: `pending`, `sent`, `delivered`, `failed`, `read`, `skipped`. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `search` | `string` | no | Fragmento del destinatario o del asunto, sin distinguir mayúsculas. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de notificaciones. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## notifications create

Crear notificación

```text
kobana dashboards notifications create [OPCIONES]
```

`POST /v1/notifications`

Renderiza la plantilla activa del tipo con las variables, guarda la notificación y la envía en el momento — la respuesta trae el resultado (`sent`, o `failed` con `failure_reason`). Con un `scheduled_for` en el futuro, queda `pending` hasta la hora marcada. Cada creación cuenta en el límite de envíos manuales por destinatario (429).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `type` | `string` | sí | Tipo de notificación (obligatorio): elige la plantilla activa de la organización, o la predeterminada de la plataforma. Valores: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `recipient` | `string` | sí | Dirección de destino (obligatoria) — un correo en el canal `email`. |
| `channel` | `string` | no | Canal de envío. Por defecto `email`. Valores: `email`, `in_app`, `webhook`. |
| `portal_user_id` | `string (uuid)` | no | Usuario del portal al que se destina la notificación; debe ser de la organización (si no, 404). |
| `resource_type` | `string` | no | Tipo del registro relacionado. |
| `resource_id` | `string (uuid)` | no | ID (UUID) del registro relacionado. |
| `scheduled_for` | `string (date-time)` | no | Fecha y hora ISO 8601 para enviar, en lugar de inmediatamente. |
| `variables` | `object` | no | Valores de las variables de la plantilla: hasta 100, nombres con letras, dígitos, `_`, `.` o `-`; valores texto (hasta 10.000 caracteres), número, booleano o `null`. |

Otros campos y objetos anidados: `kobana schema dashboards.notifications.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Notificación creada. |
| `400` | Solicitud inválida (campo obligatorio ausente, tipo o variables inválidos). |
| `404` | Recurso no encontrado en la organización actual. |

## notifications send

Enviar notificación

```text
kobana dashboards notifications send [ID] [OPCIONES]
```

`POST /v1/notifications/{id}/send`

Envía ahora una notificación guardada: el primer envío de una `pending` (programada) o `skipped`, o el reenvío de una `sent`, `delivered`, `read` o `failed`. Cada llamada envía el correo de nuevo y cuenta en el límite de envíos manuales por destinatario (429).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La notificación actualizada. |
| `400` | Solicitud inválida (campo obligatorio ausente, tipo o variables inválidos). |
| `404` | Recurso no encontrado en la organización actual. |
| `502` | El proveedor de correo rechazó el envío; el motivo viene en el mensaje. |
