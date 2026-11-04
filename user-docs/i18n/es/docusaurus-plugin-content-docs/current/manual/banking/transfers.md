---
id: transfers
title: "kobana banking transfers"
sidebar_label: "transfers"
description: "Transferencias (unificado)"
---

# kobana banking transfers

Transferencias (unificado)

Comandos de `kobana banking transfers`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`transfers list`](#transfers-list) | `GET /v2/transfers` | Listar transferencias |
| [`transfers create`](#transfers-create) | `POST /v2/transfers` | Crear uma transferencia |
| [`transfers get`](#transfers-get) | `GET /v2/transfers/{uid}` | Ver uma transferencia |
| [`transfers cancel`](#transfers-cancel) | `PUT /v2/transfers/{uid}/cancel` | Cancelar uma transferencia |

## transfers list

Listar transferencias

```text
kobana banking transfers list [OPCIONES]
```

`GET /v2/transfers`

Lista todas las transferencias de qualquer tipo (PIX, TED, Interna). Soporta filtros y paginación. Este é um endpoint unificado que retorna transfers de todos los tipos através de uma única consulta.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `transfer_kind` | `string` | no | Filtrar por tipo de transferencia Valores: `pix`, `ted`, `internal`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | transferencias encontradas |

## transfers create

Crear uma transferencia

```text
kobana banking transfers create [OPCIONES]
```

`POST /v2/transfers`

Endpoint genérico para criação de transferencias de qualquer tipo. O tipo é especificado pelo campo obrigatório 'kind' en lo nível raiz del payload:

**Requisitos previos**

Para emitir uma transferencia é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de transferencia que registra a operação é definida em `transfer_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Tipos Suportados**

**PIX por Chave**
Especificar `kind: "pix"` y fornecer los dados PIX dentro de `pix.data` (padrão Payment):
```json
{
  "financial_account_uid": "...",
  "amount": 100.0,
  "kind": "pix",
  "pix": {
    "kind": "pix",
    "data": {
      "type": "key",
      "key_type": "email",
      "key": "destinatario@example.com"
    }
  },
  "beneficiary": {
    "name": "João Silva",
    "document": "123.456.789-10"
  }
}
```

**PIX por cuenta Bancária**
Especificar `kind: "pix"` y fornecer los dados PIX dentro de `pix.data` con `type: "bank_account"`:
```json
{
  "financial_account_uid": "...",
  "amount": 200.0,
  "kind": "pix",
  "pix": {
    "kind": "pix",
    "data": {
      "type": "bank_account",
      "bank_account": {
        "bank_code": "237",
        "agency_number": "1234",
        "account_number": "56789-0"
      }
    }
  },
  "beneficiary": {
    "name": "Maria Santos",
    "document": "987.654.321-00"
  }
}
```

**TED**
Especificar `kind: "ted"` y fornecer los dados bancários dentro de `ted.data`:
```json
{
  "financial_account_uid": "...",
  "amount": 300.0,
  "kind": "ted",
  "ted": {
    "kind": "ted",
    "data": {
      "bank_account": {
        "bank_code": "341",
        "agency_number": "4321",
        "agency_digit": "5",
        "account_number": "98765-4",
        "account_type": "checking"
      }
    }
  },
  "beneficiary": {
    "name": "Pedro Oliveira",
    "document": "11.222.333/0001-44"
  }
}
```

**transferencia Interna**
Especificar `kind: "internal"` y fornecer los dados de la cuenta destino dentro de `internal.data`:
```json
{
  "financial_account_uid": "...",
  "amount": 400.0,
  "kind": "internal",
  "internal": {
    "kind": "internal",
    "data": {
      "agency_number": "0001",
      "account_number": "12345-6"
    }
  },
  "beneficiary": {
    "name": "Ana Costa",
    "document": "555.666.777-88"
  }
}
```

**Eventos de Webhook**

O evento `transfer.db.created` é disparado nesta operação.

> **Endpoint Unificado:** Este endpoint unifica a criação de todos los tipos de transferencia. Para Crear transferencias de um tipo específico, você também pode usar los endpoints dedicados em `/api/v2/transfer/{type}`.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.transfers.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | transferencia Interna creada |
| `422` | Tipo de transferencia en lo identificado |

## transfers get

Ver uma transferencia

```text
kobana banking transfers get [UID] [OPCIONES]
```

`GET /v2/transfers/{uid}`

Consulta información específicas de uma transferencia de qualquer tipo. O tipo é identificado automaticamente y a resposta é serializada adequadamente.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | transferencia encontrada |
| `404` | transferencia en lo encontrada |

## transfers cancel

Cancelar uma transferencia

```text
kobana banking transfers cancel [UID] [OPCIONES]
```

`PUT /v2/transfers/{uid}/cancel`

Cancela una transferencia que aún no fue confirmada, rechazada o cancelada. Las transferencias ya procesadas no pueden ser canceladas.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | transferencia cancelada con sucesso |
| `404` | transferencia en lo encontrada |
| `422` | La transferencia ya fue confirmada, rechazada o cancelada |
