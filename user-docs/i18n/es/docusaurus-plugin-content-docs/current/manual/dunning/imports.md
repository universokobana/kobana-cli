---
id: imports
title: "kobana dunning imports"
sidebar_label: "imports"
---

# kobana dunning imports


Comandos de `kobana dunning imports`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports` | Listar importaciones |
| [`imports create`](#imports-create) | `POST /v1/imports` | Crear importación |
| [`imports get`](#imports-get) | `GET /v1/imports/{id}` | Consultar importación |

## imports list

Listar importaciones

```text
kobana dunning imports list [OPCIONES]
```

`GET /v1/imports`

Historial de las 50 importaciones más recientes de la organización (sin paginación). El contenido del archivo y los errores por línea quedan fuera; consulta GET /imports/\{id\} para el detalle.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de las importaciones más recientes. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## imports create

Crear importación

```text
kobana dunning imports create [OPCIONES]
```

`POST /v1/imports`

Sube una planilla CSV de clientes o cobros y agenda el procesamiento asíncrono (cola). El cuerpo es JSON con el CSV en texto plano en el campo `content` (máx. 2 MB). Las líneas inválidas no tumban el lote: se registran en `errors` (`[{ line, error }]`) y el resto se procesa. Sigue el avance con GET /imports/\{id\}. Responde 202. Exige el nivel de acceso `import` (o `all`) — `write` no basta.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `resource_type` | `string` | sí | Tipo de recurso importado (`people` = clientes \| `charges` = cobros). Valores: `people`, `charges`. |
| `content` | `string` | sí | Contenido del CSV como texto plano (UTF-8), incluyendo la línea de encabezado. Máx. 2 MB. |
| `file_name` | `string` | no | Nombre del archivo enviado. |

Otros campos y objetos anidados: `kobana schema dunning.imports.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Importación aceptada y encolada para procesamiento. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Archivo mayor a 2 MB — divide la planilla en partes más pequeñas. |

## imports get

Consultar importación

```text
kobana dunning imports get [ID] [OPCIONES]
```

`GET /v1/imports/{id}`

Devuelve el estado de una importación con contadores de progreso y los errores por línea rechazada (`errors: [{ line, error }]`; la línea 1 es el encabezado, los datos empiezan en la línea 2).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Importación encontrada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
