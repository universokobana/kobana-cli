---
id: checklists
title: "kobana dashboards checklists"
sidebar_label: "checklists"
---

# kobana dashboards checklists


Comandos de `kobana dashboards checklists`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`checklists progress`](#checklists-progress) | `GET /v1/checklists/{id}/progress` | Ver progreso del tenant |
| [`checklists progress file`](#checklists-progress-file) | `GET /v1/checklists/{id}/progress/file` | Descargar archivo de la tarea |
| [`checklists progress file create`](#checklists-progress-file-create) | `POST /v1/checklists/{id}/progress/file` | Enviar archivo de la tarea |
| [`checklists progress file delete`](#checklists-progress-file-delete) | `DELETE /v1/checklists/{id}/progress/file` | Quitar archivo de la tarea |
| [`checklists progress update`](#checklists-progress-update) | `PUT /v1/checklists/{id}/progress` | Cambiar progreso del tenant |
| [`checklists progress delete`](#checklists-progress-delete) | `DELETE /v1/checklists/{id}/progress` | Reiniciar progreso del tenant |
| [`checklists list`](#checklists-list) | `GET /v1/checklists` | Listar checklists |
| [`checklists get`](#checklists-get) | `GET /v1/checklists/{id}` | Ver checklist |

## checklists progress

Ver progreso del tenant

```text
kobana dashboards checklists progress [ID] [OPCIONES] [COMANDO]
```

`GET /v1/checklists/{id}/progress`

Dónde está un tenant en una checklist, tarea por tarea, con completadas/total por etapa. Todos los usuarios del portal del mismo tenant ven este mismo progreso.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal dueños del progreso — todos los usuarios del mismo tenant ven la misma checklist. Indique este o `portal_user_id`. |
| `portal_user_id` | `string` | no | Un usuario del portal (id o external_id): vale el progreso de su tenant (un usuario sin tenant tiene una checklist propia). Indique este o `tenant_id`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El progreso del tenant. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## checklists progress file

Descargar archivo de la tarea

```text
kobana dashboards checklists progress file [ID] [OPCIONES] [COMANDO]
```

`GET /v1/checklists/{id}/progress/file`

El archivo que el tenant envió a una tarea de tipo Solicitud.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal dueños del progreso — todos los usuarios del mismo tenant ven la misma checklist. Indique este o `portal_user_id`. |
| `portal_user_id` | `string` | no | Un usuario del portal (id o external_id): vale el progreso de su tenant (un usuario sin tenant tiene una checklist propia). Indique este o `tenant_id`. |
| `task_id` | `string` | sí | ID de la tarea. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El contenido del archivo. |
| `404` | Recurso no encontrado en la organización actual. |

## checklists progress file create

Enviar archivo de la tarea

```text
kobana dashboards checklists progress file create [ID] [OPCIONES]
```

`POST /v1/checklists/{id}/progress/file`

Envía, por el tenant, el archivo de una tarea de tipo Solicitud (`content_base64`), reemplazando el anterior.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `task_id` | `string` | sí |  |
| `name` | `string` | sí |  |
| `content_base64` | `string` | sí | El contenido del archivo en base64 (hasta el límite de 2 MB de la solicitud). |
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal dueños del progreso — todos los usuarios del mismo tenant ven la misma checklist. Indique este o `portal_user_id`. |
| `portal_user_id` | `string` | no | Un usuario del portal (id o external_id): vale el progreso de su tenant (un usuario sin tenant tiene una checklist propia). Indique este o `tenant_id`. |
| `content_type` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.checklists.progress.file.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | El archivo registrado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## checklists progress file delete

Quitar archivo de la tarea

```text
kobana dashboards checklists progress file delete [ID] [OPCIONES]
```

`DELETE /v1/checklists/{id}/progress/file`

Retira el archivo que el tenant envió a una tarea.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal dueños del progreso — todos los usuarios del mismo tenant ven la misma checklist. Indique este o `portal_user_id`. |
| `portal_user_id` | `string` | no | Un usuario del portal (id o external_id): vale el progreso de su tenant (un usuario sin tenant tiene una checklist propia). Indique este o `tenant_id`. |
| `task_id` | `string` | sí | ID de la tarea. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Quitar archivo de la tarea |
| `404` | Recurso no encontrado en la organización actual. |

## checklists progress update

Cambiar progreso del tenant

```text
kobana dashboards checklists progress update [ID] [OPCIONES]
```

`PUT /v1/checklists/{id}/progress`

Define el estado de una o más tareas para el tenant (crea el progreso en el primer uso). Los usuarios del tenant ven el cambio en el portal.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `task_ids` | `array<string>` | sí |  |
| `status` | `string` | sí | `pending`, `in_progress` o `done`. Una tarea que nadie tocó es `pending`. Valores: `pending`, `in_progress`, `done`. |
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal dueños del progreso — todos los usuarios del mismo tenant ven la misma checklist. Indique este o `portal_user_id`. |
| `portal_user_id` | `string` | no | Un usuario del portal (id o external_id): vale el progreso de su tenant (un usuario sin tenant tiene una checklist propia). Indique este o `tenant_id`. |

Otros campos y objetos anidados: `kobana schema dashboards.checklists.progress.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El progreso del tenant. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## checklists progress delete

Reiniciar progreso del tenant

```text
kobana dashboards checklists progress delete [ID] [OPCIONES]
```

`DELETE /v1/checklists/{id}/progress`

Vuelve las tareas de `task_ids` (todas, sin él) a pendiente y olvida los archivos enviados. Es una actualización: el historial queda en los eventos y la auditoría.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `tenant_id` | `string` | no | Tenant ID de los usuarios del portal dueños del progreso — todos los usuarios del mismo tenant ven la misma checklist. Indique este o `portal_user_id`. |
| `portal_user_id` | `string` | no | Un usuario del portal (id o external_id): vale el progreso de su tenant (un usuario sin tenant tiene una checklist propia). Indique este o `tenant_id`. |
| `task_ids` | `string` | no | IDs de las tareas, separados por coma. Sin él, todas las tareas de la checklist. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El progreso del tenant. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## checklists list

Listar checklists

```text
kobana dashboards checklists list [OPCIONES]
```

`GET /v1/checklists`

Los bloques Lista de Tareas del portal publicado del workspace, página por página. Las tareas se escriben en el builder; el progreso es de cada tenant (`GET /checklists/{id}/progress`).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Las checklists. |
| `404` | Recurso no encontrado en la organización actual. |

## checklists get

Ver checklist

```text
kobana dashboards checklists get [ID] [OPCIONES]
```

`GET /v1/checklists/{id}`

Una checklist del portal publicado: etapas y tareas visibles.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La checklist. |
| `404` | Recurso no encontrado en la organización actual. |
