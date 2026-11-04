---
id: me
title: "kobana dashboards me"
sidebar_label: "me"
description: "Identidade do token atual"
---

# kobana dashboards me

Identidade do token atual

Comandos de `kobana dashboards me`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidade do token atual |

## me

Identidade do token atual

```text
kobana dashboards me [OPÇÕES]
```

`GET /v1/me`

Retorna quem está por trás do token Bearer: a pessoa (`user`, `null` para um token de máquina), a organização e os escopos, o nível de acesso e a expiração do próprio token. Serve para validar a credencial configurada numa integração. Nunca devolve o token, hashes ou chaves.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada atua: o `id` ou o `public_id` mostrados em Workspaces no dashboard. Dentro da organização do token, os dados ficam em workspaces. Pode ser omitido enquanto a organização tem um único workspace; com dois ou mais, a falta dele retorna `400 workspace_required`. Um workspace que não é da organização retorna `403 unknown_workspace`. Com um token de pessoa, só os workspaces a que ela tem acesso (proprietários da organização alcançam todos); os demais retornam `403 workspace_access_denied`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Identidade do token autenticado. |
| `404` | Recurso não encontrado na organização atual. |
