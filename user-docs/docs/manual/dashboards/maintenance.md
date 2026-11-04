---
id: maintenance
title: "kobana dashboards maintenance"
sidebar_label: "maintenance"
---

# kobana dashboards maintenance


Comandos de `kobana dashboards maintenance`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`maintenance uppercase-names create`](#maintenance-uppercase-names-create) | `POST /v1/maintenance/uppercase_names` | Colocar nomes em maiúsculas |

## maintenance uppercase-names create

Colocar nomes em maiúsculas

```text
kobana dashboards maintenance uppercase-names create [OPÇÕES]
```

`POST /v1/maintenance/uppercase_names`

Converte para maiúsculas o nome de todo contato vinculado a um usuário do portal do workspace. Não é reversível (a grafia anterior não é guardada).

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Quantos nomes mudaram. |
| `404` | Recurso não encontrado na organização atual. |
