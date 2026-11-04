---
id: webhook-deliveries
title: "kobana kadu webhook-deliveries"
sidebar_label: "webhook-deliveries"
---

# kobana kadu webhook-deliveries


Commands of `kobana kadu webhook-deliveries`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`webhook-deliveries list`](#webhook-deliveries-list) | `GET /v1/webhook_deliveries` | Webhook deliveries (50 most recent) |
| [`webhook-deliveries get`](#webhook-deliveries-get) | `GET /v1/webhook_deliveries/{id}` | Get webhook delivery details |
| [`webhook-deliveries replay`](#webhook-deliveries-replay) | `POST /v1/webhook_deliveries/{id}/replay` | Requeue the delivery |

## webhook-deliveries list

Webhook deliveries (50 most recent)

```text
kobana kadu webhook-deliveries list [OPTIONS]
```

`GET /v1/webhook_deliveries`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `webhook_id` | `string` | no |  |
| `status` | `string` | no | Values: `pending`, `success`, `failed`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `400` | BAD_REQUEST — invalid `status` or `webhook_id` |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## webhook-deliveries get

Get webhook delivery details

```text
kobana kadu webhook-deliveries get [ID] [OPTIONS]
```

`GET /v1/webhook_deliveries/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |

## webhook-deliveries replay

Requeue the delivery

```text
kobana kadu webhook-deliveries replay [ID] [OPTIONS]
```

`POST /v1/webhook_deliveries/{id}/replay`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `202` | accepted |
| `404` | NOT_FOUND |
| `409` | REPLAY_IN_FLIGHT — a resend of this event is already pending / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — token verification is unavailable (Authorization Server down or configuration missing). Try again (`Retry-After`). |
