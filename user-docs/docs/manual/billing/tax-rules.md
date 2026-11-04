---
id: tax-rules
title: "kobana billing tax-rules"
sidebar_label: "tax-rules"
---

# kobana billing tax-rules


Comandos de `kobana billing tax-rules`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`tax-rules list`](#tax-rules-list) | `GET /v1/tax_rules` | Listar regras tributárias |
| [`tax-rules create`](#tax-rules-create) | `POST /v1/tax_rules` | Criar regra tributária |
| [`tax-rules update`](#tax-rules-update) | `PUT /v1/tax_rules` | Criar ou atualizar regra fiscal |
| [`tax-rules get`](#tax-rules-get) | `GET /v1/tax_rules/{id}` | Obter regra tributária |
| [`tax-rules delete`](#tax-rules-delete) | `DELETE /v1/tax_rules/{id}` | Excluir regra tributária |

## tax-rules list

Listar regras tributárias

```text
kobana billing tax-rules list [OPÇÕES]
```

`GET /v1/tax_rules`

Retorna uma lista paginada das regras tributárias da organização atual. Use os filtros de `scope`, `company_id`, `service_item_id`, `billing_account_id` e `is_active` para restringir o resultado.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `scope` | `string` | não | Filtra pelo escopo: `global`, `organization`, `company`, `service`, `customer` ou `municipality`. Valores: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `company_id` | `string` | não | Filtra pelas regras de uma empresa específica. |
| `service_item_id` | `string` | não | Filtra pelas regras de um item de serviço específico. |
| `billing_account_id` | `string` | não | Filtra pelas regras de uma conta de cobrança específica. |
| `is_active` | `string` | não | Quando `true`, retorna apenas regras ativas; quando `false`, apenas inativas. Valores: `true`, `false`. |
| `search` | `string` | não | Busca textual em `name` e `description`. |
| `customer_tax_type` | `string` | não | Filtra pelo tipo tributário do cliente: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` ou `entidade_publica`. |
| `operation_type` | `string` | não | Filtra pelo tipo de operação: `interno` (mesmo município) ou `externo` (município diferente). |
| `municipality` | `string` | não | Filtra pelo código IBGE do município (7 dígitos). |
| `priority_from` | `integer,null` | não | Prioridade mínima (inclusive) para filtrar regras. |
| `priority_to` | `integer,null` | não | Prioridade máxima (inclusive) para filtrar regras. |
| `effective_from_from` | `string` | não | Retorna regras cuja data de início de vigência seja igual ou posterior a este valor (ISO 8601). |
| `effective_from_to` | `string` | não | Retorna regras cuja data de início de vigência seja igual ou anterior a este valor (ISO 8601). |
| `effective_to_from` | `string` | não | Retorna regras cuja data de fim de vigência seja igual ou posterior a este valor (ISO 8601). |
| `effective_to_to` | `string` | não | Retorna regras cuja data de fim de vigência seja igual ou anterior a este valor (ISO 8601). |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de regras tributárias. |
| `404` | Recurso não encontrado na organização atual. |

## tax-rules create

Criar regra tributária

```text
kobana billing tax-rules create [OPÇÕES]
```

`POST /v1/tax_rules`

Cria uma nova regra tributária com seus itens (alíquotas por tributo). O `scope` define o nível de aplicação e a `priority` define a precedência — regras de maior prioridade sobrescrevem as menores. É obrigatório informar ao menos um item em `items`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome legível da regra. |
| `scope` | `string` | sim | Escopo da regra: `global`, `organization`, `company`, `service`, `customer` ou `municipality`. Valores: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `items` | `array<object>` | sim | Lista de itens (alíquotas por tributo). Obrigatório ao menos 1 item. |
| `description` | `string` | não | Descrição opcional da regra. |
| `priority` | `integer` | não | Prioridade da regra. Maior valor sobrescreve regras de menor prioridade. Padrão: `0`. |
| `company_id` | `string (uuid)` | não | UUID da empresa quando `scope = company`. A empresa precisa pertencer à organização atual. |
| `service_item_id` | `string (uuid)` | não | UUID do item de serviço quando `scope = service`. O item precisa pertencer à organização atual. |
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança quando `scope = customer`. A conta precisa pertencer à organização atual. |
| `customer_tax_type` | `string` | não | Tipo tributário do cliente: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` ou `entidade_publica`. Valores: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante`, `entidade_publica`. |
| `municipality` | `string` | não | Código IBGE do município (7 dígitos) quando `scope = municipality`. |
| `operation_type` | `string` | não | Tipo de operação: `interno` (mesmo município) ou `externo` (município diferente). Valores: `interno`, `externo`. |
| `effective_from` | `string (date-time)` | não | Data/hora ISO 8601 em que a regra passa a valer. |
| `effective_to` | `string (date-time)` | não | Data/hora ISO 8601 em que a regra deixa de valer. `null` significa vigência indeterminada. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.tax-rules.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Regra tributária criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## tax-rules update

Criar ou atualizar regra fiscal

```text
kobana billing tax-rules update [OPÇÕES]
```

`PUT /v1/tax_rules`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome legível da regra. |
| `description` | `string` | não | Descrição opcional da regra. |
| `scope` | `string` | não | Escopo da regra: `global`, `organization`, `company`, `service`, `customer` ou `municipality`. Valores: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `priority` | `integer` | não | Prioridade da regra. Maior valor sobrescreve regras de menor prioridade. Padrão: `0`. |
| `company_id` | `string (uuid)` | não | UUID da empresa quando `scope = company`. A empresa precisa pertencer à organização atual. |
| `service_item_id` | `string (uuid)` | não | UUID do item de serviço quando `scope = service`. O item precisa pertencer à organização atual. |
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança quando `scope = customer`. A conta precisa pertencer à organização atual. |
| `customer_tax_type` | `string` | não | Tipo tributário do cliente: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` ou `entidade_publica`. Valores: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante`, `entidade_publica`. |
| `municipality` | `string` | não | Código IBGE do município (7 dígitos) quando `scope = municipality`. |
| `operation_type` | `string` | não | Tipo de operação: `interno` (mesmo município) ou `externo` (município diferente). Valores: `interno`, `externo`. |
| `effective_from` | `string (date-time)` | não | Data/hora ISO 8601 em que a regra passa a valer. |
| `effective_to` | `string (date-time)` | não | Data/hora ISO 8601 em que a regra deixa de valer. `null` significa vigência indeterminada. |
| `items` | `array<object>` | não | Lista de itens (alíquotas por tributo). Obrigatório ao menos 1 item. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.tax-rules.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## tax-rules get

Obter regra tributária

```text
kobana billing tax-rules get [ID] [OPÇÕES]
```

`GET /v1/tax_rules/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da regra tributária. |
| `404` | Recurso não encontrado na organização atual. |

## tax-rules delete

Excluir regra tributária

```text
kobana billing tax-rules delete [ID] [OPÇÕES]
```

`DELETE /v1/tax_rules/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Regra tributária removida. |
| `404` | Recurso não encontrado na organização atual. |
