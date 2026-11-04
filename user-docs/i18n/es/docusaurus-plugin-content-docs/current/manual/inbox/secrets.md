---
id: secrets
title: "kobana inbox secrets"
sidebar_label: "secrets"
---

# kobana inbox secrets


Comandos de `kobana inbox secrets`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`secrets list`](#secrets-list) | `GET /v1/secrets` | Listar secrets |
| [`secrets create`](#secrets-create) | `POST /v1/secrets` | Crear secret |
| [`secrets update`](#secrets-update) | `PUT /v1/secrets` | Crear o actualizar secret |
| [`secrets get`](#secrets-get) | `GET /v1/secrets/{id}` | Obtener secret |
| [`secrets delete`](#secrets-delete) | `DELETE /v1/secrets/{id}` | Eliminar secret |

## secrets list

Listar secrets

```text
kobana inbox secrets list [OPCIONES]
```

`GET /v1/secrets`

Paginación por keyset. Los filtros son query params tipados (`campo=valor`, `campo[op]=valor`). El valor de un secret nunca lo devuelve ningún endpoint: se guarda cifrado y solo lo lee el ejecutor de workflows. Para rotarlo, haga `PATCH` sobre la fila (o `PUT` por `key`) con un nuevo `value`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no |  |
| `cursor` | `string` | no | Cursor opaco devuelto en `meta.next_cursor` de la página anterior. |
| `sort_by` | `string` | no | Campo por el que ordenar. Permitidos: `created_at`, `updated_at`, `key`. Sin él, el orden es `created_at` descendente. Valores: `created_at`, `updated_at`, `key`. |
| `sort_order` | `string` | no | `asc` (por defecto) o `desc`. Requiere `sort_by`. Valores: `asc`, `desc`. |
| `key` | `string` | no | Equality. Also `key[in]=a,b`. |
| `created_at` | `string` | no | Igualdad. También `created_at[in]=a,b` y `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | no |  |
| `created_at[lte]` | `string` | no |  |
| `updated_at` | `string` | no | Igualdad. También `updated_at[in]=a,b` y `updated_at[gt\|gte\|lt\|lte]`. |
| `updated_at[gte]` | `string` | no |  |
| `updated_at[lte]` | `string` | no |  |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de resultados. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## secrets create

Crear secret

```text
kobana inbox secrets create [OPCIONES]
```

`POST /v1/secrets`

Guarda un secret que un workflow del agente puede referenciar como `${{secrets.KEY}}`. Una `key` duplicada en el workspace se rechaza con 409 `CONFLICT`. El valor es de solo escritura: se guarda cifrado y nunca se devuelve.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | sí |  |
| `value` | `string` | sí | Se guarda cifrado; nunca se devuelve. |
| `notes` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema inbox.secrets.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## secrets update

Crear o actualizar secret

```text
kobana inbox secrets update [OPCIONES]
```

`PUT /v1/secrets`

Upsert emparejado por `key` dentro del workspace de quien llama — el camino de la rotación, sin buscar el id. Crea con 201 cuando la clave es nueva, reemplaza el valor con 200 cuando ya existe. El valor es de solo escritura.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | sí | Clave de búsqueda del upsert. |
| `value` | `string` | sí |  |
| `notes` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema inbox.secrets.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `201` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
| `404` | Recurso no encontrado o excluido por soft-delete. |
| `409` | Se violó una restricción de unicidad (por ejemplo, `external_id` o el `email` del inbox ya en uso). |
| `422` | Validación del cuerpo que requiere contexto del lado del servidor (por ejemplo, dominio del inbox no coincide). |

## secrets get

Obtener secret

```text
kobana inbox secrets get [ID] [OPCIONES]
```

`GET /v1/secrets/{id}`

Acepta el UUID interno o el `external_id` del recurso.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `404` | Recurso no encontrado o excluido por soft-delete. |

## secrets delete

Eliminar secret

```text
kobana inbox secrets delete [ID] [OPCIONES]
```

`DELETE /v1/secrets/{id}`

Borrado lógico. Los pasos de workflow que referencian la clave empiezan a fallar con `secret not defined` desde la siguiente ejecución.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Eliminado (soft delete). |
| `404` | Recurso no encontrado o excluido por soft-delete. |
