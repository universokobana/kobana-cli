---
id: secrets
title: "kobana kadu secrets"
sidebar_label: "secrets"
---

# kobana kadu secrets


Comandos de `kobana kadu secrets`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`secrets list`](#secrets-list) | `GET /v1/secrets` | Lista secrets |
| [`secrets create`](#secrets-create) | `POST /v1/secrets` | Crea secret |
| [`secrets get`](#secrets-get) | `GET /v1/secrets/{id}` | Detalla el secret |
| [`secrets update`](#secrets-update) | `PATCH /v1/secrets/{id}` | Actualiza secret |
| [`secrets delete`](#secrets-delete) | `DELETE /v1/secrets/{id}` | Elimina secret (soft delete) |

## secrets list

Lista secrets

```text
kobana kadu secrets list [OPCIONES]
```

`GET /v1/secrets`

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

## secrets create

Crea secret

```text
kobana kadu secrets create [OPCIONES]
```

`POST /v1/secrets`

El valor se guarda cifrado y nunca se devuelve.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `environment_id` | `string (uuid)` | sí |  |
| `key` | `string` | sí |  |
| `value` | `string` | sí |  |
| `workspace_id` | `string (uuid)` | no |  |
| `notes` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.secrets.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | VALIDATION_ERROR — entorno o workspace fuera de la organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## secrets get

Detalla el secret

```text
kobana kadu secrets get [ID] [OPCIONES]
```

`GET /v1/secrets/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## secrets update

Actualiza secret

```text
kobana kadu secrets update [ID] [OPCIONES]
```

`PATCH /v1/secrets/{id}`

`value` ausente o vacío mantiene el valor guardado.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `environment_id` | `string (uuid)` | no |  |
| `workspace_id` | `string (uuid)` | no |  |
| `key` | `string` | no |  |
| `value` | `string` | no |  |
| `notes` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.secrets.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | VALIDATION_ERROR — entorno o workspace fuera de la organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## secrets delete

Elimina secret (soft delete)

```text
kobana kadu secrets delete [ID] [OPCIONES]
```

`DELETE /v1/secrets/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
