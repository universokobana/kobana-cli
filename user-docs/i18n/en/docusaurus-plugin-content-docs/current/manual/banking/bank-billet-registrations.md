---
id: bank-billet-registrations
title: "kobana banking bank-billet-registrations"
sidebar_label: "bank-billet-registrations"
---

# kobana banking bank-billet-registrations


Commands of `kobana banking bank-billet-registrations`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`bank-billet-registrations list`](#bank-billet-registrations-list) | `GET /v1/bank_billet_registrations` | List Billet Records. |
| [`bank-billet-registrations get`](#bank-billet-registrations-get) | `GET /v1/bank_billet_registrations/{id}` | Billet Record Information |

## bank-billet-registrations list

List Billet Records.

```text
kobana banking bank-billet-registrations list [OPTIONS]
```

`GET /v1/bank_billet_registrations`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `registered_from` | `string` | no | From Registration Date. |
| `registered_to` | `string` | no | To Registration Date. |
| `bank_billet_id` | `string` | no | Billet ID. |
| `bank_billet_account_id` | `string` | no | Wallet ID. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing billet records |

## bank-billet-registrations get

Billet Record Information

```text
kobana banking bank-billet-registrations get [ID] [OPTIONS]
```

`GET /v1/bank_billet_registrations/{id}`

Query specific information from a Billet Record.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Billet Record found |
| `404` | Return record not found |
