---
id: workspaces
title: "kobana dunning workspaces"
sidebar_label: "workspaces"
description: "Listar workspaces"
---

# kobana dunning workspaces

Listar workspaces

Comandos de `kobana dunning workspaces`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`workspaces`](#workspaces) | `GET /v1/workspaces` | Listar workspaces |

## workspaces

Listar workspaces

```text
kobana dunning workspaces [OPÇÕES]
```

`GET /v1/workspaces`

Lista os workspaces que o token alcança — todos os da organização para um token de integração. Envie o `id` (ou `public_id`) no header `X-Workspace-Id` das demais chamadas; ele é opcional enquanto a organização tiver um só workspace. Esta rota não exige o header.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Workspaces alcançáveis. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
