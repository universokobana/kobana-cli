---
id: team-roles
title: "kobana dashboards team-roles"
sidebar_label: "team-roles"
---

# kobana dashboards team-roles


Comandos de `kobana dashboards team-roles`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`team-roles list`](#team-roles-list) | `GET /v1/team_roles` | Listar roles del equipo |
| [`team-roles create`](#team-roles-create) | `POST /v1/team_roles` | Crear rol personalizado |
| [`team-roles update`](#team-roles-update) | `PUT /v1/team_roles` | Crear o actualizar un rol de equipo |
| [`team-roles get`](#team-roles-get) | `GET /v1/team_roles/{id}` | Obtener rol |
| [`team-roles delete`](#team-roles-delete) | `DELETE /v1/team_roles/{id}` | Eliminar rol |

## team-roles list

Listar roles del equipo

```text
kobana dashboards team-roles list [OPCIONES]
```

`GET /v1/team_roles`

Devuelve la lista paginada de roles configurados para el equipo de la organización. Incluye tanto roles de sistema (`is_system = true`) como roles personalizados.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda textual por nombre o slug. |
| `is_active` | `string` | no | Filtra por roles activos (`true`, el predeterminado), inactivos (`false`) o todos (`all`). Valores: `true`, `false`, `all`. |
| `is_system` | `boolean,null` | no | Filtra por roles de sistema (`true`) o personalizados (`false`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de roles. |
| `404` | Recurso no encontrado en la organización actual. |

## team-roles create

Crear rol personalizado

```text
kobana dashboards team-roles create [OPCIONES]
```

`POST /v1/team_roles`

Crea un rol personalizado. El `slug` debe empezar con una letra minúscula y contener solo letras minúsculas, dígitos y `_`. Con un token de persona, esta debe tener acceso a todos los workspaces de la organización (o ser propietaria): un rol vale en todos ellos. De lo contrario, 403.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del rol (mostrado en la interfaz). |
| `slug` | `string` | sí | Slug estable usado para referenciar el rol (ej.: `support_manager`). |
| `permissions` | `array<string>` | sí | Lista de slugs de permisos concedidos por el rol. |
| `description` | `string` | no | Descripción opcional de lo que concede el rol. |
| `is_active` | `boolean` | no | Indica si el rol está activo y puede ser asignado. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema dashboards.team-roles.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Rol creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## team-roles update

Crear o actualizar un rol de equipo

```text
kobana dashboards team-roles update [OPCIONES]
```

`PUT /v1/team_roles`

Localiza el rol por `id`, `public_id` o `external_id`. El `slug` no tiene restricción de unicidad y por lo tanto **no** identifica un registro. Si lo encuentra, lo actualiza; si no, lo crea y devuelve `201`. Con un token de persona, esta debe tener acceso a todos los workspaces de la organización (o ser propietaria): un rol vale en todos ellos. De lo contrario, 403.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre del rol (mostrado en la interfaz). |
| `slug` | `string` | no | Slug estable usado para referenciar el rol (ej.: `support_manager`). |
| `description` | `string` | no | Descripción opcional de lo que concede el rol. |
| `permissions` | `array<string>` | no | Lista de slugs de permisos concedidos por el rol. |
| `is_active` | `boolean` | no | Indica si el rol está activo y puede ser asignado. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema dashboards.team-roles.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## team-roles get

Obtener rol

```text
kobana dashboards team-roles get [ID] [OPCIONES]
```

`GET /v1/team_roles/{id}`

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del rol. |
| `404` | Recurso no encontrado en la organización actual. |

## team-roles delete

Eliminar rol

```text
kobana dashboards team-roles delete [ID] [OPCIONES]
```

`DELETE /v1/team_roles/{id}`

Realiza soft delete del rol. Los roles de sistema (`is_system = true`) no pueden eliminarse. Con un token de persona, esta debe tener acceso a todos los workspaces de la organización (o ser propietaria): un rol vale en todos ellos. De lo contrario, 403.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Rol eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
