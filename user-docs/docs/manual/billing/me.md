---
id: me
title: "kobana billing me"
sidebar_label: "me"
description: "Identidade do token atual"
---

# kobana billing me

Identidade do token atual

Comandos de `kobana billing me`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidade do token atual |

## me

Identidade do token atual

```text
kobana billing me [OPÇÕES]
```

`GET /v1/me`

Retorna quem é o dono do token Bearer (pessoa e organização) e os escopos e a expiração do próprio token. Serve para testar se o token configurado é válido. Nunca devolve o token nem segredos. `user` é `null` quando o token não é de uma pessoa.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Identidade do token. |
| `404` | Recurso não encontrado na organização atual. |
