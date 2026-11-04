---
id: transfer
title: "kobana banking transfer"
sidebar_label: "transfer"
description: "Transferências (Pix, TED, interna)"
---

# kobana banking transfer

Transferências (Pix, TED, interna)

Comandos de `kobana banking transfer`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`transfer accounts list`](#transfer-accounts-list) | `GET /v2/transfer/accounts` | Listar Contas de Transferência |
| [`transfer accounts create`](#transfer-accounts-create) | `POST /v2/transfer/accounts` | Criar Conta de Transferência |
| [`transfer accounts get`](#transfer-accounts-get) | `GET /v2/transfer/accounts/{uid}` | Buscar Conta de Transferência |
| [`transfer accounts update`](#transfer-accounts-update) | `PATCH /v2/transfer/accounts/{uid}` | Atualizar Conta de Transferência |
| [`transfer batches list`](#transfer-batches-list) | `GET /v2/transfer/batches` | Listar Lotes de Transferência |
| [`transfer batches get`](#transfer-batches-get) | `GET /v2/transfer/batches/{uid}` | Visualizar um Lote de Transferência |
| [`transfer batches approve`](#transfer-batches-approve) | `PUT /v2/transfer/batches/{uid}/approve` | Aprovar um Lote de Transferência |
| [`transfer batches cancel`](#transfer-batches-cancel) | `PUT /v2/transfer/batches/{uid}/cancel` | Cancelar um Lote de Transferência |
| [`transfer batches reprove`](#transfer-batches-reprove) | `PUT /v2/transfer/batches/{uid}/reprove` | Reprovar um Lote de Transferência |
| [`transfer internal list`](#transfer-internal-list) | `GET /v2/transfer/internal` | Listar Transferências Entre Contas |
| [`transfer internal create`](#transfer-internal-create) | `POST /v2/transfer/internal` | Criar uma Transferência Entre Contas |
| [`transfer internal get`](#transfer-internal-get) | `GET /v2/transfer/internal/{uid}` | Visualizar uma Transferência Entre Contas |
| [`transfer internal approve`](#transfer-internal-approve) | `PUT /v2/transfer/internal/{uid}/approve` | Liberar (aprovar) uma Transferência interna |
| [`transfer internal cancel`](#transfer-internal-cancel) | `PUT /v2/transfer/internal/{uid}/cancel` | Cancelar uma Transferência Entre Contas |
| [`transfer internal reschedule`](#transfer-internal-reschedule) | `PUT /v2/transfer/internal/{uid}/reschedule` | Reagendar (mudar a data) uma Transferência interna |
| [`transfer internal-batches create`](#transfer-internal-batches-create) | `POST /v2/transfer/internal_batches` | Criar um Lote de Transferência Entre Contas |
| [`transfer pix list`](#transfer-pix-list) | `GET /v2/transfer/pix` | Listar Transferências Pix |
| [`transfer pix create`](#transfer-pix-create) | `POST /v2/transfer/pix` | Criar uma Transferência Pix |
| [`transfer pix get`](#transfer-pix-get) | `GET /v2/transfer/pix/{uid}` | Visualizar uma Transferência Pix |
| [`transfer pix approve`](#transfer-pix-approve) | `PUT /v2/transfer/pix/{uid}/approve` | Liberar (aprovar) uma Transferência Pix |
| [`transfer pix cancel`](#transfer-pix-cancel) | `PUT /v2/transfer/pix/{uid}/cancel` | Cancelar uma Transferência Pix |
| [`transfer pix reschedule`](#transfer-pix-reschedule) | `PUT /v2/transfer/pix/{uid}/reschedule` | Reagendar (mudar a data) uma Transferência Pix |
| [`transfer pix-batches create`](#transfer-pix-batches-create) | `POST /v2/transfer/pix_batches` | Criar um Lote de Transferência Pix |
| [`transfer ted list`](#transfer-ted-list) | `GET /v2/transfer/ted` | Listar Transferências TED |
| [`transfer ted create`](#transfer-ted-create) | `POST /v2/transfer/ted` | Criar uma Transferência TED |
| [`transfer ted get`](#transfer-ted-get) | `GET /v2/transfer/ted/{uid}` | Visualizar uma Transferência TED |
| [`transfer ted approve`](#transfer-ted-approve) | `PUT /v2/transfer/ted/{uid}/approve` | Liberar (aprovar) uma Transferência TED |
| [`transfer ted cancel`](#transfer-ted-cancel) | `PUT /v2/transfer/ted/{uid}/cancel` | Cancelar uma Transferência TED |
| [`transfer ted reschedule`](#transfer-ted-reschedule) | `PUT /v2/transfer/ted/{uid}/reschedule` | Reagendar (mudar a data) uma Transferência TED |
| [`transfer ted-batches create`](#transfer-ted-batches-create) | `POST /v2/transfer/ted_batches` | Criar um Lote de Transferência TED |

## transfer accounts list

Listar Contas de Transferência

```text
kobana banking transfer accounts list [OPÇÕES]
```

`GET /v2/transfer/accounts`

Lista todas as contas de transferência cadastradas na conta. Suporta paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `financial_account_uid` | `string` | não | Filtrar por UID da conta financeira |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Contas de Transferência acessadas com escopo read |

## transfer accounts create

Criar Conta de Transferência

```text
kobana banking transfer accounts create [OPÇÕES]
```

`POST /v2/transfer/accounts`

Cria uma nova conta de transferência associada a uma conta financeira. Use quando a conta financeira precisa de mais de uma conta de transferência. A conta é criada sem conexão bancária (`connection_uid` retorna `null`); associe uma conexão antes de habilitar remessas/registro. Requer escopo `write` + `transfer.accounts`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string` | sim | UUID da Conta Financeira que receberá a nova conta de transferência |
| `agreement_code` | `string` | não | Código do convênio (até 30 dígitos). Só os dígitos são gravados; string vazia vira `null`; valor não vazio sem nenhum dígito responde 422. |
| `external_id` | `string` | não | Identificador externo da conta de transferência |
| `remittance_enabled` | `boolean` | não | Habilita ou desabilita a geração de remessas EDI para esta conta. Se omitido na criação, herda automaticamente `true` quando a conta financeira já possui um EdiBox de pagamento ativo (carta EDI validada e habilitada); caso contrário, nasce `false`. Enviar `null` explicitamente equivale a `false` (desabilita e não aciona a herança automática). |
| `provider_data` | `object` | não | Atributos específicos do provider bancário. |

Outros campos e objetos aninhados: `kobana schema banking.transfer.accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conta de Transferência criada |
| `404` | Conta Financeira não encontrada |
| `422` | Erro de validação. `agreement_code` acima do limite. |

## transfer accounts get

Buscar Conta de Transferência

```text
kobana banking transfer accounts get [UID] [OPÇÕES]
```

`GET /v2/transfer/accounts/{uid}`

Retorna os detalhes de uma conta de transferência específica pelo UID

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta de Transferência acessada com escopo read |
| `404` | Conta de Transferência não encontrada |

## transfer accounts update

Atualizar Conta de Transferência

```text
kobana banking transfer accounts update [UID] [OPÇÕES]
```

`PATCH /v2/transfer/accounts/{uid}`

Atualiza atributos editáveis de uma conta de transferência. Aceita `remittance_enabled` (booleano, controla a geração de remessas EDI), `provider_data.service_point_number` (string, ponto de atendimento — por exemplo `numeroPa` em TED Sicoob), `provider_data.cooperative_cnpj` (string, CNPJ da cooperativa Sicoob) e `agreement_code` (string, número do convênio; só os dígitos são gravados, string vazia ou só com espaços vira `null`, valor não vazio sem nenhum dígito ou com mais de 30 dígitos retorna 422). O corpo aceita PATCH parcial: envie apenas os campos que deseja atualizar.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `uid` | `string` | não | UUID da conta de transferência |
| `financial_account_uid` | `string` | não | UUID da conta financeira associada |
| `connection_uid` | `string (uuid)` | não | Conexão bancária da conta. |
| `agreement_code` | `string` | não | Código do convênio (até 30 dígitos). Só os dígitos são gravados; string vazia vira `null`; valor não vazio sem nenhum dígito responde 422. |
| `provider_data` | `object` | não | Dados específicos do provider. Para Sicoob, expõe `service_point_number` e `cooperative_cnpj`. |
| `external_id` | `string` | não | Identificador externo da conta de transferência |
| `balance_validation_enabled_at` | `string (date-time)` | não | Data e hora em que a validação de saldo foi habilitada |
| `remittance_enabled_at` | `string (date-time)` | não | Data e hora em que a remessa EDI foi habilitada |
| `remittance_enabled` | `boolean` | não | Habilita ou desabilita a geração de remessas EDI para esta conta. |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.transfer.accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Atualiza o número do convênio (`agreement_code`) |
| `404` | Conta de Transferência não encontrada |
| `422` | Erro de validação. `remittance_enabled` deve ser verdadeiro ou falso. |

## transfer batches list

Listar Lotes de Transferência

```text
kobana banking transfer batches list [OPÇÕES]
```

`GET /v2/transfer/batches`

Listar Lotes de Transferência.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem dos lotes |

## transfer batches get

Visualizar um Lote de Transferência

```text
kobana banking transfer batches get [UID] [OPÇÕES]
```

`GET /v2/transfer/batches/{uid}`

Consulta informações específicas de um Lote de Transferência.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lote de Transferência encontrado |
| `404` | Lote de Transferência não encontrado |

## transfer batches approve

Aprovar um Lote de Transferência

```text
kobana banking transfer batches approve [UID] [OPÇÕES]
```

`PUT /v2/transfer/batches/{uid}/approve`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de aprovação para o Lote de Transferência.
  A resposta traz em `data` o comando de aprovação do lote. Esse comando é a autorização do lote: cada transferência apta (aguardando aprovação e com registro confirmado) é liberada separadamente na instituição financeira sob essa autorização, inclusive as que ficarem aptas depois desta chamada. Você pode consultar o comando a qualquer momento pelo `data.uid`.

  ## Idempotência

  Repetir a chamada enquanto a autorização estiver em pé (comando `pending` ou `confirmed`) e o lote ainda estiver aguardando aprovação responde `200` com o **mesmo** `data.uid` e libera apenas as transferências que ficaram aptas desde a chamada anterior.
  Um `data.uid` diferente só aparece quando a autorização anterior deixou de valer:
  - o lote foi reprovado antes da primeira liberação (o comando anterior fica `interrupted`);
  - a instituição financeira recusou a liberação de todas as transferências (o comando anterior fica `failed`).

  A resposta `422` com `Nenhuma transferência do lote está apta a ser aprovada.` ocorre quando não há autorização em pé e nenhuma transferência do lote pôde ser enviada (nenhuma transferência do lote está apta ou o provedor financeiro não oferece a operação via API).

  ## Aprovação não atômica

  Cada transferência é liberada separadamente. O comando vai a `confirmed` na primeira liberação; o `status` do lote só vai a `approved` quando todas as transferências forem liberadas. Acompanhe cada transferência pelo `status` dela.

  ## Controle por item

  Para liberar uma transferência por vez, use a aprovação da própria transferência: `PUT /api/v2/transfer/ted/{uid}/approve` ou `PUT /api/v2/transfer/pix/{uid}/approve`. Cada chamada cria um comando só para aquela transferência, que precisa estar apta. Uma aprovação do lote feita depois não duplica a liberação: transferências com aprovação já em andamento são ignoradas. Transferências internas não têm liberação por item via API.

  ## Eventos de Webhook

  - `transfer.transfer_batch.approve.requested`: quando a requisição é realizada.
  - `transfer.transfer_batch.approve.confirmed`: na **primeira** liberação, com as demais transferências ainda em processamento.
  - `transfer.transfer_batch.approve.failed`: quando a instituição financeira recusa a liberação de todas as transferências.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando de aprovação do lote |
| `404` | Lote de Transferência não encontrado |
| `422` | Não foi possível gerar o comando |

## transfer batches cancel

Cancelar um Lote de Transferência

```text
kobana banking transfer batches cancel [UID] [OPÇÕES]
```

`PUT /v2/transfer/batches/{uid}/cancel`

> **Cancelamento:** Este endpoint cancela o Lote de Transferência. O comportamento depende do estado de registro do lote.

  - **Não registrado no provedor:** O lote é cancelado imediatamente de forma local, sem interação com a instituição financeira. A resposta contém o lote atualizado.
  - **Registrado no provedor (API de cancelamento suportada):** Um Comando de cancelamento é criado e processado assincronamente. A resposta contém o Comando criado.
  - **Registrado no provedor (API de cancelamento não suportada):** Um Comando de cancelamento é criado por Transferência individualmente. A resposta contém o lote atualizado.

  O cancelamento é possível nos estados: `pending`, `awaiting_approval`, `confirmed` e `approved`.

  > ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cancelado com sucesso |
| `422` | Lote de Transferência não pode ser cancelado no estado atual |

## transfer batches reprove

Reprovar um Lote de Transferência

```text
kobana banking transfer batches reprove [UID] [OPÇÕES]
```

`PUT /v2/transfer/batches/{uid}/reprove`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de reprovação para o Lote de Transferência.
  A resposta traz em `data` o comando de reprovação do lote. Esse comando é a autorização da reprovação: cada transferência apta (aguardando aprovação e com registro confirmado) é rejeitada separadamente na instituição financeira sob essa autorização. A reprovação também interrompe as liberações de transferências ainda pendentes. Você pode consultar o comando a qualquer momento pelo `data.uid`.

  ## Idempotência

  Repetir a chamada enquanto a autorização estiver em pé (comando `pending` ou `confirmed`) e o lote ainda estiver aguardando aprovação responde `200` com o **mesmo** `data.uid` e rejeita apenas as transferências que ficaram aptas desde a chamada anterior.
  Um `data.uid` diferente só aparece quando a autorização anterior deixou de valer:
  - o lote foi aprovado antes da primeira rejeição (o comando anterior fica `interrupted`);
  - a instituição financeira recusou a rejeição de todas as transferências (o comando anterior fica `failed`).

  A resposta `422` com `Nenhuma transferência do lote está apta a ser reprovada.` ocorre quando não há autorização em pé e nenhuma transferência do lote pôde ser enviada (nenhuma transferência do lote está apta ou o provedor financeiro não oferece a operação via API).

  ## Reprovação não atômica

  Cada transferência é rejeitada separadamente. O comando vai a `confirmed` na primeira rejeição; o `status` do lote só vai a `reproved` quando todas as transferências forem rejeitadas. Se a instituição financeira recusar a rejeição de parte das transferências, o lote continua `awaiting_approval`. Acompanhe cada transferência pelo `status` dela.

  ## Eventos de Webhook

  - `transfer.transfer_batch.reprove.requested`: quando a requisição é realizada.
  - `transfer.transfer_batch.reprove.confirmed`: na **primeira** rejeição, com as demais transferências ainda em processamento.
  - `transfer.transfer_batch.reprove.failed`: quando a instituição financeira recusa a rejeição de todas as transferências.

  > ⚠️ **Atenção ao status Reprovado:** O status de Reprovado é terminal. Quando a instituição financeira cancelar as transferências, as mesmas terão seus status alterados para Reprovado.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando de reprovação do lote |
| `404` | Lote de Transferência não encontrado |
| `422` | Não foi possível gerar o comando |

## transfer internal list

Listar Transferências Entre Contas

```text
kobana banking transfer internal list [OPÇÕES]
```

`GET /v2/transfer/internal`

Listar Transferências Entre Contas.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem das transferências |

## transfer internal create

Criar uma Transferência Entre Contas

```text
kobana banking transfer internal create [OPÇÕES]
```

`POST /v2/transfer/internal`

Esse endpoint é utilizado para a geração de uma transferência Entre Contas.

**Pré-requisito**

Para emitir uma transferência é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de transferência que registra a operação é definida em `transfer_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `transfer.db.created` é disparado nesta operação.

> **Próximos passos:** Após criar uma transferência, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/transfer/internal_batches`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Quantia |
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `beneficiary` | `object` | sim | Dados do Recebedor |
| `internal` | `object` | sim | Dados bancários |
| `scheduled_to` | `string (date)` | não | Data de Agendamento |
| `transfer_purpose` | `string` | não | Objetivo da Transferência |
| `transfer_account_uid` | `string (uuid)` | não | Conta de transferência de origem (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à `financial_account_uid`. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |

Outros campos e objetos aninhados: `kobana schema banking.transfer.internal.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Transferência Entre Contas criada |
| `422` | Erro de validação |

## transfer internal get

Visualizar uma Transferência Entre Contas

```text
kobana banking transfer internal get [UID] [OPÇÕES]
```

`GET /v2/transfer/internal/{uid}`

Consulta informações específicas de uma Transferência Entre Contas.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Transferência Entre Contas encontrada |
| `404` | Transferência Entre Contas não encontrada |

## transfer internal approve

Liberar (aprovar) uma Transferência interna

```text
kobana banking transfer internal approve [UID] [OPÇÕES]
```

`PUT /v2/transfer/internal/{uid}/approve`

> **Processamento Assíncrono:** cria um comando de liberação (approve) para a Transferência aguardando aprovação.

> ⚠️ **Suporte por banco:** nenhum provedor financeiro suporta liberação de transferência interna via API no momento — a chamada responde `422` ("não possui suporte a esta operação via API para este provedor financeiro"). O endpoint existe para simetria e habilita o suporte automaticamente quando algum provedor o implementar.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `422` | Provedor sem suporte à liberação via API |

## transfer internal cancel

Cancelar uma Transferência Entre Contas

```text
kobana banking transfer internal cancel [UID] [OPÇÕES]
```

`PUT /v2/transfer/internal/{uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para a Transferência.
O cancelamento de uma Transferência é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, a Transferência tem seu status alterado para Cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `transfer.internal.cancel.requested` é disparado.
O nosso servidor então começa as tentativas de envio do cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então a transferência é marcada como cancelada em nosso banco de dados.
Neste momento o evento `transfer.internal.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. A transferência não poderá mais ser processada após o cancelamento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso |
| `422` | Transferência já foi confirmada, rejeitada ou cancelada |

## transfer internal reschedule

Reagendar (mudar a data) uma Transferência interna

```text
kobana banking transfer internal reschedule [UID] [OPÇÕES]
```

`PUT /v2/transfer/internal/{uid}/reschedule`

> **Processamento Assíncrono:** cria um comando de reagendamento (mudança da data de pagamento) para a Transferência.

> ⚠️ **Suporte por banco:** nenhum provedor financeiro suporta reagendamento de transferência interna via API no momento — a chamada responde `422` ("não possui suporte a esta operação via API para este provedor financeiro"). O endpoint existe para simetria e habilita o suporte automaticamente quando algum provedor o implementar.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `date` | `string (date)` | sim | Nova data de pagamento (YYYY-MM-DD). |

Outros campos e objetos aninhados: `kobana schema banking.transfer.internal.reschedule`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `422` | Provedor sem suporte ao reagendamento via API |

## transfer internal-batches create

Criar um Lote de Transferência Entre Contas

```text
kobana banking transfer internal-batches create [OPÇÕES]
```

`POST /v2/transfer/internal_batches`

Esse endpoint é utilizado para a geração de um lote de transferência Entre Contas.

**Pré-requisito**

Para emitir um lote de transferência é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de transferência aplicada ao lote é definida em `transfer_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `transfer.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de Transferência, o sistema cria um comando para registrá-lo na instituição financeira. Após o registro do lote ser confirmado na instituição, o evento `transfer.internal_batch.confirmed` é disparado.

O evento `transfer.db.created` também é disparado nesta operação caso o lote contenha uma nova transferência.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `transfers` | `array<anyOf>` | sim | Lista de Transferências Entre Contas novas ou existentes. |
| `transfer_account_uid` | `string (uuid)` | não | Conta de transferência aplicada a todo o lote (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à conta financeira do lote. |

Outros campos e objetos aninhados: `kobana schema banking.transfer.internal-batches.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Lote de Transferência Entre Contas criado |
| `422` | Erro de validação — external_id duplicado |

## transfer pix list

Listar Transferências Pix

```text
kobana banking transfer pix list [OPÇÕES]
```

`GET /v2/transfer/pix`

Listar Transferências Pix.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem das transferências |

## transfer pix create

Criar uma Transferência Pix

```text
kobana banking transfer pix create [OPÇÕES]
```

`POST /v2/transfer/pix`

Esse endpoint é utilizado para a geração de uma transferência Pix.

**Pré-requisito**

Para emitir uma transferência é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de transferência que registra a operação é definida em `transfer_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Recebedor**

`beneficiary.name` e `beneficiary.document_number` (CPF/CNPJ) são obrigatórios para qualquer `type`, inclusive `key`, e para qualquer banco. A chave Pix não dispensa esses dados; sem eles, a API responde 422.

O 422 vem com `param: "target"` e `detail: "Dados de destino Beneficiário não pode ficar em branco"`, o mesmo quando falta o nome ou o documento: o erro não traz um `param` por campo do recebedor.

**Eventos de Webhook**

O evento `transfer.db.created` é disparado nesta operação.

> **Próximos passos:** Após criar uma transferência, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/transfer/pix_batches`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Quantia |
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `beneficiary` | `object` | sim | Dados do Recebedor. Obrigatório para qualquer `type`, inclusive `key`, e para qualquer banco: a chave Pix não dispensa o nome nem o CPF/CNPJ do recebedor. |
| `type` | `string` | sim | Tipo do Pix `key`: Chave. `bank_account`: Dados bancários |
| `scheduled_to` | `string (date)` | não | Data de Agendamento |
| `transfer_purpose` | `string` | não | Objetivo da Transferência |
| `transfer_account_uid` | `string (uuid)` | não | Conta de transferência de origem (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à `financial_account_uid`. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |
| `key_type` | `string` | não | Tipo da Chave (Obrigatório se o campo `type` for `key`) `cpf`: CPF. `cnpj`: CNPJ. `random`: Aleatória. `email`: E-mail. `phone`: Celular |
| `key` | `string` | não | Chave Pix (Obrigatório se o campo `type` for `key`). Nome e CPF/CNPJ do recebedor continuam exigidos em `beneficiary` |
| `bank_account` | `object` | não | Dados bancários (Obrigatório se o campo `type` for `bank_account`) |
| `identifier` | `string` | não | Identificação do pagamento no comprovante do provedor financeiro. (Uso exclusivo do banco) Bancos que aceitam esse campo (via API): Itaú |

Outros campos e objetos aninhados: `kobana schema banking.transfer.pix.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Transferência Pix criada |
| `422` | Erro de validação |

## transfer pix get

Visualizar uma Transferência Pix

```text
kobana banking transfer pix get [UID] [OPÇÕES]
```

`GET /v2/transfer/pix/{uid}`

Consulta informações específicas de uma Transferência Pix.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Transferência Pix encontrada |
| `404` | Transferência Pix não encontrada |

## transfer pix approve

Liberar (aprovar) uma Transferência Pix

```text
kobana banking transfer pix approve [UID] [OPÇÕES]
```

`PUT /v2/transfer/pix/{uid}/approve`

> **Processamento Assíncrono:** cria um comando de liberação (approve) para a Transferência Pix aguardando aprovação.

> ⚠️ **Suporte por banco:** nem todo provedor financeiro suporta liberação via API. Quando não suportado, a API responde `422` ("não possui suporte a esta operação via API para este provedor financeiro").

> ℹ️ **Transferência de lote:** este endpoint libera uma transferência de um lote sem aprovar o lote inteiro (ver `PUT /api/v2/transfer/batches/{uid}/approve`).

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso |
| `422` | Transferência não pode ser liberada |

## transfer pix cancel

Cancelar uma Transferência Pix

```text
kobana banking transfer pix cancel [UID] [OPÇÕES]
```

`PUT /v2/transfer/pix/{uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para a Transferência.
O cancelamento de uma Transferência é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, a Transferência tem seu status alterado para Cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `transfer.pix.cancel.requested` é disparado.
O nosso servidor então começa as tentativas de envio do cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então a transferência é marcada como cancelada em nosso banco de dados.
Neste momento o evento `transfer.pix.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. A transferência não poderá mais ser processada após o cancelamento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso |
| `422` | Transferência já foi confirmada, rejeitada ou cancelada |

## transfer pix reschedule

Reagendar (mudar a data) uma Transferência Pix

```text
kobana banking transfer pix reschedule [UID] [OPÇÕES]
```

`PUT /v2/transfer/pix/{uid}/reschedule`

> **Processamento Assíncrono:** cria um comando de reagendamento (mudança da data de pagamento) para a Transferência Pix.

> ⚠️ **Suporte por banco:** nem todo provedor financeiro suporta reagendamento via API. Quando não suportado, a API responde `422` ("não possui suporte a esta operação via API para este provedor financeiro").

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `date` | `string (date)` | sim | Nova data de pagamento (YYYY-MM-DD). |

Outros campos e objetos aninhados: `kobana schema banking.transfer.pix.reschedule`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso |
| `422` | Transferência não pode ser reagendada |

## transfer pix-batches create

Criar um Lote de Transferência Pix

```text
kobana banking transfer pix-batches create [OPÇÕES]
```

`POST /v2/transfer/pix_batches`

Esse endpoint é utilizado para a geração de um lote de transferência pix.

**Pré-requisito**

Para emitir um lote de transferência é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de transferência aplicada ao lote é definida em `transfer_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `transfer.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de Transferência, o sistema cria um comando para registrá-lo na instituição financeira. Após o registro do lote ser confirmado na instituição, o evento `transfer.pix_batch.confirmed` é disparado.

O evento `transfer.db.created` também é disparado nesta operação caso o lote contenha uma nova transferência.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `transfers` | `array<anyOf>` | sim | Lista de Transferências Pix novas ou existentes. |
| `transfer_account_uid` | `string (uuid)` | não | Conta de transferência aplicada a todo o lote (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à conta financeira do lote. |

Outros campos e objetos aninhados: `kobana schema banking.transfer.pix-batches.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Lote de Transferência Pix criado |
| `422` | Erro de validação — external_id duplicado |

## transfer ted list

Listar Transferências TED

```text
kobana banking transfer ted list [OPÇÕES]
```

`GET /v2/transfer/ted`

Listar Transferências TED.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem das transferências |

## transfer ted create

Criar uma Transferência TED

```text
kobana banking transfer ted create [OPÇÕES]
```

`POST /v2/transfer/ted`

Esse endpoint é utilizado para a geração de uma transferência TED.

**Pré-requisito**

Para emitir uma transferência é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de transferência que registra a operação é definida em `transfer_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `transfer.db.created` é disparado nesta operação.

> **Próximos passos:** Após criar uma transferência, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/transfer/ted_batches`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Quantia |
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `scheduled_to` | `string (date)` | não | Data de Agendamento |
| `transfer_purpose` | `string` | não | Objetivo da Transferência |
| `transfer_account_uid` | `string (uuid)` | não | Conta de transferência de origem (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à `financial_account_uid`. |
| `external_id` | `string` | não | ID no seu sistema. Opcional para controle interno. |
| `custom_data` | `object` | não | Hash com chave e valor no formato JSON. |
| `tags` | `array<string>` | não | Tags associadas |
| `beneficiary` | `object` | não | Dados do Recebedor (obrigatório, exceto em depósito judicial) |
| `bank_account` | `object` | não | Dados bancários (obrigatório, exceto em depósito judicial) |
| `judicial_deposit_number` | `string` | não | Número do depósito judicial (TED de depósito judicial, exclusivo Banco do Brasil — mutuamente exclusivo com `bank_account`) |
| `judicial_deposit_bank_code` | `string` | não | Código COMPE do banco de destino do depósito judicial (obrigatório com `judicial_deposit_number`). Use `001` para depósito no próprio Banco do Brasil |

Outros campos e objetos aninhados: `kobana schema banking.transfer.ted.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Transferência TED de depósito judicial criada, sem beneficiário (conta de origem Banco do Brasil) |
| `422` | TED sem bank_account e sem judicial_deposit_number (nenhum dos dois) |

## transfer ted get

Visualizar uma Transferência TED

```text
kobana banking transfer ted get [UID] [OPÇÕES]
```

`GET /v2/transfer/ted/{uid}`

Consulta informações específicas de uma Transferência TED.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Transferência TED encontrada |
| `404` | Transferência TED não encontrada |

## transfer ted approve

Liberar (aprovar) uma Transferência TED

```text
kobana banking transfer ted approve [UID] [OPÇÕES]
```

`PUT /v2/transfer/ted/{uid}/approve`

> **Processamento Assíncrono:** cria um comando de liberação (approve) para a Transferência aguardando aprovação.

A liberação é processada assincronamente: a API guarda a solicitação, cria um Comando e responde com o ID dele. Após confirmado na instituição financeira, a Transferência é liberada.

> ⚠️ **Suporte por banco:** nem todo provedor financeiro suporta liberação via API. Quando não suportado, a API responde `422` com a mensagem de "não possui suporte a esta operação via API para este provedor financeiro".

> ℹ️ **Transferência de lote:** este endpoint libera uma transferência de um lote sem aprovar o lote inteiro (ver `PUT /api/v2/transfer/batches/{uid}/approve`).

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso |
| `422` | Transferência não pode ser liberada |

## transfer ted cancel

Cancelar uma Transferência TED

```text
kobana banking transfer ted cancel [UID] [OPÇÕES]
```

`PUT /v2/transfer/ted/{uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para a Transferência.
O cancelamento de uma Transferência é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, a Transferência tem seu status alterado para Cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `transfer.ted.cancel.requested` é disparado.
O nosso servidor então começa as tentativas de envio do cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então a transferência é marcada como cancelada em nosso banco de dados.
Neste momento o evento `transfer.ted.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. A transferência não poderá mais ser processada após o cancelamento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso |
| `422` | Transferência já foi confirmada, rejeitada ou cancelada |

## transfer ted reschedule

Reagendar (mudar a data) uma Transferência TED

```text
kobana banking transfer ted reschedule [UID] [OPÇÕES]
```

`PUT /v2/transfer/ted/{uid}/reschedule`

> **Processamento Assíncrono:** cria um comando de reagendamento (mudança da data de pagamento) para a Transferência.

> ⚠️ **Suporte por banco:** nem todo provedor financeiro suporta reagendamento via API. Quando não suportado, a API responde `422` ("não possui suporte a esta operação via API para este provedor financeiro").

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `date` | `string (date)` | sim | Nova data de pagamento (YYYY-MM-DD). |

Outros campos e objetos aninhados: `kobana schema banking.transfer.ted.reschedule`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso |
| `422` | Transferência não pode ser reagendada |

## transfer ted-batches create

Criar um Lote de Transferência TED

```text
kobana banking transfer ted-batches create [OPÇÕES]
```

`POST /v2/transfer/ted_batches`

Esse endpoint é utilizado para a geração de um lote de transferência TED.

**Pré-requisito**

Para emitir um lote de transferência é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de transferência aplicada ao lote é definida em `transfer_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Eventos de Webhook**

O evento `transfer.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de Transferência, o sistema cria um comando para registrá-lo na instituição financeira. Após o registro do lote ser confirmado na instituição, o evento `transfer.ted_batch.confirmed` é disparado.

O evento `transfer.db.created` também é disparado nesta operação caso o lote contenha uma nova transferência.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sim | UID da Conta Financeira de Origem |
| `transfers` | `array<anyOf>` | sim | Lista de Transferências TED novas ou existentes. |
| `transfer_account_uid` | `string (uuid)` | não | Conta de transferência aplicada a todo o lote (opcional). Se omitida, usa-se a mais antiga da conta financeira. Se informada, deve pertencer à conta financeira do lote. |

Outros campos e objetos aninhados: `kobana schema banking.transfer.ted-batches.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Lote de Transferência TED com item de depósito judicial (exclusivo Banco do Brasil) |
| `422` | Erro de validação — external_id duplicado |
