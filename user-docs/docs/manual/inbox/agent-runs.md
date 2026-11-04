---
id: agent-runs
title: "kobana inbox agent-runs"
sidebar_label: "agent-runs"
---

# kobana inbox agent-runs


Comandos de `kobana inbox agent-runs`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`agent-runs list`](#agent-runs-list) | `GET /v1/agent-runs` | Listar agent runs |
| [`agent-runs get`](#agent-runs-get) | `GET /v1/agent-runs/{id}` | Obter agent run |

## agent-runs list

Listar agent runs

```text
kobana inbox agent-runs list [OPÇÕES]
```

`GET /v1/agent-runs`

Paginação keyset. Filtros são query params tipados (`campo=valor`, `campo[op]=valor`); filtros desconhecidos, campos não ordenáveis e valores malformados são rejeitados com 400 `VALIDATION_ERROR`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `updated_at`, `total_cost`, `status`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `updated_at`, `total_cost`, `status`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Igualdade. Também `status[in]=a,b`. Valores: `running`, `success`, `failed`. |
| `agent_id` | `string` | não | Igualdade. Também `agent_id[in]=a,b`. |
| `email_id` | `string` | não | Igualdade. Também `email_id[in]=a,b`. |
| `inbox_id` | `string` | não | Igualdade. Também `inbox_id[in]=a,b`. |
| `llm_model` | `string` | não | Igualdade. Também `llm_model[in]=a,b`. |
| `created_at` | `string` | não | Igualdade. Também `created_at[in]=a,b` e `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | não |  |
| `created_at[lte]` | `string` | não |  |
| `finished_at` | `string` | não | Igualdade. Também `finished_at[in]=a,b` e `finished_at[gt\|gte\|lt\|lte]`. |
| `finished_at[gte]` | `string` | não |  |
| `finished_at[lte]` | `string` | não |  |
| `total_cost` | `number` | não | Igualdade. Também `total_cost[in]=a,b` e `total_cost[gt\|gte\|lt\|lte]`. |
| `total_cost[gte]` | `number` | não |  |
| `total_cost[lte]` | `number` | não |  |

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

## agent-runs get

Obter agent run

```text
kobana inbox agent-runs get [ID] [OPÇÕES]
```

`GET /v1/agent-runs/{id}`

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
