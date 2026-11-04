---
id: volumes
title: "kobana kadu volumes"
sidebar_label: "volumes"
---

# kobana kadu volumes


Comandos de `kobana kadu volumes`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`volumes list`](#volumes-list) | `GET /v1/volumes` | Lista volumes |
| [`volumes create`](#volumes-create) | `POST /v1/volumes` | Cria volume |
| [`volumes get`](#volumes-get) | `GET /v1/volumes/{id}` | Detalha o volume |
| [`volumes update`](#volumes-update) | `PATCH /v1/volumes/{id}` | Atualiza volume |
| [`volumes delete`](#volumes-delete) | `DELETE /v1/volumes/{id}` | Remove volume (soft delete) |
| [`volumes files`](#volumes-files) | `GET /v1/volumes/{id}/files` | Lista os arquivos do volume |
| [`volumes syncs`](#volumes-syncs) | `GET /v1/volumes/{id}/syncs` | Lista os syncs do volume |

## volumes list

Lista volumes

```text
kobana kadu volumes list [OPÇÕES]
```

`GET /v1/volumes`

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

## volumes create

Cria volume

```text
kobana kadu volumes create [OPÇÕES]
```

`POST /v1/volumes`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim |  |
| `environment_id` | `string (uuid)` | não |  |
| `label` | `string` | não |  |
| `workspace_id` | `string (uuid)` | não |  |
| `quota_mb` | `integer` | não | Padrão: 1024. |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.volumes.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | CONFLICT — já existe volume vivo com este nome no ambiente / IDEMPOTENCY_KEY_REUSED |
| `422` | VALIDATION_ERROR — ambiente ou workspace de outra organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## volumes get

Detalha o volume

```text
kobana kadu volumes get [ID] [OPÇÕES]
```

`GET /v1/volumes/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## volumes update

Atualiza volume

```text
kobana kadu volumes update [ID] [OPÇÕES]
```

`PATCH /v1/volumes/{id}`

Nome e ambiente não mudam: as skills e o `kadu.yaml` referenciam o volume por eles.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `label` | `string` | não |  |
| `workspace_id` | `string (uuid)` | não |  |
| `quota_mb` | `integer` | não | Padrão: 1024. |
| `custom_metadata` | `object` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.volumes.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## volumes delete

Remove volume (soft delete)

```text
kobana kadu volumes delete [ID] [OPÇÕES]
```

`DELETE /v1/volumes/{id}`

O conteúdo fica guardado por 30 dias antes do expurgo. Recusado enquanto o `kadu.yaml` declarar o volume ou uma execução estiver gravando nele.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | REPO_MANAGED — declarado no `kadu.yaml` / VOLUME_IN_USE — execução gravando nele |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## volumes files

Lista os arquivos do volume

```text
kobana kadu volumes files [ID] [OPÇÕES]
```

`GET /v1/volumes/{id}/files`

Em ordem de caminho. Só metadados: o conteúdo nunca é devolvido.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `path_prefix` | `string` | não |  |
| `cursor` | `string` | não |  |
| `limit` | `number` | não | Padrão: 200. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — cursor inválido |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## volumes syncs

Lista os syncs do volume

```text
kobana kadu volumes syncs [ID] [OPÇÕES]
```

`GET /v1/volumes/{id}/syncs`

As execuções que montaram o volume, da mais recente, com o desfecho de cada sync.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `limit` | `number` | não |  |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
