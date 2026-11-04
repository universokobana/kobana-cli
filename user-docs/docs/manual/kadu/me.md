---
id: me
title: "kobana kadu me"
sidebar_label: "me"
description: "Identidade por trás do token"
---

# kobana kadu me

Identidade por trás do token

Comandos de `kobana kadu me`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidade por trás do token |

## me

Identidade por trás do token

```text
kobana kadu me [OPÇÕES]
```

`GET /v1/me`

Devolve a pessoa, a organização e o token (tipo, escopos, nível de acesso, expiração) da credencial Bearer. Serve para testar se o token configurado é válido: é a única rota isenta da checagem de escopo, então qualquer token válido responde. `user` é a pessoa do `sub` quando o token traz e-mail, e null num token de máquina. Nunca devolve o token.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
