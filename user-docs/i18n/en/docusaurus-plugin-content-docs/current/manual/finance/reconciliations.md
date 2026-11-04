---
id: reconciliations
title: "kobana finance reconciliations"
sidebar_label: "reconciliations"
---

# kobana finance reconciliations


Commands of `kobana finance reconciliations`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`reconciliations create`](#reconciliations-create) | `POST /v1/reconciliations` | Create a reconciliation |
| [`reconciliations get`](#reconciliations-get) | `GET /v1/reconciliations/{id}` | Get a reconciliation by id |
| [`reconciliations delete`](#reconciliations-delete) | `DELETE /v1/reconciliations/{id}` | Delete a reconciliation |

## reconciliations create

Create a reconciliation

```text
kobana finance reconciliations create [OPTIONS]
```

`POST /v1/reconciliations`

Reconciles billings against bank transactions (n:m), settling every billing (paid/received). Requires scope `finance/reconciliations` with access level `write`. All billings must share one polarity and one company; transaction polarity must match (payable↔debit, receivable↔credit). A billing already reconciled/linked, cancelled or draft is rejected (422). Pass an `X-Idempotency-Key` header on retries.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `billings` | `array<object>` | yes | Billings to reconcile. All must share one kind and one company. |
| `transaction_ids` | `array<string (uuid)>` | yes | Bank transactions to reconcile the billings against. |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

More fields and nested objects: `kobana schema finance.reconciliations.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Reconciliation created; the `Location` header points at `GET /reconciliations/{id}`. |
| `400` | Malformed JSON body. |
| `404` | A billing or transaction was not found within the tenant. |
| `409` | `X-Idempotency-Key` reuse with a different body. |
| `422` | Validation failure: mixed polarity/company, wrong transaction polarity, a member already reconciled/linked, or a cancelled/draft billing. |

## reconciliations get

Get a reconciliation by id

```text
kobana finance reconciliations get [ID] [OPTIONS]
```

`GET /v1/reconciliations/{id}`

Returns the reconciliation's light id-list shape (`source`, `kind`, `billing_ids`, `transaction_ids`). Requires scope `finance/reconciliations` with access level `read`. 404 when not in the tenant.

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
| `200` | The reconciliation. |
| `404` | Reconciliation not found within the tenant. |

## reconciliations delete

Delete a reconciliation

```text
kobana finance reconciliations delete [ID] [OPTIONS]
```

`DELETE /v1/reconciliations/{id}`

Dissolves a `manual` reconciliation: reverts every member billing to pending and clears the links. Requires scope `finance/reconciliations` with access level `delete`. Imported (`fintera`) reconciliations are read-only → 422.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Reconciliation dissolved. |
| `404` | Reconciliation not found within the tenant. |
| `422` | The reconciliation is imported (`source: fintera`) and cannot be dissolved via the API. |
