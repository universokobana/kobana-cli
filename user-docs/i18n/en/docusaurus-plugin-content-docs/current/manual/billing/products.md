---
id: products
title: "kobana billing products"
sidebar_label: "products"
---

# kobana billing products


Commands of `kobana billing products`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`products bulk delete`](#products-bulk-delete) | `DELETE /v1/products/bulk` | Bulk delete products |
| [`products reorder create`](#products-reorder-create) | `POST /v1/products/reorder` | Reorder products |
| [`products reorder update`](#products-reorder-update) | `PUT /v1/products/reorder` | Reorder products (PUT /reorder) |
| [`products list`](#products-list) | `GET /v1/products` | List products |
| [`products create`](#products-create) | `POST /v1/products` | Create product |
| [`products update`](#products-update) | `PUT /v1/products` | Upsert product |
| [`products bulk-archive`](#products-bulk-archive) | `POST /v1/products/bulk-archive` | Bulk archive products |
| [`products bulk-change-company`](#products-bulk-change-company) | `POST /v1/products/bulk-change-company` | Bulk change product company |
| [`products bulk-move-group`](#products-bulk-move-group) | `POST /v1/products/bulk-move-group` | Move products to another group |
| [`products bulk-visibility`](#products-bulk-visibility) | `POST /v1/products/bulk-visibility` | Bulk change visibility |
| [`products get`](#products-get) | `GET /v1/products/{id}` | Get product |
| [`products delete`](#products-delete) | `DELETE /v1/products/{id}` | Delete product |
| [`products archive`](#products-archive) | `POST /v1/products/{id}/archive` | Archive product |
| [`products publish`](#products-publish) | `POST /v1/products/{id}/publish` | Publish product |
| [`products restore`](#products-restore) | `POST /v1/products/{id}/restore` | Restore archived product |

## products bulk delete

Bulk delete products

```text
kobana billing products bulk delete [OPTIONS]
```

`DELETE /v1/products/bulk`

Deletes many products (soft delete: `deleted_at`, the same effect as `DELETE /v1/products/{id}` on each). IDs in use by active subscriptions are skipped and reported in the response. Requires the `delete` access level. Replaces `POST /v1/products/bulk-delete`, which was removed.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | IDs of the products to delete. |

More fields and nested objects: `kobana schema billing.products.bulk.delete`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk operation result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products reorder create

Reorder products

```text
kobana billing products reorder create [OPTIONS]
```

`POST /v1/products/reorder`

Updates product display order. Array index 0 becomes the first item in the list.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `product_ids` | `array<string (uuid)>` | yes | Product IDs in the new desired order. Index 0 becomes the first in the list. |

More fields and nested objects: `kobana schema billing.products.reorder.create`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Order updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products reorder update

Reorder products (PUT /reorder)

```text
kobana billing products reorder update [OPTIONS]
```

`PUT /v1/products/reorder`

Alias `PUT` for `/products/reorder`. Same behavior as the `POST` version.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `product_ids` | `array<string (uuid)>` | yes | Product IDs in the new desired order. Index 0 becomes the first in the list. |

More fields and nested objects: `kobana schema billing.products.reorder.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Order updated. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products list

List products

```text
kobana billing products list [OPTIONS]
```

`GET /v1/products`

Returns a paginated list of the current organization's products.

**Query parameters**

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | `integer` | no | Page (starts at 1). Default: `1`. |
| `per_page` | `integer` | no | Items per page. Default: `20`. Maximum: `100`. |
| `sort_by` | `string` | no | Sort field. Default depends on the resource. |
| `sort_order` | `string` | no | Sort direction. Default: `desc`. Values: `asc`, `desc`. |
| `status` | `string` | no | Filter by product status. Values: `all`, `draft`, `active`, `archived`. |
| `visibility` | `string` | no | Filter by visibility. Values: `public`, `private`. |
| `product_type` | `string` | no | Filter by product type. Values: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `product_group_id` | `string` | no | Filter by product group. |
| `company_id` | `string` | no | Filter by company. |
| `parent_product_id` | `string` | no | Filter by parent product. |
| `root_only` | `string` | no | When `true`, returns only root products (with no parent). Values: `true`, `false`. |
| `search` | `string` | no | Free-text search (name, slug). |
| `service_item_id` | `string` | no | Filter by linked service item. |
| `is_required` | `string` | no | When `true`, returns only products marked as required. Values: `true`, `false`. |
| `self_service_editable` | `string` | no | When `true`, returns only products that the customer can edit in the portal. Values: `true`, `false`. |
| `external_id` | `string` | no | Filter by external identifier. |
| `slug` | `string` | no | Filter by exact product slug. |
| `created_after` | `string` | no | Start of creation date range (YYYY-MM-DD). |
| `created_before` | `string` | no | End of creation date range (YYYY-MM-DD). |
| `updated_after` | `string` | no | Start of update date range (YYYY-MM-DD). |
| `updated_before` | `string` | no | End of update date range (YYYY-MM-DD). |

**Responses**

| HTTP | Description |
|---|---|
| `200` | Paginated list of products. |
| `404` | Resource not found in the current organization. |

## products create

Create product

```text
kobana billing products create [OPTIONS]
```

`POST /v1/products`

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | yes | Product name. |
| `slug` | `string` | yes | Unique slug within the group. Accepts only `[a-z0-9_-]+`. |
| `description` | `string` | no |  |
| `product_type` | `string` | no | Product type. Default: `base`. Values: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `visibility` | `string` | no | Portal visibility. Default: `public`. Values: `public`, `private`. |
| `unit_label` | `string` | no |  |
| `increment` | `integer` | no | Minimum increment. Default: `1`. |
| `website_url` | `string (uri)` | no |  |
| `is_required` | `boolean` | no |  |
| `parent_product_id` | `string (uuid)` | no |  |
| `product_group_id` | `string (uuid)` | no |  |
| `company_id` | `string (uuid)` | no |  |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no |  |

More fields and nested objects: `kobana schema billing.products.create`.

**Responses**

| HTTP | Description |
|---|---|
| `201` | Product created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products update

Upsert product

```text
kobana billing products update [OPTIONS]
```

`PUT /v1/products`

Takes the same body as POST, plus `id` and `public_id`, and dedups on `id`, `public_id`, `external_id` and the resource's natural keys, in that order, stopping at the first key that finds a record. A match is updated and returns 200; otherwise the record is created and returns 201. The POST's required fields are required only when no record is found, and updates are partial: absent fields keep their stored value.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `name` | `string` | no | Product name. |
| `slug` | `string` | no | Unique slug within the group. Accepts only `[a-z0-9_-]+`. |
| `description` | `string` | no |  |
| `product_type` | `string` | no | Product type. Default: `base`. Values: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `visibility` | `string` | no | Portal visibility. Default: `public`. Values: `public`, `private`. |
| `unit_label` | `string` | no |  |
| `increment` | `integer` | no | Minimum increment. Default: `1`. |
| `website_url` | `string (uri)` | no |  |
| `is_required` | `boolean` | no |  |
| `parent_product_id` | `string (uuid)` | no |  |
| `product_group_id` | `string (uuid)` | no |  |
| `company_id` | `string (uuid)` | no |  |
| `external_id` | `string` | no | Identifier in your external system, for idempotency/sync. |
| `custom_metadata` | `object` | no |  |
| `id` | `string (uuid)` | no | Record UUID. Used only to locate it — never written. |
| `public_id` | `string (uuid)` | no | Public UUID of the record, used to locate it. |

More fields and nested objects: `kobana schema billing.products.update`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Record updated. |
| `201` | Record created. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products bulk-archive

Bulk archive products

```text
kobana billing products bulk-archive [OPTIONS]
```

`POST /v1/products/bulk-archive`

Archives multiple products. IDs in use by active subscriptions or already archived are skipped and reported in the response.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes | IDs of products to archive. |

More fields and nested objects: `kobana schema billing.products.bulk-archive`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk operation result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products bulk-change-company

Bulk change product company

```text
kobana billing products bulk-change-company [OPTIONS]
```

`POST /v1/products/bulk-change-company`

Reassigns multiple products to another company. Send `company_id: null` to clear the link.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes |  |
| `company_id` | `string (uuid)` | yes | Target company. Use `null` to clear the link. |

More fields and nested objects: `kobana schema billing.products.bulk-change-company`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk operation result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products bulk-move-group

Move products to another group

```text
kobana billing products bulk-move-group [OPTIONS]
```

`POST /v1/products/bulk-move-group`

Moves multiple products to a product group. Send `product_group_id: null` to remove from the current group.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes |  |
| `product_group_id` | `string (uuid)` | yes | Target group. Use `null` to remove from the current group. |

More fields and nested objects: `kobana schema billing.products.bulk-move-group`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk operation result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products bulk-visibility

Bulk change visibility

```text
kobana billing products bulk-visibility [OPTIONS]
```

`POST /v1/products/bulk-visibility`

Updates the visibility (`public` or `private`) of multiple products.

**Request body**

| Name | Type | Required | Description |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | yes |  |
| `visibility` | `string` | yes | New visibility to apply to all products. Values: `public`, `private`. |

More fields and nested objects: `kobana schema billing.products.bulk-visibility`.

**Responses**

| HTTP | Description |
|---|---|
| `200` | Bulk operation result. |
| `400` | Request body failed validation. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products get

Get product

```text
kobana billing products get [ID] [OPTIONS]
```

`GET /v1/products/{id}`

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Product detail. |
| `404` | Resource not found in the current organization. |

## products delete

Delete product

```text
kobana billing products delete [ID] [OPTIONS]
```

`DELETE /v1/products/{id}`

Deletes the product (soft delete: `deleted_at` — it disappears from every read and leaves every plan; subscriptions and invoices that cite it keep the reference). Requires the `delete` access level. Answers 422 when the product is in use by active subscriptions. Until October 2026 this route archived the product — archiving is `POST /v1/products/{id}/archive`, reversible.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Product deleted. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products archive

Archive product

```text
kobana billing products archive [ID] [OPTIONS]
```

`POST /v1/products/{id}/archive`

Archives the product. Returns 422 when the product is in use by active subscriptions or is already archived.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Product archived. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products publish

Publish product

```text
kobana billing products publish [ID] [OPTIONS]
```

`POST /v1/products/{id}/publish`

Promotes a draft product to active status.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Product published. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |

## products restore

Restore archived product

```text
kobana billing products restore [ID] [OPTIONS]
```

`POST /v1/products/{id}/restore`

Restores an archived product back to active status.

**Path arguments:** `id`

**Responses**

| HTTP | Description |
|---|---|
| `200` | Product restored. |
| `404` | Resource not found in the current organization. |
| `422` | Request body failed validation. |
