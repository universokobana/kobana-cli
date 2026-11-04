---
id: helpers
title: "kobana banking +helpers"
sidebar_label: "Helpers"
---

# Helpers — banking

Atalhos `+verbo` que montam a requisição a partir de flags nomeadas. Veja [Helpers](/usage/helpers).

## +emitir

Emitir boleto com interface simplificada

```text
kobana banking +emitir --valor <VALOR> --vencimento <DATA> --nome <NOME> --cpf-cnpj <DOC> --carteira <ID> [OPÇÕES]
```

| Opções do helper | Descrição |
|---|---|
| `--valor <VALOR>` | Valor do boleto (ex.: 150.50) |
| `--vencimento <DATA>` | Data de vencimento (AAAA-MM-DD) |
| `--nome <NOME>` | Nome do pagador |
| `--cpf-cnpj <DOC>` | CPF ou CNPJ do pagador |
| `--carteira <ID>` | ID da carteira de cobrança |
| `--descricao <TEXT>` | Descrição/instruções do boleto |

## +cancelar-lote

Cancelar múltiplos boletos de uma vez

```text
kobana banking +cancelar-lote --ids <IDS> [OPÇÕES]
```

| Opções do helper | Descrição |
|---|---|
| `--ids <IDS>` | IDs dos boletos separados por vírgula |

## +cobrar

Criar cobrança Pix com interface simplificada

```text
kobana banking +cobrar --valor <VALOR> --vencimento <DATA> --conta-pix <UID> [OPÇÕES]
```

| Opções do helper | Descrição |
|---|---|
| `--valor <VALOR>` | Valor da cobrança |
| `--vencimento <DATA>` | Data e hora de vencimento, ISO 8601 (ex.: 2026-12-02T10:00:00-03:00) |
| `--conta-pix <UID>` | UID da conta Pix |
| `--nome <NOME>` | Nome do pagador (com --cpf-cnpj) |
| `--cpf-cnpj <DOC>` | CPF ou CNPJ do pagador (com --nome) |
| `--descricao <TEXT>` | Descrição da cobrança |
