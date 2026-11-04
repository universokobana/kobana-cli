---
id: checklists
title: "kobana dashboards checklists"
sidebar_label: "checklists"
---

# kobana dashboards checklists


Comandos de `kobana dashboards checklists`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`checklists progress`](#checklists-progress) | `GET /v1/checklists/{id}/progress` | Ver progresso do tenant |
| [`checklists progress file`](#checklists-progress-file) | `GET /v1/checklists/{id}/progress/file` | Baixar arquivo da tarefa |
| [`checklists progress file create`](#checklists-progress-file-create) | `POST /v1/checklists/{id}/progress/file` | Enviar arquivo da tarefa |
| [`checklists progress file delete`](#checklists-progress-file-delete) | `DELETE /v1/checklists/{id}/progress/file` | Remover arquivo da tarefa |
| [`checklists progress update`](#checklists-progress-update) | `PUT /v1/checklists/{id}/progress` | Alterar progresso do tenant |
| [`checklists progress delete`](#checklists-progress-delete) | `DELETE /v1/checklists/{id}/progress` | Reiniciar progresso do tenant |
| [`checklists list`](#checklists-list) | `GET /v1/checklists` | Listar checklists |
| [`checklists get`](#checklists-get) | `GET /v1/checklists/{id}` | Ver checklist |

## checklists progress

Ver progresso do tenant

```text
kobana dashboards checklists progress [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/checklists/{id}/progress`

Onde um tenant está numa checklist, tarefa por tarefa, com concluídas/total por etapa. Todos os usuários do portal do mesmo tenant veem esse mesmo progresso.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal donos do progresso — todos os usuários do mesmo tenant veem a mesma checklist. Informe este ou `portal_user_id`. |
| `portal_user_id` | `string` | não | Um usuário do portal (id ou external_id): vale o progresso do tenant dele (um usuário sem tenant tem uma checklist só dele). Informe este ou `tenant_id`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O progresso do tenant. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## checklists progress file

Baixar arquivo da tarefa

```text
kobana dashboards checklists progress file [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/checklists/{id}/progress/file`

O arquivo que o tenant enviou para uma tarefa do tipo Solicitação.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal donos do progresso — todos os usuários do mesmo tenant veem a mesma checklist. Informe este ou `portal_user_id`. |
| `portal_user_id` | `string` | não | Um usuário do portal (id ou external_id): vale o progresso do tenant dele (um usuário sem tenant tem uma checklist só dele). Informe este ou `tenant_id`. |
| `task_id` | `string` | sim | ID da tarefa. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O conteúdo do arquivo. |
| `404` | Recurso não encontrado na organização atual. |

## checklists progress file create

Enviar arquivo da tarefa

```text
kobana dashboards checklists progress file create [ID] [OPÇÕES]
```

`POST /v1/checklists/{id}/progress/file`

Envia, pelo tenant, o arquivo de uma tarefa do tipo Solicitação (`content_base64`), substituindo o anterior.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `task_id` | `string` | sim |  |
| `name` | `string` | sim |  |
| `content_base64` | `string` | sim | O conteúdo do arquivo em base64 (até o limite de 2 MB da requisição). |
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal donos do progresso — todos os usuários do mesmo tenant veem a mesma checklist. Informe este ou `portal_user_id`. |
| `portal_user_id` | `string` | não | Um usuário do portal (id ou external_id): vale o progresso do tenant dele (um usuário sem tenant tem uma checklist só dele). Informe este ou `tenant_id`. |
| `content_type` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.checklists.progress.file.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | O arquivo registrado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## checklists progress file delete

Remover arquivo da tarefa

```text
kobana dashboards checklists progress file delete [ID] [OPÇÕES]
```

`DELETE /v1/checklists/{id}/progress/file`

Retira o arquivo que o tenant enviou para uma tarefa.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal donos do progresso — todos os usuários do mesmo tenant veem a mesma checklist. Informe este ou `portal_user_id`. |
| `portal_user_id` | `string` | não | Um usuário do portal (id ou external_id): vale o progresso do tenant dele (um usuário sem tenant tem uma checklist só dele). Informe este ou `tenant_id`. |
| `task_id` | `string` | sim | ID da tarefa. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Remover arquivo da tarefa |
| `404` | Recurso não encontrado na organização atual. |

## checklists progress update

Alterar progresso do tenant

```text
kobana dashboards checklists progress update [ID] [OPÇÕES]
```

`PUT /v1/checklists/{id}/progress`

Define o status de uma ou mais tarefas para o tenant (cria o progresso no primeiro uso). Os usuários do tenant veem a mudança no portal.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `task_ids` | `array<string>` | sim |  |
| `status` | `string` | sim | `pending`, `in_progress` ou `done`. Uma tarefa que ninguém tocou é `pending`. Valores: `pending`, `in_progress`, `done`. |
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal donos do progresso — todos os usuários do mesmo tenant veem a mesma checklist. Informe este ou `portal_user_id`. |
| `portal_user_id` | `string` | não | Um usuário do portal (id ou external_id): vale o progresso do tenant dele (um usuário sem tenant tem uma checklist só dele). Informe este ou `tenant_id`. |

Outros campos e objetos aninhados: `kobana schema dashboards.checklists.progress.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O progresso do tenant. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## checklists progress delete

Reiniciar progresso do tenant

```text
kobana dashboards checklists progress delete [ID] [OPÇÕES]
```

`DELETE /v1/checklists/{id}/progress`

Volta as tarefas de `task_ids` (todas, sem ele) para pendente e esquece os arquivos enviados. É uma atualização: o histórico fica nos eventos e na auditoria.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal donos do progresso — todos os usuários do mesmo tenant veem a mesma checklist. Informe este ou `portal_user_id`. |
| `portal_user_id` | `string` | não | Um usuário do portal (id ou external_id): vale o progresso do tenant dele (um usuário sem tenant tem uma checklist só dele). Informe este ou `tenant_id`. |
| `task_ids` | `string` | não | IDs das tarefas, separados por vírgula. Sem ele, todas as tarefas da checklist. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O progresso do tenant. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## checklists list

Listar checklists

```text
kobana dashboards checklists list [OPÇÕES]
```

`GET /v1/checklists`

Os blocos Lista de Tarefas do portal publicado do workspace, página por página. As tarefas são escritas no builder; o progresso é de cada tenant (`GET /checklists/{id}/progress`).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | As checklists. |
| `404` | Recurso não encontrado na organização atual. |

## checklists get

Ver checklist

```text
kobana dashboards checklists get [ID] [OPÇÕES]
```

`GET /v1/checklists/{id}`

Uma checklist do portal publicado: etapas e tarefas visíveis.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A checklist. |
| `404` | Recurso não encontrado na organização atual. |
