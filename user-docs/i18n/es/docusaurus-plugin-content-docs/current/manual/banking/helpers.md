---
id: helpers
title: "kobana banking +helpers"
sidebar_label: "Helpers"
---

# Helpers — banking

Atajos `+verbo` que arman la solicitud a partir de flags con nombre. Mira [Helpers](/usage/helpers).

## +emitir

Emitir un boleto con una interfaz simplificada

```text
kobana banking +emitir --valor <VALOR> --vencimento <DATA> --nome <NOME> --cpf-cnpj <DOC> --carteira <ID> [OPCIONES]
```

| Opciones del helper | Descripción |
|---|---|
| `--valor <VALOR>` | Monto del boleto (ej.: 150.50) |
| `--vencimento <DATA>` | Fecha de vencimiento (AAAA-MM-DD) |
| `--nome <NOME>` | Nombre del pagador |
| `--cpf-cnpj <DOC>` | CPF o CNPJ del pagador |
| `--carteira <ID>` | ID de la cartera de cobro |
| `--descricao <TEXT>` | Descripción/instrucciones del boleto |

## +cancelar-lote

Cancelar varios boletos a la vez

```text
kobana banking +cancelar-lote --ids <IDS> [OPCIONES]
```

| Opciones del helper | Descripción |
|---|---|
| `--ids <IDS>` | IDs de los boletos separados por comas |

## +cobrar

Crear un cobro Pix con una interfaz simplificada

```text
kobana banking +cobrar --valor <VALOR> --vencimento <DATA> --conta-pix <UID> [OPCIONES]
```

| Opciones del helper | Descripción |
|---|---|
| `--valor <VALOR>` | Monto del cobro |
| `--vencimento <DATA>` | Fecha y hora de vencimiento, ISO 8601 (ej.: 2026-12-02T10:00:00-03:00) |
| `--conta-pix <UID>` | UID de la cuenta Pix |
| `--nome <NOME>` | Nombre del pagador (con --cpf-cnpj) |
| `--cpf-cnpj <DOC>` | CPF o CNPJ del pagador (con --nome) |
| `--descricao <TEXT>` | Descripción del cobro |
