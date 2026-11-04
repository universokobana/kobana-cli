---
id: plan-changes
title: "kobana billing plan-changes"
sidebar_label: "plan-changes"
---

# kobana billing plan-changes


Comandos de `kobana billing plan-changes`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`plan-changes config`](#plan-changes-config) | `GET /v1/plan_changes/config` | Obter configuração de mudança de plano |
| [`plan-changes config update`](#plan-changes-config-update) | `PATCH /v1/plan_changes/config` | Atualizar configuração de mudança de plano |
| [`plan-changes rules list`](#plan-changes-rules-list) | `GET /v1/plan_changes/rules` | Listar regras de mudança de plano |
| [`plan-changes rules create`](#plan-changes-rules-create) | `POST /v1/plan_changes/rules` | Criar regra de mudança de plano |
| [`plan-changes rules get`](#plan-changes-rules-get) | `GET /v1/plan_changes/rules/{id}` | Obter regra de mudança de plano |
| [`plan-changes rules update`](#plan-changes-rules-update) | `PATCH /v1/plan_changes/rules/{id}` | Atualizar regra de mudança de plano |
| [`plan-changes rules delete`](#plan-changes-rules-delete) | `DELETE /v1/plan_changes/rules/{id}` | Excluir regra de mudança de plano |
| [`plan-changes stats`](#plan-changes-stats) | `GET /v1/plan_changes/stats` | Estatísticas de mudanças de plano |
| [`plan-changes list`](#plan-changes-list) | `GET /v1/plan_changes` | Listar mudanças de plano |
| [`plan-changes create`](#plan-changes-create) | `POST /v1/plan_changes` | Executar mudança de plano |
| [`plan-changes get`](#plan-changes-get) | `GET /v1/plan_changes/{id}` | Obter mudança de plano |

## plan-changes config

Obter configuração de mudança de plano

```text
kobana billing plan-changes config [OPÇÕES] [COMANDO]
```

`GET /v1/plan_changes/config`

Retorna a configuração padrão da organização para upgrades e downgrades — comportamento de proração, timing e geração de crédito/refund.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Configuração atual. |
| `404` | Recurso não encontrado na organização atual. |

## plan-changes config update

Atualizar configuração de mudança de plano

```text
kobana billing plan-changes config update [OPÇÕES]
```

`PATCH /v1/plan_changes/config`

Atualiza parcialmente (PATCH) a configuração de mudança de plano da organização. Todos os campos são opcionais.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `allow_upgrade` | `boolean` | não | Habilita upgrades de plano para a organização. |
| `allow_downgrade` | `boolean` | não | Habilita downgrades de plano para a organização. |
| `upgrade_proration` | `string` | não | Método de proração padrão para upgrades: `full_proration`, `no_proration` ou `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `downgrade_proration` | `string` | não | Método de proração padrão para downgrades. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `upgrade_timing` | `string` | não | Quando aplicar upgrades: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `downgrade_timing` | `string` | não | Quando aplicar downgrades: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `refund_on_downgrade` | `boolean` | não | Estorna a diferença ao fazer downgrade. |
| `credit_on_downgrade` | `boolean` | não | Gera crédito (em vez de estorno) ao fazer downgrade. |
| `apply_discount_on_change` | `boolean` | não | Mantém descontos ativos ao mudar de plano. |

Outros campos e objetos aninhados: `kobana schema billing.plan-changes.config.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Configuração atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plan-changes rules list

Listar regras de mudança de plano

```text
kobana billing plan-changes rules list [OPÇÕES]
```

`GET /v1/plan_changes/rules`

Lista as regras customizadas que sobrescrevem o comportamento padrão para transições específicas entre planos.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista de regras de mudança de plano. |
| `404` | Recurso não encontrado na organização atual. |

## plan-changes rules create

Criar regra de mudança de plano

```text
kobana billing plan-changes rules create [OPÇÕES]
```

`POST /v1/plan_changes/rules`

Cria uma nova regra para uma transição entre planos. Permite bloquear/permitir a transição, definir timing, proração, descontos e dias bônus.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `from_plan_id` | `string (uuid)` | não | UUID do plano de origem. Quando ausente, a regra cobre qualquer plano de origem. |
| `to_plan_id` | `string (uuid)` | não | UUID do plano de destino. Quando ausente, a regra cobre qualquer plano de destino. |
| `change_type` | `string` | não | Tipo da mudança: `upgrade`, `downgrade` ou `lateral`. Valores: `upgrade`, `downgrade`, `lateral`. |
| `allowed` | `boolean` | não | Se `false`, bloqueia explicitamente a transição. |
| `timing` | `string` | não | Sobrescreve o timing da mudança: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `proration_method` | `string` | não | Sobrescreve o método de proração: `full_proration`, `no_proration` ou `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `discount_percent` | `number` | não | Desconto percentual aplicado quando esta regra é usada (0 a 100). |
| `bonus_days` | `integer` | não | Dias bônus adicionados ao período corrente quando esta regra é aplicada. |
| `message` | `string` | não | Mensagem livre exibida no portal/checkout (ex.: motivo do bloqueio). |
| `priority` | `integer` | não | Prioridade para resolução de conflito quando múltiplas regras casam (maior vence). |

Outros campos e objetos aninhados: `kobana schema billing.plan-changes.rules.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Regra criada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plan-changes rules get

Obter regra de mudança de plano

```text
kobana billing plan-changes rules get [ID] [OPÇÕES]
```

`GET /v1/plan_changes/rules/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da regra. |
| `404` | Recurso não encontrado na organização atual. |

## plan-changes rules update

Atualizar regra de mudança de plano

```text
kobana billing plan-changes rules update [ID] [OPÇÕES]
```

`PATCH /v1/plan_changes/rules/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `allowed` | `boolean` | não | Se `false`, bloqueia explicitamente a transição. |
| `timing` | `string` | não | Sobrescreve o timing da mudança: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `proration_method` | `string` | não | Sobrescreve o método de proração: `full_proration`, `no_proration` ou `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `discount_percent` | `number` | não | Desconto percentual aplicado quando esta regra é usada (0 a 100). |
| `bonus_days` | `integer` | não | Dias bônus adicionados ao período corrente quando esta regra é aplicada. |
| `message` | `string` | não | Mensagem livre exibida no portal/checkout (ex.: motivo do bloqueio). |
| `priority` | `integer` | não | Prioridade para resolução de conflito quando múltiplas regras casam (maior vence). |
| `is_active` | `boolean` | não | Desativa a regra sem removê-la. |

Outros campos e objetos aninhados: `kobana schema billing.plan-changes.rules.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Regra atualizada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plan-changes rules delete

Excluir regra de mudança de plano

```text
kobana billing plan-changes rules delete [ID] [OPÇÕES]
```

`DELETE /v1/plan_changes/rules/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Regra removida. |
| `404` | Recurso não encontrado na organização atual. |

## plan-changes stats

Estatísticas de mudanças de plano

```text
kobana billing plan-changes stats [OPÇÕES]
```

`GET /v1/plan_changes/stats`

Retorna agregados de mudanças de plano (totais por tipo, por status, créditos e cobranças de proração) para uma janela em dias.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `days` | `integer` | não | Janela em dias para os agregados (padrão: `30`). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Estatísticas agregadas de mudanças de plano. |
| `404` | Recurso não encontrado na organização atual. |

## plan-changes list

Listar mudanças de plano

```text
kobana billing plan-changes list [OPÇÕES]
```

`GET /v1/plan_changes`

Retorna o histórico paginado de mudanças de plano (upgrades, downgrades e laterais) da organização atual.

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
| `subscription_id` | `string` | não | Filtra pelas mudanças de uma assinatura. |
| `billing_account_id` | `string` | não | Filtra pelas mudanças de uma conta de cobrança. |
| `from_plan_id` | `string` | não | Filtra pelo plano de origem (UUID). |
| `to_plan_id` | `string` | não | Filtra pelo plano de destino (UUID). |
| `change_type` | `string` | não | Filtra pelo tipo: `upgrade`, `downgrade` ou `lateral`. Valores: `upgrade`, `downgrade`, `lateral`. |
| `status` | `string` | não | Filtra pelo status: `pending`, `scheduled`, `completed`, `canceled` ou `failed`. Valores: `pending`, `scheduled`, `completed`, `canceled`, `failed`. |
| `timing` | `string` | não | Filtra pelo timing: `immediate` ou `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `proration_method` | `string` | não | Filtra pelo método de proração (ex.: `full_proration`, `no_proration`, `partial_proration`). |
| `price_difference_min` | `number,null` | não | Diferença de preço mínima em R$. Ex.: `10.50`. |
| `price_difference_max` | `number,null` | não | Diferença de preço máxima em R$. Ex.: `500.00`. |
| `proration_charge_min` | `number,null` | não | Cobrança de proração mínima em R$. |
| `proration_charge_max` | `number,null` | não | Cobrança de proração máxima em R$. |
| `proration_credit_min` | `number,null` | não | Crédito de proração mínimo em R$. |
| `proration_credit_max` | `number,null` | não | Crédito de proração máximo em R$. |
| `effective_date_from` | `string` | não | Data de vigência a partir de (inclusive). Formato: `YYYY-MM-DD`. |
| `effective_date_to` | `string` | não | Data de vigência até (inclusive). Formato: `YYYY-MM-DD`. |
| `scheduled_date_from` | `string` | não | Data agendada a partir de (inclusive). Formato: `YYYY-MM-DD`. |
| `scheduled_date_to` | `string` | não | Data agendada até (inclusive). Formato: `YYYY-MM-DD`. |
| `completed_after` | `string` | não | Concluída a partir de (inclusive). Formato: `YYYY-MM-DD`. |
| `completed_before` | `string` | não | Concluída até (inclusive). Formato: `YYYY-MM-DD`. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de mudanças de plano. |
| `404` | Recurso não encontrado na organização atual. |

## plan-changes create

Executar mudança de plano

```text
kobana billing plan-changes create [OPÇÕES]
```

`POST /v1/plan_changes`

Executa uma mudança de plano para uma assinatura existente. O `timing` controla se a mudança é aplicada imediatamente (com proração) ou no fim do período atual. Quando bem-sucedido, retorna os UUIDs do `plan_change`, da fatura de proração (se houver) e do crédito gerado (se houver).

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `subscription_id` | `string (uuid)` | sim | UUID da assinatura que terá o plano alterado. |
| `to_plan_id` | `string (uuid)` | sim | UUID do plano de destino. |
| `timing` | `string` | não | Quando aplicar a mudança: `immediate` (padrão para upgrades) ou `end_of_period` (padrão para downgrades). Valores: `immediate`, `end_of_period`. |
| `reason` | `string` | não | Motivo da mudança (registrado no audit log). |

Outros campos e objetos aninhados: `kobana schema billing.plan-changes.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Mudança de plano executada. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## plan-changes get

Obter mudança de plano

```text
kobana billing plan-changes get [ID] [OPÇÕES]
```

`GET /v1/plan_changes/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Detalhe da mudança de plano. |
| `404` | Recurso não encontrado na organização atual. |
