---
id: subscription-item-changes
title: "kobana billing subscription-item-changes"
sidebar_label: "subscription-item-changes"
---

# kobana billing subscription-item-changes


Comandos de `kobana billing subscription-item-changes`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`subscription-item-changes list`](#subscription-item-changes-list) | `GET /v1/subscription_item_changes` | Listar mudanças de itens de assinatura |
| [`subscription-item-changes get`](#subscription-item-changes-get) | `GET /v1/subscription_item_changes/{id}` | Obter mudança de item de assinatura |
| [`subscription-item-changes cancel`](#subscription-item-changes-cancel) | `POST /v1/subscription_item_changes/{id}/cancel` | Cancela uma mudança agendada |

## subscription-item-changes list

Listar mudanças de itens de assinatura

```text
kobana billing subscription-item-changes list [OPÇÕES]
```

`GET /v1/subscription_item_changes`

Retorna uma lista paginada das mudanças aplicadas (ou agendadas) sobre itens de assinaturas da organização — adições, remoções, alterações de quantidade e trocas de preço. Cada registro inclui o snapshot das operações, os valores de proporção (proration) em reais (float, até 4 casas decimais) e os ponteiros para a fatura/credit gerados.

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
| `subscription_id` | `string` | não | Filtra pelas mudanças de uma assinatura específica. |
| `billing_account_id` | `string` | não | Filtra pelas mudanças de uma conta de cobrança específica. |
| `status` | `string` | não | Filtra por status: `pending`, `scheduled`, `completed`, `canceled` ou `failed`. Valores: `pending`, `scheduled`, `completed`, `canceled`, `failed`. |
| `timing` | `string` | não | Filtra pelo momento de aplicação da mudança: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `initiated_by_role` | `string` | não | Filtra pela origem da mudança: `dashboard`, `portal` ou `api`. Valores: `dashboard`, `portal`, `api`. |
| `proration_charge_min` | `number,null` | não | Valor mínimo da cobrança de proporção, em reais (R$). Internamente armazenado em reais (float, até 4 casas decimais). |
| `proration_charge_max` | `number,null` | não | Valor máximo da cobrança de proporção, em reais (R$). Internamente armazenado em reais (float, até 4 casas decimais). |
| `proration_credit_min` | `number,null` | não | Valor mínimo do crédito de proporção, em reais (R$). Internamente armazenado em reais (float, até 4 casas decimais). |
| `proration_credit_max` | `number,null` | não | Valor máximo do crédito de proporção, em reais (R$). Internamente armazenado em reais (float, até 4 casas decimais). |
| `effective_date_from` | `string` | não | Data efetiva mínima da mudança (inclusive), no formato `YYYY-MM-DD`. |
| `effective_date_to` | `string` | não | Data efetiva máxima da mudança (inclusive), no formato `YYYY-MM-DD`. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de mudanças de itens. |
| `404` | Recurso não encontrado na organização atual. |

## subscription-item-changes get

Obter mudança de item de assinatura

```text
kobana billing subscription-item-changes get [ID] [OPÇÕES]
```

`GET /v1/subscription_item_changes/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da mudança de item. |
| `404` | Recurso não encontrado na organização atual. |

## subscription-item-changes cancel

Cancela uma mudança agendada

```text
kobana billing subscription-item-changes cancel [ID] [OPÇÕES]
```

`POST /v1/subscription_item_changes/{id}/cancel`

Cancela uma mudança de itens ainda agendada (`timing=end_of_period`). Mudanças já aplicadas não podem ser canceladas. Devolve o registro com o novo status. Exige o nível de acesso `write`. Substitui `DELETE /v1/subscription_item_changes/{id}`, removido.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Mudança cancelada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
