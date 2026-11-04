---
id: discharges
title: "kobana banking discharges"
sidebar_label: "discharges"
---

# kobana banking discharges


Commands of `kobana banking discharges`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`discharges list`](#discharges-list) | `GET /v1/discharges` | List CNABs |
| [`discharges create`](#discharges-create) | `POST /v1/discharges` | Send CNAB |
| [`discharges get`](#discharges-get) | `GET /v1/discharges/{id}` | CNAB Information |
| [`discharges download`](#discharges-download) | `GET /v1/discharges/{id}/download` | CNAB Download |
| [`discharges pay-off`](#discharges-pay-off) | `PUT /v1/discharges/{id}/pay_off` | Settle Billets |
| [`discharges reprocess`](#discharges-reprocess) | `PUT /v1/discharges/{id}/reprocess` | Reprocess CNAB |

## discharges list

List CNABs

```text
kobana banking discharges list [OPTIONS]
```

`GET /v1/discharges`

List CNABs.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `created_from` | `string` | no | From Return Creation Date. |
| `created_to` | `string` | no | To Return Creation Date. |
| `bank_billet_account_id` | `string` | no | Filter by Wallet ID. |
| `tags` | `string` | no | Filter by tags (separated by comma). |
| `page` | `integer` | no | Page Number |
| `per_page` | `integer` | no | Number of records per page |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Success in listing CNABs |

## discharges create

Send CNAB

```text
kobana banking discharges create [OPTIONS]
```

`POST /v1/discharges`

Sends a CNAB file of bank return for processing payments and billing debits.

**Request body**

Sent as a `multipart/form-data` form: files go in `--file FIELD=PATH`.

| Name | Type | Required | Description |
|---|---|---|---|
| `file` | `string (binary)` | no | File |
| `filename` | `string` | no | File name |
| `bank_billet_account_id` | `integer` | no | ID of the Billing Account. |
| `external_id` | `string` | no | External ID. |
| `custom_data` | `object` | no | Custom data. |
| `tags` | `array<string>` | no | Tags. |

More fields and nested objects: `kobana schema banking.discharges.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | CNAB sent with custom_data and tags |
| `422` | Invalid data |

## discharges get

CNAB Information

```text
kobana banking discharges get [ID] [OPTIONS]
```

`GET /v1/discharges/{id}`

Query specific information from a CNAB.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | CNAB found |
| `404` | CNAB not found |

## discharges download

CNAB Download

```text
kobana banking discharges download [ID] [OPTIONS]
```

`GET /v1/discharges/{id}/download`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Download successful |
| `404` | CNAB not found |

## discharges pay-off

Settle Billets

```text
kobana banking discharges pay-off [ID] [OPTIONS]
```

`PUT /v1/discharges/{id}/pay_off`

Settle billets.

You must have already sent the CNAB. All billets identified within the CNAB will be marked as PAID and Webhooks will be triggered.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | Billets settled |
| `404` | CNAB not found |

## discharges reprocess

Reprocess CNAB

```text
kobana banking discharges reprocess [ID] [OPTIONS]
```

`PUT /v1/discharges/{id}/reprocess`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `204` | CNAB reprocessed |
| `404` | CNAB not found |
