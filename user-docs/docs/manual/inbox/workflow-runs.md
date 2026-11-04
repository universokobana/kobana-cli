---
id: workflow-runs
title: "kobana inbox workflow-runs"
sidebar_label: "workflow-runs"
---

# kobana inbox workflow-runs


Comandos de `kobana inbox workflow-runs`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`workflow-runs list`](#workflow-runs-list) | `GET /v1/workflow-runs` | Listar execuções de workflow |
| [`workflow-runs get`](#workflow-runs-get) | `GET /v1/workflow-runs/{id}` | Obter execução de workflow |

## workflow-runs list

Listar execuções de workflow

```text
kobana inbox workflow-runs list [OPÇÕES]
```

`GET /v1/workflow-runs`

Paginação por keyset. Os filtros são query params tipados (`campo=valor`, `campo[op]=valor`). Os passos não vêm embutidos na listagem — `GET /v1/workflow-runs/{id}` os devolve.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `updated_at`, `status`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `updated_at`, `status`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Igualdade. Também `status[in]=a,b`. Valores: `pending`, `running`, `success`, `failed`. |
| `email_id` | `string` | não | Igualdade. Também `email_id[in]=a,b`. |
| `inbox_id` | `string` | não | Igualdade. Também `inbox_id[in]=a,b`. |
| `agent_id` | `string` | não | Igualdade. Também `agent_id[in]=a,b`. |
| `agent_run_id` | `string` | não | Equality. Also `agent_run_id[in]=a,b`. |
| `created_at` | `string` | não | Igualdade. Também `created_at[in]=a,b` e `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | não |  |
| `created_at[lte]` | `string` | não |  |
| `finished_at` | `string` | não | Igualdade. Também `finished_at[in]=a,b` e `finished_at[gt\|gte\|lt\|lte]`. |
| `finished_at[gte]` | `string` | não |  |
| `finished_at[lte]` | `string` | não |  |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de resultados. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## workflow-runs get

Obter execução de workflow

```text
kobana inbox workflow-runs get [ID] [OPÇÕES]
```

`GET /v1/workflow-runs/{id}`

A execução com todos os passos na ordem de execução. Requisições e respostas são guardadas com os valores de secrets substituídos por `[REDACTED]`; os corpos são truncados em 4000 caracteres.

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
