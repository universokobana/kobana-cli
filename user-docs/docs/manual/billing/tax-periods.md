---
id: tax-periods
title: "kobana billing tax-periods"
sidebar_label: "tax-periods"
---

# kobana billing tax-periods


Comandos de `kobana billing tax-periods`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`tax-periods list`](#tax-periods-list) | `GET /v1/tax_periods` | Listar períodos tributários |
| [`tax-periods create`](#tax-periods-create) | `POST /v1/tax_periods` | Processar período tributário mensal |
| [`tax-periods get`](#tax-periods-get) | `GET /v1/tax_periods/{id}` | Obter período tributário |
| [`tax-periods update`](#tax-periods-update) | `PATCH /v1/tax_periods/{id}` | Executar ação no período tributário |
| [`tax-periods delete`](#tax-periods-delete) | `DELETE /v1/tax_periods/{id}` | Excluir período tributário |

## tax-periods list

Listar períodos tributários

```text
kobana billing tax-periods list [OPÇÕES]
```

`GET /v1/tax_periods`

Retorna uma lista paginada dos períodos tributários (mensais ou trimestrais) da organização. Permite filtrar por empresa, tipo de período, ano e status.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `company_id` | `string` | não | Filtra por empresa emissora. |
| `period_type` | `string` | não | Filtra pelo tipo de período: `monthly` ou `quarterly`. Valores: `monthly`, `quarterly`. |
| `year` | `string` | não | Filtra pelo ano de competência (ex.: `2026`). |
| `status` | `string` | não | Filtra pelo status: `open`, `calculated` ou `closed`. Valores: `open`, `calculated`, `closed`. |
| `month` | `integer` | não | Filtra pelo mês de competência (1 a 12). |
| `quarter` | `integer` | não | Filtra pelo trimestre de competência (1 a 4). |
| `gross_revenue_min` | `number,null` | não | Receita bruta mínima do período, em reais (ex.: `1000.00`). |
| `gross_revenue_max` | `number,null` | não | Receita bruta máxima do período, em reais (ex.: `50000.00`). |
| `das_estimated_min` | `number,null` | não | DAS estimado mínimo do período, em reais (ex.: `100.00`). |
| `das_estimated_max` | `number,null` | não | DAS estimado máximo do período, em reais (ex.: `5000.00`). |
| `closed_after` | `string` | não | Retorna períodos encerrados a partir desta data (formato `YYYY-MM-DD`). |
| `closed_before` | `string` | não | Retorna períodos encerrados até esta data (formato `YYYY-MM-DD`). |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de períodos tributários. |
| `404` | Recurso não encontrado na organização atual. |

## tax-periods create

Processar período tributário mensal

```text
kobana billing tax-periods create [OPÇÕES]
```

`POST /v1/tax_periods`

Cria (ou recupera) o período tributário mensal informado e calcula os impostos devidos (ISS, PIS, COFINS, IRPJ, CSLL, CBS, IBS) com base nas NFes emitidas e nas retenções a compensar. Quando `company_id` é omitido, usa a empresa padrão da organização.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `year` | `integer` | sim | Ano de competência (ex.: `2026`). |
| `month` | `integer` | sim | Mês de competência (1 a 12). |
| `company_id` | `string (uuid)` | não | UUID da empresa emissora. Omita para usar a empresa padrão da organização. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.tax-periods.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Período tributário processado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## tax-periods get

Obter período tributário

```text
kobana billing tax-periods get [ID] [OPÇÕES]
```

`GET /v1/tax_periods/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do período tributário. |
| `404` | Recurso não encontrado na organização atual. |

## tax-periods update

Executar ação no período tributário

```text
kobana billing tax-periods update [ID] [OPÇÕES]
```

`PATCH /v1/tax_periods/{id}`

Aplica uma ação sobre o período: `recalculate` recalcula os impostos (somente períodos mensais), `close` encerra o período e `reopen` o reabre.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `action` | `string` | sim | Ação a executar: `recalculate`, `close` ou `reopen`. Valores: `recalculate`, `close`, `reopen`. |

Outros campos e objetos aninhados: `kobana schema billing.tax-periods.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Período tributário atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## tax-periods delete

Excluir período tributário

```text
kobana billing tax-periods delete [ID] [OPÇÕES]
```

`DELETE /v1/tax_periods/{id}`

Remove permanentemente o período tributário. Use com cautela — períodos encerrados normalmente não devem ser excluídos.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Período tributário removido. |
| `404` | Recurso não encontrado na organização atual. |
