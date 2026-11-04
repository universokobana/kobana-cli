---
id: payments
title: "kobana billing payments"
sidebar_label: "payments"
---

# kobana billing payments


Comandos de `kobana billing payments`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`payments list`](#payments-list) | `GET /v1/payments` | Listar pagamentos |
| [`payments create`](#payments-create) | `POST /v1/payments` | Processar pagamento |
| [`payments bulk-update-invoice`](#payments-bulk-update-invoice) | `POST /v1/payments/bulk-update-invoice` | Reatribuir pagamentos a outra fatura |
| [`payments get`](#payments-get) | `GET /v1/payments/{id}` | Obter pagamento |
| [`payments delete`](#payments-delete) | `DELETE /v1/payments/{id}` | Excluir pagamento |
| [`payments cancel`](#payments-cancel) | `POST /v1/payments/{id}/cancel` | Cancelar pagamento |
| [`payments change-card`](#payments-change-card) | `POST /v1/payments/{id}/change-card` | Trocar cartão do pagamento |
| [`payments change-due-date`](#payments-change-due-date) | `POST /v1/payments/{id}/change-due-date` | Alterar vencimento do boleto |
| [`payments refund`](#payments-refund) | `POST /v1/payments/{id}/refund` | Estornar pagamento |
| [`payments retry`](#payments-retry) | `POST /v1/payments/{id}/retry` | Tentar pagamento novamente |
| [`payments send-email`](#payments-send-email) | `POST /v1/payments/{id}/send-email` | Enviar boleto por e-mail |
| [`payments sync`](#payments-sync) | `POST /v1/payments/{id}/sync` | Sincronizar pagamento com gateway |

## payments list

Listar pagamentos

```text
kobana billing payments list [OPÇÕES]
```

`GET /v1/payments`

Retorna uma lista paginada dos pagamentos da organização atual.

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
| `status` | `string` | não | Filtra por status do pagamento. Valores: `pending`, `processing`, `succeeded`, `failed`, `canceled`, `partial`, `refunded`, `partially_refunded`. |
| `invoice_id` | `string` | não | Filtra pelos pagamentos de uma fatura específica. |
| `billing_account_id` | `string` | não | Filtra pelos pagamentos de uma conta de cobrança. |
| `company_id` | `string` | não | Filtra pelos pagamentos de uma empresa específica. |
| `gateway_id` | `string` | não | Filtra pelos pagamentos processados por um gateway específico. |
| `payment_method_id` | `string` | não | Filtra pelos pagamentos de um meio de pagamento específico. |
| `payment_method_type` | `string` | não | Filtra por meio de pagamento: `card`, `bank_slip`, `pix` ou `bank_transfer`. Valores: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `external_id` | `string` | não | Filtra pelo ID externo (campo livre para identificadores do sistema legado). |
| `gateway_payment_id` | `string` | não | Filtra pelo identificador do pagamento no gateway upstream. |
| `invoice_filter` | `string` | não | Filtra por presença de fatura: `all`, `with_invoice` ou `without_invoice`. Valores: `all`, `with_invoice`, `without_invoice`. |
| `search` | `string` | não | Busca textual livre. |
| `amount_min` | `number,null` | não | Valor mínimo cobrado (em BRL). |
| `amount_max` | `number,null` | não | Valor máximo cobrado (em BRL). |
| `paid_amount_min` | `number,null` | não | Valor mínimo efetivamente recebido (em BRL). |
| `paid_amount_max` | `number,null` | não | Valor máximo efetivamente recebido (em BRL). |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `paid_after` | `string` | não | Data inicial de confirmação do pagamento — formato ISO 8601. |
| `paid_before` | `string` | não | Data final de confirmação do pagamento — formato ISO 8601. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de pagamentos. |
| `404` | Recurso não encontrado na organização atual. |

## payments create

Processar pagamento

```text
kobana billing payments create [OPÇÕES]
```

`POST /v1/payments`

Processa um pagamento para uma fatura existente. Aceita um `payment_method_id` previamente registrado **ou** um bloco `payment_method` com dados de cartão/PIX/boleto. Quando `amount` é omitido, cobra o saldo aberto da fatura.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `invoice_id` | `string (uuid)` | sim | UUID da fatura a ser paga. |
| `payment_method_id` | `string (uuid)` | não | UUID de um meio de pagamento previamente registrado para a conta. |
| `payment_method` | `object` | não |  |
| `amount` | `number` | não | Valor a cobrar em reais (float, até 4 casas decimais). Padrão: saldo aberto da fatura. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `custom_metadata` | `object` | não | Metadados livres definidos pelo cliente para correlação interna. |

Outros campos e objetos aninhados: `kobana schema billing.payments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pagamento criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payments bulk-update-invoice

Reatribuir pagamentos a outra fatura

```text
kobana billing payments bulk-update-invoice [OPÇÕES]
```

`POST /v1/payments/bulk-update-invoice`

Atualiza em massa a fatura associada aos pagamentos informados. Útil para corrigir vínculos quando pagamentos chegaram apontando para a fatura errada. Um pagamento que já quita mais de uma fatura não é movido: volta em `failed` (com `id` e `error`), e `failed_count` conta esses casos.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `payment_ids` | `array<string (uuid)>` | sim | Lista de UUIDs dos pagamentos a atualizar. |
| `invoice_id` | `string (uuid)` | sim | UUID da fatura de destino. |

Outros campos e objetos aninhados: `kobana schema billing.payments.bulk-update-invoice`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamentos reatribuídos. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payments get

Obter pagamento

```text
kobana billing payments get [ID] [OPÇÕES]
```

`GET /v1/payments/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe do pagamento. |
| `404` | Recurso não encontrado na organização atual. |

## payments delete

Excluir pagamento

```text
kobana billing payments delete [ID] [OPÇÕES]
```

`DELETE /v1/payments/{id}`

Remove um pagamento. Pagamentos já confirmados não podem ser excluídos — use `cancel` ou `refund`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento removido. |
| `404` | Recurso não encontrado na organização atual. |

## payments cancel

Cancelar pagamento

```text
kobana billing payments cancel [ID] [OPÇÕES]
```

`POST /v1/payments/{id}/cancel`

Cancela um pagamento em aberto. Quando `cancel_on_gateway` é `true` (padrão), o cancelamento é propagado ao gateway.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `reason` | `string` | não | Motivo do cancelamento (registrado no audit log). |
| `cancel_on_gateway` | `boolean` | não | Quando `true` (padrão), também cancela no gateway upstream. |

Outros campos e objetos aninhados: `kobana schema billing.payments.cancel`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento cancelado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payments change-card

Trocar cartão do pagamento

```text
kobana billing payments change-card [ID] [OPÇÕES]
```

`POST /v1/payments/{id}/change-card`

Troca o cartão de crédito de um pagamento falho processado pelo Pagar.me. O novo cartão precisa estar registrado previamente em `payment_methods`.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `credit_card_id` | `string (uuid)` | sim | UUID do cartão de crédito ativo da conta de cobrança. |

Outros campos e objetos aninhados: `kobana schema billing.payments.change-card`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cartão trocado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payments change-due-date

Alterar vencimento do boleto

```text
kobana billing payments change-due-date [ID] [OPÇÕES]
```

`POST /v1/payments/{id}/change-due-date`

Altera o vencimento de um pagamento em boleto pendente emitido via gateway Kobana. A nova data precisa ser futura.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `due_date` | `string` | sim | Nova data de vencimento no formato `YYYY-MM-DD`. |

Outros campos e objetos aninhados: `kobana schema billing.payments.change-due-date`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Vencimento atualizado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payments refund

Estornar pagamento

```text
kobana billing payments refund [ID] [OPÇÕES]
```

`POST /v1/payments/{id}/refund`

Estorna um pagamento bem-sucedido. Sem `amount`, estorna o valor total; com valor parcial, gera estorno parcial.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | não | Valor a estornar em reais (float, até 4 casas decimais). Padrão: valor total do pagamento. |
| `reason` | `string` | não | Motivo do estorno (registrado no audit log). |

Outros campos e objetos aninhados: `kobana schema billing.payments.refund`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento estornado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payments retry

Tentar pagamento novamente

```text
kobana billing payments retry [ID] [OPÇÕES]
```

`POST /v1/payments/{id}/retry`

Reprocessa um pagamento falho. Pode opcionalmente trocar para um cartão novo (via `card`) ou para um meio de pagamento salvo (`payment_method_id`).

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `card` | `object` | não |  |
| `payment_method_id` | `string (uuid)` | não | UUID de um meio de pagamento salvo a ser usado na nova tentativa. |

Outros campos e objetos aninhados: `kobana schema billing.payments.retry`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento reprocessado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payments send-email

Enviar boleto por e-mail

```text
kobana billing payments send-email [ID] [OPÇÕES]
```

`POST /v1/payments/{id}/send-email`

Envia o link do boleto por e-mail para o destinatário informado. Disponível apenas para pagamentos em boleto pendentes.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `email` | `string (email)` | sim | Endereço de e-mail do destinatário. |
| `message` | `string` | não | Mensagem opcional incluída no corpo do e-mail. |

Outros campos e objetos aninhados: `kobana schema billing.payments.send-email`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | E-mail enviado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## payments sync

Sincronizar pagamento com gateway

```text
kobana billing payments sync [ID] [OPÇÕES]
```

`POST /v1/payments/{id}/sync`

Sincroniza o status do pagamento com o gateway upstream (PIX/boleto Kobana ou estorno Pagar.me). Retorna o pagamento atualizado.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento sincronizado. |
| `404` | Recurso não encontrado na organização atual. |
