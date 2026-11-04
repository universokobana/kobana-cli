---
id: automatic-rules
title: "kobana finance automatic-rules"
sidebar_label: "automatic-rules"
---

# kobana finance automatic-rules


Commands of `kobana finance automatic-rules`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`automatic-rules list`](#automatic-rules-list) | `GET /v1/automatic-rules` | List automatic rules |
| [`automatic-rules create`](#automatic-rules-create) | `POST /v1/automatic-rules` | Create an automatic rule |
| [`automatic-rules reorder`](#automatic-rules-reorder) | `POST /v1/automatic-rules/reorder` | Reorder automatic rules |
| [`automatic-rules get`](#automatic-rules-get) | `GET /v1/automatic-rules/{id}` | Get an automatic rule by id |
| [`automatic-rules update`](#automatic-rules-update) | `PATCH /v1/automatic-rules/{id}` | Update an automatic rule |
| [`automatic-rules delete`](#automatic-rules-delete) | `DELETE /v1/automatic-rules/{id}` | Delete an automatic rule |

## automatic-rules list

List automatic rules

```text
kobana finance automatic-rules list [OPTIONS]
```

`GET /v1/automatic-rules`

Returns a cursor-paginated page of automatic (classification) rules for the workspace tied to the token's `sub` claim. Rules fire on freshly-created transactions/billings, first-match-wins within `(company, scope, position)`. Default order is the evaluation order `(company, scope, position)`. Requires scope `finance/automatic_rules` with access level `read`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string` | no | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `scope` | `string` | no | Filter by target ledger: `transactions` or `billings`. Values: `transactions`, `billings`. |
| `active` | `boolean` | no | Filter by active flag. `?active=true` or `?active=false`. |
| `sort_by` | `string` | no | Field to order by. Default: `(company, scope, position)` ascending — the evaluation order. Values: `created_at`, `position`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`; ignored without `sort_by`. Values: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Page of rules plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## automatic-rules create

Create an automatic rule

```text
kobana finance automatic-rules create [OPTIONS]
```

`POST /v1/automatic-rules`

Creates a rule and appends it at the end of its `(company, scope)` group (`position` = next). Requires scope `finance/automatic_rules` with access level `write`. `workspace_id` comes from the token. A rule needs at least one criterion and at least one action; `account_id` is only valid for `scope=transactions` and `set_expected_account_id` only for `scope=billings`. Pass an `X-Idempotency-Key` header on retries.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string (uuid)` | yes | Owning company UUID. Must belong to the tenant. |
| `scope` | `string` | yes | Target ledger. Immutable after create. Values: `transactions`, `billings`. |
| `active` | `boolean` | no | Defaults to `true`. |
| `account_id` | `string (uuid)` | no | Criterion — financial account. Only valid for `scope=transactions` (422 otherwise). |
| `transaction_type` | `string` | no | Values: `debit`, `credit`, `null`. |
| `match_person_id` | `string (uuid)` | no |  |
| `description_contains` | `string` | no | Case-insensitive substring on description. `""` opts into a wildcard (matches every row). |
| `notes_contains` | `string` | no |  |
| `set_description` | `string` | no |  |
| `set_person_id` | `string (uuid)` | no |  |
| `set_category_id` | `string (uuid)` | no |  |
| `set_classification_center_id` | `string (uuid)` | no | Action — classification center. Maps to the internal cost-center column. |
| `set_expected_account_id` | `string (uuid)` | no | Action — expected settlement account. Only valid for `scope=billings` (422 otherwise). |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

More fields and nested objects: `kobana schema finance.automatic-rules.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Rule created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; `company_id` or a referenced entity not in the tenant; no criterion / no action; or a scope↔field mismatch. |

## automatic-rules reorder

Reorder automatic rules

```text
kobana finance automatic-rules reorder [OPTIONS]
```

`POST /v1/automatic-rules/reorder`

Rewrites `position` for one `(company_id, scope)` group so it matches `ordered_ids` (first → position 1). Requires scope `finance/automatic_rules` with access level `write`. `ordered_ids` must be the COMPLETE ordered set for that group — a partial or foreign set is rejected (422). Idempotent via `X-Idempotency-Key`. Answers `200 { data: { ordered_ids } }`.

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `company_id` | `string (uuid)` | yes | Company whose rules are being reordered. |
| `scope` | `string` | yes | Ledger whose rules are being reordered. Values: `transactions`, `billings`. |
| `ordered_ids` | `array<string (uuid)>` | yes | The COMPLETE ordered set of rule ids for `(company_id, scope)`, first to last. Positions are rewritten 1..N to match. A partial or foreign set is rejected (422). |

More fields and nested objects: `kobana schema finance.automatic-rules.reorder`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Positions rewritten. Echoes the ids in the order they now hold. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; `company_id` not in the tenant; or `ordered_ids` is not the exact set for `(company, scope)`. |

## automatic-rules get

Get an automatic rule by id

```text
kobana finance automatic-rules get [ID] [OPTIONS]
```

`GET /v1/automatic-rules/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/automatic_rules` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get an automatic rule by id |
| `404` | Rule not found within the tenant. |

## automatic-rules update

Update an automatic rule

```text
kobana finance automatic-rules update [ID] [OPTIONS]
```

`PATCH /v1/automatic-rules/{id}`

Partial update of criteria/actions and `active`. Requires scope `finance/automatic_rules` with access level `write`. `scope` and `company_id` are immutable (delete + recreate to move a rule); `position` is changed via `POST /automatic-rules/reorder`. The merged rule must still have at least one criterion and one action.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `active` | `boolean` | no |  |
| `account_id` | `string (uuid)` | no | Criterion — financial account. Only valid for `scope=transactions` (422 otherwise). |
| `transaction_type` | `string` | no | Values: `debit`, `credit`, `null`. |
| `match_person_id` | `string (uuid)` | no |  |
| `description_contains` | `string` | no | Case-insensitive substring on description. `""` opts into a wildcard (matches every row). |
| `notes_contains` | `string` | no |  |
| `set_description` | `string` | no |  |
| `set_person_id` | `string (uuid)` | no |  |
| `set_category_id` | `string (uuid)` | no |  |
| `set_classification_center_id` | `string (uuid)` | no | Action — classification center. Maps to the internal cost-center column. |
| `set_expected_account_id` | `string (uuid)` | no | Action — expected settlement account. Only valid for `scope=billings` (422 otherwise). |
| `custom_metadata` | `object` | no | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | no | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Values: `replace`, `merge`. |

More fields and nested objects: `kobana schema finance.automatic-rules.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Updated rule. |
| `400` | Malformed JSON body. |
| `404` | Rule not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; a referenced entity not in the tenant; or the merged rule loses its last criterion/action. |

## automatic-rules delete

Delete an automatic rule

```text
kobana finance automatic-rules delete [ID] [OPTIONS]
```

`DELETE /v1/automatic-rules/{id}`

Hard-deletes a rule. Requires scope `finance/automatic_rules` with access level `delete`. Surviving rules keep their positions (a gap is left — reorder to compact). Calling DELETE twice returns 200 then 404.

**Path arguments:** `id`

**Headers**

| Name | Type | Required | Description |
|---|---|---|---|
| `X-Workspace-Id` | `string` | yes | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Rule deleted. |
| `404` | Rule not found within the tenant. |
