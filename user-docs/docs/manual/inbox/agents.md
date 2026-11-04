---
id: agents
title: "kobana inbox agents"
sidebar_label: "agents"
---

# kobana inbox agents


Comandos de `kobana inbox agents`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`agents list`](#agents-list) | `GET /v1/agents` | Listar agents |
| [`agents create`](#agents-create) | `POST /v1/agents` | Criar agent |
| [`agents update`](#agents-update) | `PUT /v1/agents` | Criar ou atualizar agente |
| [`agents get`](#agents-get) | `GET /v1/agents/{id}` | Obter agent |
| [`agents delete`](#agents-delete) | `DELETE /v1/agents/{id}` | Excluir agent |
| [`agents versions`](#agents-versions) | `GET /v1/agents/{id}/versions` | Listar versões do prompt do agente |

## agents list

Listar agents

```text
kobana inbox agents list [OPÇÕES]
```

`GET /v1/agents`

Paginação keyset. Filtros são query params tipados (`campo=valor`, `campo[op]=valor`); filtros desconhecidos, campos não ordenáveis e valores malformados são rejeitados com 400 `VALIDATION_ERROR`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `updated_at`, `label`, `version`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `updated_at`, `label`, `version`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `llm_model` | `string` | não | Igualdade. Também `llm_model[in]=a,b`. |
| `external_id` | `string` | não | Igualdade. Também `external_id[in]=a,b`. |
| `created_at` | `string` | não | Igualdade. Também `created_at[in]=a,b` e `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | não |  |
| `created_at[lte]` | `string` | não |  |
| `updated_at` | `string` | não | Igualdade. Também `updated_at[in]=a,b` e `updated_at[gt\|gte\|lt\|lte]`. |
| `updated_at[gte]` | `string` | não |  |
| `updated_at[lte]` | `string` | não |  |

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

## agents create

Criar agent

```text
kobana inbox agents create [OPÇÕES]
```

`POST /v1/agents`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `llm_model` | `string` | sim |  |
| `system_message` | `string` | sim |  |
| `external_id` | `string` | não | Opcional. Usa por padrão o `id` interno gerado. |
| `skills_repo` | `string (uri)` | não |  |
| `workflow_allowed_hosts` | `array<string>` | não | Hosts (exatos, ex.: `api.example.com`) para os quais um workflow devolvido por este agente pode enviar `${{secrets.*}}`, `${{email.attachments.*}}` ou a saída de um passo derivada deles. Vazio (o padrão) bloqueia todos os destinos para esses valores; passos sem eles não são afetados. |
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. Envie `null` para limpar. Máx. 64 KB serializados, 16 níveis de profundidade. |

Outros campos e objetos aninhados: `kobana schema inbox.agents.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## agents update

Criar ou atualizar agente

```text
kobana inbox agents update [OPÇÕES]
```

`PUT /v1/agents`

Upsert. Recebe o mesmo corpo do POST mais as chaves de busca `id` e `external_id`, casadas nessa ordem. Um match é atualizado in place e responde 200 — campos ausentes do corpo mantêm o valor armazenado. Sem chave, ou sem match, cria o agente exatamente como o POST faria e responde 201.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `llm_model` | `string` | sim |  |
| `system_message` | `string` | sim |  |
| `external_id` | `string` | não | Casa pelo id do sistema de origem. Também gravado no registro quando ele é criado. |
| `skills_repo` | `string (uri)` | não |  |
| `workflow_allowed_hosts` | `array<string>` | não | Hosts (exatos, ex.: `api.example.com`) para os quais um workflow devolvido por este agente pode enviar `${{secrets.*}}`, `${{email.attachments.*}}` ou a saída de um passo derivada deles. Vazio (o padrão) bloqueia todos os destinos para esses valores; passos sem eles não são afetados. |
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. Envie `null` para limpar. Máx. 64 KB serializados, 16 níveis de profundidade. |
| `id` | `string (uuid)` | não | Casa pelo id interno. Verificado antes de `external_id`. |

Outros campos e objetos aninhados: `kobana schema inbox.agents.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## agents get

Obter agent

```text
kobana inbox agents get [ID] [OPÇÕES]
```

`GET /v1/agents/{id}`

Aceita tanto o UUID interno quanto o `external_id` do recurso.

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

## agents delete

Excluir agent

```text
kobana inbox agents delete [ID] [OPÇÕES]
```

`DELETE /v1/agents/{id}`

Aceita tanto o UUID interno quanto o `external_id` do recurso.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Excluído (soft delete). |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## agents versions

Listar versões do prompt do agente

```text
kobana inbox agents versions [ID] [OPÇÕES]
```

`GET /v1/agents/{id}/versions`

Paginação keyset. Filtros são query params tipados (`campo=valor`, `campo[op]=valor`); filtros desconhecidos, campos não ordenáveis e valores malformados são rejeitados com 400 `VALIDATION_ERROR`.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `version`. Sem ele, a ordem é `version` decrescente. Valores: `created_at`, `version`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `version` | `integer` | não | Igualdade. Também `version[in]=a,b` e `version[gt\|gte\|lt\|lte]`. |
| `version[gte]` | `integer` | não |  |
| `version[lte]` | `integer` | não |  |
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
