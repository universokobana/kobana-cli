---
id: operation-approvals
title: "kobana dunning operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana dunning operation-approvals


Comandos de `kobana dunning operation-approvals`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation-approvals` | Listar minhas aprovações |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation-approvals/{id}` | Consultar uma aprovação |

## operation-approvals list

Listar minhas aprovações

```text
kobana dunning operation-approvals list [OPÇÕES]
```

`GET /v1/operation-approvals`

Lista as aprovações por link da pessoa do token, na organização do token e no workspace da chamada, da mais recente para a mais antiga. Só leitura: nada aqui aprova, cancela ou reabre uma aprovação. Não exige permissão de papel nem escopo de recurso próprio — o dado é da própria pessoa; basta o nível de leitura.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `status` | `string` | não | Só esta situação. `expired` inclui a pendente vencida; `pending` só a que ainda vale. Valores: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | não | Pedidas a partir deste instante (ISO 8601, inclusive). |
| `created_before` | `string` | não | Pedidas antes deste instante (ISO 8601, exclusivo). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Aprovações da pessoa. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## operation-approvals get

Consultar uma aprovação

```text
kobana dunning operation-approvals get [ID] [OPÇÕES]
```

`GET /v1/operation-approvals/{id}`

Situação e resultado de uma aprovação por link, pelo `approval.id` que a tool destrutiva devolveu em `approval_required`. Chame depois que a pessoa disser que decidiu na tela — não em laço. Aprovação de outra pessoa, de outra organização ou id inexistente: o mesmo 404. O `result` sai com segredos mascarados.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A aprovação. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
