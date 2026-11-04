---
id: bank-billet-batch-exports
title: "kobana banking bank-billet-batch-exports"
sidebar_label: "bank-billet-batch-exports"
---

# kobana banking bank-billet-batch-exports


Comandos de `kobana banking bank-billet-batch-exports`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`bank-billet-batch-exports get`](#bank-billet-batch-exports-get) | `GET /v1/bank_billet_batch_exports/{id}` | Arquivo de Exportação do Lote |

## bank-billet-batch-exports get

Arquivo de Exportação do Lote

```text
kobana banking bank-billet-batch-exports get [ID] [OPÇÕES]
```

`GET /v1/bank_billet_batch_exports/{id}`

Arquivo de exportação do lote.

Caso o arquivo ainda não tenha sido processado a resposta será 404. Caso o arquivo já esteja pronto para download a resposta será um redirect para o download do arquivo.

**Argumentos do caminho:** `id`

**Respostas**

| HTTP | Descrição |
|---|---|
| `302` | Exportação de lote gerada com sucesso |
| `404` | Exportação de lote não processada |
