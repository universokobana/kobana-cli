---
id: operation-approvals
title: "kobana kadu operation-approvals"
sidebar_label: "operation-approvals"
---

# kobana kadu operation-approvals


Comandos de `kobana kadu operation-approvals`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`operation-approvals list`](#operation-approvals-list) | `GET /v1/operation_approvals` | Aprovações por link da pessoa do token |
| [`operation-approvals get`](#operation-approvals-get) | `GET /v1/operation_approvals/{id}` | Detalha uma aprovação por link |

## operation-approvals list

Aprovações por link da pessoa do token

```text
kobana kadu operation-approvals list [OPÇÕES]
```

`GET /v1/operation_approvals`

Operações que um assistente pediu pelo MCP e que só rodam depois que a pessoa confirma pelo link no console. Lista só as aprovações da pessoa do token, na organização do token, mais recentes primeiro. Só leitura: Continuar e Cancelar acontecem no console. Escopo: `kadu/system_events`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Valores: `pending`, `approved`, `executed`, `declined`, `failed`, `expired`. |
| `resource` | `string` | não |  |
| `created_at[gte]` | `string` | não | ISO 8601 (data ou data e hora). |
| `created_at[lte]` | `string` | não | ISO 8601 (data ou data e hora). |
| `cursor` | `string` | não |  |
| `limit` | `number` | não |  |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — filtro ou cursor inválido |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## operation-approvals get

Detalha uma aprovação por link

```text
kobana kadu operation-approvals get [ID] [OPÇÕES]
```

`GET /v1/operation_approvals/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
