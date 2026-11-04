---
id: dashboard-users
title: "kobana dashboards dashboard-users"
sidebar_label: "dashboard-users"
---

# kobana dashboards dashboard-users


Comandos de `kobana dashboards dashboard-users`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`dashboard-users workspaces`](#dashboard-users-workspaces) | `GET /v1/dashboard_users/{id}/workspaces` | Consultar los workspaces de un miembro |
| [`dashboard-users workspaces update`](#dashboard-users-workspaces-update) | `PUT /v1/dashboard_users/{id}/workspaces` | Definir los workspaces de un miembro |
| [`dashboard-users list`](#dashboard-users-list) | `GET /v1/dashboard_users` | Listar usuarios del dashboard |
| [`dashboard-users create`](#dashboard-users-create) | `POST /v1/dashboard_users` | Invitar usuario del dashboard |
| [`dashboard-users update`](#dashboard-users-update) | `PUT /v1/dashboard_users` | Crear o actualizar un usuario del panel |
| [`dashboard-users get`](#dashboard-users-get) | `GET /v1/dashboard_users/{id}` | Obtener usuario del dashboard |
| [`dashboard-users delete`](#dashboard-users-delete) | `DELETE /v1/dashboard_users/{id}` | Eliminar usuario del dashboard |
| [`dashboard-users resend`](#dashboard-users-resend) | `POST /v1/dashboard_users/{id}/resend` | Reenviar invitación de miembro |

## dashboard-users workspaces

Consultar los workspaces de un miembro

```text
kobana dashboards dashboard-users workspaces [ID] [OPCIONES] [COMANDO]
```

`GET /v1/dashboard_users/{id}/workspaces`

Lista todos los workspaces de la organización, marcando a los que el miembro accede — también es la lista de lo que se puede conceder.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Workspaces de la organización con el acceso del miembro. |
| `404` | Recurso no encontrado en la organización actual. |

## dashboard-users workspaces update

Definir los workspaces de un miembro

```text
kobana dashboards dashboard-users workspaces update [ID] [OPCIONES]
```

`PUT /v1/dashboard_users/{id}/workspaces`

Reemplaza el conjunto de workspaces del miembro. Nadie cambia su propio acceso (400), solo un dueño cambia el acceso de un dueño, y solo un dueño concede o revoca un workspace al que él mismo no accede (403). Un token de máquina puede conceder cualquier workspace, pero nunca cambia el acceso de un dueño. Quitar al último dueño de un workspace responde 403.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `workspace_ids` | `array<string (uuid)>` | sí | El conjunto final de workspaces a los que el miembro debe acceder. Los quitados se pausan, no se borran. |

Otros campos y objetos anidados: `kobana schema dashboards.dashboard-users.workspaces.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Workspaces de la organización con el acceso del miembro. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## dashboard-users list

Listar usuarios del dashboard

```text
kobana dashboards dashboard-users list [OPCIONES]
```

`GET /v1/dashboard_users`

Devuelve una lista paginada de los usuarios del dashboard (miembros del equipo) de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda textual por nombre o correo. |
| `status` | `string` | no | Filtra por estado: `all` (por defecto), `active`, `pending_invitation` o `suspended`. Valores: `all`, `active`, `pending_invitation`, `suspended`. |
| `role` | `string` | no | Filtra por los usuarios que tienen el rol indicado (slug del rol). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de usuarios del dashboard. |
| `404` | Recurso no encontrado en la organización actual. |

## dashboard-users create

Invitar usuario del dashboard

```text
kobana dashboards dashboard-users create [OPCIONES]
```

`POST /v1/dashboard_users`

Invita a un nuevo miembro al equipo de la organización. Es obligatorio informar `email` y al menos un rol en `role_ids`. El usuario recibe una invitación por correo y entra con estado `pending_invitation`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string (email)` | sí | Correo del usuario a invitar. |
| `role_ids` | `array<string (uuid)>` | sí | Lista de UUIDs de roles (`team_roles`) a asignar al usuario. Debe contener al menos un ítem. |
| `name` | `string` | no | Nombre del usuario (opcional — el usuario puede completarlo al primer acceso). |
| `external_id` | `string` | no | Identificador externo del usuario, guardado como atributo del miembro. No define la identidad del usuario: siempre se resuelve por `email`. |
| `permissions` | `array<string>` | no | Permisos ad-hoc adicionales (además de los heredados de los roles). Acepta los mismos slugs de permisos. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema dashboards.dashboard-users.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Usuario invitado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Datos inválidos. Con el código `IDENTITY_CONFLICT`, el `external_id` informado es el identificador de otra identidad, con otro correo. |

## dashboard-users update

Crear o actualizar un usuario del panel

```text
kobana dashboards dashboard-users update [OPCIONES]
```

`PUT /v1/dashboard_users`

Localiza al usuario por `id`, `public_id`, `external_id` o `email` — este último es único por organización. Si lo encuentra, lo actualiza; si no, lo crea y devuelve `201`. Al actualizar, con un token de persona, esta debe tener acceso a todos los workspaces en los que trabaja el miembro (o ser propietaria de la organización); de lo contrario, 403.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string (email)` | no | Correo del usuario. Además de campo del recurso, sirve como clave natural de búsqueda. |
| `name` | `string` | no | Nombre del usuario (opcional — el usuario puede completarlo al primer acceso). |
| `external_id` | `string` | no | Identificador externo del usuario, guardado como atributo del miembro. No define la identidad del usuario: siempre se resuelve por `email`. |
| `role_ids` | `array<string (uuid)>` | no | Lista de UUIDs de roles (`team_roles`) a asignar al usuario. Debe contener al menos un ítem. |
| `permissions` | `array<string>` | no | Permisos ad-hoc adicionales (además de los heredados de los roles). Acepta los mismos slugs de permisos. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema dashboards.dashboard-users.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | Datos inválidos. Con el código `IDENTITY_CONFLICT`, el `external_id` informado es el identificador de otra identidad, con otro correo. |

## dashboard-users get

Obtener usuario del dashboard

```text
kobana dashboards dashboard-users get [ID] [OPCIONES]
```

`GET /v1/dashboard_users/{id}`

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del usuario del dashboard. |
| `404` | Recurso no encontrado en la organización actual. |

## dashboard-users delete

Eliminar usuario del dashboard

```text
kobana dashboards dashboard-users delete [ID] [OPCIONES]
```

`DELETE /v1/dashboard_users/{id}`

Elimina al usuario del dashboard de la organización. Acción irreversible — el usuario debe ser invitado de nuevo para recuperar el acceso. Con un token de persona, esta debe tener acceso a todos los workspaces en los que trabaja el miembro (o ser propietaria de la organización); de lo contrario, 403.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Usuario eliminado. |
| `404` | Recurso no encontrado en la organización actual. |

## dashboard-users resend

Reenviar invitación de miembro

```text
kobana dashboards dashboard-users resend [ID] [OPCIONES]
```

`POST /v1/dashboard_users/{id}/resend`

Reenvía el correo de invitación, con un nuevo enlace de acceso, a un miembro cuya invitación sigue pendiente. Limitado a 5 envíos por hora por destinatario (429 por encima de eso).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Invitación reenviada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El miembro no tiene una invitación pendiente (`INVALID_STATUS`). |
| `502` | No se pudo enviar el correo (`EMAIL_ERROR`). |
