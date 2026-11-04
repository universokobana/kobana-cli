---
id: portal-users
title: "kobana dashboards portal-users"
sidebar_label: "portal-users"
---

# kobana dashboards portal-users


Comandos de `kobana dashboards portal-users`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`portal-users activity`](#portal-users-activity) | `GET /v1/portal_users/{id}/activity` | Consultar la actividad del usuario del portal |
| [`portal-users activity timeline`](#portal-users-activity-timeline) | `GET /v1/portal_users/{id}/activity/timeline` | Listar la línea de tiempo del usuario del portal |
| [`portal-users list`](#portal-users-list) | `GET /v1/portal_users` | Listar usuarios del portal |
| [`portal-users create`](#portal-users-create) | `POST /v1/portal_users` | Crear (invitar) usuario del portal |
| [`portal-users invite`](#portal-users-invite) | `POST /v1/portal_users/invite` | Invitar usuario del portal |
| [`portal-users get`](#portal-users-get) | `GET /v1/portal_users/{id}` | Obtener usuario del portal |
| [`portal-users update`](#portal-users-update) | `PATCH /v1/portal_users/{id}` | Actualizar usuario del portal |
| [`portal-users delete`](#portal-users-delete) | `DELETE /v1/portal_users/{id}` | Eliminar usuario del portal |
| [`portal-users email-preview`](#portal-users-email-preview) | `GET /v1/portal_users/{id}/email-preview` | Previsualizar correo del usuario del portal |
| [`portal-users resend`](#portal-users-resend) | `POST /v1/portal_users/{id}/resend` | Reenviar correo de invitación o verificación |

## portal-users activity

Consultar la actividad del usuario del portal

```text
kobana dashboards portal-users activity [ID] [OPCIONES] [COMANDO]
```

`GET /v1/portal_users/{id}/activity`

Analítica de uso de un usuario del portal: inicios de sesión y páginas visitadas (desde siempre), páginas distintas, la actividad más reciente, una serie diaria de visitas e inicios de sesión en los últimos `days` días (días de São Paulo) y las páginas más abiertas. Las entradas individuales están en `GET /portal_users/{id}/activity/timeline`.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `days` | `integer` | no | Días que cubre la `series`, hoy incluido (1 a 365). Por defecto: 30. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resumen de la actividad del usuario. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-users activity timeline

Listar la línea de tiempo del usuario del portal

```text
kobana dashboards portal-users activity timeline [ID] [OPCIONES]
```

`GET /v1/portal_users/{id}/activity/timeline`

Inicios de sesión (`kind: login`, con el `method` usado: `credentials`, `magic-link` o `sso`) y páginas abiertas (`kind: page_view`, con `path` y `page_name`), de la más reciente a la más antigua. Paginación por cursor: envíe `meta.next_before` como `before` para la página siguiente; `null` indica que no hay más. Los accesos de un operador viendo el portal como el usuario no cuentan.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `before` | `string` | no | Instante ISO 8601: solo entradas más antiguas que él. |
| `limit` | `integer` | no | Entradas por página (1 a 100). Por defecto: 20. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Una página de la línea de tiempo. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-users list

Listar usuarios del portal

```text
kobana dashboards portal-users list [OPCIONES]
```

`GET /v1/portal_users`

Devuelve los usuarios del portal del workspace, con paginación. Los campos sensibles (contraseña, tokens) nunca se exponen.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `tenant_id` | `string` | no | Restringe a los usuarios de un ID del titular (comparación exacta). |
| `status` | `string` | no | Filtra por estado: `active`, `pending_invitation`, `pending_verification`, `suspended` o `locked`. Valores: `active`, `pending_verification`, `pending_invitation`, `suspended`, `locked`. |
| `email` | `string` | no | Busca por fragmento del correo. |
| `external_id` | `string` | no | Filtra por el `external_id` exacto. |
| `sso_only` | `string` | no | Filtra por `sso_only` (`true` o `false`). Valores: `true`, `false`. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `last_login_after` | `string` | no | Último acceso a partir de esta fecha (ISO 8601). |
| `last_login_before` | `string` | no | Último acceso hasta esta fecha (ISO 8601). |
| `person_id` | `string` | no | Filtra por UUID de la persona asociada. |
| `person_document_number` | `string` | no | Filtra por CPF/CNPJ de la persona (coincidencia exacta). |
| `person_name_contains` | `string` | no | Filtra por nombre de la persona (búsqueda parcial, sin distinción de mayúsculas). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de usuarios del portal. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-users create

Crear (invitar) usuario del portal

```text
kobana dashboards portal-users create [OPCIONES]
```

`POST /v1/portal_users`

Crea el usuario en el workspace con estado `pending_invitation` y envía el correo de invitación. La invitación expira en 7 días.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string` | sí | Correo del usuario (obligatorio). Único por workspace, sin distinguir mayúsculas. |
| `tenant_id` | `string` | no | ID del titular de los registros que el usuario verá en el portal. Envíe `null` para limpiarlo. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `sso_only` | `boolean` | no | Cuando es `true`, el usuario solo entra por el SSO. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por la organización (clave/valor). |

Otros campos y objetos anidados: `kobana schema dashboards.portal-users.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Invitación creada y correo encolado para envío. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Ya existe un usuario o invitación para este correo en el workspace. |

## portal-users invite

Invitar usuario del portal

```text
kobana dashboards portal-users invite [OPCIONES]
```

`POST /v1/portal_users/invite`

Equivalente a `POST /portal_users`. Disponible como ruta de verbo para SDKs que prefieren URLs descriptivas.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string` | sí | Correo del usuario (obligatorio). Único por workspace, sin distinguir mayúsculas. |
| `tenant_id` | `string` | no | ID del titular de los registros que el usuario verá en el portal. Envíe `null` para limpiarlo. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `sso_only` | `boolean` | no | Cuando es `true`, el usuario solo entra por el SSO. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por la organización (clave/valor). |

Otros campos y objetos anidados: `kobana schema dashboards.portal-users.invite`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Invitación creada y correo encolado para envío. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Ya existe un usuario o invitación para este correo en el workspace. |

## portal-users get

Obtener usuario del portal

```text
kobana dashboards portal-users get [ID] [OPCIONES]
```

`GET /v1/portal_users/{id}`

El parámetro `{id}` acepta el UUID o el `external_id` del usuario.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del usuario del portal. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-users update

Actualizar usuario del portal

```text
kobana dashboards portal-users update [ID] [OPCIONES]
```

`PATCH /v1/portal_users/{id}`

Actualiza `email`, `tenant_id`, `external_id`, `sso_only`, `status` y `custom_metadata`. Transiciones de estado permitidas: `active` ↔ `suspended`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string` | no | Correo del usuario (obligatorio). Único por workspace, sin distinguir mayúsculas. |
| `tenant_id` | `string` | no | ID del titular de los registros que el usuario verá en el portal. Envíe `null` para limpiarlo. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `sso_only` | `boolean` | no | Cuando es `true`, el usuario solo entra por el SSO. |
| `status` | `string` | no | Transiciones permitidas: `active` ↔ `suspended`. Al reactivar, el usuario vuelve al estado que tenía antes de la suspensión (quien aún no había confirmado el correo sigue pendiente). Valores: `active`, `suspended`. |
| `unlock` | `boolean` | no | `true` termina el bloqueo de inicio de sesión (demasiadas contraseñas incorrectas): reinicia los intentos y permite entrar. 400 cuando el usuario no está bloqueado. Valores: `true`. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por la organización (clave/valor). |

Otros campos y objetos anidados: `kobana schema dashboards.portal-users.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Usuario actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Ya existe un usuario o invitación para este correo en el workspace. |

## portal-users delete

Eliminar usuario del portal

```text
kobana dashboards portal-users delete [ID] [OPCIONES]
```

`DELETE /v1/portal_users/{id}`

Las invitaciones pendientes se eliminan definitivamente; los demás usuarios se suspenden y sus sesiones se cierran. Un usuario ya suspendido no cambia. `outcome` indica lo que ocurrió.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Confirmación de eliminación. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-users email-preview

Previsualizar correo del usuario del portal

```text
kobana dashboards portal-users email-preview [ID] [OPCIONES]
```

`GET /v1/portal_users/{id}/email-preview`

Devuelve el asunto y el mensaje del correo de invitación o verificación que se enviaría al usuario, sin disparar el envío. Disponible solo para usuarios `pending_invitation` o `pending_verification`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Previsualización del correo. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-users resend

Reenviar correo de invitación o verificación

```text
kobana dashboards portal-users resend [ID] [OPCIONES]
```

`POST /v1/portal_users/{id}/resend`

Reenvía el correo de invitación (para `pending_invitation`, con rotación de token) o de verificación (para `pending_verification`). Otros estados devuelven 400.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Correo reenviado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
