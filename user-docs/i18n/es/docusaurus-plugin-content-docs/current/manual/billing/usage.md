---
id: usage
title: "kobana billing usage"
sidebar_label: "usage"
---

# kobana billing usage


Comandos de `kobana billing usage`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`usage credits`](#usage-credits) | `GET /v1/usage/credits` | Consultar consumo y saldo de créditos |
| [`usage credits publish`](#usage-credits-publish) | `POST /v1/usage/credits/publish` | Publicar consumo de créditos |
| [`usage snapshots daily list`](#usage-snapshots-daily-list) | `GET /v1/usage/snapshots/daily` | Listar snapshots diarios de uso |
| [`usage snapshots daily create`](#usage-snapshots-daily-create) | `POST /v1/usage/snapshots/daily` | Crear snapshots diarios de uso |
| [`usage snapshots monthly list`](#usage-snapshots-monthly-list) | `GET /v1/usage/snapshots/monthly` | Listar snapshots mensuales de uso |
| [`usage snapshots monthly create`](#usage-snapshots-monthly-create) | `POST /v1/usage/snapshots/monthly` | Crear snapshots mensuales de uso |
| [`usage summary`](#usage-summary) | `GET /v1/usage/summary` | Resumen de uso del período |
| [`usage list`](#usage-list) | `GET /v1/usage` | Listar registros de uso |
| [`usage create`](#usage-create) | `POST /v1/usage` | Reportar uso (metered) |

## usage credits

Consultar consumo y saldo de créditos

```text
kobana billing usage credits [OPCIONES] [COMANDO]
```

`GET /v1/usage/credits`

Devuelve el consumo de créditos prepagos en el período (totales y serie diaria) y el saldo actual. Sin `billing_account_id`, suma todas las cuentas de la organización. Sin fechas, usa el mes actual (huso de São Paulo).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string` | no | Filtra por cuenta de cobro. |
| `product_id` | `string` | no | Producto de créditos. Por defecto: el primer producto activo. |
| `start_date` | `string` | no | Inicio del período (`YYYY-MM-DD`). |
| `end_date` | `string` | no | Fin del período, inclusive (`YYYY-MM-DD`). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Consumo del período y saldo. |
| `404` | Recurso no encontrado en la organización actual. |

## usage credits publish

Publicar consumo de créditos

```text
kobana billing usage credits publish [OPCIONES]
```

`POST /v1/usage/credits/publish`

Publica el consumo de créditos prepagos (producto de tipo `credits`) por cuenta de cobro × producto × día, registro único **o** en lote `{ records: [...] }` con hasta **100** ítems. La clave (cuenta, producto, `usage_date`) es única: publicar el mismo día de nuevo **sobrescribe** el registro (`overwritten: true`), nunca suma. Los lotes con fallos parciales devuelven **207 Multi-Status**.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema billing.usage.credits.publish`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registros procesados (`207` cuando parte del lote falló). |
| `207` | Registros procesados (`207` cuando parte del lote falló). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## usage snapshots daily list

Listar snapshots diarios de uso

```text
kobana billing usage snapshots daily list [OPCIONES]
```

`GET /v1/usage/snapshots/daily`

Lista los snapshots diarios de uso de la organización. Filtros opcionales: `billing_account_id`, `product_id` y ventana `start_date`/`end_date`. Admite paginación por `limit`/`offset`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string` | no | Filtra snapshots por UUID de la cuenta de cobro. |
| `product_id` | `string` | no | Filtra snapshots por UUID del producto. |
| `start_date` | `string` | no | Fecha/hora inicial del filtro — formato ISO 8601. |
| `end_date` | `string` | no | Fecha/hora final del filtro — formato ISO 8601. |
| `limit` | `integer` | no | Ítems por página. Por defecto: `100`. Máximo: `1000`. |
| `offset` | `integer,null` | no | Desplazamiento (offset) para paginación. Por defecto: `0`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de snapshots diarios con `meta` que incluye `total`, `limit`, `offset`. |
| `404` | Recurso no encontrado en la organización actual. |

## usage snapshots daily create

Crear snapshots diarios de uso

```text
kobana billing usage snapshots daily create [OPCIONES]
```

`POST /v1/usage/snapshots/daily`

Crea snapshots diarios de uso, registro único **o** lote `{ records: [...] }` con hasta **100** ítems. La clave natural (organización, cuenta de cobro, producto, fecha) es única: si ya existe, el snapshot es sobrescrito (`overwritten: true`). Resuelve la cuenta por `billing_account_id` **o** `billing_account_public_id` y el producto por `product_slug` **o** `product_id` dentro del `product_group_slug` informado. Los lotes con fallas parciales devuelven **207 Multi-Status**.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema billing.usage.snapshots.daily.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado del procesamiento (registro único o lote). |
| `207` | Resultado del procesamiento (registro único o lote). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## usage snapshots monthly list

Listar snapshots mensuales de uso

```text
kobana billing usage snapshots monthly list [OPCIONES]
```

`GET /v1/usage/snapshots/monthly`

Lista los snapshots mensuales de uso de la organización. Filtros opcionales: `billing_account_id`, `product_id` y ventana `start_month`/`end_month`. Admite paginación por `limit`/`offset`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string` | no | Filtra snapshots por UUID de la cuenta de cobro. |
| `product_id` | `string` | no | Filtra snapshots por UUID del producto. |
| `start_month` | `string` | no | Mes inicial del filtro en formato `YYYY-MM-DD` (primer día del mes). |
| `end_month` | `string` | no | Mes final del filtro en formato `YYYY-MM-DD` (primer día del mes). |
| `limit` | `integer` | no | Ítems por página. Por defecto: `100`. Máximo: `1000`. |
| `offset` | `integer,null` | no | Desplazamiento (offset) para paginación. Por defecto: `0`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de snapshots mensuales con `meta` que incluye `total`, `limit`, `offset`. |
| `404` | Recurso no encontrado en la organización actual. |

## usage snapshots monthly create

Crear snapshots mensuales de uso

```text
kobana billing usage snapshots monthly create [OPCIONES]
```

`POST /v1/usage/snapshots/monthly`

Crea snapshots mensuales de uso, registro único **o** lote `{ records: [...] }` con hasta **100** ítems. La clave natural (organización, cuenta de cobro, producto, mes) es única: si ya existe, el snapshot es sobrescrito (`overwritten: true`). Resuelve la cuenta por `billing_account_id` **o** `billing_account_public_id` y el producto por `product_slug` **o** `product_id` dentro de `product_group_slug`. Los lotes con fallas parciales devuelven **207 Multi-Status**.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema billing.usage.snapshots.monthly.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado del procesamiento (registro único o lote). |
| `207` | Resultado del procesamiento (registro único o lote). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## usage summary

Resumen de uso del período

```text
kobana billing usage summary [OPCIONES]
```

`GET /v1/usage/summary`

Devuelve la agregación de uso de un ítem de suscripción en el período de cobro vigente (o en un período arbitrario vía `period_start`/`period_end`). Considera registros `set` (que reemplazan) y la suma de los `increment` posteriores. Cuando el precio es por tiers o unitario, calcula un costo estimado en `estimated_cost` (float, hasta 4 decimales).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `subscription_item_id` | `string` | no | UUID del ítem de suscripción. Alternativa a `product_slug` + `customer_id`. |
| `product_slug` | `string` | no | Slug del producto. Combina con `customer_id`. |
| `customer_id` | `string` | no | Identificador externo del cliente. Combina con `product_slug`. |
| `period_start` | `string` | no | Inicio del período (ISO 8601). Por defecto: `current_period_start` de la suscripción. |
| `period_end` | `string` | no | Fin del período (ISO 8601). Por defecto: `current_period_end` de la suscripción. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resumen de uso, período y costo estimado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## usage list

Listar registros de uso

```text
kobana billing usage list [OPCIONES]
```

`GET /v1/usage`

Devuelve los registros de uso (metered) de un ítem de suscripción. Identifica el ítem por `subscription_item_id` **o** por la combinación `product_slug` + `customer_id`. Admite filtros por rango de fechas y ventana `limit`/`offset`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `subscription_item_id` | `string` | no | UUID del ítem de suscripción. Alternativa a `product_slug` + `customer_id`. |
| `product_slug` | `string` | no | Slug del producto. Combina con `customer_id`. |
| `customer_id` | `string` | no | Identificador externo del cliente. Combina con `product_slug`. |
| `start_date` | `string` | no | Fecha/hora inicial del filtro — formato ISO 8601. |
| `end_date` | `string` | no | Fecha/hora final del filtro — formato ISO 8601. |
| `limit` | `integer` | no | Ítems por página. Por defecto: `100`. Máximo: `1000`. |
| `offset` | `integer,null` | no | Desplazamiento (offset) para paginación. Por defecto: `0`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de registros de uso con agregación `total_quantity` en `meta`. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## usage create

Reportar uso (metered)

```text
kobana billing usage create [OPCIONES]
```

`POST /v1/usage`

Reporta uso para facturación metered. Acepta un registro único **o** un lote `{ records: [...] }` con hasta **100** ítems. Cada registro debe identificar el ítem de suscripción por `subscription_item_id` **o** por `product_slug` + `customer_id` (recomendado para integraciones externas). El precio/producto debe estar configurado con `usage_type=metered` y la suscripción debe estar `active` o `trialing`. Usa `idempotency_key` para evitar duplicados — cuando la clave ya fue vista, la respuesta marca `duplicate: true`. Los lotes con fallas parciales devuelven **207 Multi-Status**.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema billing.usage.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado del procesamiento (registro único o lote). |
| `207` | Resultado del procesamiento (registro único o lote). |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
