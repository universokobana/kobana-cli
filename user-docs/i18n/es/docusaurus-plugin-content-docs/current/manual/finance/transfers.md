---
id: transfers
title: "kobana finance transfers"
sidebar_label: "transfers"
---

# kobana finance transfers


Comandos de `kobana finance transfers`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`transfers create`](#transfers-create) | `POST /v1/transfers` | Crear transferencia |
| [`transfers get`](#transfers-get) | `GET /v1/transfers/{id}` | Obtener transferencia por ID |

## transfers create

Crear transferencia

```text
kobana finance transfers create [OPCIONES]
```

`POST /v1/transfers`

Creates an account-to-account transfer as a linked debit/credit pair sharing one `transfer_id`. Requires scope `finance/transactions` with access level `write`. The two accounts must differ, both must belong to the tenant, and both must have a linked company (cross-company is allowed). Pass an `X-Idempotency-Key` header on retries.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `from_account_id` | `string (uuid)` | sí | Source account UUID (debited). Must belong to the tenant. |
| `to_account_id` | `string (uuid)` | sí | Destination account UUID (credited). Must belong to the tenant and differ from `from_account_id`. |
| `amount` | `number` | sí | Positive number in BRL (up to 13 integer digits, 2 decimals). |
| `occurred_at` | `string (date-time)` | sí | When the movement hits both accounts (ISO 8601). |
| `description` | `string` | sí |  |
| `competence_date` | `string (date-time)` | no | Accounting period this transfer belongs to. Defaults to `occurred_at` when omitted. |
| `notes` | `string` | no |  |
| `is_pending` | `boolean` | no | Defaults to `false`. `true` marks a forecast transfer. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre, gravado nas duas pernas. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Otros campos y objetos anidados: `kobana schema finance.transfers.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Transfer created. The `Location` header points at `GET /transfers/{transfer_id}`. |
| `400` | Malformed JSON body. |
| `404` | A source/destination account was not found within the tenant. |
| `409` | `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, same source/destination account, non-positive amount, or an account without a linked company. |

## transfers get

Obtener transferencia por ID

```text
kobana finance transfers get [ID] [OPCIONES]
```

`GET /v1/transfers/{id}`

Returns the linked debit/credit pair for a `transfer_id`. Requires scope `finance/transactions` with access level `read`. 404 when the transfer is not in the tenant.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | The transfer. |
| `404` | Transfer not found within the tenant. |
