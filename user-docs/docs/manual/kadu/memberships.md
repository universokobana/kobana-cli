---
id: memberships
title: "kobana kadu memberships"
sidebar_label: "memberships"
description: "Lista vínculos"
---

# kobana kadu memberships

Lista vínculos

Comandos de `kobana kadu memberships`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`memberships`](#memberships) | `GET /v1/memberships` | Lista vínculos |

## memberships

Lista vínculos

```text
kobana kadu memberships [OPÇÕES]
```

`GET /v1/memberships`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `cursor` | `string` | não |  |
| `limit` | `number` | não |  |
| `sort_by` | `string` | não | Campo em snake_case: created_at (padrão), updated_at ou um campo filtrável. |
| `sort_order` | `string` | não | Padrão: desc. Valores: `asc`, `desc`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
