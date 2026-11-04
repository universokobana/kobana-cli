---
id: operation-approvals
title: "kobana billing operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana billing operation-approvals


Comandos de `kobana billing operation-approvals`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation_approvals` | Listar suas aprovações de operação |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation_approvals/{id}` | Consultar uma aprovação de operação |

## operation-approvals list

Listar suas aprovações de operação

```text
kobana billing operation-approvals list [OPÇÕES]
```

`GET /v1/operation_approvals`

As aprovações por link da pessoa dona do token, na organização do token, da mais recente para a mais antiga: cada tool destrutiva do MCP que esperou a confirmação na tela de aprovação, com o que aconteceu depois. Qualquer token com pessoa consulta as próprias aprovações, sem escopo de recurso; um token sem pessoa recebe a lista vazia. O retorno de cada operação (`result`, `error`, o `amount` lido do registro) só vem quando o token lê o recurso daquela aprovação e a pessoa ainda tem permissão de leitura dele; senão vem `null` com `result_hidden: true`. Só leitura: nada aqui aprova, cancela ou reabre uma aprovação. O `title` vem no idioma do `Accept-Language` (pt-BR por padrão).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `status` | `string` | não | Só aprovações nesta situação. `pending` deixa de fora a pendente vencida, que é `expired`. Valores: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | não | Só aprovações pedidas a partir deste instante (ISO 8601). |
| `created_before` | `string` | não | Só aprovações pedidas até este instante (ISO 8601). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de aprovações. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## operation-approvals get

Consultar uma aprovação de operação

```text
kobana billing operation-approvals get [ID] [OPÇÕES]
```

`GET /v1/operation_approvals/{id}`

Uma aprovação da pessoa dona do token, na organização do token — o `approval.id` que a tool do MCP devolveu com `approval_required`. Aprovação de outra pessoa, de outra organização ou id inexistente respondem o mesmo 404. Nunca devolve o link, o token do link, os argumentos nem o retrato da tela.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A aprovação. |
| `404` | Recurso não encontrado na organização atual. |
