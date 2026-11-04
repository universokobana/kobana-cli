---
id: executions
title: "kobana kadu executions"
sidebar_label: "executions"
---

# kobana kadu executions


Comandos de `kobana kadu executions`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`executions search`](#executions-search) | `GET /v1/executions/search` | Busca textual nas execuções |
| [`executions list`](#executions-list) | `GET /v1/executions` | Lista execuções |
| [`executions get`](#executions-get) | `GET /v1/executions/{id}` | Detalha a execução |
| [`executions cancel`](#executions-cancel) | `POST /v1/executions/{id}/cancel` | Cancela a execução |
| [`executions consolidate-memories`](#executions-consolidate-memories) | `POST /v1/executions/{id}/consolidate_memories` | Consolida as memórias da execução |
| [`executions continue`](#executions-continue) | `POST /v1/executions/{id}/continue` | Continua a conversa (próxima rodada) |
| [`executions events`](#executions-events) | `GET /v1/executions/{id}/events` | Eventos da execução |
| [`executions logs`](#executions-logs) | `GET /v1/executions/{id}/logs` | Logs da execução |
| [`executions todos`](#executions-todos) | `GET /v1/executions/{id}/todos` | Pendências (todos) da execução |

## executions search

Busca textual nas execuções

```text
kobana kadu executions search [OPÇÕES]
```

`GET /v1/executions/search`

Procura em eventos, todos, logs, resultado e erro. Até 30 execuções, mais recentes primeiro, cada uma com até 3 trechos. Só entram as fontes que o chamador poderia ler direto: logs exigem a liberação `executions_logs.view` da organização, e num token de pessoa o papel precisa de `agent.logs.list`, `agent.events.list` e `agent.todos.list` para logs, eventos e todos.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `q` | `string` | sim |  |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — `q` com menos de 2 caracteres |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## executions list

Lista execuções

```text
kobana kadu executions list [OPÇÕES]
```

`GET /v1/executions`

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

## executions get

Detalha a execução

```text
kobana kadu executions get [ID] [OPÇÕES]
```

`GET /v1/executions/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## executions cancel

Cancela a execução

```text
kobana kadu executions cancel [ID] [OPÇÕES]
```

`POST /v1/executions/{id}/cancel`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | cancelled (ou o status terminal atual, se já terminou) |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## executions consolidate-memories

Consolida as memórias da execução

```text
kobana kadu executions consolidate-memories [ID] [OPÇÕES]
```

`POST /v1/executions/{id}/consolidate_memories`

Lê o resultado, os eventos, as pendências e o pedido do usuário desta execução terminada e grava os fatos duráveis como memórias, no escopo da Task — mesmo com memórias desligadas nela. Refaz se já foi consolidada (atualiza as mesmas memórias). É uma chamada de modelo cobrada da organização. Assíncrono.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | enfileirado |
| `404` | NOT_FOUND |
| `409` | CONFLICT — execução não terminada / IDEMPOTENCY_KEY_REUSED |
| `503` | SERVICE_UNAVAILABLE — fila indisponível |

## executions continue

Continua a conversa (próxima rodada)

```text
kobana kadu executions continue [ID] [OPÇÕES]
```

`POST /v1/executions/{id}/continue`

Só para Task conversacional (`conversational`): cria uma execução filha desta que retoma a sessão do agente (contexto, arquivos de saída e estado) e atende ao pedido em `message`. Usa os parâmetros, a data de referência e o ambiente desta execução, que precisa estar terminada (`success` ou `failed`). Continuar uma rodada antiga abre um ramo. A nova execução traz `parentExecutionId` (esta) e `rootExecutionId` (a primeira rodada).

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `message` | `string` | sim |  |
| `locale` | `string` | não |  |
| `input_file_ids` | `array<string (uuid)>` | não | Arquivos novos para esta rodada (`POST /files`, ainda não usados). |

Outros campos e objetos aninhados: `kobana schema kadu.executions.continue`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | rodada criada (na fila da Task) |
| `404` | NOT_FOUND |
| `409` | NOT_CONVERSATIONAL / EXECUTOR_NOT_SUPPORTED / EXECUTOR_CHANGED / NOT_FINISHED / SESSION_UNAVAILABLE |
| `422` | ENVIRONMENT_REQUIRED / MISSING_SECRETS (details.keys) / INVALID_PARAMETERS / INVALID_INPUT_FILES / COST_LIMIT_REACHED |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## executions events

Eventos da execução

```text
kobana kadu executions events [ID] [OPÇÕES]
```

`GET /v1/executions/{id}/events`

Por `sequence`, até 500 por chamada. Para acompanhar, passe em `after_sequence` o último `sequence` recebido e repita.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `after_sequence` | `integer,null` | não |  |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## executions logs

Logs da execução

```text
kobana kadu executions logs [ID] [OPÇÕES]
```

`GET /v1/executions/{id}/logs`

Em ordem de chegada, até 1000 por chamada. Para acompanhar uma execução em andamento, passe em `after_id` o último `id` recebido e repita.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `after_id` | `integer,null` | não |  |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## executions todos

Pendências (todos) da execução

```text
kobana kadu executions todos [ID] [OPÇÕES]
```

`GET /v1/executions/{id}/todos`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
