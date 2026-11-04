---
id: exports
title: "kobana dashboards exports"
sidebar_label: "exports"
---

# kobana dashboards exports


Comandos de `kobana dashboards exports`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`exports list`](#exports-list) | `GET /v1/exports` | Listar exportações |
| [`exports create`](#exports-create) | `POST /v1/exports` | Solicitar exportação |
| [`exports get`](#exports-get) | `GET /v1/exports/{id}` | Consultar exportação |
| [`exports download`](#exports-download) | `GET /v1/exports/{id}/download` | Baixar arquivo da exportação |

## exports list

Listar exportações

```text
kobana dashboards exports list [OPÇÕES]
```

`GET /v1/exports`

Lista os pedidos de exportação do workspace.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `resource_type` | `string` | não | Recurso exportado, ex.: `portal_users`. |
| `status` | `string` | não | Filtra pelo status da exportação. Valores: `pending`, `processing`, `completed`, `failed`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de exportações. |
| `404` | Recurso não encontrado na organização atual. |

## exports create

Solicitar exportação

```text
kobana dashboards exports create [OPÇÕES]
```

`POST /v1/exports`

Agenda a exportação dos usuários do portal do workspace em CSV, XLSX ou JSON. Assíncrono: responde 202 com a exportação `pending`; consulte `GET /exports/{id}` até `completed` e baixe em `/download`. O arquivo expira em 7 dias.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `resource_type` | `string` | sim | O que exportar. Hoje: `portal_users`. Valores: `portal_users`. |
| `format` | `string` | não | `csv` (padrão), `xlsx` ou `json`. Valores: `csv`, `xlsx`, `json`. |
| `filters` | `object` | não | Filtros opcionais das linhas exportadas. |

Outros campos e objetos aninhados: `kobana schema dashboards.exports.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Exportação agendada. |
| `400` | Corpo ausente, JSON inválido ou campos incompatíveis. |
| `404` | Recurso não encontrado na organização atual. |

## exports get

Consultar exportação

```text
kobana dashboards exports get [ID] [OPÇÕES]
```

`GET /v1/exports/{id}`

Status e metadados de um pedido de exportação.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A exportação. |
| `404` | Recurso não encontrado na organização atual. |

## exports download

Baixar arquivo da exportação

```text
kobana dashboards exports download [ID] [OPÇÕES]
```

`GET /v1/exports/{id}/download`

Devolve o arquivo de uma exportação concluída (o painel redireciona para uma URL assinada; a API entrega os bytes). 404 enquanto não estiver pronta.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O arquivo, como anexo. |
| `404` | Recurso não encontrado na organização atual. |
