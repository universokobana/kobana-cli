---
id: imports
title: "kobana dashboards imports"
sidebar_label: "imports"
---

# kobana dashboards imports


Comandos de `kobana dashboards imports`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports` | Listar importaciones |
| [`imports create`](#imports-create) | `POST /v1/imports` | Importar usuarios del portal desde CSV |
| [`imports get`](#imports-get) | `GET /v1/imports/{id}` | Consultar importación |

## imports list

Listar importaciones

```text
kobana dashboards imports list [OPCIONES]
```

`GET /v1/imports`

Lista las solicitudes de importación del workspace, con contadores de éxito y error.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `resource_type` | `string` | no | Recurso importado, p. ej. `portal_users`. |
| `status` | `string` | no | Filtra por estado de la importación. Valores: `pending`, `processing`, `completed`, `completed_with_errors`, `failed`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de importaciones. |
| `404` | Recurso no encontrado en la organización actual. |

## imports create

Importar usuarios del portal desde CSV

```text
kobana dashboards imports create [OPCIONES]
```

`POST /v1/imports`

Envía un CSV en base64 y programa la importación de usuarios del portal en el workspace. El panel sube el archivo antes; la API lo recibe en el cuerpo (límite de 2 MB por solicitud). Sigue la configuración de la organización "permitir crear usuarios del portal" (403 si está desactivada). Asíncrono: 202; consulte `GET /imports/{id}`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `resource_type` | `string` | sí | Qué importar. Hoy: `portal_users`. Valores: `portal_users`. |
| `file_name` | `string` | sí | Nombre original del archivo, p. ej. `usuarios.csv`. |
| `content_base64` | `string` | sí | El CSV codificado en base64. |
| `content_type` | `string` | no | Tipo de archivo. Por defecto `text/csv`. |

Otros campos y objetos anidados: `kobana schema dashboards.imports.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Importación programada. |
| `400` | Cuerpo ausente, JSON inválido o campos incompatibles. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## imports get

Consultar importación

```text
kobana dashboards imports get [ID] [OPCIONES]
```

`GET /v1/imports/{id}`

Estado, contadores y errores por fila de una importación. Una importación aún `pending` 2 minutos después de la solicitud aparece como `failed` (cola detenida), sin que la consulta modifique el registro.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La importación. |
| `404` | Recurso no encontrado en la organización actual. |
