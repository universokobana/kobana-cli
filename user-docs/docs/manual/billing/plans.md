---
id: plans
title: "kobana billing plans"
sidebar_label: "plans"
---

# kobana billing plans


Comandos de `kobana billing plans`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`plans bulk delete`](#plans-bulk-delete) | `DELETE /v1/plans/bulk` | Excluir planos em lote |
| [`plans items`](#plans-items) | `GET /v1/plans/{id}/items` | Listar itens do plano |
| [`plans items update`](#plans-items-update) | `PUT /v1/plans/{id}/items` | Substituir itens do plano |
| [`plans list`](#plans-list) | `GET /v1/plans` | Listar planos |
| [`plans create`](#plans-create) | `POST /v1/plans` | Criar plano |
| [`plans update`](#plans-update) | `PUT /v1/plans` | Criar ou atualizar plano |
| [`plans bulk-archive`](#plans-bulk-archive) | `POST /v1/plans/bulk-archive` | Arquivar planos em lote |
| [`plans bulk-move-group`](#plans-bulk-move-group) | `POST /v1/plans/bulk-move-group` | Mover planos para outro grupo em lote |
| [`plans get`](#plans-get) | `GET /v1/plans/{id}` | Obter plano |
| [`plans delete`](#plans-delete) | `DELETE /v1/plans/{id}` | Excluir plano |
| [`plans archive`](#plans-archive) | `POST /v1/plans/{id}/archive` | Arquivar plano |
| [`plans duplicate`](#plans-duplicate) | `POST /v1/plans/{id}/duplicate` | Duplicar plano |

## plans bulk delete

Excluir planos em lote

```text
kobana billing plans bulk delete [OPÇÕES]
```

`DELETE /v1/plans/bulk`

Exclui os planos informados (soft delete: `deleted_at`, mesmo efeito de `DELETE /v1/plans/{id}` em cada um). Planos com assinatura que não esteja cancelada são pulados e retornados em `skipped_ids`. Exige o nível de acesso `delete`. Substitui `POST /v1/plans/bulk-delete`, removido.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | Lista de UUIDs de planos. Mínimo 1. |

Outros campos e objetos aninhados: `kobana schema billing.plans.bulk.delete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resumo da exclusão em lote. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plans items

Listar itens do plano

```text
kobana billing plans items [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/plans/{id}/items`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de itens do plano. |
| `404` | Recurso não encontrado na organização atual. |

## plans items update

Substituir itens do plano

```text
kobana billing plans items update [ID] [OPÇÕES]
```

`PUT /v1/plans/{id}/items`

Substitui de forma atômica todos os itens do plano. Apenas produtos raiz são aceitos e cada `product_id` deve ser único na lista.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `items` | `array<object>` | sim |  |

Outros campos e objetos aninhados: `kobana schema billing.plans.items.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Itens do plano substituídos. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plans list

Listar planos

```text
kobana billing plans list [OPÇÕES]
```

`GET /v1/plans`

Retorna uma lista paginada dos planos da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `person_id` | `string` | não | Filtra por UUID da pessoa associada. |
| `person_document_number` | `string` | não | Filtra por CPF/CNPJ da pessoa (exato). |
| `person_name_contains` | `string` | não | Filtra por nome da pessoa (busca parcial, sem distinção de maiúsculas). |
| `status` | `string` | não | Filtra por status do plano. Valores: `draft`, `active`, `archived`. |
| `plan_type` | `string` | não | Filtra por tipo do plano. Valores: `standard`, `custom`, `promotional`. |
| `visibility` | `string` | não | Filtra por visibilidade. Valores: `public`, `private`. |
| `plan_group_id` | `string` | não | Filtra por grupo de planos. |
| `company_id` | `string` | não | Filtra por empresa emissora. |
| `billing_account_id` | `string` | não | Filtra planos custom associados a uma conta de cobrança. |
| `scoped_billing_account_id` | `string` | não | Retorna planos disponíveis para a conta de cobrança informada (padrão + custom da conta). |
| `exclude_custom_plans` | `string` | não | Quando `true`, exclui planos do tipo `custom` dos resultados. Valores: `true`, `false`. |
| `default_billing_cycle` | `string` | não | Filtra por periodicidade de cobrança padrão do plano. Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `external_id` | `string` | não | Filtra pelo identificador externo do plano. |
| `slug` | `string` | não | Filtra pelo slug do plano. |
| `search` | `string` | não | Busca textual livre. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `updated_after` | `string` | não | Data de atualização inicial do intervalo (YYYY-MM-DD). |
| `updated_before` | `string` | não | Data de atualização final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de planos. |
| `404` | Recurso não encontrado na organização atual. |

## plans create

Criar plano

```text
kobana billing plans create [OPÇÕES]
```

`POST /v1/plans`

Cria um plano para a organização autenticada.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim |  |
| `slug` | `string` | sim | Apenas letras minúsculas, dígitos e hífens. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não |  |
| `plan_group_id` | `string (uuid)` | não |  |
| `description` | `string` | não |  |
| `status` | `string` | não | Valores: `draft`, `active`, `archived`. |
| `plan_type` | `string` | não | Padrão: `standard`. Valores: `standard`, `custom`, `promotional`. |
| `billing_account_id` | `string (uuid)` | não |  |
| `visibility` | `string` | não | Padrão: `public`. Valores: `public`, `private`. |
| `features` | `array<string>` | não |  |
| `limits` | `object` | não |  |
| `trial_period_days` | `integer` | não |  |
| `default_billing_cycle` | `string` | não | Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `available_billing_cycles` | `array<string>` | não |  |
| `items` | `array<object>` | não |  |
| `custom_metadata` | `object` | não | Metadados customizados em formato livre (`object`). |

Outros campos e objetos aninhados: `kobana schema billing.plans.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Plano criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plans update

Criar ou atualizar plano

```text
kobana billing plans update [OPÇÕES]
```

`PUT /v1/plans`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não |  |
| `plan_group_id` | `string (uuid)` | não |  |
| `name` | `string` | não |  |
| `slug` | `string` | não | Apenas letras minúsculas, dígitos e hífens. |
| `description` | `string` | não |  |
| `status` | `string` | não | Valores: `draft`, `active`, `archived`. |
| `plan_type` | `string` | não | Padrão: `standard`. Valores: `standard`, `custom`, `promotional`. |
| `billing_account_id` | `string (uuid)` | não |  |
| `visibility` | `string` | não | Padrão: `public`. Valores: `public`, `private`. |
| `features` | `array<string>` | não |  |
| `limits` | `object` | não |  |
| `trial_period_days` | `integer` | não |  |
| `default_billing_cycle` | `string` | não | Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `available_billing_cycles` | `array<string>` | não |  |
| `items` | `array<object>` | não |  |
| `custom_metadata` | `object` | não | Metadados customizados em formato livre (`object`). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.plans.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plans bulk-archive

Arquivar planos em lote

```text
kobana billing plans bulk-archive [OPÇÕES]
```

`POST /v1/plans/bulk-archive`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | Lista de UUIDs de planos. Mínimo 1. |

Outros campos e objetos aninhados: `kobana schema billing.plans.bulk-archive`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resumo do arquivamento em lote. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plans bulk-move-group

Mover planos para outro grupo em lote

```text
kobana billing plans bulk-move-group [OPÇÕES]
```

`POST /v1/plans/bulk-move-group`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim |  |
| `plan_group_id` | `string (uuid)` | sim | UUID do grupo destino. Use `null` para remover o agrupamento. |

Outros campos e objetos aninhados: `kobana schema billing.plans.bulk-move-group`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resumo da movimentação em lote. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plans get

Obter plano

```text
kobana billing plans get [ID] [OPÇÕES]
```

`GET /v1/plans/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do plano. |
| `404` | Recurso não encontrado na organização atual. |

## plans delete

Excluir plano

```text
kobana billing plans delete [ID] [OPÇÕES]
```

`DELETE /v1/plans/{id}`

Exclui o plano (soft delete: `deleted_at`, ele some de toda leitura; as regras de mudança de plano que o citam são removidas). Exige o nível de acesso `delete`. Retorna 422 enquanto houver assinatura no plano que não esteja cancelada. Até outubro de 2026 esta rota arquivava o plano, ou excluía um rascunho com `?hard=true` (o parâmetro deixou de existir) — arquivar é `POST /v1/plans/{id}/archive`, reversível.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Plano excluído. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plans archive

Arquivar plano

```text
kobana billing plans archive [ID] [OPÇÕES]
```

`POST /v1/plans/{id}/archive`

Arquiva o plano: ele continua listado e pode ser restaurado, mas não aceita novas assinaturas; as existentes seguem cobrando. Retorna 422 enquanto houver assinaturas ativas no plano. Exige o nível de acesso `write`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Plano arquivado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plans duplicate

Duplicar plano

```text
kobana billing plans duplicate [ID] [OPÇÕES]
```

`POST /v1/plans/{id}/duplicate`

Cria uma cópia do plano (incluindo seus itens) no estado de rascunho.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Plano duplicado. |
| `404` | Recurso não encontrado na organização atual. |
