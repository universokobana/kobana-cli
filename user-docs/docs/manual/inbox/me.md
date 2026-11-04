---
id: me
title: "kobana inbox me"
sidebar_label: "me"
description: "Identidade por trás do token"
---

# kobana inbox me

Identidade por trás do token

Comandos de `kobana inbox me`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidade por trás do token |

## me

Identidade por trás do token

```text
kobana inbox me [OPÇÕES]
```

`GET /v1/me`

Quem o token Bearer representa: a pessoa (`null` para um token de máquina), a organização e o alcance do próprio token. Use para testar se um token configurado é válido. Não exige escopo de recurso: qualquer token que seja verificado recebe resposta. O token em si nunca é devolvido.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Identidade. |
