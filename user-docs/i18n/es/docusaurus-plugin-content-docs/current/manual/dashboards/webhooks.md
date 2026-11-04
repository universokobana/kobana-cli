---
id: webhooks
title: "kobana dashboards webhooks"
sidebar_label: "webhooks"
---

# kobana dashboards webhooks


Comandos de `kobana dashboards webhooks`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`webhooks deliveries retry`](#webhooks-deliveries-retry) | `POST /v1/webhooks/{id}/deliveries/{delivery_id}/retry` | Reintentar entrega de webhook |
| [`webhooks deliveries list`](#webhooks-deliveries-list) | `GET /v1/webhooks/{id}/deliveries` | Listar entregas de un webhook |
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | Listar endpoints de webhook |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Crear endpoint de webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Consultar endpoint de webhook |
| [`webhooks update`](#webhooks-update) | `PATCH /v1/webhooks/{id}` | Actualizar endpoint de webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Eliminar endpoint de webhook |
| [`webhooks rotate-secret`](#webhooks-rotate-secret) | `POST /v1/webhooks/{id}/rotate_secret` | Rotar el secreto del webhook |
| [`webhooks test`](#webhooks-test) | `POST /v1/webhooks/{id}/test` | Enviar webhook de prueba |

## webhooks deliveries retry

Reintentar entrega de webhook

```text
kobana dashboards webhooks deliveries retry [ID] [DELIVERY_ID] [OPCIONES]
```

`POST /v1/webhooks/{id}/deliveries/{delivery_id}/retry`

Devuelve la entrega a `pending` y la encola para un nuevo envío. El envío es asíncrono: siga el resultado en el log de entregas.

**Argumentos de la ruta:** `id`, `delivery_id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Entrega encolada para reenvío. |
| `404` | Recurso no encontrado en la organización actual. |

## webhooks deliveries list

Listar entregas de un webhook

```text
kobana dashboards webhooks deliveries list [ID] [OPCIONES]
```

`GET /v1/webhooks/{id}/deliveries`

Log de entregas de un endpoint, de la más reciente a la más antigua.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | no | `pending`, `delivered` o `failed`. Valores: `pending`, `delivered`, `failed`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de entregas. |
| `404` | Recurso no encontrado en la organización actual. |

## webhooks list

Listar endpoints de webhook

```text
kobana dashboards webhooks list [OPCIONES]
```

`GET /v1/webhooks`

Lista los endpoints de webhook del workspace, del más reciente al más antiguo, con el total de entregas de cada uno.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | no | `active` o `inactive`. Valores: `active`, `inactive`. |
| `search` | `string` | no | Fragmento de la URL o de la descripción, sin distinguir mayúsculas. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de endpoints de webhook. |
| `404` | Recurso no encontrado en la organización actual. |

## webhooks create

Crear endpoint de webhook

```text
kobana dashboards webhooks create [OPCIONES]
```

`POST /v1/webhooks`

Crea un endpoint activo en el workspace. La respuesta trae el `secret` de firma — solo aparece aquí (y en la rotación).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `url` | `string` | sí | URL del endpoint (obligatoria en la creación). Debe ser una URL válida. |
| `events` | `array<string>` | sí | Tipos de evento a recibir. Al menos uno. |
| `description` | `string` | no | Descripción libre. `null` la limpia. |
| `auth_type` | `string` | no | Autenticación de las entregas (por defecto `none`). Al cambiarla, envíe también el `auth_config` correspondiente. Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | no | Credenciales del `auth_type`: basic → `{ username, password }`, bearer → `{ token }`, header → `{ key, value }`. Se guardan cifradas y nunca se devuelven. |

Otros campos y objetos anidados: `kobana schema dashboards.webhooks.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Endpoint creado, con el secreto de firma. |
| `400` | Solicitud inválida: cuerpo malformado, campo obligatorio ausente o filtro con valor inválido. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## webhooks get

Consultar endpoint de webhook

```text
kobana dashboards webhooks get [ID] [OPCIONES]
```

`GET /v1/webhooks/{id}`

Devuelve un endpoint de webhook del workspace. Las entregas están en `GET /webhooks/{id}/deliveries`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El endpoint de webhook. |
| `404` | Recurso no encontrado en la organización actual. |

## webhooks update

Actualizar endpoint de webhook

```text
kobana dashboards webhooks update [ID] [OPCIONES]
```

`PATCH /v1/webhooks/{id}`

Cambia la URL, la descripción, los eventos suscritos, `active` o la autenticación. Los campos ausentes quedan como están. Cuando la `url` cambia de origen, de ruta o de query string (el orden de los parámetros no cuenta) y `auth_type` no viene en la misma solicitud, las credenciales guardadas se descartan (`auth_type` pasa a `none`): fueron dadas para el destino anterior. El secreto de firma se mantiene — use `rotate_secret` si el endpoint cambió de dueño.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `url` | `string` | no | URL del endpoint (obligatoria en la creación). Debe ser una URL válida. |
| `events` | `array<string>` | no | Tipos de evento a recibir. Al menos uno. |
| `description` | `string` | no | Descripción libre. `null` la limpia. |
| `active` | `boolean` | no | `false` pausa el endpoint; `true` lo reanuda. |
| `auth_type` | `string` | no | Autenticación de las entregas (por defecto `none`). Al cambiarla, envíe también el `auth_config` correspondiente. Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | no | Credenciales del `auth_type`: basic → `{ username, password }`, bearer → `{ token }`, header → `{ key, value }`. Se guardan cifradas y nunca se devuelven. |

Otros campos y objetos anidados: `kobana schema dashboards.webhooks.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Endpoint actualizado. |
| `400` | Solicitud inválida: cuerpo malformado, campo obligatorio ausente o filtro con valor inválido. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## webhooks delete

Eliminar endpoint de webhook

```text
kobana dashboards webhooks delete [ID] [OPCIONES]
```

`DELETE /v1/webhooks/{id}`

Elimina el endpoint definitivamente, junto con el log de entregas. Los eventos dejan de enviarse de inmediato.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Endpoint eliminado. |
| `404` | Recurso no encontrado en la organización actual. |

## webhooks rotate-secret

Rotar el secreto del webhook

```text
kobana dashboards webhooks rotate-secret [ID] [OPCIONES]
```

`POST /v1/webhooks/{id}/rotate_secret`

Genera un nuevo secreto de firma. El anterior deja de valer de inmediato; el nuevo `secret` solo aparece en esta respuesta.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Endpoint con el nuevo secreto. |
| `404` | Recurso no encontrado en la organización actual. |

## webhooks test

Enviar webhook de prueba

```text
kobana dashboards webhooks test [ID] [OPCIONES]
```

`POST /v1/webhooks/{id}/test`

Envía ahora un evento `test.webhook` firmado a la URL del endpoint y lo registra en el log de entregas. Responde 200 con `success: false` cuando el endpoint responde fuera de 2xx o no se alcanza, y 422 cuando la URL apunta a una dirección bloqueada (red privada).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado del intento. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
