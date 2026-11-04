---
id: environments
title: "kobana kadu environments"
sidebar_label: "environments"
---

# kobana kadu environments


Comandos de `kobana kadu environments`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`environments list`](#environments-list) | `GET /v1/environments` | Lista ambientes |
| [`environments create`](#environments-create) | `POST /v1/environments` | Cria ambiente |
| [`environments get`](#environments-get) | `GET /v1/environments/{id}` | Detalha o ambiente |
| [`environments update`](#environments-update) | `PATCH /v1/environments/{id}` | Atualiza ambiente |
| [`environments delete`](#environments-delete) | `DELETE /v1/environments/{id}` | Remove ambiente (soft delete) |

## environments list

Lista ambientes

```text
kobana kadu environments list [OPÇÕES]
```

`GET /v1/environments`

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

## environments create

Cria ambiente

```text
kobana kadu environments create [OPÇÕES]
```

`POST /v1/environments`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | sim |  |
| `color` | `string` | sim |  |
| `branch` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.environments.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — já existe ambiente vivo com este label / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## environments get

Detalha o ambiente

```text
kobana kadu environments get [ID] [OPÇÕES]
```

`GET /v1/environments/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## environments update

Atualiza ambiente

```text
kobana kadu environments update [ID] [OPÇÕES]
```

`PATCH /v1/environments/{id}`

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não |  |
| `color` | `string` | não |  |
| `branch` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.environments.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — label já usado / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## environments delete

Remove ambiente (soft delete)

```text
kobana kadu environments delete [ID] [OPÇÕES]
```

`DELETE /v1/environments/{id}`

Recusado enquanto o ambiente tiver secrets ou agendamentos.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | ENVIRONMENT_IN_USE — `details: { secrets, schedules }` / REPO_MANAGED — declarado no `kadu.yaml`: remova-o de lá |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
