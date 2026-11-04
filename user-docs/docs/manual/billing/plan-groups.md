---
id: plan-groups
title: "kobana billing plan-groups"
sidebar_label: "plan-groups"
---

# kobana billing plan-groups


Comandos de `kobana billing plan-groups`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`plan-groups list`](#plan-groups-list) | `GET /v1/plan_groups` | Listar grupos de planos |
| [`plan-groups create`](#plan-groups-create) | `POST /v1/plan_groups` | Criar grupo de planos |
| [`plan-groups update`](#plan-groups-update) | `PUT /v1/plan_groups` | Criar ou atualizar grupo de planos |
| [`plan-groups reorder`](#plan-groups-reorder) | `POST /v1/plan_groups/reorder` | Reordenar grupos de planos |
| [`plan-groups get`](#plan-groups-get) | `GET /v1/plan_groups/{id}` | Obter grupo de planos |
| [`plan-groups delete`](#plan-groups-delete) | `DELETE /v1/plan_groups/{id}` | Remover grupo de planos |
| [`plan-groups archive`](#plan-groups-archive) | `POST /v1/plan_groups/{id}/archive` | Arquivar grupo de planos |

## plan-groups list

Listar grupos de planos

```text
kobana billing plan-groups list [OPÇÕES]
```

`GET /v1/plan_groups`

Retorna todos os grupos de planos da organização atual ordenados por `display_order`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | não | Busca textual livre pelo nome ou slug. |
| `include_archived` | `string` | não | Quando `true`, inclui grupos arquivados na lista. Valores: `true`, `false`. |
| `company_id` | `string` | não | Filtra grupos de planos pela empresa informada (UUID). |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `archived_after` | `string` | não | Retorna apenas grupos arquivados a partir desta data (YYYY-MM-DD). |
| `archived_before` | `string` | não | Retorna apenas grupos arquivados até esta data (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de grupos de planos. |
| `404` | Recurso não encontrado na organização atual. |

## plan-groups create

Criar grupo de planos

```text
kobana billing plan-groups create [OPÇÕES]
```

`POST /v1/plan_groups`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do grupo de planos. |
| `slug` | `string` | não | Identificador legível (opcional). Gerado automaticamente se omitido. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.plan-groups.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Grupo de planos criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plan-groups update

Criar ou atualizar grupo de planos

```text
kobana billing plan-groups update [OPÇÕES]
```

`PUT /v1/plan_groups`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome do grupo de planos. |
| `slug` | `string` | não | Identificador legível (opcional). Gerado automaticamente se omitido. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.plan-groups.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plan-groups reorder

Reordenar grupos de planos

```text
kobana billing plan-groups reorder [OPÇÕES]
```

`POST /v1/plan_groups/reorder`

Atualiza o `display_order` dos grupos conforme a ordem do array `group_ids`. Todos os grupos devem pertencer à organização atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | sim | Lista de UUIDs dos grupos na nova ordem de exibição. Todos devem pertencer à organização atual. |

Outros campos e objetos aninhados: `kobana schema billing.plan-groups.reorder`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Ordem atualizada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plan-groups get

Obter grupo de planos

```text
kobana billing plan-groups get [ID] [OPÇÕES]
```

`GET /v1/plan_groups/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do grupo de planos. |
| `404` | Recurso não encontrado na organização atual. |

## plan-groups delete

Remover grupo de planos

```text
kobana billing plan-groups delete [ID] [OPÇÕES]
```

`DELETE /v1/plan_groups/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Confirmação de remoção. |
| `404` | Recurso não encontrado na organização atual. |

## plan-groups archive

Arquivar grupo de planos

```text
kobana billing plan-groups archive [ID] [OPÇÕES]
```

`POST /v1/plan_groups/{id}/archive`

Marca o grupo como arquivado preenchendo `archived_at`. Grupos arquivados ficam ocultos das listagens padrão.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Grupo de planos arquivado. |
| `404` | Recurso não encontrado na organização atual. |
