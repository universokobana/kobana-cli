---
id: memories
title: "kobana kadu memories"
sidebar_label: "memories"
---

# kobana kadu memories


Comandos de `kobana kadu memories`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`memories list`](#memories-list) | `GET /v1/memories` | Lista memórias |
| [`memories create`](#memories-create) | `POST /v1/memories` | Cria memória |
| [`memories consolidate`](#memories-consolidate) | `POST /v1/memories/consolidate` | Consolida as memórias de um período |
| [`memories get`](#memories-get) | `GET /v1/memories/{id}` | Detalha a memória |
| [`memories update`](#memories-update) | `PATCH /v1/memories/{id}` | Atualiza memória |
| [`memories delete`](#memories-delete) | `DELETE /v1/memories/{id}` | Remove memória (soft delete) |
| [`memories rollback`](#memories-rollback) | `POST /v1/memories/{id}/rollback` | Volta a memória para uma versão |
| [`memories versions`](#memories-versions) | `GET /v1/memories/{id}/versions` | Lista as versões da memória |

## memories list

Lista memórias

```text
kobana kadu memories list [OPÇÕES]
```

`GET /v1/memories`

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

## memories create

Cria memória

```text
kobana kadu memories create [OPÇÕES]
```

`POST /v1/memories`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `content` | `string` | sim |  |
| `workspace_id` | `string (uuid)` | não |  |
| `activity_id` | `string (uuid)` | não |  |
| `entity_type` | `string` | não |  |
| `entity_value` | `string` | não |  |
| `key` | `string` | não |  |
| `summary` | `string` | não |  |
| `status` | `string` | não |  |
| `external_id` | `string` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.memories.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT (external_id duplicado na organização) / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — workspace/activity fora da organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## memories consolidate

Consolida as memórias de um período

```text
kobana kadu memories consolidate [OPÇÕES]
```

`POST /v1/memories/consolidate`

Lê as execuções terminadas (`success`/`failed`) do período que ainda não foram consolidadas — inclusive de Task com memórias desligadas — e grava os fatos duráveis como memórias, no escopo de cada Task. Cada execução é uma chamada de modelo cobrada da organização. `dry_run: true` devolve só a prévia. Assíncrono: as memórias aparecem conforme os jobs rodam.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `from` | `string` | sim | Dia inicial (YYYY-MM-DD, UTC), inclusive. |
| `to` | `string` | sim | Dia final (YYYY-MM-DD, UTC), inclusive. Até 366 dias. |
| `task_id` | `string (uuid)` | não | Só as execuções desta Task. |
| `dry_run` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.memories.consolidate`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | prévia (dry_run) |
| `202` | enfileirado |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | VALIDATION_ERROR — período ou Task inválidos |
| `503` | SERVICE_UNAVAILABLE — fila indisponível |

## memories get

Detalha a memória

```text
kobana kadu memories get [ID] [OPÇÕES]
```

`GET /v1/memories/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## memories update

Atualiza memória

```text
kobana kadu memories update [ID] [OPÇÕES]
```

`PATCH /v1/memories/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `content` | `string` | não |  |
| `workspace_id` | `string (uuid)` | não |  |
| `activity_id` | `string (uuid)` | não |  |
| `entity_type` | `string` | não |  |
| `entity_value` | `string` | não |  |
| `key` | `string` | não |  |
| `summary` | `string` | não |  |
| `status` | `string` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.memories.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | VALIDATION_ERROR — workspace/activity fora da organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## memories delete

Remove memória (soft delete)

```text
kobana kadu memories delete [ID] [OPÇÕES]
```

`DELETE /v1/memories/{id}`

Exige o nível `delete` (ou `all`): `write` não apaga.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## memories rollback

Volta a memória para uma versão

```text
kobana kadu memories rollback [ID] [OPÇÕES]
```

`POST /v1/memories/{id}/rollback`

Grava uma versão NOVA com o conteúdo da escolhida, que vira a vigente; o histórico fica inteiro (`rollback_from`).

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `version` | `integer` | sim |  |

Outros campos e objetos aninhados: `kobana schema kadu.memories.rollback`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND (memória ou versão) |
| `409` | CONFLICT — já é a versão vigente / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — workspace/activity da versão não vale mais |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## memories versions

Lista as versões da memória

```text
kobana kadu memories versions [ID] [OPÇÕES]
```

`GET /v1/memories/{id}/versions`

Mais nova primeiro. Toda alteração de conteúdo (pela API, pelo agente, pela consolidação ou pelo console) grava uma versão; `current: true` marca a vigente.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
