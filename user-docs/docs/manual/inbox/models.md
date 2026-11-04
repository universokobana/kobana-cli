---
id: models
title: "kobana inbox models"
sidebar_label: "models"
description: "Listar modelos LLM disponíveis"
---

# kobana inbox models

Listar modelos LLM disponíveis

Comandos de `kobana inbox models`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`models`](#models) | `GET /v1/models` | Listar modelos LLM disponíveis |

## models

Listar modelos LLM disponíveis

```text
kobana inbox models [OPÇÕES]
```

`GET /v1/models`

Modelos aceitos em `Agent.llm_model`, com preço por token: o catálogo do AI Gateway restrito aos modelos que têm preço, de modo que todo modelo listado é custeado e contado contra o limite do workspace.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | não | Segredo compartilhado entre o CloudFront e a origem. Obrigatório em produção; ignorado em desenvolvimento a menos que `CF_ORIGIN_SHARED_SECRET` esteja definido. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Catálogo. |
| `502` | `GATEWAY_ERROR`: não foi possível obter o catálogo do AI Gateway. |
