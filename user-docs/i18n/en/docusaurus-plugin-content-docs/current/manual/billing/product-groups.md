---
id: product-groups
title: "kobana billing product-groups"
sidebar_label: "product-groups"
---

# kobana billing product-groups


Commands of `kobana billing product-groups`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`product-groups reorder create`](#product-groups-reorder-create) | `POST /v1/product_groups/reorder` | Reorder product groups |
| [`product-groups reorder update`](#product-groups-reorder-update) | `PUT /v1/product_groups/reorder` | Reorder product groups (PUT alias) |
| [`product-groups list`](#product-groups-list) | `GET /v1/product_groups` | List product groups |
| [`product-groups create`](#product-groups-create) | `POST /v1/product_groups` | Create product group |
| [`product-groups update`](#product-groups-update) | `PUT /v1/product_groups` | Upsert product group |
| [`product-groups get`](#product-groups-get) | `GET /v1/product_groups/{id}` | Get product group |
| [`product-groups delete`](#product-groups-delete) | `DELETE /v1/product_groups/{id}` | Remove product group |
| [`product-groups archive`](#product-groups-archive) | `POST /v1/product_groups/{id}/archive` | Archive product group |
| [`product-groups unarchive`](#product-groups-unarchive) | `POST /v1/product_groups/{id}/unarchive` | Unarchive product group |

## product-groups reorder create

Reorder product groups

```text
kobana billing product-groups reorder create [OPTIONS]
```

`POST /v1/product_groups/reorder`

Updates the display order of product groups. The `group_ids` array defines the new order — index 0 becomes the first group.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | yes | List of UUIDs in the new desired order. All IDs must belong to the organization. |

More fields and nested objects: `kobana schema billing.product-groups.reorder.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Order updated successfully. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## product-groups reorder update

Reorder product groups (PUT alias)

```text
kobana billing product-groups reorder update [OPTIONS]
```

`PUT /v1/product_groups/reorder`

Alias `PUT` for `POST /product_groups/reorder`. Same contract.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | yes | List of UUIDs in the new desired order. All IDs must belong to the organization. |

More fields and nested objects: `kobana schema billing.product-groups.reorder.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Order updated successfully. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## product-groups list

List product groups

```text
kobana billing product-groups list [OPTIONS]
```

`GET /v1/product_groups`

Returns a paginated list of the current organization's product groups.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `search` | `string` | no | Free-text search by group name. |
| `company_id` | `string` | no | Filter by associated issuing company. |
| `include_archived` | `boolean,null` | no | When `true`, includes archived groups in the listing. Default: `false`. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `archived_after` | `string` | no | Returns groups archived from this date onwards (ISO 8601). |
| `archived_before` | `string` | no | Returns groups archived up to this date (ISO 8601). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of product groups. |
| `404` | Resource not found in the current organization. |

## product-groups create

Create product group

```text
kobana billing product-groups create [OPTIONS]
```

`POST /v1/product_groups`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Group name. |
| `slug` | `string` | no | Group slug. Generated from the name when omitted. |
| `visibility` | `string` | no | Visibility in the customer portal. Default: `public`. Values: `public`, `private`. |
| `company_id` | `string (uuid)` | no |  |
| `proposal_template_id` | `string (uuid)` | no |  |
| `service_item_id` | `string (uuid)` | no |  |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |

More fields and nested objects: `kobana schema billing.product-groups.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Product group created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## product-groups update

Upsert product group

```text
kobana billing product-groups update [OPTIONS]
```

`PUT /v1/product_groups`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Group name. |
| `slug` | `string` | no | Group slug. Generated from the name when omitted. |
| `visibility` | `string` | no | Visibility in the customer portal. Default: `public`. Values: `public`, `private`. |
| `company_id` | `string (uuid)` | no |  |
| `proposal_template_id` | `string (uuid)` | no |  |
| `service_item_id` | `string (uuid)` | no |  |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no | Arbitrary client-defined metadata (key/value). |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.product-groups.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## product-groups get

Get product group

```text
kobana billing product-groups get [ID] [OPTIONS]
```

`GET /v1/product_groups/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Product group detail. |
| `404` | Resource not found in the current organization. |

## product-groups delete

Remove product group

```text
kobana billing product-groups delete [ID] [OPTIONS]
```

`DELETE /v1/product_groups/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Removal confirmation. |
| `404` | Resource not found in the current organization. |

## product-groups archive

Archive product group

```text
kobana billing product-groups archive [ID] [OPTIONS]
```

`POST /v1/product_groups/{id}/archive`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Product group archived. |
| `404` | Resource not found in the current organization. |

## product-groups unarchive

Unarchive product group

```text
kobana billing product-groups unarchive [ID] [OPTIONS]
```

`POST /v1/product_groups/{id}/unarchive`

Brings an archived product group back into the catalog. Requires the `write` access level. Replaces `DELETE /v1/product_groups/{id}/archive`, which was removed: a `DELETE` is always a deletion.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Product group unarchived. |
| `404` | Resource not found in the current organization. |
