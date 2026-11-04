---
id: usage
title: "kobana billing usage"
sidebar_label: "usage"
---

# kobana billing usage


Comandos de `kobana billing usage`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`usage credits`](#usage-credits) | `GET /v1/usage/credits` | Consultar consumo e saldo de créditos |
| [`usage credits publish`](#usage-credits-publish) | `POST /v1/usage/credits/publish` | Publicar consumo de créditos |
| [`usage snapshots daily list`](#usage-snapshots-daily-list) | `GET /v1/usage/snapshots/daily` | Listar snapshots diários de uso |
| [`usage snapshots daily create`](#usage-snapshots-daily-create) | `POST /v1/usage/snapshots/daily` | Criar snapshots diários de uso |
| [`usage snapshots monthly list`](#usage-snapshots-monthly-list) | `GET /v1/usage/snapshots/monthly` | Listar snapshots mensais de uso |
| [`usage snapshots monthly create`](#usage-snapshots-monthly-create) | `POST /v1/usage/snapshots/monthly` | Criar snapshots mensais de uso |
| [`usage summary`](#usage-summary) | `GET /v1/usage/summary` | Resumo de uso do período |
| [`usage list`](#usage-list) | `GET /v1/usage` | Listar registros de uso |
| [`usage create`](#usage-create) | `POST /v1/usage` | Reportar uso (metered) |

## usage credits

Consultar consumo e saldo de créditos

```text
kobana billing usage credits [OPÇÕES] [COMANDO]
```

`GET /v1/usage/credits`

Retorna o consumo de créditos pré-pagos no período (totais e série diária) e o saldo atual. Sem `billing_account_id`, soma todas as contas da organização. Sem datas, considera o mês atual (fuso de São Paulo).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string` | não | Filtra por conta de cobrança. |
| `product_id` | `string` | não | Produto de créditos. Padrão: o primeiro produto ativo. |
| `start_date` | `string` | não | Início do período (`YYYY-MM-DD`). |
| `end_date` | `string` | não | Fim do período, inclusivo (`YYYY-MM-DD`). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Consumo do período e saldo. |
| `404` | Recurso não encontrado na organização atual. |

## usage credits publish

Publicar consumo de créditos

```text
kobana billing usage credits publish [OPÇÕES]
```

`POST /v1/usage/credits/publish`

Publica o consumo de créditos pré-pagos (produto do tipo `credits`) por conta de cobrança × produto × dia, registro único **ou** em lote `{ records: [...] }` com até **100** itens. A chave (conta, produto, `usage_date`) é única: publicar o mesmo dia de novo **sobrescreve** o registro (`overwritten: true`), nunca soma. Lotes com falhas parciais retornam **207 Multi-Status**.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema billing.usage.credits.publish`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registros processados (`207` quando parte do lote falhou). |
| `207` | Registros processados (`207` quando parte do lote falhou). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## usage snapshots daily list

Listar snapshots diários de uso

```text
kobana billing usage snapshots daily list [OPÇÕES]
```

`GET /v1/usage/snapshots/daily`

Lista os snapshots diários de uso da organização. Filtros opcionais: `billing_account_id`, `product_id` e janela `start_date`/`end_date`. Suporta paginação por `limit`/`offset`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string` | não | Filtra snapshots por UUID da conta de cobrança. |
| `product_id` | `string` | não | Filtra snapshots por UUID do produto. |
| `start_date` | `string` | não | Data/hora inicial do filtro — formato ISO 8601. |
| `end_date` | `string` | não | Data/hora final do filtro — formato ISO 8601. |
| `limit` | `integer` | não | Itens por página. Padrão: `100`. Máximo: `1000`. |
| `offset` | `integer,null` | não | Deslocamento (offset) para paginação. Padrão: `0`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de snapshots diários com `meta` contendo `total`, `limit`, `offset`. |
| `404` | Recurso não encontrado na organização atual. |

## usage snapshots daily create

Criar snapshots diários de uso

```text
kobana billing usage snapshots daily create [OPÇÕES]
```

`POST /v1/usage/snapshots/daily`

Cria snapshots diários de uso, registro único **ou** em lote `{ records: [...] }` com até **100** itens. O conjunto natural (organização, conta de cobrança, produto, data) é único: se já existir, o snapshot é sobrescrito (`overwritten: true`). Resolva a conta por `billing_account_id` **ou** `billing_account_public_id` e o produto por `product_slug` **ou** `product_id` dentro do `product_group_slug` informado. Lotes com falhas parciais retornam **207 Multi-Status**.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema billing.usage.snapshots.daily.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do processamento (registro único ou lote). |
| `207` | Resultado do processamento (registro único ou lote). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## usage snapshots monthly list

Listar snapshots mensais de uso

```text
kobana billing usage snapshots monthly list [OPÇÕES]
```

`GET /v1/usage/snapshots/monthly`

Lista os snapshots mensais de uso da organização. Filtros opcionais: `billing_account_id`, `product_id` e janela `start_month`/`end_month`. Suporta paginação por `limit`/`offset`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string` | não | Filtra snapshots por UUID da conta de cobrança. |
| `product_id` | `string` | não | Filtra snapshots por UUID do produto. |
| `start_month` | `string` | não | Mês inicial do filtro em formato `YYYY-MM-DD` (primeiro dia do mês). |
| `end_month` | `string` | não | Mês final do filtro em formato `YYYY-MM-DD` (primeiro dia do mês). |
| `limit` | `integer` | não | Itens por página. Padrão: `100`. Máximo: `1000`. |
| `offset` | `integer,null` | não | Deslocamento (offset) para paginação. Padrão: `0`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de snapshots mensais com `meta` contendo `total`, `limit`, `offset`. |
| `404` | Recurso não encontrado na organização atual. |

## usage snapshots monthly create

Criar snapshots mensais de uso

```text
kobana billing usage snapshots monthly create [OPÇÕES]
```

`POST /v1/usage/snapshots/monthly`

Cria snapshots mensais de uso, registro único **ou** em lote `{ records: [...] }` com até **100** itens. O conjunto natural (organização, conta de cobrança, produto, mês) é único: se já existir, o snapshot é sobrescrito (`overwritten: true`). Resolva a conta por `billing_account_id` **ou** `billing_account_public_id` e o produto por `product_slug` **ou** `product_id` dentro do `product_group_slug`. Lotes com falhas parciais retornam **207 Multi-Status**.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema billing.usage.snapshots.monthly.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do processamento (registro único ou lote). |
| `207` | Resultado do processamento (registro único ou lote). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## usage summary

Resumo de uso do período

```text
kobana billing usage summary [OPÇÕES]
```

`GET /v1/usage/summary`

Retorna a agregação de uso de um item de assinatura no período de cobrança vigente (ou em um período arbitrário via `period_start`/`period_end`). Considera registros `set` (substituem) e somatórios de `increment` posteriores. Quando o preço é tiered ou unitário, calcula um custo estimado em `estimated_cost` (float, até 4 casas decimais).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `subscription_item_id` | `string` | não | UUID do item de assinatura. Alternativa a `product_slug` + `customer_id`. |
| `product_slug` | `string` | não | Slug do produto. Combine com `customer_id`. |
| `customer_id` | `string` | não | Identificador externo do cliente. Combine com `product_slug`. |
| `period_start` | `string` | não | Início do período (ISO 8601). Padrão: `current_period_start` da assinatura. |
| `period_end` | `string` | não | Fim do período (ISO 8601). Padrão: `current_period_end` da assinatura. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resumo de uso, período e custo estimado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## usage list

Listar registros de uso

```text
kobana billing usage list [OPÇÕES]
```

`GET /v1/usage`

Retorna os registros de uso (metered) de um item de assinatura. Identifique o item por `subscription_item_id` **ou** pela combinação `product_slug` + `customer_id`. Suporta filtros por intervalo de datas e janela com `limit`/`offset`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `subscription_item_id` | `string` | não | UUID do item de assinatura. Alternativa a `product_slug` + `customer_id`. |
| `product_slug` | `string` | não | Slug do produto. Combine com `customer_id`. |
| `customer_id` | `string` | não | Identificador externo do cliente. Combine com `product_slug`. |
| `start_date` | `string` | não | Data/hora inicial do filtro — formato ISO 8601. |
| `end_date` | `string` | não | Data/hora final do filtro — formato ISO 8601. |
| `limit` | `integer` | não | Itens por página. Padrão: `100`. Máximo: `1000`. |
| `offset` | `integer,null` | não | Deslocamento (offset) para paginação. Padrão: `0`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de registros de uso com agregação `total_quantity` no `meta`. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |

## usage create

Reportar uso (metered)

```text
kobana billing usage create [OPÇÕES]
```

`POST /v1/usage`

Reporta uso para faturamento metered. Aceita um registro único **ou** um lote `{ records: [...] }` com até **100** itens. Cada registro precisa identificar o item de assinatura por `subscription_item_id` **ou** por `product_slug` + `customer_id` (recomendado para integrações externas). O preço/produto deve estar configurado como `usage_type=metered` e a assinatura precisa estar `active` ou `trialing`. Use `idempotency_key` para evitar duplicidade — em caso de chave já vista, a resposta marca `duplicate: true`. Lotes com falhas parciais retornam **207 Multi-Status**.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema billing.usage.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do processamento (registro único ou lote). |
| `207` | Resultado do processamento (registro único ou lote). |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
