---
id: imports
title: "kobana dunning imports"
sidebar_label: "imports"
---

# kobana dunning imports


Comandos de `kobana dunning imports`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`imports list`](#imports-list) | `GET /v1/imports` | Listar importações |
| [`imports create`](#imports-create) | `POST /v1/imports` | Criar importação |
| [`imports get`](#imports-get) | `GET /v1/imports/{id}` | Consultar importação |

## imports list

Listar importações

```text
kobana dunning imports list [OPÇÕES]
```

`GET /v1/imports`

Histórico das 50 importações mais recentes da organização (sem paginação). O conteúdo do arquivo e os erros por linha ficam de fora; consulte GET /imports/\{id\} para o detalhe.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lista das importações mais recentes. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |

## imports create

Criar importação

```text
kobana dunning imports create [OPÇÕES]
```

`POST /v1/imports`

Sobe uma planilha CSV de clientes ou cobranças e agenda o processamento assíncrono (fila). O corpo é JSON com o CSV em texto puro no campo `content` (máx. 2 MB). Linhas inválidas não derrubam o lote: são registradas em `errors` (`[{ line, error }]`) e o restante é processado. Acompanhe pelo GET /imports/\{id\}. Responde 202. Exige o nível de acesso `import` (ou `all`) — `write` não basta.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `resource_type` | `string` | sim | Tipo de recurso importado (`people` = clientes \| `charges` = cobranças). Valores: `people`, `charges`. |
| `content` | `string` | sim | Conteúdo do CSV como texto puro (UTF-8), incluindo a linha de cabeçalho. Máx. 2 MB. |
| `file_name` | `string` | não | Nome do arquivo enviado. |

Outros campos e objetos aninhados: `kobana schema dunning.imports.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `202` | Importação aceita e enfileirada para processamento. |
| `400` | Requisição inválida (erro de validação). |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Arquivo acima de 2 MB — divida a planilha em partes menores. |

## imports get

Consultar importação

```text
kobana dunning imports get [ID] [OPÇÕES]
```

`GET /v1/imports/{id}`

Retorna o status de uma importação com contadores de progresso e os erros por linha rejeitada (`errors: [{ line, error }]`; a linha 1 é o cabeçalho, os dados começam na linha 2).

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | não | Workspace em que a chamada age (`id` ou `public_id`, listados em `GET /v1/workspaces`). Opcional enquanto a organização tem um só workspace; com dois ou mais, sem ele a resposta é `400 WORKSPACE_REQUIRED`. Um workspace de outra organização é `403 UNKNOWN_WORKSPACE`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Importação encontrada. |
| `404` | Recurso inexistente ou fora da sua organização. |
| `413` | Corpo da requisição acima de 2 MB. |
