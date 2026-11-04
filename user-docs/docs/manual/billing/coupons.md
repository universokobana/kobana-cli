---
id: coupons
title: "kobana billing coupons"
sidebar_label: "coupons"
---

# kobana billing coupons


Comandos de `kobana billing coupons`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`coupons list`](#coupons-list) | `GET /v1/coupons` | Listar cupons |
| [`coupons create`](#coupons-create) | `POST /v1/coupons` | Criar cupom |
| [`coupons update`](#coupons-update) | `PUT /v1/coupons` | Criar ou atualizar cupom |
| [`coupons get`](#coupons-get) | `GET /v1/coupons/{id}` | Obter cupom |
| [`coupons delete`](#coupons-delete) | `DELETE /v1/coupons/{id}` | Excluir cupom |
| [`coupons deactivate`](#coupons-deactivate) | `POST /v1/coupons/{id}/deactivate` | Desativar cupom |
| [`coupons redemptions`](#coupons-redemptions) | `GET /v1/coupons/{id}/redemptions` | Listar resgates do cupom |

## coupons list

Listar cupons

```text
kobana billing coupons list [OPÇÕES]
```

`GET /v1/coupons`

Retorna uma lista paginada dos cupons da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `is_active` | `string` | não | Filtra por status: `true` retorna apenas cupons ativos; `false`, apenas inativos. Valores: `true`, `false`. |
| `search` | `string` | não | Busca textual livre em `code` e `name`. |
| `discount_type` | `string` | não | Filtra pelo tipo de desconto: `percentage` ou `fixed_amount`. Valores: `percentage`, `fixed_amount`. |
| `code` | `string` | não | Filtra pelo código exato do cupom. |
| `min_purchase_min` | `number,null` | não | Valor mínimo de `min_purchase` (em reais) para filtrar cupons. |
| `min_purchase_max` | `number,null` | não | Valor máximo de `min_purchase` (em reais) para filtrar cupons. |
| `max_discount_min` | `number,null` | não | Valor mínimo de `max_discount` (em reais) para filtrar cupons. |
| `max_discount_max` | `number,null` | não | Valor máximo de `max_discount` (em reais) para filtrar cupons. |
| `valid_from_from` | `string` | não | Retorna cupons cuja data de início de validade é igual ou posterior a esta data (AAAA-MM-DD). |
| `valid_from_to` | `string` | não | Retorna cupons cuja data de início de validade é igual ou anterior a esta data (AAAA-MM-DD). |
| `valid_until_from` | `string` | não | Retorna cupons cuja data de expiração é igual ou posterior a esta data (AAAA-MM-DD). |
| `valid_until_to` | `string` | não | Retorna cupons cuja data de expiração é igual ou anterior a esta data (AAAA-MM-DD). |
| `created_after` | `string` | não | Retorna cupons criados a partir desta data (AAAA-MM-DD). |
| `created_before` | `string` | não | Retorna cupons criados até esta data (AAAA-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de cupons. |
| `404` | Recurso não encontrado na organização atual. |

## coupons create

Criar cupom

```text
kobana billing coupons create [OPÇÕES]
```

`POST /v1/coupons`

Cria um cupom de desconto. O `code` deve ser único por organização. Use `discount_type=percentage` com `discount_value` entre `0` e `100`, ou `discount_type=fixed_amount` com `discount_value` em reais (float, até 4 casas decimais).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `code` | `string` | sim | Código público do cupom — único por organização. |
| `name` | `string` | sim | Nome interno do cupom. |
| `discount_type` | `string` | sim | Tipo de desconto: `percentage` ou `fixed_amount`. Valores: `percentage`, `fixed_amount`. |
| `discount_value` | `number` | sim | Valor do desconto. Para `percentage`, inteiro entre `0` e `100`. Para `fixed_amount`, valor em reais (float, até 4 casas decimais). |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `description` | `string` | não | Descrição opcional. |
| `currency` | `string` | não | Moeda ISO 4217. Padrão: `BRL`. |
| `min_purchase` | `number` | não | Valor mínimo da fatura, em reais (float, até 4 casas decimais), para o cupom poder ser aplicado. |
| `max_discount` | `number` | não | Teto absoluto do desconto, em reais (float, até 4 casas decimais). |
| `usage_limit` | `integer` | não | Total máximo de resgates na organização. Omita para ilimitado. |
| `usage_limit_per_user` | `integer` | não | Máximo de resgates por conta de cobrança. Omita para ilimitado. |
| `valid_from` | `string` | não | Início do período de validade (ISO 8601 — data ou data/hora). |
| `valid_until` | `string` | não | Fim do período de validade (ISO 8601 — data ou data/hora). |
| `applies_to` | `object` | não |  |
| `excludes` | `object` | não |  |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.coupons.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Cupom criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## coupons update

Criar ou atualizar cupom

```text
kobana billing coupons update [OPÇÕES]
```

`PUT /v1/coupons`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `code` | `string` | não | Código público do cupom — único por organização. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `name` | `string` | não | Nome interno do cupom. |
| `description` | `string` | não | Descrição opcional. |
| `discount_type` | `string` | não | Tipo de desconto: `percentage` ou `fixed_amount`. Valores: `percentage`, `fixed_amount`. |
| `discount_value` | `number` | não | Valor do desconto. Para `percentage`, inteiro entre `0` e `100`. Para `fixed_amount`, valor em reais (float, até 4 casas decimais). |
| `currency` | `string` | não | Moeda ISO 4217. Padrão: `BRL`. |
| `min_purchase` | `number` | não | Valor mínimo da fatura, em reais (float, até 4 casas decimais), para o cupom poder ser aplicado. |
| `max_discount` | `number` | não | Teto absoluto do desconto, em reais (float, até 4 casas decimais). |
| `usage_limit` | `integer` | não | Total máximo de resgates na organização. Omita para ilimitado. |
| `usage_limit_per_user` | `integer` | não | Máximo de resgates por conta de cobrança. Omita para ilimitado. |
| `valid_from` | `string` | não | Início do período de validade (ISO 8601 — data ou data/hora). |
| `valid_until` | `string` | não | Fim do período de validade (ISO 8601 — data ou data/hora). |
| `applies_to` | `object` | não |  |
| `excludes` | `object` | não |  |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.coupons.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## coupons get

Obter cupom

```text
kobana billing coupons get [ID] [OPÇÕES]
```

`GET /v1/coupons/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do cupom. |
| `404` | Recurso não encontrado na organização atual. |

## coupons delete

Excluir cupom

```text
kobana billing coupons delete [ID] [OPÇÕES]
```

`DELETE /v1/coupons/{id}`

Exclui o cupom permanentemente. Só é permitido enquanto o cupom **não tiver resgates registrados** — caso contrário, use `POST /coupons/{id}/deactivate` para preservar o histórico.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cupom excluído. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## coupons deactivate

Desativar cupom

```text
kobana billing coupons deactivate [ID] [OPÇÕES]
```

`POST /v1/coupons/{id}/deactivate`

Desativa o cupom: novos resgates passam a ser bloqueados, mas os resgates anteriores são preservados.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cupom desativado. |
| `404` | Recurso não encontrado na organização atual. |

## coupons redemptions

Listar resgates do cupom

```text
kobana billing coupons redemptions [ID] [OPÇÕES]
```

`GET /v1/coupons/{id}/redemptions`

Retorna uma lista paginada dos resgates registrados para o cupom.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de resgates do cupom. |
| `404` | Recurso não encontrado na organização atual. |
