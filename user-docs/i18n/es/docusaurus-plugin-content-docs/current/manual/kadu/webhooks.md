---
id: webhooks
title: "kobana kadu webhooks"
sidebar_label: "webhooks"
---

# kobana kadu webhooks


Comandos de `kobana kadu webhooks`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | Lista webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Crea webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Detalla el webhook |
| [`webhooks update`](#webhooks-update) | `PATCH /v1/webhooks/{id}` | Actualiza webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Elimina webhooks (soft delete) |
| [`webhooks rotate-secret`](#webhooks-rotate-secret) | `POST /v1/webhooks/{id}/rotate-secret` | Rota el secreto de firma |

## webhooks list

Lista webhooks

```text
kobana kadu webhooks list [OPCIONES]
```

`GET /v1/webhooks`

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

## webhooks create

Crea webhook

```text
kobana kadu webhooks create [OPCIONES]
```

`POST /v1/webhooks`

La URL se verifica contra la red interna. `signing_secret` se devuelve solo en esta respuesta (el replay de idempotencia devuelve `null`).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `url` | `string (uri)` | sí |  |
| `auth` | `string` | sí | Valores: `basic`, `bearer`, `header`. |
| `workspace_id` | `string (uuid)` | no |  |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |
| `subscribed_events` | `array<string>` | no |  |
| `emails` | `array<object>` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.webhooks.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | VALIDATION_ERROR — workspace fuera de la organización |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## webhooks get

Detalla el webhook

```text
kobana kadu webhooks get [ID] [OPCIONES]
```

`GET /v1/webhooks/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## webhooks update

Actualiza webhook

```text
kobana kadu webhooks update [ID] [OPCIONES]
```

`PATCH /v1/webhooks/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no |  |
| `url` | `string (uri)` | no |  |
| `subscribed_events` | `array<string>` | no |  |
| `status` | `string` | no | Valores: `active`, `inactive`. |
| `emails` | `array<object>` | no |  |
| `auth_basic_user` | `string` | no |  |
| `auth_basic_password` | `string` | no |  |
| `auth_bearer_token` | `string` | no |  |
| `auth_header_key` | `string` | no |  |
| `auth_header_value` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.webhooks.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## webhooks delete

Elimina webhooks (soft delete)

```text
kobana kadu webhooks delete [ID] [OPCIONES]
```

`DELETE /v1/webhooks/{id}`

Exige el nivel `delete` (o `all`): `write` no elimina. Exige la habilitación `webhooks.delete`. Un webhook declarado en el `kadu.yaml` (`repo_managed`) responde 409 REPO_MANAGED: quítalo de allí.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## webhooks rotate-secret

Rota el secreto de firma

```text
kobana kadu webhooks rotate-secret [ID] [OPCIONES]
```

`POST /v1/webhooks/{id}/rotate-secret`

Devuelve el secreto nuevo solo en esta respuesta; el anterior sigue firmando por 24h (el replay de idempotencia devuelve `signing_secret: null`).

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
