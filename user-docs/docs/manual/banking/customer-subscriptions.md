---
id: customer-subscriptions
title: "kobana banking customer-subscriptions"
sidebar_label: "customer-subscriptions"
---

# kobana banking customer-subscriptions


Comandos de `kobana banking customer-subscriptions`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`customer-subscriptions list`](#customer-subscriptions-list) | `GET /v1/customer_subscriptions` | Listar Assinaturas |
| [`customer-subscriptions create`](#customer-subscriptions-create) | `POST /v1/customer_subscriptions` | Criar uma Assinatura |
| [`customer-subscriptions get`](#customer-subscriptions-get) | `GET /v1/customer_subscriptions/{id}` | Informações da Assinatura |
| [`customer-subscriptions update`](#customer-subscriptions-update) | `PUT /v1/customer_subscriptions/{id}` | Atualizar a Assinatura |
| [`customer-subscriptions delete`](#customer-subscriptions-delete) | `DELETE /v1/customer_subscriptions/{id}` | Excluir a Assinatura |
| [`customer-subscriptions next-charge`](#customer-subscriptions-next-charge) | `POST /v1/customer_subscriptions/{id}/next_charge` | Gerar Próxima Cobrança |

## customer-subscriptions list

Listar Assinaturas

```text
kobana banking customer-subscriptions list [OPÇÕES]
```

`GET /v1/customer_subscriptions`

Lista assinaturas.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `customer_person_name` | `string` | não | Filtro por Nome/Razão Social. |
| `customer_cnpj_cpf` | `string` | não | Filtro por CPF/CNPJ da assinatura. Deve ser formatado com máscara. Acha todos os registros cujo campo seja igual ao string. |
| `customer_email` | `string` | não | Filtro por e-mail do cliente. Acha todos os registros cujo campo seja igual ao string. |
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de assinaturas |

## customer-subscriptions create

Criar uma Assinatura

```text
kobana banking customer-subscriptions create [OPÇÕES]
```

`POST /v1/customer_subscriptions`

Cria uma nova assinatura recorrente para cobrança automática de clientes com boletos bancários.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `customer_person_name` | `string` | sim | Nome do cliente |
| `customer_cnpj_cpf` | `string` | sim | CPF/CNPJ do cliente |
| `customer_zipcode` | `string` | sim | CEP (formato 99999999) |
| `customer_address` | `string` | sim | Endereço |
| `customer_city_name` | `string` | sim | Cidade(Nome deve estar correto e completo) |
| `customer_state` | `string` | sim | Estado |
| `customer_neighborhood` | `string` | sim | Bairro |
| `bank_billet_account_id` | `integer` | sim | ID da Carteira de Cobrança. Se não informado, usará a carteira padrão. |
| `amount` | `number` | sim | Valor da Assinatura (R$) |
| `id` | `integer` | não | ID da assinatura |
| `customer_id` | `integer` | não | ID do Cliente. Quando esse ID é passado, os campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` e `customer_neighborhood` não são obrigatórios. |
| `customer_nickname` | `string` | não | Apelido ou Nome Fantasia do Pagador |
| `customer_email` | `string (email)` | não | E-mail do Pagador |
| `customer_email_cc` | `string (email)` | não | E-mail alternativo do Pagador |
| `customer_address_number` | `string` | não | Número |
| `customer_address_complement` | `string` | não | Complemento |
| `customer_phone_number` | `string` | não | Telefone (com DDD, DDI é opcional) |
| `customer_person_type` | `string` | não | Tipo de Pagador. * `individual` Pessoa Física * `juridical` Pessoa Jurídica Valores: `individual`, `juridical`. |
| `customer_mobile_local_code` | `string` | não | DDD do Celular |
| `customer_mobile_number` | `string` | não | Celular |
| `customer_notes` | `string` | não | Observações do Pagador |
| `customer_ignore_email` | `boolean` | não | Nunca enviar e-mail para este cliente |
| `customer_ignore_sms` | `boolean` | não | Nunca enviar SMS para este cliente |
| `customer_contact_person` | `string` | não | Contato |
| `customer_update` | `string` | não | Contato |
| `cycle` | `string` | não | Ciclo da assinatura. Default: monthly * `biweekly` Quinzenal * `bimonthly` Bimestral * `monthly` Mensal * `quarterly` Trimestral * `semiannual` Semestral * `annual` Anual Valores: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `next_billing` | `string (date)` | não | Data da Primeira ou Próxima cobrança. Caso não seja enviado uma data, esse campo será calculado para ter o valor do dia da criação da assinatura mais o ciclo escolhido. Ex.: Mensal(Hoje + 30 dias) |
| `end_at` | `string (date)` | não | Data em que deseja parar as cobranças. Caso em branco, as cobranças serão geradas automaticamente até que se informe uma data ou se exclua a assinatura. |
| `description` | `string` | não | Descrição do produto ou serviço |
| `instructions` | `string` | não | Instruções para o Caixa |
| `days_in_advance` | `integer` | não | Com quantos dias de antecedência à data de vencimento a cobrança será gerada. Default: 7. |
| `fine_type` | `integer` | não | Tipo de multa: * `0` Inexistente (Padrão) * `1` Para percentual do valor do boleto * `2` Para valor fixo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | não | Quantidade de dias após o vencimento que a multa começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `fine_percentage` | `number` | não | Porcentagem de Multa por Atraso Ex: 2% x R$ 250,00 = R$ 5,00. Obrigatória se fine_type é igual a 1 |
| `fine_value` | `number` | não | Valor da multa (R$). Obrigatório se fine_type é igual a 2. |
| `interest_type` | `integer` | não | Tipo de juros/mora: * `0` Inexistente (Padrão) * `1` Para porcentagem diária * `2` Para valor diário Valores: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | não | Quantidade de dias após o vencimento que a mora começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `interest_percentage` | `number` | não | Porcentagem diária de juros. De 0.0 a 100.0 (Ex 1.5% = 1.5) Obrigatório se interest_type é igual a 1. |
| `interest_value` | `number` | não | Valor diário de juros (R$). Obrigatório se interest_type é igual a 2. |
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
| `bank_billet_layout_id` | `integer` | não | ID do Modelo de Boleto |
| `notes` | `string` | não | Observações |
| `tags` | `array<string>` | não | Tags associadas ao boleto |
| `bank_billet_ids` | `array<integer>` | não | IDs de boletos vinculados ao carnê |
| `prevent_registration` | `boolean` | não | Impedir envio de registro ao banco: Caso `true`, impede que o boleto seja registrado. Para ser usado nos casos em que o boleto já foi registrado fora da KOBANA mas deseja-se incluí-lo no sistema. |
| `divergent_payment_type` | `integer` | não | Tipo de pagamento divergente: Válido apenas para Itaú e Caixa. * `1` Aceita qualquer valor divergente * `2` Aceita pagamentos dentro de uma faixa de valores ou percentuais * `3` Não aceita pagamento de valores divergentes * `4` Aceita pagamentos de valores superiores a um valor ou percentual mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | não | Tipo de valor a considerar para os limites de pagamentos: Válido apenas para Itaú e Caixa. * `1` Informa pagamentos divergentes por valores * `2` Informa pagamentos divergentes por percentuais Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | não | Valor máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_value` | `number` | não | Valor mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | não | Percentual máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | não | Percentual mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_limit` | `integer` | não | Quantidade de pagamentos permitida. Obrigatório se informados dados para pagamento divergente. Usado somente pela Caixa. |
| `custom_attachment_name` | `string` | não | Nome para ser usado nos arquivos de boleto enviados para o cliente em notificações. Aceita uso de variáveis. Caso seja deixado vazio, o padrão é a palavra “boleto” acompanhada do ID. |
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
| `days_for_revoke` | `integer` | não | Dias corridos para Baixa/Devolução: Nulo/Branco: Obedece ao padrão do banco. 0: Baixa/Devolução no mesmo dia do vencimento. 1 ou mais: Baixa/Devolução após o vencimento(Vencimento + X dias corridos). |
| `days_for_negativation` | `integer` | não | Dias corridos para Negativação: Disponível apenas para os seguintes bancos e formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sim \|	Sim	\| Não * Itaú \| Não \| Sim \| Não |
| `days_for_sue` | `integer` | não | Dias corridos para Protesto |
| `payment_count` | `integer` | não | Quantidade de pagamentos parciais aceitos para este boleto. |
| `import_id` | `integer` | não | ID da Importação |
| `ignore_whatsapp` | `boolean` | não | Nunca enviar esta asinatura por WhatsApp |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.customer-subscriptions.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Assinatura criada |
| `422` | Assinatura inválida |

## customer-subscriptions get

Informações da Assinatura

```text
kobana banking customer-subscriptions get [ID] [OPÇÕES]
```

`GET /v1/customer_subscriptions/{id}`

Consulta informações específicas de uma Assinatura.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Assinatura encontrada |
| `404` | Assinatura não encontrado |

## customer-subscriptions update

Atualizar a Assinatura

```text
kobana banking customer-subscriptions update [ID] [OPÇÕES]
```

`PUT /v1/customer_subscriptions/{id}`

Atualiza informações específicas de uma Assinatura

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `customer_person_name` | `string` | sim | Nome do cliente |
| `customer_cnpj_cpf` | `string` | sim | CPF/CNPJ do cliente |
| `customer_zipcode` | `string` | sim | CEP (formato 99999999) |
| `customer_address` | `string` | sim | Endereço |
| `customer_city_name` | `string` | sim | Cidade(Nome deve estar correto e completo) |
| `customer_state` | `string` | sim | Estado |
| `customer_neighborhood` | `string` | sim | Bairro |
| `bank_billet_account_id` | `integer` | sim | ID da Carteira de Cobrança. Se não informado, usará a carteira padrão. |
| `amount` | `number` | sim | Valor da Assinatura (R$) |
| `id` | `integer` | não | ID da assinatura |
| `customer_id` | `integer` | não | ID do Cliente. Quando esse ID é passado, os campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` e `customer_neighborhood` não são obrigatórios. |
| `customer_nickname` | `string` | não | Apelido ou Nome Fantasia do Pagador |
| `customer_email` | `string (email)` | não | E-mail do Pagador |
| `customer_email_cc` | `string (email)` | não | E-mail alternativo do Pagador |
| `customer_address_number` | `string` | não | Número |
| `customer_address_complement` | `string` | não | Complemento |
| `customer_phone_number` | `string` | não | Telefone (com DDD, DDI é opcional) |
| `customer_person_type` | `string` | não | Tipo de Pagador. * `individual` Pessoa Física * `juridical` Pessoa Jurídica Valores: `individual`, `juridical`. |
| `customer_mobile_local_code` | `string` | não | DDD do Celular |
| `customer_mobile_number` | `string` | não | Celular |
| `customer_notes` | `string` | não | Observações do Pagador |
| `customer_ignore_email` | `boolean` | não | Nunca enviar e-mail para este cliente |
| `customer_ignore_sms` | `boolean` | não | Nunca enviar SMS para este cliente |
| `customer_contact_person` | `string` | não | Contato |
| `customer_update` | `string` | não | Contato |
| `cycle` | `string` | não | Ciclo da assinatura. Default: monthly * `biweekly` Quinzenal * `bimonthly` Bimestral * `monthly` Mensal * `quarterly` Trimestral * `semiannual` Semestral * `annual` Anual Valores: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `next_billing` | `string (date)` | não | Data da Primeira ou Próxima cobrança. Caso não seja enviado uma data, esse campo será calculado para ter o valor do dia da criação da assinatura mais o ciclo escolhido. Ex.: Mensal(Hoje + 30 dias) |
| `end_at` | `string (date)` | não | Data em que deseja parar as cobranças. Caso em branco, as cobranças serão geradas automaticamente até que se informe uma data ou se exclua a assinatura. |
| `description` | `string` | não | Descrição do produto ou serviço |
| `instructions` | `string` | não | Instruções para o Caixa |
| `days_in_advance` | `integer` | não | Com quantos dias de antecedência à data de vencimento a cobrança será gerada. Default: 7. |
| `fine_type` | `integer` | não | Tipo de multa: * `0` Inexistente (Padrão) * `1` Para percentual do valor do boleto * `2` Para valor fixo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | não | Quantidade de dias após o vencimento que a multa começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `fine_percentage` | `number` | não | Porcentagem de Multa por Atraso Ex: 2% x R$ 250,00 = R$ 5,00. Obrigatória se fine_type é igual a 1 |
| `fine_value` | `number` | não | Valor da multa (R$). Obrigatório se fine_type é igual a 2. |
| `interest_type` | `integer` | não | Tipo de juros/mora: * `0` Inexistente (Padrão) * `1` Para porcentagem diária * `2` Para valor diário Valores: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | não | Quantidade de dias após o vencimento que a mora começará a incidir. O valor default é 1 dia (o dia posterior ao vencimento). |
| `interest_percentage` | `number` | não | Porcentagem diária de juros. De 0.0 a 100.0 (Ex 1.5% = 1.5) Obrigatório se interest_type é igual a 1. |
| `interest_value` | `number` | não | Valor diário de juros (R$). Obrigatório se interest_type é igual a 2. |
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
| `bank_billet_layout_id` | `integer` | não | ID do Modelo de Boleto |
| `notes` | `string` | não | Observações |
| `tags` | `array<string>` | não | Tags associadas ao boleto |
| `bank_billet_ids` | `array<integer>` | não | IDs de boletos vinculados ao carnê |
| `prevent_registration` | `boolean` | não | Impedir envio de registro ao banco: Caso `true`, impede que o boleto seja registrado. Para ser usado nos casos em que o boleto já foi registrado fora da KOBANA mas deseja-se incluí-lo no sistema. |
| `divergent_payment_type` | `integer` | não | Tipo de pagamento divergente: Válido apenas para Itaú e Caixa. * `1` Aceita qualquer valor divergente * `2` Aceita pagamentos dentro de uma faixa de valores ou percentuais * `3` Não aceita pagamento de valores divergentes * `4` Aceita pagamentos de valores superiores a um valor ou percentual mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | não | Tipo de valor a considerar para os limites de pagamentos: Válido apenas para Itaú e Caixa. * `1` Informa pagamentos divergentes por valores * `2` Informa pagamentos divergentes por percentuais Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | não | Valor máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_value` | `number` | não | Valor mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | não | Percentual máximo para a faixa de pagamentos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | não | Percentual mínimo para a faixa de pagamentos divergentes. |
| `divergent_payment_limit` | `integer` | não | Quantidade de pagamentos permitida. Obrigatório se informados dados para pagamento divergente. Usado somente pela Caixa. |
| `custom_attachment_name` | `string` | não | Nome para ser usado nos arquivos de boleto enviados para o cliente em notificações. Aceita uso de variáveis. Caso seja deixado vazio, o padrão é a palavra “boleto” acompanhada do ID. |
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
| `days_for_revoke` | `integer` | não | Dias corridos para Baixa/Devolução: Nulo/Branco: Obedece ao padrão do banco. 0: Baixa/Devolução no mesmo dia do vencimento. 1 ou mais: Baixa/Devolução após o vencimento(Vencimento + X dias corridos). |
| `days_for_negativation` | `integer` | não | Dias corridos para Negativação: Disponível apenas para os seguintes bancos e formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sim \|	Sim	\| Não * Itaú \| Não \| Sim \| Não |
| `days_for_sue` | `integer` | não | Dias corridos para Protesto |
| `payment_count` | `integer` | não | Quantidade de pagamentos parciais aceitos para este boleto. |
| `import_id` | `integer` | não | ID da Importação |
| `ignore_whatsapp` | `boolean` | não | Nunca enviar esta asinatura por WhatsApp |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.customer-subscriptions.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Assinatura atualizado |
| `404` | Assinatura não encontrado |

## customer-subscriptions delete

Excluir a Assinatura

```text
kobana banking customer-subscriptions delete [ID] [OPÇÕES]
```

`DELETE /v1/customer_subscriptions/{id}`

Exclui uma Assinatura.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Assinatura excluída |
| `404` | Assinatura não encontrado |

## customer-subscriptions next-charge

Gerar Próxima Cobrança

```text
kobana banking customer-subscriptions next-charge [ID] [OPÇÕES]
```

`POST /v1/customer_subscriptions/{id}/next_charge`

Gera a próxima cobrança.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Próxima cobrança gerada |
| `404` | Assinatura não encontrado |
