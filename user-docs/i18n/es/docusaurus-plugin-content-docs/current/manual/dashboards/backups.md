---
id: backups
title: "kobana dashboards backups"
sidebar_label: "backups"
---

# kobana dashboards backups


Comandos de `kobana dashboards backups`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`backups list`](#backups-list) | `GET /v1/backups` | Listar copias de seguridad |
| [`backups create`](#backups-create) | `POST /v1/backups` | Solicitar copia de seguridad |
| [`backups restore`](#backups-restore) | `POST /v1/backups/restore` | Restaurar copia de seguridad |
| [`backups get`](#backups-get) | `GET /v1/backups/{id}` | Consultar copia de seguridad |
| [`backups delete`](#backups-delete) | `DELETE /v1/backups/{id}` | Eliminar copia de seguridad |
| [`backups cancel`](#backups-cancel) | `POST /v1/backups/{id}/cancel` | Cancelar restauración |
| [`backups download`](#backups-download) | `GET /v1/backups/{id}/download` | Descargar copia de seguridad |

## backups list

Listar copias de seguridad

```text
kobana dashboards backups list [OPCIONES]
```

`GET /v1/backups`

Lista copias de seguridad y restauraciones de la organización (solo metadatos) e informa si puede pedirse una nueva copia ahora (`backup_allowed`, `next_backup_at` — una cada 12 horas en producción). Recurso de la organización.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `type` | `string` | no | `backup` o `restore`. Valores: `backup`, `restore`. |
| `status` | `string` | no | Filtra por estado. Valores: `pending`, `processing`, `completed`, `failed`, `canceled`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de copias de seguridad. |
| `404` | Recurso no encontrado en la organización actual. |

## backups create

Solicitar copia de seguridad

```text
kobana dashboards backups create [OPCIONES]
```

`POST /v1/backups`

Programa una copia completa (ZIP) de la organización, con todos los workspaces. Asíncrono: 202; consulte `GET /backups/{id}`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Copia programada. |
| `404` | Recurso no encontrado en la organización actual. |

## backups restore

Restaurar copia de seguridad

```text
kobana dashboards backups restore [OPCIONES]
```

`POST /v1/backups/restore`

DESTRUCTIVO: restaura una copia en el workspace de la llamada (`X-Workspace-Id`), reescribiendo usuarios del portal emparejados por correo/ID público. Envíe `backup_id` (una copia terminada de esta organización — el archivo se copia) O `file_name` (`.zip`) + `content_base64` (limitado a los 2 MB de la solicitud). Exige `settings.update` y acceso a todos los workspaces. Asíncrono: 202; sígala en `GET /backups/{id}` y cancele con `/cancel`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `backup_id` | `string (uuid)` | no | Copia terminada de esta organización a restaurar. |
| `file_name` | `string` | no | Nombre del archivo `.zip`, junto con `content_base64`. |
| `content_base64` | `string` | no | El ZIP de la copia en base64. |

Otros campos y objetos anidados: `kobana schema dashboards.backups.restore`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Restauración programada (registro de tipo `restore`). |
| `400` | Cuerpo ausente, JSON inválido o campos incompatibles. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## backups get

Consultar copia de seguridad

```text
kobana dashboards backups get [ID] [OPCIONES]
```

`GET /v1/backups/{id}`

Estado y progreso de una copia o restauración. Una restauración en curso cuyo job desapareció de la cola se marca como `failed`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La copia de seguridad. |
| `404` | Recurso no encontrado en la organización actual. |

## backups delete

Eliminar copia de seguridad

```text
kobana dashboards backups delete [ID] [OPCIONES]
```

`DELETE /v1/backups/{id}`

Elimina el registro y su archivo del almacenamiento. Definitivo.

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

## backups cancel

Cancelar restauración

```text
kobana dashboards backups cancel [ID] [OPCIONES]
```

`POST /v1/backups/{id}/cancel`

Detiene una restauración pendiente o en curso. Lo ya restaurado permanece. 422 para una copia o una restauración ya terminada.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La copia de seguridad. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## backups download

Descargar copia de seguridad

```text
kobana dashboards backups download [ID] [OPCIONES]
```

`GET /v1/backups/{id}/download`

Devuelve el ZIP de la copia. Exige `backups.export` y, para el token de una persona, ser dueña de la organización o tener acceso a todos los workspaces — la copia los contiene a todos. Queda en la auditoría.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El archivo ZIP. |
| `404` | Recurso no encontrado en la organización actual. |
