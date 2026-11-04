---
id: webhooks
title: "kobana inbox webhooks"
sidebar_label: "webhooks"
---

# kobana inbox webhooks


Comandos de `kobana inbox webhooks`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`webhooks deliveries get`](#webhooks-deliveries-get) | `GET /v1/webhooks/{id}/deliveries/{deliveryId}` | Obter entrega de webhook |
| [`webhooks deliveries replay`](#webhooks-deliveries-replay) | `POST /v1/webhooks/{id}/deliveries/{deliveryId}/replay` | Reenviar (replay) uma entrega de webhook |
| [`webhooks deliveries list`](#webhooks-deliveries-list) | `GET /v1/webhooks/{id}/deliveries` | Listar entregas de webhook |
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | Listar webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Criar webhook |
| [`webhooks update`](#webhooks-update) | `PUT /v1/webhooks` | Criar ou atualizar webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Obter webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Excluir webhook |
| [`webhooks regenerate-secret`](#webhooks-regenerate-secret) | `POST /v1/webhooks/{id}/regenerate-secret` | Rotacionar o signing secret |
| [`webhooks test`](#webhooks-test) | `POST /v1/webhooks/{id}/test` | Enviar um evento webhook.ping |

## webhooks deliveries get

Obter entrega de webhook

```text
kobana inbox webhooks deliveries get [ID] [DELIVERYID] [OPÇÕES]
```

`GET /v1/webhooks/{id}/deliveries/{deliveryId}`

Aceita tanto o UUID interno quanto o `external_id` do recurso.

**Argumentos do caminho:** `id`, `deliveryId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## webhooks deliveries replay

Reenviar (replay) uma entrega de webhook

```text
kobana inbox webhooks deliveries replay [ID] [DELIVERYID] [OPÇÕES]
```

`POST /v1/webhooks/{id}/deliveries/{deliveryId}/replay`

Aceita tanto o UUID interno quanto o `external_id` do recurso.

**Argumentos do caminho:** `id`, `deliveryId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Aceito e enfileirado. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## webhooks deliveries list

Listar entregas de webhook

```text
kobana inbox webhooks deliveries list [ID] [OPÇÕES]
```

`GET /v1/webhooks/{id}/deliveries`

Aceita tanto o UUID interno quanto o `external_id` do recurso.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `attempt`, `status`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `attempt`, `status`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Igualdade. Também `status[in]=a,b`. Valores: `pending`, `success`, `failed`. |
| `attempt` | `integer` | não | Igualdade. Também `attempt[in]=a,b` e `attempt[gt\|gte\|lt\|lte]`. |
| `attempt[gte]` | `integer` | não |  |
| `attempt[lte]` | `integer` | não |  |
| `response_status` | `integer` | não | Igualdade. Também `response_status[in]=a,b` e `response_status[gt\|gte\|lt\|lte]`. |
| `response_status[gte]` | `integer` | não |  |
| `response_status[lte]` | `integer` | não |  |
| `system_event_id` | `string` | não | Igualdade. Também `system_event_id[in]=a,b`. |
| `created_at` | `string` | não | Igualdade. Também `created_at[in]=a,b` e `created_at[gt\|gte\|lt\|lte]`. |
| `created_at[gte]` | `string` | não |  |
| `created_at[lte]` | `string` | não |  |
| `delivered_at` | `string` | não | Igualdade. Também `delivered_at[in]=a,b` e `delivered_at[gt\|gte\|lt\|lte]`. |
| `delivered_at[gte]` | `string` | não |  |
| `delivered_at[lte]` | `string` | não |  |

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

## webhooks list

Listar webhooks

```text
kobana inbox webhooks list [OPÇÕES]
```

`GET /v1/webhooks`

Paginação keyset. Filtros são query params tipados (`campo=valor`, `campo[op]=valor`); filtros desconhecidos, campos não ordenáveis e valores malformados são rejeitados com 400 `VALIDATION_ERROR`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `integer` | não |  |
| `cursor` | `string` | não | Cursor opaco retornado em `meta.next_cursor` da página anterior. |
| `sort_by` | `string` | não | Campo de ordenação. Permitidos: `created_at`, `updated_at`, `label`. Sem ele, a ordem é `created_at` decrescente. Valores: `created_at`, `updated_at`, `label`. |
| `sort_order` | `string` | não | `asc` (padrão) ou `desc`. Exige `sort_by`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Igualdade. Também `status[in]=a,b`. Valores: `active`, `inactive`. |
| `inbox_id` | `string` | não | Igualdade. Também `inbox_id[in]=a,b`. |
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

## webhooks create

Criar webhook

```text
kobana inbox webhooks create [OPÇÕES]
```

`POST /v1/webhooks`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `url` | `string (uri)` | sim |  |
| `external_id` | `string` | não | Opcional. Usa por padrão o `id` interno gerado. |
| `inbox_id` | `string (uuid)` | não |  |
| `subscribed_events` | `array<string>` | não |  |
| `auth` | `string` | não | Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_basic_user` | `string` | não |  |
| `auth_basic_password` | `string` | não |  |
| `auth_bearer_token` | `string` | não |  |
| `auth_header_key` | `string` | não |  |
| `auth_header_value` | `string` | não |  |
| `signing_secret` | `string` | não | Opcional. Traga seu próprio signing secret: `whsec_` + base64 padrão de 24-64 bytes aleatórios. Gerado (32 bytes) quando omitido. Retornado em texto claro somente nesta resposta. |
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. Envie `null` para limpar. Máx. 64 KB serializados, 16 níveis de profundidade. |

Outros campos e objetos aninhados: `kobana schema inbox.webhooks.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## webhooks update

Criar ou atualizar webhook

```text
kobana inbox webhooks update [OPÇÕES]
```

`PUT /v1/webhooks`

Upsert. Recebe o mesmo corpo do POST mais as chaves de busca `id` e `external_id`, casadas nessa ordem. Um match é atualizado in place e responde 200 — campos ausentes do corpo mantêm o valor armazenado. Sem chave, ou sem match, cria o webhook exatamente como o POST faria e responde 201.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `url` | `string (uri)` | sim |  |
| `external_id` | `string` | não | Casa pelo id do sistema de origem. Também gravado no registro quando ele é criado. |
| `inbox_id` | `string (uuid)` | não |  |
| `subscribed_events` | `array<string>` | não |  |
| `auth` | `string` | não | Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_basic_user` | `string` | não |  |
| `auth_basic_password` | `string` | não |  |
| `auth_bearer_token` | `string` | não |  |
| `auth_header_key` | `string` | não |  |
| `auth_header_value` | `string` | não |  |
| `signing_secret` | `string` | não | Opcional. Traga seu próprio signing secret: `whsec_` + base64 padrão de 24-64 bytes aleatórios. Gerado (32 bytes) quando omitido. Retornado em texto claro somente nesta resposta. |
| `custom_metadata` | `object` | não | Metadados do usuário, formato livre. Envie `null` para limpar. Máx. 64 KB serializados, 16 níveis de profundidade. |
| `id` | `string (uuid)` | não | Casa pelo id interno. Verificado antes de `external_id`. |

Outros campos e objetos aninhados: `kobana schema inbox.webhooks.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso. |
| `201` | Sucesso. |
| `400` | O corpo falhou na validação Zod. `error.details` carrega `formErrors`/`fieldErrors`. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
| `409` | Restrição de unicidade violada (ex.: `external_id` ou `email` da inbox já em uso). |
| `422` | Validação do corpo que requer contexto do lado do servidor (ex.: domínio da inbox incompatível). |

## webhooks get

Obter webhook

```text
kobana inbox webhooks get [ID] [OPÇÕES]
```

`GET /v1/webhooks/{id}`

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

## webhooks delete

Excluir webhook

```text
kobana inbox webhooks delete [ID] [OPÇÕES]
```

`DELETE /v1/webhooks/{id}`

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

## webhooks regenerate-secret

Rotacionar o signing secret

```text
kobana inbox webhooks regenerate-secret [ID] [OPÇÕES]
```

`POST /v1/webhooks/{id}/regenerate-secret`

Substitui o signing secret e retorna o novo valor **uma única vez**. Por padrão o secret anterior continua assinando entregas por 24 horas (`previous_secret_grace_hours`), período em que cada entrega carrega uma assinatura por secret, para que o receptor troque a chave sem perder eventos. Informe `0` para revogar o secret anterior imediatamente (secret vazado). Aceita tanto o UUID interno quanto o `external_id` do recurso.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `signing_secret` | `string` | não | Opcional. Traga seu próprio signing secret: `whsec_` + base64 padrão de 24-64 bytes aleatórios. Gerado (32 bytes) quando omitido. Retornado em texto claro somente nesta resposta. |
| `previous_secret_grace_hours` | `integer` | não | Horas em que o secret anterior permanece válido após a rotação. `0` revoga imediatamente. |

Outros campos e objetos aninhados: `kobana schema inbox.webhooks.regenerate-secret`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Novo signing secret (exibido uma única vez). |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |

## webhooks test

Enviar um evento webhook.ping

```text
kobana inbox webhooks test [ID] [OPÇÕES]
```

`POST /v1/webhooks/{id}/test`

Aceita tanto o UUID interno quanto o `external_id` do recurso.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Aceito e enfileirado. |
| `404` | Recurso não encontrado ou filtrado por soft-delete. |
