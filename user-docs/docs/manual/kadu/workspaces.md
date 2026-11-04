---
id: workspaces
title: "kobana kadu workspaces"
sidebar_label: "workspaces"
---

# kobana kadu workspaces


Comandos de `kobana kadu workspaces`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | Lista workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Cria workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Detalha o workspace |
| [`workspaces update`](#workspaces-update) | `PATCH /v1/workspaces/{id}` | Atualiza workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Remove workspaces (soft delete) |

## workspaces list

Lista workspaces

```text
kobana kadu workspaces list [OPÇÕES]
```

`GET /v1/workspaces`

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

## workspaces create

Cria workspace

```text
kobana kadu workspaces create [OPÇÕES]
```

`POST /v1/workspaces`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `external_id` | `string` | não | Id do registro no sistema de origem (texto livre). |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.workspaces.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — registro duplicado / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## workspaces get

Detalha o workspace

```text
kobana kadu workspaces get [ID] [OPÇÕES]
```

`GET /v1/workspaces/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## workspaces update

Atualiza workspace

```text
kobana kadu workspaces update [ID] [OPÇÕES]
```

`PATCH /v1/workspaces/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.workspaces.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## workspaces delete

Remove workspaces (soft delete)

```text
kobana kadu workspaces delete [ID] [OPÇÕES]
```

`DELETE /v1/workspaces/{id}`

Exige o nível `delete` (ou `all`): `write` não apaga. Cascata: activities, tasks, execuções, webhooks, memórias e secrets do workspace; cancela agendamentos e desliga o endpoint das tasks.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
