---
id: schedules
title: "kobana kadu schedules"
sidebar_label: "schedules"
---

# kobana kadu schedules


Comandos de `kobana kadu schedules`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`schedules list`](#schedules-list) | `GET /v1/schedules` | Lista agendamentos |
| [`schedules create`](#schedules-create) | `POST /v1/schedules` | Agenda uma Task num ambiente |
| [`schedules get`](#schedules-get) | `GET /v1/schedules/{id}` | Detalha o agendamento |
| [`schedules update`](#schedules-update) | `PATCH /v1/schedules/{id}` | Altera, pausa ou retoma agendamento |
| [`schedules delete`](#schedules-delete) | `DELETE /v1/schedules/{id}` | Cancela agendamento (soft delete) |

## schedules list

Lista agendamentos

```text
kobana kadu schedules list [OPÇÕES]
```

`GET /v1/schedules`

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

## schedules create

Agenda uma Task num ambiente

```text
kobana kadu schedules create [OPÇÕES]
```

`POST /v1/schedules`

Um agendamento por Task e ambiente. Agendamento não empilha: o disparo é pulado se a Task já tem execução ativa ou na fila.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `task_id` | `string (uuid)` | sim |  |
| `environment_id` | `string (uuid)` | sim |  |
| `cron` | `string` | sim |  |
| `timezone` | `string` | não |  |
| `paused` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.schedules.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `409` | CONFLICT — a Task já tem agendamento no ambiente (`details.schedule_id`) / IDEMPOTENCY_KEY_REUSED |
| `422` | INVALID_CRON / VALIDATION_ERROR — Task ou ambiente fora da organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## schedules get

Detalha o agendamento

```text
kobana kadu schedules get [ID] [OPÇÕES]
```

`GET /v1/schedules/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## schedules update

Altera, pausa ou retoma agendamento

```text
kobana kadu schedules update [ID] [OPÇÕES]
```

`PATCH /v1/schedules/{id}`

`paused: true` mantém o cron e faz o dispatcher pular o agendamento.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cron` | `string` | não |  |
| `timezone` | `string` | não |  |
| `paused` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.schedules.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | INVALID_CRON |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## schedules delete

Cancela agendamento (soft delete)

```text
kobana kadu schedules delete [ID] [OPÇÕES]
```

`DELETE /v1/schedules/{id}`

Um novo POST para a mesma Task e ambiente reativa o agendamento.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declarado no `schedules` do frontmatter da skill: remova-o de lá |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
