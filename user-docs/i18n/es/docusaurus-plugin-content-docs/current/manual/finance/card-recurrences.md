---
id: card-recurrences
title: "kobana finance card-recurrences"
sidebar_label: "card-recurrences"
---

# kobana finance card-recurrences


Comandos de `kobana finance card-recurrences`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`card-recurrences list`](#card-recurrences-list) | `GET /v1/card-recurrences` | Listar las recurrencias de la tarjeta |
| [`card-recurrences create`](#card-recurrences-create) | `POST /v1/card-recurrences` | Crear una recurrencia en la tarjeta |
| [`card-recurrences get`](#card-recurrences-get) | `GET /v1/card-recurrences/{id}` | Obtener una recurrencia de la tarjeta por id |
| [`card-recurrences update`](#card-recurrences-update) | `PATCH /v1/card-recurrences/{id}` | Actualizar una recurrencia de la tarjeta |
| [`card-recurrences delete`](#card-recurrences-delete) | `DELETE /v1/card-recurrences/{id}` | Eliminar una recurrencia de la tarjeta |

## card-recurrences list

Listar las recurrencias de la tarjeta

```text
kobana finance card-recurrences list [OPCIONES]
```

`GET /v1/card-recurrences`

Returns a cursor-paginated page of the recurring charges of the workspace's cards (subscriptions), each with its next charge, the latest confirmed occurrence, its deviation from the forecast and, after 3 periods in a row more than 5% off, the amount to update it to (`suggested_amount`). Requires scope `finance/credit_cards` with access level `read`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `credit_card_id` | `string` | no | Filter by card. Accepts `[in]`. |
| `active` | `string` | no | Valores: `true`, `false`. |
| `is_international` | `string` | no | Valores: `true`, `false`. |
| `frequency` | `string` | no | Accepts `[in]`. Valores: `monthly`, `quarterly`, `yearly`. |
| `person_id` | `string` | no | Filter by supplier. Accepts `[in]`. |
| `category_id` | `string` | no | Accepts `[in]`. |
| `sort_by` | `string` | no | Default: `description` ascending. Valores: `description`, `charge_day`, `created_at`. |
| `sort_order` | `string` | no | Order direction. Defaults to `asc`. Valores: `asc`, `desc`. |
| `limit` | `integer` | no | Page size. Default 25, max 100. |
| `cursor` | `string` | no | Opaque cursor for the next page. Round-trip `meta.next_cursor`. |
| `fields` | `string` | no | Comma-separated top-level fields to return, e.g. `fields=description,amount,due_date`. Omit for every field. `id` is always included; naming an object field returns it whole. An unknown name is a 400 whose `details` lists `unknown` and `available`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Page of card recurrences. |
| `400` | Malformed query (unknown filter / operator, invalid limit, bad cursor). |

## card-recurrences create

Crear una recurrencia en la tarjeta

```text
kobana finance card-recurrences create [OPCIONES]
```

`POST /v1/card-recurrences`

Creates a recurring charge and its forecasts: one `recurring` launch with status `expected` in each open or future statement inside the validity, dated on the charge day. Forecasts have no mirror in the ledger and are confirmed through `PATCH /card-transactions/{id}` (`status: actual`). An international recurrence is kept in its currency; its forecasts use the estimated rate and the IOF in force. Requires scope `finance/credit_cards` with access level `write`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `credit_card_id` | `string (uuid)` | sí |  |
| `description` | `string` | sí |  |
| `expected_amount` | `number` | sí | In `currency`. |
| `person_id` | `string (uuid)` | no |  |
| `category_id` | `string (uuid)` | no | Omitted on create = the card's default category. |
| `classification_center_id` | `string (uuid)` | no | Omitted on create = the card's default classification center. |
| `currency` | `string` | no | Defaults to BRL. Another currency makes the recurrence international. |
| `is_international` | `boolean` | no | Derived from `currency` when omitted. |
| `charge_day` | `integer` | no | Defaults to the day of `start_date`. |
| `frequency` | `string` | no | Defaults to `monthly`. Valores: `monthly`, `quarterly`, `yearly`. |
| `start_date` | `string` | no | Defaults to today. |
| `end_date` | `string` | no | Ends the recurrence: forecasts after it are cancelled. `null` reopens it. |
| `issuer_match_pattern` | `string` | no |  |
| `active` | `boolean` | no |  |
| `ir_relevant` | `boolean` | no |  |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema finance.card-recurrences.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Recurrence created. `warnings` says when international forecasts wait for a PTAX rate. |
| `400` | Malformed JSON body. |
| `404` | The card or a referenced person, category or classification center is not in the tenant. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an end date before the start date. |

## card-recurrences get

Obtener una recurrencia de la tarjeta por id

```text
kobana finance card-recurrences get [ID] [OPCIONES]
```

`GET /v1/card-recurrences/{id}`

Returns the same shape as the rows from the list endpoint. Requires scope `finance/credit_cards` with access level `read`. 404 covers both "doesn't exist" and "not in the tenant" — the API does not distinguish.

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
| `200` | Get a card recurrence by id |
| `404` | Card recurrence not found within the tenant. |

## card-recurrences update

Actualizar una recurrencia de la tarjeta

```text
kobana finance card-recurrences update [ID] [OPCIONES]
```

`PATCH /v1/card-recurrences/{id}`

Partially updates a recurrence. Its forecasts nobody touched are rebuilt from it; confirmed, skipped and individually edited occurrences stay as they are. An `end_date` ends the recurrence and cancels the forecasts after it; `active: false` cancels the open and future ones. Requires scope `finance/credit_cards` with access level `write`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `description` | `string` | no |  |
| `expected_amount` | `number` | no |  |
| `person_id` | `string (uuid)` | no |  |
| `category_id` | `string (uuid)` | no | Omitted on create = the card's default category. |
| `classification_center_id` | `string (uuid)` | no | Omitted on create = the card's default classification center. |
| `currency` | `string` | no | Defaults to BRL. Another currency makes the recurrence international. |
| `is_international` | `boolean` | no | Derived from `currency` when omitted. |
| `charge_day` | `integer` | no | Defaults to the day of `start_date`. |
| `frequency` | `string` | no | Defaults to `monthly`. Valores: `monthly`, `quarterly`, `yearly`. |
| `start_date` | `string` | no | Defaults to today. |
| `end_date` | `string` | no | Ends the recurrence: forecasts after it are cancelled. `null` reopens it. |
| `issuer_match_pattern` | `string` | no |  |
| `active` | `boolean` | no |  |
| `ir_relevant` | `boolean` | no |  |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |
| `custom_metadata_mode` | `string` | no | Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema finance.card-recurrences.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | The recurrence as updated. |
| `400` | Malformed JSON body. |
| `404` | Card recurrence not found within the tenant, or a referenced id is not in it. |
| `409` | `X-Idempotency-Key` reused with a different body. |
| `422` | Schema validation failure, a full card number in a text field, or an end date before the start date. |

## card-recurrences delete

Eliminar una recurrencia de la tarjeta

```text
kobana finance card-recurrences delete [ID] [OPCIONES]
```

`DELETE /v1/card-recurrences/{id}`

Deletes a recurrence and its forecasts while none was confirmed, skipped or billed in a closed statement; after that it is ended with `end_date` instead (`reason: recurrence_has_history`). Requires scope `finance/credit_cards` with access level `delete`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | sí | Workspace to act on, as a UUID. The token identifies the organization; this header picks which of its workspaces the request reads or writes. It is checked against the token's organization, so a workspace owned by anyone else answers `UNKNOWN_WORKSPACE` — the same code as one that does not exist. Omitting it answers `WORKSPACE_REQUIRED` (400). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Recurrence deleted. |
| `404` | Card recurrence not found within the tenant. |
| `422` | An occurrence was already confirmed, skipped or billed in a closed statement (`reason: recurrence_has_history`). |
