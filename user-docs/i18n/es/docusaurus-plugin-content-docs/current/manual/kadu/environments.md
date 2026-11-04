---
id: environments
title: "kobana kadu environments"
sidebar_label: "environments"
---

# kobana kadu environments


Comandos de `kobana kadu environments`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`environments list`](#environments-list) | `GET /v1/environments` | Lista entornos |
| [`environments create`](#environments-create) | `POST /v1/environments` | Crea entorno |
| [`environments get`](#environments-get) | `GET /v1/environments/{id}` | Detalla el entorno |
| [`environments update`](#environments-update) | `PATCH /v1/environments/{id}` | Actualiza entorno |
| [`environments delete`](#environments-delete) | `DELETE /v1/environments/{id}` | Elimina entorno (soft delete) |

## environments list

Lista entornos

```text
kobana kadu environments list [OPCIONES]
```

`GET /v1/environments`

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

## environments create

Crea entorno

```text
kobana kadu environments create [OPCIONES]
```

`POST /v1/environments`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `color` | `string` | sí |  |
| `branch` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.environments.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — ya existe un entorno vivo con este label / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## environments get

Detalla el entorno

```text
kobana kadu environments get [ID] [OPCIONES]
```

`GET /v1/environments/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## environments update

Actualiza entorno

```text
kobana kadu environments update [ID] [OPCIONES]
```

`PATCH /v1/environments/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no |  |
| `color` | `string` | no |  |
| `branch` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.environments.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — label ya usado / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## environments delete

Elimina entorno (soft delete)

```text
kobana kadu environments delete [ID] [OPCIONES]
```

`DELETE /v1/environments/{id}`

Rechazado mientras el entorno tenga secrets o programaciones.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | ENVIRONMENT_IN_USE — `details: { secrets, schedules }` / REPO_MANAGED — declarado en el `kadu.yaml`: quítalo de allí |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
