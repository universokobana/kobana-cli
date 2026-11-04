---
id: syntax
title: Sintaxe dos comandos
sidebar_label: Sintaxe
sidebar_position: 1
description: Como um comando é montado — produto, recurso, método — e como passar parâmetros, corpo e arquivos.
---

# Sintaxe dos comandos

```bash
kobana <produto> <recurso> <método> [argumentos] [flags]
```

- **`<produto>`**: `banking`, `finance`, `billing`, `inbox`, `dunning`, `dashboards` ou `kadu` ([Produtos](/reference/products)).
- **`<recurso>`**: pode ser aninhado — `bank-billets`, `charge pix`, `webhooks deliveries`.
- **`<método>`**: a ação — `list`, `get`, `create`, `update`, `delete` ou uma ação do recurso, como `cancel`, `pause`, `replay`.

Nenhum comando é escrito à mão: o CLI lê as especificações OpenAPI embutidas no binário e monta a árvore a cada execução. Por isso o [Manual](/manual) é gerado do próprio binário.

## Como os nomes são escolhidos

| Rota da API | Comando |
|---|---|
| `GET /v1/bank_billets` | `kobana banking bank-billets list` |
| `GET /v1/bank_billets/{id}` | `kobana banking bank-billets get 123` |
| `POST /v2/charge/pix` | `kobana banking charge pix create` |
| `PUT /v1/bank_billets/{id}/cancel` | `kobana banking bank-billets cancel 123` |

- **A versão da API nunca entra no comando.** `/v1` e `/v2` ficam por conta do CLI.
- **Recursos que só leem rodam pelo nome**: `kobana banking me`, `kobana billing organization`.
- **Ações cujo nome já é um verbo** rodam sem o verbo HTTP: `kobana banking webhook-deliveries resend`.
- Um `delete` nunca fica implícito.
- Nomes antigos continuam valendo como alias, fora da ajuda.

## Identificadores do caminho

Os ids da rota vão como **argumentos**, na ordem em que aparecem:

```bash
kobana banking bank-billets get 123
kobana banking charge pix commands get PIX_UID COMMAND_UID
```

`--params '{"id": "123"}'` continua valendo; quando os dois aparecem, o argumento vence.

## Parâmetros e corpo

Parâmetros de URL, de consulta e de cabeçalho vão em `--params`; o corpo da requisição vai em `--json`:

```bash
kobana billing subscriptions list --params '{"status": "active", "per_page": 50}'
kobana banking charge pix create --json '{"amount": 99.90, "pix_account_uid": "UID"}'
```

## Arquivos

`--file campo=caminho`, repetível, envia a requisição como `multipart/form-data`. Cada campo de `--json` vira uma parte de texto; o tipo do arquivo vem da extensão (`.eml` vai como `message/rfc822`). Só os endpoints que recebem formulário aceitam `--file`.

```bash
kobana inbox emails create --file file=./mensagem.eml --json '{"inbox_id": "INBOX_ID"}'
kobana finance attachments create --file file=./nota.pdf --json '{"title": "NF 123"}'
```

## Ajuda em qualquer nível

```bash
kobana --help
kobana finance --help
kobana finance payables --help
kobana finance payables list --help
```

Veja todas as flags em [Opções globais](/manual/commands/global-options).
