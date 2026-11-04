---
id: bank-billet-accounts
title: "kobana banking bank-billet-accounts"
sidebar_label: "bank-billet-accounts"
---

# kobana banking bank-billet-accounts


Comandos de `kobana banking bank-billet-accounts`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`bank-billet-accounts list`](#bank-billet-accounts-list) | `GET /v1/bank_billet_accounts` | Listar Carteiras |
| [`bank-billet-accounts create`](#bank-billet-accounts-create) | `POST /v1/bank_billet_accounts` | Criar Carteira de Cobrança |
| [`bank-billet-accounts get`](#bank-billet-accounts-get) | `GET /v1/bank_billet_accounts/{id}` | Informações da Carteira |
| [`bank-billet-accounts update`](#bank-billet-accounts-update) | `PUT /v1/bank_billet_accounts/{id}` | Atualizar a Carteira |
| [`bank-billet-accounts delete`](#bank-billet-accounts-delete) | `DELETE /v1/bank_billet_accounts/{id}` | Excluir a Carteira de Cobrança |
| [`bank-billet-accounts ask`](#bank-billet-accounts-ask) | `GET /v1/bank_billet_accounts/{id}/ask` | Solicitar Homologação da Carteira de Cobrança |
| [`bank-billet-accounts set-default`](#bank-billet-accounts-set-default) | `PUT /v1/bank_billet_accounts/{id}/set_default` | Alterar Carteira de Cobrança padrão |
| [`bank-billet-accounts validate`](#bank-billet-accounts-validate) | `PUT /v1/bank_billet_accounts/{id}/validate` | Validar Carteira de Cobrança |

## bank-billet-accounts list

Listar Carteiras

```text
kobana banking bank-billet-accounts list [OPÇÕES]
```

`GET /v1/bank_billet_accounts`

Lista Carteiras.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Carteiras Encontradas |

## bank-billet-accounts create

Criar Carteira de Cobrança

```text
kobana banking bank-billet-accounts create [OPÇÕES]
```

`POST /v1/bank_billet_accounts`

Cria uma nova carteira de cobrança para emissão de boletos bancários. A carteira precisa ser homologada antes de poder ser utilizada em produção.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `bank_contract_slug` | `string` | sim | Slug da Carteira |
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira. Quando informado, os dados de agência e conta serão preenchidos automaticamente a partir da conta financeira. |
| `beneficiary_name` | `string` | sim | Nome do Beneficiário |
| `beneficiary_cnpj_cpf` | `string` | sim | CNPJ/CPF do Beneficiário |
| `beneficiary_address_street` | `string` | sim | Rua do Beneficiário |
| `beneficiary_address_street_number` | `string` | sim | Numero da rua do Beneficiário |
| `beneficiary_address_complement` | `string` | sim | Complemento do endereço do Beneficiário |
| `beneficiary_address_neighborhood` | `string` | sim | Bairro do Beneficiário |
| `beneficiary_address_city` | `string` | sim | Cidade do Beneficiário |
| `beneficiary_address_state` | `string` | sim | Estado do Beneficiário |
| `beneficiary_address_zipcode` | `string` | sim | CEP do Beneficiário (formato 99999999) |
| `beneficiary_address` | `string` | sim | Endereço completo do Beneficiário. |
| `id` | `integer` | não | ID da carteira |
| `next_our_number` | `integer` | não | Próximo Nosso Número. Default: 1 |
| `next_remittance_number` | `integer` | não | Último sequencial da remessa Default: 1 |
| `agency_number` | `string` | não | Agência. Preenchido automaticamente quando financial_account_uid é fornecido. |
| `agency_digit` | `string` | não | Dígito da Agência. Preenchido automaticamente quando financial_account_uid é fornecido. |
| `account_number` | `string` | não | Conta. Preenchida automaticamente quando financial_account_uid é fornecido. |
| `account_digit` | `string` | não | Dígito da Conta. Preenchido automaticamente quando financial_account_uid é fornecido. |
| `extra1` | `string` | não | Campo extra 1. Dependendo da carteira escolhida poderá ser obrigátorio. |
| `extra1_digit` | `string` | não | Digito do Campo extra 1. Dependendo da carteira escolhida poderá ser obrigátorio. |
| `extra2` | `string` | não | Campo extra 2. Dependendo da carteira escolhida poderá ser obrigátorio. |
| `extra2_digit` | `string` | não | Dígito do Campo extra 2. Dependendo da carteira escolhida poderá ser obrigátorio. |
| `extra3` | `string` | não | Código de Remessa |
| `payment_place` | `string` | não | Local de Pagamento |
| `contract` | `string` | não | Contrato |
| `contract_type` | `string` | não | Tipo do Contrato |
| `financial_account` | `object` | não | Conta Financeira associada à carteira |
| `bolepix_enabled_at` | `string` | não | Data e Hora em que o boleto híbrido foi habilitado. |
| `bolepix` | `object` | não | Configuração de Boleto Híbrido |
| `name` | `string` | não | Nome da Conta. Não é recebido na criação e nem na atualização, só é retornado na consulta e listagem |
| `status` | `string` | não | Situação da carteira: Quando a carteira acaba de ser cadastrada, ela ganha o status `pending`. Nesse momento o usuário deve aceitar os termos e iniciar a homologação. Ao clicar em “Prosseguir com a Homologação”, o status passa para `homologating`. Será gerado um boleto e uma remessa de Homologação para ser enviada ao banco. Após o término, o status passa para `validating`. A partir desse momento o usuário precisa informar o valor do boleto que foi gerado ou enviar o retorno processado pelo banco. Quando o valor é informado corretamente, ou no retorno processado, o banco diz estar tudo certo, o status passa para `active`. * `pending` Homologação não iniciada * `homologating` Em homologação, aguardando pagamento do boleto * `validating` Boleto pago, aguardando validação * `active` Ativa e pronta para uso Valores: `pending`, `homologating`, `validating`, `active`. |
| `default` | `boolean` | não | Padrão. O campo `default` determina a carteira de cobrança que será usada na criação do boleto quando nenhuma carteira for informada. No momento que a primeira carteira é homologada (passa para o `status` = `active`), ela recebe o valor `default` = `true` |
| `configuration` | `object` | não | Configuração de dados padrões para boleto |
| `bank_contract` | `object` | não | Dados da Carteira. Não é recebido na criação e nem na atualização, só é retornado na consulta e listagem. |
| `custom_name` | `string` | não | Nome da Carteira para identificação dentro da Kobana. |
| `kind` | `string` | não | Tipo de CNAB: * `cnab400` CNAB 400 * `cnab240` CNAB 240 Valores: `cnab400`, `cnab240`. |
| `remittance_disabled` | `boolean` | não | Desabilita geração de remessas. |
| `remittance_disabled_at` | `string (date-time)` | não | Data e hora em que a geração de remessa foi desabilitada. |
| `allow_expiration_on_weekends` | `boolean` | não | Permitir vencimento em fim de semana e feriados |
| `created_at` | `string (date-time)` | não | Data e hora de criação da carteira |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização da carteira |
| `created_via_api` | `boolean` | não | Indica se a carteira foi criada por API |

Outros campos e objetos aninhados: `kobana schema banking.bank-billet-accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Carteira de cobrança criada com financial_account_uid |
| `422` | Carteira inválida - Beneficiário CPF |

## bank-billet-accounts get

Informações da Carteira

```text
kobana banking bank-billet-accounts get [ID] [OPÇÕES]
```

`GET /v1/bank_billet_accounts/{id}`

Consulta informações específicas de uma Carteira

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Carteira encontrada |
| `404` | Carteira não encontrado |

## bank-billet-accounts update

Atualizar a Carteira

```text
kobana banking bank-billet-accounts update [ID] [OPÇÕES]
```

`PUT /v1/bank_billet_accounts/{id}`

Atualiza informações específicas de uma Carteira.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `kind` | `string` | não | Tipo de CNAB: * `cnab400` CNAB 400 * `cnab240` CNAB 240 Valores: `cnab400`, `cnab240`. |
| `bank_contract_slug` | `string` | não | Slug da Carteira. Carteiras ativas não poderão ter o campo bank_contract_slug alterados. |
| `financial_account` | `object` | não | Conta Financeira associada à carteira |
| `payment_place` | `string` | não | Local de Pagamento |
| `custom_name` | `string` | não | Nome da Carteira para identificação dentro da Kobana. |
| `allow_expiration_on_weekends` | `boolean` | não | Permitir vencimento em fim de semana e feriados |
| `beneficiary_name` | `string` | não | Nome do Beneficiário |
| `beneficiary_cnpj_cpf` | `string` | não | CNPJ/CPF do Beneficiário |
| `agency_number` | `string` | não | Agência. Preenchido automaticamente quando financial_account_uid é fornecido. |
| `agency_digit` | `string` | não | Dígito da Agência. Opcional: será calculado automaticamente usando Módulo 11 quando o banco exigir e não for fornecido. Preenchido automaticamente quando financial_account_uid é fornecido. |
| `account_number` | `string` | não | Conta. Preenchida automaticamente quando financial_account_uid é fornecido. |
| `account_digit` | `string` | não | Dígito da Conta. Preenchido automaticamente quando financial_account_uid é fornecido. |
| `extra1` | `string` | não | Campo extra 1. |
| `extra1_digit` | `string` | não | Digito do Campo extra 1. |
| `extra2` | `string` | não | Campo extra 2. |
| `extra2_digit` | `string` | não | Dígito do Campo extra 2. |
| `contract` | `string` | não | Contrato |
| `contract_type` | `string` | não | Tipo do Contrato |
| `extra3` | `string` | não | Código de Remessa |
| `beneficiary_address_street` | `string` | não | Rua do Beneficiário |
| `beneficiary_address_street_number` | `string` | não | Numero da rua do Beneficiário |
| `beneficiary_address_complement` | `string` | não | Complemento do endereço do Beneficiário |
| `beneficiary_address_neighborhood` | `string` | não | Bairro do Beneficiário |
| `beneficiary_address_city` | `string` | não | Cidade do Beneficiário |
| `beneficiary_address_state` | `string` | não | Estado do Beneficiário |
| `beneficiary_address_zipcode` | `string` | não | CEP do Beneficiário (formato 99999999) |
| `beneficiary_address` | `string` | não | Endereço completo do Beneficiário. |
| `bolepix_enabled` | `boolean` | não | Habilitar Bolepix (legado — prefira o objeto `bolepix`) |
| `bolepix` | `object` | não | Configuração de Boleto Híbrido |
| `bolepix_key` | `string` | não | Chave Pix que será usada no Bolepix |
| `default` | `boolean` | não | Padrão. O campo `default` determina a carteira de cobrança que será usada na criação do boleto quando nenhuma carteira for informada. No momento que a primeira carteira é homologada (passa para o `status` = `active`), ela recebe o valor `default` = `true` |
| `remittance_disabled` | `boolean` | não | Desabilita geração de remessas. |
| `next_our_number` | `integer` | não | Último nosso número |
| `next_remittance_number` | `integer` | não | Último sequencial da remessa Default: 1 |
| `bank_billet_account_next_our_number_attributes` | `object` | não |  |
| `bank_billet_account_next_remittance_number_attributes` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema banking.bank-billet-accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Carteira atualizada |
| `404` | Carteira não encontrado |
| `422` | Unprocessable Entity |

## bank-billet-accounts delete

Excluir a Carteira de Cobrança

```text
kobana banking bank-billet-accounts delete [ID] [OPÇÕES]
```

`DELETE /v1/bank_billet_accounts/{id}`

Exclui uma Carteira de Cobrança. Opcionalmente, exclui também a Conta Financeira associada.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `delete_financial_account` | `boolean` | não | Quando `true`, exclui também a Conta Financeira associada à Carteira. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Carteira e Conta Financeira excluídas (delete_financial_account=true) |
| `204` | Carteira enfileirada para exclusão (delete_financial_account=false) |
| `404` | Carteira não encontrada |

## bank-billet-accounts ask

Solicitar Homologação da Carteira de Cobrança

```text
kobana banking bank-billet-accounts ask [ID] [OPÇÕES]
```

`GET /v1/bank_billet_accounts/{id}/ask`

Solicitar homologação da Carteira de Cobrança

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Solicitação de homologação realizada com sucesso |
| `404` | Carteira não encontrada |
| `422` | Carteira já homologada |

## bank-billet-accounts set-default

Alterar Carteira de Cobrança padrão

```text
kobana banking bank-billet-accounts set-default [ID] [OPÇÕES]
```

`PUT /v1/bank_billet_accounts/{id}/set_default`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `default` | `boolean` | sim | Define a Carteira padrão(Default: true) ou deixar de ser padrão(Default: false) |

Outros campos e objetos aninhados: `kobana schema banking.bank-billet-accounts.set-default`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Carteira definida com padrão com sucesso |
| `404` | Carteira não encontrada |
| `422` | Unprocessable Entity |

## bank-billet-accounts validate

Validar Carteira de Cobrança

```text
kobana banking bank-billet-accounts validate [ID] [OPÇÕES]
```

`PUT /v1/bank_billet_accounts/{id}/validate`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `homologation_amount` | `string` | sim | Valor recebido pelo boleto (R$) Ex.: 1,87 |

Outros campos e objetos aninhados: `kobana schema banking.bank-billet-accounts.validate`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Carteira Homologada com sucesso |
| `404` | Carteira não encontrada |
| `422` | Unprocessable Entity |
