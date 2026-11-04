---
id: product-groups
title: "kobana billing product-groups"
sidebar_label: "product-groups"
---

# kobana billing product-groups


Comandos de `kobana billing product-groups`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`product-groups reorder create`](#product-groups-reorder-create) | `POST /v1/product_groups/reorder` | Reordenar grupos de productos |
| [`product-groups reorder update`](#product-groups-reorder-update) | `PUT /v1/product_groups/reorder` | Reordenar grupos de productos (alias PUT) |
| [`product-groups list`](#product-groups-list) | `GET /v1/product_groups` | Listar grupos de productos |
| [`product-groups create`](#product-groups-create) | `POST /v1/product_groups` | Crear grupo de productos |
| [`product-groups update`](#product-groups-update) | `PUT /v1/product_groups` | Crear o actualizar grupo de productos |
| [`product-groups get`](#product-groups-get) | `GET /v1/product_groups/{id}` | Obtener grupo de productos |
| [`product-groups delete`](#product-groups-delete) | `DELETE /v1/product_groups/{id}` | Eliminar grupo de productos |
| [`product-groups archive`](#product-groups-archive) | `POST /v1/product_groups/{id}/archive` | Archivar grupo de productos |
| [`product-groups unarchive`](#product-groups-unarchive) | `POST /v1/product_groups/{id}/unarchive` | Desarchivar grupo de productos |

## product-groups reorder create

Reordenar grupos de productos

```text
kobana billing product-groups reorder create [OPCIONES]
```

`POST /v1/product_groups/reorder`

Actualiza el orden de visualización de los grupos de productos. El array `group_ids` define el nuevo orden — el índice 0 pasa a ser el primer grupo.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | sí | Lista de UUIDs en el nuevo orden deseado. Todos los IDs deben pertenecer a la organización. |

Otros campos y objetos anidados: `kobana schema billing.product-groups.reorder.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Orden actualizado con éxito. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## product-groups reorder update

Reordenar grupos de productos (alias PUT)

```text
kobana billing product-groups reorder update [OPCIONES]
```

`PUT /v1/product_groups/reorder`

Alias `PUT` para `POST /product_groups/reorder`. Mismo contrato.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | sí | Lista de UUIDs en el nuevo orden deseado. Todos los IDs deben pertenecer a la organización. |

Otros campos y objetos anidados: `kobana schema billing.product-groups.reorder.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Orden actualizado con éxito. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## product-groups list

Listar grupos de productos

```text
kobana billing product-groups list [OPCIONES]
```

`GET /v1/product_groups`

Devuelve una lista paginada de los grupos de productos de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda textual libre por el nombre del grupo. |
| `company_id` | `string` | no | Filtra por empresa emisora asociada. |
| `include_archived` | `boolean,null` | no | Cuando es `true`, incluye grupos archivados en el listado. Por defecto: `false`. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `archived_after` | `string` | no | Devuelve grupos archivados a partir de esta fecha (ISO 8601). |
| `archived_before` | `string` | no | Devuelve grupos archivados hasta esta fecha (ISO 8601). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de grupos de productos. |
| `404` | Recurso no encontrado en la organización actual. |

## product-groups create

Crear grupo de productos

```text
kobana billing product-groups create [OPCIONES]
```

`POST /v1/product_groups`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del grupo. |
| `slug` | `string` | no | Slug del grupo. Generado a partir del nombre cuando se omite. |
| `visibility` | `string` | no | Visibilidad en el portal del cliente. Por defecto: `public`. Valores: `public`, `private`. |
| `company_id` | `string (uuid)` | no |  |
| `proposal_template_id` | `string (uuid)` | no |  |
| `service_item_id` | `string (uuid)` | no |  |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.product-groups.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Grupo de productos creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## product-groups update

Crear o actualizar grupo de productos

```text
kobana billing product-groups update [OPCIONES]
```

`PUT /v1/product_groups`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre del grupo. |
| `slug` | `string` | no | Slug del grupo. Generado a partir del nombre cuando se omite. |
| `visibility` | `string` | no | Visibilidad en el portal del cliente. Por defecto: `public`. Valores: `public`, `private`. |
| `company_id` | `string (uuid)` | no |  |
| `proposal_template_id` | `string (uuid)` | no |  |
| `service_item_id` | `string (uuid)` | no |  |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.product-groups.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## product-groups get

Obtener grupo de productos

```text
kobana billing product-groups get [ID] [OPCIONES]
```

`GET /v1/product_groups/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del grupo de productos. |
| `404` | Recurso no encontrado en la organización actual. |

## product-groups delete

Eliminar grupo de productos

```text
kobana billing product-groups delete [ID] [OPCIONES]
```

`DELETE /v1/product_groups/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Confirmación de eliminación. |
| `404` | Recurso no encontrado en la organización actual. |

## product-groups archive

Archivar grupo de productos

```text
kobana billing product-groups archive [ID] [OPCIONES]
```

`POST /v1/product_groups/{id}/archive`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Grupo de productos archivado. |
| `404` | Recurso no encontrado en la organización actual. |

## product-groups unarchive

Desarchivar grupo de productos

```text
kobana billing product-groups unarchive [ID] [OPCIONES]
```

`POST /v1/product_groups/{id}/unarchive`

Devuelve al catálogo un grupo de productos archivado. Exige el nivel de acceso `write`. Reemplaza `DELETE /v1/product_groups/{id}/archive`, eliminado: un `DELETE` siempre es una eliminación.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Grupo de productos desarchivado. |
| `404` | Recurso no encontrado en la organización actual. |
