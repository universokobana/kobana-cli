---
id: workspaces
title: "kobana inbox workspaces"
sidebar_label: "workspaces"
---

# kobana inbox workspaces


Comandos de `kobana inbox workspaces`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`workspaces list`](#workspaces-list) | `GET /v1/workspaces` | Listar workspaces |
| [`workspaces create`](#workspaces-create) | `POST /v1/workspaces` | Criar workspace |
| [`workspaces update`](#workspaces-update) | `PUT /v1/workspaces` | Criar ou atualizar workspace |
| [`workspaces get`](#workspaces-get) | `GET /v1/workspaces/{id}` | Obter workspace |
| [`workspaces delete`](#workspaces-delete) | `DELETE /v1/workspaces/{id}` | Excluir workspace |

## workspaces list

Listar workspaces

```text
kobana inbox workspaces list [OPÇÕES]
```

`GET /v1/workspaces`

Escopo: a organização do workspace do token — todo workspace vivo dessa organização é alcançável; um fora dela é 404. Um token cujo workspace não tem organização enxerga só a si mesmo. Paginação keyset com filtros tipados.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `updated_at`, `label`, `slug`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `updated_at`, `label`, `slug`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `source` | `string` | não | Igualdade. Também `source[in]=a,b`. |
| `slug` | `string` | não | Igualdade. Também `slug[in]=a,b`. |
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

## workspaces create

Criar workspace

```text
kobana inbox workspaces create [OPÇÕES]
```

`POST /v1/workspaces`

Exige token `src=system`. O workspace é criado dentro da organização do token. Escopo: a organização do workspace do token — todo workspace vivo dessa organização é alcançável; um fora dela é 404. Um token cujo workspace não tem organização enxerga só a si mesmo.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `source` | `string` | sim | Valores: `kobana_app`, `kobana_billing`, `kobana_inbox`, `kobana_agent`, `kobana_finance`, `kobana_legal`, `kobana_team`. |
| `label` | `string` | sim |  |
| `external_id` | `string` | não | Opcional. Quando omitido, o servidor usa por padrão o `id` interno gerado como `external_id`. |
| `slug` | `string` | não | Opcional. Slug minúsculo e dasherized, único dentro da organização, usado como segmento do meio dos endereços de caixa. Limitado a 20 caracteres para que a parte local composta caiba no limite SMTP de 64 caracteres. Quando omitido, o servidor deriva do `label` dasherized. |
| `monthly_cost_limit` | `number` | não |  |
| `monthly_emails_limit` | `integer` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. Envie `null` para limpar. Máx. 64 KB serializados, 16 níveis de profundidade. |

Outros campos e objetos aninhados: `kobana schema inbox.workspaces.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## workspaces update

Criar ou atualizar workspace

```text
kobana inbox workspaces update [OPÇÕES]
```

`PUT /v1/workspaces`

Upsert. Recebe o mesmo corpo do POST mais as chaves de busca `id` e `external_id`, casadas nessa ordem. Um match é atualizado in place e responde 200 — campos ausentes do corpo mantêm o valor armazenado. Sem chave, ou sem match, cria o workspace exatamente como o POST faria e responde 201. Escopo: a organização do workspace do token — todo workspace vivo dessa organização é alcançável; um fora dela é 404. Um token cujo workspace não tem organização enxerga só a si mesmo. Exige token `src=system`; `external_id` não pode ser alterado em um workspace casado (409).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `source` | `string` | sim | Valores: `kobana_app`, `kobana_billing`, `kobana_inbox`, `kobana_agent`, `kobana_finance`, `kobana_legal`, `kobana_team`. |
| `label` | `string` | sim |  |
| `external_id` | `string` | não | Casa pelo id do sistema de origem. Também gravado no registro quando ele é criado. |
| `slug` | `string` | não | Opcional. Slug minúsculo e dasherized, único dentro da organização, usado como segmento do meio dos endereços de caixa. Limitado a 20 caracteres para que a parte local composta caiba no limite SMTP de 64 caracteres. Quando omitido, o servidor deriva do `label` dasherized. |
| `monthly_cost_limit` | `number` | não |  |
| `monthly_emails_limit` | `integer` | não |  |
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. Envie `null` para limpar. Máx. 64 KB serializados, 16 níveis de profundidade. |
| `id` | `string (uuid)` | não | Casa pelo id interno. Verificado antes de `external_id`. |

Outros campos e objetos aninhados: `kobana schema inbox.workspaces.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## workspaces get

Obter workspace

```text
kobana inbox workspaces get [ID] [OPÇÕES]
```

`GET /v1/workspaces/{id}`

Aceita tanto o UUID interno quanto o `external_id` do recurso. Escopo: a organização do workspace do token — todo workspace vivo dessa organização é alcançável; um fora dela é 404. Um token cujo workspace não tem organização enxerga só a si mesmo.

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

## workspaces delete

Excluir workspace

```text
kobana inbox workspaces delete [ID] [OPÇÕES]
```

`DELETE /v1/workspaces/{id}`

Aceita tanto o UUID interno quanto o `external_id` do recurso. Escopo: a organização do workspace do token — todo workspace vivo dessa organização é alcançável; um fora dela é 404. Um token cujo workspace não tem organização enxerga só a si mesmo. Exige token `src=system`.

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
