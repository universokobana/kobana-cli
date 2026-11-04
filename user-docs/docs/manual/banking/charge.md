---
id: charge
title: "kobana banking charge"
sidebar_label: "charge"
description: "Cobranças (Pix, boletos, Pix automático)"
---

# kobana banking charge

Cobranças (Pix, boletos, Pix automático)

Comandos de `kobana banking charge`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`charge automatic-pix accounts list`](#charge-automatic-pix-accounts-list) | `GET /v2/charge/automatic_pix/accounts` | Lista todas as contas PIX automáticas |
| [`charge automatic-pix accounts create`](#charge-automatic-pix-accounts-create) | `POST /v2/charge/automatic_pix/accounts` | Criar nova conta PIX automática |
| [`charge automatic-pix accounts get`](#charge-automatic-pix-accounts-get) | `GET /v2/charge/automatic_pix/accounts/{uid}` | Visualizar conta PIX automática |
| [`charge automatic-pix accounts update`](#charge-automatic-pix-accounts-update) | `PATCH /v2/charge/automatic_pix/accounts/{uid}` | Atualizar conta PIX automática |
| [`charge automatic-pix accounts delete`](#charge-automatic-pix-accounts-delete) | `DELETE /v2/charge/automatic_pix/accounts/{uid}` | Excluir conta PIX automática |
| [`charge automatic-pix locations list`](#charge-automatic-pix-locations-list) | `GET /v2/charge/automatic_pix/locations` | Lista todas as Locations do Pix Automático |
| [`charge automatic-pix locations create`](#charge-automatic-pix-locations-create) | `POST /v2/charge/automatic_pix/locations` | Criar nova Location do PIX Automático |
| [`charge automatic-pix locations get`](#charge-automatic-pix-locations-get) | `GET /v2/charge/automatic_pix/locations/{uid}` | Consulta uma Location do PIX Automático específica |
| [`charge automatic-pix pix list`](#charge-automatic-pix-pix-list) | `GET /v2/charge/automatic_pix/pix` | Lista todas as cobranças PIX automáticas |
| [`charge automatic-pix pix get`](#charge-automatic-pix-pix-get) | `GET /v2/charge/automatic_pix/pix/{uid}` | Informações da cobrança PIX automática |
| [`charge automatic-pix pix update`](#charge-automatic-pix-pix-update) | `PATCH /v2/charge/automatic_pix/pix/{uid}` | Atualizar cobrança PIX automática |
| [`charge automatic-pix pix cancel`](#charge-automatic-pix-pix-cancel) | `PUT /v2/charge/automatic_pix/pix/{uid}/cancel` | Cancelar cobrança PIX automática |
| [`charge automatic-pix pix retry`](#charge-automatic-pix-pix-retry) | `PUT /v2/charge/automatic_pix/pix/{uid}/retry` | Retentar cobrança PIX automática |
| [`charge automatic-pix recurrences list`](#charge-automatic-pix-recurrences-list) | `GET /v2/charge/automatic_pix/recurrences` | Lista todas as recorrências PIX automáticas |
| [`charge automatic-pix recurrences create`](#charge-automatic-pix-recurrences-create) | `POST /v2/charge/automatic_pix/recurrences` | Criar nova recorrência PIX automática |
| [`charge automatic-pix recurrences pix`](#charge-automatic-pix-recurrences-pix) | `POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/pix` | Criar nova cobrança PIX automática |
| [`charge automatic-pix recurrences requests`](#charge-automatic-pix-recurrences-requests) | `POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/requests` | Criar nova solicitação PIX automática |
| [`charge automatic-pix recurrences get`](#charge-automatic-pix-recurrences-get) | `GET /v2/charge/automatic_pix/recurrences/{uid}` | Informações da recorrência PIX automática |
| [`charge automatic-pix recurrences update`](#charge-automatic-pix-recurrences-update) | `PATCH /v2/charge/automatic_pix/recurrences/{uid}` | Atualizar recorrência PIX automática |
| [`charge automatic-pix recurrences cancel`](#charge-automatic-pix-recurrences-cancel) | `PUT /v2/charge/automatic_pix/recurrences/{uid}/cancel` | Cancelar recorrência PIX automática |
| [`charge automatic-pix requests list`](#charge-automatic-pix-requests-list) | `GET /v2/charge/automatic_pix/requests` | Lista todas as solicitações PIX automáticas |
| [`charge automatic-pix requests get`](#charge-automatic-pix-requests-get) | `GET /v2/charge/automatic_pix/requests/{uid}` | Informações da solicitação PIX automática |
| [`charge automatic-pix requests update`](#charge-automatic-pix-requests-update) | `PATCH /v2/charge/automatic_pix/requests/{uid}` | Atualizar solicitação PIX automática |
| [`charge automatic-pix requests cancel`](#charge-automatic-pix-requests-cancel) | `PUT /v2/charge/automatic_pix/requests/{uid}/cancel` | Cancelar solicitação PIX automática |
| [`charge payments list`](#charge-payments-list) | `GET /v2/charge/payments` | Listar Recebimentos |
| [`charge payments create`](#charge-payments-create) | `POST /v2/charge/payments` | Informar Recebimento |
| [`charge payments get`](#charge-payments-get) | `GET /v2/charge/payments/{uid}` | Informações do Recebimento |
| [`charge payments delete`](#charge-payments-delete) | `DELETE /v2/charge/payments/{uid}` | Excluir Recebimento |
| [`charge pix commands get`](#charge-pix-commands-get) | `GET /v2/charge/pix/{pix_uid}/commands/{uid}` | Visualizar um Comando |
| [`charge pix commands list`](#charge-pix-commands-list) | `GET /v2/charge/pix/{pix_uid}/commands` | Listar Comandos de um Pix |
| [`charge pix list`](#charge-pix-list) | `GET /v2/charge/pix` | Listar Pix |
| [`charge pix create`](#charge-pix-create) | `POST /v2/charge/pix` | Criar um Pix |
| [`charge pix cancel`](#charge-pix-cancel) | `PUT /v2/charge/pix/{pix_uid}/cancel` | Cancelar um Pix |
| [`charge pix update`](#charge-pix-update) | `PUT /v2/charge/pix/{pix_uid}/update` | Atualizar um Pix |
| [`charge pix get`](#charge-pix-get) | `GET /v2/charge/pix/{uid}` | Visualizar um Pix |
| [`charge pix delete`](#charge-pix-delete) | `DELETE /v2/charge/pix/{uid}` | Excluir um Pix |
| [`charge pix-accounts list`](#charge-pix-accounts-list) | `GET /v2/charge/pix_accounts` | Listar Contas Pix |
| [`charge pix-accounts create`](#charge-pix-accounts-create) | `POST /v2/charge/pix_accounts` | Criar Pix Account |
| [`charge pix-accounts get`](#charge-pix-accounts-get) | `GET /v2/charge/pix_accounts/{uid}` | Visualizar uma conta Pix |
| [`charge pix-accounts update`](#charge-pix-accounts-update) | `PUT /v2/charge/pix_accounts/{uid}` | Atualizar Pix Account |
| [`charge pix-accounts delete`](#charge-pix-accounts-delete) | `DELETE /v2/charge/pix_accounts/{uid}` | Deletar Pix Account |

## charge automatic-pix accounts list

Lista todas as contas PIX automáticas

```text
kobana banking charge automatic-pix accounts list [OPÇÕES]
```

`GET /v2/charge/automatic_pix/accounts`

Lista todas as contas PIX automáticas do usuário com paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da página para paginação |
| `per_page` | `integer` | não | Quantidade de itens por página (padrão: 50, máximo: 50) |
| `external_id` | `string` | não | Filtrar por ID externo |
| `financial_account_uid` | `string` | não | Filtrar por UID da conta financeira |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de contas PIX automáticas filtrada por financial_account_uid |

## charge automatic-pix accounts create

Criar nova conta PIX automática

```text
kobana banking charge automatic-pix accounts create [OPÇÕES]
```

`POST /v2/charge/automatic_pix/accounts`

Cria uma nova conta PIX automática.

**Importante**

Após criar a conta, é necessário associar uma conexão a ela para que o PIX Automático funcione.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string` | sim | UID da conta financeira associada |
| `custom_name` | `string` | não | Nome customizado da conta |
| `external_id` | `string` | não | ID externo para integração |
| `enabled` | `boolean` | não | Define se a conta está habilitada. |
| `custom_data` | `object` | não | Dados customizados |
| `tags` | `array<string>` | não | Tags da conta |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conta PIX automática criada com sucesso |
| `422` | Dados inválidos |

## charge automatic-pix accounts get

Visualizar conta PIX automática

```text
kobana banking charge automatic-pix accounts get [UID] [OPÇÕES]
```

`GET /v2/charge/automatic_pix/accounts/{uid}`

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhes da conta PIX automática |
| `404` | Conta PIX automática não encontrada |

## charge automatic-pix accounts update

Atualizar conta PIX automática

```text
kobana banking charge automatic-pix accounts update [UID] [OPÇÕES]
```

`PATCH /v2/charge/automatic_pix/accounts/{uid}`

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `custom_name` | `string` | não | Nome customizado da conta |
| `external_id` | `string` | não | ID externo para integração |
| `enabled` | `boolean` | não | Define se a conta está habilitada. |
| `custom_data` | `object` | não | Dados customizados |
| `tags` | `array<string>` | não | Tags da conta |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta PIX automática atualizada |

## charge automatic-pix accounts delete

Excluir conta PIX automática

```text
kobana banking charge automatic-pix accounts delete [UID] [OPÇÕES]
```

`DELETE /v2/charge/automatic_pix/accounts/{uid}`

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta PIX automática excluída |
| `422` | Conta PIX automática possui registros associados (ex: recorrência) |

## charge automatic-pix locations list

Lista todas as Locations do Pix Automático

```text
kobana banking charge automatic-pix locations list [OPÇÕES]
```

`GET /v2/charge/automatic_pix/locations`

Lista todas as Locations do Pix Automático do usuário com paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da página para paginação |
| `per_page` | `integer` | não | Quantidade de itens por página (padrão: 50, máximo: 50) |
| `external_id` | `string` | não | Filtrar por ID externo |
| `charge_automatic_pix_account_uid` | `string` | não | Filtrar por UID da conta Pix Automático |
| `financial_provider_external_id` | `string` | não | Filtrar por ID externo do provedor financeiro |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de Locations do Pix Automático |

## charge automatic-pix locations create

Criar nova Location do PIX Automático

```text
kobana banking charge automatic-pix locations create [OPÇÕES]
```

`POST /v2/charge/automatic_pix/locations`

Cria uma nova Location do PIX Automático.

A resposta `201` vem antes do registro no banco: `financial_provider_external_id` e `location` chegam nulos e são preenchidos de forma assíncrona. Antes de usar o `uid` desta location numa recorrência, aguarde o webhook `charge.automatic_pix.location.register.confirmed` ou consulte `GET /v2/charge/automatic_pix/locations/{uid}` até `financial_provider_external_id` vir preenchido; antes disso, a recorrência é recusada com `Localização deve conter financial_provider_external_id preenchido`.

Se nenhum evento `charge.automatic_pix.location.register.*` chegar e o `GET` continuar sem `financial_provider_external_id`, contate o suporte.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `charge_automatic_pix_account_uid` | `string` | sim | UID da conta Pix Automático |
| `external_id` | `string` | não | ID externo da location |
| `tags` | `array<string>` | não | Tags para organização |
| `custom_data` | `object` | não | Dados customizados |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.locations.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Location do PIX Automático criada com sucesso |
| `422` | Dados inválidos para criação |

## charge automatic-pix locations get

Consulta uma Location do PIX Automático específica

```text
kobana banking charge automatic-pix locations get [UID] [OPÇÕES]
```

`GET /v2/charge/automatic_pix/locations/{uid}`

Retorna os detalhes de uma Location do PIX Automático específica.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Location do PIX Automático encontrada |
| `404` | Location do PIX Automático não encontrada |

## charge automatic-pix pix list

Lista todas as cobranças PIX automáticas

```text
kobana banking charge automatic-pix pix list [OPÇÕES]
```

`GET /v2/charge/automatic_pix/pix`

Lista todas as cobranças PIX automáticas com paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `automatic_pix_recurrence_uid` | `string` | não | Filtrar por UID da recorrência PIX automática |
| `external_id` | `string` | não | Filtrar por ID externo |
| `page` | `integer` | não | Número da página para paginação |
| `per_page` | `integer` | não | Quantidade de itens por página (padrão: 50, máximo: 50) |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de cobranças PIX automáticas |

## charge automatic-pix pix get

Informações da cobrança PIX automática

```text
kobana banking charge automatic-pix pix get [UID] [OPÇÕES]
```

`GET /v2/charge/automatic_pix/pix/{uid}`

Consulta informações específicas de uma cobrança PIX automática.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cobrança PIX automática encontrada |
| `404` | Cobrança PIX automática não encontrada |

## charge automatic-pix pix update

Atualizar cobrança PIX automática

```text
kobana banking charge automatic-pix pix update [UID] [OPÇÕES]
```

`PATCH /v2/charge/automatic_pix/pix/{uid}`

Atualiza informações de uma cobrança PIX automática existente.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `external_id` | `string` | não | ID externo da cobrança. |
| `tags` | `array<string>` | não | Tags. |
| `custom_data` | `object` | não | Dados personalizados. |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.pix.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cobrança PIX automática atualizada com sucesso |
| `404` | Cobrança PIX automática não encontrada |
| `422` | ID Externo duplicado ao atualizar |

## charge automatic-pix pix cancel

Cancelar cobrança PIX automática

```text
kobana banking charge automatic-pix pix cancel [UID] [OPÇÕES]
```

`PUT /v2/charge/automatic_pix/pix/{uid}/cancel`

Cancela uma cobrança PIX automática existente.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cobrança PIX automática cancelada com sucesso |
| `404` | Cobrança PIX automática não encontrada |
| `422` | Erro ao cancelar |

## charge automatic-pix pix retry

Retentar cobrança PIX automática

```text
kobana banking charge automatic-pix pix retry [UID] [OPÇÕES]
```

`PUT /v2/charge/automatic_pix/pix/{uid}/retry`

Retentar uma cobrança PIX automática que falhou ou expirou.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `date` | `string (date)` | sim | Data para retentar a cobrança. |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.pix.retry`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Cobrança PIX automática agendada para retentativa com sucesso |
| `404` | Cobrança PIX automática não encontrada |
| `422` | Data inválida |

## charge automatic-pix recurrences list

Lista todas as recorrências PIX automáticas

```text
kobana banking charge automatic-pix recurrences list [OPÇÕES]
```

`GET /v2/charge/automatic_pix/recurrences`

Lista todas as recorrências PIX automáticas do usuário com paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da página para paginação |
| `per_page` | `integer` | não | Quantidade de itens por página (padrão: 50, máximo: 50) |
| `external_id` | `string` | não | Filtrar por ID externo |
| `charge_automatic_pix_account_uid` | `string` | não | Filtrar por UID da conta Pix Automático |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de recorrências PIX automáticas |

## charge automatic-pix recurrences create

Criar nova recorrência PIX automática

```text
kobana banking charge automatic-pix recurrences create [OPÇÕES]
```

`POST /v2/charge/automatic_pix/recurrences`

Cria uma nova recorrência PIX automática.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `charge_automatic_pix_account_uid` | `string` | sim | UID da conta Pix Automático. A conta financeira e a connection são herdadas desta conta. |
| `document` | `string` | sim | Número, identificador, ou código que representa o objeto da autorização (contrato, pedido etc.). |
| `start_at` | `string (date_time)` | sim | Data de início da recorrência. |
| `recurrence_term` | `string` | sim | Prazo da recorrência. Valores: `indefinite`, `fixed`. |
| `cycle` | `string` | sim | Ciclo da recorrência. Valores: `weekly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `amount_term` | `string` | sim | Tipo do valor da recorrência. Valores: `fixed`, `variable`. |
| `retry_policy` | `string` | sim | Política de retentativa. Valores: `disallow`, `allow_3r7d`. |
| `debtor` | `object` | sim | Informações da pessoa devedora |
| `pix_uid` | `string` | não | UID do Pix (opcional). Quando informado junto com location_uid: o qrcode gerado será da jornada instant (Jornada 3) para Pix instantâneo ou offer (Jornada 4) para Pix com vencimento. |
| `location_uid` | `string` | não | UID da localização. Quando informado, o QR Code será gerado conforme a jornada (recurrent / instant / offer). A location precisa estar registrada no banco (`financial_provider_external_id` preenchido); aguarde o webhook `charge.automatic_pix.location.register.confirmed` ou consulte `GET /v2/charge/automatic_pix/locations/{uid}` até `financial_provider_external_id` vir preenchido. |
| `description` | `string` | não | Descrição da recorrência. |
| `finish_at` | `string (date_time)` | não | Data de fim da recorrência. |
| `amount` | `number` | não | Valor (quando amount_term é fixed). |
| `minimum_amount` | `number` | não | Valor mínimo (quando amount_term é variable). |
| `agreement_code` | `string` | não | Número do Convênio de Pagamento. |
| `external_id` | `string` | não | ID externo da recorrência. |
| `custom_data` | `object` | não | Dados personalizados. |
| `tags` | `array<string>` | não | Tags. |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.recurrences.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Recorrência PIX automática criada com pix_uid e location_uid (JORNADA_3) |
| `422` | External ID já existe |

## charge automatic-pix recurrences pix

Criar nova cobrança PIX automática

```text
kobana banking charge automatic-pix recurrences pix [RECURRENCE_UID] [OPÇÕES]
```

`POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/pix`

Cria uma nova cobrança PIX automática para uma recorrência.

**Argumentos do caminho:** `recurrence_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Valor da cobrança. |
| `retry_policy` | `string` | sim | Política de retentativa. Valores: `disallow`, `allow_3r7d`. |
| `next_business_day` | `boolean` | sim | Se deve considerar próximo dia útil. |
| `expire_at` | `string (date_time)` | sim | Data de expiração da cobrança. |
| `additional_info` | `string` | não | Informações adicionais da cobrança. |
| `txid` | `string` | não | Identificador da transação (opcional; gerado automaticamente se omitido). |
| `external_id` | `string` | não | ID externo da cobrança. |
| `tags` | `array<string>` | não | Tags. |
| `custom_data` | `object` | não | Dados personalizados. |
| `debtor` | `object` | não | Dados adicionais do devedor |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.recurrences.pix`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Cobrança PIX automática criada com txid personalizado |
| `422` | ID Externo duplicado |

## charge automatic-pix recurrences requests

Criar nova solicitação PIX automática

```text
kobana banking charge automatic-pix recurrences requests [RECURRENCE_UID] [OPÇÕES]
```

`POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/requests`

Cria uma nova solicitação PIX automática para uma recorrência.

**Argumentos do caminho:** `recurrence_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `expire_at` | `string (date_time)` | sim | Data de expiração da solicitação. |
| `addressee_financial_account` | `object` | sim | Conta Financeira do Destinatário (usuário pagador) |
| `external_id` | `string` | não | ID externo da solicitação. |
| `custom_data` | `object` | não | Dados personalizados. |
| `tags` | `array<string>` | não | Tags. |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.recurrences.requests`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Solicitação PIX automática criada com sucesso |
| `422` | ID Externo duplicado |

## charge automatic-pix recurrences get

Informações da recorrência PIX automática

```text
kobana banking charge automatic-pix recurrences get [UID] [OPÇÕES]
```

`GET /v2/charge/automatic_pix/recurrences/{uid}`

Consulta informações específicas de uma recorrência PIX automática.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Recorrência PIX automática encontrada |
| `404` | Recorrência PIX automática não encontrada |

## charge automatic-pix recurrences update

Atualizar recorrência PIX automática

```text
kobana banking charge automatic-pix recurrences update [UID] [OPÇÕES]
```

`PATCH /v2/charge/automatic_pix/recurrences/{uid}`

Atualiza informações de uma recorrência PIX automática existente.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `external_id` | `string` | não | ID externo da recorrência. |
| `tags` | `array<string>` | não | Tags. |
| `custom_data` | `object` | não | Dados personalizados. |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.recurrences.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Recorrência PIX automática atualizada com sucesso |
| `404` | Recorrência PIX automática não encontrada |
| `422` | External ID já existe ao atualizar |

## charge automatic-pix recurrences cancel

Cancelar recorrência PIX automática

```text
kobana banking charge automatic-pix recurrences cancel [UID] [OPÇÕES]
```

`PUT /v2/charge/automatic_pix/recurrences/{uid}/cancel`

Cancela uma recorrência PIX automática existente.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Recorrência PIX automática cancelada com sucesso |
| `422` | Erro ao cancelar |

## charge automatic-pix requests list

Lista todas as solicitações PIX automáticas

```text
kobana banking charge automatic-pix requests list [OPÇÕES]
```

`GET /v2/charge/automatic_pix/requests`

Lista todas as solicitações PIX automáticas com paginação.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da página para paginação |
| `per_page` | `integer` | não | Quantidade de itens por página (padrão: 50, máximo: 50) |
| `automatic_pix_recurrence_uid` | `string` | não | Filtrar por UID da recorrência PIX automática |
| `charge_automatic_pix_account_uid` | `string` | não | Filtrar por UID da conta Pix Automático |
| `external_id` | `string` | não | Filtrar por ID externo |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de solicitações PIX automáticas |

## charge automatic-pix requests get

Informações da solicitação PIX automática

```text
kobana banking charge automatic-pix requests get [UID] [OPÇÕES]
```

`GET /v2/charge/automatic_pix/requests/{uid}`

Consulta informações específicas de uma solicitação PIX automática.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Solicitação PIX automática encontrada |
| `404` | Solicitação PIX automática não encontrada |

## charge automatic-pix requests update

Atualizar solicitação PIX automática

```text
kobana banking charge automatic-pix requests update [UID] [OPÇÕES]
```

`PATCH /v2/charge/automatic_pix/requests/{uid}`

Atualiza informações de uma solicitação PIX automática existente.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `external_id` | `string` | não | ID externo da solicitação. |
| `tags` | `array<string>` | não | Tags. |
| `custom_data` | `object` | não | Dados personalizados. |

Outros campos e objetos aninhados: `kobana schema banking.charge.automatic-pix.requests.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Solicitação PIX automática atualizada com sucesso |
| `404` | Solicitação PIX automática não encontrada |
| `422` | ID Externo duplicado ao atualizar |

## charge automatic-pix requests cancel

Cancelar solicitação PIX automática

```text
kobana banking charge automatic-pix requests cancel [UID] [OPÇÕES]
```

`PUT /v2/charge/automatic_pix/requests/{uid}/cancel`

Cancela uma solicitação PIX automática existente.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Solicitação PIX automática cancelada com sucesso |
| `404` | Solicitação PIX automática não encontrada |
| `422` | Erro ao cancelar |

## charge payments list

Listar Recebimentos

```text
kobana banking charge payments list [OPÇÕES]
```

`GET /v2/charge/payments`

Listar Recebimentos.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página. |
| `per_page` | `integer` | não | Quantidade de registros por página. |
| `paid_from` | `string` | não | A partir da Data de Recebimento. |
| `paid_to` | `string` | não | Até a Data de Recebimento. |
| `amount_from` | `number` | não | A partir do Valor do Recebimento. |
| `amount_to` | `number` | não | Até o Valor do Recebimento. |
| `kind` | `string` | não | Tipo de Recebimento. Direto ao beneficiario/Em mãos, Boleto (Confirmado pelo banco via arquivo de retorno ou API) ou PIX (Confirmado pelo banco). Valores: `direct`, `bank_billet`, `pix`. |
| `payable_type` | `string` | não | Tipo de Recurso Pagável. Valores: `BankBillet`, `Pix`. |
| `payable_public_id` | `string` | não | UID do Recurso Pagável. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Buscando por tipo do recebimento |

## charge payments create

Informar Recebimento

```text
kobana banking charge payments create [OPÇÕES]
```

`POST /v2/charge/payments`

Informa um Recebimento.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Valor do Recebimento. |
| `kind` | `string` | sim | Tipo de Recebimento. Valores: `direct`, `bank_billet`, `pix`. |
| `paid_at` | `string (date)` | sim | Data do Recebimento. |
| `payable` | `object` | sim | Recurso associado ao pagamento (Boleto ou Pix). |
| `uid` | `string (uuid)` | não | UID do Recebimento. |
| `debtor` | `object` | não | Informações do devedor (sacado) do boleto. |
| `payer` | `object` | não | Informações de quem efetivamente realizou o pagamento (portador). Preenchido automaticamente via consulta diária ao banco, disponível D+2 dias úteis após o pagamento. Null quando a consulta ainda não foi realizada ou o banco não retornou os dados do pagador. |
| `bank` | `object` | não | Informações bancárias. |
| `receiving` | `object` | não | Informações de recebimento. |
| `credit_at` | `string (date)` | não | Data de crédito. |
| `created_via_api` | `boolean` | não | Enviado pela API. |
| `transaction_identifier` | `string` | não | Identificador da Transação. |
| `status` | `string` | não | Situação. |
| `duplicated` | `object` | não | Presente quando o recebimento é uma duplicata. Contém o UID do recebimento original. |
| `created_at` | `string (date)` | não | Data e hora de criação. |
| `updated_at` | `string (date)` | não | Data e hora da alteração. |

Outros campos e objetos aninhados: `kobana schema banking.charge.payments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Recebimento de pix informado com sucesso |
| `422` | Erro de validação |

## charge payments get

Informações do Recebimento

```text
kobana banking charge payments get [UID] [OPÇÕES]
```

`GET /v2/charge/payments/{uid}`

Consulta informações específicas de um Recebimento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Recebimento encontrado |
| `404` | Recebimento não encontrado |

## charge payments delete

Excluir Recebimento

```text
kobana banking charge payments delete [UID] [OPÇÕES]
```

`DELETE /v2/charge/payments/{uid}`

Exclui um Recebimento.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Recebimento excluído |
| `404` | Recebimento não encontrado |

## charge pix commands get

Visualizar um Comando

```text
kobana banking charge pix commands get [PIX_UID] [UID] [OPÇÕES]
```

`GET /v2/charge/pix/{pix_uid}/commands/{uid}`

Consulta informações detalhadas de um comando específico executado em uma cobrança Pix.

**Argumentos do caminho:** `pix_uid`, `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Atualização do Pix encontrada |
| `404` | Atualização do Pix não encontrada |

## charge pix commands list

Listar Comandos de um Pix

```text
kobana banking charge pix commands list [PIX_UID] [OPÇÕES]
```

`GET /v2/charge/pix/{pix_uid}/commands`

Lista todos os comandos (operações) executados em uma cobrança Pix específica, incluindo atualizações, cancelamentos e outras operações.

**Argumentos do caminho:** `pix_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pix encontrado |
| `404` | Pix não encontrado |

## charge pix list

Listar Pix

```text
kobana banking charge pix list [OPÇÕES]
```

`GET /v2/charge/pix`

Lista todas as cobranças Pix cadastradas. Permite filtrar por status, TXID, data de vencimento, data de pagamento, CNPJ/CPF do pagador e tags.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `txid` | `string` | não | Filtro por TXID do Pix |
| `status` | `string` | não | Filtro por status do Pix |
| `pix_account_id` | `integer` | não | Filtro por ID da conta do Pix |
| `pix_account_uid` | `string` | não | Filtro por UID da conta do Pix |
| `expire_from` | `string` | não | Data inicial do filtro de data de vencimento do Pix |
| `expire_to` | `string` | não | Data final do filtro de data de vencimento do Pix |
| `paid_from` | `string` | não | Data inicial do filtro de data de pagamento do Pix |
| `paid_to` | `string` | não | Data final do filtro de data de pagamento do Pix |
| `created_from` | `string` | não | Data inicial do filtro de data de criação do Pix |
| `created_to` | `string` | não | Data final do filtro de data de criação do Pix |
| `cnpj_cpf` | `string` | não | Filtro por CNPJ/CPF do pagador |
| `tags` | `string` | não | Filtro por tags do pix |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Buscando Pix por tags |
| `422` | cnpj_cpf inválido |

## charge pix create

Criar um Pix

```text
kobana banking charge pix create [OPÇÕES]
```

`POST /v2/charge/pix`

Esse endpoint é utilizado para a geração de uma cobrança Pix.

A criação do Pix é sincrona, porém o registro na instituição financeira é realizada em segundo plano. Sendo assim, o retorno da requisição não irá conter o campo `qrcode`.

Para receber o qrcode é necessário assinar um webhook para receber o payload completo do Pix quando o registro for confirmado.

**Pré-requisito**

Para emitir uma cobrança Pix é necessário ter cadastrado em sua conta uma Conta Pix. Todo Pix pertence a uma Conta Pix que representa a instituição de pagamento e a chave que será usada para a emissão do Pix.
A criação da Conta Pix precisa ser feita pela interface web do sistema ou pelo endpoint `POST /api/v2/charge/pix_accounts`.

**Eventos de Webhook**

Ao cadastrar um Pix o sistema cria um comando para registrar o PIX na instituição financeira e o evento `pix.register.requested` é disparado. Após o registro do Pix ser confirmado na instituição o evento `pix.register.confirmed` é disparado.

O evento `pix.db.created` também é disparado nesta operação.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | sim | Quantia em reais |
| `pix_account_uid` | `string (uuid)` | sim | UID da Conta Pix |
| `expire_at` | `string (date-time)` | sim | Data e hora de vencimento. Formato iso8601 |
| `id` | `integer` | não | ID do Pix (DEPRECATED: use uid) |
| `uid` | `string (uuid)` | não | UID do Pix |
| `payer` | `object` | não | Dados do Pagador |
| `status` | `string` | não | Status do Pix. * `opened`: Cobrança Pix em aberto (não pago). [Estado Inicial] * `paid`: Cobrança Pix paga. * `overdue`: Cobrança Pix em atraso. * `canceled`: Cobrança Pix cancelada. * `generation_failed`: Cobrança Pix criada, mas não foi registrada no banco. Valores: `opened`, `paid`, `overdue`, `canceled`, `generation_failed`. |
| `pix_account_id` | `integer` | não | ID da Conta Pix (DEPRECATED: use pix_account_uid) |
| `txid` | `anyOf` | não | TXID do Pix - Identificador único da transação |
| `canceled_at` | `string (date-time)` | não | Data e hora de cancelamento do Pix |
| `registered_at` | `string (date-time)` | não | Data e hora de registro no provedor |
| `paid_at` | `string (date-time)` | não | Data e hora do pagamento |
| `paid_amount` | `number` | não | Valor efetivamente pago |
| `registration_status` | `string` | não | Status do registro no provedor Valores: `pending`, `skipped`, `requested`, `confirmed`, `rejected`, `failed`. |
| `revoke_days` | `anyOf` | não | Número de dias ativo após o vencimento. OBRIGATÓRIO e deve ser maior que zero quando o Pix possui juros (interest) ou multa (fine) configurados. |
| `message` | `anyOf` | não | Mensagem de solicitação ao pagador |
| `message_payer` | `anyOf` | não | Mensagem para o pagador (campo readOnly) |
| `beneficiary` | `object` | não | Dados do beneficiário do Pix |
| `url` | `string` | não | URL para visualização do Pix |
| `formats` | `object` | não | URLs com formatos disponíveis para o Pix |
| `additional_info` | `anyOf` | não | Informações adicionais para o pagador (máximo 10 pares chave-valor) |
| `qrcode` | `object` | não | QR Code do Pix |
| `registration_kind` | `string` | não | Tipo do Pix. * `instant`: Imediato (padrão) * `billing`: Cobrança Valores: `instant`, `billing`. |
| `kind` | `string` | não | Tipo do Pix. * `instant`: Imediato (padrão) * `billing`: Cobrança Valores: `instant`, `billing`. |
| `command_ids` | `array<integer>` | não |  |
| `payments` | `array<string (uuid)>` | não | Lista de UIDs dos Recebimentos vinculados ao Pix |
| `end_to_end_id` | `anyOf` | não | End To End Id do Pix - Identificador único da transferência |
| `custom_data` | `anyOf` | não | Hash com chave e valor no formato JSON. |
| `external_id` | `string` | não | Identificador externo para o Pix |
| `virtual_pix_uid` | `string` | não | UID do Pix gerado por membro BackOffice. (BackOffice precisa estar habilitado). |
| `fine_type` | `integer` | não | Tipo da multa: * `0` Sem multa (`none`) * `1` Por valor (`amount`) * `2` Por percentual (`percentage`) No retorno os dados vêm no objeto `fine`. Valores: `0`, `1`, `2`. |
| `fine_amount` | `number` | não | Valor da multa em reais. Obrigatório quando `fine_type` = `amount`. |
| `fine_percentage` | `number` | não | Percentual da multa (0.0 a 100.0). Obrigatório quando `fine_type` = `percentage`. |
| `interest_type` | `integer` | não | Tipo de juros: * `0` Não se aplica / isento (`none`) * `1` Valor diário — dias corridos (`daily_amount_calendar`) * `2` Percentual diário — dias corridos (`daily_percentage_calendar`) * `3` Percentual mensal — dias corridos (`monthly_percentage_calendar`) * `4` Percentual anual — dias corridos (`yearly_percentage_calendar`) * `5` Valor diário — dias úteis (`daily_amount_business`) * `6` Percentual diário — dias úteis (`daily_percentage_business`) * `7` Percentual mensal — dias úteis (`monthly_percentage_business`) * `8` Percentual anual — dias úteis (`yearly_percentage_business`) No retorno os dados vêm no objeto `interest`. Valores: `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`. |
| `interest_amount` | `number` | não | Valor diário de juros em reais. Obrigatório quando `interest_type` for por valor (`daily_amount_*`). |
| `interest_percentage` | `number` | não | Percentual de juros. Obrigatório quando `interest_type` for por percentual. |
| `reduction_type` | `integer` | não | Tipo de abatimento: * `1` Por valor (`amount`) * `2` Por percentual (`percentage`) * `3` Não se aplica — remove o abatimento (`none`) No retorno os dados vêm no objeto `reduction`. Valores: `1`, `2`, `3`. |
| `reduction_amount` | `number` | não | Valor do abatimento em reais. Obrigatório quando `reduction_type` = `amount`. |
| `reduction_percentage` | `number` | não | Percentual do abatimento. Obrigatório quando `reduction_type` = `percentage`. |
| `discount_type` | `integer` | não | Tipo de desconto aplicado aos campos `discount_*` (nulo/ausente = sem desconto): * `1` Valor fixo (`fixed_amount`) * `2` Percentual fixo (`fixed_percentage`) * `3` Antecipação por valor — dias corridos (`advance_amount_calendar`) * `4` Antecipação por valor — dias úteis (`advance_amount_business`) * `5` Antecipação por percentual — dias corridos (`advance_percentage_calendar`) * `6` Antecipação por percentual — dias úteis (`advance_percentage_business`) As modalidades de antecipação (`3`–`6`) aceitam apenas uma faixa. No retorno o tipo vem em `discounts.type`. Valores: `1`, `2`, `3`, `4`, `5`, `6`. |
| `discount_amount` | `number` | não | Valor do 1º desconto em reais. Obrigatório quando `discount_type` for por valor. |
| `discount_percentage` | `number` | não | Percentual do 1º desconto. Obrigatório quando `discount_type` for por percentual. |
| `discount_days` | `integer` | não | Dias antes do vencimento em que o 1º desconto é válido. |
| `discount_second_amount` | `number` | não | Valor do 2º desconto em reais. |
| `discount_second_percentage` | `number` | não | Percentual do 2º desconto. |
| `discount_second_days` | `integer` | não | Dias antes do vencimento em que o 2º desconto é válido. |
| `discount_third_amount` | `number` | não | Valor do 3º desconto em reais. |
| `discount_third_percentage` | `number` | não | Percentual do 3º desconto. |
| `discount_third_days` | `integer` | não | Dias antes do vencimento em que o 3º desconto é válido. |
| `fine` | `object` | não | Configuração de multa (RETORNO/response como objeto nested). No ENVIO/request use os campos flat `fine_type`, `fine_amount` e `fine_percentage`. |
| `reduction` | `object` | não | Configuração de abatimento (RETORNO/response como objeto nested). No ENVIO/request use os campos flat `reduction_type`, `reduction_amount` e `reduction_percentage`. |
| `interest` | `object` | não | Configuração de juros (RETORNO/response como objeto nested). No ENVIO/request use os campos flat `interest_type`, `interest_amount` e `interest_percentage`. |
| `discounts` | `object` | não | Configuração de descontos (RETORNO/response como objeto nested com até 3 descontos). No ENVIO/request use os campos flat `discount_*`, `discount_second_*` e `discount_third_*`. |
| `ignore_whatsapp` | `boolean` | não | Nunca enviar este Pix por WhatsApp |
| `password_protected_mode` | `integer` | não | Modo de proteção por senha: 0=desabilitado, 1=últimos 4 dígitos, 2=primeiros 5 dígitos Valores: `0`, `1`, `2`. |
| `tags` | `array<string>` | não | Tags associadas ao pix |
| `tag_list` | `string` | não | Tags associadas ao pix |
| `created_via_api` | `boolean` | não | Indica se o Pix foi criado por API |
| `financial_provider_external_id` | `string` | não | ID na instituição financeira |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.charge.pix.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Pix criado com dados de cliente |
| `422` | Erro de validação |

## charge pix cancel

Cancelar um Pix

```text
kobana banking charge pix cancel [PIX_UID] [OPÇÕES]
```

`PUT /v2/charge/pix/{pix_uid}/cancel`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de cancelamento para o PIX.
O cancelamento de um PIX é feito assincronamente, ou seja, ao receber a requisição, a nossa API guarda a solicitação e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, o Pix é cancelado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento. Veja mais em Listar Comandos de um Pix e Visualizar um Comando.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `pix.command.cancel.started` é disparado.
O nosso servidor então começa as tentativas de envio da cancelamento para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então o pix é cancelado em nosso banco de dados.
Neste momento o evento `pix.command.cancel.confirmed` é disparado.

O evento `pix.db.updated` também é disparado na confirmação desta operação.
> ⚠️ **Atenção ao payload de retorno:** A resposta da chamada de cancelamento de um PIX é o payload de um Comando. Dentro do payload do comando tem a chave `pix` com os dados do PIX em questão.

**Argumentos do caminho:** `pix_uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso |
| `422` | Pix já cancelado |

## charge pix update

Atualizar um Pix

```text
kobana banking charge pix update [PIX_UID] [OPÇÕES]
```

`PUT /v2/charge/pix/{pix_uid}/update`

> **Processamento Assíncrono:** Este endpoint é processado assincronamente e cria um comando de atualização para o PIX.
A atualização de um PIX é feita assincronamente, ou seja, ao receber a requisição, a nossa API guarda as mudanças que você está solicitando e cria um Comando. Esse comando é processado em segundo plano e depois de confirmado, o Pix é atualizado. Você recebe na resposta desta requisição o ID do comando criado e pode consultá-lo a qualquer momento. Veja mais em Listar Comandos de um Pix e Visualizar um Comando.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `pix.update.requested` é disparado.
O nosso servidor então começa as tentativas de envio da alteração para a instituição financeira.
Somente depois de confirmado na instituição financeira, aí então o pix é atualizado.
Neste momento o evento `pix.update.confirmed` é disparado.

O evento `pix.db.updated` também é disparado na confirmação desta operação.
> ⚠️ **Atenção ao payload de retorno:** A resposta da chamada de atualização de um PIX é o payload de um Comando. Dentro do payload do comando tem a chave `pix` com os dados do PIX em questão.

**Argumentos do caminho:** `pix_uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `amount` | `number` | não | Quantia |
| `pix_account_uid` | `string (uuid)` | não | UID da Conta Pix para alterar a chave Pix do recebedor. Este campo é útil quando você deseja alterar a chave Pix no provedor financeiro. **Importante**: Somente são aceitas contas Pix que estejam ativas (status `active`). |
| `payer` | `object` | não | Dados do Pagador |
| `status` | `string` | não | Status do Pix. `opened`: Cobrança Pix em aberto (não pago). [Estado Inicial] `paid`: Cobrança Pix paga. `overdue`: Cobrança Pix em atraso. `canceled`: Cobrança Pix cancelada. `generation_failed`: Cobrança Pix criada, mas não foi registrada no banco. |
| `expire_at` | `string (date-time)` | não | Data e hora de vencimento. Formato iso8601 |
| `revoke_days` | `integer` | não | Número de dias ativo após o vencimento |
| `message` | `string` | não | Mensagem de solicitação ao pagador |
| `additional_info` | `object` | não | Informações adicionais para o pagador |
| `fine_type` | `integer` | não | Tipo da multa: * `0` Sem multa (`none`) * `1` Por valor (`amount`) * `2` Por percentual (`percentage`) No retorno os dados vêm no objeto `fine`. Valores: `0`, `1`, `2`. |
| `fine_amount` | `number` | não | Valor da multa em reais. Obrigatório quando `fine_type` = `amount`. |
| `fine_percentage` | `number` | não | Percentual da multa (0.0 a 100.0). Obrigatório quando `fine_type` = `percentage`. |
| `interest_type` | `integer` | não | Tipo de juros: * `0` Não se aplica / isento (`none`) * `1` Valor diário — dias corridos (`daily_amount_calendar`) * `2` Percentual diário — dias corridos (`daily_percentage_calendar`) * `3` Percentual mensal — dias corridos (`monthly_percentage_calendar`) * `4` Percentual anual — dias corridos (`yearly_percentage_calendar`) * `5` Valor diário — dias úteis (`daily_amount_business`) * `6` Percentual diário — dias úteis (`daily_percentage_business`) * `7` Percentual mensal — dias úteis (`monthly_percentage_business`) * `8` Percentual anual — dias úteis (`yearly_percentage_business`) No retorno os dados vêm no objeto `interest`. Valores: `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`. |
| `interest_amount` | `number` | não | Valor diário de juros em reais. Obrigatório quando `interest_type` for por valor (`daily_amount_*`). |
| `interest_percentage` | `number` | não | Percentual de juros. Obrigatório quando `interest_type` for por percentual. |
| `reduction_type` | `integer` | não | Tipo de abatimento: * `1` Por valor (`amount`) * `2` Por percentual (`percentage`) * `3` Não se aplica — remove o abatimento (`none`) No retorno os dados vêm no objeto `reduction`. Valores: `1`, `2`, `3`. |
| `reduction_amount` | `number` | não | Valor do abatimento em reais. Obrigatório quando `reduction_type` = `amount`. |
| `reduction_percentage` | `number` | não | Percentual do abatimento. Obrigatório quando `reduction_type` = `percentage`. |
| `discount_type` | `integer` | não | Tipo de desconto aplicado aos campos `discount_*` (nulo/ausente = sem desconto): * `1` Valor fixo (`fixed_amount`) * `2` Percentual fixo (`fixed_percentage`) * `3` Antecipação por valor — dias corridos (`advance_amount_calendar`) * `4` Antecipação por valor — dias úteis (`advance_amount_business`) * `5` Antecipação por percentual — dias corridos (`advance_percentage_calendar`) * `6` Antecipação por percentual — dias úteis (`advance_percentage_business`) As modalidades de antecipação (`3`–`6`) aceitam apenas uma faixa. No retorno o tipo vem em `discounts.type`. Valores: `1`, `2`, `3`, `4`, `5`, `6`. |
| `discount_amount` | `number` | não | Valor do 1º desconto em reais. Obrigatório quando `discount_type` for por valor. |
| `discount_percentage` | `number` | não | Percentual do 1º desconto. Obrigatório quando `discount_type` for por percentual. |
| `discount_days` | `integer` | não | Dias antes do vencimento em que o 1º desconto é válido. |
| `discount_second_amount` | `number` | não | Valor do 2º desconto em reais. |
| `discount_second_percentage` | `number` | não | Percentual do 2º desconto. |
| `discount_second_days` | `integer` | não | Dias antes do vencimento em que o 2º desconto é válido. |
| `discount_third_amount` | `number` | não | Valor do 3º desconto em reais. |
| `discount_third_percentage` | `number` | não | Percentual do 3º desconto. |
| `discount_third_days` | `integer` | não | Dias antes do vencimento em que o 3º desconto é válido. |
| `created_via_api` | `boolean` | não | Indica se o Pix foi criado por API |
| `financial_provider_external_id` | `string` | não | ID na instituição financeira |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.charge.pix.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Comando criado com sucesso - com pix_account_uid |
| `422` | Pix pago |

## charge pix get

Visualizar um Pix

```text
kobana banking charge pix get [UID] [OPÇÕES]
```

`GET /v2/charge/pix/{uid}`

Consulta informações específicas de um Pix.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Pix encontrado |
| `404` | Pix não encontrado |

## charge pix delete

Excluir um Pix

```text
kobana banking charge pix delete [UID] [OPÇÕES]
```

`DELETE /v2/charge/pix/{uid}`

Deleta um Pix cancelado.

Esta operação excluí o PIX definitivamente do nosso banco de dados. Mesmo realizando a exclusão, o PIX é contabilizado na volumetria para o plano contratad

**Eventos de Webhook**

O evento `pix.db.deleted` é disparado nesta operação.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Pix deletado |
| `404` | Pix não encontrado |
| `422` | Pix aberto |

## charge pix-accounts list

Listar Contas Pix

```text
kobana banking charge pix-accounts list [OPÇÕES]
```

`GET /v2/charge/pix_accounts`

Lista as contas Pix cadastradas na conta.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Contas Pix encontradas |

## charge pix-accounts create

Criar Pix Account

```text
kobana banking charge pix-accounts create [OPÇÕES]
```

`POST /v2/charge/pix_accounts`

Este endpoint é utilizado para cadastrar uma nova conta pix.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `financial_account_uid` | `string` | sim | UID da conta financeira. O provedor e dados de beneficiário são derivados automaticamente. |
| `key` | `string` | sim | Chave pix |
| `custom_name` | `string` | não | Nome customizado da conta pix |
| `enabled` | `boolean` | não | Habilitar Conta Pix |
| `default` | `boolean` | não | Conta default |

Outros campos e objetos aninhados: `kobana schema banking.charge.pix-accounts.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Conta Pix criada |
| `422` | Conta Pix inválida |

## charge pix-accounts get

Visualizar uma conta Pix

```text
kobana banking charge pix-accounts get [UID] [OPÇÕES]
```

`GET /v2/charge/pix_accounts/{uid}`

Consulta informações específicas de uma conta Pix.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta Pix encontrada |
| `404` | Conta Pix não encontrada |

## charge pix-accounts update

Atualizar Pix Account

```text
kobana banking charge pix-accounts update [UID] [OPÇÕES]
```

`PUT /v2/charge/pix_accounts/{uid}`

Este endpoint é utilizado para atualizar uma nova conta pix.

**Argumentos do caminho:** `uid`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `custom_name` | `string` | não | Nome customizado da conta pix |
| `key` | `string` | não | Chave pix |
| `enabled` | `boolean` | não | Habilitar Conta Pix |
| `default` | `boolean` | não | Conta default |
| `financial_account_uid` | `string` | não | UID da conta financeira (para associar a uma conta existente) |

Outros campos e objetos aninhados: `kobana schema banking.charge.pix-accounts.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Conta Pix atualizada |
| `404` | Conta Pix não encontrada |
| `422` | Chave Pix inválida |

## charge pix-accounts delete

Deletar Pix Account

```text
kobana banking charge pix-accounts delete [UID] [OPÇÕES]
```

`DELETE /v2/charge/pix_accounts/{uid}`

Deleta uma conta Pix.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Conta Pix deletada |
| `404` | Conta Pix não encontrada |
