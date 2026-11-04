---
id: dashboard-users
title: "kobana billing dashboard-users"
sidebar_label: "dashboard-users"
---

# kobana billing dashboard-users


Comandos de `kobana billing dashboard-users`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`dashboard-users list`](#dashboard-users-list) | `GET /v1/dashboard_users` | Listar usuarios del dashboard |
| [`dashboard-users create`](#dashboard-users-create) | `POST /v1/dashboard_users` | Invitar usuario del dashboard |
| [`dashboard-users update`](#dashboard-users-update) | `PUT /v1/dashboard_users` | Crea o actualiza un usuario del panel |
| [`dashboard-users get`](#dashboard-users-get) | `GET /v1/dashboard_users/{id}` | Obtener usuario del dashboard |
| [`dashboard-users delete`](#dashboard-users-delete) | `DELETE /v1/dashboard_users/{id}` | Eliminar usuario del dashboard |

## dashboard-users list

Listar usuarios del dashboard

```text
kobana billing dashboard-users list [OPCIONES]
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

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de usuarios del dashboard. |
| `404` | Recurso no encontrado en la organización actual. |

## dashboard-users create

Invitar usuario del dashboard

```text
kobana billing dashboard-users create [OPCIONES]
```

`POST /v1/dashboard_users`

Invita a un nuevo miembro al equipo de la organización. Es obligatorio informar `email` y al menos un rol en `role_ids`. El usuario recibe una invitación por correo y entra con estado `pending_invitation`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string (email)` | sí | Correo del usuario a invitar. |
| `role_ids` | `array<string (uuid)>` | sí | Lista de UUIDs de roles (`team_roles`) a asignar al usuario. Debe contener al menos un ítem. |
| `name` | `string` | no | Nombre del usuario (opcional — el usuario puede completarlo al primer acceso). |
| `external_id` | `string` | no | Identificador externo del usuario en Kobana OAuth, cuando ya se conoce. |
| `permissions` | `array<string>` | no | Permisos ad-hoc adicionales (además de los heredados de los roles). Acepta los mismos slugs de permisos. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.dashboard-users.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Usuario invitado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## dashboard-users update

Crea o actualiza un usuario del panel

```text
kobana billing dashboard-users update [OPCIONES]
```

`PUT /v1/dashboard_users`

Localiza al usuario por `id`, `public_id`, `external_id` o `email` — este último es único por organización. Si lo encuentra, lo actualiza; si no, lo crea y devuelve `201`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string (email)` | no | Correo del usuario. Además de campo del recurso, sirve como clave natural de búsqueda. |
| `name` | `string` | no | Nombre del usuario (opcional — el usuario puede completarlo al primer acceso). |
| `external_id` | `string` | no | Identificador externo del usuario en Kobana OAuth, cuando ya se conoce. |
| `role_ids` | `array<string (uuid)>` | no | Lista de UUIDs de roles (`team_roles`) a asignar al usuario. Debe contener al menos un ítem. |
| `permissions` | `array<string>` | no | Permisos ad-hoc adicionales (además de los heredados de los roles). Acepta los mismos slugs de permisos. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.dashboard-users.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## dashboard-users get

Obtener usuario del dashboard

```text
kobana billing dashboard-users get [ID] [OPCIONES]
```

`GET /v1/dashboard_users/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del usuario del dashboard. |
| `404` | Recurso no encontrado en la organización actual. |

## dashboard-users delete

Eliminar usuario del dashboard

```text
kobana billing dashboard-users delete [ID] [OPCIONES]
```

`DELETE /v1/dashboard_users/{id}`

Elimina al usuario del dashboard de la organización. Acción irreversible — el usuario debe ser invitado de nuevo para recuperar el acceso.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Usuario eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
