---
id: data
title: "kobana banking data"
sidebar_label: "data"
description: "Consultas (boletos, QR codes Pix)"
---

# kobana banking data

Consultas (boletos, QR codes Pix)

Comandos de `kobana banking data`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`data bank-billet-queries list`](#data-bank-billet-queries-list) | `GET /v2/data/bank_billet_queries` | Listar Consultas de Boletos |
| [`data bank-billet-queries create`](#data-bank-billet-queries-create) | `POST /v2/data/bank_billet_queries` | Criar uma Consulta de Boleto |
| [`data people list`](#data-people-list) | `GET /v2/data/people` | Buscar Pessoas |
| [`data people create`](#data-people-create) | `POST /v2/data/people` | Criar uma Pessoa |
| [`data people get`](#data-people-get) | `GET /v2/data/people/{uid}` | Visualizar uma Pessoa |
| [`data people delete`](#data-people-delete) | `DELETE /v2/data/people/{uid}` | Excluir uma Pessoa |
| [`data pix-qrcode-queries list`](#data-pix-qrcode-queries-list) | `GET /v2/data/pix_qrcode_queries` | Listar Consultas de Pix QR Code |
| [`data pix-qrcode-queries create`](#data-pix-qrcode-queries-create) | `POST /v2/data/pix_qrcode_queries` | Criar uma Consulta de Pix QR Code |

## data bank-billet-queries list

Listar Consultas de Boletos

```text
kobana banking data bank-billet-queries list [OPÇÕES]
```

`GET /v2/data/bank_billet_queries`

Lista todas as consultas de boletos realizadas pela conta autenticada. Permite filtrar por status, tags, datas de criação e outros parâmetros.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de consultas de boletos |

## data bank-billet-queries create

Criar uma Consulta de Boleto

```text
kobana banking data bank-billet-queries create [OPÇÕES]
```

`POST /v2/data/bank_billet_queries`

Cria uma nova consulta de boleto bancário através da linha digitável ou código de barras. A consulta é processada automaticamente e retorna informações detalhadas do boleto.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `uid` | `string` | não | ID único do registro no formato UUID |
| `status` | `string` | não | Situação da consulta: * `pending` Pendente (somente quando configurado para não processar automaticamente) * `success` Realizada com sucesso * `error` Falha na consulta Valores: `pending`, `success`, `error`. |
| `error_code` | `string` | não | Código do erro quando `status` é `error`: * `bank_billet_unavailable` Boleto indisponível: pode já ter sido pago, cancelado ou baixado * `document_not_supported` O provedor não confirmou o documento para pagamento: o convênio ou a modalidade pode não ser suportado * `provider_error` Falha do provedor ao consultar o boleto * `unavailable_environment` Consulta disponível apenas em produção Valores: `null`, `bank_billet_unavailable`, `document_not_supported`, `provider_error`, `unavailable_environment`. |
| `error_message` | `string` | não | Mensagem de erro quando a consulta é processada mas os dados não são retornados. |
| `line_or_barcode` | `string` | não | Linha Digitável ou Código de Barras, nos formatos: Linha Digitável: 34191.79001 01043.510047 91020.150008 7 75870000001000 Código de Barras: 34197758700000010001790010104351004791020150 |
| `line` | `string` | não | Linha Digitável |
| `barcode` | `string` | não | Código de Barras |
| `expire_at` | `string (date)` | não | Data de Vencimento |
| `pay_due_date` | `string (date)` | não | Data-limite para pagamento (prazo de tolerância), quando informada pela origem |
| `amount` | `number` | não | Valor a pagar |
| `minimum_amount` | `number` | não | Valor mínimo |
| `maximum_amount` | `number` | não | Valor máximo |
| `original_amount` | `number` | não | Valor original |
| `valid_for_payment` | `boolean` | não | Indica se o boleto está apto para pagamento no momento da consulta |
| `payment_status_reason` | `string` | não | Motivo retornado quando o boleto não está apto para pagamento |
| `allow_change_value` | `boolean` | não | Indica se o valor do pagamento pode ser alterado dentro dos limites retornados |
| `allow_partial_payment` | `boolean` | não | Indica se a origem permite pagamento parcial |
| `issue_bank` | `object` | não | Banco emissor do boleto |
| `beneficiary` | `object` | não | Dados do Beneficiário |
| `guarantor` | `object` | não | Dados do Beneficiário Final |
| `guarantor_drawer` | `object` | não | Dados do Sacador Avalista |
| `payer` | `object` | não | Dados do Pagador |
| `fine` | `object` | não | Multa. Retornado como `{"value": <reais>}`, com o valor em reais (decimal), não em centavos. |
| `interest` | `object` | não | Juros. Retornado como `{"value": <reais>}`, com o valor em reais (decimal), não em centavos. |
| `discount` | `object` | não | Desconto. Retornado como `{"value": <reais>}`, com o valor em reais (decimal), não em centavos. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |
| `created_via_api` | `boolean` | não | Indica se o registro foi criado por API |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.data.bank-billet-queries.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Consulta de Boleto criada |
| `422` | Consulta de Boleto inválida |

## data people list

Buscar Pessoas

```text
kobana banking data people list [OPÇÕES]
```

`GET /v2/data/people`

Lista todas as pessoas cadastradas na conta autenticada. Permite filtrar por código externo, tipo de pessoa, documento, e-mail, tags e período de criação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `external_code` | `string` | não | Filtro por Código Externo. Acha todos os registros cujo campo seja igual ao string. |
| `kind` | `string` | não | Filtro por Tipo de Pessoa (Natural ou Jurídica). |
| `document_type` | `string` | não | Filtro por Tipo do Documento (CPF ou CNPJ). |
| `document_number` | `string` | não | Filtro por Número do Documento. Acha todos os registros cujo campo seja igual ao string. |
| `tags` | `array` | não | Tags associadas ao registro. |
| `created_from` | `string` | não | Data inicial do filtro de data de criação do registro. |
| `created_to` | `string` | não | Data final do filtro de data de criação do registro. |
| `per_page` | `integer` | não | Quantidade de registros por página. Máximo de 50. |
| `page` | `integer` | não | Número da página. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registros encontrados (página 2 com prev_url) |

## data people create

Criar uma Pessoa

```text
kobana banking data people create [OPÇÕES]
```

`POST /v2/data/people`

Cria uma nova pessoa (física ou jurídica) com as informações fornecidas. A pessoa será associada à conta autenticada.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome Completo ou Razão Social |
| `uid` | `string (uuid)` | não | Identificador único do registro |
| `nickname` | `string` | não | Apelido ou Nome Fantasia |
| `kind` | `string` | não | Tipo de Pessoa |
| `external_code` | `string` | não | Código ou ID no seu sistema de gestão. |
| `birthday` | `string (date)` | não | Data de Nascimento. |
| `document_type` | `string` | não | Tipo do Documento. Valores válidos: CPF, CNPJ |
| `document_number` | `string` | não | Número do Documento. |
| `addresses` | `array<object>` | não | Lista de Endereços |
| `phones` | `array<object>` | não | Lista de Telefones |
| `emails` | `array<object>` | não | Lista de Emails |
| `tags` | `array<string>` | não | Tags associadas ao registro |
| `tag_list` | `string` | não | Lista de tags separadas por vírgula |
| `custom_data` | `object` | não | Disponível para envio de um JSON com uma estrutura de dados livre |
| `notes` | `string` | não | Observações. Campo de texto livre |
| `created_via_api` | `boolean` | não | Indica se o registro foi criado pela API |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro no banco de dados |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro no banco de dados |

Outros campos e objetos aninhados: `kobana schema banking.data.people.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pessoa criada |
| `422` | Erro de validação |

## data people get

Visualizar uma Pessoa

```text
kobana banking data people get [UID] [OPÇÕES]
```

`GET /v2/data/people/{uid}`

Retorna informações detalhadas de uma pessoa específica, incluindo dados pessoais, endereços, telefones, e-mails e dados personalizados.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pessoa encontrada |
| `404` | Pessoa não encontrada |

## data people delete

Excluir uma Pessoa

```text
kobana banking data people delete [UID] [OPÇÕES]
```

`DELETE /v2/data/people/{uid}`

Exclui permanentemente uma pessoa do sistema. Esta ação não pode ser desfeita.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Registro excluído |
| `404` | Registro não encontrado |

## data pix-qrcode-queries list

Listar Consultas de Pix QR Code

```text
kobana banking data pix-qrcode-queries list [OPÇÕES]
```

`GET /v2/data/pix_qrcode_queries`

Lista todas as consultas de Pix QR Code realizadas pela conta autenticada. Permite filtrar por tags, external_id e outros parâmetros.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `external_id` | `string` | não | Filtrar por ID externo |
| `tags` | `array` | não | Filtrar por tags |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de consultas de Pix QR Code |

## data pix-qrcode-queries create

Criar uma Consulta de Pix QR Code

```text
kobana banking data pix-qrcode-queries create [OPÇÕES]
```

`POST /v2/data/pix_qrcode_queries`

Cria uma nova consulta de Pix QR Code através do qrcode. A consulta é processada automaticamente e retorna informações detalhadas do QR Code Pix.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `uid` | `string` | não | ID único do registro no formato UUID |
| `qrcode` | `string` | não | QR Code Pix para decodificação |
| `pix_key` | `string` | não | Chave Pix do beneficiário |
| `expires_at` | `string (date-time)` | não | Data de expiração do QR Code |
| `finality` | `string` | não | Finalidade do pagamento (pagamento, saque, troco) |
| `amount` | `number` | não | Valor principal |
| `amount_change` | `number` | não | Valor do troco |
| `interest` | `number` | não | Valor dos juros |
| `fine` | `number` | não | Valor da multa |
| `discount` | `number` | não | Valor do desconto |
| `total_amount` | `number` | não | Valor total a pagar |
| `beneficiary` | `object` | não | Dados do Beneficiário/Recebedor |
| `debtor` | `object` | não | Dados do Devedor |
| `description` | `string` | não | Mensagem ao pagador |
| `can_be_paid` | `boolean` | não | Indica se o QR Code pode ser pago |
| `can_be_paid_with_different_value` | `boolean` | não | Indica se pode ser pago com valor diferente |
| `cannot_be_paid_reason` | `string` | não | Motivo pelo qual não pode ser pago |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas para categorização |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.data.pix-qrcode-queries.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Consulta de Pix QR Code criada |
| `422` | Consulta de Pix QR Code inválida |
