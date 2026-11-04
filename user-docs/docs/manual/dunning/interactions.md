---
id: interactions
title: "kobana dunning interactions"
sidebar_label: "interactions"
---

# kobana dunning interactions


Comandos de `kobana dunning interactions`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`interactions list`](#interactions-list) | `GET /v1/interactions` | Listar interações |
| [`interactions create`](#interactions-create) | `POST /v1/interactions` | Registrar interação |
| [`interactions get`](#interactions-get) | `GET /v1/interactions/{id}` | Consultar interação |
| [`interactions update`](#interactions-update) | `PUT /v1/interactions/{id}` | Atualizar interação |
| [`interactions delete`](#interactions-delete) | `DELETE /v1/interactions/{id}` | Excluir interação |

## interactions list

Listar interações

```text
kobana dunning interactions list [OPÇÕES]
```

`GET /v1/interactions`

Lista as interações da organização com paginação e filtros. Ordenável via `sort_by` (`contacted_at` | `interaction_type` | `created_at` | `updated_at`; padrão `contacted_at` desc).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `sort_by` | `string` | não | Campo de ordenação (nome público em snake_case, ex.: `due_date`). |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `interaction_type` | `string` | não | Filtra pelo tipo (`call` \| `email` \| `whatsapp` \| `meeting` \| `note`). Valores: `call`, `email`, `whatsapp`, `meeting`, `note`. |
| `direction` | `string` | não | Filtra pela direção (`inbound` \| `outbound`). Valores: `inbound`, `outbound`. |
| `outcome` | `string` | não | Filtra pelo desfecho (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Valores: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`. |
| `person_id` | `string` | não | Filtra pelo cliente (UUID). |
| `charge_id` | `string` | não | Filtra pela cobrança (UUID). |
| `include` | `string` | não | Relações a incluir, separadas por vírgula (`person`, `user`, `charge`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de interações. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## interactions create

Registrar interação

```text
kobana dunning interactions create [OPÇÕES]
```

`POST /v1/interactions`

Registra uma interação com um cliente, opcionalmente vinculada a uma cobrança. Suporta `Idempotency-Key`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_id` | `string (uuid)` | sim | Cliente da interação. |
| `interaction_type` | `string` | sim | Tipo (`call` \| `email` \| `whatsapp` \| `meeting` \| `note`). Valores: `call`, `email`, `whatsapp`, `meeting`, `note`. |
| `direction` | `string` | sim | Direção (`inbound` = cliente contatou, `outbound` = fomos ao cliente). Valores: `inbound`, `outbound`. |
| `charge_id` | `string (uuid)` | não | Cobrança relacionada à interação. |
| `summary` | `string` | não | Resumo/notas da interação. |
| `outcome` | `string` | não | Desfecho (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Valores: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`, `null`. |
| `promise_date` | `string (date-time)` | não | Data prometida para pagamento. |
| `promised_amount` | `number` | não | Valor prometido (número positivo). |
| `contacted_at` | `string (date-time)` | não | Momento do contato (ISO 8601; padrão: agora). |
| `duration_seconds` | `integer` | não | Duração do contato em segundos. |
| `external_id` | `string` | não | Identificador no SEU sistema (chave das integrações). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.interactions.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Interação registrada (inclui os resumos de cliente, usuário e cobrança). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## interactions get

Consultar interação

```text
kobana dunning interactions get [ID] [OPÇÕES]
```

`GET /v1/interactions/{id}`

Retorna uma interação pelo ID, com resumos de cliente, usuário e cobrança.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Interação encontrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## interactions update

Atualizar interação

```text
kobana dunning interactions update [ID] [OPÇÕES]
```

`PUT /v1/interactions/{id}`

Atualiza os campos editáveis de uma interação (parcial).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `summary` | `string` | não | Resumo/notas da interação. |
| `notes` | `string` | não | Alias de `summary` (gravado no mesmo campo; se ambos vierem, `notes` prevalece). |
| `outcome` | `string` | não | Desfecho (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Valores: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`, `null`. |
| `promise_date` | `string (date-time)` | não | Data prometida para pagamento. |
| `promised_amount` | `number` | não | Valor prometido (número positivo). |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `custom_data_mode` | `string` | não | Como aplicar `custom_data` no update: `replace` (default) sobrescreve o objeto inteiro; `merge` mescla (deep) o payload sobre o valor armazenado — objetos aninhados mesclam, escalares/arrays/null sobrescrevem a chave. Valores: `replace`, `merge`. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.interactions.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Interação atualizada. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## interactions delete

Excluir interação

```text
kobana dunning interactions delete [ID] [OPÇÕES]
```

`DELETE /v1/interactions/{id}`

Exclui (soft delete) uma interação; o registro permanece para trilha de auditoria.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Interação excluída. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
