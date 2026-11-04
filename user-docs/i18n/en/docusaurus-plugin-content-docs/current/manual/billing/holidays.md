---
id: holidays
title: "kobana billing holidays"
sidebar_label: "holidays"
---

# kobana billing holidays


Commands of `kobana billing holidays`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`holidays list`](#holidays-list) | `GET /v1/holidays` | List holidays |
| [`holidays create`](#holidays-create) | `POST /v1/holidays` | Create a holiday |
| [`holidays sync`](#holidays-sync) | `POST /v1/holidays/sync` | Import national holidays |
| [`holidays get`](#holidays-get) | `GET /v1/holidays/{id}` | Get a holiday |
| [`holidays update`](#holidays-update) | `PATCH /v1/holidays/{id}` | Update a holiday |
| [`holidays delete`](#holidays-delete) | `DELETE /v1/holidays/{id}` | Delete a holiday |

## holidays list

List holidays

```text
kobana billing holidays list [OPTIONS]
```

`GET /v1/holidays`

Returns the organization holidays — the days business-day counting skips. A late fee policy with `day_count = business` reads exactly this table.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `year` | `integer` | no | Filters by calendar year. |
| `date_from` | `string` | no | Start date, inclusive, as `YYYY-MM-DD`. |
| `date_to` | `string` | no | End date, inclusive, as `YYYY-MM-DD`. |
| `type` | `string` | no | Filters by the holiday scope. Values: `national`, `state`, `municipal`, `optional`, `custom`. |
| `include_deleted` | `string` | no | Includes soft-deleted holidays. Values: `true`, `false`. |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of holidays. |
| `404` | Resource not found in the current organization. |

## holidays create

Create a holiday

```text
kobana billing holidays create [OPTIONS]
```

`POST /v1/holidays`

Creates a holiday. Use it for state and municipal holidays and optional days off — the national ones arrive through the automatic import.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `date` | `string` | yes | Calendar day of the holiday, as `YYYY-MM-DD`. No time of day: a holiday is a date on a wall calendar, and accepting an instant would make the stored day depend on the sender timezone. |
| `name` | `string` | yes | Holiday name. Together with the date it identifies the row — a municipal and a national holiday can fall on the same day. |
| `type` | `string` | no | Scope: national, state, municipal, an optional day off (`optional`), or the organization own (`custom`). Values: `national`, `state`, `municipal`, `optional`, `custom`. |
| `notes` | `string` | no | Free text — the city of a municipal holiday, or any note. |

More fields and nested objects: `kobana schema billing.holidays.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Holiday created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## holidays sync

Import national holidays

```text
kobana billing holidays sync [OPTIONS]
```

`POST /v1/holidays/sync`

Imports the Brazilian national holidays from BrasilAPI for the given years (default: the current one plus the next two). Idempotent: it imports only what is missing and never resurrects a holiday removed on purpose.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `years` | `array<integer>` | no | Years to import. Defaults to the current one plus the next two. |

More fields and nested objects: `kobana schema billing.holidays.sync`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Import result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## holidays get

Get a holiday

```text
kobana billing holidays get [ID] [OPTIONS]
```

`GET /v1/holidays/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Holiday found. |
| `404` | Resource not found in the current organization. |

## holidays update

Update a holiday

```text
kobana billing holidays update [ID] [OPTIONS]
```

`PATCH /v1/holidays/{id}`

**Path arguments:** `id`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `date` | `string` | no | Calendar day of the holiday, as `YYYY-MM-DD`. No time of day: a holiday is a date on a wall calendar, and accepting an instant would make the stored day depend on the sender timezone. |
| `name` | `string` | no | Holiday name. Together with the date it identifies the row — a municipal and a national holiday can fall on the same day. |
| `type` | `string` | no | Scope: national, state, municipal, an optional day off (`optional`), or the organization own (`custom`). Values: `national`, `state`, `municipal`, `optional`, `custom`. |
| `notes` | `string` | no | Free text — the city of a municipal holiday, or any note. |

More fields and nested objects: `kobana schema billing.holidays.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Holiday updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## holidays delete

Delete a holiday

```text
kobana billing holidays delete [ID] [OPTIONS]
```

`DELETE /v1/holidays/{id}`

Soft delete. The row stays, and that is what makes the import respect the removal: a holiday taken out on purpose does not come back on the next sync.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Holiday deleted. |
| `404` | Resource not found in the current organization. |
