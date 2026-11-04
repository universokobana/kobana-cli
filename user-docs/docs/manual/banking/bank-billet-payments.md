---
id: bank-billet-payments
title: "kobana banking bank-billet-payments"
sidebar_label: "bank-billet-payments"
---

# kobana banking bank-billet-payments


Comandos de `kobana banking bank-billet-payments`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`bank-billet-payments list`](#bank-billet-payments-list) | `GET /v1/bank_billet_payments` | Listar Pagamentos de Boleto |
| [`bank-billet-payments create`](#bank-billet-payments-create) | `POST /v1/bank_billet_payments` | Efetuar Pagamento de Boleto |
| [`bank-billet-payments get`](#bank-billet-payments-get) | `GET /v1/bank_billet_payments/{id}` | Informações do Pagamento de Boleto |
| [`bank-billet-payments delete`](#bank-billet-payments-delete) | `DELETE /v1/bank_billet_payments/{id}` | Excluir Pagamento de Boleto |

## bank-billet-payments list

Listar Pagamentos de Boleto

```text
kobana banking bank-billet-payments list [OPÇÕES]
```

`GET /v1/bank_billet_payments`

Listar Pagamentos de Boleto.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `paid_from` | `string` | não | A partir da Data de pagamento. |
| `paid_to` | `string` | não | Até a Data de pagamento. |
| `bank_billet_id` | `string` | não | ID do Boleto. |
| `bank_billet_account_id` | `string` | não | ID da Carteira. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem dos pagamentos de boletos |

## bank-billet-payments create

Efetuar Pagamento de Boleto

```text
kobana banking bank-billet-payments create [OPÇÕES]
```

`POST /v1/bank_billet_payments`

Efetuar Pagamento de Boleto.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `bank_billet_id` | `integer` | sim | ID do Boleto. |
| `bank_billet_account_id` | `integer` | sim | ID da Carteira. |
| `paid_amount` | `number` | sim | Valor Pago |
| `paid_at` | `string (date)` | sim | Data do Recebimento. |
| `kind` | `integer` | sim | Tipo de Recebimento: * `0` Direto ao beneficíario/Em mãos * `1` Boleto (Confirmado pelo banco via arquivo de retorno ou API) * `2` Pix (Confirmado pelo banco via Pix) Valores: `0`, `1`, `2`. |
| `id` | `integer` | não | ID do Recebimento de Boleto |
| `discharge_id` | `integer` | não | ID do Retorno. |
| `amount` | `number` | não | Valor do Boleto |
| `paid_bank` | `string` | não | Banco de Recebimento. |
| `paid_agency` | `string` | não | Agência de Recebimento (com dígito quando disponível). |
| `paid_agency_number` | `string` | não | Número da Agência de Recebimento (sem dígito). |
| `paid_agency_digit` | `string` | não | Dígito da Agência de Recebimento. |
| `bank_rate` | `number` | não | Taxa bancária |
| `credit_at` | `string (date)` | não | Data de crédito. |
| `direct` | `boolean` | não | Recebimento direto ao beneficíario |
| `expire_at` | `string (date)` | não | Data de vencimento. |
| `document_number` | `string` | não | Número do Documento. |
| `our_number` | `string` | não | Nosso Número. |
| `created_via_api` | `boolean` | não | Enviado pela API |
| `created_at` | `string (date-time)` | não | Data e hora de criação. |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização. |
| `customer_person_name` | `string` | não | Nome do cliente |
| `paid_by_pix` | `boolean` | não | Recebimento via Pix |
| `financial_provider_external_id` | `string` | não | ID do recebimento na instituição financeira |
| `line_parsed` | `string (json)` | não | Linha parseada com informações vindas do banco: Conteúdo do retorno bancário. Pode mudar de acordo com o banco e o formato 240 ou 400. Se algum dos campos estiver em branco é porque o seu banco não disponibiliza essa informação. \| Valor \| Descrição \| \|---\|---\| \| paid_at \| Data de recebimento \| \| paid_amount \| Valor pago \| \| bank_rate \| Taxa do boleto paga ao banco \| \| credit_at \| Data de crédito \| \| credit_amount \| Valor do Crédito \| \| our_number \| Nosso número \| \| processed_our_number_raw \| Nosso número formatado \| \| discount \| Desconto \| \| billet_fine \| Acréscimos(Multa/Juros cobrados) \| \| other_fines \| Outras cobranças adicionais \| \| other_credit \| Outros valores pagos \| \| value_rebate \| Outras taxas \| \| error_code \| Código de erro \| \| event_type \| Ocorrência bancária \| \| agency_number \| Agência \| \| account_number \| Conta \| \| banco_recebedor \| Banco recebedor \| \| agencia_recebedora \| Agência recebedora \| \| control_number \| Número de Controle \| \| document_number \| Número do Documento \| |

Outros campos e objetos aninhados: `kobana schema banking.bank-billet-payments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pagamento efetuado com sucesso |
| `422` | Dados inválidos |

## bank-billet-payments get

Informações do Pagamento de Boleto

```text
kobana banking bank-billet-payments get [ID] [OPÇÕES]
```

`GET /v1/bank_billet_payments/{id}`

Consulta informações específicas de um Pagamento de Boleto.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento de boleto encontrado |
| `404` | Pagamento de boleto não encontrado |

## bank-billet-payments delete

Excluir Pagamento de Boleto

```text
kobana banking bank-billet-payments delete [ID] [OPÇÕES]
```

`DELETE /v1/bank_billet_payments/{id}`

Excluir Pagamento de Boleto.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Pagamento de Boleto excluído |
| `404` | Pagamento de Boleto não encontrado |
