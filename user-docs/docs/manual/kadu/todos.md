---
id: todos
title: "kobana kadu todos"
sidebar_label: "todos"
---

# kobana kadu todos


Comandos de `kobana kadu todos`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`todos update`](#todos-update) | `PATCH /v1/todos/{id}` | Resolve pendência |

## todos update

Resolve pendência

```text
kobana kadu todos update [ID] [OPÇÕES]
```

`PATCH /v1/todos/{id}`

A única transição pela API: `status: resolved`.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `status` | `string` | não | Valores: `resolved`. |

Outros campos e objetos aninhados: `kobana schema kadu.todos.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
