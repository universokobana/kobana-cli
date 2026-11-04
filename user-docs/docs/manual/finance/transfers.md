---
id: transfers
title: "kobana finance transfers"
sidebar_label: "transfers"
---

# kobana finance transfers


Comandos de `kobana finance transfers`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`transfers create`](#transfers-create) | `POST /v1/transfers` | Criar transferência |
| [`transfers get`](#transfers-get) | `GET /v1/transfers/{id}` | Obter transferência por ID |

## transfers create

Criar transferência

```text
kobana finance transfers create [OPÇÕES]
```

`POST /v1/transfers`

Creates an account-to-account transfer as a linked debit/credit pair sharing one `transfer_id`. Requires scope `finance/transactions` with access level `write`. The two accounts must differ, both must belong to the tenant, and both must have a linked company (cross-company is allowed). Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `from_account_id` | `string (uuid)` | sim | Source account UUID (debited). Must belong to the tenant. |
| `to_account_id` | `string (uuid)` | sim | Destination account UUID (credited). Must belong to the tenant and differ from `from_account_id`. |
| `amount` | `number` | sim | Positive number in BRL (up to 13 integer digits, 2 decimals). |
| `occurred_at` | `string (date-time)` | sim | When the movement hits both accounts (ISO 8601). |
| `description` | `string` | sim |  |
| `competence_date` | `string (date-time)` | não | Accounting period this transfer belongs to. Defaults to `occurred_at` when omitted. |
| `notes` | `string` | não |  |
| `is_pending` | `boolean` | não | Defaults to `false`. `true` marks a forecast transfer. |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre, gravado nas duas pernas. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.transfers.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Transfer created. The `Location` header points at `GET /transfers/{transfer_id}`. |
| `400` | Malformed JSON body. |
| `404` | A source/destination account was not found within the tenant. |
| `409` | `X-Idempotency-Key` reuse with a different body. |
| `422` | Schema validation failure, same source/destination account, non-positive amount, or an account without a linked company. |

## transfers get

Obter transferência por ID

```text
kobana finance transfers get [ID] [OPÇÕES]
```

`GET /v1/transfers/{id}`

Returns the linked debit/credit pair for a `transfer_id`. Requires scope `finance/transactions` with access level `read`. 404 when the transfer is not in the tenant.

**Argumentos do caminho:** `id`

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | The transfer. |
| `404` | Transfer not found within the tenant. |
