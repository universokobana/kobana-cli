---
id: me
title: "kobana banking me"
sidebar_label: "me"
description: "Informações da conta"
---

# kobana banking me

Informações da conta

Comandos de `kobana banking me`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`me`](#me) | `GET /v2/me` | Obter informações do usuário autenticado |

## me

Obter informações do usuário autenticado

```text
kobana banking me [OPÇÕES]
```

`GET /v2/me`

Retorna informações sobre o usuário autenticado, incluindo dados da conta, usuário e token de acesso.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Informações do usuário sem usuário associado ao token |
