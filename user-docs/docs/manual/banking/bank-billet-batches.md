---
id: bank-billet-batches
title: "kobana banking bank-billet-batches"
sidebar_label: "bank-billet-batches"
---

# kobana banking bank-billet-batches


Comandos de `kobana banking bank-billet-batches`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`bank-billet-batches list`](#bank-billet-batches-list) | `GET /v1/bank_billet_batches` | Listar Lotes |
| [`bank-billet-batches create`](#bank-billet-batches-create) | `POST /v1/bank_billet_batches` | Criar Lote |
| [`bank-billet-batches get`](#bank-billet-batches-get) | `GET /v1/bank_billet_batches/{id}/` | Informações do Lote |
| [`bank-billet-batches delete`](#bank-billet-batches-delete) | `DELETE /v1/bank_billet_batches/{id}/` | Excluir o lote |
| [`bank-billet-batches add-bank-billets`](#bank-billet-batches-add-bank-billets) | `PUT /v1/bank_billet_batches/{id}/add_bank_billets` | Incluir Boletos no Lote |
| [`bank-billet-batches pdf`](#bank-billet-batches-pdf) | `POST /v1/bank_billet_batches/{id}/pdf` | Exportar Lote no Formato .pdf |
| [`bank-billet-batches remove-bank-billet`](#bank-billet-batches-remove-bank-billet) | `DELETE /v1/bank_billet_batches/{id}/remove_bank_billet` | Excluir Boleto do Lote |
| [`bank-billet-batches zip`](#bank-billet-batches-zip) | `POST /v1/bank_billet_batches/{id}/zip` | Exportar Lote no Formato .zip |

## bank-billet-batches list

Listar Lotes

```text
kobana banking bank-billet-batches list [OPÇÕES]
```

`GET /v1/bank_billet_batches`

Listar Lotes.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Sucesso na listagem dos lotes |

## bank-billet-batches create

Criar Lote

```text
kobana banking bank-billet-batches create [OPÇÕES]
```

`POST /v1/bank_billet_batches`

Criar Lote.

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | `string` | sim | Nome do lote. |
| `bank_billet_ids` | `string` | não | IDs dos Boletos separados por vírgula. |

Outros campos e objetos aninhados: `kobana schema banking.bank-billet-batches.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Lote criado com sucesso |
| `422` | Dados inválidos |

## bank-billet-batches get

Informações do Lote

```text
kobana banking bank-billet-batches get [ID] [OPÇÕES]
```

`GET /v1/bank_billet_batches/{id}/`

Consulta informações específicas de um Lote.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Lote encontrado |
| `404` | Lote não encontrado |

## bank-billet-batches delete

Excluir o lote

```text
kobana banking bank-billet-batches delete [ID] [OPÇÕES]
```

`DELETE /v1/bank_billet_batches/{id}/`

Excluir lote.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Lote excluído |
| `404` | Lote não encontrado |

## bank-billet-batches add-bank-billets

Incluir Boletos no Lote

```text
kobana banking bank-billet-batches add-bank-billets [ID] [OPÇÕES]
```

`PUT /v1/bank_billet_batches/{id}/add_bank_billets`

Incluir boletos no lote.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `bank_billet_ids` | `string` | sim | IDs dos Boletos separados por vírgula. |

Outros campos e objetos aninhados: `kobana schema banking.bank-billet-batches.add-bank-billets`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Boletos adicionados com sucesso |
| `404` | Lote não encontrado |
| `422` | Dados inválidos |

## bank-billet-batches pdf

Exportar Lote no Formato .pdf

```text
kobana banking bank-billet-batches pdf [ID] [OPÇÕES]
```

`POST /v1/bank_billet_batches/{id}/pdf`

Exportar lote no formato .pdf.

Após a requisição a exportação entra numa fila para gerar o arquivo do lote. A URL do arquivo do lote será retornada pelo endpoint de Exportação do Lote.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Solicitação de exportação do lote criada com sucesso |
| `404` | Lote não encontrado |

## bank-billet-batches remove-bank-billet

Excluir Boleto do Lote

```text
kobana banking bank-billet-batches remove-bank-billet [ID] [OPÇÕES]
```

`DELETE /v1/bank_billet_batches/{id}/remove_bank_billet`

Excluir boleto do lote.

**Argumentos do caminho:** `id`

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `bank_billet_id` | `integer` | sim | ID dos Boleto separados por vírgula. |

Outros campos e objetos aninhados: `kobana schema banking.bank-billet-batches.remove-bank-billet`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `204` | Boleto removido com sucesso |
| `404` | Lote não encontrado |

## bank-billet-batches zip

Exportar Lote no Formato .zip

```text
kobana banking bank-billet-batches zip [ID] [OPÇÕES]
```

`POST /v1/bank_billet_batches/{id}/zip`

Exportar lote no formato .zip.

Após a requisição a exportação entra numa fila para gerar o arquivo do lote. A URL do arquivo do lote será retornada pelo endpoint de Exportação do Lote.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Solicitação de exportação do lote criada com sucesso |
| `404` | Lote não encontrado |
