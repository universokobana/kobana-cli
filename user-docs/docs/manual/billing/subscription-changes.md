---
id: subscription-changes
title: "kobana billing subscription-changes"
sidebar_label: "subscription-changes"
---

# kobana billing subscription-changes


Comandos de `kobana billing subscription-changes`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`subscription-changes config list`](#subscription-changes-config-list) | `GET /v1/subscription_changes/config` | Obter configuração de mudanças de assinatura |
| [`subscription-changes config update`](#subscription-changes-config-update) | `PUT /v1/subscription_changes/config` | Atualizar configuração de mudanças de assinatura |

## subscription-changes config list

Obter configuração de mudanças de assinatura

```text
kobana billing subscription-changes config list [OPÇÕES]
```

`GET /v1/subscription_changes/config`

Retorna a configuração da organização para mudanças em itens de assinaturas — operações permitidas (incluir/remover/alterar quantidade/trocar preço), método de proporção (proration) por operação, momento de aplicação (`immediate` ou `end_of_period`), reembolso/crédito na remoção e self-service no portal. Se ainda não existir, é criada com os defaults na primeira leitura.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Configuração da organização. |
| `404` | Recurso não encontrado na organização atual. |

## subscription-changes config update

Atualizar configuração de mudanças de assinatura

```text
kobana billing subscription-changes config update [OPÇÕES]
```

`PUT /v1/subscription_changes/config`

Atualiza parcialmente a configuração. Apenas os campos enviados são alterados. `refund_on_remove` exige que `credit_on_remove` também esteja habilitado — reembolso é uma variante de crédito.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `allow_item_add` | `boolean` | não | Permite adicionar novos itens a uma assinatura existente. |
| `allow_item_remove` | `boolean` | não | Permite remover itens de uma assinatura existente. |
| `allow_quantity_change` | `boolean` | não | Permite alterar a quantidade de itens já presentes na assinatura. |
| `allow_price_swap` | `boolean` | não | Permite trocar o preço (variação) de um item existente sem removê-lo. |
| `add_proration` | `string` | não | Método de proporção ao adicionar itens: `full_proration`, `no_proration` ou `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `remove_proration` | `string` | não | Método de proporção ao remover itens: `full_proration`, `no_proration` ou `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `quantity_up_proration` | `string` | não | Método de proporção ao aumentar a quantidade de um item. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `quantity_down_proration` | `string` | não | Método de proporção ao reduzir a quantidade de um item. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `add_timing` | `string` | não | Momento de aplicação ao adicionar itens: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `remove_timing` | `string` | não | Momento de aplicação ao remover itens: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `quantity_up_timing` | `string` | não | Momento de aplicação ao aumentar quantidade: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `quantity_down_timing` | `string` | não | Momento de aplicação ao reduzir quantidade: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `refund_on_remove` | `boolean` | não | Quando `true`, gera reembolso ao cliente ao remover itens. Exige `credit_on_remove` habilitado. |
| `credit_on_remove` | `boolean` | não | Quando `true`, gera crédito a favor do cliente ao remover itens. |
| `portal_self_service` | `boolean` | não | Quando `true`, libera ao cliente alterar a própria assinatura no portal. |
| `portal_require_confirmation` | `boolean` | não | Quando `true`, exige confirmação explícita do cliente antes de aplicar a mudança no portal. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados arbitrários definidos pelo cliente (chave/valor). |

Outros campos e objetos aninhados: `kobana schema billing.subscription-changes.config.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Configuração atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |
