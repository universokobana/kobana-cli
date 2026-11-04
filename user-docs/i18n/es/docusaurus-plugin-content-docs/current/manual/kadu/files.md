---
id: files
title: "kobana kadu files"
sidebar_label: "files"
---

# kobana kadu files


Comandos de `kobana kadu files`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`files list`](#files-list) | `GET /v1/files` | Lista archivos |
| [`files create`](#files-create) | `POST /v1/files` | Inicia la carga de un archivo de entrada |
| [`files get`](#files-get) | `GET /v1/files/{id}` | Detalla el archivo |
| [`files confirm`](#files-confirm) | `POST /v1/files/{id}/confirm` | Confirma la carga (pending → uploaded) |
| [`files download`](#files-download) | `GET /v1/files/{id}/download` | URL temporal de descarga |

## files list

Lista archivos

```text
kobana kadu files list [OPCIONES]
```

`GET /v1/files`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Campo en snake_case: created_at (predeterminado), updated_at o un campo filtrable. |
| `sort_order` | `string` | no | Predeterminado: desc. Valores: `asc`, `desc`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## files create

Inicia la carga de un archivo de entrada

```text
kobana kadu files create [OPCIONES]
```

`POST /v1/files`

Flujo en tres pasos: (1) este POST crea el archivo `pending` y devuelve `upload_url`; (2) `PUT` de los bytes en `upload_url` con el mismo `Content-Type`; (3) `POST /files/{id}/confirm`. El id va en `input_file_ids` del `POST /tasks/{id}/run` (la Task debe aceptar archivos de entrada). Tamaño máximo: 100 MB.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `task_id` | `string (uuid)` | sí |  |
| `filename` | `string` | sí |  |
| `size_bytes` | `integer` | sí |  |
| `content_type` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.files.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | TASK_DOES_NOT_ACCEPT_FILES / VALIDATION_ERROR — Task fuera de la organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## files get

Detalla el archivo

```text
kobana kadu files get [ID] [OPCIONES]
```

`GET /v1/files/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## files confirm

Confirma la carga (pending → uploaded)

```text
kobana kadu files confirm [ID] [OPCIONES]
```

`POST /v1/files/{id}/confirm`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — no es un archivo de entrada pendiente / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## files download

URL temporal de descarga

```text
kobana kadu files download [ID] [OPCIONES]
```

`GET /v1/files/{id}/download`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | FILE_NOT_UPLOADED |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
