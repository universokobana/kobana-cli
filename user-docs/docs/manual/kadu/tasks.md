---
id: tasks
title: "kobana kadu tasks"
sidebar_label: "tasks"
---

# kobana kadu tasks


Comandos de `kobana kadu tasks`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`tasks endpoint`](#tasks-endpoint) | `GET /v1/tasks/{id}/endpoint` | Endpoint HTTP de disparo da Task |
| [`tasks endpoint update`](#tasks-endpoint-update) | `PUT /v1/tasks/{id}/endpoint` | Configura o endpoint HTTP da Task |
| [`tasks endpoint delete`](#tasks-endpoint-delete) | `DELETE /v1/tasks/{id}/endpoint` | Desabilita o endpoint HTTP da Task |
| [`tasks list`](#tasks-list) | `GET /v1/tasks` | Lista tarefas |
| [`tasks create`](#tasks-create) | `POST /v1/tasks` | Cria task |
| [`tasks get`](#tasks-get) | `GET /v1/tasks/{id}` | Detalha a tarefa |
| [`tasks update`](#tasks-update) | `PATCH /v1/tasks/{id}` | Atualiza task |
| [`tasks delete`](#tasks-delete) | `DELETE /v1/tasks/{id}` | Remove tasks (soft delete) |
| [`tasks readiness`](#tasks-readiness) | `GET /v1/tasks/{id}/readiness` | Pré-checagem do disparo |
| [`tasks run`](#tasks-run) | `POST /v1/tasks/{id}/run` | Dispara execução |

## tasks endpoint

Endpoint HTTP de disparo da Task

```text
kobana kadu tasks endpoint [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/tasks/{id}/endpoint`

Configuração do endpoint público `POST /api/run-task/{task_id}`. Credenciais nunca são devolvidas: só as flags `*_set`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## tasks endpoint update

Configura o endpoint HTTP da Task

```text
kobana kadu tasks endpoint update [ID] [OPÇÕES]
```

`PUT /v1/tasks/{id}/endpoint`

Cria ou atualiza. Credencial omitida mantém a gravada enquanto o tipo de auth não muda. Evite `auth: params`: o segredo vai na URL e fica nos logs de acesso de proxies e CDNs; prefira `bearer` ou `header`.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `enabled` | `boolean` | sim |  |
| `auth` | `string` | sim | Valores: `basic`, `bearer`, `header`, `params`. |
| `auth_basic_user` | `string` | não |  |
| `auth_basic_password` | `string` | não |  |
| `auth_bearer_token` | `string` | não |  |
| `auth_header_key` | `string` | não |  |
| `auth_header_value` | `string` | não |  |
| `auth_param_key` | `string` | não |  |
| `auth_param_value` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.tasks.endpoint.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | INCOMPLETE_AUTH — falta credencial para o tipo de auth escolhido |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## tasks endpoint delete

Desabilita o endpoint HTTP da Task

```text
kobana kadu tasks endpoint delete [ID] [OPÇÕES]
```

`DELETE /v1/tasks/{id}/endpoint`

Mantém a configuração; um PUT com `enabled: true` reativa.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## tasks list

Lista tarefas

```text
kobana kadu tasks list [OPÇÕES]
```

`GET /v1/tasks`

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

## tasks create

Cria task

```text
kobana kadu tasks create [OPÇÕES]
```

`POST /v1/tasks`

A task nasce no tier de preço padrão; tier e modelo de IA são definidos pela Kobana (`model` no corpo é recusado).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `workspace_id` | `string (uuid)` | sim |  |
| `activity_id` | `string (uuid)` | sim |  |
| `skill_slug` | `string` | sim | `<plugin>:<skill>` do repositório da organização: plugin do workspace (quando ele tem um), skill presente no catálogo sincronizado (quando há catálogo), nunca `kadu:*`. |
| `parameters_schema` | `object` | não |  |
| `parameters_sensitive` | `boolean` | não |  |
| `external_id` | `string` | não | Id do registro no sistema de origem (texto livre). |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.tasks.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | VALIDATION_ERROR — workspace/activity fora da organização ou `skill_slug` não permitido |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## tasks get

Detalha a tarefa

```text
kobana kadu tasks get [ID] [OPÇÕES]
```

`GET /v1/tasks/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## tasks update

Atualiza task

```text
kobana kadu tasks update [ID] [OPÇÕES]
```

`PATCH /v1/tasks/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não |  |
| `skill_slug` | `string` | não | `<plugin>:<skill>` do repositório da organização: plugin do workspace (quando ele tem um), skill presente no catálogo sincronizado (quando há catálogo), nunca `kadu:*`. |
| `parameters_schema` | `object` | não |  |
| `parameters_sensitive` | `boolean` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.tasks.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | VALIDATION_ERROR — `skill_slug` não permitido |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## tasks delete

Remove tasks (soft delete)

```text
kobana kadu tasks delete [ID] [OPÇÕES]
```

`DELETE /v1/tasks/{id}`

Exige o nível `delete` (ou `all`): `write` não apaga. Cascata: execuções e arquivos da task; cancela agendamentos e desliga o endpoint.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## tasks readiness

Pré-checagem do disparo

```text
kobana kadu tasks readiness [ID] [OPÇÕES]
```

`GET /v1/tasks/{id}/readiness`

Se a Task pode rodar no ambiente: ambiente válido e valor para todas as secrets exigidas (no workspace da Task ou globais). Traz também a fila da Task. O disparo revalida tudo.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `environment_id` | `string` | não |  |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## tasks run

Dispara execução

```text
kobana kadu tasks run [ID] [OPÇÕES]
```

`POST /v1/tasks/{id}/run`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `reference_date` | `string` | sim |  |
| `environment_id` | `string (uuid)` | sim |  |
| `parameters` | `object` | não |  |
| `locale` | `string` | não |  |
| `additional_instructions` | `string` | não |  |
| `input_file_ids` | `array<string (uuid)>` | não | Arquivos enviados por `POST /files` (confirmados e ainda não usados). |
| `mode` | `string` | não | `resolve_pendencies` só resolve as pendências das execuções anteriores da Task (console: "Resolver pendências"); `parameters` é ignorado. Valores: `default`, `resolve_pendencies`. |

Outros campos e objetos aninhados: `kobana schema kadu.tasks.run`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | COST_LIMIT_REACHED / INVALID_PARAMETERS / ENVIRONMENT_REQUIRED / MISSING_SECRETS (details.keys) / INVALID_INPUT_FILES |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
