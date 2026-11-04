---
id: operation-approvals
title: "kobana inbox operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana inbox operation-approvals


Comandos de `kobana inbox operation-approvals`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation-approvals` | Listar aprovações de operação |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation-approvals/{id}` | Consultar aprovação de operação |

## operation-approvals list

Listar aprovações de operação

```text
kobana inbox operation-approvals list [OPÇÕES]
```

`GET /v1/operation-approvals`

As aprovações de operação que uma tool MCP destrutiva pediu, da mais recente para a mais antiga, paginadas por página. Só as aprovações de quem chama — a pessoa do token — na organização do token e no workspace da chamada. Não exige escopo de recurso, como `/v1/me`: o dado é da própria pessoa. Um token sem pessoa não tem nenhuma. O link da aprovação, o token dele e o card guardado nunca são devolvidos.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Só as aprovações neste status; uma pendente vencida conta como `expired`. Valores: `pending`, `approved`, `executed`, `failed`, `declined`, `expired`. |
| `created_after` | `string` | não | Pedidas neste instante ISO 8601 ou depois. |
| `created_before` | `string` | não | Pedidas neste instante ISO 8601 ou antes. |
| `page` | `integer` | não | Número da página, padrão 1. |
| `per_page` | `integer` | não | Tamanho da página, padrão 25, máximo 100. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de aprovações. |
| `400` | Um parâmetro de consulta é inválido. |

## operation-approvals get

Consultar aprovação de operação

```text
kobana inbox operation-approvals get [ID] [OPÇÕES]
```

`GET /v1/operation-approvals/{id}`

Uma aprovação de operação: status, título, datas e resultado. Aprovação de outra pessoa, de outra organização ou id inexistente respondem o mesmo `404 NOT_FOUND`. Só as aprovações de quem chama — a pessoa do token — na organização do token e no workspace da chamada. Não exige escopo de recurso, como `/v1/me`: o dado é da própria pessoa. Um token sem pessoa não tem nenhuma. O link da aprovação, o token dele e o card guardado nunca são devolvidos.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
