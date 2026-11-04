---
id: files
title: "kobana kadu files"
sidebar_label: "files"
---

# kobana kadu files


Comandos de `kobana kadu files`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`files list`](#files-list) | `GET /v1/files` | Lista arquivos |
| [`files create`](#files-create) | `POST /v1/files` | Inicia upload de arquivo de entrada |
| [`files get`](#files-get) | `GET /v1/files/{id}` | Detalha o arquivo |
| [`files confirm`](#files-confirm) | `POST /v1/files/{id}/confirm` | Confirma o upload (pending → uploaded) |
| [`files download`](#files-download) | `GET /v1/files/{id}/download` | URL temporária de download |

## files list

Lista arquivos

```text
kobana kadu files list [OPÇÕES]
```

`GET /v1/files`

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

## files create

Inicia upload de arquivo de entrada

```text
kobana kadu files create [OPÇÕES]
```

`POST /v1/files`

Fluxo em três passos: (1) este POST cria o arquivo `pending` e devolve `upload_url`; (2) `PUT` dos bytes em `upload_url` com o mesmo `Content-Type`; (3) `POST /files/{id}/confirm`. O id vai em `input_file_ids` do `POST /tasks/{id}/run` (a Task precisa aceitar arquivos de entrada). Tamanho máximo: 100 MB.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `task_id` | `string (uuid)` | sim |  |
| `filename` | `string` | sim |  |
| `size_bytes` | `integer` | sim |  |
| `content_type` | `string` | não |  |

Outros campos e objetos aninhados: `kobana schema kadu.files.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | created |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (ou CONFLICT da rota) |
| `422` | TASK_DOES_NOT_ACCEPT_FILES / VALIDATION_ERROR — Task fora da organização |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## files get

Detalha o arquivo

```text
kobana kadu files get [ID] [OPÇÕES]
```

`GET /v1/files/{id}`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## files confirm

Confirma o upload (pending → uploaded)

```text
kobana kadu files confirm [ID] [OPÇÕES]
```

`POST /v1/files/{id}/confirm`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | CONFLICT — não é um arquivo de entrada pendente / IDEMPOTENCY_KEY_REUSED |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |

## files download

URL temporária de download

```text
kobana kadu files download [ID] [OPÇÕES]
```

`GET /v1/files/{id}/download`

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `409` | FILE_NOT_UPLOADED |
| `503` | AS_UNAVAILABLE — a verificação do token está indisponível (Authorization Server fora do ar ou configuração ausente). Tente de novo (`Retry-After`). |
