---
id: helpers
title: "kobana banking +helpers"
sidebar_label: "Helpers"
---

# Helpers — banking

`+verb` shortcuts that build the request from named flags. See [Helpers](/usage/helpers).

## +emitir

Issue a bank slip with a simplified interface

```text
kobana banking +emitir --valor <VALOR> --vencimento <DATA> --nome <NOME> --cpf-cnpj <DOC> --carteira <ID> [OPTIONS]
```

| Helper options | Description |
|---|---|
| `--valor <VALOR>` | Bank slip amount (e.g. 150.50) |
| `--vencimento <DATA>` | Due date (YYYY-MM-DD) |
| `--nome <NOME>` | Payer name |
| `--cpf-cnpj <DOC>` | Payer CPF or CNPJ |
| `--carteira <ID>` | Billing wallet ID |
| `--descricao <TEXT>` | Bank slip description/instructions |

## +cancelar-lote

Cancel several bank slips at once

```text
kobana banking +cancelar-lote --ids <IDS> [OPTIONS]
```

| Helper options | Description |
|---|---|
| `--ids <IDS>` | Comma-separated bank slip IDs |

## +cobrar

Create a Pix charge with a simplified interface

```text
kobana banking +cobrar --valor <VALOR> --vencimento <DATA> --conta-pix <UID> [OPTIONS]
```

| Helper options | Description |
|---|---|
| `--valor <VALOR>` | Charge amount |
| `--vencimento <DATA>` | Due date and time, ISO 8601 (e.g. 2026-12-02T10:00:00-03:00) |
| `--conta-pix <UID>` | Pix account UID |
| `--nome <NOME>` | Payer name (with --cpf-cnpj) |
| `--cpf-cnpj <DOC>` | Payer CPF or CNPJ (with --nome) |
| `--descricao <TEXT>` | Charge description |
