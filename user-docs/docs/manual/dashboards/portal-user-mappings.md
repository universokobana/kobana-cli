---
id: portal-user-mappings
title: "kobana dashboards portal-user-mappings"
sidebar_label: "portal-user-mappings"
description: "Consultar mapeamento de usuários do portal"
---

# kobana dashboards portal-user-mappings

Consultar mapeamento de usuários do portal

Comandos de `kobana dashboards portal-user-mappings`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`portal-user-mappings`](#portal-user-mappings) | `GET /v1/portal_user_mappings` | Consultar mapeamento de usuários do portal |
| [`portal-user-mappings update`](#portal-user-mappings-update) | `PUT /v1/portal_user_mappings` | Mapear entidade aos usuários do portal |
| [`portal-user-mappings delete`](#portal-user-mappings-delete) | `DELETE /v1/portal_user_mappings` | Remover mapeamento de usuários do portal |

## portal-user-mappings

Consultar mapeamento de usuários do portal

```text
kobana dashboards portal-user-mappings [OPÇÕES] [COMANDO]
```

`GET /v1/portal_user_mappings`

A entidade que alimenta os usuários do portal do workspace (`mapping`, `null` se nenhuma, com o resultado da última sincronização) e as entidades que podem ser mapeadas, com seus campos.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O mapeamento. |
| `404` | Recurso não encontrado na organização atual. |

## portal-user-mappings update

Mapear entidade aos usuários do portal

```text
kobana dashboards portal-user-mappings update [OPÇÕES]
```

`PUT /v1/portal_user_mappings`

Mapeia uma entidade: um usuário do portal por e-mail dos seus registros, com o ID do titular vindo de um campo. Substitui o mapeamento anterior e sincroniza na hora (devolve `stats`); as próximas sincronizações da entidade mantêm os usuários em dia.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `entity_type_id` | `string (uuid)` | sim | A entidade cujos registros viram usuários do portal. |
| `tenant_id_field_key` | `string` | sim |  |
| `email_field_key` | `string` | sim |  |
| `name_field_key` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema dashboards.portal-user-mappings.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | O mapeamento salvo e o resultado da sincronização. |
| `400` | Corpo da requisição falhou na validação. |
| `404` | Recurso não encontrado na organização atual. |
| `422` | Entidade ou campo inválido (ex.: entidade que sincroniza no acesso). |

## portal-user-mappings delete

Remover mapeamento de usuários do portal

```text
kobana dashboards portal-user-mappings delete [OPÇÕES]
```

`DELETE /v1/portal_user_mappings`

Para de sincronizar; os usuários já criados continuam.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Mapeamento removido (`deleted: false` se não havia). |
| `404` | Recurso não encontrado na organização atual. |
