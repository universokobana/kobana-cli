---
id: coupons
title: "kobana billing coupons"
sidebar_label: "coupons"
---

# kobana billing coupons


Comandos de `kobana billing coupons`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`coupons list`](#coupons-list) | `GET /v1/coupons` | Listar cupones |
| [`coupons create`](#coupons-create) | `POST /v1/coupons` | Crear cupón |
| [`coupons update`](#coupons-update) | `PUT /v1/coupons` | Crear o actualizar cupón |
| [`coupons get`](#coupons-get) | `GET /v1/coupons/{id}` | Obtener cupón |
| [`coupons delete`](#coupons-delete) | `DELETE /v1/coupons/{id}` | Eliminar cupón |
| [`coupons deactivate`](#coupons-deactivate) | `POST /v1/coupons/{id}/deactivate` | Desactivar cupón |
| [`coupons redemptions`](#coupons-redemptions) | `GET /v1/coupons/{id}/redemptions` | Listar canjes del cupón |

## coupons list

Listar cupones

```text
kobana billing coupons list [OPCIONES]
```

`GET /v1/coupons`

Devuelve una lista paginada de los cupones de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `is_active` | `string` | no | Filtra por estado: `true` devuelve solo cupones activos; `false`, solo inactivos. Valores: `true`, `false`. |
| `search` | `string` | no | Búsqueda textual libre en `code` y `name`. |
| `discount_type` | `string` | no | Filtra por tipo de descuento: `percentage` o `fixed_amount`. Valores: `percentage`, `fixed_amount`. |
| `code` | `string` | no | Filtra por el código exacto del cupón. |
| `min_purchase_min` | `number,null` | no | Valor mínimo de `min_purchase` (en BRL) para filtrar cupones. |
| `min_purchase_max` | `number,null` | no | Valor máximo de `min_purchase` (en BRL) para filtrar cupones. |
| `max_discount_min` | `number,null` | no | Valor mínimo de `max_discount` (en BRL) para filtrar cupones. |
| `max_discount_max` | `number,null` | no | Valor máximo de `max_discount` (en BRL) para filtrar cupones. |
| `valid_from_from` | `string` | no | Devuelve cupones cuya fecha de inicio de validez es igual o posterior a esta fecha (AAAA-MM-DD). |
| `valid_from_to` | `string` | no | Devuelve cupones cuya fecha de inicio de validez es igual o anterior a esta fecha (AAAA-MM-DD). |
| `valid_until_from` | `string` | no | Devuelve cupones cuya fecha de expiración es igual o posterior a esta fecha (AAAA-MM-DD). |
| `valid_until_to` | `string` | no | Devuelve cupones cuya fecha de expiración es igual o anterior a esta fecha (AAAA-MM-DD). |
| `created_after` | `string` | no | Devuelve cupones creados a partir de esta fecha (AAAA-MM-DD). |
| `created_before` | `string` | no | Devuelve cupones creados hasta esta fecha (AAAA-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de cupones. |
| `404` | Recurso no encontrado en la organización actual. |

## coupons create

Crear cupón

```text
kobana billing coupons create [OPCIONES]
```

`POST /v1/coupons`

Crea un cupón de descuento. El `code` debe ser único por organización. Usa `discount_type=percentage` con `discount_value` entre `0` y `100`, o `discount_type=fixed_amount` con `discount_value` en BRL (float, hasta 4 decimales).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `code` | `string` | sí | Código público del cupón — único por organización. |
| `name` | `string` | sí | Nombre interno del cupón. |
| `discount_type` | `string` | sí | Tipo de descuento: `percentage` o `fixed_amount`. Valores: `percentage`, `fixed_amount`. |
| `discount_value` | `number` | sí | Valor del descuento. Para `percentage`, entero entre `0` y `100`. Para `fixed_amount`, importe en BRL (float, hasta 4 decimales). |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `description` | `string` | no | Descripción opcional. |
| `currency` | `string` | no | Moneda ISO 4217. Por defecto: `BRL`. |
| `min_purchase` | `number` | no | Importe mínimo de la factura, en BRL (float, hasta 4 decimales), para que el cupón pueda aplicarse. |
| `max_discount` | `number` | no | Tope absoluto del descuento, en BRL (float, hasta 4 decimales). |
| `usage_limit` | `integer` | no | Máximo total de canjes en la organización. Omite para ilimitado. |
| `usage_limit_per_user` | `integer` | no | Máximo de canjes por cuenta de cobro. Omite para ilimitado. |
| `valid_from` | `string` | no | Inicio del periodo de validez (ISO 8601 — fecha o fecha/hora). |
| `valid_until` | `string` | no | Fin del periodo de validez (ISO 8601 — fecha o fecha/hora). |
| `applies_to` | `object` | no |  |
| `excludes` | `object` | no |  |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.coupons.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cupón creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## coupons update

Crear o actualizar cupón

```text
kobana billing coupons update [OPCIONES]
```

`PUT /v1/coupons`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `code` | `string` | no | Código público del cupón — único por organización. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `name` | `string` | no | Nombre interno del cupón. |
| `description` | `string` | no | Descripción opcional. |
| `discount_type` | `string` | no | Tipo de descuento: `percentage` o `fixed_amount`. Valores: `percentage`, `fixed_amount`. |
| `discount_value` | `number` | no | Valor del descuento. Para `percentage`, entero entre `0` y `100`. Para `fixed_amount`, importe en BRL (float, hasta 4 decimales). |
| `currency` | `string` | no | Moneda ISO 4217. Por defecto: `BRL`. |
| `min_purchase` | `number` | no | Importe mínimo de la factura, en BRL (float, hasta 4 decimales), para que el cupón pueda aplicarse. |
| `max_discount` | `number` | no | Tope absoluto del descuento, en BRL (float, hasta 4 decimales). |
| `usage_limit` | `integer` | no | Máximo total de canjes en la organización. Omite para ilimitado. |
| `usage_limit_per_user` | `integer` | no | Máximo de canjes por cuenta de cobro. Omite para ilimitado. |
| `valid_from` | `string` | no | Inicio del periodo de validez (ISO 8601 — fecha o fecha/hora). |
| `valid_until` | `string` | no | Fin del periodo de validez (ISO 8601 — fecha o fecha/hora). |
| `applies_to` | `object` | no |  |
| `excludes` | `object` | no |  |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.coupons.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## coupons get

Obtener cupón

```text
kobana billing coupons get [ID] [OPCIONES]
```

`GET /v1/coupons/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del cupón. |
| `404` | Recurso no encontrado en la organización actual. |

## coupons delete

Eliminar cupón

```text
kobana billing coupons delete [ID] [OPCIONES]
```

`DELETE /v1/coupons/{id}`

Elimina el cupón permanentemente. Solo se permite mientras el cupón **no tenga canjes registrados** — de lo contrario, usa `POST /coupons/{id}/deactivate` para preservar el historial.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cupón eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## coupons deactivate

Desactivar cupón

```text
kobana billing coupons deactivate [ID] [OPCIONES]
```

`POST /v1/coupons/{id}/deactivate`

Desactiva el cupón: los nuevos canjes quedan bloqueados, pero los canjes anteriores se preservan.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cupón desactivado. |
| `404` | Recurso no encontrado en la organización actual. |

## coupons redemptions

Listar canjes del cupón

```text
kobana billing coupons redemptions [ID] [OPCIONES]
```

`GET /v1/coupons/{id}/redemptions`

Devuelve una lista paginada de los canjes registrados para el cupón.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de canjes del cupón. |
| `404` | Recurso no encontrado en la organización actual. |
