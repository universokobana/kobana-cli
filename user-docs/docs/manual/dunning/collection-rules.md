---
id: collection-rules
title: "kobana dunning collection-rules"
sidebar_label: "collection-rules"
---

# kobana dunning collection-rules


Comandos de `kobana dunning collection-rules`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`collection-rules steps`](#collection-rules-steps) | `GET /v1/collection-rules/{id}/steps` | Listar etapas da régua |
| [`collection-rules steps create`](#collection-rules-steps-create) | `POST /v1/collection-rules/{id}/steps` | Criar etapa |
| [`collection-rules steps update`](#collection-rules-steps-update) | `PUT /v1/collection-rules/{id}/steps/{stepId}` | Atualizar etapa |
| [`collection-rules steps delete`](#collection-rules-steps-delete) | `DELETE /v1/collection-rules/{id}/steps/{stepId}` | Excluir etapa |
| [`collection-rules list`](#collection-rules-list) | `GET /v1/collection-rules` | Listar réguas |
| [`collection-rules create`](#collection-rules-create) | `POST /v1/collection-rules` | Criar régua |
| [`collection-rules get`](#collection-rules-get) | `GET /v1/collection-rules/{id}` | Consultar régua |
| [`collection-rules update`](#collection-rules-update) | `PUT /v1/collection-rules/{id}` | Atualizar régua |
| [`collection-rules delete`](#collection-rules-delete) | `DELETE /v1/collection-rules/{id}` | Excluir régua |

## collection-rules steps

Listar etapas da régua

```text
kobana dunning collection-rules steps [ID] [OPÇÕES] [COMANDO]
```

`GET /v1/collection-rules/{id}/steps`

Lista as etapas da régua ordenadas por `position`, com o template de mensagem embutido.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Etapas da régua (envelope `data`, sem paginação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## collection-rules steps create

Criar etapa

```text
kobana dunning collection-rules steps create [ID] [OPÇÕES]
```

`POST /v1/collection-rules/{id}/steps`

Cria uma etapa na régua. Sem `position`, entra no fim; com `position`, desloca as etapas seguintes.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome da etapa. |
| `trigger_type` | `string` | sim | Gatilho (`before_due` \| `on_due` \| `after_due`). Os gatilhos `after_issue`/`after_payment` só podem ser definidos via régua. Valores: `before_due`, `on_due`, `after_due`. |
| `trigger_days` | `integer` | sim | Dias relativos ao gatilho (0 = no dia). |
| `action_type` | `string` | sim | Ação (`notification` \| `negativation` \| `protest` \| `manual_task` \| `negotiation_offer` \| `judicial`). Valores: `notification`, `negativation`, `protest`, `manual_task`, `negotiation_offer`, `judicial`. |
| `channels` | `array<string>` | sim | Canais de envio (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `message_template_id` | `string (uuid)` | não | Template de mensagem usado pela etapa (deve pertencer à organização). |
| `position` | `integer` | não | Posição da etapa na régua (a partir de 1). |
| `tone` | `string` | não | Tom da mensagem (`friendly` \| `neutral` \| `firm` \| `urgent`). Valores: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `preferred_time` | `string` | não | Horário preferido de envio (ex.: `09:00`). |
| `activated_at` | `string (date-time)` | não | Momento da ativação (null = inativa). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.collection-rules.steps.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Etapa criada (com template de mensagem). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## collection-rules steps update

Atualizar etapa

```text
kobana dunning collection-rules steps update [ID] [STEPID] [OPÇÕES]
```

`PUT /v1/collection-rules/{id}/steps/{stepId}`

Atualiza uma etapa (parcial). Mudar `position` reordena as demais etapas automaticamente.

**Argumentos do caminho:** `id`, `stepId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `message_template_id` | `string (uuid)` | não | Template de mensagem usado pela etapa (deve pertencer à organização). |
| `position` | `integer` | não | Posição da etapa na régua (a partir de 1). |
| `name` | `string` | não | Nome da etapa. |
| `trigger_type` | `string` | não | Gatilho (`before_due` \| `on_due` \| `after_due`). Os gatilhos `after_issue`/`after_payment` só podem ser definidos via régua. Valores: `before_due`, `on_due`, `after_due`. |
| `trigger_days` | `integer` | não | Dias relativos ao gatilho (0 = no dia). |
| `action_type` | `string` | não | Ação (`notification` \| `negativation` \| `protest` \| `manual_task` \| `negotiation_offer` \| `judicial`). Valores: `notification`, `negativation`, `protest`, `manual_task`, `negotiation_offer`, `judicial`. |
| `channels` | `array<string>` | não | Canais de envio (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `tone` | `string` | não | Tom da mensagem (`friendly` \| `neutral` \| `firm` \| `urgent`). Valores: `friendly`, `neutral`, `firm`, `urgent`, `null`. |
| `preferred_time` | `string` | não | Horário preferido de envio (ex.: `09:00`). |
| `activated_at` | `string (date-time)` | não | Momento da ativação (null = inativa). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.collection-rules.steps.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Etapa atualizada (com template de mensagem). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## collection-rules steps delete

Excluir etapa

```text
kobana dunning collection-rules steps delete [ID] [STEPID] [OPÇÕES]
```

`DELETE /v1/collection-rules/{id}/steps/{stepId}`

Exclui (soft delete) uma etapa e reordena as restantes. Falha com 409 se a etapa tiver notificações associadas.

**Argumentos do caminho:** `id`, `stepId`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Etapa excluída. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## collection-rules list

Listar réguas

```text
kobana dunning collection-rules list [OPÇÕES]
```

`GET /v1/collection-rules`

Lista as réguas de cobrança da organização, ordenadas por prioridade. `_count.charges` traz o total de cobranças ativas na régua.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `classification_id` | `string` | não | Filtra pela classificação associada (UUID). |
| `is_default` | `string` | não | Filtra pela régua padrão (`true` \| `false`). Valores: `true`, `false`. |
| `activated_at` | `string` | não | Filtra por ativação: `true` = ativas, `false` = inativas. Valores: `true`, `false`. |
| `include` | `string` | não | Relações a embutir, separadas por vírgula. Suportado: `steps`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de réguas. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## collection-rules create

Criar régua

```text
kobana dunning collection-rules create [OPÇÕES]
```

`POST /v1/collection-rules`

Cria uma régua de cobrança, opcionalmente já com etapas. Marcar `is_default` desmarca a régua padrão anterior.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome da régua. |
| `classification_id` | `string (uuid)` | não | Classificação de clientes atendida pela régua (segmentação). |
| `description` | `string` | não | Descrição da régua. |
| `is_default` | `boolean` | não | true quando é a régua padrão da organização (única). |
| `priority` | `integer` | não | Prioridade na seleção de régua (maior vence). |
| `activated_at` | `string (date-time)` | não | Momento da ativação (null = inativa). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |
| `steps` | `array<object>` | não | Etapas da régua ordenadas por `position` (presente com `include=steps` e nas respostas de criação/atualização). |

Outros campos e objetos aninhados: `kobana schema dunning.collection-rules.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Régua criada (com classificação e etapas). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## collection-rules get

Consultar régua

```text
kobana dunning collection-rules get [ID] [OPÇÕES]
```

`GET /v1/collection-rules/{id}`

Retorna uma régua pelo ID. Use `include=steps` para incluir as etapas.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `include` | `string` | não | Relações a embutir, separadas por vírgula. Suportado: `steps`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Régua encontrada (envelope `data`). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## collection-rules update

Atualizar régua

```text
kobana dunning collection-rules update [ID] [OPÇÕES]
```

`PUT /v1/collection-rules/{id}`

Atualiza os dados da régua (parcial). O array `steps`, quando enviado, faz upsert e exclui as etapas ausentes.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `classification_id` | `string (uuid)` | não | Classificação de clientes atendida pela régua (segmentação). |
| `name` | `string` | não | Nome da régua. |
| `description` | `string` | não | Descrição da régua. |
| `is_default` | `boolean` | não | true quando é a régua padrão da organização (única). |
| `priority` | `integer` | não | Prioridade na seleção de régua (maior vence). |
| `activated_at` | `string (date-time)` | não | Momento da ativação (null = inativa). |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |
| `steps` | `array<object>` | não | Conjunto completo de etapas (upsert): com `id` atualiza, sem `id` cria, ausentes são excluídas. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.collection-rules.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Régua atualizada (envelope `data`). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## collection-rules delete

Excluir régua

```text
kobana dunning collection-rules delete [ID] [OPÇÕES]
```

`DELETE /v1/collection-rules/{id}`

Exclui (soft delete) a régua e suas etapas. Falha com 409 se houver cobranças usando a régua.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Régua excluída. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |
