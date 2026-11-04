---
id: sources
title: "kobana kadu sources"
sidebar_label: "sources"
---

# kobana kadu sources


Comandos de `kobana kadu sources`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`sources list`](#sources-list) | `GET /v1/sources` | Lista los repositorios de código |
| [`sources create`](#sources-create) | `POST /v1/sources` | Registra un repositorio de código |
| [`sources get`](#sources-get) | `GET /v1/sources/{id}` | Detalla el repositorio de código |
| [`sources update`](#sources-update) | `PATCH /v1/sources/{id}` | Actualiza un repositorio de código |
| [`sources delete`](#sources-delete) | `DELETE /v1/sources/{id}` | Elimina un repositorio de código (soft delete) |
| [`sources check`](#sources-check) | `POST /v1/sources/{id}/check` | Verifica el acceso al repositorio |

## sources list

Lista los repositorios de código

```text
kobana kadu sources list [OPCIONES]
```

`GET /v1/sources`

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

## sources create

Registra un repositorio de código

```text
kobana kadu sources create [OPCIONES]
```

`POST /v1/sources`

Si la instalación del GitHub App de la organización cubre el repositorio, el acceso se verifica en el momento. Si no, la respuesta trae la deploy key a registrar en GitHub antes de verificar el acceso.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí |  |
| `repository` | `string` | sí | `owner/repo` o la URL del repositorio en GitHub. |
| `ref` | `string` | no | Branch o tag; null = la branch predeterminada. |
| `label` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.sources.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — ya existe un repositorio vivo con este nombre / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — repositorio, ref o workspace inválido |
| `503` | GITHUB_UNAVAILABLE — GitHub no respondió |

## sources get

Detalla el repositorio de código

```text
kobana kadu sources get [ID] [OPCIONES]
```

`GET /v1/sources/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## sources update

Actualiza un repositorio de código

```text
kobana kadu sources update [ID] [OPCIONES]
```

`PATCH /v1/sources/{id}`

El nombre y el repositorio no cambian: las skills referencian el nombre y la deploy key es del repositorio. Cambiar la ref reinicia la verificación de acceso.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ref` | `string` | no | Branch o tag; null = la branch predeterminada. |
| `label` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.sources.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — ref o workspace inválido |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## sources delete

Elimina un repositorio de código (soft delete)

```text
kobana kadu sources delete [ID] [OPCIONES]
```

`DELETE /v1/sources/{id}`

Las Tasks que lo declaran fallan en la próxima ejecución. Se rechaza mientras el `kadu.yaml` declare el repositorio.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declarado en el `kadu.yaml` |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## sources check

Verifica el acceso al repositorio

```text
kobana kadu sources check [ID] [OPCIONES]
```

`POST /v1/sources/{id}/check`

Lista la ref en GitHub con la credencial del repositorio, sin clonar, y guarda el resultado. La falta de acceso no es un error HTTP: viene en `access_status` y `last_check_error`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
