---
id: webhook-deliveries
title: "kobana kadu webhook-deliveries"
sidebar_label: "webhook-deliveries"
---

# kobana kadu webhook-deliveries


Comandos de `kobana kadu webhook-deliveries`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`webhook-deliveries list`](#webhook-deliveries-list) | `GET /v1/webhook_deliveries` | Entregas de webhook (las 50 más recientes) |
| [`webhook-deliveries get`](#webhook-deliveries-get) | `GET /v1/webhook_deliveries/{id}` | Detalla la entrega de webhook |
| [`webhook-deliveries replay`](#webhook-deliveries-replay) | `POST /v1/webhook_deliveries/{id}/replay` | Reencola la entrega |

## webhook-deliveries list

Entregas de webhook (las 50 más recientes)

```text
kobana kadu webhook-deliveries list [OPCIONES]
```

`GET /v1/webhook_deliveries`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `webhook_id` | `string` | no |  |
| `status` | `string` | no | Valores: `pending`, `success`, `failed`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | BAD_REQUEST — `status` o `webhook_id` inválido |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## webhook-deliveries get

Detalla la entrega de webhook

```text
kobana kadu webhook-deliveries get [ID] [OPCIONES]
```

`GET /v1/webhook_deliveries/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## webhook-deliveries replay

Reencola la entrega

```text
kobana kadu webhook-deliveries replay [ID] [OPCIONES]
```

`POST /v1/webhook_deliveries/{id}/replay`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | accepted |
| `404` | NOT_FOUND |
| `409` | REPLAY_IN_FLIGHT — ya hay un reenvío pendiente de este evento / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
