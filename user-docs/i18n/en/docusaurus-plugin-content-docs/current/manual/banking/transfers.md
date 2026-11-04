---
id: transfers
title: "kobana banking transfers"
sidebar_label: "transfers"
description: "Transfers (unified)"
---

# kobana banking transfers

Transfers (unified)

Commands of `kobana banking transfers`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`transfers list`](#transfers-list) | `GET /v2/transfers` | List Transfers |
| [`transfers create`](#transfers-create) | `POST /v2/transfers` | Create a Transfer |
| [`transfers get`](#transfers-get) | `GET /v2/transfers/{uid}` | View a Transfer |
| [`transfers cancel`](#transfers-cancel) | `PUT /v2/transfers/{uid}/cancel` | Cancel a Transfer |

## transfers list

List Transfers

```text
kobana banking transfers list [OPTIONS]
```

`GET /v2/transfers`

Lists all transfers of any type (PIX, TED, Internal). Supports filters and pagination. This is a unified endpoint that returns transfers of all types through a single query.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `transfer_kind` | `string` | no | Filter by transfer type Values: `pix`, `ted`, `internal`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Transfers Found |

## transfers create

Create a Transfer

```text
kobana banking transfers create [OPTIONS]
```

`POST /v2/transfers`

Generic endpoint for creating transfers of any type. The type is specified by the mandatory 'kind' field at the root level of the payload:

**Prerequisites**

To issue a transfer, you must have a Financial Account, specified in `financial_account_uid`. The transfer account that records the operation is defined in `transfer_account_uid`; if omitted, the oldest linked to the financial account is used; if specified, it must belong to `financial_account_uid`.

**Supported Types**

**PIX by Key**
Specify `kind: "pix"` and provide the PIX data within `pix.data` (Payment standard):
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
      "key": "recipient@example.com"
    }
  },
  "beneficiary": {
    "name": "João Silva",
    "document": "123.456.789-10"
  }
}
```

**PIX by Bank Account**
Specify `kind: "pix"` and provide the PIX data within `pix.data` with `type: "bank_account"`:
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
Specify `kind: "ted"` and provide the bank data within `ted.data`:
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

**Internal Transfer**
Specify `kind: "internal"` and provide the destination account data within `internal.data`:
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

**Webhook Events**

The `transfer.db.created` event is triggered in this operation.

> **Unified Endpoint:** This endpoint unifies the creation of all transfer types. To create transfers of a specific type, you can also use the dedicated endpoints in `/api/v2/transfer/{type}`.

**Request body**

The body is not a plain object (or has variants). See `kobana schema banking.transfers.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Internal Transfer Created |
| `422` | Transfer type not identified |

## transfers get

View a Transfer

```text
kobana banking transfers get [UID] [OPTIONS]
```

`GET /v2/transfers/{uid}`

Queries specific information about a Transfer of any type. The type is identified automatically and the response is serialized appropriately.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Transfer Found |
| `404` | Transfer Not Found |

## transfers cancel

Cancel a Transfer

```text
kobana banking transfers cancel [UID] [OPTIONS]
```

`PUT /v2/transfers/{uid}/cancel`

Cancels a transfer that has not yet been confirmed, rejected, or canceled. Transfers that have already been processed cannot be canceled.

**Path arguments:** `uid`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Transfer successfully cancelled |
| `404` | Transfer Not Found |
| `422` | Transfer has already been confirmed, rejected, or canceled |
