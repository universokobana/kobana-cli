---
id: activities
title: "kobana kadu activities"
sidebar_label: "activities"
---

# kobana kadu activities


Comandos de `kobana kadu activities`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`activities list`](#activities-list) | `GET /v1/activities` | Lista atividades |
| [`activities create`](#activities-create) | `POST /v1/activities` | Cria activity |
| [`activities get`](#activities-get) | `GET /v1/activities/{id}` | Detalha a atividade |
| [`activities update`](#activities-update) | `PATCH /v1/activities/{id}` | Atualiza activity |
| [`activities delete`](#activities-delete) | `DELETE /v1/activities/{id}` | Remove activities (soft delete) |

## activities list

Lista atividades

```text
kobana kadu activities list [OPÇÕES]
```

`GET /v1/activities`

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

## activities create

Cria activity

```text
kobana kadu activities create [OPÇÕES]
```

`POST /v1/activities`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `workspace_id` | `string (uuid)` | sim |  |
| `external_id` | `string` | não | Id do registro no sistema de origem (texto livre). |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.activities.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | VALIDATION_ERROR — workspace fora da organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## activities get

Detalha a atividade

```text
kobana kadu activities get [ID] [OPÇÕES]
```

`GET /v1/activities/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## activities update

Atualiza activity

```text
kobana kadu activities update [ID] [OPÇÕES]
```

`PATCH /v1/activities/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.activities.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## activities delete

Remove activities (soft delete)

```text
kobana kadu activities delete [ID] [OPÇÕES]
```

`DELETE /v1/activities/{id}`

Exige o nível `delete` (ou `all`): `write` não apaga. Cascata: tasks, execuções e memórias da activity; cancela agendamentos e desliga o endpoint das tasks.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
