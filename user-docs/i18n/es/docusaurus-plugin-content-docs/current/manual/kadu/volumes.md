---
id: volumes
title: "kobana kadu volumes"
sidebar_label: "volumes"
---

# kobana kadu volumes


Comandos de `kobana kadu volumes`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`volumes list`](#volumes-list) | `GET /v1/volumes` | Lista volúmenes |
| [`volumes create`](#volumes-create) | `POST /v1/volumes` | Crea volumen |
| [`volumes get`](#volumes-get) | `GET /v1/volumes/{id}` | Detalla el volumen |
| [`volumes update`](#volumes-update) | `PATCH /v1/volumes/{id}` | Actualiza volumen |
| [`volumes delete`](#volumes-delete) | `DELETE /v1/volumes/{id}` | Elimina volumen (soft delete) |
| [`volumes files`](#volumes-files) | `GET /v1/volumes/{id}/files` | Lista los archivos del volumen |
| [`volumes syncs`](#volumes-syncs) | `GET /v1/volumes/{id}/syncs` | Lista los syncs del volumen |

## volumes list

Lista volúmenes

```text
kobana kadu volumes list [OPCIONES]
```

`GET /v1/volumes`

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

## volumes create

Crea volumen

```text
kobana kadu volumes create [OPCIONES]
```

`POST /v1/volumes`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí |  |
| `environment_id` | `string (uuid)` | no |  |
| `label` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `quota_mb` | `integer` | no | Predeterminado: 1024. |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.volumes.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — ya existe un volumen activo con este nombre en el entorno / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — entorno o workspace de otra organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## volumes get

Detalla el volumen

```text
kobana kadu volumes get [ID] [OPCIONES]
```

`GET /v1/volumes/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## volumes update

Actualiza volumen

```text
kobana kadu volumes update [ID] [OPCIONES]
```

`PATCH /v1/volumes/{id}`

El nombre y el entorno no cambian: las skills y el `kadu.yaml` referencian el volumen por ellos.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `quota_mb` | `integer` | no | Predeterminado: 1024. |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.volumes.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## volumes delete

Elimina volumen (soft delete)

```text
kobana kadu volumes delete [ID] [OPCIONES]
```

`DELETE /v1/volumes/{id}`

El contenido se guarda por 30 días antes de la purga. Se rechaza mientras el `kadu.yaml` declare el volumen o una ejecución esté escribiendo en él.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declarado en el `kadu.yaml` / VOLUME_IN_USE — una ejecución está escribiendo en él |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## volumes files

Lista los archivos del volumen

```text
kobana kadu volumes files [ID] [OPCIONES]
```

`GET /v1/volumes/{id}/files`

En orden de ruta. Solo metadatos: el contenido nunca se devuelve.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `path_prefix` | `string` | no |  |
| `cursor` | `string` | no |  |
| `limit` | `number` | no | Predeterminado: 200. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — cursor inválido |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## volumes syncs

Lista los syncs del volumen

```text
kobana kadu volumes syncs [ID] [OPCIONES]
```

`GET /v1/volumes/{id}/syncs`

Las ejecuciones que montaron el volumen, de la más reciente, con el resultado de cada sync.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `number` | no |  |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
