---
id: sources
title: "kobana kadu sources"
sidebar_label: "sources"
---

# kobana kadu sources


Comandos de `kobana kadu sources`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`sources list`](#sources-list) | `GET /v1/sources` | Lista repositórios de código |
| [`sources create`](#sources-create) | `POST /v1/sources` | Cadastra repositório de código |
| [`sources get`](#sources-get) | `GET /v1/sources/{id}` | Detalha o repositório de código |
| [`sources update`](#sources-update) | `PATCH /v1/sources/{id}` | Atualiza repositório de código |
| [`sources delete`](#sources-delete) | `DELETE /v1/sources/{id}` | Remove repositório de código (soft delete) |
| [`sources check`](#sources-check) | `POST /v1/sources/{id}/check` | Verifica o acesso ao repositório |

## sources list

Lista repositórios de código

```text
kobana kadu sources list [OPÇÕES]
```

`GET /v1/sources`

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

## sources create

Cadastra repositório de código

```text
kobana kadu sources create [OPÇÕES]
```

`POST /v1/sources`

Se a instalação do GitHub App da organização cobre o repositório, o acesso é verificado na hora. Senão, a resposta traz a deploy key a cadastrar no GitHub antes de verificar o acesso.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim |  |
| `repository` | `string` | sim | `owner/repo` ou a URL do repositório no GitHub. |
| `ref` | `string` | não | Branch ou tag; null = a branch padrão. |
| `label` | `string` | não |  |
| `workspace_id` | `string (uuid)` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.sources.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — já existe repositório vivo com este nome / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — repositório, ref ou workspace inválido |
| `503` | GITHUB_UNAVAILABLE — o GitHub não respondeu |

## sources get

Detalha o repositório de código

```text
kobana kadu sources get [ID] [OPÇÕES]
```

`GET /v1/sources/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## sources update

Atualiza repositório de código

```text
kobana kadu sources update [ID] [OPÇÕES]
```

`PATCH /v1/sources/{id}`

Nome e repositório não mudam: as skills referenciam o nome e a deploy key é do repositório. Mudar a ref zera a verificação de acesso.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `ref` | `string` | não | Branch ou tag; null = a branch padrão. |
| `label` | `string` | não |  |
| `workspace_id` | `string (uuid)` | não |  |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.sources.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — ref ou workspace inválido |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## sources delete

Remove repositório de código (soft delete)

```text
kobana kadu sources delete [ID] [OPÇÕES]
```

`DELETE /v1/sources/{id}`

As Tasks que o declaram falham na próxima execução. Recusado enquanto o `kadu.yaml` declarar o repositório.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declarado no `kadu.yaml` |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## sources check

Verifica o acesso ao repositório

```text
kobana kadu sources check [ID] [OPÇÕES]
```

`POST /v1/sources/{id}/check`

Lista a ref no GitHub com a credencial do repositório, sem clonar, e grava o resultado. Falta de acesso não é erro HTTP: vem em `access_status` e `last_check_error`.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
