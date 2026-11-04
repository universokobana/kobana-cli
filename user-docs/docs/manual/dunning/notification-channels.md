---
id: notification-channels
title: "kobana dunning notification-channels"
sidebar_label: "notification-channels"
---

# kobana dunning notification-channels


Comandos de `kobana dunning notification-channels`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`notification-channels list`](#notification-channels-list) | `GET /v1/notification-channels` | Listar canais de notificação |
| [`notification-channels create`](#notification-channels-create) | `POST /v1/notification-channels` | Criar canal de notificação |
| [`notification-channels get`](#notification-channels-get) | `GET /v1/notification-channels/{id}` | Consultar canal de notificação |
| [`notification-channels update`](#notification-channels-update) | `PATCH /v1/notification-channels/{id}` | Atualizar canal de notificação |
| [`notification-channels delete`](#notification-channels-delete) | `DELETE /v1/notification-channels/{id}` | Remover canal de notificação |

## notification-channels list

Listar canais de notificação

```text
kobana dunning notification-channels list [OPÇÕES]
```

`GET /v1/notification-channels`

Retorna os canais da organização. As credenciais nunca são incluídas.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (a partir de 1). |
| `per_page` | `integer` | não | Itens por página (máx. 100). |
| `channel_type` | `string` | não | Filtra pelo tipo de canal. Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | não | Filtra pelo provedor. |
| `activated_at` | `string` | não | Filtra por ativação: `true` (ativos) \| `false`/`null` (inativos). Valores: `true`, `false`, `null`. |
| `sort` | `string` | não | Campo de ordenação: `channel_type` \| `provider` \| `created_at` \| `updated_at`. Valores: `channelType`, `provider`, `createdAt`, `updatedAt`. |
| `order` | `string` | não | Direção da ordenação: `asc` \| `desc`. Valores: `asc`, `desc`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista paginada de canais. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## notification-channels create

Criar canal de notificação

```text
kobana dunning notification-channels create [OPÇÕES]
```

`POST /v1/notification-channels`

Cria um canal. As credenciais são cifradas em repouso antes de gravar. Um canal por combinação de tipo + provedor.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `channel_type` | `string` | sim | Tipo do canal: `email` \| `sms` \| `whatsapp` \| `voice` \| `manual`. Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | sim | Provedor do canal (ex.: `sendgrid`, `twilio`, `meta`). |
| `credentials` | `object` | sim | Credenciais do provedor (ex.: `api_key`/`api_secret`). Cifradas em repouso na escrita e NUNCA retornadas na leitura. Somente escrita. |
| `settings` | `object` | não | Configurações não-sensíveis do canal (JSON livre). |
| `cost_per_message` | `number` | não | Custo por mensagem enviada (string decimal). |
| `activated_at` | `string (date-time)` | não | Data de ativação; `null` = canal inativo. |
| `external_id` | `string` | não | Identificador do canal no sistema externo do integrador. |
| `custom_data` | `object` | não | Dados personalizados (JSON) definidos pelo integrador. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Etiquetas livres para organização. |

Outros campos e objetos aninhados: `kobana schema dunning.notification-channels.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Canal criado (sem credenciais). |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `409` | Conflito com o estado atual do recurso (ou chave de idempotência reusada). |
| `413` | Corpo da requisição acima de 2 MB. |

## notification-channels get

Consultar canal de notificação

```text
kobana dunning notification-channels get [ID] [OPÇÕES]
```

`GET /v1/notification-channels/{id}`

Retorna um canal pelo ID. As credenciais nunca são incluídas.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal encontrado. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## notification-channels update

Atualizar canal de notificação

```text
kobana dunning notification-channels update [ID] [OPÇÕES]
```

`PATCH /v1/notification-channels/{id}`

Atualiza um canal (parcial). Se `credentials` for enviado, é re-cifrado; omitir mantém o segredo atual.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `channel_type` | `string` | não | Tipo do canal: `email` \| `sms` \| `whatsapp` \| `voice` \| `manual`. Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `provider` | `string` | não | Provedor do canal (ex.: `sendgrid`, `twilio`, `meta`). |
| `credentials` | `object` | não | Credenciais do provedor (ex.: `api_key`/`api_secret`). Cifradas em repouso na escrita e NUNCA retornadas na leitura. Somente escrita. |
| `settings` | `object` | não | Configurações não-sensíveis do canal (JSON livre). |
| `cost_per_message` | `number` | não | Custo por mensagem enviada (string decimal). |
| `activated_at` | `string (date-time)` | não | Data de ativação; `null` = canal inativo. |
| `external_id` | `string` | não | Identificador do canal no sistema externo do integrador. |
| `custom_data` | `object` | não | Dados personalizados (JSON) definidos pelo integrador. |
| `custom_metadata` | `object` | não | Metadados do usuário (objeto JSON livre, leitura e escrita). Ver `custom_data` para o campo legado dos recursos núcleo. |
| `tags` | `array<string>` | não | Etiquetas livres para organização. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor gravado: `replace` (padrão) substitui o objeto inteiro; `merge` faz deep merge — objetos aninhados mesclam, escalares e arrays substituem a chave, e uma chave enviada com `null` é gravada como `null` (não é removida). Só na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema dunning.notification-channels.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Canal atualizado. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## notification-channels delete

Remover canal de notificação

```text
kobana dunning notification-channels delete [ID] [OPÇÕES]
```

`DELETE /v1/notification-channels/{id}`

Remove o canal (soft delete).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Canal removido. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
