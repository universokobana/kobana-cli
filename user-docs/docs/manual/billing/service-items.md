---
id: service-items
title: "kobana billing service-items"
sidebar_label: "service-items"
---

# kobana billing service-items


Comandos de `kobana billing service-items`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`service-items list`](#service-items-list) | `GET /v1/service_items` | Listar itens de serviço |
| [`service-items create`](#service-items-create) | `POST /v1/service_items` | Criar item de serviço |
| [`service-items update`](#service-items-update) | `PUT /v1/service_items` | Criar ou atualizar item de serviço |
| [`service-items get`](#service-items-get) | `GET /v1/service_items/{id}` | Obter item de serviço |
| [`service-items delete`](#service-items-delete) | `DELETE /v1/service_items/{id}` | Excluir item de serviço |

## service-items list

Listar itens de serviço

```text
kobana billing service-items list [OPÇÕES]
```

`GET /v1/service_items`

Retorna uma lista paginada dos itens de serviço (catálogo LC 116/2003) da organização atual. Use os filtros para restringir por empresa, código LC 116, NBS, classificação tributária, indicador de operação ou anexo do Simples.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `company_id` | `string` | não | Filtra por UUID da empresa emissora associada ao item. |
| `search` | `string` | não | Busca textual livre em `name`, `internal_code` e `description`. |
| `is_active` | `string` | não | Filtra por status: `true` retorna apenas itens ativos; `false`, apenas inativos. Valores: `true`, `false`. |
| `lc116_code` | `string` | não | Filtra pelo código LC 116/2003 (ex.: `01.05`). |
| `nbs_code` | `string` | não | Filtra pelo código NBS (Nomenclatura Brasileira de Serviços). |
| `c_class_trib` | `string` | não | Filtra pelo CST IBS/CBS (`c_class_trib`). |
| `indop_code` | `string` | não | Filtra pelo Indicador de Operação (`indop_code`) usado na NFS-e SNNFSE. |
| `simples_annex` | `string` | não | Filtra pelo anexo do Simples Nacional aplicável: `annex_iii`, `annex_iv` ou `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `cnae_code` | `string` | não | Filtra pelo código CNAE relacionado ao serviço. |
| `municipal_code` | `string` | não | Filtra pelo código municipal do serviço. |
| `internal_code` | `string` | não | Filtra pelo código interno do item. |
| `cbs_ibs_category` | `string` | não | Filtra pela categoria CBS/IBS do item conforme a reforma tributária. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de itens de serviço. |
| `404` | Recurso não encontrado na organização atual. |

## service-items create

Criar item de serviço

```text
kobana billing service-items create [OPÇÕES]
```

`POST /v1/service_items`

Cria um item de serviço no catálogo LC 116/2003. O `internal_code` é o identificador interno do item e o `lc116_code` deve seguir o catálogo da Lei Complementar 116/2003 (ex.: `01.05`). Quando `company_id` for informado, a empresa precisa pertencer à organização autenticada.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do item de serviço. |
| `internal_code` | `string` | sim | Código interno do item — imutável depois da criação. |
| `lc116_code` | `string` | sim | Código LC 116/2003 do serviço (ex.: `01.05`). |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não | UUID da empresa emissora associada. Omita ou envie `null` para um item aplicável a todas as empresas. |
| `description` | `string` | não | Descrição opcional. |
| `municipal_code` | `string` | não | Código municipal do serviço, quando aplicável. |
| `nbs_code` | `string` | não | Código NBS. |
| `cnae_code` | `string` | não | Código CNAE relacionado. |
| `simples_annex` | `string` | não | Anexo do Simples Nacional aplicável: `annex_iii`, `annex_iv` ou `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `default_iss_rate` | `number` | não | Alíquota padrão de ISS em pontos percentuais (ex.: `2.5`). |
| `c_class_trib` | `string` | não | Classificação tributária CST IBS/CBS. |
| `cbs_ibs_category` | `string` | não | Categoria CBS/IBS do item. |
| `indop_code` | `string` | não | Indicador de Operação (`indOp`) da NFS-e SNNFSE. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.service-items.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Item de serviço criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## service-items update

Criar ou atualizar item de serviço

```text
kobana billing service-items update [OPÇÕES]
```

`PUT /v1/service_items`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não | UUID da empresa emissora associada. Omita ou envie `null` para um item aplicável a todas as empresas. |
| `name` | `string` | não | Nome do item de serviço. |
| `description` | `string` | não | Descrição opcional. |
| `internal_code` | `string` | não | Código interno do item — imutável depois da criação. |
| `lc116_code` | `string` | não | Código LC 116/2003 do serviço (ex.: `01.05`). |
| `municipal_code` | `string` | não | Código municipal do serviço, quando aplicável. |
| `nbs_code` | `string` | não | Código NBS. |
| `cnae_code` | `string` | não | Código CNAE relacionado. |
| `simples_annex` | `string` | não | Anexo do Simples Nacional aplicável: `annex_iii`, `annex_iv` ou `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `default_iss_rate` | `number` | não | Alíquota padrão de ISS em pontos percentuais (ex.: `2.5`). |
| `c_class_trib` | `string` | não | Classificação tributária CST IBS/CBS. |
| `cbs_ibs_category` | `string` | não | Categoria CBS/IBS do item. |
| `indop_code` | `string` | não | Indicador de Operação (`indOp`) da NFS-e SNNFSE. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.service-items.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## service-items get

Obter item de serviço

```text
kobana billing service-items get [ID] [OPÇÕES]
```

`GET /v1/service_items/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do item de serviço. |
| `404` | Recurso não encontrado na organização atual. |

## service-items delete

Excluir item de serviço

```text
kobana billing service-items delete [ID] [OPÇÕES]
```

`DELETE /v1/service_items/{id}`

Exclui o item de serviço (soft delete: `deleted_at`, ele some de toda leitura; regras tributárias, produtos e NFS-e já emitidas que o citam guardam a referência). Exige o nível de acesso `delete`. Para só deixar de usá-lo, desative-o (`PATCH` com `is_active=false`). Até outubro de 2026 esta rota excluía o item definitivamente, ou só o desativava quando uma regra tributária o usava.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Item de serviço excluído. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
