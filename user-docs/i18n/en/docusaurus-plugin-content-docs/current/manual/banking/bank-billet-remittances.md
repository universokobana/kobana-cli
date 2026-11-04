---
id: bank-billet-remittances
title: "kobana banking bank-billet-remittances"
sidebar_label: "bank-billet-remittances"
---

# kobana banking bank-billet-remittances


Commands of `kobana banking bank-billet-remittances`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`bank-billet-remittances occurrence create`](#bank-billet-remittances-occurrence-create) | `POST /v1/bank_billet_remittances/{id}/occurrence/{our_code}` | Create Pending Items |
| [`bank-billet-remittances pending`](#bank-billet-remittances-pending) | `GET /v1/bank_billet_remittances/pending` | List Pending Remittance Records |
| [`bank-billet-remittances list`](#bank-billet-remittances-list) | `GET /v1/bank_billet_remittances` | List Remittance Records |
| [`bank-billet-remittances get`](#bank-billet-remittances-get) | `GET /v1/bank_billet_remittances/{id}` | Remittance Record Information |
| [`bank-billet-remittances delete`](#bank-billet-remittances-delete) | `DELETE /v1/bank_billet_remittances/{id}` | Delete Pending Items |

## bank-billet-remittances occurrence create

Create Pending Items

```text
kobana banking bank-billet-remittances occurrence create [ID] [OUR_CODE] [OPTIONS]
```

`POST /v1/bank_billet_remittances/{id}/occurrence/{our_code}`

Create pending items.

**Path arguments:** `id`, `our_code`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Pending items created |
| `422` | Could not process |

## bank-billet-remittances pending

List Pending Remittance Records

```text
kobana banking bank-billet-remittances pending [OPTIONS]
```

`GET /v1/bank_billet_remittances/pending`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `our_code` | `string` | no | Operation code for record. See the possible values for each bank. |
| `expire_from` | `string` | no | From Due Date. |
| `expire_to` | `string` | no | To Due Date. |
| `bank_billet_id` | `string` | no | Billet ID. |
| `bank_billet_account_id` | `string` | no | Wallet ID. |
| `remittance_id` | `string` | no | Remittance ID. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing remittance records |

## bank-billet-remittances list

List Remittance Records

```text
kobana banking bank-billet-remittances list [OPTIONS]
```

`GET /v1/bank_billet_remittances`

List Remittance Records.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |
| `our_code` | `string` | no | Operation code for record. See the possible values for each bank. |
| `expire_from` | `string` | no | From Due Date. |
| `expire_to` | `string` | no | To Due Date. |
| `bank_billet_id` | `string` | no | Billet ID. |
| `bank_billet_account_id` | `string` | no | Wallet ID. |
| `remittance_id` | `string` | no | Remittance ID. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing remittance records |

## bank-billet-remittances get

Remittance Record Information

```text
kobana banking bank-billet-remittances get [ID] [OPTIONS]
```

`GET /v1/bank_billet_remittances/{id}`

Query specific information from a Remittance Record.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Remittance Record found |
| `404` | Remittance Record not found |

## bank-billet-remittances delete

Delete Pending Items

```text
kobana banking bank-billet-remittances delete [ID] [OPTIONS]
```

`DELETE /v1/bank_billet_remittances/{id}`

Delete pending items.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Pending items deleted |
| `404` | Remittance return not found |
