---
id: payment
title: "kobana banking payment"
sidebar_label: "payment"
description: "Pagamentos (boletos, Pix, taxas, concessionárias)"
---

# kobana banking payment

Pagamentos (boletos, Pix, taxas, concessionárias)

Comandos de `kobana banking payment`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`payment accounts list`](#payment-accounts-list) | `GET /v2/payment/accounts` | Listar Contas de Pagamento |
| [`payment accounts create`](#payment-accounts-create) | `POST /v2/payment/accounts` | Criar Conta de Pagamento |
| [`payment accounts get`](#payment-accounts-get) | `GET /v2/payment/accounts/{uid}` | Buscar Conta de Pagamento |
| [`payment accounts update`](#payment-accounts-update) | `PATCH /v2/payment/accounts/{uid}` | Atualizar Conta de Pagamento |
| [`payment bank-billet-batches create`](#payment-bank-billet-batches-create) | `POST /v2/payment/bank_billet_batches` | Criar um Lote de Pagamento de Boletos |
| [`payment bank-billets list`](#payment-bank-billets-list) | `GET /v2/payment/bank_billets` | Listar Pagamentos de Boleto |
| [`payment bank-billets create`](#payment-bank-billets-create) | `POST /v2/payment/bank_billets` | Criar um Pagamento de Boleto |
| [`payment bank-billets get`](#payment-bank-billets-get) | `GET /v2/payment/bank_billets/{uid}` | Visualizar um Pagamento de Boleto |
| [`payment bank-billets cancel`](#payment-bank-billets-cancel) | `PUT /v2/payment/bank_billets/{uid}/cancel` | Cancelar um Pagamento de Boleto |
| [`payment batches list`](#payment-batches-list) | `GET /v2/payment/batches` | Listar Lotes de Pagamentos |
| [`payment batches get`](#payment-batches-get) | `GET /v2/payment/batches/{uid}` | Visualizar um Lote de Pagamento |
| [`payment batches approve`](#payment-batches-approve) | `PUT /v2/payment/batches/{uid}/approve` | Aprovar um Lote de Pagamento |
| [`payment batches cancel`](#payment-batches-cancel) | `PUT /v2/payment/batches/{uid}/cancel` | Cancelar um Lote de Pagamento |
| [`payment batches reprove`](#payment-batches-reprove) | `PUT /v2/payment/batches/{uid}/reprove` | Reprovar um Lote de Pagamento |
| [`payment dda-accounts list`](#payment-dda-accounts-list) | `GET /v2/payment/dda_accounts` | Listar Contas de DDA |
| [`payment dda-accounts create`](#payment-dda-accounts-create) | `POST /v2/payment/dda_accounts` | Criar Conta de DDA |
| [`payment dda-accounts get`](#payment-dda-accounts-get) | `GET /v2/payment/dda_accounts/{uid}` | Buscar Conta de DDA |
| [`payment dda-accounts update`](#payment-dda-accounts-update) | `PUT /v2/payment/dda_accounts/{uid}` | Atualizar Conta de DDA |
| [`payment dda-accounts disable`](#payment-dda-accounts-disable) | `PUT /v2/payment/dda_accounts/{uid}/disable` | Desabilitar Conta de DDA |
| [`payment dda-accounts enable`](#payment-dda-accounts-enable) | `PUT /v2/payment/dda_accounts/{uid}/enable` | Habilitar Conta de DDA |
| [`payment dda-bank-billets list`](#payment-dda-bank-billets-list) | `GET /v2/payment/dda_bank_billets` | Listar Boletos DDA |
| [`payment dda-bank-billets get`](#payment-dda-bank-billets-get) | `GET /v2/payment/dda_bank_billets/{uid}` | Buscar Boleto DDA |
| [`payment dda-bank-billets delete`](#payment-dda-bank-billets-delete) | `DELETE /v2/payment/dda_bank_billets/{uid}` | Excluir Boleto DDA |
| [`payment dda-bank-billets reject`](#payment-dda-bank-billets-reject) | `PUT /v2/payment/dda_bank_billets/{uid}/reject` | Rejeitar Boleto DDA |
| [`payment dda-bank-billets release`](#payment-dda-bank-billets-release) | `PUT /v2/payment/dda_bank_billets/{uid}/release` | Liberar Boleto DDA para Pagamento |
| [`payment pix list`](#payment-pix-list) | `GET /v2/payment/pix` | Listar Pagamentos Pix |
| [`payment pix create`](#payment-pix-create) | `POST /v2/payment/pix` | Criar um Pagamento Pix |
| [`payment pix get`](#payment-pix-get) | `GET /v2/payment/pix/{uid}` | Visualizar um Pagamento Pix |
| [`payment pix cancel`](#payment-pix-cancel) | `PUT /v2/payment/pix/{uid}/cancel` | Cancelar um Pagamento Pix |
| [`payment pix-batches create`](#payment-pix-batches-create) | `POST /v2/payment/pix_batches` | Criar um Lote de Pagamento Pix |
| [`payment tax-batches create`](#payment-tax-batches-create) | `POST /v2/payment/tax_batches` | Criar Lote de Pagamentos de Tributo |
| [`payment taxes list`](#payment-taxes-list) | `GET /v2/payment/taxes` | Listar Pagamentos de Tributo |
| [`payment taxes create`](#payment-taxes-create) | `POST /v2/payment/taxes` | Criar Pagamento de Tributo |
| [`payment taxes get`](#payment-taxes-get) | `GET /v2/payment/taxes/{uid}` | Exibir Pagamento de Tributo |
| [`payment taxes cancel`](#payment-taxes-cancel) | `PUT /v2/payment/taxes/{uid}/cancel` | Cancelar um Pagamento de Tributo |
| [`payment utilities list`](#payment-utilities-list) | `GET /v2/payment/utilities` | Listar Pagamentos de Contas de Consumo |
| [`payment utilities create`](#payment-utilities-create) | `POST /v2/payment/utilities` | Criar um Pagamento de Conta de Consumo |
| [`payment utilities get`](#payment-utilities-get) | `GET /v2/payment/utilities/{uid}` | Visualizar um Pagamento de Conta de Consumo |
| [`payment utilities cancel`](#payment-utilities-cancel) | `PUT /v2/payment/utilities/{uid}/cancel` | Cancelar um Pagamento de Conta de Consumo |
| [`payment utility-batches create`](#payment-utility-batches-create) | `POST /v2/payment/utility_batches` | Criar um Lote de Pagamento de Contas de Consumo |

## payment accounts list

Listar Contas de Pagamento

```text
kobana banking payment accounts list [OPÇÕES]
```

`GET /v2/payment/accounts`

Lista todas as contas de pagamento cadastradas na conta. Suporta paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `financial_account_uid` | `string` | não | Filtrar por UID da conta financeira |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Contas de Pagamento acessadas com escopo read |

## payment accounts create

Criar Conta de Pagamento

```text
kobana banking payment accounts create [OPÇÕES]
```

`POST /v2/payment/accounts`

Cria uma nova conta de pagamento associada a uma conta financeira. Use quando a conta financeira precisa de mais de uma conta de pagamento (por exemplo, múltiplos convênios no mesmo banco). A conta é criada sem conexão bancária (`connection_uid` retorna `null`); associe uma conexão antes de habilitar remessas/registro. Requer escopo `write` + `payment.accounts`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string` | sim | UUID da Conta Financeira que receberá a nova conta de pagamento |
| `agreement_code` | `string` | não | Código do convênio (até 30 dígitos). Só os dígitos são gravados; string vazia vira `null`; valor não vazio sem nenhum dígito responde 422. |
| `external_id` | `string` | não | Identificador externo da conta de pagamento |
| `remittance_enabled` | `boolean` | não | Habilita ou desabilita a geração de remessas EDI para esta conta. Se omitido na criação, herda automaticamente `true` quando a conta financeira já possui um EdiBox de pagamento ativo (carta EDI validada e habilitada); caso contrário, nasce `false`. Enviar `null` explicitamente equivale a `false` (desabilita e não aciona a herança automática). |
| `provider_data` | `object` | não | Atributos específicos do provider bancário. |

Outros campos e objetos aninhados: `kobana schema banking.payment.accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conta de Pagamento criada |
| `404` | Conta Financeira não encontrada |
| `422` | Erro de validação. `agreement_code` acima do limite. |

## payment accounts get

Buscar Conta de Pagamento

```text
kobana banking payment accounts get [UID] [OPÇÕES]
```

`GET /v2/payment/accounts/{uid}`

Retorna os detalhes de uma conta de pagamento específica pelo UID

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de Pagamento acessada com escopo read |
| `404` | Conta de Pagamento não encontrada |

## payment accounts update

Atualizar Conta de Pagamento

```text
kobana banking payment accounts update [UID] [OPÇÕES]
```

`PATCH /v2/payment/accounts/{uid}`

Atualiza atributos editáveis de uma conta de pagamento. Aceita `remittance_enabled` (booleano, controla a geração de remessas EDI), `provider_data.service_point_number` (string, ponto de atendimento — por exemplo `numeroPa` no Sicoob), `provider_data.cooperative_cnpj` (string, CNPJ da cooperativa Sicoob) e `agreement_code` (string, número do convênio; só os dígitos são gravados, string vazia ou só com espaços vira `null`, valor não vazio sem nenhum dígito ou com mais de 30 dígitos retorna 422). O corpo aceita PATCH parcial: envie apenas os campos que deseja atualizar.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `uid` | `string` | não | UUID da conta de pagamento |
| `financial_account_uid` | `string` | não | UUID da conta financeira associada |
| `connection_uid` | `string (uuid)` | não | Conexão bancária da conta. |
| `agreement_code` | `string` | não | Código do convênio (até 30 dígitos). Só os dígitos são gravados; string vazia vira `null`; valor não vazio sem nenhum dígito responde 422. |
| `provider_data` | `object` | não | Dados específicos do provider. |
| `external_id` | `string` | não | Identificador externo da conta de pagamento |
| `balance_validation_enabled_at` | `string (date-time)` | não | Data e hora em que a validação de saldo foi habilitada |
| `remittance_enabled_at` | `string (date-time)` | não | Data e hora em que a remessa EDI foi habilitada |
| `remittance_enabled` | `boolean` | não | Habilita ou desabilita a geração de remessas EDI para esta conta. |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.payment.accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Atualiza o número do convênio (`agreement_code`) |
| `404` | Conta de Pagamento não encontrada |
| `422` | Erro de validação. `remittance_enabled` deve ser verdadeiro ou falso. |

## payment bank-billet-batches create

Criar um Lote de Pagamento de Boletos

```text
kobana banking payment bank-billet-batches create [OPÇÕES]
```

`POST /v2/payment/bank_billet_batches`

Esse endpoint é utilizado para a geração de um lote de pagamento.

**Pré-requisito**

Para emitir um lote de pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento aplicada ao lote é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de Pagamento, o sistema cria um comando para registrá-lo na instituição financeira. Após o registro do lote ser confirmado na instituição, o evento `payment.bank_billet_batch.confirmed` é disparado.

O evento `payment.db.created` também é disparado nesta operação caso o lote contenha um novo pagamento.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `payments` | `array<anyOf>` | sim | Lista de Pagamentos de Boletos novos ou existentes. |
| `payment_account_uid` | `string (uuid)` | não | Conta de pagamento aplicada a todo o lote (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à conta financeira do lote. |

Outros campos e objetos aninhados: `kobana schema banking.payment.bank-billet-batches.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Lote de Pagamento criado |
| `422` | Erro de validação — external_id duplicado |

## payment bank-billets list

Listar Pagamentos de Boleto

```text
kobana banking payment bank-billets list [OPÇÕES]
```

`GET /v2/payment/bank_billets`

Lista todos os pagamentos de boletos bancários realizados. Suporta paginação e retorna informações detalhadas sobre cada pagamento.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamentos encontrados |

## payment bank-billets create

Criar um Pagamento de Boleto

```text
kobana banking payment bank-billets create [OPÇÕES]
```

`POST /v2/payment/bank_billets`

Esse endpoint é utilizado para a geração de um pagamento.

**Pré-requisito**

Para emitir um pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento que registra a operação é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.db.created` é disparado nesta operação.

> **Próximos passos:** Após criar um pagamento, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/payment/bank_billet_batches`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Quantia |
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `code` | `string` | sim | Código de Barras ou Linha Digitável |
| `scheduled_to` | `string (date)` | não | Data de Agendamento |
| `payment_account_uid` | `string (uuid)` | não | Conta de pagamento de origem (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à `financial_account_uid`. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |
| `transfer_purpose` | `string` | não | Objetivo do pagamento. Define o tipo de serviço do lote na remessa enviada ao banco. Opcional. Quando não informado, a remessa sai como `98` (Pagamentos Diversos). Informe apenas se o banco exigir um tipo de lote específico: o Itaú, por exemplo, recusa remessas cujo tipo de lote a empresa não tenha contratado. Pagamentos com objetivos diferentes são enviados em remessas separadas, porque o tipo de serviço descreve o lote inteiro. Em pagamento, hoje só o Itaú lê este campo na remessa. Nos demais bancos o tipo do lote é fixo: o campo é aceito e devolvido na resposta, mas não altera o arquivo enviado. `20`: Pagamento Fornecedor. `98`: Pagamentos Diversos Valores: `20`, `98`. |
| `beneficiary` | `object` | não | Dados do Beneficiário. Obrigatório quando a conta financeira for BB; caso contrário, opcional — quando informado, substitui os dados extraídos do código de barras. |

Outros campos e objetos aninhados: `kobana schema banking.payment.bank-billets.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pagamento criado |
| `422` | Erro de validação |

## payment bank-billets get

Visualizar um Pagamento de Boleto

```text
kobana banking payment bank-billets get [UID] [OPÇÕES]
```

`GET /v2/payment/bank_billets/{uid}`

Consulta informações específicas de um Pagamento de Boleto.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento encontrado |
| `404` | Pagamento não encontrado |

## payment bank-billets cancel

Cancelar um Pagamento de Boleto

```text
kobana banking payment bank-billets cancel [UID] [OPÇÕES]
```

`PUT /v2/payment/bank_billets/{uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para o Pagamento.
O cancelamento de um Pagamento é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, o Pagamento tem seu status alterado para Cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `payment.bank_billet.cancel.requested` é disparado.
O nosso servidor então começa as tentativas de envio do cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então o pagamento é marcado como cancelado em nosso banco de dados.
Neste momento o evento `payment.bank_billet.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. O pagamento não poderá mais ser processado após o cancelamento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento cancelado sem interação com provedor |
| `422` | Pagamento já foi confirmado, rejeitado ou cancelado |

## payment batches list

Listar Lotes de Pagamentos

```text
kobana banking payment batches list [OPÇÕES]
```

`GET /v2/payment/batches`

Retorna uma lista paginada de todos os lotes de pagamentos da conta. Os lotes são retornados em ordem decrescente de data de criação, com os mais recentes primeiro. Este endpoint permite filtrar e paginar os resultados para facilitar a navegação através de grandes volumes de lotes.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lotes encontrados |

## payment batches get

Visualizar um Lote de Pagamento

```text
kobana banking payment batches get [UID] [OPÇÕES]
```

`GET /v2/payment/batches/{uid}`

Consulta informações específicas de um Pagamento de Boleto.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lote de Pagamento encontrado |
| `404` | Lote de Pagamento não encontrado |

## payment batches approve

Aprovar um Lote de Pagamento

```text
kobana banking payment batches approve [UID] [OPÇÕES]
```

`PUT /v2/payment/batches/{uid}/approve`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de aprovação para o Lote de Pagamento.
  A resposta traz em `data` o comando de aprovação do lote. Esse comando é a autorização do lote: cada pagamento apto (aguardando aprovação e com registro confirmado) é liberado separadamente na instituição financeira sob essa autorização, inclusive os que ficarem aptos depois desta chamada. Você pode consultar o comando a qualquer momento pelo `data.uid`.

  ## Idempotência

  Repetir a chamada enquanto a autorização estiver em pé (comando `pending` ou `confirmed`) e o lote ainda estiver aguardando aprovação responde `200` com o **mesmo** `data.uid` e libera apenas os pagamentos que ficaram aptos desde a chamada anterior.
  Um `data.uid` diferente só aparece quando a autorização anterior deixou de valer:
  - o lote foi reprovado antes da primeira liberação (o comando anterior fica `interrupted`);
  - a instituição financeira recusou a liberação de todos os pagamentos (o comando anterior fica `failed`).

  A resposta `422` com `Nenhum pagamento do lote está apto a ser aprovado.` ocorre quando não há autorização em pé e nenhum pagamento do lote pôde ser enviado (nenhum pagamento do lote está apto ou o provedor financeiro não oferece a operação via API).

  ## Aprovação não atômica

  Cada pagamento é liberado separadamente. O comando vai a `confirmed` na primeira liberação; o `status` do lote só vai a `approved` quando todos os pagamentos forem liberados. Acompanhe cada pagamento pelo `status` dele.

  ## Controle por item

  Para liberar um pagamento por vez, use a aprovação do próprio pagamento: `PUT /api/v2/payments/{uid}/approve`. Cada chamada cria um comando só para aquele pagamento, que precisa estar apto. Uma aprovação do lote feita depois não duplica a liberação: pagamentos com aprovação já em andamento são ignorados.

  ## Eventos de Webhook

  - `payment.payment_batch.approve.requested`: quando a requisição é realizada.
  - `payment.payment_batch.approve.confirmed`: na **primeira** liberação, com os demais pagamentos ainda em processamento.
  - `payment.payment_batch.approve.failed`: quando a instituição financeira recusa a liberação de todos os pagamentos.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando de aprovação do lote |
| `404` | Lote de Pagamento não encontrado |
| `422` | Não foi possível gerar o comando |

## payment batches cancel

Cancelar um Lote de Pagamento

```text
kobana banking payment batches cancel [UID] [OPÇÕES]
```

`PUT /v2/payment/batches/{uid}/cancel`

> **Processamento Assíncrono:** cria um comando de cancelamento para o Lote de Pagamento.

  Quando o provedor financeiro expõe API de cancelamento de lote (ex.: Banco do Brasil),
  é criado 1 Comando de cancelamento no lote. Para provedores sem API de lote, o
  cancelamento é feito pagamento a pagamento (fallback per-item). Antes do registro, o
  lote é cancelado localmente sem interação com provedor.

  ## Eventos de Webhook

  Quando a requisição é realizada, um evento `payment.{type}_batch.cancel.requested` é disparado.
  Somente depois de confirmado na instituição financeira o lote é marcado como cancelado,
  disparando `payment.{type}_batch.cancel.confirmed`.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando de cancelamento criado com sucesso |
| `422` | Lote de Pagamento não pode ser cancelado no estado atual |

## payment batches reprove

Reprovar um Lote de Pagamento

```text
kobana banking payment batches reprove [UID] [OPÇÕES]
```

`PUT /v2/payment/batches/{uid}/reprove`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de reprovação para o Lote de Pagamento.
  A resposta traz em `data` o comando de reprovação do lote. Esse comando é a autorização da reprovação: cada pagamento apto (aguardando aprovação e com registro confirmado) é rejeitado separadamente na instituição financeira sob essa autorização. A reprovação também interrompe as liberações de pagamentos ainda pendentes. Você pode consultar o comando a qualquer momento pelo `data.uid`.

  ## Idempotência

  Repetir a chamada enquanto a autorização estiver em pé (comando `pending` ou `confirmed`) e o lote ainda estiver aguardando aprovação responde `200` com o **mesmo** `data.uid` e rejeita apenas os pagamentos que ficaram aptos desde a chamada anterior.
  Um `data.uid` diferente só aparece quando a autorização anterior deixou de valer:
  - o lote foi aprovado antes da primeira rejeição (o comando anterior fica `interrupted`);
  - a instituição financeira recusou a rejeição de todos os pagamentos (o comando anterior fica `failed`).

  A resposta `422` com `Nenhum pagamento do lote está apto a ser rejeitado.` ocorre quando não há autorização em pé e nenhum pagamento do lote pôde ser enviado (nenhum pagamento do lote está apto ou o provedor financeiro não oferece a operação via API).

  ## Reprovação não atômica

  Cada pagamento é rejeitado separadamente. O comando vai a `confirmed` na primeira rejeição; o `status` do lote só vai a `reproved` quando todos os pagamentos forem rejeitados. Se a instituição financeira recusar a rejeição de parte dos pagamentos, o lote continua `awaiting_approval`. Acompanhe cada pagamento pelo `status` dele.

  ## Eventos de Webhook

  - `payment.payment_batch.reprove.requested`: quando a requisição é realizada.
  - `payment.payment_batch.reprove.confirmed`: na **primeira** rejeição, com os demais pagamentos ainda em processamento.
  - `payment.payment_batch.reprove.failed`: quando a instituição financeira recusa a rejeição de todos os pagamentos.

  > ⚠️ **Atenção ao status Reprovado:** O status de Reprovado é terminal. Quando a instituição financeira cancelar os pagamentos, os mesmos terão seus status alterados para Reprovado.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando de reprovação do lote |
| `404` | Lote de Pagamento não encontrado |
| `422` | Não foi possível gerar o comando |

## payment dda-accounts list

Listar Contas de DDA

```text
kobana banking payment dda-accounts list [OPÇÕES]
```

`GET /v2/payment/dda_accounts`

Lista todas as contas DDA cadastradas na conta. Suporta paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista vazia quando não há contas DDA |

## payment dda-accounts create

Criar Conta de DDA

```text
kobana banking payment dda-accounts create [OPÇÕES]
```

`POST /v2/payment/dda_accounts`

Cria uma nova conta DDA.

**Eventos de Webhook**

O evento `payment.dda.account.db.created` é disparado nesta operação.

Após a criação, o sistema gera automaticamente a carta DDA (PDF), disparando os eventos `payment.dda.account.letter.pending`, `payment.dda.account.letter.waiting_signatures` e `payment.dda.account.letter.signed` à medida que o fluxo de assinatura progride.

Após aprovação pelo Manager, os eventos `payment.dda.account.confirmed` ou `payment.dda.account.rejected` são disparados.

Após `confirmed`, o sistema inicia o registro da conta no banco de forma assíncrona, disparando `payment.dda.account.register.requested` e `payment.dda.account.register.confirmed` (ou `failed`/`error`) conforme a resposta do provider.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cnpj` | `string` | sim | CNPJ ou CPF da empresa/pessoa para a qual o DDA será habilitado. Deve ser um documento válido. Cada conta Kobana pode ter apenas uma conta DDA ativa. |
| `signers` | `array<object>` | não | Lista de signatários da carta de adesão ao DDA (mínimo 1, sem máximo). Cada signatário receberá um e-mail com link para assinatura digital. |
| `business_legal_name` | `string` | não | Razão social da empresa associada ao CNPJ da conta DDA. Campo opcional. |
| `phone_number` | `string` | não | Telefone de contato da empresa associada ao CNPJ da conta DDA. Campo opcional. |
| `address` | `object` | não | Endereço da empresa associada ao CNPJ da conta DDA. Campo opcional; todos os subcampos também são opcionais. |

Outros campos e objetos aninhados: `kobana schema banking.payment.dda-accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conta de DDA criada com sucesso |
| `422` | Subcampo de address com valor não escalar (array) |

## payment dda-accounts get

Buscar Conta de DDA

```text
kobana banking payment dda-accounts get [UID] [OPÇÕES]
```

`GET /v2/payment/dda_accounts/{uid}`

Retorna os detalhes de uma conta de DDA específica pelo UID.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de DDA encontrada |
| `404` | Conta de DDA não encontrada |

## payment dda-accounts update

Atualizar Conta de DDA

```text
kobana banking payment dda-accounts update [UID] [OPÇÕES]
```

`PUT /v2/payment/dda_accounts/{uid}`

Atualiza os signatários de uma conta DDA. Somente permitido quando a conta é editável (editable=true).

**Eventos de Webhook**

O evento `payment.dda.account.db.updated` é disparado nesta operação.

Se a conta possuía carta em `external_letter_status: waiting_signatures`, o sistema sincroniza os novos signatários com o provider de assinatura de forma assíncrona. Caso o provider reinicie o fluxo de assinatura, os eventos `payment.dda.account.letter.pending` e subsequentes serão disparados.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `signers` | `array<object>` | não | Lista de signatários da carta de adesão ao DDA (mínimo 1, sem máximo). Cada signatário receberá um e-mail com link para assinatura digital. |
| `business_legal_name` | `string` | não | Razão social da empresa associada ao CNPJ da conta DDA. Campo opcional. |
| `phone_number` | `string` | não | Telefone de contato da empresa associada ao CNPJ da conta DDA. Campo opcional. |
| `address` | `object` | não | Endereço da empresa associada ao CNPJ da conta DDA. Campo opcional; todos os subcampos também são opcionais. Substitui o hash inteiro salvo anteriormente — não faz merge parcial. |

Outros campos e objetos aninhados: `kobana schema banking.payment.dda-accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de DDA atualizada com sucesso |
| `422` | Conta não editável |

## payment dda-accounts disable

Desabilitar Conta de DDA

```text
kobana banking payment dda-accounts disable [UID] [OPÇÕES]
```

`PUT /v2/payment/dda_accounts/{uid}/disable`

Solicita a desabilitação de uma conta DDA existente. Não remove `enabled_at` diretamente.

**Eventos de Webhook**

O evento `payment.dda.account.db.updated` é disparado nesta operação, refletindo a mudança em `disable_requested_at`. A desabilitação em si só é confirmada de forma assíncrona, quando o provedor responde: `payment.dda.account.disabled` (sucesso) ou `payment.dda.account.failed` (falha).

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de DDA desabilitada com sucesso |
| `404` | Conta de DDA não encontrada |

## payment dda-accounts enable

Habilitar Conta de DDA

```text
kobana banking payment dda-accounts enable [UID] [OPÇÕES]
```

`PUT /v2/payment/dda_accounts/{uid}/enable`

Solicita a habilitação de uma conta DDA previamente desabilitada. Sujeito a período de cooldown de 21 minutos após desabilitação.

**Eventos de Webhook**

O evento `payment.dda.account.db.updated` é disparado nesta operação, refletindo a mudança em `enable_requested_at`. A habilitação em si só é confirmada de forma assíncrona, quando o provedor responde: `payment.dda.account.enabled` (sucesso) ou `payment.dda.account.failed` (falha/resposta incompleta do provedor).

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de DDA habilitada com sucesso |
| `404` | Conta de DDA não encontrada |
| `422` | Conta de DDA não pode ser habilitada sem carta assinada e status aprovado |

## payment dda-bank-billets list

Listar Boletos DDA

```text
kobana banking payment dda-bank-billets list [OPÇÕES]
```

`GET /v2/payment/dda_bank_billets`

Lista todos os boletos DDA vinculados à conta. Suporta paginação.

**Eventos de Webhook**

Boletos DDA chegam exclusivamente via sincronização automática (não há criação manual). Os eventos de webhook relacionados são:

- `payment.dda.bank_billet.db.created` — disparado quando um novo boleto é sincronizado do provider
- `payment.dda.bank_billet.external_status_changed` — disparado quando o status externo do boleto muda (ex: `open` → `paid`)

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista vazia quando não há boletos DDA |

## payment dda-bank-billets get

Buscar Boleto DDA

```text
kobana banking payment dda-bank-billets get [UID] [OPÇÕES]
```

`GET /v2/payment/dda_bank_billets/{uid}`

Retorna os detalhes de um boleto DDA específico pelo UID.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Boleto DDA encontrado |
| `404` | Boleto DDA não encontrado |

## payment dda-bank-billets delete

Excluir Boleto DDA

```text
kobana banking payment dda-bank-billets delete [UID] [OPÇÕES]
```

`DELETE /v2/payment/dda_bank_billets/{uid}`

Exclui (soft delete) um boleto DDA existente.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Boleto DDA excluído com sucesso |
| `404` | Boleto DDA não encontrado |

## payment dda-bank-billets reject

Rejeitar Boleto DDA

```text
kobana banking payment dda-bank-billets reject [UID] [OPÇÕES]
```

`PUT /v2/payment/dda_bank_billets/{uid}/reject`

Rejeita um boleto DDA com status `pending`. Boletos rejeitados não serão processados para pagamento.

**Eventos de Webhook**

O evento `payment.dda.bank_billet.db.updated` é disparado nesta operação com as alterações de status (`pending` → `rejected`) e o campo `rejected_at`. Este evento é disparado em qualquer atualização do boleto DDA, não apenas nesta operação.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Boleto DDA rejeitado com sucesso |
| `404` | Boleto DDA não encontrado |
| `422` | Transição de estado inválida |

## payment dda-bank-billets release

Liberar Boleto DDA para Pagamento

```text
kobana banking payment dda-bank-billets release [UID] [OPÇÕES]
```

`PUT /v2/payment/dda_bank_billets/{uid}/release`

Libera um boleto DDA com status `pending` para pagamento. Somente boletos com `external_status: open` podem ser liberados.

**Eventos de Webhook**

O evento `payment.dda.bank_billet.db.updated` é disparado nesta operação com as alterações de status (`pending` → `released`) e o campo `released_at`. Este evento é disparado em qualquer atualização do boleto DDA, não apenas nesta operação.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Boleto DDA liberado para pagamento com sucesso |
| `404` | Boleto DDA não encontrado |
| `422` | Transição de estado inválida |

## payment pix list

Listar Pagamentos Pix

```text
kobana banking payment pix list [OPÇÕES]
```

`GET /v2/payment/pix`

Lista todos os pagamentos realizados via Pix. Suporta paginação e retorna informações detalhadas sobre cada pagamento.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamentos encontrados |

## payment pix create

Criar um Pagamento Pix

```text
kobana banking payment pix create [OPÇÕES]
```

`POST /v2/payment/pix`

Esse endpoint é utilizado para a geração de um pagamento.

**Pré-requisito**

Para emitir um pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento que registra a operação é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.db.created` é disparado nesta operação.

> **Próximos passos:** Após criar um pagamento, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/payment/pix_batches`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `qrcode` | `string` | sim | QR Code Copia e Cola |
| `amount` | `number` | não | Quantia. Opcional, o valor será extraído automaticamente do `qrcode`. |
| `scheduled_to` | `string (date)` | não | Data de Agendamento |
| `payment_account_uid` | `string (uuid)` | não | Conta de pagamento de origem (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à `financial_account_uid`. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `transfer_purpose` | `string` | não | Objetivo do pagamento. Define o tipo de serviço do lote na remessa enviada ao banco. Opcional. Quando não informado, a remessa sai como `98` (Pagamentos Diversos). Informe apenas se o banco exigir um tipo de lote específico: o Itaú, por exemplo, recusa remessas cujo tipo de lote a empresa não tenha contratado. Pagamentos com objetivos diferentes são enviados em remessas separadas, porque o tipo de serviço descreve o lote inteiro. Em pagamento, hoje só o Itaú lê este campo na remessa. Nos demais bancos o tipo do lote é fixo: o campo é aceito e devolvido na resposta, mas não altera o arquivo enviado. No Itaú o lote de Pix QR-Code usa tipo de serviço fixo `20`: o valor informado é aceito e devolvido, mas não altera a remessa. `01`: Cobrança. `03`: Boleto de Pagamento Eletrônico. `04`: Conciliação Bancária. `05`: Débitos. `06`: Custódia de Cheques. `07`: Gestão de Caixa. `08`: Consulta/Informação Margem. `09`: Averbação da Consignação/Retenção. `10`: Pagamento Dividendos. `11`: Manutenção da Consignação. `12`: Consignação de Parcelas. `13`: Glosa da Consignação (INSS). `14`: Consulta de Tributos a pagar. `20`: Pagamento Fornecedor. `22`: Pagamento de Contas, Tributos e Impostos. `23`: Interoperabilidade entre Contas de Instituições de Pagamentos. `25`: Compror. `26`: Compror Rotativo. `29`: Alegação do Pagador. `30`: Pagamento Salários. `32`: Pagamento de honorários. `33`: Pagamento de bolsa auxílio. `34`: Pagamento de prebenda (remuneração a padres e sacerdotes). `40`: Vendor. `41`: Vendor a Termo. `50`: Pagamento Sinistros Segurados. `60`: Pagamento Despesas Viajante em Trânsito. `70`: Pagamento Autorizado. `75`: Pagamento Credenciados. `77`: Pagamento de Remuneração. `80`: Pagamento Representantes / Vendedores Autorizados. `90`: Pagamento Benefícios. `98`: Pagamentos Diversos Valores: `01`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `20`, `22`, `23`, `25`, `26`, `29`, `30`, `32`, `33`, `34`, `40`, `41`, `50`, `60`, `70`, `75`, `77`, `80`, `90`, `98`. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |
| `identifier` | `string` | não | Identificação do pagamento no comprovante do provedor financeiro. (Uso exclusivo do banco) Bancos que aceitam esse campo (via API): Itaú |
| `beneficiary` | `object` | não | Dados do beneficiário. |

Outros campos e objetos aninhados: `kobana schema banking.payment.pix.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pagamento criado |
| `422` | Erro de validação |

## payment pix get

Visualizar um Pagamento Pix

```text
kobana banking payment pix get [UID] [OPÇÕES]
```

`GET /v2/payment/pix/{uid}`

Consulta informações específicas de um Pagamento Pix.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento encontrado |
| `404` | Pagamento não encontrado |

## payment pix cancel

Cancelar um Pagamento Pix

```text
kobana banking payment pix cancel [UID] [OPÇÕES]
```

`PUT /v2/payment/pix/{uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para o Pagamento.
O cancelamento de um Pagamento é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, o Pagamento tem seu status alterado para Cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `payment.pix.cancel.requested` é disparado.
O nosso servidor então começa as tentativas de envio do cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então o pagamento é marcado como cancelado em nosso banco de dados.
Neste momento o evento `payment.pix.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. O pagamento não poderá mais ser processado após o cancelamento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento cancelado sem interação com provedor |
| `422` | Pagamento já foi confirmado, rejeitado ou cancelado |

## payment pix-batches create

Criar um Lote de Pagamento Pix

```text
kobana banking payment pix-batches create [OPÇÕES]
```

`POST /v2/payment/pix_batches`

Esse endpoint é utilizado para a geração de um lote de pagamento.

**Pré-requisito**

Para emitir um lote de pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento aplicada ao lote é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de Pagamento, o sistema cria um comando para registrá-lo na instituição financeira. Após o registro do lote ser confirmado na instituição, o evento `payment.pix_batch.confirmed` é disparado.

O evento `payment.db.created` também é disparado nesta operação caso o lote contenha um novo pagamento.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `payments` | `array<anyOf>` | sim | Lista de Pagamentos de Pix novos ou existentes. |
| `payment_account_uid` | `string (uuid)` | não | Conta de pagamento aplicada a todo o lote (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à conta financeira do lote. |

Outros campos e objetos aninhados: `kobana schema banking.payment.pix-batches.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Lote de Pagamento criado |
| `422` | Erro de validação — external_id duplicado |

## payment tax-batches create

Criar Lote de Pagamentos de Tributo

```text
kobana banking payment tax-batches create [OPÇÕES]
```

`POST /v2/payment/tax_batches`

Esse endpoint é utilizado para a geração de um lote de pagamento de tributos municipais, estaduais e federais.

> ⚠️ **Novidade em Desenvolvimento!** Esta API está em versão beta - isso significa que estamos constantemente melhorando e ajustando suas funcionalidades. Embora seja totalmente funcional, algumas coisas podem mudar conforme recebemos feedback dos desenvolvedores.
>
> Para dúvidas sobre esta funcionalidade, entre em contato conosco.

**Pré-requisito**

Para emitir um lote de pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento aplicada ao lote é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de Pagamento, o sistema cria um comando para registrá-lo na instituição financeira. Após o registro do lote ser confirmado na instituição, o evento `payment.tax_batch.confirmed` é disparado.

O evento `payment.db.created` também é disparado nesta operação caso o lote contenha um novo pagamento.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `payments` | `array<anyOf>` | sim | Lista de Pagamentos de Tributo novos ou existentes. |
| `payment_account_uid` | `string (uuid)` | não | Conta de pagamento aplicada a todo o lote (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à conta financeira do lote. |

Outros campos e objetos aninhados: `kobana schema banking.payment.tax-batches.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Lote criado com format=data (GRU) |
| `422` | Erro de validação — external_id duplicado |

## payment taxes list

Listar Pagamentos de Tributo

```text
kobana banking payment taxes list [OPÇÕES]
```

`GET /v2/payment/taxes`

Esse endpoint é utilizado para a geração de um pagamento de tributo municipal, estadual ou federal.

> ⚠️ **Novidade em Desenvolvimento!** Esta API está em versão beta - isso significa que estamos constantemente melhorando e ajustando suas funcionalidades. Embora seja totalmente funcional, algumas coisas podem mudar conforme recebemos feedback dos desenvolvedores.
>
> Para dúvidas sobre esta funcionalidade, entre em contato conosco.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamentos encontrados |

## payment taxes create

Criar Pagamento de Tributo

```text
kobana banking payment taxes create [OPÇÕES]
```

`POST /v2/payment/taxes`

**Pré-requisitos**

Para emitir um pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento que registra a operação é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Quantia |
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `kind` | `string` | sim | Tipo do Tributo Valores: `itbi`, `icms`, `iss`, `iptu`, `fgts`, `dare`, `darf`, `gru`, `gps`. |
| `scheduled_to` | `string (date)` | não | Data de Agendamento |
| `payment_account_uid` | `string (uuid)` | não | Conta de pagamento de origem (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à `financial_account_uid`. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `transfer_purpose` | `string` | não | Objetivo do pagamento. Define o tipo de serviço do lote na remessa enviada ao banco. Opcional. Quando não informado, a remessa sai como `98` (Pagamentos Diversos). Informe apenas se o banco exigir um tipo de lote específico: o Itaú, por exemplo, recusa remessas cujo tipo de lote a empresa não tenha contratado. Pagamentos com objetivos diferentes são enviados em remessas separadas, porque o tipo de serviço descreve o lote inteiro. Em pagamento, hoje só o Itaú lê este campo na remessa. Nos demais bancos o tipo do lote é fixo: o campo é aceito e devolvido na resposta, mas não altera o arquivo enviado. Quando `format` é `code`, apenas `20` e `98` são aceitos: o tributo com código de barras viaja no mesmo lote das contas de consumo. `01`: Cobrança. `03`: Boleto de Pagamento Eletrônico. `04`: Conciliação Bancária. `05`: Débitos. `06`: Custódia de Cheques. `07`: Gestão de Caixa. `08`: Consulta/Informação Margem. `09`: Averbação da Consignação/Retenção. `10`: Pagamento Dividendos. `11`: Manutenção da Consignação. `12`: Consignação de Parcelas. `13`: Glosa da Consignação (INSS). `14`: Consulta de Tributos a pagar. `20`: Pagamento Fornecedor. `22`: Pagamento de Contas, Tributos e Impostos. `23`: Interoperabilidade entre Contas de Instituições de Pagamentos. `25`: Compror. `26`: Compror Rotativo. `29`: Alegação do Pagador. `30`: Pagamento Salários. `32`: Pagamento de honorários. `33`: Pagamento de bolsa auxílio. `34`: Pagamento de prebenda (remuneração a padres e sacerdotes). `40`: Vendor. `41`: Vendor a Termo. `50`: Pagamento Sinistros Segurados. `60`: Pagamento Despesas Viajante em Trânsito. `70`: Pagamento Autorizado. `75`: Pagamento Credenciados. `77`: Pagamento de Remuneração. `80`: Pagamento Representantes / Vendedores Autorizados. `90`: Pagamento Benefícios. `98`: Pagamentos Diversos Valores: `01`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `20`, `22`, `23`, `25`, `26`, `29`, `30`, `32`, `33`, `34`, `40`, `41`, `50`, `60`, `70`, `75`, `77`, `80`, `90`, `98`. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |
| `format` | `string` | não | Formato do tributo. `code`: Informar código de barras/linha digitável (padrão). `data`: Informar campos estruturados (disponível para kinds: darf, gru, gps, fgts). Valores: `code`, `data`. |
| `code` | `string` | não | Código de Barras ou Linha Digitável do Tributo. Obrigatório quando format=code. |
| `data` | `object` | não | Campos estruturados do tributo. Obrigatório quando format=data. |

Outros campos e objetos aninhados: `kobana schema banking.payment.taxes.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pagamento criado com format=data (DARF) |
| `422` | Parâmetros inválidos |

## payment taxes get

Exibir Pagamento de Tributo

```text
kobana banking payment taxes get [UID] [OPÇÕES]
```

`GET /v2/payment/taxes/{uid}`

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento encontrado |
| `404` | Pagamento não encontrado |

## payment taxes cancel

Cancelar um Pagamento de Tributo

```text
kobana banking payment taxes cancel [UID] [OPÇÕES]
```

`PUT /v2/payment/taxes/{uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para o Pagamento.
O cancelamento de um Pagamento é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, o Pagamento tem seu status alterado para Cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `payment.tax.cancel.requested` é disparado.
O nosso servidor então começa as tentativas de envio do cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então o pagamento é marcado como cancelado em nosso banco de dados.
Neste momento o evento `payment.tax.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. O pagamento não poderá mais ser processado após o cancelamento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento cancelado sem interação com provedor |
| `422` | Pagamento já foi confirmado, rejeitado ou cancelado |

## payment utilities list

Listar Pagamentos de Contas de Consumo

```text
kobana banking payment utilities list [OPÇÕES]
```

`GET /v2/payment/utilities`

Lista todos os pagamentos de contas de consumo (água, luz, gás, telefone, etc.) realizados. Suporta paginação e retorna informações detalhadas.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamentos encontrados |

## payment utilities create

Criar um Pagamento de Conta de Consumo

```text
kobana banking payment utilities create [OPÇÕES]
```

`POST /v2/payment/utilities`

Esse endpoint é utilizado para a geração de um pagamento.

**Pré-requisito**

Para emitir um pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento que registra a operação é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.db.created` é disparado nesta operação.

> **Próximos passos:** Após criar um pagamento, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/payment/utility_batches`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Quantia |
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `code` | `string` | sim | Código de Barras ou Linha Digitável |
| `scheduled_to` | `string (date)` | não | Data de Agendamento |
| `payment_account_uid` | `string (uuid)` | não | Conta de pagamento de origem (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à `financial_account_uid`. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |
| `transfer_purpose` | `string` | não | Objetivo do pagamento. Define o tipo de serviço do lote na remessa enviada ao banco. Opcional. Quando não informado, a remessa sai como `98` (Pagamentos Diversos). Informe apenas se o banco exigir um tipo de lote específico: o Itaú, por exemplo, recusa remessas cujo tipo de lote a empresa não tenha contratado. Pagamentos com objetivos diferentes são enviados em remessas separadas, porque o tipo de serviço descreve o lote inteiro. Em pagamento, hoje só o Itaú lê este campo na remessa. Nos demais bancos o tipo do lote é fixo: o campo é aceito e devolvido na resposta, mas não altera o arquivo enviado. `20`: Pagamento Fornecedor. `98`: Pagamentos Diversos Valores: `20`, `98`. |

Outros campos e objetos aninhados: `kobana schema banking.payment.utilities.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pagamento criado |
| `422` | Erro de validação |

## payment utilities get

Visualizar um Pagamento de Conta de Consumo

```text
kobana banking payment utilities get [UID] [OPÇÕES]
```

`GET /v2/payment/utilities/{uid}`

Consulta informações específicas de um Pagamento de Conta de Consumo.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento encontrado |
| `404` | Pagamento não encontrado |

## payment utilities cancel

Cancelar um Pagamento de Conta de Consumo

```text
kobana banking payment utilities cancel [UID] [OPÇÕES]
```

`PUT /v2/payment/utilities/{uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para o Pagamento.
O cancelamento de um Pagamento é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, o Pagamento tem seu status alterado para Cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `payment.utility.cancel.requested` é disparado.
O nosso servidor então começa as tentativas de envio do cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então o pagamento é marcado como cancelado em nosso banco de dados.
Neste momento o evento `payment.utility.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. O pagamento não poderá mais ser processado após o cancelamento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pagamento cancelado sem interação com provedor |
| `422` | Pagamento já foi confirmado, rejeitado ou cancelado |

## payment utility-batches create

Criar um Lote de Pagamento de Contas de Consumo

```text
kobana banking payment utility-batches create [OPÇÕES]
```

`POST /v2/payment/utility_batches`

Esse endpoint é utilizado para a geração de um lote de pagamento.

**Pré-requisito**

Para emitir um lote de pagamento é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de pagamento aplicada ao lote é definida em `payment_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de Pagamento, o sistema cria um comando para registrá-lo na instituição financeira. Após o registro do lote ser confirmado na instituição, o evento `payment.utility_batch.confirmed` é disparado.

O evento `payment.db.created` também é disparado nesta operação caso o lote contenha um novo pagamento.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `payments` | `array<anyOf>` | sim | Lista de Pagamentos de Contas de Consumo novas ou existentes. |
| `payment_account_uid` | `string (uuid)` | não | Conta de pagamento aplicada a todo o lote (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à conta financeira do lote. |

Outros campos e objetos aninhados: `kobana schema banking.payment.utility-batches.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Lote de Pagamento criado |
| `422` | Erro de validação — external_id duplicado |
