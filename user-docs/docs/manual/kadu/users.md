---
id: users
title: "kobana kadu users"
sidebar_label: "users"
description: "Membros da organização (até 100)"
---

# kobana kadu users

Membros da organização (até 100)

Comandos de `kobana kadu users`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`users`](#users) | `GET /v1/users` | Membros da organização (até 100) |

## users

Membros da organização (até 100)

```text
kobana kadu users [OPÇÕES]
```

`GET /v1/users`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
