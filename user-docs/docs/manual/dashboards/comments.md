---
id: comments
title: "kobana dashboards comments"
sidebar_label: "comments"
---

# kobana dashboards comments


Comandos de `kobana dashboards comments`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`comments list`](#comments-list) | `GET /v1/comments` | Listar comentários |
| [`comments create`](#comments-create) | `POST /v1/comments` | Criar comentário |
| [`comments get`](#comments-get) | `GET /v1/comments/{id}` | Buscar comentário |
| [`comments delete`](#comments-delete) | `DELETE /v1/comments/{id}` | Excluir comentário |

## comments list

Listar comentários

```text
kobana dashboards comments list [OPÇÕES]
```

`GET /v1/comments`

Os comentários da conversa de um tenant (`tenant_id` ou `portal_user_id`) ou, sem nenhum dos dois, de todas as conversas do workspace. Mais recentes primeiro, paginado.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal donos da conversa — todos os usuários do mesmo tenant veem os mesmos comentários. Informe este ou `portal_user_id`. |
| `portal_user_id` | `string` | não | Um usuário do portal (id ou external_id): vale a conversa do tenant dele (um usuário sem tenant tem uma conversa só dele). Informe este ou `tenant_id`. |
| `internal` | `string` | não | `true` = só notas internas; `false` = só o que o portal mostra. Valores: `true`, `false`. |
| `author_type` | `string` | não | Só comentários escritos por `portal_user` (usuário do portal), `member` (membro da equipe) ou `api`. Valores: `portal_user`, `member`, `api`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comentários. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## comments create

Criar comentário

```text
kobana dashboards comments create [OPÇÕES]
```

`POST /v1/comments`

Adiciona um comentário à conversa de um tenant, em nome da equipe. Todos os usuários do portal desse tenant veem a mensagem no bloco Comentários — a menos que `internal` seja `true`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `body` | `string` | sim | A mensagem (até 5000 caracteres). |
| `tenant_id` | `string` | não | Tenant ID dos usuários do portal donos da conversa — todos os usuários do mesmo tenant veem os mesmos comentários. Informe este ou `portal_user_id`. |
| `portal_user_id` | `string` | não | Um usuário do portal (id ou external_id): vale a conversa do tenant dele (um usuário sem tenant tem uma conversa só dele). Informe este ou `tenant_id`. |
| `internal` | `boolean` | não | Nota interna: só a equipe vê (dashboard, API, MCP); o portal nunca mostra. |
| `author_name` | `string` | não | Nome mostrado ao lado da mensagem. Na criação, o padrão é o e-mail do operador do token ou "Equipe". |

Outros campos e objetos aninhados: `kobana schema dashboards.comments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Comentário criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## comments get

Buscar comentário

```text
kobana dashboards comments get [ID] [OPÇÕES]
```

`GET /v1/comments/{id}`

Um comentário.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O comentário. |
| `404` | Recurso não encontrado na organização atual. |

## comments delete

Excluir comentário

```text
kobana dashboards comments delete [ID] [OPÇÕES]
```

`DELETE /v1/comments/{id}`

Tira o comentário da conversa (exclusão lógica): ele some do portal e do dashboard para todos.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comentário excluído. |
| `404` | Recurso não encontrado na organização atual. |
