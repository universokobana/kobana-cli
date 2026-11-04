---
id: tasks
title: "kobana dunning tasks"
sidebar_label: "tasks"
---

# kobana dunning tasks


Comandos de `kobana dunning tasks`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`tasks list`](#tasks-list) | `GET /v1/tasks` | Listar tarefas |
| [`tasks create`](#tasks-create) | `POST /v1/tasks` | Criar tarefa |
| [`tasks bulk-assign`](#tasks-bulk-assign) | `POST /v1/tasks/bulk-assign` | Atribuir responsável em massa |
| [`tasks bulk-complete`](#tasks-bulk-complete) | `POST /v1/tasks/bulk-complete` | Concluir tarefas em massa |
| [`tasks get`](#tasks-get) | `GET /v1/tasks/{id}` | Consultar tarefa |
| [`tasks update`](#tasks-update) | `PUT /v1/tasks/{id}` | Atualizar tarefa |
| [`tasks complete`](#tasks-complete) | `POST /v1/tasks/{id}/complete` | Concluir tarefa |

## tasks list

Listar tarefas

```text
kobana dunning tasks list [OPÇÕES]
```

`GET /v1/tasks`

Lista as tarefas da organização com paginação e filtros. Ordenável via `sort_by` (`title` | `priority` | `status` | `due_at` | `created_at` | `updated_at`).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `sort_by` | `string` | não | Campo de ordenação (nome público em snake_case, ex.: `due_date`). |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra por status (`pending` \| `in_progress` \| `completed` \| `cancelled`). Valores: `pending`, `in_progress`, `completed`, `cancelled`. |
| `priority` | `string` | não | Filtra por prioridade (`low` \| `medium` \| `high` \| `urgent`). Valores: `low`, `medium`, `high`, `urgent`. |
| `task_type` | `string` | não | Filtra pelo tipo (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Valores: `call`, `email`, `visit`, `review`, `follow_up`. |
| `assigned_to_id` | `string` | não | Filtra pelo responsável (UUID). |
| `person_id` | `string` | não | Filtra pelo cliente (UUID). |
| `charge_id` | `string` | não | Filtra pela cobrança (UUID). |
| `include` | `string` | não | Relações a incluir, separadas por vírgula (`person`, `assigned_to`, `charge`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de tarefas. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## tasks create

Criar tarefa

```text
kobana dunning tasks create [OPÇÕES]
```

`POST /v1/tasks`

Cria uma tarefa, opcionalmente vinculada a um cliente, uma cobrança e um responsável (todos da própria organização). Suporta `Idempotency-Key`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `title` | `string` | sim | Título da tarefa. |
| `task_type` | `string` | sim | Tipo (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Valores: `call`, `email`, `visit`, `review`, `follow_up`. |
| `person_id` | `string (uuid)` | não | Cliente vinculado à tarefa. |
| `charge_id` | `string (uuid)` | não | Cobrança vinculada à tarefa. |
| `assigned_to_id` | `string (uuid)` | não | Usuário responsável pela tarefa. |
| `description` | `string` | não | Descrição detalhada. |
| `priority` | `string` | não | Prioridade inicial (`low` \| `medium` \| `high` \| `urgent`; padrão `medium`). Valores: `low`, `medium`, `high`, `urgent`. |
| `status` | `string` | não | Status inicial (padrão `pending`). Valores: `pending`, `in_progress`, `completed`, `cancelled`. |
| `due_at` | `string (date-time)` | não | Prazo da tarefa (ISO 8601). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.tasks.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Tarefa criada (inclui os resumos de cliente, responsável e cobrança). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## tasks bulk-assign

Atribuir responsável em massa

```text
kobana dunning tasks bulk-assign [OPÇÕES]
```

`POST /v1/tasks/bulk-assign`

Atribui (ou remove, com `assignedToId: null`) o responsável de até 100 tarefas. O usuário-alvo deve pertencer à organização. Tarefa cancelada volta em `failed` com `invalid_status`. Resultado parcial não é erro.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | IDs dos registros-alvo (1 a 100 por chamada). Duplicados são ignorados. |
| `assigned_to_id` | `string (uuid)` | sim | ID do usuário responsável, ou `null` para remover a atribuição. |

Outros campos e objetos aninhados: `kobana schema dunning.tasks.bulk-assign`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado por item da atribuição em massa. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## tasks bulk-complete

Concluir tarefas em massa

```text
kobana dunning tasks bulk-complete [OPÇÕES]
```

`POST /v1/tasks/bulk-complete`

Conclui até 100 tarefas numa chamada. Tarefa já concluída volta em `failed` com `already_completed`; cancelada, com `invalid_status`. Resultado parcial não é erro.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | IDs dos registros-alvo (1 a 100 por chamada). Duplicados são ignorados. |

Outros campos e objetos aninhados: `kobana schema dunning.tasks.bulk-complete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado por item da conclusão em massa. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## tasks get

Consultar tarefa

```text
kobana dunning tasks get [ID] [OPÇÕES]
```

`GET /v1/tasks/{id}`

Retorna uma tarefa pelo ID, com resumos de cliente, responsável e cobrança.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Tarefa encontrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## tasks update

Atualizar tarefa

```text
kobana dunning tasks update [ID] [OPÇÕES]
```

`PUT /v1/tasks/{id}`

Atualiza os dados de uma tarefa (parcial).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_id` | `string (uuid)` | não | Cliente vinculado à tarefa. |
| `charge_id` | `string (uuid)` | não | Cobrança vinculada à tarefa. |
| `assigned_to_id` | `string (uuid)` | não | Usuário responsável pela tarefa. |
| `title` | `string` | não | Título da tarefa. |
| `description` | `string` | não | Descrição detalhada. |
| `task_type` | `string` | não | Tipo (`call` \| `email` \| `visit` \| `review` \| `follow_up`). Valores: `call`, `email`, `visit`, `review`, `follow_up`. |
| `priority` | `string` | não | Prioridade (`low` \| `medium` \| `high` \| `urgent`). Valores: `low`, `medium`, `high`, `urgent`. |
| `status` | `string` | não | Novo status; `completed` preenche `completed_at` automaticamente. Valores: `pending`, `in_progress`, `completed`, `cancelled`. |
| `due_at` | `string (date-time)` | não | Prazo da tarefa (ISO 8601). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_data_mode` | `string` | não | Como aplicar `custom_data` no update: `replace` (default) sobrescreve o objeto inteiro; `merge` mescla (deep) o payload sobre o valor armazenado — objetos aninhados mesclam, escalares/arrays/null sobrescrevem a chave. Valores: `replace`, `merge`. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.tasks.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Tarefa atualizada. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## tasks complete

Concluir tarefa

```text
kobana dunning tasks complete [ID] [OPÇÕES]
```

`POST /v1/tasks/{id}/complete`

Marca a tarefa como concluída e preenche `completed_at`. Retorna 409 se a tarefa já estiver concluída ou cancelada.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Tarefa concluída. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |
