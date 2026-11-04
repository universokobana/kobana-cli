---
id: webhook-deliveries
title: "kobana kadu webhook-deliveries"
sidebar_label: "webhook-deliveries"
---

# kobana kadu webhook-deliveries


Comandos de `kobana kadu webhook-deliveries`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`webhook-deliveries list`](#webhook-deliveries-list) | `GET /v1/webhook_deliveries` | Entregas de webhook (50 mais recentes) |
| [`webhook-deliveries get`](#webhook-deliveries-get) | `GET /v1/webhook_deliveries/{id}` | Detalha a entrega de webhook |
| [`webhook-deliveries replay`](#webhook-deliveries-replay) | `POST /v1/webhook_deliveries/{id}/replay` | Reenfileira a entrega |

## webhook-deliveries list

Entregas de webhook (50 mais recentes)

```text
kobana kadu webhook-deliveries list [OPÇÕES]
```

`GET /v1/webhook_deliveries`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `webhook_id` | `string` | não |  |
| `status` | `string` | não | Valores: `pending`, `success`, `failed`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | BAD_REQUEST — `status` ou `webhook_id` inválido |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## webhook-deliveries get

Detalha a entrega de webhook

```text
kobana kadu webhook-deliveries get [ID] [OPÇÕES]
```

`GET /v1/webhook_deliveries/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## webhook-deliveries replay

Reenfileira a entrega

```text
kobana kadu webhook-deliveries replay [ID] [OPÇÕES]
```

`POST /v1/webhook_deliveries/{id}/replay`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | accepted |
| `404` | NOT_FOUND |
| `409` | REPLAY_IN_FLIGHT — já há reenvio pendente deste evento / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
