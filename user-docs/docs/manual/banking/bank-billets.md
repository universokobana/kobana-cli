---
id: bank-billets
title: "kobana banking bank-billets"
sidebar_label: "bank-billets"
---

# kobana banking bank-billets


Comandos de `kobana banking bank-billets`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`bank-billets protest update`](#bank-billets-protest-update) | `PUT /v1/bank_billets/{id}/protest/{type}` | Protestar Boleto |
| [`bank-billets list`](#bank-billets-list) | `GET /v1/bank_billets` | Listar Boletos |
| [`bank-billets create`](#bank-billets-create) | `POST /v1/bank_billets` | Criar um Boleto |
| [`bank-billets cancel-all`](#bank-billets-cancel-all) | `POST /v1/bank_billets/cancel_all` | Cancelar Boletos em Lote |
| [`bank-billets get`](#bank-billets-get) | `GET /v1/bank_billets/{id}` | Visualizar o Boleto |
| [`bank-billets update`](#bank-billets-update) | `PUT /v1/bank_billets/{id}` | Atualizar o Boleto |
| [`bank-billets cancel`](#bank-billets-cancel) | `PUT /v1/bank_billets/{id}/cancel` | Cancelar o Boleto |
| [`bank-billets duplicate`](#bank-billets-duplicate) | `POST /v1/bank_billets/{id}/duplicate` | Duplicar Boleto |
| [`bank-billets pay`](#bank-billets-pay) | `PUT /v1/bank_billets/{id}/pay` | Marcar Boleto Como Pago |
| [`bank-billets send-email`](#bank-billets-send-email) | `POST /v1/bank_billets/{id}/send_email` | Enviar Boleto por E-mail |
| [`bank-billets send-sms`](#bank-billets-send-sms) | `POST /v1/bank_billets/{id}/send_sms` | Enviar Boleto por SMS |

## bank-billets protest update

Protestar Boleto

```text
kobana banking bank-billets protest update [ID] [TYPE] [OPÇÕES]
```

`PUT /v1/bank_billets/{id}/protest/{type}`

Protesta ou cancela protesto de um boleto vencido. O parâmetro :type na rota é obrigatório: use "send" para protestar o boleto ou "cancel" para cancelar um protesto existente.

**Argumentos do caminho:** `id`, `type`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Boleto protestado com sucesso |
| `404` | Boleto não encontrado |
| `422` | Tipo de protesto inválido |

## bank-billets list

Listar Boletos

```text
kobana banking bank-billets list [OPÇÕES]
```

`GET /v1/bank_billets`

Lista boletos.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `bank_billet_account_id` | `string` | não | Filtro por ID da Carteira. Acha todas os registros que tenham o ID. |
| `status` | `string` | não | Filtro por Situação. Acha todos os registros com o status. Valores: `generating`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `our_number` | `string` | não | Filtro por Nosso Número |
| `processed_our_number_raw` | `string` | não | Filtro por Nosso Número com DV (limpo). |
| `cnpj_cpf` | `string` | não | Filtro por CPF/CNPJ do cliente. Deve ser formatado com máscara. Acha todos os registros cujo campo seja igual ao string. |
| `created_from` | `string` | não | Data inicial do filtro de data de criação do registro. |
| `created_to` | `string` | não | Data final do filtro de data de criação do registro. |
| `expire_from` | `string` | não | Data inicial do filtro de data de vencimento |
| `expire_to` | `string` | não | Data final do filtro de data de vencimento. |
| `paid_from` | `string` | não | Data inicial do filtro de data de pagamento. |
| `paid_to` | `string` | não | Data final do filtro de data de pagamento. |
| `registered_to` | `string` | não | Data inicial do filtro de data de registro. |
| `registered_from` | `string` | não | Data final do filtro de data de registro. |
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `updated_from` | `string` | não | Data inicial do filtro de data de atualização do registro. |
| `updated_to` | `string` | não | Data final do filtro de data de atualização do registro. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de boletos |

## bank-billets create

Criar um Boleto

```text
kobana banking bank-billets create [OPÇÕES]
```

`POST /v1/bank_billets`

Cria um novo boleto bancário para cobrança. O boleto será processado e registrado na instituição financeira de acordo com as configurações da carteira de cobrança utilizada.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Quantia |
| `expire_at` | `string (date)` | sim | Data de vencimento |
| `customer_person_name` | `string` | sim | Nome do cliente |
| `customer_cnpj_cpf` | `string` | sim | CPF/CNPJ do cliente |
| `customer_state` | `string` | sim | Estado |
| `customer_city_name` | `string` | sim | Cidade(Nome deve estar correto e completo) |
| `customer_zipcode` | `string` | sim | CEP (formato 99999999) |
| `customer_address` | `string` | sim | Endereço |
| `customer_neighborhood` | `string` | sim | Bairro |
| `id` | `integer` | não | ID do boleto |
| `bank_billet_account_id` | `integer` | não | ID da Carteira de Cobrança. Se não informado, usará a carteira padrão. |
| `bank_billet_layout_id` | `integer` | não | ID do Modelo de Boleto |
| `customer_id` | `integer` | não | ID do Cliente. Quando esse ID é passado, os campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` e `customer_neighborhood` não são obrigatórios. |
| `customer_address_complement` | `string` | não | Complemento |
| `customer_address_number` | `string` | não | Número |
| `customer_email` | `string (email)` | não | E-mail do Pagador |
| `customer_email_cc` | `string (email)` | não | E-mail alternativo do Pagador |
| `customer_phone_number` | `string` | não | Telefone (com DDD, DDI é opcional) |
| `customer_ignore_email` | `boolean` | não | Nunca enviar e-mail para este cliente |
| `customer_ignore_sms` | `boolean` | não | Nunca enviar SMS para este cliente |
| `customer_mobile_local_code` | `string` | não | DDD do Celular |
| `customer_mobile_number` | `string` | não | Celular |
| `customer_nickname` | `string` | não | Apelido ou Nome Fantasia do Pagador |
| `customer_notes` | `string` | não | Observações do Pagador |
| `customer_contact_person` | `string` | não | Contato |
| `interest_type` | `integer` | não | Tipo de juros/mora: * `0` Inexistente (Padrão) * `1` Para porcentagem diária * `2` Para valor diário * `7` Para porcentagem mensal - Bancos suportados: Bradesco, BB, BTG, Caixa, Inter, Itaú, Safra, Santander, Sicoob e Sicredi Valores: `0`, `1`, `2`, `7`. |
| `days_for_interest` | `integer` | não | Quantidade de dias após o vencimento que a mora começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `interest_percentage` | `number` | não | Porcentagem de juros. De 0.0 a 100.0 (Ex 1.5% = 1.5) Obrigatório se interest_type é igual a 1 ou 7. Até 2 casas decimais. |
| `interest_value` | `number` | não | Valor diário de juros (R$). Obrigatório se interest_type é igual a 2. Até 2 casas decimais. |
| `interest_days_type` | `integer` | não | Tipo de Dias para juros: * `0` Corridos * `1` Úteis Valores: `0`, `1`. |
| `fine_type` | `integer` | não | Tipo de multa: * `0` Inexistente (Padrão) * `1` Para percentual do valor do boleto * `2` Para valor fixo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | não | Quantidade de dias após o vencimento que a multa começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `fine_percentage` | `number` | não | Porcentagem de Multa por Atraso Ex: 2% x R$ 250,00 = R$ 5,00. Obrigatória se fine_type é igual a 1. Até 2 casas decimais. |
| `fine_value` | `number` | não | Valor da multa (R$). Obrigatório se fine_type é igual a 2. Até 2 casas decimais. |
| `discount_type` | `integer` | não | Tipo de desconto: O tipo de desconto será o mesmo para todos os três descontos, caso existam. : * `0` Inexistente (Padrão) * `1` Para valor fixo * `2` Para percentual do valor do boleto Valores: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | não | Dias para desconto. Obrigatório se discount_type é diferente de 0(zero) |
| `discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao desconto. Obrigatório se discount_type é igual a 2 |
| `discount_value` | `number` | não | Valor do desconto (R$). Obrigatório se discount_type é igual a 1. |
| `days_for_second_discount` | `integer` | não | Dias para segundo desconto. |
| `second_discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao segundo desconto. |
| `second_discount_value` | `number` | não | Valor do segundo desconto (R$). |
| `days_for_third_discount` | `integer` | não | Dias para terceiro desconto. |
| `third_discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao terceiro desconto. |
| `third_discount_value` | `number` | não | Valor do terceiro desconto (R$). |
| `tags` | `array<string>` | não | Tags associadas ao boleto |
| `tag_list` | `string` | não | Tags associadas ao boleto |
| `charge_type` | `integer` | não | Tipo de Cobrança: * `1` Simples * `2` Vinculada * `3` Descontada * `4` Vendor Valores: `1`, `2`, `3`, `4`. |
| `dispatch_type` | `integer` | não | Tipo de Cobrança: Quando o boleto precisa ser enviado pelo correio. É preciso contratar o serviço junto ao banco e pagará tarifa. * `1` Cliente * `2` Banco Valores: `1`, `2`. |
| `guarantor_name` | `string` | não | Nome do Beneficiário final (Sacador/Avalista) |
| `guarantor_cnpj_cpf` | `string` | não | CNPJ/CPF do Beneficiário final (Sacador/Avalista) |
| `guarantor_address_number` | `string` | não | Número do Beneficiário final (Sacador/Avalista) |
| `guarantor_neighborhood` | `string` | não | Bairro do Beneficiário final (Sacador/Avalista) |
| `guarantor_phone_number` | `string` | não | Telefone (com DDD) do Beneficiário final (Sacador/Avalista) |
| `guarantor_city_name` | `string` | não | Cidade(Nome deve estar correto e completo) do Beneficiário final (Sacador/Avalista) |
| `guarantor_state` | `string` | não | Estado do Beneficiário final (Sacador/Avalista) |
| `guarantor_zipcode` | `string` | não | CEP (formato 99999999) do Beneficiário final (Sacador/Avalista) |
| `guarantor_address` | `string` | não | Endereço do Beneficiário final (Sacador/Avalista) |
| `guarantor_address_complement` | `string` | não | Complemento do Beneficiário final (Sacador/Avalista) |
| `description` | `string` | não | Descrição do produto ou serviço |
| `instructions` | `string` | não | Instruções para o Caixa |
| `document_date` | `string (date)` | não | Data do Documento |
| `document_type` | `string` | não | Tipo de Documento: * `Código` \| `Sigla` \| Descrição * `01` \| `CH` \| Cheque * `02` \| `DM` \| Duplicata Mercantil (Padrão) * `03` \| `DMI` \| Duplicata Mercantil p/ Indicação * `04` \| `DS` \| Duplicata de Serviço * `05` \| `DSI` \| Duplicata de Serviço p/ Indicação * `06` \| `DR` \| Duplicata Rural * `07` \| `LC` \| Letra de Câmbio * `08` \| `NCC` \| Nota de Crédito Comercial * `09` \| `NCE` \| Nota de Crédito a Exportação * `10` \| `NCI` \| Nota de Crédito Industrial * `11` \| `NCR` \| Nota de Crédito Rural * `12` \| `NP` \| Nota Promissória * `13` \| `NPR` \| Nota Promissória Rural * `14` \| `TM` \| Triplicata Mercantil * `15` \| `TS` \| Triplicata de Serviço * `16` \| `NS` \| Nota de Seguro * `17` \| `RC` \| Recibo * `18` \| `FAT` \| Fatura * `19` \| `ND` \| Nota de Débito * `20` \| `AP` \| Apólice de Seguro * `21` \| `ME` \| Mensalidade Escolar * `22` \| `PC` \| Parcela de Consórcio * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Documento de Dívida * `25` \| `CPR` \| Cédula de Produto Rural * `26` \| `CTR` \| Contrato * `27` \| `CSG` \| Cosseguros * `28` \| `EC` \| Encargos Condominiais * `29` \| `CPS` \| Conta de Prestação de Serviços * `30` \| `WR` \| Warrant * `31` \| `DP` \| Duplicata Prestação * `32` \| `CSR` \| Cobrança Seriada * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Apólice Ramos Elementares * `35` \| `CC` \| Cartão de Crédito * `36` \| `BDP` \| Boleto de Proposta * `37` \| `NPD` \| Nota Promissória Direta * `38` \| `DAE` \| Dívida Ativa de Estado * `39` \| `DAM` \| Divida Ativa de Município * `40` \| `DAU` \| Dívida Ativa União * `41` \| `CCB` \| Célula de Crédito Bancário * `42` \| `FI` \| Financiamento * `43` \| `RD` \| Rateio de Despesas * `44` \| `DRI` \| Duplicata Rural p/ Indicação * `45` \| `ECI` \| Encargos Condominiais p/ Indicação * `99` \| `Outros` \| Outros Valores: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `document_type_label` | `string` | não | Tipo de Documento (Sigla) |
| `document_number` | `string` | não | Número do Documento, Tamanho depende do banco e formato do arquivo Remessa: * Banco \|	Formato\| Tamanho \| * Abc \|`Cnab240`\|	15	\| * Ailos \|`Cnab240`\|	15  \| * Arbi \|`Cnab240`\|	15	\| * Banese \|`Cnab240`\|	15	\| * Banestes \|`Cnab400`\|	10	\| * Banrisul \|`Cnab240`\|	15	\| * Banrisul \|`Cnab400`\|	10	\| * Bb \|`Cnab240`\|	15	\| * Bb \|`Cnab400`\|	10	\| * Bib \|`Cnab240`\|	15	\| * Bnb \|`Cnab400`\|	10	\| * Bnpparibas \|`Cnab400`\|	10	\| * Bradesco \|`Cnab240`\|	15	\| * Bradesco \|`Cnab400`\|	10	\| * Brb \|`Cnab400`\|	10	\| * Caixa \|`Cnab240`\|	11	\| * Caixa \|`Cnab400`\|	10	\| * Caruana \|`Cnab400`\|	10	\| * Citibank \|`Cnab400`\|	10	\| * Credisis \|`Cnab240`\|	15	\| * Cresol \|`Cnab240`\|	10	\| * Cresol \|`Cnab400`\|	10	\| * Cresol Bradesco \|`Cnab240`\|	15	\| * Cresol Bradesco \|`Cnab400`\|	10	\| * Daycoval \|`Cnab400`\|	10	\| * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| * Mercantil \|`Cnab240`\|	10	\| * Moneyplus \|`Cnab240`\|	10	\| * Rendimento \|`Cnab400`\|	10	\| * Safra \|`Cnab400`\|	102	\| * Santander \|`Cnab240`\|	15	\| * Santander \|`Cnab400`\|	10	\| * Semear \|`Cnab400`\|	10	\| * Sicoob \|`Cnab240`\|	15	\| * Sicoob \|`Cnab400`\|	10	\| * Sicredi \|`Cnab240`\|	15	\| * Sicredi \|`Cnab400`\|	10	\| * Sofisa \|`Cnab240`\|	15	\| * Unicred \|`Cnab240`\|	15	\| * Uniprime \|`Cnab400`\|	10	\| * Uniprime99 \|`Cnab400`\|	10	\| * Santander \|`Cnab400`\|	10 |
| `acceptance` | `string` | não | Aceite: * `N` Não (Padrão) * `S` Sim Valores: `N`, `S`. |
| `our_number` | `string` | não | Nosso Número. Se não informado, usará o Próximo Nosso Número da Carteira de Cobrança. |
| `processed_our_number` | `string` | não | Nosso Número calculado com DV (formatado) |
| `processed_our_number_raw` | `string` | não | Nosso Número calculado com DV (limpo) |
| `paid_amount` | `number` | não | Valor pago |
| `paid_at` | `string (date)` | não | Data do pagamento |
| `days_for_revoke` | `integer` | não | Dias corridos para Baixa/Devolução: Nulo/Branco: Obedece ao padrão do banco. 0: Baixa/Devolução no mesmo dia do vencimento. 1 ou mais: Baixa/Devolução após o vencimento(Vencimento + X dias corridos). |
| `credit_bureau` | `integer` | não | Birô de Crédito/Órgão Negativador. `0`: Serasa. `1`: Quod. `2`: SPC Opções disponíveis para cada banco suportado: * Banco do Brasil: Serasa e Quod. Valores: `null`, `0`, `1`, `2`. |
| `days_for_negativation` | `integer` | não | Dias corridos para Negativação: Disponível apenas para os seguintes bancos e formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sim \|	Sim	\| Não * Itaú \| Não \| Sim \| Não |
| `days_for_sue_type` | `integer` | não | Tipo de dias para protesto: * `0` Corridos * `1` Úteis Valores: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | não | Dias corridos para Protesto |
| `sue_history` | `array<object>` | não | Histórico de eventos de protesto do boleto |
| `sue_code` | `string` | não | Código de Protesto(CNAB 240). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `revoke_code` | `string` | não | Código de Baixa(CNAB 240). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `first_instruction` | `string` | não | Primeira Instrução(CNAB 400). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `second_instruction` | `string` | não | Segunda Instrução(CNAB 400). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `watermark` | `boolean` | não | Endereço |
| `payment_count` | `integer` | não | Quantidade de pagamentos parciais aceitos para este boleto. |
| `divergent_payment_type` | `integer` | não | Tipo de pagamento divergente: Válido apenas para Itaú e Caixa. * `1` Aceita qualquer valor divergente * `2` Aceita pagamentos dentro de uma faixa de valores ou percentuais * `3` Não aceita pagamento de valores divergentes * `4` Aceita pagamentos de valores superiores a um valor ou percentual mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | não | Tipo de valor a considerar para os limites de pagamentos: Válido apenas para Itaú e Caixa. * `1` Informa pagamentos divergentes por valores * `2` Informa pagamentos divergentes por percentuais Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | não | Valor máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_value` | `number` | não | Valor mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | não | Percentual máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | não | Percentual mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_limit` | `integer` | não | Quantidade de pagamentos permitida. Obrigatório se informados dados para pagamento divergente. Usado somente pela Caixa. |
| `prevent_registration` | `boolean` | não | Impedir envio de registro ao banco: Caso `true`, impede que o boleto seja registrado. Para ser usado nos casos em que o boleto já foi registrado fora da KOBANA mas deseja-se incluí-lo no sistema. |
| `issued_at` | `string (date-time)` | não | Data de emissão do boleto. Aceito somente quando `prevent_registration: true`. |
| `control_number` | `string` | não | Número de controle: Pode conter qualquer informação de interesse da Empresa. A informação contida neste campo sempre retornará com o respectivo título no arquivo-retorno. |
| `ignore_email` | `boolean` | não | Não enviar este boleto por email |
| `ignore_sms` | `boolean` | não | Nunca enviar este boleto por SMS |
| `ignore_whatsapp` | `boolean` | não | Nunca enviar este boleto por WhatsApp |
| `addons` | `string (jsonb)` | não | Endereço |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `meta` | `object` | não | Hash com chave e valor no formato JSON. |
| `notes` | `string` | não | Observações |
| `custom_attachment_name` | `string` | não | Nome para ser usado nos arquivos de boleto enviados para o cliente em notificações. Aceita uso de variáveis. Caso seja deixado vazio, o padrão é a palavra “boleto” acompanhada do ID. |
| `split_payment` | `boolean` | não | Split de Pagamento. Válido apenas para ABC Brasil, Bradesco e Itaú. Caso true, o rateio do boleto será registrado. Informar as contas para rateio em `split_accounts`. Para o Itaú, é necessário informar o tipo de split de pagamento em `split_payment_type` |
| `split_payment_type` | `integer` | não | Tipo de Split de Pagamento: Válido apenas para Itau. Usado apenas com Split de Pagamento true. * `1` Rateio de crédito por percentual  (%) – Valor nominal do título * `2` Rateio de crédito em valor (R$) – Valor nominal do título * `3` Rateio de crédito por percentual (%) – Valor líquido recebido * `4` Rateio de crédito em valor (R$) – Valor líquido recebido, rateado proporcionalmente Valores: `1`, `2`, `3`, `4`. |
| `split_accounts` | `array<object>` | não | Contas para Split de pagamento. |
| `payment_place` | `string` | não | Local de Pagamento |
| `installment_id` | `integer` | não | ID do Carnê |
| `installment_number` | `integer` | não | Número da parcela do carnê |
| `installment_total` | `integer` | não | Total de parcelas do carnê |
| `customer_subscription_id` | `integer` | não | ID da Assinatura |
| `beneficiary_name` | `string` | não | Nome do Beneficiário |
| `beneficiary_cnpj_cpf` | `string` | não | CNPJ/CPF do Beneficiário |
| `beneficiary_address` | `string` | não | Endereço do Beneficiário |
| `beneficiary_assignor_code` | `string` | não | Agência/Código do Beneficiário |
| `bank_contract_slug` | `string` | não | Slug da Carteira |
| `agency_number` | `string` | não | Agência |
| `agency_digit` | `string` | não | Dígito da Agência |
| `account_number` | `string` | não | Conta |
| `account_digit` | `string` | não | Dígito da Conta |
| `extra1` | `string` | não | Campo extra 1 |
| `extra1_digit` | `string` | não | Digito do Campo extra 1 |
| `extra2` | `string` | não | Campo extra 2 |
| `extra2_digit` | `string` | não | Dígito do Campo extra 2 |
| `created_via_api` | `boolean` | não | Indica se o boleto foi criado por API |
| `created_at` | `string (date-time)` | não | Data e hora de criação do boleto |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do boleto |
| `registration_status` | `string` | não | Situação do registro no banco: * `pending` Pendente * `skipped` Ignorado * `requested` Requisitado * `confirmed` Confirmado * `rejected` Rejeitado (ainda será tentado novamente) * `failed` Falha (não será tentado novamente) Valores: `pending`, `skipped`, `requested`, `confirmed`, `rejected`, `failed`. |
| `registered_at` | `string (date-time)` | não | Data e hora do registro (quando confirmado) |
| `register_type` | `integer` | não | Tipo de Registro: * `1` API * `2` Banco Valores: `1`, `2`. |
| `cancel_type` | `integer` | não | Tipo de Cancelamento: * `1` Cliente * `2` Banco Valores: `null`, `1`, `2`. |
| `cancellation_reason` | `integer` | não | Motivo de Cancelamento: * `1` Outro * `2` Fraude * `3` Óbito * `4` Erro operacional * `5` Quitação paga Valores: `null`, `1`, `2`, `3`, `4`, `5`. |
| `line` | `string` | não | Linha Digitável |
| `barcode` | `string` | não | Código de Barras |
| `shorten_url` | `string` | não | URL para visualização do boleto |
| `url` | `string` | não | URL para visualização do boleto |
| `carne_url` | `string` | não | URL para visualização do carnê(Quando for parcela) |
| `formats` | `object` | não | URLs com formatos disponíveis. Ex.: PDF, Imagem, Pix e etc |
| `pix_enabled` | `boolean` | não | Indica se o boleto é híbrido e tem QRcode Pix |
| `pix_qrcode` | `string` | não | QRcode Pix do boleto híbrido |
| `pix_txid` | `string` | não | Campo txid do Pix. Gerado automaticamente por default caso não fornecido. |
| `prevent_pix` | `boolean` | não | Caso verdadeiro, impede a criação do Pix para carteiras com Pix habilitado. Não é considerado se a carteira não tem Pix habilitado. |
| `status` | `string` | não | Situação do boleto: * `generating` Gerando * `draft` Rascunho * `generation_failed` Falha ao gerar * `opened` Aberto * `canceled` Cancelado * `paid` Pago * `overdue` Vencido * `validation_failed` Inválido * `chargeback` Estornado Valores: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `recipient_account` | `string` | não | Conta Destinatária + Dígito |
| `reduction_type` | `integer` | não | Tipo de abatimento: `1`: Valor. `2`: Porcentagem. `3`: Sem abatimento Valores: `1`, `2`, `3`. |
| `reduction_amount` | `number` | não | Valor do abatimento. Obrigatório se reduction_type é igual a 1. |
| `reduction_percentage` | `number` | não | Porcentagem de Abatimento. Ex: 2% x R$ 250,00 = R$ 5,00. Obrigatória se reduction_type é igual a 2. Até 2 casas decimais. |
| `instructions_mode` | `integer` | não | Cálculo de datas na Instrução para o Caixa: * `0` Não preencher as instruções para o caixa * `1` Calcular data pela via de registro (API/CNAB) * `2` Usar a data configurada no boleto Valores: `0`, `1`, `2`. |
| `import_id` | `integer` | não | ID da Importação |
| `virtual_bank_billet_id` | `integer` | não | ID do Boleto gerado por membro de contrato BackOffice. (BackOffice precisa estar habilitado). |
| `external_id` | `string` | não | ID do boleto no sistema do cliente. Opcional para controle e busca interna. |
| `financial_provider_external_id` | `string` | não | ID na instituição financeira |
| `after_create` | `array<string>` | não | Execução automática de comandos após o boleto ser criado. Valores permitidos: * `sync`: Sincronização com o provedor financeiro. |

Outros campos e objetos aninhados: `kobana schema banking.bank-billets.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Boleto criado |
| `422` | Pagamento parcial Itaú requer tipo de autorização específico |

## bank-billets cancel-all

Cancelar Boletos em Lote

```text
kobana banking bank-billets cancel-all [OPÇÕES]
```

`POST /v1/bank_billets/cancel_all`

Cancelar boletos em lote

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Situação do boleto: * `generating` Gerando * `draft` Rascunho * `generation_failed` Falha ao gerar * `opened` Aberto * `canceled` Cancelado * `paid` Pago * `overdue` Vencido * `validation_failed` Inválido * `chargeback` Estornado Valores: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `expire_from` | `string (date)` | não | A partir de Data de vencimento |
| `expire_to` | `string (date)` | não | Até a Data de vencimento |
| `cnpj_cpf` | `string` | não | CNPJ ou CPF do Pagador |
| `bank_billet_ids` | `array<integer>` | não | IDs de boletos a serem cancelados |

Outros campos e objetos aninhados: `kobana schema banking.bank-billets.cancel-all`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Solicitação realizada com sucesso |
| `422` | Unprocessable Entity |

## bank-billets get

Visualizar o Boleto

```text
kobana banking bank-billets get [ID] [OPÇÕES]
```

`GET /v1/bank_billets/{id}`

Consulta informações específicas de um Boleto.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Boleto encontrado |
| `404` | Boleto não encontrado |

## bank-billets update

Atualizar o Boleto

```text
kobana banking bank-billets update [ID] [OPÇÕES]
```

`PUT /v1/bank_billets/{id}`

Atualiza informações específicas de um Boleto.

Você pode alterar boletos no status de Aberto(`opened`) ou Vencido(`overdue`)

Você receberá webhooks `generating` e `opened` para o boleto alterado.

Em carteiras registradas, a alteração irá entrar na remessa e pode ser cobrada taxa bancária.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | não | Valor do Boleto |
| `expire_at` | `string (date)` | não | Data de vencimento |
| `tags` | `array<string>` | não | Tags associadas ao boleto |
| `notes` | `string` | não | Observações |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. Pode ser alterado em qualquer status do boleto. |
| `days_for_sue_type` | `integer` | não | Tipo de dias para protesto: * `0` Corridos * `1` Úteis Valores: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | não | Dias corridos para Protesto |
| `days_for_revoke` | `integer` | não | Dias corridos para Baixa Bancos Suportados: Itaú |
| `sue_code` | `string` | não | Código de Protesto(Somente por CNAB 240). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `instructions` | `string` | não | Instruções para o Caixa |
| `description` | `string` | não | Descrição do produto ou serviço |
| `interest_type` | `integer` | não | Tipo de juros/mora: * `0` Inexistente (Padrão) * `1` Para porcentagem diária * `2` Para valor diário * `7` Para porcentagem mensal - Bancos suportados: Bradesco, BB, BTG, Caixa, Inter, Itaú, Safra, Santander, Sicoob e Sicredi Valores: `0`, `1`, `2`, `7`. |
| `days_for_interest` | `integer` | não | Quantidade de dias após o vencimento que a mora começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `interest_percentage` | `number` | não | Porcentagem de juros. De 0.0 a 100.0 (Ex 1.5% = 1.5) Obrigatório se interest_type é igual a 1 ou 7. Até 2 casas decimais. |
| `interest_value` | `number` | não | Valor diário de juros (R$). Obrigatório se interest_type é igual a 2. Até 2 casas decimais. |
| `discount_type` | `integer` | não | Tipo de desconto: O tipo de desconto será o mesmo para todos os três descontos, caso existam. : * `0` Inexistente (Padrão) * `1` Para valor fixo * `2` Para percentual do valor do boleto Valores: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | não | Dias para desconto. Obrigatório se discount_type é diferente de 0(zero) |
| `discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao desconto. Obrigatório se discount_type é igual a 2 |
| `discount_value` | `number` | não | Valor do desconto (R$). Obrigatório se discount_type é igual a 1. |
| `days_for_second_discount` | `integer` | não | Dias para segundo desconto. |
| `second_discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao segundo desconto. |
| `second_discount_value` | `number` | não | Valor do segundo desconto (R$). |
| `days_for_third_discount` | `integer` | não | Dias para terceiro desconto. |
| `third_discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao terceiro desconto. |
| `third_discount_value` | `number` | não | Valor do terceiro desconto (R$). |
| `reduction_type` | `integer` | não | Tipo de abatimento: `1`: Valor. `2`: Porcentagem. `3`: Sem abatimento Valores: `1`, `2`, `3`. |
| `reduction_amount` | `number` | não | Valor do abatimento. Obrigatório se reduction_type é igual a 1. |
| `reduction_percentage` | `number` | não | Porcentagem de Abatimento. Ex: 2% x R$ 250,00 = R$ 5,00. Obrigatória se reduction_type é igual a 2. Até 2 casas decimais. |
| `divergent_payment_type` | `integer` | não | Tipo de pagamento divergente: * `1` Aceita qualquer valor divergente * `2` Aceita pagamentos dentro de uma faixa de valores ou percentuais * `3` Não aceita pagamento de valores divergentes * `4` Aceita pagamentos de valores superiores a um valor ou percentual mínimo Bancos suportados: Itaú Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | não | Tipo de valor a considerar para os limites de pagamentos: Válido apenas para Itaú e Caixa. * `1` Informa pagamentos divergentes por valores * `2` Informa pagamentos divergentes por percentuais Bancos suportados: Itaú Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | não | Valor máximo para a faixa de pagamentos divergentes. Bancos suportados: Itaú |
| `divergent_payment_minimum_value` | `number` | não | Valor mínimo para a faixa de pagamentos divergentes. Bancos suportados: Itaú |
| `divergent_payment_maximum_percentage` | `number` | não | Percentual máximo para a faixa de pagamentos divergentes. Bancos suportados: Itaú |
| `divergent_payment_minimum_percentage` | `number` | não | Percentual mínimo para a faixa de pagamentos divergentes. Bancos suportados: Itaú |
| `fine_type` | `integer` | não | Tipo de multa: * `0` Inexistente (Padrão) * `1` Para percentual do valor do boleto * `2` Para valor fixo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | não | Quantidade de dias após o vencimento que a multa começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `credit_bureau` | `integer` | não | Birô de Crédito/Órgão Negativador. `0`: Serasa. `1`: Quod. `2`: SPC Opções disponíveis para cada banco suportado: * Banco do Brasil: Serasa e Quod. Valores: `null`, `0`, `1`, `2`. |
| `days_for_negativation` | `integer` | não | Quantidade de dias após o vencimento para negativar o título. |
| `fine_percentage` | `number` | não | Porcentagem de Multa por Atraso Ex: 2% x R$ 250,00 = R$ 5,00. Obrigatória se fine_type é igual a 1. Até 2 casas decimais. |
| `fine_value` | `number` | não | Valor da multa (R$). Obrigatório se fine_type é igual a 2. Até 2 casas decimais. |
| `document_number` | `string` | não | Número do Documento, Tamanho depende do banco, formato e forma de envio (API / EDI): * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| |
| `customer_person_name` | `string` | não | Nome do cliente. Bancos Suportados: Santander |
| `customer_cnpj_cpf` | `string` | não | CPF/CNPJ do cliente. Bancos Suportados: Santander |
| `customer_state` | `string` | não | Estado. Bancos Suportados: Santander |
| `customer_city_name` | `string` | não | Cidade(Nome deve estar correto e completo). Bancos Suportados: Santander |
| `customer_zipcode` | `string` | não | CEP (formato 99999999). Bancos Suportados: Santander |
| `customer_address` | `string` | não | Endereço. Bancos Suportados: Santander |
| `customer_neighborhood` | `string` | não | Bairro. Bancos Suportados: Santander |
| `guarantor_name` | `string` | não | Nome do Beneficiário final (Sacador/Avalista). Bancos Suportados: Santander |
| `guarantor_cnpj_cpf` | `string` | não | CNPJ/CPF do Beneficiário final (Sacador/Avalista). Bancos Suportados: Santander |
| `document_type` | `string` | não | Tipo de Documento: * `Código` \| `Sigla` \| Descrição * `01` \| `CH` \| Cheque * `02` \| `DM` \| Duplicata Mercantil (Padrão) * `03` \| `DMI` \| Duplicata Mercantil p/ Indicação * `04` \| `DS` \| Duplicata de Serviço * `05` \| `DSI` \| Duplicata de Serviço p/ Indicação * `06` \| `DR` \| Duplicata Rural * `07` \| `LC` \| Letra de Câmbio * `08` \| `NCC` \| Nota de Crédito Comercial * `09` \| `NCE` \| Nota de Crédito a Exportação * `10` \| `NCI` \| Nota de Crédito Industrial * `11` \| `NCR` \| Nota de Crédito Rural * `12` \| `NP` \| Nota Promissória * `13` \| `NPR` \| Nota Promissória Rural * `14` \| `TM` \| Triplicata Mercantil * `15` \| `TS` \| Triplicata de Serviço * `16` \| `NS` \| Nota de Seguro * `17` \| `RC` \| Recibo * `18` \| `FAT` \| Fatura * `19` \| `ND` \| Nota de Débito * `20` \| `AP` \| Apólice de Seguro * `21` \| `ME` \| Mensalidade Escolar * `22` \| `PC` \| Parcela de Consórcio * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Documento de Dívida * `25` \| `CPR` \| Cédula de Produto Rural * `26` \| `CTR` \| Contrato * `27` \| `CSG` \| Cosseguros * `28` \| `EC` \| Encargos Condominiais * `29` \| `CPS` \| Conta de Prestação de Serviços * `30` \| `WR` \| Warrant * `31` \| `DP` \| Duplicata Prestação * `32` \| `CSR` \| Cobrança Seriada * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Apólice Ramos Elementares * `35` \| `CC` \| Cartão de Crédito * `36` \| `BDP` \| Boleto de Proposta * `37` \| `NPD` \| Nota Promissória Direta * `38` \| `DAE` \| Dívida Ativa de Estado * `39` \| `DAM` \| Divida Ativa de Município * `40` \| `DAU` \| Dívida Ativa União * `41` \| `CCB` \| Célula de Crédito Bancário * `42` \| `FI` \| Financiamento * `43` \| `RD` \| Rateio de Despesas * `44` \| `DRI` \| Duplicata Rural p/ Indicação * `45` \| `ECI` \| Encargos Condominiais p/ Indicação * `99` \| `Outros` \| Outros Bancos Suportados: Santander Valores: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `control_number` | `string` | não | Número de controle: Pode conter qualquer informação de interesse da Empresa. A informação contida neste campo sempre retornará com o respectivo título no arquivo-retorno. Bancos Suportados: Santander |
| `payment_count` | `integer` | não | Quantidade de pagamentos parciais aceitos para este boleto. Bancos Suportados: Santander |
| `issued_at` | `string (date)` | não | Data de emissão. Bancos Suportados: Santander |
| `bolepix_key` | `string` | não | Chave do bolepix. Bancos Suportados: Santander |
| `pix_txid` | `string` | não | TxId do pix. Bancos Suportados: Santander |

Outros campos e objetos aninhados: `kobana schema banking.bank-billets.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Boleto atualizado |
| `404` | Boleto não encontrado |
| `422` | Campos não suportados pelo banco |

## bank-billets cancel

Cancelar o Boleto

```text
kobana banking bank-billets cancel [ID] [OPÇÕES]
```

`PUT /v1/bank_billets/{id}/cancel`

Cancela boletos nos status de Aberto(opened) ou Vencido(overdue)

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cancellation_reason` | `integer` | não | Motivo do cancelamento: * `1` Outro * `2` Fraude * `3` Óbito * `4` Erro operacional * `5` Quitação paga Valores: `1`, `2`, `3`, `4`, `5`. |

Outros campos e objetos aninhados: `kobana schema banking.bank-billets.cancel`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Boleto cancelado |
| `404` | Boleto não encontrado |
| `422` | Boleto não pode ser cancelado |

## bank-billets duplicate

Duplicar Boleto

```text
kobana banking bank-billets duplicate [ID] [OPÇÕES]
```

`POST /v1/bank_billets/{id}/duplicate`

Duplica boleto permitindo alterar informações do novo boleto e ou cancelar o boleto duplicado

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `expire_at_in_days` | `integer` | não | Nº de dias para vencimento a partir da data de hoje. Default: 7 |
| `amount` | `number` | não | Valor do novo Boleto |
| `cancel` | `boolean` | não | Cancelar o boleto que está sendo duplicado. Default: true |
| `with_fines` | `boolean` | não | Atualizar o valor do novo boleto com juros e multa. Default: false |
| `bank_billet_account_id` | `integer` | não | ID da Carteira de Cobrança. Se não informado, usará a carteira padrão. |
| `bank_billet_layout_id` | `integer` | não | ID do Modelo de Boleto |
| `customer_id` | `integer` | não | ID do Cliente. Quando esse ID é passado, os campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` e `customer_neighborhood` não são obrigatórios. |
| `customer_person_name` | `string` | não | Nome do cliente |
| `customer_cnpj_cpf` | `string` | não | CPF/CNPJ do cliente |
| `customer_state` | `string` | não | Estado |
| `customer_city_name` | `string` | não | Cidade(Nome deve estar correto e completo) |
| `customer_zipcode` | `string` | não | CEP (formato 99999999) |
| `customer_address` | `string` | não | Endereço |
| `customer_address_complement` | `string` | não | Complemento |
| `customer_address_number` | `string` | não | Número |
| `customer_email` | `string (email)` | não | E-mail do Pagador |
| `customer_email_cc` | `string (email)` | não | E-mail alternativo do Pagador |
| `customer_neighborhood` | `string` | não | Bairro |
| `customer_phone_number` | `string` | não | Telefone (com DDD, DDI é opcional) |
| `customer_ignore_email` | `boolean` | não | Nunca enviar e-mail para este cliente |
| `customer_ignore_sms` | `boolean` | não | Nunca enviar SMS para este cliente |
| `customer_mobile_local_code` | `string` | não | DDD do Celular |
| `customer_mobile_number` | `string` | não | Celular |
| `customer_nickname` | `string` | não | Apelido ou Nome Fantasia do Pagador |
| `customer_notes` | `string` | não | Observações do Pagador |
| `customer_contact_person` | `string` | não | Contato |
| `interest_type` | `integer` | não | Tipo de juros/mora: * `0` Inexistente (Padrão) * `1` Para porcentagem diária * `2` Para valor diário Valores: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | não | Quantidade de dias após o vencimento que a mora começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `interest_percentage` | `number` | não | Porcentagem diária de juros. De 0.0 a 100.0 (Ex 1.5% = 1.5) Obrigatório se interest_type é igual a 1. |
| `interest_value` | `number` | não | Valor diário de juros (R$). Obrigatório se interest_type é igual a 2. |
| `interest_days_type` | `integer` | não | Tipo de Dias para juros: * `0` Corridos * `1` Úteis Valores: `0`, `1`. |
| `fine_type` | `integer` | não | Tipo de multa: * `0` Inexistente (Padrão) * `1` Para percentual do valor do boleto * `2` Para valor fixo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | não | Quantidade de dias após o vencimento que a multa começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `fine_percentage` | `number` | não | Porcentagem de Multa por Atraso Ex: 2% x R$ 250,00 = R$ 5,00. Obrigatória se fine_type é igual a 1 |
| `fine_value` | `number` | não | Valor da multa (R$). Obrigatório se fine_type é igual a 2. |
| `discount_type` | `integer` | não | Tipo de desconto: O tipo de desconto será o mesmo para todos os três descontos, caso existam. : * `0` Inexistente (Padrão) * `1` Para valor fixo * `2` Para percentual do valor do boleto Valores: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | não | Dias para desconto. Obrigatório se discount_type é diferente de 0(zero) |
| `discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao desconto. Obrigatório se discount_type é igual a 2 |
| `discount_value` | `number` | não | Valor do desconto (R$). Obrigatório se discount_type é igual a 1. |
| `days_for_second_discount` | `integer` | não | Dias para segundo desconto. |
| `second_discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao segundo desconto. |
| `second_discount_value` | `number` | não | Valor do segundo desconto (R$). |
| `days_for_third_discount` | `integer` | não | Dias para terceiro desconto. |
| `third_discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao terceiro desconto. |
| `third_discount_value` | `number` | não | Valor do terceiro desconto (R$). |
| `tags` | `array<string>` | não | Tags associadas ao boleto |
| `tag_list` | `string` | não | Tags associadas ao boleto |
| `charge_type` | `integer` | não | Tipo de Cobrança: * `1` Simples * `2` Vinculada * `3` Descontada * `4` Vendor Valores: `1`, `2`, `3`, `4`. |
| `dispatch_type` | `integer` | não | Tipo de Cobrança: Quando o boleto precisa ser enviado pelo correio. É preciso contratar o serviço junto ao banco e pagará tarifa. * `1` Cliente * `2` Banco Valores: `1`, `2`. |
| `guarantor_name` | `string` | não | Nome do Beneficiário final (Sacador/Avalista) |
| `guarantor_cnpj_cpf` | `string` | não | CNPJ/CPF do Beneficiário final (Sacador/Avalista) |
| `guarantor_address_number` | `string` | não | Número do Beneficiário final (Sacador/Avalista) |
| `guarantor_neighborhood` | `string` | não | Bairro do Beneficiário final (Sacador/Avalista) |
| `guarantor_phone_number` | `string` | não | Telefone (com DDD) do Beneficiário final (Sacador/Avalista) |
| `guarantor_city_name` | `string` | não | Cidade(Nome deve estar correto e completo) do Beneficiário final (Sacador/Avalista) |
| `guarantor_state` | `string` | não | Estado do Beneficiário final (Sacador/Avalista) |
| `guarantor_zipcode` | `string` | não | CEP (formato 99999999) do Beneficiário final (Sacador/Avalista) |
| `guarantor_address` | `string` | não | Endereço do Beneficiário final (Sacador/Avalista) |
| `guarantor_address_complement` | `string` | não | Complemento do Beneficiário final (Sacador/Avalista) |
| `description` | `string` | não | Descrição do produto ou serviço |
| `instructions` | `string` | não | Instruções para o Caixa |
| `document_date` | `string (date)` | não | Data do Documento |
| `document_type` | `string` | não | Tipo de Documento: * `Código` \| `Sigla` \| Descrição * `01` \| `CH` \| Cheque * `02` \| `DM` \| Duplicata Mercantil (Padrão) * `03` \| `DMI` \| Duplicata Mercantil p/ Indicação * `04` \| `DS` \| Duplicata de Serviço * `05` \| `DSI` \| Duplicata de Serviço p/ Indicação * `06` \| `DR` \| Duplicata Rural * `07` \| `LC` \| Letra de Câmbio * `08` \| `NCC` \| Nota de Crédito Comercial * `09` \| `NCE` \| Nota de Crédito a Exportação * `10` \| `NCI` \| Nota de Crédito Industrial * `11` \| `NCR` \| Nota de Crédito Rural * `12` \| `NP` \| Nota Promissória * `13` \| `NPR` \| Nota Promissória Rural * `14` \| `TM` \| Triplicata Mercantil * `15` \| `TS` \| Triplicata de Serviço * `16` \| `NS` \| Nota de Seguro * `17` \| `RC` \| Recibo * `18` \| `FAT` \| Fatura * `19` \| `ND` \| Nota de Débito * `20` \| `AP` \| Apólice de Seguro * `21` \| `ME` \| Mensalidade Escolar * `22` \| `PC` \| Parcela de Consórcio * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Documento de Dívida * `25` \| `CPR` \| Cédula de Produto Rural * `26` \| `CTR` \| Contrato * `27` \| `CSG` \| Cosseguros * `28` \| `EC` \| Encargos Condominiais * `29` \| `CPS` \| Conta de Prestação de Serviços * `30` \| `WR` \| Warrant * `31` \| `DP` \| Duplicata Prestação * `32` \| `CSR` \| Cobrança Seriada * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Apólice Ramos Elementares * `35` \| `CC` \| Cartão de Crédito * `36` \| `BDP` \| Boleto de Proposta * `37` \| `NPD` \| Nota Promissória Direta * `38` \| `DAE` \| Dívida Ativa de Estado * `39` \| `DAM` \| Divida Ativa de Município * `40` \| `DAU` \| Dívida Ativa União * `41` \| `CCB` \| Célula de Crédito Bancário * `42` \| `FI` \| Financiamento * `43` \| `RD` \| Rateio de Despesas * `44` \| `DRI` \| Duplicata Rural p/ Indicação * `45` \| `ECI` \| Encargos Condominiais p/ Indicação * `99` \| `Outros` \| Outros Valores: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `document_type_label` | `string` | não | Tipo de Documento (Sigla) |
| `document_number` | `string` | não | Número do Documento |
| `acceptance` | `string` | não | Aceite: * `N` Não (Padrão) * `S` Sim Valores: `N`, `S`. |
| `our_number` | `string` | não | Nosso Número. Se não informado, usará o Próximo Nosso Número da Carteira de Cobrança. |
| `processed_our_number` | `string` | não | Nosso Número calculado com DV (formatado) |
| `processed_our_number_raw` | `string` | não | Nosso Número calculado com DV (limpo) |
| `paid_amount` | `number` | não | Valor pago |
| `paid_at` | `string (date)` | não | Data do pagamento |
| `days_for_revoke` | `integer` | não | Dias corridos para Baixa/Devolução: Nulo/Branco: Obedece ao padrão do banco. 0: Baixa/Devolução no mesmo dia do vencimento. 1 ou mais: Baixa/Devolução após o vencimento(Vencimento + X dias corridos). |
| `days_for_negativation` | `integer` | não | Dias corridos para Negativação: Disponível apenas para os seguintes bancos e formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sim \|	Sim	\| Não * Itaú \| Não \| Sim \| Não |
| `days_for_sue_type` | `integer` | não | Tipo de dias para protesto: * `0` Corridos * `1` Úteis Valores: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | não | Dias corridos para Protesto |
| `sue_code` | `string` | não | Código de Protesto(CNAB 240). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `revoke_code` | `string` | não | Código de Baixa(CNAB 240). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `first_instruction` | `string` | não | Primeira Instrução(CNAB 400). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `second_instruction` | `string` | não | Segunda Instrução(CNAB 400). Consulte os possíveis valores &lt;a href="https://docs.banking.kobana.com.br/pt/api/bancos/" target="_blank">para cada banco&lt;/a>. |
| `watermark` | `boolean` | não | Endereço |
| `divergent_payment_type` | `integer` | não | Tipo de pagamento divergente: Válido apenas para Itaú e Caixa. * `1` Aceita qualquer valor divergente * `2` Aceita pagamentos dentro de uma faixa de valores ou percentuais * `3` Não aceita pagamento de valores divergentes * `4` Aceita pagamentos de valores superiores a um valor ou percentual mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | não | Tipo de valor a considerar para os limites de pagamentos: Válido apenas para Itaú e Caixa. * `1` Informa pagamentos divergentes por valores * `2` Informa pagamentos divergentes por percentuais Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | não | Valor mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_value` | `number` | não | Valor máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | não | Percentual mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | não | Percentual máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_limit` | `integer` | não | Quantidade de pagamentos permitida. Obrigatório se informados dados para pagamento divergente. Usado somente pela Caixa. |
| `prevent_registration` | `boolean` | não | Impedir envio de registro ao banco: Caso `true`, impede que o boleto seja registrado. Para ser usado nos casos em que o boleto já foi registrado fora da KOBANA mas deseja-se incluí-lo no sistema. |
| `control_number` | `string` | não | Número de controle: Pode conter qualquer informação de interesse da Empresa. A informação contida neste campo sempre retornará com o respectivo título no arquivo-retorno. |
| `ignore_email` | `boolean` | não | Não enviar este boleto por email |
| `ignore_sms` | `boolean` | não | Nunca enviar este boleto por SMS |
| `ignore_whatsapp` | `boolean` | não | Nunca enviar este boleto por WhatsApp |
| `addons` | `string (jsonb)` | não | Endereço |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `meta` | `object` | não | Hash com chave e valor no formato JSON. |
| `notes` | `string` | não | Observações |
| `custom_attachment_name` | `string` | não | Nome para ser usado nos arquivos de boleto enviados para o cliente em notificações. Aceita uso de variáveis. Caso seja deixado vazio, o padrão é a palavra “boleto” acompanhada do ID. |
| `split_payment` | `boolean` | não | Split de Pagamento. Válido apenas para ABC Brasil (remessa CNAB 240), Itaú e Bradesco (remessa CNAB 400). Caso true, o rateio do boleto será registrado. Informar as contas para rateio em `split_accounts` |
| `split_payment_type` | `integer` | não | Tipo de Split de Pagamento: Válido apenas para Itau. Usado apenas com Split de Pagamento true. * `1` Rateio de crédito por percentual  (%) – Valor nominal do título * `2` Rateio de crédito em valor (R$) – Valor nominal do título * `3` Rateio de crédito por percentual (%) – Valor líquido recebido * `4` Rateio de crédito em valor (R$) – Valor líquido recebido, rateado proporcionalmente Valores: `1`, `2`, `3`, `4`. |
| `split_accounts` | `array<object>` | não | Contas para Split de pagamento. |
| `payment_place` | `string` | não | Local de Pagamento |
| `installment_id` | `integer` | não | ID do Carnê |
| `installment_number` | `integer` | não | Número da parcela do carnê |
| `installment_total` | `integer` | não | Total de parcelas do carnê |
| `customer_subscription_id` | `integer` | não | ID da Assinatura |
| `beneficiary_name` | `string` | não | Nome do Beneficiário |
| `beneficiary_cnpj_cpf` | `string` | não | CNPJ/CPF do Beneficiário |
| `beneficiary_address` | `string` | não | Endereço do Beneficiário |
| `beneficiary_assignor_code` | `string` | não | Agência/Código do Beneficiário |
| `bank_contract_slug` | `string` | não | Slug da Carteira |
| `agency_number` | `string` | não | Agência |
| `agency_digit` | `string` | não | Dígito da Agência |
| `account_number` | `string` | não | Conta |
| `account_digit` | `string` | não | Dígito da Conta |
| `extra1` | `string` | não | Campo extra 1 |
| `extra1_digit` | `string` | não | Digito do Campo extra 1 |
| `extra2` | `string` | não | Campo extra 2 |
| `extra2_digit` | `string` | não | Dígito do Campo extra 2 |
| `created_via_api` | `boolean` | não | Indica se o boleto foi criado por API |
| `created_at` | `string (date-time)` | não | Data e hora de criação do boleto |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do boleto |
| `registered_at` | `string (date-time)` | não | Data e hora do registro |
| `register_type` | `integer` | não | Tipo de Registro: * `1` API * `2` Banco Valores: `1`, `2`. |
| `cancel_type` | `integer` | não | Tipo de Cancelamento: * `1` Cliente * `2` Banco Valores: `1`, `2`. |
| `payment_count` | `integer` | não | Quantidade de pagamentos parciais aceitos para este boleto. |
| `line` | `string` | não | Linha Digitável |
| `barcode` | `string` | não | Endereço |
| `shorten_url` | `string` | não | URL para visualização do boleto |
| `url` | `string` | não | URL para visualização do boleto |
| `carne_url` | `string` | não | URL para visualização do carnê(Quando for parcela) |
| `formats` | `object` | não | URLs com formatos disponíveis. Ex.: PDF, Imagem, Pix e etc |
| `pix_enabled` | `boolean` | não | Indica se o boleto é híbrido e tem QRcode Pix |
| `pix_qrcode` | `string` | não | QRcode Pix do boleto híbrido |
| `status` | `string` | não | Situação do boleto: * `generating` Gerando * `draft` Rascunho * `generation_failed` Falha ao gerar * `opened` Aberto * `canceled` Cancelado * `paid` Pago * `overdue` Vencido * `validation_failed` Inválido * `chargeback` Estornado Valores: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `recipient_account` | `string` | não | Conta Destinatária + Dígito |

Outros campos e objetos aninhados: `kobana schema banking.bank-billets.duplicate`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Boleto duplicado |
| `404` | Boleto não encontrado |

## bank-billets pay

Marcar Boleto Como Pago

```text
kobana banking bank-billets pay [ID] [OPÇÕES]
```

`PUT /v1/bank_billets/{id}/pay`

Marcar boleto como pago

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `paid_amount` | `number` | sim | Valor pago do boleto. |
| `paid_at` | `string (date)` | sim | Data de pagamento do boleto |
| `bank_rate` | `string` | não | Valor da taxa bancária. |
| `direct_payment` | `boolean` | não | Informa se o pagamento foi feito diretamente ao beneficiário. |

Outros campos e objetos aninhados: `kobana schema banking.bank-billets.pay`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Boleto pago |
| `404` | Boleto não encontrado |

## bank-billets send-email

Enviar Boleto por E-mail

```text
kobana banking bank-billets send-email [ID] [OPÇÕES]
```

`POST /v1/bank_billets/{id}/send_email`

Enviar boleto por e-mail

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `form` | `object` | sim | Formulário de E-mail. |

Outros campos e objetos aninhados: `kobana schema banking.bank-billets.send-email`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | E-mail enviado com sucesso |
| `400` | Falha ao enviar o e-mail |

## bank-billets send-sms

Enviar Boleto por SMS

```text
kobana banking bank-billets send-sms [ID] [OPÇÕES]
```

`POST /v1/bank_billets/{id}/send_sms`

Enviar boleto por sms

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `form` | `object` | sim | Formulário de SMS. |

Outros campos e objetos aninhados: `kobana schema banking.bank-billets.send-sms`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | SMS enviado com sucesso |
| `400` | Falha ao enviar o sms |
