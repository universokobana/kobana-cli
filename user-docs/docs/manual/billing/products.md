---
id: products
title: "kobana billing products"
sidebar_label: "products"
---

# kobana billing products


Comandos de `kobana billing products`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`products bulk delete`](#products-bulk-delete) | `DELETE /v1/products/bulk` | Excluir produtos em lote |
| [`products reorder create`](#products-reorder-create) | `POST /v1/products/reorder` | Reordenar produtos |
| [`products reorder update`](#products-reorder-update) | `PUT /v1/products/reorder` | Reordenar produtos (PUT /reorder) |
| [`products list`](#products-list) | `GET /v1/products` | Listar produtos |
| [`products create`](#products-create) | `POST /v1/products` | Criar produto |
| [`products update`](#products-update) | `PUT /v1/products` | Criar ou atualizar produto |
| [`products bulk-archive`](#products-bulk-archive) | `POST /v1/products/bulk-archive` | Arquivar produtos em lote |
| [`products bulk-change-company`](#products-bulk-change-company) | `POST /v1/products/bulk-change-company` | Trocar empresa de produtos em lote |
| [`products bulk-move-group`](#products-bulk-move-group) | `POST /v1/products/bulk-move-group` | Mover produtos para outro grupo |
| [`products bulk-visibility`](#products-bulk-visibility) | `POST /v1/products/bulk-visibility` | Alterar visibilidade em lote |
| [`products get`](#products-get) | `GET /v1/products/{id}` | Obter produto |
| [`products delete`](#products-delete) | `DELETE /v1/products/{id}` | Excluir produto |
| [`products archive`](#products-archive) | `POST /v1/products/{id}/archive` | Arquivar produto |
| [`products publish`](#products-publish) | `POST /v1/products/{id}/publish` | Publicar produto |
| [`products restore`](#products-restore) | `POST /v1/products/{id}/restore` | Restaurar produto arquivado |

## products bulk delete

Excluir produtos em lote

```text
kobana billing products bulk delete [OPÇÕES]
```

`DELETE /v1/products/bulk`

Exclui vários produtos (soft delete: `deleted_at`, mesmo efeito de `DELETE /v1/products/{id}` em cada um). IDs em uso por assinaturas ativas são ignorados e reportados na resposta. Exige o nível de acesso `delete`. Substitui `POST /v1/products/bulk-delete`, removido.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | IDs dos produtos a excluir. |

Outros campos e objetos aninhados: `kobana schema billing.products.bulk.delete`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da operação em lote. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products reorder create

Reordenar produtos

```text
kobana billing products reorder create [OPÇÕES]
```

`POST /v1/products/reorder`

Atualiza a ordem de exibição dos produtos. O índice 0 do array vira o primeiro item da lista.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `product_ids` | `array<string (uuid)>` | sim | IDs de produtos na nova ordem desejada. Índice 0 vira o primeiro da lista. |

Outros campos e objetos aninhados: `kobana schema billing.products.reorder.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Ordem atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products reorder update

Reordenar produtos (PUT /reorder)

```text
kobana billing products reorder update [OPÇÕES]
```

`PUT /v1/products/reorder`

Alias `PUT` para `/products/reorder`. Mesmo comportamento da versão `POST`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `product_ids` | `array<string (uuid)>` | sim | IDs de produtos na nova ordem desejada. Índice 0 vira o primeiro da lista. |

Outros campos e objetos aninhados: `kobana schema billing.products.reorder.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Ordem atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products list

Listar produtos

```text
kobana billing products list [OPÇÕES]
```

`GET /v1/products`

Retorna uma lista paginada dos produtos da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra por status do produto. Valores: `all`, `draft`, `active`, `archived`. |
| `visibility` | `string` | não | Filtra por visibilidade. Valores: `public`, `private`. |
| `product_type` | `string` | não | Filtra por tipo de produto. Valores: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `product_group_id` | `string` | não | Filtra por grupo de produto. |
| `company_id` | `string` | não | Filtra por empresa. |
| `parent_product_id` | `string` | não | Filtra por produto pai. |
| `root_only` | `string` | não | Quando `true`, retorna apenas produtos raiz (sem pai). Valores: `true`, `false`. |
| `search` | `string` | não | Busca textual livre (nome, slug). |
| `service_item_id` | `string` | não | Filtra por item de serviço vinculado. |
| `is_required` | `string` | não | Quando `true`, retorna apenas produtos marcados como obrigatórios. Valores: `true`, `false`. |
| `self_service_editable` | `string` | não | Quando `true`, retorna apenas produtos que o cliente pode editar no portal. Valores: `true`, `false`. |
| `external_id` | `string` | não | Filtra por identificador externo. |
| `slug` | `string` | não | Filtra por slug exato do produto. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `updated_after` | `string` | não | Data de atualização inicial do intervalo (YYYY-MM-DD). |
| `updated_before` | `string` | não | Data de atualização final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de produtos. |
| `404` | Recurso não encontrado na organização atual. |

## products create

Criar produto

```text
kobana billing products create [OPÇÕES]
```

`POST /v1/products`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do produto. |
| `slug` | `string` | sim | Slug único dentro do grupo. Aceita apenas `[a-z0-9_-]+`. |
| `description` | `string` | não |  |
| `product_type` | `string` | não | Tipo do produto. Padrão: `base`. Valores: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `visibility` | `string` | não | Visibilidade no portal. Padrão: `public`. Valores: `public`, `private`. |
| `unit_label` | `string` | não |  |
| `increment` | `integer` | não | Incremento mínimo. Padrão: `1`. |
| `website_url` | `string (uri)` | não |  |
| `is_required` | `boolean` | não |  |
| `parent_product_id` | `string (uuid)` | não |  |
| `product_group_id` | `string (uuid)` | não |  |
| `company_id` | `string (uuid)` | não |  |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema billing.products.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Produto criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products update

Criar ou atualizar produto

```text
kobana billing products update [OPÇÕES]
```

`PUT /v1/products`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do produto. |
| `slug` | `string` | não | Slug único dentro do grupo. Aceita apenas `[a-z0-9_-]+`. |
| `description` | `string` | não |  |
| `product_type` | `string` | não | Tipo do produto. Padrão: `base`. Valores: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `visibility` | `string` | não | Visibilidade no portal. Padrão: `public`. Valores: `public`, `private`. |
| `unit_label` | `string` | não |  |
| `increment` | `integer` | não | Incremento mínimo. Padrão: `1`. |
| `website_url` | `string (uri)` | não |  |
| `is_required` | `boolean` | não |  |
| `parent_product_id` | `string (uuid)` | não |  |
| `product_group_id` | `string (uuid)` | não |  |
| `company_id` | `string (uuid)` | não |  |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não |  |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.products.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products bulk-archive

Arquivar produtos em lote

```text
kobana billing products bulk-archive [OPÇÕES]
```

`POST /v1/products/bulk-archive`

Arquiva múltiplos produtos. IDs em uso por assinaturas ativas ou já arquivados são ignorados e reportados na resposta.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim | IDs de produtos a arquivar. |

Outros campos e objetos aninhados: `kobana schema billing.products.bulk-archive`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da operação em lote. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products bulk-change-company

Trocar empresa de produtos em lote

```text
kobana billing products bulk-change-company [OPÇÕES]
```

`POST /v1/products/bulk-change-company`

Reatribui múltiplos produtos a outra empresa. Envie `company_id: null` para limpar o vínculo.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim |  |
| `company_id` | `string (uuid)` | sim | Empresa de destino. Use `null` para limpar o vínculo. |

Outros campos e objetos aninhados: `kobana schema billing.products.bulk-change-company`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da operação em lote. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products bulk-move-group

Mover produtos para outro grupo

```text
kobana billing products bulk-move-group [OPÇÕES]
```

`POST /v1/products/bulk-move-group`

Move múltiplos produtos para um grupo de produto. Envie `product_group_id: null` para remover do grupo atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim |  |
| `product_group_id` | `string (uuid)` | sim | Grupo de destino. Use `null` para remover do grupo atual. |

Outros campos e objetos aninhados: `kobana schema billing.products.bulk-move-group`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da operação em lote. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products bulk-visibility

Alterar visibilidade em lote

```text
kobana billing products bulk-visibility [OPÇÕES]
```

`POST /v1/products/bulk-visibility`

Atualiza a visibilidade (`public` ou `private`) de múltiplos produtos.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sim |  |
| `visibility` | `string` | sim | Nova visibilidade a aplicar em todos os produtos. Valores: `public`, `private`. |

Outros campos e objetos aninhados: `kobana schema billing.products.bulk-visibility`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da operação em lote. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products get

Obter produto

```text
kobana billing products get [ID] [OPÇÕES]
```

`GET /v1/products/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do produto. |
| `404` | Recurso não encontrado na organização atual. |

## products delete

Excluir produto

```text
kobana billing products delete [ID] [OPÇÕES]
```

`DELETE /v1/products/{id}`

Exclui o produto (soft delete: `deleted_at`, ele some de toda leitura e sai de todos os planos; as assinaturas e faturas que o citam guardam a referência). Exige o nível de acesso `delete`. Retorna 422 quando o produto está em uso por assinaturas ativas. Até outubro de 2026 esta rota arquivava o produto — arquivar é `POST /v1/products/{id}/archive`, reversível.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Produto excluído. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products archive

Arquivar produto

```text
kobana billing products archive [ID] [OPÇÕES]
```

`POST /v1/products/{id}/archive`

Arquiva o produto. Retorna 422 quando o produto está em uso por assinaturas ativas ou já está arquivado.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Produto arquivado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products publish

Publicar produto

```text
kobana billing products publish [ID] [OPÇÕES]
```

`POST /v1/products/{id}/publish`

Promove um produto em rascunho para o status ativo.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Produto publicado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## products restore

Restaurar produto arquivado

```text
kobana billing products restore [ID] [OPÇÕES]
```

`POST /v1/products/{id}/restore`

Restaura um produto arquivado de volta para o status ativo.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Produto restaurado. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
