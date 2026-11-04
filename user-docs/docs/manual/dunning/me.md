---
id: me
title: "kobana dunning me"
sidebar_label: "me"
description: "Identidade do token"
---

# kobana dunning me

Identidade do token

Comandos de `kobana dunning me`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidade do token |

## me

Identidade do token

```text
kobana dunning me [OPÇÕES]
```

`GET /v1/me`

Retorna quem é o dono do token Bearer enviado. Serve para testar se o token configurado é válido. Nunca devolve o token nem qualquer segredo.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Identidade do token. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
