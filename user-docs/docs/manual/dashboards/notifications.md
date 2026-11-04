---
id: notifications
title: "kobana dashboards notifications"
sidebar_label: "notifications"
---

# kobana dashboards notifications


Comandos de `kobana dashboards notifications`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`notifications stats`](#notifications-stats) | `GET /v1/notifications/stats` | Estatísticas de notificações |
| [`notifications list`](#notifications-list) | `GET /v1/notifications` | Listar notificações |
| [`notifications create`](#notifications-create) | `POST /v1/notifications` | Criar notificação |
| [`notifications send`](#notifications-send) | `POST /v1/notifications/{id}/send` | Enviar notificação |

## notifications stats

Estatísticas de notificações

```text
kobana dashboards notifications stats [OPÇÕES]
```

`GET /v1/notifications/stats`

Contagens por status, tipo e canal das notificações criadas nos últimos `days` dias, com as taxas de entrega e de leitura.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `days` | `integer` | não | Janela em dias, de 1 a 365. Padrão 30. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | As estatísticas. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## notifications list

Listar notificações

```text
kobana dashboards notifications list [OPÇÕES]
```

`GET /v1/notifications`

Lista as notificações do workspace e as da organização inteira, das mais novas para as mais antigas.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Página (começa em 1). Padrão: `1`. |
| `per_page` | `integer` | não | Itens por página. Padrão: `20`. Máximo: `100`. |
| `sort_by` | `string` | não | Campo de ordenação. Padrão depende do recurso. |
| `sort_order` | `string` | não | Direção da ordenação. Padrão: `desc`. Valores: `asc`, `desc`. |
| `type` | `string` | não | Somente este tipo de notificação. Valores: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `channel` | `string` | não | Somente este canal. Valores: `email`, `in_app`, `webhook`. |
| `status` | `string` | não | Somente este status. Valores: `pending`, `sent`, `delivered`, `failed`, `read`, `skipped`. |
| `created_after` | `string` | não | Data de criação inicial do intervalo (YYYY-MM-DD). |
| `created_before` | `string` | não | Data de criação final do intervalo (YYYY-MM-DD). |
| `search` | `string` | não | Trecho do destinatário ou do assunto, sem diferenciar maiúsculas. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Página de notificações. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Corpo da requisição falhou na validação. |

## notifications create

Criar notificação

```text
kobana dashboards notifications create [OPÇÕES]
```

`POST /v1/notifications`

Renderiza o modelo ativo do tipo com as variáveis, grava a notificação e a envia na hora — a resposta traz o resultado (`sent`, ou `failed` com `failure_reason`). Com um `scheduled_for` no futuro, ela fica `pending` até a hora marcada. Cada criação conta no limite de envios manuais por destinatário (429).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `type` | `string` | sim | Tipo de notificação (obrigatório): escolhe o modelo ativo da organização, ou o padrão da plataforma. Valores: `api_key_expiring`, `account_suspended`, `account_activated`, `portal_invitation`, `portal_signup_verification`, `magic_link_portal`, `magic_link_dashboard`, `email_verification`, `password_reset`, `welcome`, `email_change_verification`, `password_verification`, `account_invitation`, `organization_welcome`, `dashboard_user_welcome`. |
| `recipient` | `string` | sim | Endereço de destino (obrigatório) — um e-mail no canal `email`. |
| `channel` | `string` | não | Canal de envio. Padrão `email`. Valores: `email`, `in_app`, `webhook`. |
| `portal_user_id` | `string (uuid)` | não | Usuário do portal a que a notificação se destina; precisa ser da organização (senão, 404). |
| `resource_type` | `string` | não | Tipo do registro relacionado. |
| `resource_id` | `string (uuid)` | não | ID (UUID) do registro relacionado. |
| `scheduled_for` | `string (date-time)` | não | Data e hora ISO 8601 para enviar, em vez de imediatamente. |
| `variables` | `object` | não | Valores das variáveis do modelo: até 100, nomes com letras, dígitos, `_`, `.` ou `-`; valores texto (até 10.000 caracteres), número, booleano ou `null`. |

Outros campos e objetos aninhados: `kobana schema dashboards.notifications.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Notificação criada. |
| `400` | Requisição inválida (campo obrigatório ausente, tipo ou variáveis inválidos). |
| `404` | Recurso não encontrado na organização atual. |

## notifications send

Enviar notificação

```text
kobana dashboards notifications send [ID] [OPÇÕES]
```

`POST /v1/notifications/{id}/send`

Envia agora uma notificação gravada: o primeiro envio de uma `pending` (agendada) ou `skipped`, ou o reenvio de uma `sent`, `delivered`, `read` ou `failed`. Cada chamada envia o e-mail de novo e conta no limite de envios manuais por destinatário (429).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | A notificação atualizada. |
| `400` | Requisição inválida (campo obrigatório ausente, tipo ou variáveis inválidos). |
| `404` | Recurso não encontrado na organização atual. |
| `502` | O provedor de e-mail recusou o envio; o motivo vem na mensagem. |
