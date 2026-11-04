---
id: transfers
title: "kobana banking transfers"
sidebar_label: "transfers"
description: "Transferências (unificado)"
---

# kobana banking transfers

Transferências (unificado)

Comandos de `kobana banking transfers`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`transfers list`](#transfers-list) | `GET /v2/transfers` | Listar Transferências |
| [`transfers create`](#transfers-create) | `POST /v2/transfers` | Criar uma Transferência |
| [`transfers get`](#transfers-get) | `GET /v2/transfers/{uid}` | Visualizar uma Transferência |
| [`transfers cancel`](#transfers-cancel) | `PUT /v2/transfers/{uid}/cancel` | Cancelar uma Transferência |

## transfers list

Listar Transferências

```text
kobana banking transfers list [OPÇÕES]
```

`GET /v2/transfers`

Lista todas as transferências de qualquer tipo (PIX, TED, Interna). Suporta filtros e paginação. Este é um endpoint unificado que retorna transfers de todos os tipos através de uma única consulta.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `page` | `integer` | não | Número da Página |
| `per_page` | `integer` | não | Quantidade de registros por página |
| `transfer_kind` | `string` | não | Filtrar por tipo de transferência Valores: `pix`, `ted`, `internal`. |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Transferências encontradas |

## transfers create

Criar uma Transferência

```text
kobana banking transfers create [OPÇÕES]
```

`POST /v2/transfers`

Endpoint genérico para criação de transferências de qualquer tipo. O tipo é especificado pelo campo obrigatório 'kind' no nível raiz do payload:

**Pré-requisitos**

Para emitir uma transferência é necessário ter uma Conta Financeira, informada em `financial_account_uid`. A conta de transferência que registra a operação é definida em `transfer_account_uid`; se omitida, usa-se a mais antiga vinculada à conta financeira; se informada, deve pertencer à `financial_account_uid`.

**Tipos Suportados**

**PIX por Chave**
Especificar `kind: "pix"` e fornecer os dados PIX dentro de `pix.data` (padrão Payment):
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

**PIX por Conta Bancária**
Especificar `kind: "pix"` e fornecer os dados PIX dentro de `pix.data` com `type: "bank_account"`:
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
Especificar `kind: "ted"` e fornecer os dados bancários dentro de `ted.data`:
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

**Transferência Interna**
Especificar `kind: "internal"` e fornecer os dados da conta destino dentro de `internal.data`:
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

> **Endpoint Unificado:** Este endpoint unifica a criação de todos os tipos de transferência. Para criar transferências de um tipo específico, você também pode usar os endpoints dedicados em `/api/v2/transfer/{type}`.

**Corpo da requisição**

O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema banking.transfers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Transferência Interna criada |
| `422` | Tipo de transferência não identificado |

## transfers get

Visualizar uma Transferência

```text
kobana banking transfers get [UID] [OPÇÕES]
```

`GET /v2/transfers/{uid}`

Consulta informações específicas de uma Transferência de qualquer tipo. O tipo é identificado automaticamente e a resposta é serializada adequadamente.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Transferência encontrada |
| `404` | Transferência não encontrada |

## transfers cancel

Cancelar uma Transferência

```text
kobana banking transfers cancel [UID] [OPÇÕES]
```

`PUT /v2/transfers/{uid}/cancel`

Cancela uma transferência que ainda não foi confirmada, rejeitada ou cancelada. Transferências já processadas não podem ser canceladas.

**Argumentos do caminho:** `uid`

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Transferência cancelada com sucesso |
| `404` | Transferência não encontrada |
| `422` | Transferência já foi confirmada, rejeitada ou cancelada |
