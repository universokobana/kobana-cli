---
id: settings
title: "kobana dashboards settings"
sidebar_label: "settings"
description: "Consultar configurações da organização"
---

# kobana dashboards settings

Consultar configurações da organização

Comandos de `kobana dashboards settings`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`settings`](#settings) | `GET /v1/settings` | Consultar configurações da organização |
| [`settings update`](#settings-update) | `PUT /v1/settings` | Atualizar configurações da organização |

## settings

Consultar configurações da organização

```text
kobana dashboards settings [OPÇÕES] [COMANDO]
```

`GET /v1/settings`

Nome, e-mail de suporte, fuso horário e as opções de cadastros da organização. Recurso da organização.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | As configurações. |
| `404` | Recurso não encontrado na organização atual. |

## settings update

Atualizar configurações da organização

```text
kobana dashboards settings update [OPÇÕES]
```

`PUT /v1/settings`

Altera parcialmente as configurações: envie só o que muda. Campos desconhecidos são recusados (400).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | não |  |
| `support_email` | `string (email)` | não |  |
| `timezone` | `string` | não | Fuso horário IANA, ex.: `America/Sao_Paulo`. |
| `crm_uppercase_names` | `boolean` | não | Salvar nomes em MAIÚSCULAS. |
| `crm_allow_edit_public_id` | `boolean` | não |  |
| `crm_allow_edit_document` | `boolean` | não |  |
| `crm_allow_create_customers` | `boolean` | não |  |
| `crm_allow_create_portal_users` | `boolean` | não | Permitir criar usuários do portal pelo painel e por importação. |
| `crm_allow_edit_portal_user_email` | `boolean` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.settings.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | As configurações. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
