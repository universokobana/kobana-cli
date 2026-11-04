---
id: system-events
title: "kobana inbox system-events"
sidebar_label: "system-events"
---

# kobana inbox system-events


Comandos de `kobana inbox system-events`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`system-events list`](#system-events-list) | `GET /v1/system-events` | Listar system events |
| [`system-events get`](#system-events-get) | `GET /v1/system-events/{id}` | Obter system event |

## system-events list

Listar system events

```text
kobana inbox system-events list [OPÇÕES]
```

`GET /v1/system-events`

Paginação keyset. Filtros são query params tipados (`campo=valor`, `campo[op]=valor`); filtros desconhecidos, campos não ordenáveis e valores malformados são rejeitados com 400 `VALIDATION_ERROR`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `slug`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `slug`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `slug` | `string` | não | Igualdade. Também `slug[in]=a,b`. |
| `inbox_id` | `string` | não | Igualdade. Também `inbox_id[in]=a,b`. |
| `email_id` | `string` | não | Igualdade. Também `email_id[in]=a,b`. |
| `created_at` | `string` | não | Igualdade. Também `created_at[in]=a,b` e `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | não |  |
| `created_at[lte]` | `string` | não |  |

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

## system-events get

Obter system event

```text
kobana inbox system-events get [ID] [OPÇÕES]
```

`GET /v1/system-events/{id}`

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
