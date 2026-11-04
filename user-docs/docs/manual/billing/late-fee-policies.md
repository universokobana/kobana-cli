---
id: late-fee-policies
title: "kobana billing late-fee-policies"
sidebar_label: "late-fee-policies"
---

# kobana billing late-fee-policies


Comandos de `kobana billing late-fee-policies`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`late-fee-policies list`](#late-fee-policies-list) | `GET /v1/late_fee_policies` | Listar políticas de juros e multa |
| [`late-fee-policies create`](#late-fee-policies-create) | `POST /v1/late_fee_policies` | Criar política de juros e multa |
| [`late-fee-policies simulate`](#late-fee-policies-simulate) | `POST /v1/late_fee_policies/simulate` | Simular juros e multa |
| [`late-fee-policies get`](#late-fee-policies-get) | `GET /v1/late_fee_policies/{id}` | Consultar política de juros e multa |
| [`late-fee-policies update`](#late-fee-policies-update) | `PATCH /v1/late_fee_policies/{id}` | Atualizar política de juros e multa |
| [`late-fee-policies delete`](#late-fee-policies-delete) | `DELETE /v1/late_fee_policies/{id}` | Remover política de juros e multa |
| [`late-fee-policies default`](#late-fee-policies-default) | `POST /v1/late_fee_policies/{id}/default` | Definir como política padrão |

## late-fee-policies list

Listar políticas de juros e multa

```text
kobana billing late-fee-policies list [OPÇÕES]
```

`GET /v1/late_fee_policies`

Retorna as políticas de juros e multa da organização. A política aplicada a uma fatura é resolvida em cascata: fatura → assinatura → conta de cobrança → política padrão da organização. Um nível nulo significa herdar, nunca "sem encargo" — não cobrar nada é uma política explícita com os dois modos em `none`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `include_deleted` | `string` | não | Inclui políticas removidas logicamente. Valores: `true`, `false`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de políticas de juros e multa. |
| `404` | Recurso não encontrado na organização atual. |

## late-fee-policies create

Criar política de juros e multa

```text
kobana billing late-fee-policies create [OPÇÕES]
```

`POST /v1/late_fee_policies`

Cria uma política. Multa e juros acima dos tetos legais (2% de multa e 1% de juros ao mês, CDC art. 52) são recusados, a menos que `enforce_legal_caps` seja `false` — o que é registrado em auditoria à parte, por ser decisão de risco jurídico.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome da política, único na organização. |
| `is_default` | `boolean` | não | Indica se é a política padrão da organização. |
| `fine_mode` | `string` | não | Como a multa é calculada. Incide uma única vez. `percentage` usa `fine_percent`, `fixed` usa `fine_amount`. Valores: `none`, `percentage`, `fixed`. |
| `fine_percent` | `number` | não | Percentual da multa sobre o saldo devedor, de 0 a 100. Preenchido apenas quando `fine_mode` é `percentage`. |
| `fine_amount` | `number` | não | Valor fixo da multa, em reais. Preenchido apenas quando `fine_mode` é `fixed`. |
| `fine_days` | `integer` | não | Dias após o vencimento em que a multa passa a incidir. O padrão é 1 — o dia seguinte ao vencimento. |
| `interest_mode` | `string` | não | Como os juros de mora correm. Os percentuais usam `interest_percent`; `daily_amount` usa `interest_amount`. Os juros incidem sobre o principal, nunca sobre principal mais multa. Valores: `none`, `daily_amount`, `daily_percentage`, `monthly_percentage`, `yearly_percentage`. |
| `interest_percent` | `number` | não | Percentual de juros por dia, mês ou ano, conforme `interest_mode`. O percentual mensal é dividido pro rata die por 30; o anual, por 360. |
| `interest_amount` | `number` | não | Valor fixo de juros por dia, em reais. Preenchido apenas quando `interest_mode` é `daily_amount`. |
| `interest_days` | `integer` | não | Dias após o vencimento em que os juros passam a correr. O padrão é 1. |
| `day_count` | `string` | não | Base de contagem dos dias de atraso. Apenas `calendar` é aceito por ora: a apuração local ainda não tem calendário de feriados, e contar dias úteis sem ele superestimaria o encargo em relação ao que o banco recolhe. Valores: `calendar`, `business`. |
| `max_interest_months` | `integer` | não | Limite de meses de juros acumulados. Nulo significa sem limite. |
| `max_total_charge_percent` | `number` | não | Limite do encargo total (multa mais juros) como percentual do saldo devedor. A multa é preservada e os juros são aparados quando o teto morde. |
| `enforce_legal_caps` | `boolean` | não | Quando verdadeiro, recusa multa acima de 2% e juros acima de 1% ao mês. Desligar é decisão deliberada para contratos fora da relação de consumo, e é auditado. |
| `partial_allocation` | `string` | não | Ordem de imputação quando um pagamento não cobre principal e encargos. `charges_first` segue o art. 354 do Código Civil: juros antes do capital. Valores: `charges_first`, `principal_first`. |
| `register_on_charge` | `boolean` | não | Registra a instrução de multa e juros no banco ao emitir boleto ou Pix, para que a própria cobrança recolha os encargos após o vencimento. |
| `apply_on_dynamic_charge` | `boolean` | não | Recalcula o valor cobrado em cartão e em reemissões, onde não existe instrução bancária que cobre o encargo. |
| `charge_instructions` | `string` | não | Texto livre impresso nas instruções do boleto. |

Outros campos e objetos aninhados: `kobana schema billing.late-fee-policies.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Política criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## late-fee-policies simulate

Simular juros e multa

```text
kobana billing late-fee-policies simulate [OPÇÕES]
```

`POST /v1/late_fee_policies/simulate`

Calcula quanto um saldo devedor deve de multa e juros numa data, sem criar nada. Informe `policy_id` ou uma `policy` inline. Serve para conferir o cálculo antes de uma fatura vencer de verdade.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `principal` | `number` | sim | Saldo devedor sobre o qual apurar os encargos, em reais. |
| `due_date` | `string (date-time)` | sim | Vencimento a partir do qual o atraso é contado. |
| `reference_date` | `string (date-time)` | não | Data de apuração. O padrão é agora. Os dias são contados na data civil em America/Sao_Paulo. |
| `policy_id` | `string (uuid)` | não | Política existente a usar. Alternativa a `policy`. |
| `policy` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema billing.late-fee-policies.simulate`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Encargos apurados na data informada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## late-fee-policies get

Consultar política de juros e multa

```text
kobana billing late-fee-policies get [ID] [OPÇÕES]
```

`GET /v1/late_fee_policies/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Política encontrada. |
| `404` | Recurso não encontrado na organização atual. |

## late-fee-policies update

Atualizar política de juros e multa

```text
kobana billing late-fee-policies update [ID] [OPÇÕES]
```

`PATCH /v1/late_fee_policies/{id}`

Atualiza a política. Faturas já finalizadas não são afetadas: cada fatura congela a política vigente no momento da finalização, para que uma renegociação não mude retroativamente o que já foi emitido.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não | Nome da política, único na organização. |
| `is_default` | `boolean` | não | Indica se é a política padrão da organização. |
| `fine_mode` | `string` | não | Como a multa é calculada. Incide uma única vez. `percentage` usa `fine_percent`, `fixed` usa `fine_amount`. Valores: `none`, `percentage`, `fixed`. |
| `fine_percent` | `number` | não | Percentual da multa sobre o saldo devedor, de 0 a 100. Preenchido apenas quando `fine_mode` é `percentage`. |
| `fine_amount` | `number` | não | Valor fixo da multa, em reais. Preenchido apenas quando `fine_mode` é `fixed`. |
| `fine_days` | `integer` | não | Dias após o vencimento em que a multa passa a incidir. O padrão é 1 — o dia seguinte ao vencimento. |
| `interest_mode` | `string` | não | Como os juros de mora correm. Os percentuais usam `interest_percent`; `daily_amount` usa `interest_amount`. Os juros incidem sobre o principal, nunca sobre principal mais multa. Valores: `none`, `daily_amount`, `daily_percentage`, `monthly_percentage`, `yearly_percentage`. |
| `interest_percent` | `number` | não | Percentual de juros por dia, mês ou ano, conforme `interest_mode`. O percentual mensal é dividido pro rata die por 30; o anual, por 360. |
| `interest_amount` | `number` | não | Valor fixo de juros por dia, em reais. Preenchido apenas quando `interest_mode` é `daily_amount`. |
| `interest_days` | `integer` | não | Dias após o vencimento em que os juros passam a correr. O padrão é 1. |
| `day_count` | `string` | não | Base de contagem dos dias de atraso. Apenas `calendar` é aceito por ora: a apuração local ainda não tem calendário de feriados, e contar dias úteis sem ele superestimaria o encargo em relação ao que o banco recolhe. Valores: `calendar`, `business`. |
| `max_interest_months` | `integer` | não | Limite de meses de juros acumulados. Nulo significa sem limite. |
| `max_total_charge_percent` | `number` | não | Limite do encargo total (multa mais juros) como percentual do saldo devedor. A multa é preservada e os juros são aparados quando o teto morde. |
| `enforce_legal_caps` | `boolean` | não | Quando verdadeiro, recusa multa acima de 2% e juros acima de 1% ao mês. Desligar é decisão deliberada para contratos fora da relação de consumo, e é auditado. |
| `partial_allocation` | `string` | não | Ordem de imputação quando um pagamento não cobre principal e encargos. `charges_first` segue o art. 354 do Código Civil: juros antes do capital. Valores: `charges_first`, `principal_first`. |
| `register_on_charge` | `boolean` | não | Registra a instrução de multa e juros no banco ao emitir boleto ou Pix, para que a própria cobrança recolha os encargos após o vencimento. |
| `apply_on_dynamic_charge` | `boolean` | não | Recalcula o valor cobrado em cartão e em reemissões, onde não existe instrução bancária que cobre o encargo. |
| `charge_instructions` | `string` | não | Texto livre impresso nas instruções do boleto. |

Outros campos e objetos aninhados: `kobana schema billing.late-fee-policies.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Política atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## late-fee-policies delete

Remover política de juros e multa

```text
kobana billing late-fee-policies delete [ID] [OPÇÕES]
```

`DELETE /v1/late_fee_policies/{id}`

Remoção lógica. Faturas já emitidas seguem valendo pelo snapshot que carregam. A política padrão não pode ser removida — defina outra como padrão antes.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Política removida. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## late-fee-policies default

Definir como política padrão

```text
kobana billing late-fee-policies default [ID] [OPÇÕES]
```

`POST /v1/late_fee_policies/{id}/default`

Torna esta a política padrão da organização — o que uma fatura herda quando nem ela, nem sua assinatura, nem sua conta de cobrança indicam uma. Exatamente uma política carrega a marca.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Política definida como padrão. |
| `404` | Recurso não encontrado na organização atual. |
