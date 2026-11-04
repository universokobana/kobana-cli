---
id: webhooks
title: "kobana kadu webhooks"
sidebar_label: "webhooks"
---

# kobana kadu webhooks


Comandos de `kobana kadu webhooks`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | Lista webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Cria webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Detalha o webhook |
| [`webhooks update`](#webhooks-update) | `PATCH /v1/webhooks/{id}` | Atualiza webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Remove webhooks (soft delete) |
| [`webhooks rotate-secret`](#webhooks-rotate-secret) | `POST /v1/webhooks/{id}/rotate-secret` | Gira o segredo de assinatura |

## webhooks list

Lista webhooks

```text
kobana kadu webhooks list [OPÇÕES]
```

`GET /v1/webhooks`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cursor` | `string` | não |  |
| `limit` | `number` | não |  |
| `sort_by` | `string` | não | Campo em snake_case: created_at (padrão), updated_at ou um campo filtrável. |
| `sort_order` | `string` | não | Padrão: desc. Valores: `asc`, `desc`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## webhooks create

Cria webhook

```text
kobana kadu webhooks create [OPÇÕES]
```

`POST /v1/webhooks`

A URL é conferida contra rede interna. `signing_secret` é devolvido só nesta resposta (o replay de idempotência devolve `null`).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `url` | `string (uri)` | sim |  |
| `auth` | `string` | sim | Valores: `basic`, `bearer`, `header`. |
| `workspace_id` | `string (uuid)` | não |  |
| `auth_basic_user` | `string` | não |  |
| `auth_basic_password` | `string` | não |  |
| `auth_bearer_token` | `string` | não |  |
| `auth_header_key` | `string` | não |  |
| `auth_header_value` | `string` | não |  |
| `subscribed_events` | `array<string>` | não |  |
| `emails` | `array<object>` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.webhooks.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | VALIDATION_ERROR — workspace fora da organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## webhooks get

Detalha o webhook

```text
kobana kadu webhooks get [ID] [OPÇÕES]
```

`GET /v1/webhooks/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## webhooks update

Atualiza webhook

```text
kobana kadu webhooks update [ID] [OPÇÕES]
```

`PATCH /v1/webhooks/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não |  |
| `url` | `string (uri)` | não |  |
| `subscribed_events` | `array<string>` | não |  |
| `status` | `string` | não | Valores: `active`, `inactive`. |
| `emails` | `array<object>` | não |  |
| `auth_basic_user` | `string` | não |  |
| `auth_basic_password` | `string` | não |  |
| `auth_bearer_token` | `string` | não |  |
| `auth_header_key` | `string` | não |  |
| `auth_header_value` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.webhooks.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## webhooks delete

Remove webhooks (soft delete)

```text
kobana kadu webhooks delete [ID] [OPÇÕES]
```

`DELETE /v1/webhooks/{id}`

Exige o nível `delete` (ou `all`): `write` não apaga. Exige a liberação `webhooks.delete`. Webhook declarado no `kadu.yaml` (`repo_managed`) responde 409 REPO_MANAGED: remova-o de lá.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## webhooks rotate-secret

Gira o segredo de assinatura

```text
kobana kadu webhooks rotate-secret [ID] [OPÇÕES]
```

`POST /v1/webhooks/{id}/rotate-secret`

Devolve o segredo novo só nesta resposta; o anterior continua assinando por 24h (o replay de idempotência devolve `signing_secret: null`).

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
