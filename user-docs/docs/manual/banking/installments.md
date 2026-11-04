---
id: installments
title: "kobana banking installments"
sidebar_label: "installments"
---

# kobana banking installments


Comandos de `kobana banking installments`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`installments list`](#installments-list) | `GET /v1/installments` | Listar Carnês |
| [`installments create`](#installments-create) | `POST /v1/installments` | Criar um Carnê |
| [`installments get`](#installments-get) | `GET /v1/installments/{id}` | Informações do Carnê |
| [`installments delete`](#installments-delete) | `DELETE /v1/installments/{id}` | Excluir o Carnê |

## installments list

Listar Carnês

```text
kobana banking installments list [OPÇÕES]
```

`GET /v1/installments`

Lista Carnês.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de carnês |

## installments create

Criar um Carnê

```text
kobana banking installments create [OPÇÕES]
```

`POST /v1/installments`

Cria um carnê para geração de múltiplos boletos com vencimentos periódicos.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `bank_billet_account_id` | `integer` | sim | ID da Carteira de Cobrança. Se não informado, usará a carteira padrão. |
| `amount` | `number` | sim | Quantia |
| `customer_person_name` | `string` | sim | Nome do cliente |
| `customer_cnpj_cpf` | `string` | sim | CPF/CNPJ do cliente |
| `customer_state` | `string` | sim | Estado |
| `customer_city_name` | `string` | sim | Cidade(Nome deve estar correto e completo) |
| `customer_zipcode` | `string` | sim | CEP (formato 99999999) |
| `customer_address` | `string` | sim | Endereço |
| `customer_neighborhood` | `string` | sim | Bairro |
| `start_at` | `string (date)` | sim | Data da primeira cobrança |
| `total` | `integer` | sim | Quantidade de parcelas |
| `document_type` | `string` | sim | Tipo de Documento: * `Código` \| `Sigla` \| Descrição * `01` \| `CH` \| Cheque * `02` \| `DM` \| Duplicata Mercantil (Padrão) * `03` \| `DMI` \| Duplicata Mercantil p/ Indicação * `04` \| `DS` \| Duplicata de Serviço * `05` \| `DSI` \| Duplicata de Serviço p/ Indicação * `06` \| `DR` \| Duplicata Rural * `07` \| `LC` \| Letra de Câmbio * `08` \| `NCC` \| Nota de Crédito Comercial * `09` \| `NCE` \| Nota de Crédito a Exportação * `10` \| `NCI` \| Nota de Crédito Industrial * `11` \| `NCR` \| Nota de Crédito Rural * `12` \| `NP` \| Nota Promissória * `13` \| `NPR` \| Nota Promissória Rural * `14` \| `TM` \| Triplicata Mercantil * `15` \| `TS` \| Triplicata de Serviço * `16` \| `NS` \| Nota de Seguro * `17` \| `RC` \| Recibo * `18` \| `FAT` \| Fatura * `19` \| `ND` \| Nota de Débito * `20` \| `AP` \| Apólice de Seguro * `21` \| `ME` \| Mensalidade Escolar * `22` \| `PC` \| Parcela de Consórcio * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Documento de Dívida * `25` \| `CPR` \| Cédula de Produto Rural * `26` \| `CTR` \| Contrato * `27` \| `CSG` \| Cosseguros * `28` \| `EC` \| Encargos Condominiais * `29` \| `CPS` \| Conta de Prestação de Serviços * `30` \| `WR` \| Warrant * `31` \| `DP` \| Duplicata Prestação * `32` \| `CSR` \| Cobrança Seriada * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Apólice Ramos Elementares * `35` \| `CC` \| Cartão de Crédito * `36` \| `BDP` \| Boleto de Proposta * `37` \| `NPD` \| Nota Promissória Direta * `38` \| `DAE` \| Dívida Ativa de Estado * `39` \| `DAM` \| Divida Ativa de Município * `40` \| `DAU` \| Dívida Ativa União * `41` \| `CCB` \| Célula de Crédito Bancário * `42` \| `FI` \| Financiamento * `43` \| `RD` \| Rateio de Despesas * `44` \| `DRI` \| Duplicata Rural p/ Indicação * `45` \| `ECI` \| Encargos Condominiais p/ Indicação * `99` \| `Outros` \| Outros Valores: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `id` | `integer` | não | ID do carnê |
| `bank_billet_layout_id` | `integer` | não | ID do Modelo de Boleto |
| `customer_id` | `integer` | não | ID do Cliente. Quando esse ID é passado, os campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` e `customer_neighborhood` não são obrigatórios. |
| `customer_address_complement` | `string` | não | Complemento |
| `customer_address_number` | `string` | não | Número |
| `customer_email` | `string (email)` | não | E-mail do Pagador |
| `customer_email_cc` | `string (email)` | não | E-mail alternativo do Pagador |
| `customer_phone_number` | `string` | não | Telefone (com DDD) |
| `customer_ignore_email` | `boolean` | não | Nunca enviar e-mail para este cliente |
| `customer_ignore_sms` | `boolean` | não | Nunca enviar SMS para este cliente |
| `customer_mobile_local_code` | `string` | não | DDD do Celular |
| `customer_mobile_number` | `string` | não | Celular |
| `customer_nickname` | `string` | não | Apelido ou Nome Fantasia do Pagador |
| `customer_notes` | `string` | não | Observações do Pagador |
| `customer_contact_person` | `string` | não | Contato |
| `customer_update` | `string` | não | Contato |
| `end_at` | `string (date)` | não | Data da última cobrança |
| `cycle` | `string` | não | Ciclo do carnê. Default: monthly * `biweekly` Quinzenal * `bimonthly` Bimestral * `monthly` Mensal * `quarterly` Trimestral * `semiannual` Semestral * `annual` Anual Valores: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `custom_bank_billets` | `array<object>` | não | Carnê customizado. Quando enviado, os valores de amount, start_at e end_at serão ignorados. |
| `bank_billet_ids` | `array<integer>` | não | IDs de boletos vinculados ao carnê |
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
| `days_for_discount` | `integer` | não | Dias para desconto. Obrigatório se discount_type é diferente de 0(zero), Não pode resultar numa data passada referente a criação. |
| `discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao desconto. Obrigatório se discount_type é igual a 2 |
| `discount_value` | `number` | não | Valor do desconto (R$). Obrigatório se discount_type é igual a 1. |
| `days_for_second_discount` | `integer` | não | Dias para segundo desconto. |
| `second_discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao segundo desconto. |
| `second_discount_value` | `number` | não | Valor do segundo desconto (R$). |
| `days_for_third_discount` | `integer` | não | Dias para terceiro desconto. |
| `third_discount_percentage` | `number` | não | Percentual do valor do boleto equivalente ao terceiro desconto. |
| `third_discount_value` | `number` | não | Valor do terceiro desconto (R$). |
| `tags` | `array<string>` | não | Tags associadas ao boleto |
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
| `document_number` | `string` | não | Número do Documento, Tamanho depende do banco e formato do arquivo Remessa: * Banco \|	Formato\| Tamanho \| * Abc \|`Cnab240`\|	15	\| * Ailos \|`Cnab240`\|	15  \| * Arbi \|`Cnab240`\|	15	\| * Banese \|`Cnab240`\|	15	\| * Banestes \|`Cnab400`\|	10	\| * Banrisul \|`Cnab240`\|	15	\| * Banrisul \|`Cnab400`\|	10	\| * Bb \|`Cnab240`\|	15	\| * Bb \|`Cnab400`\|	10	\| * Bib \|`Cnab240`\|	15	\| * Bnb \|`Cnab400`\|	10	\| * Bnpparibas \|`Cnab400`\|	10	\| * Bradesco \|`Cnab240`\|	15	\| * Bradesco \|`Cnab400`\|	10	\| * Brb \|`Cnab400`\|	10	\| * Caixa \|`Cnab240`\|	11	\| * Caixa \|`Cnab400`\|	10	\| * Caruana \|`Cnab400`\|	10	\| * Citibank \|`Cnab400`\|	10	\| * Credisis \|`Cnab240`\|	15	\| * Cresol \|`Cnab240`\|	10	\| * Cresol \|`Cnab400`\|	10	\| * Cresol Bradesco \|`Cnab240`\|	15	\| * Cresol Bradesco \|`Cnab400`\|	10	\| * Daycoval \|`Cnab400`\|	10	\| * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| * Mercantil \|`Cnab240`\|	10	\| * Moneyplus \|`Cnab240`\|	10	\| * Rendimento \|`Cnab400`\|	10	\| * Safra \|`Cnab400`\|	102	\| * Santander \|`Cnab240`\|	15	\| * Santander \|`Cnab400`\|	10	\| * Semear \|`Cnab400`\|	10	\| * Sicoob \|`Cnab240`\|	15	\| * Sicoob \|`Cnab400`\|	10	\| * Sicredi \|`Cnab240`\|	15	\| * Sicredi \|`Cnab400`\|	10	\| * Sofisa \|`Cnab240`\|	15	\| * Unicred \|`Cnab240`\|	15	\| * Uniprime \|`Cnab400`\|	10	\| * Uniprime99 \|`Cnab400`\|	10	\| * Santander \|`Cnab400`\|	10 |
| `days_for_revoke` | `integer` | não | Dias corridos para Baixa/Devolução: Nulo/Branco: Obedece ao padrão do banco. 0: Baixa/Devolução no mesmo dia do vencimento. 1 ou mais: Baixa/Devolução após o vencimento(Vencimento + X dias corridos). |
| `days_for_negativation` | `integer` | não | Dias corridos para Negativação: Disponível apenas para os seguintes bancos e formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sim \|	Sim	\| Não * Itaú \| Não \| Sim \| Não |
| `days_for_sue` | `integer` | não | Dias corridos para Protesto |
| `divergent_payment_type` | `integer` | não | Tipo de pagamento divergente: Válido apenas para Itaú e Caixa. * `1` Aceita qualquer valor divergente * `2` Aceita pagamentos dentro de uma faixa de valores ou percentuais * `3` Não aceita pagamento de valores divergentes * `4` Aceita pagamentos de valores superiores a um valor ou percentual mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | não | Tipo de valor a considerar para os limites de pagamentos: Válido apenas para Itaú e Caixa. * `1` Informa pagamentos divergentes por valores * `2` Informa pagamentos divergentes por percentuais Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | não | Valor máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_value` | `number` | não | Valor mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | não | Percentual máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | não | Percentual mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_limit` | `integer` | não | Quantidade de pagamentos permitida. Obrigatório se informados dados para pagamento divergente. Usado somente pela Caixa. |
| `prevent_registration` | `boolean` | não | Impedir envio de registro ao banco: Caso `true`, impede que o boleto seja registrado. Para ser usado nos casos em que o boleto já foi registrado fora da KOBANA mas deseja-se incluí-lo no sistema. |
| `notes` | `string` | não | Observações |
| `custom_attachment_name` | `string` | não | Nome para ser usado nos arquivos de boleto enviados para o cliente em notificações. Aceita uso de variáveis. Caso seja deixado vazio, o padrão é a palavra “boleto” acompanhada do ID. |
| `created_via_api` | `boolean` | não | Indica se o boleto foi criado por API |
| `created_at` | `string (date-time)` | não | Data e hora de criação do boleto |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do boleto |
| `payment_count` | `integer` | não | Quantidade de pagamentos parciais aceitos para este boleto. |
| `url` | `string` | não | URL para visualização do carnê |
| `status` | `string` | não | Situação do carnê: * `created` Gerando * `processed` Parcelas criadas * `generated` Aberto Valores: `created`, `processed`, `generated`. |
| `import_id` | `integer` | não | ID da Importação |
| `ignore_whatsapp` | `boolean` | não | Nunca enviar este carnê por WhatsApp |

Outros campos e objetos aninhados: `kobana schema banking.installments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Carnê Criado |
| `422` | Carnê inválido |

## installments get

Informações do Carnê

```text
kobana banking installments get [ID] [OPÇÕES]
```

`GET /v1/installments/{id}`

Consulta informações específicas de um Carnê.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Carnê encontrado com sucesso |
| `404` | Carnê não encontrado |

## installments delete

Excluir o Carnê

```text
kobana banking installments delete [ID] [OPÇÕES]
```

`DELETE /v1/installments/{id}`

Exclui um Carnê específico. Nenhum boleto gerado pela carnê será excluído.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Carnê cancelado |
| `404` | Carnê não encontrado |
