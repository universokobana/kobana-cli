---
id: payments
title: "kobana banking payments"
sidebar_label: "payments"
description: "Pagamentos (unificado)"
---

# kobana banking payments

Pagamentos (unificado)

Comandos de `kobana banking payments`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`payments list`](#payments-list) | `GET /v2/payments` | Listar Pagamentos |
| [`payments create`](#payments-create) | `POST /v2/payments` | Criar um Pagamento |
| [`payments barcode`](#payments-barcode) | `POST /v2/payments/barcode` | Criar um Pagamento por código de barras |
| [`payments get`](#payments-get) | `GET /v2/payments/{uid}` | Visualizar um Pagamento |
| [`payments approve`](#payments-approve) | `PUT /v2/payments/{uid}/approve` | Liberar (aprovar) um Pagamento |
| [`payments cancel`](#payments-cancel) | `PUT /v2/payments/{uid}/cancel` | Cancelar um Pagamento |

## payments list

Listar Pagamentos

```text
kobana banking payments list [OPÇÕES]
```

`GET /v2/payments`

Lista todos os pagamentos de qualquer tipo (PIX, Boleto, DARF, Tributos, Contas). Suporta filtros e paginação. Este é um endpoint unificado que retorna payments de todos os tipos através de uma única consulta.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `kind` | `string` | não | Filtrar por tipo de pagamento Valores: `bank_billet`, `darf`, `utility`, `gru`, `pix`, `tax`. |
| `status` | `string` | não | Filtrar por status Valores: `pending`, `scheduled`, `confirmed`, `rejected`, `canceled`. |
| `created_from` | `string` | não | Filtrar por data de criação inicial (formato: YYYY-MM-DD, inclusivo) |
| `created_to` | `string` | não | Filtrar por data de criação final (formato: YYYY-MM-DD, inclui o dia inteiro) |
| `updated_from` | `string` | não | Filtrar por data de atualização inicial (formato: YYYY-MM-DD, inclusivo) |
| `updated_to` | `string` | não | Filtrar por data de atualização final (formato: YYYY-MM-DD, inclui o dia inteiro) |
| `scheduled_from` | `string` | não | Filtrar por data de agendamento inicial (formato: YYYY-MM-DD) |
| `scheduled_to` | `string` | não | Filtrar por data de agendamento final (formato: YYYY-MM-DD) |
| `scheduled` | `boolean` | não | Filtrar apenas pagamentos agendados (true) ou não agendados (false) |
| `identifier` | `string` | não | Buscar por identificador (busca parcial) |
| `external_id` | `string` | não | Filtrar por ID externo |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamentos encontrados |

## payments create

Criar um Pagamento

```text
kobana banking payments create [OPÇÕES]
```

`POST /v2/payments`

Endpoint genérico para criação de pagamentos de qualquer tipo. O tipo deve ser especificado no campo `kind`.

**Pré-requisitos**

Para emitir um pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento que registra a operação é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Tipos Suportados**

**PIX**
Usar `kind: "pix"` e fornecer a chave `pix`:
```json
{
  "kind": "pix",
  "financial_account_uid": "...",
  "amount": 100.0,
  "pix": {
    "qrcode": "00020126330014br.gov.bcb.pix..."
  }
}
```

**Boleto Bancário**
Usar `kind: "bank_billet"` e fornecer a chave `bank_billet`:
```json
{
  "kind": "bank_billet",
  "financial_account_uid": "...",
  "amount": 200.0,
  "bank_billet": {
    "code": "81770000001-8 00005659970-7 41131079703-9 0014334-8"
  }
}
```

**Tributos (Tax)**
Usar `kind: "tax"` e fornecer a chave `tax` com código de barras:
```json
{
  "kind": "tax",
  "financial_account_uid": "...",
  "amount": 400.0,
  "tax": {
    "kind": "iptu",
    "code": "81770000001-8 00005659970-7..."
  }
}
```

DARF com campos estruturados usa `tax.kind: "darf"` e `tax.format: "data"`:
```json
{
  "kind": "tax",
  "financial_account_uid": "...",
  "amount": 300.0,
  "tax": {
    "kind": "darf",
    "format": "data",
    "data": {
      "tax_code": "0107",
      "taxpayer_type": "cnpj",
      "taxpayer_number": "16.974.923/0001-84",
      "taxpayer_name": "Empresa Exemplo LTDA",
      "calculation_date": "2024-12-31",
      "expire_at": "2025-01-10"
    }
  }
}
```

**Contas de Consumo (Utility)**
Usar `kind: "utility"` e fornecer a chave `utility`:
```json
{
  "kind": "utility",
  "financial_account_uid": "...",
  "amount": 500.0,
  "utility": {
    "code": "81770000001-8 00005659970-7..."
  }
}
```

**Eventos de Webhook**

O evento `payment.db.created` é disparado nesta operação.

> **Endpoint Unificado:** Este endpoint unifica a criação de todos os tipos de pagamento. Para criar pagamentos de um tipo específico, você também pode usar os endpoints dedicados em `/api/v2/payment/{type}`.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.payments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pagamento Conta de Consumo criado |
| `422` | Tributo com format=code e código ausente |

## payments barcode

Criar um Pagamento por código de barras

```text
kobana banking payments barcode [OPÇÕES]
```

`POST /v2/payments/barcode`

Cria um pagamento a partir só do código de barras ou da linha digitável. O tipo do pagamento é resolvido pelo próprio código, seguindo o layout FEBRABAN v08, e devolvido no campo `kind` da resposta:

| Código | `kind` |
|---|---|
| 47 posições, ou 44 que não começam com `8` | `bank_billet` |
| começa com `8`, segmento `2`, `3`, `4`, `6` ou `7` | `utility` |
| começa com `8`, segmento `1` ou `5` | `tax`, com `tax.kind = "unknown"` e `tax.format = "code"` |

O código é recusado com `422` quando um dígito verificador não confere (o geral, no boleto e na arrecadação, e os de campo ou de bloco da linha digitável), quando usa o segmento `9`, `0` ou `8`, quando o tamanho não combina com o primeiro dígito (47 posições começando com `8`, 48 que não começam com `8`), ou quando é guia de FGTS (segmento `5` com convênio `0181` ou `0182` nas posições 16 a 19).

**Pré-requisitos**

`payment_account_uid` é obrigatório e define a conta financeira de origem. `financial_account_uid` não é aceito neste endpoint.

`amount` é opcional quando o código traz o valor a pagar: arrecadação com identificador de valor `6` ou `8` (3ª posição) e boleto com valor diferente de zero. Sem `amount`, o pagamento usa o valor do código; no boleto, é o valor nominal, sem juros, multa ou desconto. É obrigatório na arrecadação com identificador `7` ou `9`, em que o valor do código é só de referência, e quando o código vem com valor zerado.

**Eventos de Webhook**

O evento `payment.db.created` é disparado nesta operação.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `code` | `string` | sim | Código de barras (44 posições) ou linha digitável (47 ou 48 posições). Espaço, ponto, hífen e barra são descartados. |
| `payment_account_uid` | `string (uuid)` | sim | UID da Conta de Pagamento de origem. Obrigatório: não há escolha automática. |
| `amount` | `number` | não | Quantia. Opcional quando o código traz o valor a pagar (arrecadação com identificador de valor `6` ou `8`, ou boleto com valor diferente de zero): sem ela, vale o valor do código, que no boleto é o nominal, sem juros, multa ou desconto. Obrigatória na arrecadação com identificador `7` ou `9` e quando o código vem com valor zerado. Quando enviada, sempre prevalece sobre o valor do código: para pagar boleto vencido com encargos, envie a quantia com eles. |
| `scheduled_to` | `string (date)` | não | Data de Agendamento |
| `beneficiary` | `object` | não | Informações do beneficiário. Obrigatório quando o código for de boleto e a conta financeira for BB. |
| `identifier` | `string` | não | Identificação do pagamento no comprovante do provedor financeiro. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |

Outros campos e objetos aninhados: `kobana schema banking.payments.barcode`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pagamento de boleto criado |
| `422` | Código com valor de referência, sem amount |

## payments get

Visualizar um Pagamento

```text
kobana banking payments get [UID] [OPÇÕES]
```

`GET /v2/payments/{uid}`

Consulta informações específicas de um Pagamento de qualquer tipo. O tipo é identificado automaticamente e a resposta é serializada adequadamente.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Tributo GRU com format=data |
| `404` | Pagamento não encontrado |

## payments approve

Liberar (aprovar) um Pagamento

```text
kobana banking payments approve [UID] [OPÇÕES]
```

`PUT /v2/payments/{uid}/approve`

> **Processamento Assíncrono:** cria um comando de liberação (approve) para um Pagamento registrado que está aguardando aprovação.

A liberação é feita assincronamente: ao receber a requisição, criamos um Comando que é processado em segundo plano. Você recebe o ID do comando criado na resposta e pode consultá-lo a qualquer momento.

> ⚠️ **Suporte por banco:** nem todos os provedores expõem liberação individual de pagamento via API. Quando o provedor não suporta a operação, a resposta é `422`.

> ℹ️ **Pagamento de lote:** este endpoint libera um pagamento de um lote sem aprovar o lote inteiro (ver `PUT /api/v2/payment/batches/{uid}/approve`).

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando de liberação criado com sucesso |
| `422` | Pagamento não está em estado que permita liberação ou provedor não suporta |

## payments cancel

Cancelar um Pagamento

```text
kobana banking payments cancel [UID] [OPÇÕES]
```

`PUT /v2/payments/{uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para o Pagamento.
O cancelamento de um Pagamento é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, o Pagamento tem seu status alterado para Cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `payment.{type}.cancel.requested` é disparado (onde \{type\} pode ser pix, bank_billet, etc).
O nosso servidor então começa as tentativas de envio do cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então o pagamento é marcado como cancelado em nosso banco de dados.
Neste momento o evento `payment.{type}.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. O pagamento não poderá mais ser processado após o cancelamento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento cancelado sem interação com provedor |
| `422` | Pagamento já foi confirmado, rejeitado ou cancelado |
