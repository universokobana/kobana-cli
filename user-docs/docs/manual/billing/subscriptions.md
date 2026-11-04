---
id: subscriptions
title: "kobana billing subscriptions"
sidebar_label: "subscriptions"
---

# kobana billing subscriptions


Comandos de `kobana billing subscriptions`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`subscriptions items`](#subscriptions-items) | `GET /v1/subscriptions/{id}/items` | Listar itens da assinatura |
| [`subscriptions items create`](#subscriptions-items-create) | `POST /v1/subscriptions/{id}/items` | Aplica mudanças nos itens da assinatura |
| [`subscriptions list`](#subscriptions-list) | `GET /v1/subscriptions` | Listar assinaturas |
| [`subscriptions create`](#subscriptions-create) | `POST /v1/subscriptions` | Criar assinatura |
| [`subscriptions update`](#subscriptions-update) | `PUT /v1/subscriptions` | Criar ou atualizar assinatura |
| [`subscriptions get`](#subscriptions-get) | `GET /v1/subscriptions/{id}` | Obter assinatura |
| [`subscriptions delete`](#subscriptions-delete) | `DELETE /v1/subscriptions/{id}` | Cancelar e remover assinatura (soft delete) |
| [`subscriptions activate`](#subscriptions-activate) | `POST /v1/subscriptions/{id}/activate` | Ativar assinatura em rascunho/confirmada |
| [`subscriptions cancel`](#subscriptions-cancel) | `POST /v1/subscriptions/{id}/cancel` | Cancelar assinatura |
| [`subscriptions change-anniversary`](#subscriptions-change-anniversary) | `POST /v1/subscriptions/{id}/change-anniversary` | Trocar o aniversário (dia de início do ciclo) da assinatura |
| [`subscriptions change-plan`](#subscriptions-change-plan) | `POST /v1/subscriptions/{id}/change-plan` | Trocar o plano da assinatura |
| [`subscriptions create-invoice`](#subscriptions-create-invoice) | `POST /v1/subscriptions/{id}/create-invoice` | Gerar uma fatura avulsa para a assinatura |
| [`subscriptions pause`](#subscriptions-pause) | `POST /v1/subscriptions/{id}/pause` | Pausar assinatura |
| [`subscriptions resume`](#subscriptions-resume) | `POST /v1/subscriptions/{id}/resume` | Retomar assinatura pausada |
| [`subscriptions revert-to-confirmed`](#subscriptions-revert-to-confirmed) | `POST /v1/subscriptions/{id}/revert-to-confirmed` | Reverter assinatura para o estado confirmado |
| [`subscriptions sync-plan-items`](#subscriptions-sync-plan-items) | `POST /v1/subscriptions/{id}/sync-plan-items` | Ressincronizar itens com o plano atual |

## subscriptions items

Listar itens da assinatura

```text
kobana billing subscriptions items [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/subscriptions/{id}/items`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de itens. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions items create

Aplica mudanças nos itens da assinatura

```text
kobana billing subscriptions items create [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/items`

Executa um lote de até 50 operações sobre os itens: adicionar, remover, alterar quantidade, trocar de preço ou definir um valor unitário avulso. As operações são aplicadas em conjunto e geram um único registro de mudança, com a fatura ou o crédito de proração resultante.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `changes` | `array<oneOf>` | sim | Operações a aplicar, de 1 a 50. Cada item é discriminado por `op`. |
| `timing` | `string` | não | Quando aplicar: `immediate` (padrão) ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `reason` | `string` | não | Motivo livre, gravado junto ao registro de mudança. |
| `idempotency_key` | `string` | não | Chave de idempotência. O cabeçalho `Idempotency-Key` tem precedência sobre este campo. |

Outros campos e objetos aninhados: `kobana schema billing.subscriptions.items.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Mudanças aplicadas. `item_change_id` aponta o registro gerado; `invoice_id` e `credit_id` vêm preenchidos quando a proração gerou fatura ou crédito. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## subscriptions list

Listar assinaturas

```text
kobana billing subscriptions list [OPÇÕES]
```

`GET /v1/subscriptions`

Retorna uma lista paginada das assinaturas da organização atual.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `person_id` | `string` | não | Filtra por UUID da pessoa associada. |
| `person_document_number` | `string` | não | Filtra por CPF/CNPJ da pessoa (exato). |
| `person_name_contains` | `string` | não | Filtra por nome da pessoa (busca parcial, sem distinção de maiúsculas). |
| `status` | `string` | não | Filtra por status da assinatura. Valores: `draft`, `trialing`, `active`, `past_due`, `canceled`, `paused`, `suspended`. |
| `plan_id` | `string` | não | Filtra por plano. |
| `billing_cycle` | `string` | não | Filtra por periodicidade de cobrança. Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `billing_account_id` | `string` | não | Filtra por conta de cobrança. |
| `search` | `string` | não | Busca textual livre. |
| `company_id` | `string` | não | Filtra por empresa (UUID). |
| `collection_method` | `string` | não | Filtra pelo método de cobrança: `charge_automatically`, `manual_charge` ou `manual_invoice`. Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `cancel_at_period_end` | `string` | não | Filtra assinaturas com cancelamento agendado para o fim do período (`true`) ou não (`false`). Valores: `true`, `false`. |
| `external_id` | `string` | não | Filtra pelo identificador externo. |
| `product_group_id` | `string` | não | Filtra por grupo de produto (UUID). |
| `proposal_id` | `string` | não | Filtra por proposta (UUID). |
| `recurring_amount_min` | `number,null` | não | Valor recorrente mínimo em reais (ex.: `50.00`). |
| `recurring_amount_max` | `number,null` | não | Valor recorrente máximo em reais (ex.: `500.00`). |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `current_period_start_from` | `string` | não | Início do período atual a partir desta data (YYYY-MM-DD). |
| `current_period_start_to` | `string` | não | Início do período atual até esta data (YYYY-MM-DD). |
| `current_period_end_from` | `string` | não | Fim do período atual a partir desta data (YYYY-MM-DD). |
| `current_period_end_to` | `string` | não | Fim do período atual até esta data (YYYY-MM-DD). |
| `trial_end_from` | `string` | não | Fim do trial a partir desta data (YYYY-MM-DD). |
| `trial_end_to` | `string` | não | Fim do trial até esta data (YYYY-MM-DD). |
| `canceled_after` | `string` | não | Cancelado a partir desta data (YYYY-MM-DD). |
| `canceled_before` | `string` | não | Cancelado até esta data (YYYY-MM-DD). |
| `activated_after` | `string` | não | Ativado a partir desta data (YYYY-MM-DD). |
| `activated_before` | `string` | não | Ativado até esta data (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de assinaturas. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions create

Criar assinatura

```text
kobana billing subscriptions create [OPÇÕES]
```

`POST /v1/subscriptions`

Cria uma assinatura. É obrigatório enviar `plan_id` ou `items`.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sim | UUID da conta de cobrança que receberá esta assinatura. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não |  |
| `plan_id` | `string (uuid)` | não | UUID do plano. Obrigatório quando `items` não é enviado. |
| `product_group_id` | `string (uuid)` | não |  |
| `billing_cycle` | `string` | não | Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `collection_method` | `string` | não | Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `trial_end` | `string (date-time)` | não | Data de término do trial (ISO 8601). |
| `trial_period_days` | `integer` | não | Alternativa a `trial_end`. Dias de trial a partir da criação. |
| `proration_behavior` | `string` | não | Valores: `create_prorations`, `none`, `always_invoice`. |
| `nfe_issuance_policy` | `string` | não | Valores: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `items` | `array<object>` | não |  |
| `commitment_months` | `integer` | não | Fidelidade: número de meses contratados (copiado da proposta aceita). Dentro da janela de fidelidade — início da assinatura + meses — o cliente não consegue cancelar pelo portal. Use `null` para remover. |
| `notes` | `string` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema billing.subscriptions.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Assinatura criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## subscriptions update

Criar ou atualizar assinatura

```text
kobana billing subscriptions update [OPÇÕES]
```

`PUT /v1/subscriptions`

Recebe o mesmo corpo do POST, acrescido de `id` e `public_id`, e deduplica por `id`, `public_id`, `external_id` e as chaves naturais do recurso, nesta ordem, parando na primeira que encontrar um registro. Se encontrar, atualiza e responde 200; se não, cria e responde 201. Os campos obrigatórios do POST só são exigidos quando nenhum registro é encontrado, e a atualização é parcial: campos ausentes mantêm o valor atual.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | não | UUID da conta de cobrança que receberá esta assinatura. |
| `external_id` | `string` | não | Identificador no seu sistema externo, para idempotência/sincronização. |
| `company_id` | `string (uuid)` | não |  |
| `plan_id` | `string (uuid)` | não | UUID do plano. Obrigatório quando `items` não é enviado. |
| `product_group_id` | `string (uuid)` | não |  |
| `billing_cycle` | `string` | não | Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `collection_method` | `string` | não | Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `trial_end` | `string (date-time)` | não | Data de término do trial (ISO 8601). |
| `trial_period_days` | `integer` | não | Alternativa a `trial_end`. Dias de trial a partir da criação. |
| `proration_behavior` | `string` | não | Valores: `create_prorations`, `none`, `always_invoice`. |
| `nfe_issuance_policy` | `string` | não | Valores: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `items` | `array<object>` | não |  |
| `commitment_months` | `integer` | não | Fidelidade: número de meses contratados (copiado da proposta aceita). Dentro da janela de fidelidade — início da assinatura + meses — o cliente não consegue cancelar pelo portal. Use `null` para remover. |
| `notes` | `string` | não |  |
| `custom_metadata` | `object` | não |  |
| `id` | `string (uuid)` | não | UUID do registro. Usado apenas para localizá-lo — nunca é gravado. |
| `public_id` | `string (uuid)` | não | UUID público do registro, usado para localizá-lo. |

Outros campos e objetos aninhados: `kobana schema billing.subscriptions.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Registro atualizado. |
| `201` | Registro criado. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## subscriptions get

Obter assinatura

```text
kobana billing subscriptions get [ID] [OPÇÕES]
```

`GET /v1/subscriptions/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da assinatura. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions delete

Cancelar e remover assinatura (soft delete)

```text
kobana billing subscriptions delete [ID] [OPÇÕES]
```

`DELETE /v1/subscriptions/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Confirmação de soft delete. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions activate

Ativar assinatura em rascunho/confirmada

```text
kobana billing subscriptions activate [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/activate`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Assinatura ativada. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions cancel

Cancelar assinatura

```text
kobana billing subscriptions cancel [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/cancel`

Cancela imediatamente, ou agenda para o fim do período quando `cancel_at_period_end=true`.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cancel_at_period_end` | `boolean` | não | Quando `true`, agenda o cancelamento para o fim do período atual. Padrão: `false` (cancela imediatamente). |
| `reason` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema billing.subscriptions.cancel`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Assinatura cancelada. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions change-anniversary

Trocar o aniversário (dia de início do ciclo) da assinatura

```text
kobana billing subscriptions change-anniversary [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/change-anniversary`

Move o dia do mês em que os ciclos começam. O ciclo em andamento mantém o início e é estendido até a véspera do novo dia; os dias acrescentados são cobrados pro rata (valor do ciclo × dias ÷ dias do ciclo) numa fatura `subscription_update`. A troca sempre avança a data, nunca a recua. Use `dry_run` para calcular sem aplicar.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `anniversary_day` | `integer` | sim | Novo dia do mês (1 a 28) em que os ciclos começam. Use 1 para mês fechado. |
| `charge_proration` | `boolean` | não | Cobrar os dias acrescentados. Padrão: `true`, exceto quando o `proration_behavior` da assinatura é `none`. |
| `dry_run` | `boolean` | não | Quando `true`, só calcula e devolve o resultado, sem alterar nada. |
| `reason` | `string` | não | Motivo da troca (opcional, registrado no audit log). |

Outros campos e objetos aninhados: `kobana schema billing.subscriptions.change-anniversary`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Resultado da troca (ou da simulação, com `dry_run`). |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## subscriptions change-plan

Trocar o plano da assinatura

```text
kobana billing subscriptions change-plan [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/change-plan`

Upgrade ou downgrade. A proração segue o `proration_behavior` da assinatura.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `plan_id` | `string (uuid)` | sim | UUID do novo plano. |
| `timing` | `string` | não | Quando a troca de plano entra em vigor. Padrão: `immediate`. Valores: `immediate`, `end_of_period`. |
| `reason` | `string` | não | Motivo da troca (opcional, registrado no audit log). |

Outros campos e objetos aninhados: `kobana schema billing.subscriptions.change-plan`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Assinatura atualizada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## subscriptions create-invoice

Gerar uma fatura avulsa para a assinatura

```text
kobana billing subscriptions create-invoice [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/create-invoice`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Fatura gerada. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions pause

Pausar assinatura

```text
kobana billing subscriptions pause [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/pause`

Suspende a cobrança mantendo a assinatura aberta. `behavior` controla como faturas geradas durante a pausa são tratadas.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `behavior` | `string` | não | Como tratar faturas geradas durante a pausa. Padrão: `mark_uncollectible`. Valores: `mark_uncollectible`, `keep_as_draft`, `void`. |
| `resumes_at` | `string (date-time)` | não | Data para retomada automática. Omita para retomada manual. |

Outros campos e objetos aninhados: `kobana schema billing.subscriptions.pause`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Assinatura pausada. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## subscriptions resume

Retomar assinatura pausada

```text
kobana billing subscriptions resume [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/resume`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Assinatura retomada. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions revert-to-confirmed

Reverter assinatura para o estado confirmado

```text
kobana billing subscriptions revert-to-confirmed [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/revert-to-confirmed`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Assinatura revertida. |
| `404` | Recurso não encontrado na organização atual. |

## subscriptions sync-plan-items

Ressincronizar itens com o plano atual

```text
kobana billing subscriptions sync-plan-items [ID] [OPÇÕES]
```

`POST /v1/subscriptions/{id}/sync-plan-items`

Realinha os itens da assinatura com o template de plan-items (reconciliação manual).

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Assinatura sincronizada. |
| `404` | Recurso não encontrado na organização atual. |
