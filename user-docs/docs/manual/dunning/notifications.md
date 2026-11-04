---
id: notifications
title: "kobana dunning notifications"
sidebar_label: "notifications"
---

# kobana dunning notifications


Comandos de `kobana dunning notifications`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`notifications list`](#notifications-list) | `GET /v1/notifications` | Listar notificações |
| [`notifications create`](#notifications-create) | `POST /v1/notifications` | Criar notificação |
| [`notifications get`](#notifications-get) | `GET /v1/notifications/{id}` | Consultar notificação |
| [`notifications update`](#notifications-update) | `PUT /v1/notifications/{id}` | Atualizar notificação |
| [`notifications delete`](#notifications-delete) | `DELETE /v1/notifications/{id}` | Excluir notificação |
| [`notifications resend`](#notifications-resend) | `POST /v1/notifications/{id}/resend` | Reenviar notificação |

## notifications list

Listar notificações

```text
kobana dunning notifications list [OPÇÕES]
```

`GET /v1/notifications`

Lista as notificações da organização com paginação e filtros. Ordenável via `sort_by` (`created_at` | `updated_at` | `sent_at` | `delivered_at` | `channel` | `status`).

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `sort_by` | `string` | não | Campo de ordenação (nome público em snake_case, ex.: `due_date`). |
| `sort_order` | `string` | não | Direção da ordenação (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `channel` | `string` | não | Filtra por canal; aceita valor único ou lista separada por vírgula (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). |
| `status` | `string` | não | Filtra por status; aceita valor único ou lista separada por vírgula (`pending` \| `queued` \| `sent` \| `delivered` \| `read` \| `failed` \| `bounced`). |
| `person_id` | `string` | não | Filtra pelo cliente (UUID). |
| `charge_id` | `string` | não | Filtra pela cobrança (UUID). |
| `include` | `string` | não | Relações a incluir, separadas por vírgula (`person`, `charge`, `step`; `template` equivale a `step`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de notificações. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## notifications create

Criar notificação

```text
kobana dunning notifications create [OPÇÕES]
```

`POST /v1/notifications`

Cria uma notificação manual para um cliente, opcionalmente vinculada a uma cobrança e a um passo da régua. O destinatário é validado conforme o canal.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `person_id` | `string (uuid)` | sim | Cliente destinatário. |
| `channel` | `string` | sim | Canal (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `recipient` | `string` | sim | Destinatário; validado conforme o canal (e-mail para `email`, telefone para `sms`/`whatsapp`/`voice`). |
| `body` | `string` | sim | Corpo da mensagem. |
| `charge_id` | `string (uuid)` | não | Cobrança relacionada à notificação. |
| `collection_rule_step_id` | `string (uuid)` | não | Passo da régua de cobrança que originou a notificação. |
| `subject` | `string` | não | Assunto (canais que suportam, ex.: e-mail). |
| `external_id` | `string` | não | Identificador no SEU sistema ou no provedor de envio. |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `tags` | `array<string>` | não | Tags livres. |

Outros campos e objetos aninhados: `kobana schema dunning.notifications.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Notificação criada com status `pending`. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## notifications get

Consultar notificação

```text
kobana dunning notifications get [ID] [OPÇÕES]
```

`GET /v1/notifications/{id}`

Retorna uma notificação pelo ID; use `include` para anexar relações.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `include` | `string` | não | Relações a incluir, separadas por vírgula (`person`, `charge`, `step`; `template` equivale a `step`). |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Notificação encontrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## notifications update

Atualizar notificação

```text
kobana dunning notifications update [ID] [OPÇÕES]
```

`PUT /v1/notifications/{id}`

Atualiza status, timestamps de entrega e metadados de uma notificação (parcial). Útil para integrações que confirmam envio/entrega.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Novo status; preenche o timestamp correspondente (sentAt/deliveredAt/readAt/failedAt) quando não informado. Valores: `pending`, `queued`, `sent`, `delivered`, `read`, `failed`, `bounced`. |
| `sent_at` | `string` | não | Timestamp ISO 8601 ou data `YYYY-MM-DD`; null limpa o campo. |
| `delivered_at` | `string` | não | Timestamp ISO 8601 ou data `YYYY-MM-DD`; null limpa o campo. |
| `read_at` | `string` | não | Timestamp ISO 8601 ou data `YYYY-MM-DD`; null limpa o campo. |
| `failed_at` | `string` | não | Timestamp ISO 8601 ou data `YYYY-MM-DD`; null limpa o campo. |
| `failure_reason` | `string` | não | Motivo da falha reportado pelo provedor. |
| `external_id` | `string` | não | Identificador no SEU sistema ou no provedor de envio. |
| `custom_data` | `object` | não | Metadados personalizáveis pelo cliente da API. Limites: 64 KB serializado e 16 níveis de aninhamento. |
| `tags` | `array<string>` | não | Tags livres. |
| `cost` | `number` | não | Custo do envio (número, mínimo 0). |

Outros campos e objetos aninhados: `kobana schema dunning.notifications.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Notificação atualizada. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## notifications delete

Excluir notificação

```text
kobana dunning notifications delete [ID] [OPÇÕES]
```

`DELETE /v1/notifications/{id}`

Exclui uma notificação DEFINITIVAMENTE (hard delete — o modelo não tem soft delete). Só a notificação ainda não entregue: a entregue é evidência probatória e responde `409`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Notificação excluída. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | A notificação já foi entregue — é evidência probatória e não pode ser excluída. |
| `413` | Corpo da requisição acima de 2 MB. |

## notifications resend

Reenviar notificação

```text
kobana dunning notifications resend [ID] [OPÇÕES]
```

`POST /v1/notifications/{id}/resend`

Cria uma NOVA notificação `pending` copiando a original (o `metadata` da nova referencia a original em `resend_of`). Só é permitido quando a original está `failed` ou `bounced`; caso contrário retorna 400. Suporta `Idempotency-Key`.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Nova notificação criada a partir da original. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
