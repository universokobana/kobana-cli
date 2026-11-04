---
id: secrets
title: "kobana kadu secrets"
sidebar_label: "secrets"
---

# kobana kadu secrets


Comandos de `kobana kadu secrets`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`secrets list`](#secrets-list) | `GET /v1/secrets` | Lista secrets |
| [`secrets create`](#secrets-create) | `POST /v1/secrets` | Cria secret |
| [`secrets get`](#secrets-get) | `GET /v1/secrets/{id}` | Detalha o secret |
| [`secrets update`](#secrets-update) | `PATCH /v1/secrets/{id}` | Atualiza secret |
| [`secrets delete`](#secrets-delete) | `DELETE /v1/secrets/{id}` | Remove secret (soft delete) |

## secrets list

Lista secrets

```text
kobana kadu secrets list [OPÇÕES]
```

`GET /v1/secrets`

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

## secrets create

Cria secret

```text
kobana kadu secrets create [OPÇÕES]
```

`POST /v1/secrets`

O valor é gravado encriptado e nunca devolvido.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `environment_id` | `string (uuid)` | sim |  |
| `key` | `string` | sim |  |
| `value` | `string` | sim |  |
| `workspace_id` | `string (uuid)` | não |  |
| `notes` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.secrets.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | VALIDATION_ERROR — ambiente ou workspace fora da organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## secrets get

Detalha o secret

```text
kobana kadu secrets get [ID] [OPÇÕES]
```

`GET /v1/secrets/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## secrets update

Atualiza secret

```text
kobana kadu secrets update [ID] [OPÇÕES]
```

`PATCH /v1/secrets/{id}`

`value` ausente ou vazio mantém o valor gravado.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `environment_id` | `string (uuid)` | não |  |
| `workspace_id` | `string (uuid)` | não |  |
| `key` | `string` | não |  |
| `value` | `string` | não |  |
| `notes` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.secrets.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | VALIDATION_ERROR — ambiente ou workspace fora da organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## secrets delete

Remove secret (soft delete)

```text
kobana kadu secrets delete [ID] [OPÇÕES]
```

`DELETE /v1/secrets/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
