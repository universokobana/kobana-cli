---
id: me
title: "kobana finance me"
sidebar_label: "me"
description: "Obter a identidade do token"
---

# kobana finance me

Obter a identidade do token

Comandos de `kobana finance me`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Obter a identidade do token |

## me

Obter a identidade do token

```text
kobana finance me [OPÇÕES]
```

`GET /v1/me`

Returns the person, organization and token details behind the Bearer token, so an integrator can test that the configured token is valid. Any valid token works: no scope and no `X-Workspace-Id` header are needed. Never returns the token itself.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The identity behind the token. |
