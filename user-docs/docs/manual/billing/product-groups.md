---
id: product-groups
title: "kobana billing product-groups"
sidebar_label: "product-groups"
---

# kobana billing product-groups


Comandos de `kobana billing product-groups`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`product-groups reorder create`](#product-groups-reorder-create) | `POST /v1/product_groups/reorder` | Reordenar grupos de produto |
| [`product-groups reorder update`](#product-groups-reorder-update) | `PUT /v1/product_groups/reorder` | Reordenar grupos de produto (alias PUT) |
| [`product-groups list`](#product-groups-list) | `GET /v1/product_groups` | Listar grupos de produto |
| [`product-groups create`](#product-groups-create) | `POST /v1/product_groups` | Criar grupo de produto |
| [`product-groups update`](#product-groups-update) | `PUT /v1/product_groups` | Criar ou atualizar grupo de produtos |
| [`product-groups get`](#product-groups-get) | `GET /v1/product_groups/{id}` | Obter grupo de produto |
| [`product-groups delete`](#product-groups-delete) | `DELETE /v1/product_groups/{id}` | Remover grupo de produto |
| [`product-groups archive`](#product-groups-archive) | `POST /v1/product_groups/{id}/archive` | Arquivar grupo de produto |
| [`product-groups unarchive`](#product-groups-unarchive) | `POST /v1/product_groups/{id}/unarchive` | Desarquivar grupo de produto |

## product-groups reorder create

Reordenar grupos de produto

```text
kobana billing product-groups reorder create [OPÇÕES]
```

`POST /v1/product_groups/reorder`

Atualiza a ordem de exibição dos grupos de produto. O array `group_ids` define a nova ordem — índice 0 vira o primeiro grupo.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | sim | Lista de UUIDs na nova ordem desejada. Todos os IDs devem pertencer à organização. |

Outros campos e objetos aninhados: `kobana schema billing.product-groups.reorder.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Ordem atualizada com sucesso. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## product-groups reorder update

Reordenar grupos de produto (alias PUT)

```text
kobana billing product-groups reorder update [OPÇÕES]
```

`PUT /v1/product_groups/reorder`

Alias `PUT` para `POST /product_groups/reorder`. Mesmo contrato.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | sim | Lista de UUIDs na nova ordem desejada. Todos os IDs devem pertencer à organização. |

Outros campos e objetos aninhados: `kobana schema billing.product-groups.reorder.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Ordem atualizada com sucesso. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## product-groups list

Listar grupos de produto

```text
kobana billing product-groups list [OPÇÕES]
```

`GET /v1/product_groups`

Retorna uma lista paginada dos grupos de produto da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca textual livre pelo nome do grupo. |
| `company_id` | `string` | não | Filtra por empresa emissora associada. |
| `include_archived` | `boolean,null` | não | Quando `true`, inclui grupos arquivados na listagem. Padrão: `false`. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `archived_after` | `string` | não | Retorna grupos arquivados a partir desta data (ISO 8601). |
| `archived_before` | `string` | não | Retorna grupos arquivados até esta data (ISO 8601). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de grupos de produto. |
| `404` | Recurso não encontrado na organização atual. |

## product-groups create

Criar grupo de produto

```text
kobana billing product-groups create [OPÇÕES]
```

`POST /v1/product_groups`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do grupo. |
| `slug` | `string` | não | Slug do grupo. Gerado a partir do nome quando omitido. |
| `visibility` | `string` | não | Visibilidade no portal do cliente. Padrão: `public`. Valores: `public`, `private`. |
| `company_id` | `string (uuid)` | não |  |
| `proposal_template_id` | `string (uuid)` | não |  |
| `service_item_id` | `string (uuid)` | não |  |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.product-groups.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Grupo de produto criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## product-groups update

Criar ou atualizar grupo de produtos

```text
kobana billing product-groups update [OPÇÕES]
```

`PUT /v1/product_groups`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do grupo. |
| `slug` | `string` | não | Slug do grupo. Gerado a partir do nome quando omitido. |
| `visibility` | `string` | não | Visibilidade no portal do cliente. Padrão: `public`. Valores: `public`, `private`. |
| `company_id` | `string (uuid)` | não |  |
| `proposal_template_id` | `string (uuid)` | não |  |
| `service_item_id` | `string (uuid)` | não |  |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.product-groups.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## product-groups get

Obter grupo de produto

```text
kobana billing product-groups get [ID] [OPÇÕES]
```

`GET /v1/product_groups/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do grupo de produto. |
| `404` | Recurso não encontrado na organização atual. |

## product-groups delete

Remover grupo de produto

```text
kobana billing product-groups delete [ID] [OPÇÕES]
```

`DELETE /v1/product_groups/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Confirmação de remoção. |
| `404` | Recurso não encontrado na organização atual. |

## product-groups archive

Arquivar grupo de produto

```text
kobana billing product-groups archive [ID] [OPÇÕES]
```

`POST /v1/product_groups/{id}/archive`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Grupo de produto arquivado. |
| `404` | Recurso não encontrado na organização atual. |

## product-groups unarchive

Desarquivar grupo de produto

```text
kobana billing product-groups unarchive [ID] [OPÇÕES]
```

`POST /v1/product_groups/{id}/unarchive`

Traz de volta ao catálogo um grupo de produto arquivado. Exige o nível de acesso `write`. Substitui `DELETE /v1/product_groups/{id}/archive`, removido: um `DELETE` é sempre uma exclusão.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Grupo de produto desarquivado. |
| `404` | Recurso não encontrado na organização atual. |
