---
id: products
title: "kobana billing products"
sidebar_label: "products"
---

# kobana billing products


Comandos de `kobana billing products`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`products bulk delete`](#products-bulk-delete) | `DELETE /v1/products/bulk` | Eliminar productos en lote |
| [`products reorder create`](#products-reorder-create) | `POST /v1/products/reorder` | Reordenar productos |
| [`products reorder update`](#products-reorder-update) | `PUT /v1/products/reorder` | Reordenar productos (PUT /reorder) |
| [`products list`](#products-list) | `GET /v1/products` | Listar productos |
| [`products create`](#products-create) | `POST /v1/products` | Crear producto |
| [`products update`](#products-update) | `PUT /v1/products` | Crear o actualizar producto |
| [`products bulk-archive`](#products-bulk-archive) | `POST /v1/products/bulk-archive` | Archivar productos en lote |
| [`products bulk-change-company`](#products-bulk-change-company) | `POST /v1/products/bulk-change-company` | Cambiar empresa de productos en lote |
| [`products bulk-move-group`](#products-bulk-move-group) | `POST /v1/products/bulk-move-group` | Mover productos a otro grupo |
| [`products bulk-visibility`](#products-bulk-visibility) | `POST /v1/products/bulk-visibility` | Cambiar visibilidad en lote |
| [`products get`](#products-get) | `GET /v1/products/{id}` | Obtener producto |
| [`products delete`](#products-delete) | `DELETE /v1/products/{id}` | Eliminar producto |
| [`products archive`](#products-archive) | `POST /v1/products/{id}/archive` | Archivar producto |
| [`products publish`](#products-publish) | `POST /v1/products/{id}/publish` | Publicar producto |
| [`products restore`](#products-restore) | `POST /v1/products/{id}/restore` | Restaurar producto archivado |

## products bulk delete

Eliminar productos en lote

```text
kobana billing products bulk delete [OPCIONES]
```

`DELETE /v1/products/bulk`

Elimina varios productos (soft delete: `deleted_at`, el mismo efecto que `DELETE /v1/products/{id}` en cada uno). Los IDs en uso por suscripciones activas se ignoran y se reportan en la respuesta. Exige el nivel de acceso `delete`. Reemplaza `POST /v1/products/bulk-delete`, eliminado.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | IDs de los productos a eliminar. |

Otros campos y objetos anidados: `kobana schema billing.products.bulk.delete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la operación en lote. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products reorder create

Reordenar productos

```text
kobana billing products reorder create [OPCIONES]
```

`POST /v1/products/reorder`

Actualiza el orden de visualización de los productos. El índice 0 del array pasa a ser el primer ítem de la lista.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `product_ids` | `array<string (uuid)>` | sí | IDs de productos en el nuevo orden deseado. El índice 0 pasa a ser el primero de la lista. |

Otros campos y objetos anidados: `kobana schema billing.products.reorder.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Orden actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products reorder update

Reordenar productos (PUT /reorder)

```text
kobana billing products reorder update [OPCIONES]
```

`PUT /v1/products/reorder`

Alias `PUT` para `/products/reorder`. Mismo comportamiento que la versión `POST`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `product_ids` | `array<string (uuid)>` | sí | IDs de productos en el nuevo orden deseado. El índice 0 pasa a ser el primero de la lista. |

Otros campos y objetos anidados: `kobana schema billing.products.reorder.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Orden actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products list

Listar productos

```text
kobana billing products list [OPCIONES]
```

`GET /v1/products`

Devuelve una lista paginada de los productos de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra por estado del producto. Valores: `all`, `draft`, `active`, `archived`. |
| `visibility` | `string` | no | Filtra por visibilidad. Valores: `public`, `private`. |
| `product_type` | `string` | no | Filtra por tipo de producto. Valores: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `product_group_id` | `string` | no | Filtra por grupo de productos. |
| `company_id` | `string` | no | Filtra por empresa. |
| `parent_product_id` | `string` | no | Filtra por producto padre. |
| `root_only` | `string` | no | Cuando es `true`, devuelve solo productos raíz (sin padre). Valores: `true`, `false`. |
| `search` | `string` | no | Búsqueda textual libre (nombre, slug). |
| `service_item_id` | `string` | no | Filtra por ítem de servicio vinculado. |
| `is_required` | `string` | no | Cuando es `true`, devuelve solo productos marcados como obligatorios. Valores: `true`, `false`. |
| `self_service_editable` | `string` | no | Cuando es `true`, devuelve solo productos que el cliente puede editar en el portal. Valores: `true`, `false`. |
| `external_id` | `string` | no | Filtra por identificador externo. |
| `slug` | `string` | no | Filtra por slug exacto del producto. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `updated_after` | `string` | no | Inicio del rango de fecha de actualización (YYYY-MM-DD). |
| `updated_before` | `string` | no | Fin del rango de fecha de actualización (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de productos. |
| `404` | Recurso no encontrado en la organización actual. |

## products create

Crear producto

```text
kobana billing products create [OPCIONES]
```

`POST /v1/products`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del producto. |
| `slug` | `string` | sí | Slug único dentro del grupo. Acepta solo `[a-z0-9_-]+`. |
| `description` | `string` | no |  |
| `product_type` | `string` | no | Tipo del producto. Por defecto: `base`. Valores: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `visibility` | `string` | no | Visibilidad en el portal. Por defecto: `public`. Valores: `public`, `private`. |
| `unit_label` | `string` | no |  |
| `increment` | `integer` | no | Incremento mínimo. Por defecto: `1`. |
| `website_url` | `string (uri)` | no |  |
| `is_required` | `boolean` | no |  |
| `parent_product_id` | `string (uuid)` | no |  |
| `product_group_id` | `string (uuid)` | no |  |
| `company_id` | `string (uuid)` | no |  |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema billing.products.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Producto creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products update

Crear o actualizar producto

```text
kobana billing products update [OPCIONES]
```

`PUT /v1/products`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre del producto. |
| `slug` | `string` | no | Slug único dentro del grupo. Acepta solo `[a-z0-9_-]+`. |
| `description` | `string` | no |  |
| `product_type` | `string` | no | Tipo del producto. Por defecto: `base`. Valores: `base`, `addon_quantity`, `addon_fixed`, `metered`, `one_time_fixed`, `one_time_quantity`, `credits`. |
| `visibility` | `string` | no | Visibilidad en el portal. Por defecto: `public`. Valores: `public`, `private`. |
| `unit_label` | `string` | no |  |
| `increment` | `integer` | no | Incremento mínimo. Por defecto: `1`. |
| `website_url` | `string (uri)` | no |  |
| `is_required` | `boolean` | no |  |
| `parent_product_id` | `string (uuid)` | no |  |
| `product_group_id` | `string (uuid)` | no |  |
| `company_id` | `string (uuid)` | no |  |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no |  |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.products.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products bulk-archive

Archivar productos en lote

```text
kobana billing products bulk-archive [OPCIONES]
```

`POST /v1/products/bulk-archive`

Archiva múltiples productos. Los IDs en uso por suscripciones activas o ya archivados se ignoran y se reportan en la respuesta.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | IDs de productos a archivar. |

Otros campos y objetos anidados: `kobana schema billing.products.bulk-archive`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la operación en lote. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products bulk-change-company

Cambiar empresa de productos en lote

```text
kobana billing products bulk-change-company [OPCIONES]
```

`POST /v1/products/bulk-change-company`

Reasigna múltiples productos a otra empresa. Envía `company_id: null` para limpiar el vínculo.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí |  |
| `company_id` | `string (uuid)` | sí | Empresa de destino. Usa `null` para limpiar el vínculo. |

Otros campos y objetos anidados: `kobana schema billing.products.bulk-change-company`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la operación en lote. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products bulk-move-group

Mover productos a otro grupo

```text
kobana billing products bulk-move-group [OPCIONES]
```

`POST /v1/products/bulk-move-group`

Mueve múltiples productos a un grupo de productos. Envía `product_group_id: null` para quitarlos del grupo actual.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí |  |
| `product_group_id` | `string (uuid)` | sí | Grupo de destino. Usa `null` para quitar del grupo actual. |

Otros campos y objetos anidados: `kobana schema billing.products.bulk-move-group`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la operación en lote. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products bulk-visibility

Cambiar visibilidad en lote

```text
kobana billing products bulk-visibility [OPCIONES]
```

`POST /v1/products/bulk-visibility`

Actualiza la visibilidad (`public` o `private`) de múltiples productos.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí |  |
| `visibility` | `string` | sí | Nueva visibilidad a aplicar en todos los productos. Valores: `public`, `private`. |

Otros campos y objetos anidados: `kobana schema billing.products.bulk-visibility`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la operación en lote. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products get

Obtener producto

```text
kobana billing products get [ID] [OPCIONES]
```

`GET /v1/products/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del producto. |
| `404` | Recurso no encontrado en la organización actual. |

## products delete

Eliminar producto

```text
kobana billing products delete [ID] [OPCIONES]
```

`DELETE /v1/products/{id}`

Elimina el producto (soft delete: `deleted_at` — desaparece de toda lectura y sale de todos los planes; las suscripciones y facturas que lo citan guardan la referencia). Exige el nivel de acceso `delete`. Devuelve 422 cuando el producto está en uso por suscripciones activas. Hasta octubre de 2026 esta ruta archivaba el producto — archivar es `POST /v1/products/{id}/archive`, reversible.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Producto eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products archive

Archivar producto

```text
kobana billing products archive [ID] [OPCIONES]
```

`POST /v1/products/{id}/archive`

Archiva el producto. Devuelve 422 cuando el producto está en uso por suscripciones activas o ya está archivado.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Producto archivado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products publish

Publicar producto

```text
kobana billing products publish [ID] [OPCIONES]
```

`POST /v1/products/{id}/publish`

Promueve un producto en borrador al estado activo.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Producto publicado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## products restore

Restaurar producto archivado

```text
kobana billing products restore [ID] [OPCIONES]
```

`POST /v1/products/{id}/restore`

Restaura un producto archivado al estado activo.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Producto restaurado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
