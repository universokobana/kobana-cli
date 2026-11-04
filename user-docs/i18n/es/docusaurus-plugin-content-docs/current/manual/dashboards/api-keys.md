---
id: api-keys
title: "kobana dashboards api-keys"
sidebar_label: "api-keys"
---

# kobana dashboards api-keys


Comandos de `kobana dashboards api-keys`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`api-keys list`](#api-keys-list) | `GET /v1/api_keys` | Listar claves de API |
| [`api-keys create`](#api-keys-create) | `POST /v1/api_keys` | Crear clave de API |
| [`api-keys get`](#api-keys-get) | `GET /v1/api_keys/{id}` | Consultar clave de API |
| [`api-keys update`](#api-keys-update) | `PATCH /v1/api_keys/{id}` | Actualizar clave de API |
| [`api-keys delete`](#api-keys-delete) | `DELETE /v1/api_keys/{id}` | Eliminar clave revocada |
| [`api-keys revoke`](#api-keys-revoke) | `POST /v1/api_keys/{id}/revoke` | Revocar clave de API |
| [`api-keys rotate`](#api-keys-rotate) | `POST /v1/api_keys/{id}/rotate` | Rotar clave de API |

## api-keys list

Listar claves de API

```text
kobana dashboards api-keys list [OPCIONES]
```

`GET /v1/api_keys`

Lista las claves de API (`kb_…`) de la organización, sin el secreto. Estas claves ya no autentican la API v1 ni el MCP — ambos usan tokens de acceso emitidos por el servidor de autorización de Kobana; los registros siguen como inventario de claves de la organización (ciclo de vida, avisos de vencimiento y webhooks `api_key.*`). Recurso de la organización: ignora `X-Workspace-Id`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra por estado: `active`, `disabled` o `revoked`. Valores: `active`, `disabled`, `revoked`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de claves de API. |
| `404` | Recurso no encontrado en la organización actual. |

## api-keys create

Crear clave de API

```text
kobana dashboards api-keys create [OPCIONES]
```

`POST /v1/api_keys`

Crea una clave activa, válida por un año salvo `expires_at`. El secreto (`key`) vuelve UNA sola vez en esta respuesta — solo se guarda su hash. Exige una persona que alcance todos los workspaces (dueña de la organización o con acceso a cada uno), como en el panel.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la clave (obligatorio). |
| `description` | `string` | no |  |
| `source` | `string` | no | Dónde se usará la clave. Por defecto `dashboard`. Valores: `api`, `dashboard`, `manage`. |
| `notification_emails` | `array<string>` | no | Correos que reciben los avisos de vencimiento. |
| `expires_at` | `string (date-time)` | no | Vencimiento (fecha-hora ISO 8601 en el futuro). Por defecto: dentro de un año. |

Otros campos y objetos anidados: `kobana schema dashboards.api-keys.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Clave creada, con el secreto en `key` (mostrado solo ahora). |
| `400` | Cuerpo ausente, JSON inválido o campos incompatibles. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## api-keys get

Consultar clave de API

```text
kobana dashboards api-keys get [ID] [OPCIONES]
```

`GET /v1/api_keys/{id}`

Devuelve una clave de la organización — metadatos y marcas del ciclo de vida, nunca el secreto.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La clave de API. |
| `404` | Recurso no encontrado en la organización actual. |

## api-keys update

Actualizar clave de API

```text
kobana dashboards api-keys update [ID] [OPCIONES]
```

`PATCH /v1/api_keys/{id}`

Edita nombre, descripción, origen, correos de aviso y vencimiento, y/o activa ⇄ desactiva la clave con `enabled`. Una clave revocada es definitiva (409).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no |  |
| `description` | `string` | no |  |
| `source` | `string` | no | Valores: `api`, `dashboard`, `manage`. |
| `notification_emails` | `array<string>` | no | Correos que reciben los avisos de vencimiento. |
| `expires_at` | `string (date-time)` | no | Nuevo vencimiento (ISO 8601); `null` quita el vencimiento. |
| `enabled` | `boolean` | no | `false` desactiva la clave; `true` la reactiva. |

Otros campos y objetos anidados: `kobana schema dashboards.api-keys.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La clave de API. |
| `400` | Cuerpo ausente, JSON inválido o campos incompatibles. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | El estado actual del recurso no permite esta operación. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## api-keys delete

Eliminar clave revocada

```text
kobana dashboards api-keys delete [ID] [OPCIONES]
```

`DELETE /v1/api_keys/{id}`

Quita de la lista una clave YA REVOCADA (borrado lógico; el historial se conserva). Una clave activa o desactivada debe revocarse antes (409).

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
| `409` | El estado actual del recurso no permite esta operación. |

## api-keys revoke

Revocar clave de API

```text
kobana dashboards api-keys revoke [ID] [OPCIONES]
```

`POST /v1/api_keys/{id}/revoke`

Revoca la clave de forma definitiva — no puede reactivarse. 409 si ya está revocada.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La clave de API. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | El estado actual del recurso no permite esta operación. |

## api-keys rotate

Rotar clave de API

```text
kobana dashboards api-keys rotate [ID] [OPCIONES]
```

`POST /v1/api_keys/{id}/rotate`

Emite una clave sucesora (mismo nombre y configuración) y mantiene la anterior funcionando durante `grace_days` días (por defecto 7, máximo 30). Solo una clave activa puede rotarse (409). El nuevo secreto vuelve UNA sola vez.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `grace_days` | `integer` | no | Días en que la clave anterior sigue valiendo (0 a 30, por defecto 7). |

Otros campos y objetos anidados: `kobana schema dashboards.api-keys.rotate`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La clave sucesora con el secreto en `key`, y `previous_key` con el plazo de gracia de la anterior. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | El estado actual del recurso no permite esta operación. |
| `422` | El cuerpo de la solicitud falló en la validación. |
