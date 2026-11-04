---
id: credit-purchases
title: "kobana billing credit-purchases"
sidebar_label: "credit-purchases"
---

# kobana billing credit-purchases


Comandos de `kobana billing credit-purchases`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`credit-purchases create`](#credit-purchases-create) | `POST /v1/credit_purchases` | Comprar créditos |

## credit-purchases create

Comprar créditos

```text
kobana billing credit-purchases create [OPÇÕES]
```

`POST /v1/credit_purchases`

Gera e finaliza uma fatura avulsa (`billing_reason = credit_purchase`) com a quantidade de créditos de um produto do tipo `credits`, independente de assinatura. O saldo passa a valer quando a fatura é paga: o webhook `invoice.paid` traz `line_items[].product_type` e `quantity`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `product_id` | `string (uuid)` | sim | Produto do tipo `credits`. |
| `quantity` | `integer` | sim | Quantidade de créditos (inteira; múltipla do incremento do produto). |
| `billing_account_id` | `string (uuid)` | não | ID da conta de cobrança. Informe este **ou** `billing_account_public_id`. |
| `billing_account_public_id` | `string (uuid)` | não | Identificador público da conta de cobrança. |
| `price_id` | `string (uuid)` | não | Preço a cobrar. Padrão: o preço padrão ativo do produto. |
| `collection_method` | `string` | não | `manual_charge` (padrão): o cliente paga a fatura. `charge_automatically`: cobra o cartão padrão ao finalizar. Valores: `charge_automatically`, `manual_charge`. |
| `due_date` | `string` | não | Vencimento. Padrão: o prazo de pagamento da conta. |

Outros campos e objetos aninhados: `kobana schema billing.credit-purchases.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Fatura da compra, com os itens. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
