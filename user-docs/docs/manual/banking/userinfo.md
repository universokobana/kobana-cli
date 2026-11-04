---
id: userinfo
title: "kobana banking userinfo"
sidebar_label: "userinfo"
description: "Informações do Usuário (descontinuado)"
---

# kobana banking userinfo

Informações do Usuário (descontinuado)

Comandos de `kobana banking userinfo`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`userinfo`](#userinfo) | `GET /v1/userinfo` | Informações do Usuário (descontinuado) |

## userinfo

Informações do Usuário (descontinuado)

```text
kobana banking userinfo [OPÇÕES]
```

`GET /v1/userinfo`

Consulta informações da conta acessada.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Usuário encontrado |
| `404` | Usuário não encontrado |
