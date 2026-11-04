---
id: transfers
title: "kobana finance transfers"
sidebar_label: "transfers"
---

# kobana finance transfers


Commands of `kobana finance transfers`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`transfers create`](#transfers-create) | `POST /v1/transfers` | Create a transfer |
| [`transfers get`](#transfers-get) | `GET /v1/transfers/{id}` | Get a transfer by id |

## transfers create

Create a transfer

```text
kobana finance transfers create [OPTIONS]
```

`POST /v1/transfers`

Creates an account-to-account transfer as a linked debit/credit pair sharing one `transfer_id`. Requires scope `finance/transactions` with access level `write`. The two accounts must differ, both must belong to the tenant, and both must have a linked company (cross-company is allowed). Pass an `X-Idempotency-Key` header on retries.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `from_account_id` | `string (uuid)` | yes | Source account UUID (debited). Must belong to the tenant. |
| `to_account_id` | `string (uuid)` | yes | Destination account UUID (credited). Must belong to the tenant and differ from `from_account_id`. |
| `amount` | `number` | yes | Positive number in BRL (up to 13 integer digits, 2 decimals). |
| `occurred_at` | `string (date-time)` | yes | When the movement hits both accounts (ISO 8601). |
| `description` | `string` | yes |  |
| `competence_date` | `string (date-time)` | no | Accounting period this transfer belongs to. Defaults to `occurred_at` when omitted. |
| `notes` | `string` | no |  |
| `is_pending` | `boolean` | no | Defaults to `false`. `true` marks a forecast transfer. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre, gravado nas duas pernas. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

More fields and nested objects: `kobana schema finance.transfers.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Transfer created. The `Location` header points at `GET /transfers/{transfer_id}`. |
| `400` | Malformed JSON body. |
| `404` | A source/destination account was not found within the tenant. |
| `409` | `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, same source/destination account, non-positive amount, or an account without a linked company. |

## transfers get

Get a transfer by id

```text
kobana finance transfers get [ID] [OPTIONS]
```

`GET /v1/transfers/{id}`

Returns the linked debit/credit pair for a `transfer_id`. Requires scope `finance/transactions` with access level `read`. 404 when the transfer is not in the tenant.

**Path arguments:** `id`

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | The transfer. |
| `404` | Transfer not found within the tenant. |
