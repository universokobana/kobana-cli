---
id: portal-oauth-providers
title: "kobana dashboards portal-oauth-providers"
sidebar_label: "portal-oauth-providers"
---

# kobana dashboards portal-oauth-providers


Comandos de `kobana dashboards portal-oauth-providers`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`portal-oauth-providers list`](#portal-oauth-providers-list) | `GET /v1/portal_oauth_providers` | Listar proveedores OIDC |
| [`portal-oauth-providers create`](#portal-oauth-providers-create) | `POST /v1/portal_oauth_providers` | Agregar proveedor OIDC |
| [`portal-oauth-providers test`](#portal-oauth-providers-test) | `POST /v1/portal_oauth_providers/test` | Probar emisor OIDC |
| [`portal-oauth-providers update`](#portal-oauth-providers-update) | `PATCH /v1/portal_oauth_providers/{id}` | Actualizar proveedor OIDC |
| [`portal-oauth-providers delete`](#portal-oauth-providers-delete) | `DELETE /v1/portal_oauth_providers/{id}` | Eliminar proveedor OIDC |

## portal-oauth-providers list

Listar proveedores OIDC

```text
kobana dashboards portal-oauth-providers list [OPCIONES]
```

`GET /v1/portal_oauth_providers`

Lista los proveedores OpenID Connect con los que los usuarios entran al portal (conjunto completo, sin paginación).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Proveedores OIDC del portal. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-oauth-providers create

Agregar proveedor OIDC

```text
kobana dashboards portal-oauth-providers create [OPCIONES]
```

`POST /v1/portal_oauth_providers`

Agrega un proveedor OpenID Connect al inicio de sesión del portal.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí |  |
| `issuer_url` | `string (uri)` | sí | URL del emisor OpenID Connect (https en producción). |
| `client_id` | `string` | sí |  |
| `client_secret` | `string` | sí | Solo escritura: se guarda cifrado y nunca se devuelve. |
| `button_label` | `string` | no | Texto del botón de inicio de sesión; si está vacío, usa el nombre. |
| `scopes` | `string` | no | Ámbitos separados por espacio. Predeterminado: `openid email profile`. |
| `email_claim` | `string` | no | Claim del token que trae el correo. Predeterminado: `email`. |
| `enabled` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.portal-oauth-providers.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | El proveedor (sin el secret). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-oauth-providers test

Probar emisor OIDC

```text
kobana dashboards portal-oauth-providers test [OPCIONES]
```

`POST /v1/portal_oauth_providers/test`

Obtiene el documento de descubrimiento del emisor e informa sus endpoints, para validar la URL antes de guardar. Una falla en el descubrimiento es un 200 con `ok: false`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `issuer_url` | `string (uri)` | sí | URL del emisor OpenID Connect (https en producción). |

Otros campos y objetos anidados: `kobana schema dashboards.portal-oauth-providers.test`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la prueba. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-oauth-providers update

Actualizar proveedor OIDC

```text
kobana dashboards portal-oauth-providers update [ID] [OPCIONES]
```

`PATCH /v1/portal_oauth_providers/{id}`

Actualiza un proveedor OpenID Connect. Parcial: solo cambian los campos enviados.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no |  |
| `button_label` | `string` | no |  |
| `issuer_url` | `string (uri)` | no |  |
| `client_id` | `string` | no |  |
| `client_secret` | `string` | no | Omitido o vacío mantiene el secret actual; un valor lo reemplaza. Nunca se devuelve. Obligatorio cuando cambia `issuer_url` (otro host, puerto o ruta): sin él la actualización se rechaza con 422 `CLIENT_SECRET_REQUIRED_FOR_ISSUER_CHANGE`, para que el secret guardado no se envíe a otro proveedor. |
| `scopes` | `string` | no |  |
| `email_claim` | `string` | no |  |
| `enabled` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.portal-oauth-providers.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El proveedor (sin el secret). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## portal-oauth-providers delete

Eliminar proveedor OIDC

```text
kobana dashboards portal-oauth-providers delete [ID] [OPCIONES]
```

`DELETE /v1/portal_oauth_providers/{id}`

Elimina el proveedor; su botón desaparece del inicio de sesión y nadie más puede entrar por él.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
