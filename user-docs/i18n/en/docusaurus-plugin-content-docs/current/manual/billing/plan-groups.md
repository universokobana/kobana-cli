---
id: plan-groups
title: "kobana billing plan-groups"
sidebar_label: "plan-groups"
---

# kobana billing plan-groups


Commands of `kobana billing plan-groups`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`plan-groups list`](#plan-groups-list) | `GET /v1/plan_groups` | List plan groups |
| [`plan-groups create`](#plan-groups-create) | `POST /v1/plan_groups` | Create plan group |
| [`plan-groups update`](#plan-groups-update) | `PUT /v1/plan_groups` | Upsert plan group |
| [`plan-groups reorder`](#plan-groups-reorder) | `POST /v1/plan_groups/reorder` | Reorder plan groups |
| [`plan-groups get`](#plan-groups-get) | `GET /v1/plan_groups/{id}` | Get plan group |
| [`plan-groups delete`](#plan-groups-delete) | `DELETE /v1/plan_groups/{id}` | Remove plan group |
| [`plan-groups archive`](#plan-groups-archive) | `POST /v1/plan_groups/{id}/archive` | Archive plan group |

## plan-groups list

List plan groups

```text
kobana billing plan-groups list [OPTIONS]
```

`GET /v1/plan_groups`

Returns all of the current organization's plan groups ordered by `display_order`.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `search` | `string` | no | Free-text search by name or slug. |
| `include_archived` | `string` | no | When `true`, includes archived groups in the list. Values: `true`, `false`. |
| `company_id` | `string` | no | Filter plan groups by the given company (UUID). |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `archived_after` | `string` | no | Return only groups archived on or after this date (YYYY-MM-DD). |
| `archived_before` | `string` | no | Return only groups archived on or before this date (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | List of plan groups. |
| `404` | Resource not found in the current organization. |

## plan-groups create

Create plan group

```text
kobana billing plan-groups create [OPTIONS]
```

`POST /v1/plan_groups`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Plan group name. |
| `slug` | `string` | no | Readable identifier (optional). Auto-generated when omitted. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.plan-groups.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Plan group created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plan-groups update

Upsert plan group

```text
kobana billing plan-groups update [OPTIONS]
```

`PUT /v1/plan_groups`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Plan group name. |
| `slug` | `string` | no | Readable identifier (optional). Auto-generated when omitted. |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.plan-groups.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plan-groups reorder

Reorder plan groups

```text
kobana billing plan-groups reorder [OPTIONS]
```

`POST /v1/plan_groups/reorder`

Updates each group `display_order` according to the order of the `group_ids` array. All groups must belong to the current organization.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | yes | List of group UUIDs in the new display order. All must belong to the current organization. |

More fields and nested objects: `kobana schema billing.plan-groups.reorder`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Order updated. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## plan-groups get

Get plan group

```text
kobana billing plan-groups get [ID] [OPTIONS]
```

`GET /v1/plan_groups/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Plan group detail. |
| `404` | Resource not found in the current organization. |

## plan-groups delete

Remove plan group

```text
kobana billing plan-groups delete [ID] [OPTIONS]
```

`DELETE /v1/plan_groups/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Removal confirmation. |
| `404` | Resource not found in the current organization. |

## plan-groups archive

Archive plan group

```text
kobana billing plan-groups archive [ID] [OPTIONS]
```

`POST /v1/plan_groups/{id}/archive`

Marks the group as archived by setting `archived_at`. Archived groups are hidden from default listings.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Plan group archived. |
| `404` | Resource not found in the current organization. |
