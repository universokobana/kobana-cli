---
id: invoices
title: "kobana billing invoices"
sidebar_label: "invoices"
---

# kobana billing invoices


Comandos de `kobana billing invoices`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`invoices installments`](#invoices-installments) | `GET /v1/invoices/{id}/installments` | Listar parcelas da fatura |
| [`invoices installments create`](#invoices-installments-create) | `POST /v1/invoices/{id}/installments` | Criar/atualizar parcelamento |
| [`invoices installments mark-paid`](#invoices-installments-mark-paid) | `POST /v1/invoices/{id}/installments/{installment_id}/mark-paid` | Marcar parcela como paga |
| [`invoices installments pay`](#invoices-installments-pay) | `POST /v1/invoices/{id}/installments/{installment_id}/pay` | Pagar parcela |
| [`invoices pending-nfe`](#invoices-pending-nfe) | `GET /v1/invoices/pending-nfe` | Listar faturas com NFe pendente |
| [`invoices pending-payment`](#invoices-pending-payment) | `GET /v1/invoices/pending-payment` | Listar faturas com pagamento pendente |
| [`invoices list`](#invoices-list) | `GET /v1/invoices` | Listar faturas |
| [`invoices create`](#invoices-create) | `POST /v1/invoices` | Criar fatura |
| [`invoices update`](#invoices-update) | `PUT /v1/invoices` | Criar ou atualizar fatura |
| [`invoices get`](#invoices-get) | `GET /v1/invoices/{id}` | Obter fatura |
| [`invoices delete`](#invoices-delete) | `DELETE /v1/invoices/{id}` | Excluir fatura |
| [`invoices apply-credits`](#invoices-apply-credits) | `POST /v1/invoices/{id}/apply-credits` | Aplicar créditos na fatura |
| [`invoices finalize`](#invoices-finalize) | `POST /v1/invoices/{id}/finalize` | Finalizar fatura |
| [`invoices issue-nfe`](#invoices-issue-nfe) | `POST /v1/invoices/{id}/issue-nfe` | Emitir NFe para fatura |
| [`invoices pay`](#invoices-pay) | `POST /v1/invoices/{id}/pay` | Pagar fatura |
| [`invoices payment-methods`](#invoices-payment-methods) | `GET /v1/invoices/{id}/payment-methods` | Listar métodos de pagamento disponíveis |
| [`invoices pdf`](#invoices-pdf) | `GET /v1/invoices/{id}/pdf` | Baixar PDF da fatura |
| [`invoices remove-credits`](#invoices-remove-credits) | `POST /v1/invoices/{id}/remove-credits` | Remover créditos aplicados |
| [`invoices send-reminder`](#invoices-send-reminder) | `POST /v1/invoices/{id}/send-reminder` | Enviar lembrete de cobrança |
| [`invoices undo-payment`](#invoices-undo-payment) | `POST /v1/invoices/{id}/undo-payment` | Desfazer pagamento da fatura |
| [`invoices void`](#invoices-void) | `POST /v1/invoices/{id}/void` | Cancelar (void) fatura |

## invoices installments

Listar parcelas da fatura

```text
kobana billing invoices installments [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/invoices/{id}/installments`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de parcelas. |
| `404` | Recurso não encontrado na organização atual. |

## invoices installments create

Criar/atualizar parcelamento

```text
kobana billing invoices installments create [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/installments`

Cria ou substitui o cronograma de parcelas da fatura. Mínimo de 2 parcelas.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `installments` | `array<object>` | sim | Lista de parcelas. Mínimo 2. |

Outros campos e objetos aninhados: `kobana schema billing.invoices.installments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Parcelamento criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## invoices installments mark-paid

Marcar parcela como paga

```text
kobana billing invoices installments mark-paid [ID] [INSTALLMENT_ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/installments/{installment_id}/mark-paid`

Marca manualmente uma parcela como paga (reconciliação, sem chamada ao gateway).

**Argumentos do caminho:** `id`, `installment_id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Parcela marcada como paga. |
| `404` | Recurso não encontrado na organização atual. |

## invoices installments pay

Pagar parcela

```text
kobana billing invoices installments pay [ID] [INSTALLMENT_ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/installments/{installment_id}/pay`

Processa o pagamento de uma parcela via gateway. A parcela anterior deve estar paga.

**Argumentos do caminho:** `id`, `installment_id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `payment_method` | `string` | sim | Quando informado, dispara o gateway (`pix`, `bank_slip` ou `card`) ou cria uma cobrança por transferência (`bank_transfer`, exige `bank_account_id` ou `pix_key_id`). Omita para marcar como paga manualmente. Valores: `pix`, `bank_slip`, `card`, `bank_transfer`. |
| `payment_method_id` | `string (uuid)` | não | UUID de um método de pagamento salvo (ex.: cartão tokenizado). |
| `card` | `object` | não | Dados do cartão quando `payment_method = card` e não há `payment_method_id`. |
| `bank_account_id` | `string (uuid)` | não | Com `payment_method = bank_transfer`: UUID da conta bancária da organização para onde o cliente deve transferir (veja `/bank_accounts`). Informe este ou `pix_key_id`, nunca os dois. |
| `pix_key_id` | `string (uuid)` | não | Com `payment_method = bank_transfer`: UUID da chave Pix da organização para onde o cliente deve transferir (veja `/pix_keys`). Informe este ou `bank_account_id`, nunca os dois. |

Outros campos e objetos aninhados: `kobana schema billing.invoices.installments.pay`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do pagamento da parcela. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## invoices pending-nfe

Listar faturas com NFe pendente

```text
kobana billing invoices pending-nfe [OPÇÕES]
```

`GET /v1/invoices/pending-nfe`

Retorna faturas pagas, mas com emissão de NFe pendente — sem NFe, ou com NFe em status `pending`, `processing` ou `error`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | não | Filtra por `billing_account_id`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de faturas com NFe pendente. |
| `404` | Recurso não encontrado na organização atual. |

## invoices pending-payment

Listar faturas com pagamento pendente

```text
kobana billing invoices pending-payment [OPÇÕES]
```

`GET /v1/invoices/pending-payment`

Retorna faturas finalizadas (`status = open`) cuja `due_date` já passou.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | não | Filtra por `billing_account_id`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de faturas com pagamento pendente. |
| `404` | Recurso não encontrado na organização atual. |

## invoices list

Listar faturas

```text
kobana billing invoices list [OPÇÕES]
```

`GET /v1/invoices`

Retorna uma lista paginada de faturas da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | não | Filtra por status: `draft`, `open`, `paid`, `void` ou `uncollectible`. Valores: `draft`, `open`, `paid`, `void`, `uncollectible`. |
| `billing_account_id` | `string` | não | Filtra por `billing_account_id`. |
| `company_id` | `string` | não | Filtra por `company_id` emissor. |
| `subscription_id` | `string` | não | Filtra por `subscription_id` de origem. |
| `due_date_from` | `string` | não | Filtra por `due_date` inicial (inclusivo). |
| `due_date_to` | `string` | não | Filtra por `due_date` final (inclusivo). |
| `date_from` | `string` | não | Filtra por `created_at` inicial (inclusivo). |
| `date_to` | `string` | não | Filtra por `created_at` final (inclusivo). |
| `search` | `string` | não | Busca textual livre. |
| `total_min` | `number,null` | não | Filtra por valor total mínimo, em reais (float, até 4 casas decimais). |
| `total_max` | `number,null` | não | Filtra por valor total máximo, em reais (float, até 4 casas decimais). |
| `document_number` | `string` | não | Filtra por CPF/CNPJ do cliente. |
| `without_payments` | `string` | não | Quando `true`, retorna apenas faturas sem pagamentos associados. Valores: `true`, `false`. |
| `plan_filter` | `string` | não | Filtra por origem: `all`, `with_plan` (com assinatura) ou `without_plan` (avulsa). Valores: `all`, `with_plan`, `without_plan`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de faturas. |
| `404` | Recurso não encontrado na organização atual. |

## invoices create

Criar fatura

```text
kobana billing invoices create [OPÇÕES]
```

`POST /v1/invoices`

Cria uma fatura para uma conta de cobrança. Aceita itens de linha, parcelas opcionais (`installments`, mínimo 2) e política de emissão de NFe.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sim | UUID da conta de cobrança alvo. **Obrigatório**. |
| `line_items` | `array<object>` | sim | Itens de linha. Mínimo 1. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não | UUID da empresa emissora. Padrão: empresa padrão da organização. |
| `subscription_id` | `string (uuid)` | não | UUID da assinatura de origem. |
| `product_group_id` | `string (uuid)` | não | UUID do grupo de produtos. |
| `collection_method` | `string` | não | Método de cobrança. Padrão: `manual_invoice`. Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `due_date` | `string` | não | Data de vencimento. Padrão: hoje + `payment_terms_days` da conta. |
| `description` | `string` | não | Descrição livre exibida no topo da fatura. |
| `footer` | `string` | não | Rodapé livre exibido ao final da fatura. |
| `discount` | `number` | não | Desconto fixo em reais. |
| `discount_description` | `string` | não | Descrição do desconto (ex.: "Desconto por pagamento anual (20%)"). |
| `installments` | `array<object>` | não | Cronograma de parcelas (mínimo 2). Cada item tem `amount` e `due_date`. |
| `nfe_issuance_policy` | `string` | não | Política de emissão de NFe para faturas parceladas. Valores: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `period_start` | `string` | não | Início do período de competência (preenchido automaticamente em faturas de assinatura). |
| `period_end` | `string` | não | Fim do período de competência (preenchido automaticamente em faturas de assinatura). |
| `custom_metadata` | `object` | não | Campos personalizados (validados contra o schema da organização). |

Outros campos e objetos aninhados: `kobana schema billing.invoices.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Fatura criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## invoices update

Criar ou atualizar fatura

```text
kobana billing invoices update [OPÇÕES]
```

`PUT /v1/invoices`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança alvo. **Obrigatório**. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não | UUID da empresa emissora. Padrão: empresa padrão da organização. |
| `subscription_id` | `string (uuid)` | não | UUID da assinatura de origem. |
| `product_group_id` | `string (uuid)` | não | UUID do grupo de produtos. |
| `collection_method` | `string` | não | Método de cobrança. Padrão: `manual_invoice`. Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `due_date` | `string` | não | Data de vencimento. Padrão: hoje + `payment_terms_days` da conta. |
| `description` | `string` | não | Descrição livre exibida no topo da fatura. |
| `footer` | `string` | não | Rodapé livre exibido ao final da fatura. |
| `line_items` | `array<object>` | não | Itens de linha. Mínimo 1. |
| `discount` | `number` | não | Desconto fixo em reais. |
| `discount_description` | `string` | não | Descrição do desconto (ex.: "Desconto por pagamento anual (20%)"). |
| `installments` | `array<object>` | não | Cronograma de parcelas (mínimo 2). Cada item tem `amount` e `due_date`. |
| `nfe_issuance_policy` | `string` | não | Política de emissão de NFe para faturas parceladas. Valores: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `period_start` | `string` | não | Início do período de competência (preenchido automaticamente em faturas de assinatura). |
| `period_end` | `string` | não | Fim do período de competência (preenchido automaticamente em faturas de assinatura). |
| `custom_metadata` | `object` | não | Campos personalizados (validados contra o schema da organização). |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.invoices.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## invoices get

Obter fatura

```text
kobana billing invoices get [ID] [OPÇÕES]
```

`GET /v1/invoices/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da fatura. |
| `404` | Recurso não encontrado na organização atual. |

## invoices delete

Excluir fatura

```text
kobana billing invoices delete [ID] [OPÇÕES]
```

`DELETE /v1/invoices/{id}`

Exclui uma fatura `draft`, `open` ou `void` (os créditos aplicados a uma fatura `open` voltam ao saldo do cliente). Uma fatura `paid` ou `uncollectible` não pode ser excluída — desfaça o pagamento ou anule-a antes.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Fatura excluída. |
| `404` | Recurso não encontrado na organização atual. |

## invoices apply-credits

Aplicar créditos na fatura

```text
kobana billing invoices apply-credits [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/apply-credits`

Aplica créditos da conta de cobrança em uma fatura aberta (`status = open`).

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `credit_ids` | `array<string (uuid)>` | sim | UUIDs dos créditos a aplicar (mínimo 1). |
| `amount` | `number` | não | Valor a aplicar em reais. Padrão: cobre toda a fatura. |

Outros campos e objetos aninhados: `kobana schema billing.invoices.apply-credits`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Créditos aplicados. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## invoices finalize

Finalizar fatura

```text
kobana billing invoices finalize [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/finalize`

Finaliza uma fatura em rascunho (`draft` → `open`).

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Fatura finalizada. |
| `404` | Recurso não encontrado na organização atual. |

## invoices issue-nfe

Emitir NFe para fatura

```text
kobana billing invoices issue-nfe [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/issue-nfe`

Dispara a emissão da NFe (Nota Fiscal Eletrônica) para a fatura.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | NFe enfileirada/emitida. |
| `404` | Recurso não encontrado na organização atual. |

## invoices pay

Pagar fatura

```text
kobana billing invoices pay [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/pay`

Dois modos: (a) quando `payment_method` é informado, cria um pagamento via gateway (`pix`, `bank_slip` ou `card`) e retorna os artefatos do método (QR code, código de barras ou status do cartão) — ou, com `bank_transfer`, uma cobrança por transferência para a conta bancária (`bank_account_id`) ou chave Pix (`pix_key_id`) da organização, sem gateway, devolvendo os dados em `bank_transfer` e ficando pendente até a baixa manual; (b) quando omitido, marca a fatura como paga manualmente (reconciliação).

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `payment_method` | `string` | não | Quando informado, dispara o gateway (`pix`, `bank_slip` ou `card`) ou cria uma cobrança por transferência (`bank_transfer`, exige `bank_account_id` ou `pix_key_id`). Omita para marcar como paga manualmente. Valores: `pix`, `bank_slip`, `card`, `bank_transfer`. |
| `payment_method_id` | `string (uuid)` | não | UUID de um método de pagamento salvo (ex.: cartão tokenizado). |
| `card` | `object` | não | Dados do cartão quando `payment_method = card` e não há `payment_method_id`. |
| `bank_account_id` | `string (uuid)` | não | Com `payment_method = bank_transfer`: UUID da conta bancária da organização para onde o cliente deve transferir (veja `/bank_accounts`). Informe este ou `pix_key_id`, nunca os dois. |
| `pix_key_id` | `string (uuid)` | não | Com `payment_method = bank_transfer`: UUID da chave Pix da organização para onde o cliente deve transferir (veja `/pix_keys`). Informe este ou `bank_account_id`, nunca os dois. |
| `amount` | `number` | não | Valor a cobrar em reais. Padrão: valor restante da fatura. Obrigatório quando `installment_ids` é informado, e deve ser igual à soma do `net` das parcelas selecionadas. |
| `installment_ids` | `array<string (uuid)>` | não | UUIDs das parcelas que o pagamento cobre (faturas parceladas). |

Outros campos e objetos aninhados: `kobana schema billing.invoices.pay`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado do pagamento ou fatura marcada como paga. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## invoices payment-methods

Listar métodos de pagamento disponíveis

```text
kobana billing invoices payment-methods [ID] [OPÇÕES]
```

`GET /v1/invoices/{id}/payment-methods`

Lista os métodos de pagamento que podem ser usados para pagar esta fatura, com base nos gateways ativos da organização e nas configurações do portal.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Métodos de pagamento disponíveis. |
| `404` | Recurso não encontrado na organização atual. |

## invoices pdf

Baixar PDF da fatura

```text
kobana billing invoices pdf [ID] [OPÇÕES]
```

`GET /v1/invoices/{id}/pdf`

Gera e baixa o PDF da fatura como anexo binário (`application/pdf`).

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | PDF da fatura. |
| `404` | Recurso não encontrado na organização atual. |

## invoices remove-credits

Remover créditos aplicados

```text
kobana billing invoices remove-credits [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/remove-credits`

Remove todos os créditos aplicados na fatura, devolvendo-os à conta de cobrança. Só funciona em faturas abertas (`status = open`).

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Créditos removidos. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## invoices send-reminder

Enviar lembrete de cobrança

```text
kobana billing invoices send-reminder [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/send-reminder`

Envia por e-mail um lembrete de cobrança da fatura para o `billing_email` da conta.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lembrete enviado. |
| `404` | Recurso não encontrado na organização atual. |

## invoices undo-payment

Desfazer pagamento da fatura

```text
kobana billing invoices undo-payment [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/undo-payment`

Reverte o pagamento da fatura (`paid` → `open`). O motivo (`reason`) é registrado no audit log.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `reason` | `string` | não | Motivo da reversão (registrado no audit log). |

Outros campos e objetos aninhados: `kobana schema billing.invoices.undo-payment`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento desfeito. |
| `404` | Recurso não encontrado na organização atual. |

## invoices void

Cancelar (void) fatura

```text
kobana billing invoices void [ID] [OPÇÕES]
```

`POST /v1/invoices/{id}/void`

Cancela uma fatura aberta (`open` → `void`). O motivo (`reason`) é registrado no audit log.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `reason` | `string` | não | Motivo do cancelamento (registrado no audit log). |

Outros campos e objetos aninhados: `kobana schema billing.invoices.void`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Fatura cancelada. |
| `404` | Recurso não encontrado na organização atual. |
