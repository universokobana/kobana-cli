---
id: automatic-rules
title: "kobana finance automatic-rules"
sidebar_label: "automatic-rules"
---

# kobana finance automatic-rules


Comandos de `kobana finance automatic-rules`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`automatic-rules list`](#automatic-rules-list) | `GET /v1/automatic-rules` | Listar regras automáticas |
| [`automatic-rules create`](#automatic-rules-create) | `POST /v1/automatic-rules` | Criar regra automática |
| [`automatic-rules reorder`](#automatic-rules-reorder) | `POST /v1/automatic-rules/reorder` | Reordenar regras automáticas |
| [`automatic-rules get`](#automatic-rules-get) | `GET /v1/automatic-rules/{id}` | Obter regra automática por ID |
| [`automatic-rules update`](#automatic-rules-update) | `PATCH /v1/automatic-rules/{id}` | Atualizar regra automática |
| [`automatic-rules delete`](#automatic-rules-delete) | `DELETE /v1/automatic-rules/{id}` | Excluir regra automática |

## automatic-rules list

Listar regras automáticas

```text
kobana finance automatic-rules list [OPÇÕES]
```

`GET /v1/automatic-rules`

Returns a cursor-paginated page of automatic (classification) rules for the workspace tied to the token's `sub` claim. Rules fire on freshly-created transactions/billings, first-match-wins within `(company, scope, position)`. Default order is the evaluation order `(company, scope, position)`. Requires scope `finance/automatic_rules` with access level `read`.

**Parâmetros de consulta**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string` | não | Filter by owning company UUID. Accepts `[in]=uuid1,uuid2` for OR matching. |
| `scope` | `string` | não | Filter by target ledger: `transactions` or `billings`. Valores: `transactions`, `billings`. |
| `active` | `boolean` | não | Filter by active flag. `?active=true` or `?active=false`. |
| `sort_by` | `string` | não | Field to order by. Default: `(company, scope, position)` ascending — the evaluation order. Valores: `created_at`, `position`. |
| `sort_order` | `string` | não | Order direction. Defaults to `asc`; ignored without `sort_by`. Valores: `asc`, `desc`. |
| `limit` | `integer` | não | Page size. Default 25, max 100. |
| `cursor` | `string` | não | Opaque cursor for the next page. Round-trip `meta.next_cursor` from the previous response. |
| `fields` | `string` | não | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Page of rules plus pagination metadata. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## automatic-rules create

Criar regra automática

```text
kobana finance automatic-rules create [OPÇÕES]
```

`POST /v1/automatic-rules`

Creates a rule and appends it at the end of its `(company, scope)` group (`position` = next). Requires scope `finance/automatic_rules` with access level `write`. `workspace_id` comes from the token. A rule needs at least one criterion and at least one action; `account_id` is only valid for `scope=transactions` and `set_expected_account_id` only for `scope=billings`. Pass an `X-Idempotency-Key` header on retries.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Owning company UUID. Must belong to the tenant. |
| `scope` | `string` | sim | Target ledger. Immutable after create. Valores: `transactions`, `billings`. |
| `active` | `boolean` | não | Defaults to `true`. |
| `account_id` | `string (uuid)` | não | Criterion — financial account. Only valid for `scope=transactions` (422 otherwise). |
| `transaction_type` | `string` | não | Valores: `debit`, `credit`, `null`. |
| `match_person_id` | `string (uuid)` | não |  |
| `description_contains` | `string` | não | Case-insensitive substring on description. `""` opts into a wildcard (matches every row). |
| `notes_contains` | `string` | não |  |
| `set_description` | `string` | não |  |
| `set_person_id` | `string (uuid)` | não |  |
| `set_category_id` | `string (uuid)` | não |  |
| `set_classification_center_id` | `string (uuid)` | não | Action — classification center. Maps to the internal cost-center column. |
| `set_expected_account_id` | `string (uuid)` | não | Action — expected settlement account. Only valid for `scope=billings` (422 otherwise). |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |

Outros campos e objetos aninhados: `kobana schema finance.automatic-rules.create`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `201` | Rule created. The `Location` header points at the new row. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; `company_id` or a referenced entity not in the tenant; no criterion / no action; or a scope↔field mismatch. |

## automatic-rules reorder

Reordenar regras automáticas

```text
kobana finance automatic-rules reorder [OPÇÕES]
```

`POST /v1/automatic-rules/reorder`

Rewrites `position` for one `(company_id, scope)` group so it matches `ordered_ids` (first → position 1). Requires scope `finance/automatic_rules` with access level `write`. `ordered_ids` must be the COMPLETE ordered set for that group — a partial or foreign set is rejected (422). Idempotent via `X-Idempotency-Key`. Answers `200 { data: { ordered_ids } }`.

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `company_id` | `string (uuid)` | sim | Company whose rules are being reordered. |
| `scope` | `string` | sim | Ledger whose rules are being reordered. Valores: `transactions`, `billings`. |
| `ordered_ids` | `array<string (uuid)>` | sim | The COMPLETE ordered set of rule ids for `(company_id, scope)`, first to last. Positions are rewritten 1..N to match. A partial or foreign set is rejected (422). |

Outros campos e objetos aninhados: `kobana schema finance.automatic-rules.reorder`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Positions rewritten. Echoes the ids in the order they now hold. |
| `400` | Malformed JSON body. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; `company_id` not in the tenant; or `ordered_ids` is not the exact set for `(company, scope)`. |

## automatic-rules get

Obter regra automática por ID

```text
kobana finance automatic-rules get [ID] [OPÇÕES]
```

`GET /v1/automatic-rules/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/automatic_rules` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get an automatic rule by id |
| `404` | Rule not found within the tenant. |

## automatic-rules update

Atualizar regra automática

```text
kobana finance automatic-rules update [ID] [OPÇÕES]
```

`PATCH /v1/automatic-rules/{id}`

Partial update of criteria/actions and `active`. Requires scope `finance/automatic_rules` with access level `write`. `scope` and `company_id` are immutable (delete + recreate to move a rule); `position` is changed via `POST /automatic-rules/reorder`. The merged rule must still have at least one criterion and one action.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Corpo da requisição**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `active` | `boolean` | não |  |
| `account_id` | `string (uuid)` | não | Criterion — financial account. Only valid for `scope=transactions` (422 otherwise). |
| `transaction_type` | `string` | não | Valores: `debit`, `credit`, `null`. |
| `match_person_id` | `string (uuid)` | não |  |
| `description_contains` | `string` | não | Case-insensitive substring on description. `""` opts into a wildcard (matches every row). |
| `notes_contains` | `string` | não |  |
| `set_description` | `string` | não |  |
| `set_person_id` | `string (uuid)` | não |  |
| `set_category_id` | `string (uuid)` | não |  |
| `set_classification_center_id` | `string (uuid)` | não | Action — classification center. Maps to the internal cost-center column. |
| `set_expected_account_id` | `string (uuid)` | não | Action — expected settlement account. Only valid for `scope=billings` (422 otherwise). |
| `custom_metadata` | `object` | não | Metadados do usuário: objeto JSON livre. Envie `null` para limpar. Máx. 64 KB serializado, 16 níveis. |
| `custom_metadata_mode` | `string` | não | Como o `custom_metadata` enviado é reconciliado com o valor armazenado: `replace` (padrão) substitui o objeto inteiro, `merge` faz deep merge sobre o valor atual. Aceito apenas na atualização. Valores: `replace`, `merge`. |

Outros campos e objetos aninhados: `kobana schema finance.automatic-rules.update`.

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Updated rule. |
| `400` | Malformed JSON body. |
| `404` | Rule not found within the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure; a referenced entity not in the tenant; or the merged rule loses its last criterion/action. |

## automatic-rules delete

Excluir regra automática

```text
kobana finance automatic-rules delete [ID] [OPÇÕES]
```

`DELETE /v1/automatic-rules/{id}`

Hard-deletes a rule. Requires scope `finance/automatic_rules` with access level `delete`. Surviving rules keep their positions (a gap is left — reorder to compact). Calling DELETE twice returns 200 then 404.

**Argumentos do caminho:** `id`

**Cabeçalhos**

| Nome | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sim | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respostas**

| HTTP | Descrição |
|---|---|
| `200` | Rule deleted. |
| `404` | Rule not found within the tenant. |
