---
id: permissions
title: "kobana dashboards permissions"
sidebar_label: "permissions"
description: "Listar permissões"
---

# kobana dashboards permissions

Listar permissões

Comandos de `kobana dashboards permissions`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`permissions`](#permissions) | `GET /v1/permissions` | Listar permissões |

## permissions

Listar permissões

```text
kobana dashboards permissions [OPÇÕES]
```

`GET /v1/permissions`

Catálogo das permissões do dashboard que um membro ou um papel da equipe pode ter, agrupadas por recurso — o vocabulário de `permissions` em `/dashboard_users` e `/team_roles`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Grupos de permissões. |
| `404` | Recurso não encontrado na organização atual. |
