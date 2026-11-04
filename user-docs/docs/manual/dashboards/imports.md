---
id: imports
title: "kobana dashboards imports"
sidebar_label: "imports"
---

# kobana dashboards imports


Comandos de `kobana dashboards imports`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports` | Listar importações |
| [`imports create`](#imports-create) | `POST /v1/imports` | Importar usuários do portal por CSV |
| [`imports get`](#imports-get) | `GET /v1/imports/{id}` | Consultar importação |

## imports list

Listar importações

```text
kobana dashboards imports list [OPÇÕES]
```

`GET /v1/imports`

Lista os pedidos de importação do workspace, com contadores de sucesso e erro.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `resource_type` | `string` | não | Recurso importado, ex.: `portal_users`. |
| `status` | `string` | não | Filtra pelo status da importação. Valores: `pending`, `processing`, `completed`, `completed_with_errors`, `failed`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de importações. |
| `404` | Recurso não encontrado na organização atual. |

## imports create

Importar usuários do portal por CSV

```text
kobana dashboards imports create [OPÇÕES]
```

`POST /v1/imports`

Envia um CSV em base64 e agenda a importação de usuários do portal no workspace. O painel faz upload prévio do arquivo; a API o recebe no corpo (limite de 2 MB por requisição). Segue a configuração da organização "permitir criar usuários do portal" (403 quando desligada). Assíncrono: 202; consulte `GET /imports/{id}`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `resource_type` | `string` | sim | O que importar. Hoje: `portal_users`. Valores: `portal_users`. |
| `file_name` | `string` | sim | Nome original do arquivo, ex.: `usuarios.csv`. |
| `content_base64` | `string` | sim | O CSV codificado em base64. |
| `content_type` | `string` | não | Tipo do arquivo. Padrão `text/csv`. |

Outros campos e objetos aninhados: `kobana schema dashboards.imports.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Importação agendada. |
| `400` | Corpo ausente, JSON inválido ou campos incompatíveis. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## imports get

Consultar importação

```text
kobana dashboards imports get [ID] [OPÇÕES]
```

`GET /v1/imports/{id}`

Status, contadores e erros por linha de uma importação. Uma importação ainda `pending` 2 minutos depois do pedido aparece como `failed` (fila parada), sem que a consulta altere o registro.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A importação. |
| `404` | Recurso não encontrado na organização atual. |
