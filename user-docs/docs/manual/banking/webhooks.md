---
id: webhooks
title: "kobana banking webhooks"
sidebar_label: "webhooks"
---

# kobana banking webhooks


Comandos de `kobana banking webhooks`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | Listar Webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Criar Webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Informações do Webhook |
| [`webhooks update`](#webhooks-update) | `PUT /v1/webhooks/{id}` | Atualizar Webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Excluir Webhook |

## webhooks list

Listar Webhooks

```text
kobana banking webhooks list [OPÇÕES]
```

`GET /v1/webhooks`

Lista webhooks.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem de webhooks |

## webhooks create

Criar Webhook

```text
kobana banking webhooks create [OPÇÕES]
```

`POST /v1/webhooks`

Cria um webhook para receber notificações de eventos do sistema via HTTP.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `url` | `string` | sim | Endereço onde as requisições serão realizadas |
| `events` | `array<string>` | sim | Eventos que devem gerar notificações para o webhook. Escolha um ou mais eventos. |
| `id` | `integer` | não | ID do webhook |
| `name` | `string` | não | Nome de identificação do webhook |
| `content_type` | `string` | não | Content-type da requisição que será realizada. Valores válidos `application/json` (valor padrão) ou `application/x-www-form-urlencoded`. |
| `active` | `boolean` | não | Define se o webhook estará ativo, recebendo notificações. O valor padrão é `true`. |
| `ssl_verification_enabled` | `boolean` | não | Define se a verificação SSL está habilitada no webhook. |
| `subaccounts` | `boolean` | não | Define se o webhook estará ativo para os eventos das subcontas. |
| `bank_billet_account_id` | `integer` | não | ID da Carteira de Cobrança. Se informado, o webhook apenas receberá notificações de eventos relacionados à carteira indicada. |
| `auth_kind` | `string` | não | Tipo de autenticação configurado. Retornado nas respostas. Para configurar, envie o objeto `auth` no body. Valores: `none`, `basic_auth`, `bearer`, `api_key`. |
| `auth` | `object` | não | Configuração de autenticação do endpoint receptor. Apenas para criação/atualização — não retornado nas respostas. |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.webhooks.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Webhook criado com Basic Auth |
| `422` | Webhook inválido |

## webhooks get

Informações do Webhook

```text
kobana banking webhooks get [ID] [OPÇÕES]
```

`GET /v1/webhooks/{id}`

Consulta informações específicas de um Webhook.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Webhook encontrado |
| `404` | Webhook não encontrado |

## webhooks update

Atualizar Webhook

```text
kobana banking webhooks update [ID] [OPÇÕES]
```

`PUT /v1/webhooks/{id}`

Atualiza informações específicas de um webhook.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `url` | `string` | sim | Endereço onde as requisições serão realizadas |
| `events` | `array<string>` | sim | Eventos que devem gerar notificações para o webhook. Escolha um ou mais eventos. |
| `id` | `integer` | não | ID do webhook |
| `name` | `string` | não | Nome de identificação do webhook |
| `content_type` | `string` | não | Content-type da requisição que será realizada. Valores válidos `application/json` (valor padrão) ou `application/x-www-form-urlencoded`. |
| `active` | `boolean` | não | Define se o webhook estará ativo, recebendo notificações. O valor padrão é `true`. |
| `ssl_verification_enabled` | `boolean` | não | Define se a verificação SSL está habilitada no webhook. |
| `subaccounts` | `boolean` | não | Define se o webhook estará ativo para os eventos das subcontas. |
| `bank_billet_account_id` | `integer` | não | ID da Carteira de Cobrança. Se informado, o webhook apenas receberá notificações de eventos relacionados à carteira indicada. |
| `auth_kind` | `string` | não | Tipo de autenticação configurado. Retornado nas respostas. Para configurar, envie o objeto `auth` no body. Valores: `none`, `basic_auth`, `bearer`, `api_key`. |
| `auth` | `object` | não | Configuração de autenticação do endpoint receptor. Apenas para criação/atualização — não retornado nas respostas. |
| `created_at` | `string (date-time)` | não | Data e hora de criação do registro |
| `updated_at` | `string (date-time)` | não | Data e hora da última atualização do registro |

Outros campos e objetos aninhados: `kobana schema banking.webhooks.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Atualização parcial de auth preserva campos secretos existentes |
| `404` | Webhook não encontrado |

## webhooks delete

Excluir Webhook

```text
kobana banking webhooks delete [ID] [OPÇÕES]
```

`DELETE /v1/webhooks/{id}`

Exclui um webhook específicos

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Webhook excluido |
| `404` | Webhook não encontrado |
