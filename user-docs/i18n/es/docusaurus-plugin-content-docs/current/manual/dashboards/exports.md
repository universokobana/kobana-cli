---
id: exports
title: "kobana dashboards exports"
sidebar_label: "exports"
---

# kobana dashboards exports


Comandos de `kobana dashboards exports`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`exports list`](#exports-list) | `GET /v1/exports` | Listar exportaciones |
| [`exports create`](#exports-create) | `POST /v1/exports` | Solicitar exportación |
| [`exports get`](#exports-get) | `GET /v1/exports/{id}` | Consultar exportación |
| [`exports download`](#exports-download) | `GET /v1/exports/{id}/download` | Descargar archivo de la exportación |

## exports list

Listar exportaciones

```text
kobana dashboards exports list [OPCIONES]
```

`GET /v1/exports`

Lista las solicitudes de exportación del workspace.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `resource_type` | `string` | no | Recurso exportado, p. ej. `portal_users`. |
| `status` | `string` | no | Filtra por estado de la exportación. Valores: `pending`, `processing`, `completed`, `failed`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de exportaciones. |
| `404` | Recurso no encontrado en la organización actual. |

## exports create

Solicitar exportación

```text
kobana dashboards exports create [OPCIONES]
```

`POST /v1/exports`

Programa la exportación de los usuarios del portal del workspace en CSV, XLSX o JSON. Asíncrono: responde 202 con la exportación `pending`; consulte `GET /exports/{id}` hasta `completed` y descargue en `/download`. El archivo vence en 7 días.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `resource_type` | `string` | sí | Qué exportar. Hoy: `portal_users`. Valores: `portal_users`. |
| `format` | `string` | no | `csv` (por defecto), `xlsx` o `json`. Valores: `csv`, `xlsx`, `json`. |
| `filters` | `object` | no | Filtros opcionales de las filas exportadas. |

Otros campos y objetos anidados: `kobana schema dashboards.exports.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Exportación programada. |
| `400` | Cuerpo ausente, JSON inválido o campos incompatibles. |
| `404` | Recurso no encontrado en la organización actual. |

## exports get

Consultar exportación

```text
kobana dashboards exports get [ID] [OPCIONES]
```

`GET /v1/exports/{id}`

Estado y metadatos de una solicitud de exportación.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La exportación. |
| `404` | Recurso no encontrado en la organización actual. |

## exports download

Descargar archivo de la exportación

```text
kobana dashboards exports download [ID] [OPCIONES]
```

`GET /v1/exports/{id}/download`

Devuelve el archivo de una exportación terminada (el panel redirige a una URL firmada; la API entrega los bytes). 404 mientras no esté lista.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El archivo, como adjunto. |
| `404` | Recurso no encontrado en la organización actual. |
