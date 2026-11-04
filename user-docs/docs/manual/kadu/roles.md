---
id: roles
title: "kobana kadu roles"
sidebar_label: "roles"
---

# kobana kadu roles


Comandos de `kobana kadu roles`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`roles list`](#roles-list) | `GET /v1/roles` | Lista papéis |
| [`roles create`](#roles-create) | `POST /v1/roles` | Cria papel |

## roles list

Lista papéis

```text
kobana kadu roles list [OPÇÕES]
```

`GET /v1/roles`

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

## roles create

Cria papel

```text
kobana kadu roles create [OPÇÕES]
```

`POST /v1/roles`

Permissões no formato `*`, `agent.*`, `agent.<recurso>.*` ou `agent.<recurso>.<ação>`. Num token de pessoa, nenhuma pode ir além do papel de quem cria.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `permissions` | `array<string>` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.roles.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
