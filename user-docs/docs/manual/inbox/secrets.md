---
id: secrets
title: "kobana inbox secrets"
sidebar_label: "secrets"
---

# kobana inbox secrets


Comandos de `kobana inbox secrets`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`secrets list`](#secrets-list) | `GET /v1/secrets` | Listar secrets |
| [`secrets create`](#secrets-create) | `POST /v1/secrets` | Criar secret |
| [`secrets update`](#secrets-update) | `PUT /v1/secrets` | Criar ou atualizar secret |
| [`secrets get`](#secrets-get) | `GET /v1/secrets/{id}` | Obter secret |
| [`secrets delete`](#secrets-delete) | `DELETE /v1/secrets/{id}` | Excluir secret |

## secrets list

Listar secrets

```text
kobana inbox secrets list [OPÇÕES]
```

`GET /v1/secrets`

Paginação por keyset. Os filtros são query params tipados (`campo=valor`, `campo[op]=valor`). O valor de um secret nunca é devolvido por nenhum endpoint: ele é guardado criptografado e só é lido pelo executor de workflows. Para rotacioná-lo, faça `PATCH` na linha (ou `PUT` por `key`) com um novo `value`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `updated_at`, `key`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `updated_at`, `key`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `key` | `string` | não | Equality. Also `key[in]=a,b`. |
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

## secrets create

Criar secret

```text
kobana inbox secrets create [OPÇÕES]
```

`POST /v1/secrets`

Guarda um secret que um workflow do agente pode referenciar como `${{secrets.KEY}}`. Uma `key` duplicada no workspace é recusada com 409 `CONFLICT`. O valor é somente escrita: guardado criptografado e nunca devolvido.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | sim |  |
| `value` | `string` | sim | Guardado criptografado; nunca devolvido. |
| `notes` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema inbox.secrets.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## secrets update

Criar ou atualizar secret

```text
kobana inbox secrets update [OPÇÕES]
```

`PUT /v1/secrets`

Upsert casado por `key` dentro do workspace de quem chama — o caminho da rotação, sem precisar buscar o id. Cria com 201 quando a chave é nova, substitui o valor com 200 quando já existe. O valor é somente escrita.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | `string` | sim | Chave de busca do upsert. |
| `value` | `string` | sim |  |
| `notes` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema inbox.secrets.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## secrets get

Obter secret

```text
kobana inbox secrets get [ID] [OPÇÕES]
```

`GET /v1/secrets/{id}`

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

## secrets delete

Excluir secret

```text
kobana inbox secrets delete [ID] [OPÇÕES]
```

`DELETE /v1/secrets/{id}`

Soft delete. Passos de workflow que referenciam a chave passam a falhar com `secret not defined` a partir da próxima execução.

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
