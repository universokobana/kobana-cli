---
id: billing-accounts
title: "kobana billing billing-accounts"
sidebar_label: "billing-accounts"
---

# kobana billing billing-accounts


Comandos de `kobana billing billing-accounts`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`billing-accounts list`](#billing-accounts-list) | `GET /v1/billing_accounts` | Listar cuentas de cobro |
| [`billing-accounts create`](#billing-accounts-create) | `POST /v1/billing_accounts` | Crear cuenta de cobro |
| [`billing-accounts update`](#billing-accounts-update) | `PUT /v1/billing_accounts` | Crear o actualizar cuenta de cobro |
| [`billing-accounts get`](#billing-accounts-get) | `GET /v1/billing_accounts/{id}` | Obtener cuenta de cobro |
| [`billing-accounts delete`](#billing-accounts-delete) | `DELETE /v1/billing_accounts/{id}` | Eliminar cuenta de cobro |
| [`billing-accounts close`](#billing-accounts-close) | `POST /v1/billing_accounts/{id}/close` | Cerrar cuenta de cobro |
| [`billing-accounts credits`](#billing-accounts-credits) | `GET /v1/billing_accounts/{id}/credits` | Listar créditos de la cuenta |
| [`billing-accounts entitlements`](#billing-accounts-entitlements) | `GET /v1/billing_accounts/{id}/entitlements` | Listar productos contratados de la cuenta |
| [`billing-accounts invoices`](#billing-accounts-invoices) | `GET /v1/billing_accounts/{id}/invoices` | Listar facturas de la cuenta |
| [`billing-accounts reactivate`](#billing-accounts-reactivate) | `POST /v1/billing_accounts/{id}/reactivate` | Reactivar cuenta de cobro |
| [`billing-accounts subscriptions`](#billing-accounts-subscriptions) | `GET /v1/billing_accounts/{id}/subscriptions` | Listar suscripciones de la cuenta |
| [`billing-accounts suspend`](#billing-accounts-suspend) | `POST /v1/billing_accounts/{id}/suspend` | Suspender cuenta de cobro |

## billing-accounts list

Listar cuentas de cobro

```text
kobana billing billing-accounts list [OPCIONES]
```

`GET /v1/billing_accounts`

Devuelve una lista paginada de las cuentas de cobro de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra por estado de la cuenta de cobro. Valores: `active`, `suspended`, `closed`. |
| `search` | `string` | no | Búsqueda textual libre. |
| `external_id` | `string` | no | Filtra por el identificador externo de la cuenta. |
| `currency` | `string` | no | Filtra por moneda ISO 4217 (ej.: `BRL`). |
| `auto_collection` | `string` | no | Filtra por cobro automático habilitado (`true`) o deshabilitado (`false`). Valores: `true`, `false`. |
| `tax_exempt` | `string` | no | Filtra cuentas exentas de impuestos (`true`) o no exentas (`false`). Valores: `true`, `false`. |
| `customer_tax_type` | `string` | no | Filtra por tipo tributario del cliente. |
| `balance_min` | `number,null` | no | Saldo mínimo de la cuenta en BRL (float, hasta 4 decimales). |
| `balance_max` | `number,null` | no | Saldo máximo de la cuenta en BRL (float, hasta 4 decimales). |
| `mrr_min` | `number,null` | no | MRR mínimo en BRL (float, hasta 4 decimales). |
| `mrr_max` | `number,null` | no | MRR máximo en BRL (float, hasta 4 decimales). |
| `allowed_negative_balance_min` | `number,null` | no | Saldo negativo permitido mínimo en BRL (float, hasta 4 decimales). |
| `allowed_negative_balance_max` | `number,null` | no | Saldo negativo permitido máximo en BRL (float, hasta 4 decimales). |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `updated_after` | `string` | no | Inicio del rango de fecha de actualización (YYYY-MM-DD). |
| `updated_before` | `string` | no | Fin del rango de fecha de actualización (YYYY-MM-DD). |
| `suspended_after` | `string` | no | Filtra cuentas suspendidas a partir de esta fecha (formato `YYYY-MM-DD`). |
| `suspended_before` | `string` | no | Filtra cuentas suspendidas hasta esta fecha (formato `YYYY-MM-DD`). |
| `person_id` | `string` | no | Filtra por UUID de la persona asociada. |
| `person_document_number` | `string` | no | Filtra por CPF/CNPJ de la persona (coincidencia exacta). |
| `person_name_contains` | `string` | no | Filtra por nombre de la persona (búsqueda parcial, sin distinción de mayúsculas). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de cuentas de cobro. |
| `404` | Recurso no encontrado en la organización actual. |

## billing-accounts create

Crear cuenta de cobro

```text
kobana billing billing-accounts create [OPCIONES]
```

`POST /v1/billing_accounts`

Crea una cuenta de cobro asociada a un cliente. El bloque `customer` localiza un cliente existente por `id`, `external_id` o `document_number`; cuando no se encuentra, crea uno nuevo (requiere `name` y `document_number`).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `customer` | `object` | sí |  |
| `external_id` | `string` | no | Identificador externo de la cuenta en el sistema del cliente. |
| `billing_email` | `string (email)` | no | Correo de cobro. Por defecto: `customer.email`. |
| `payment_terms_days` | `integer` | no | Plazo de vencimiento de factura en días (1–90). Por defecto: el plazo predeterminado configurado en la organización (10 cuando no está configurado). |
| `auto_collection` | `boolean` | no | Habilita el cobro automático. Por defecto: `true`. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.billing-accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cuenta de cobro creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## billing-accounts update

Crear o actualizar cuenta de cobro

```text
kobana billing billing-accounts update [OPCIONES]
```

`PUT /v1/billing_accounts`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `customer` | `object` | no |  |
| `external_id` | `string` | no | Identificador externo de la cuenta en el sistema del cliente. |
| `billing_email` | `string (email)` | no | Correo de cobro. Por defecto: `customer.email`. |
| `payment_terms_days` | `integer` | no | Plazo de vencimiento de factura en días (1–90). Por defecto: el plazo predeterminado configurado en la organización (10 cuando no está configurado). |
| `auto_collection` | `boolean` | no | Habilita el cobro automático. Por defecto: `true`. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.billing-accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## billing-accounts get

Obtener cuenta de cobro

```text
kobana billing billing-accounts get [ID] [OPCIONES]
```

`GET /v1/billing_accounts/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la cuenta de cobro. |
| `404` | Recurso no encontrado en la organización actual. |

## billing-accounts delete

Eliminar cuenta de cobro

```text
kobana billing billing-accounts delete [ID] [OPCIONES]
```

`DELETE /v1/billing_accounts/{id}`

Elimina la cuenta de cobro (soft delete: `deleted_at`), junto con sus métodos de pago, tarjetas, usuarios del portal y archivos — todo desaparece de toda lectura; las facturas, pagos y NFS-e ya emitidos quedan en el historial. Exige el nivel de acceso `delete`. Devuelve 422 mientras haya una suscripción que no esté cancelada, una factura en borrador o abierta, o un pago pendiente. Hasta octubre de 2026 esta ruta cerraba la cuenta (y aceptaba `?reason=`) — cerrar es `POST /v1/billing_accounts/{id}/close`, reversible.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta de cobro eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## billing-accounts close

Cerrar cuenta de cobro

```text
kobana billing billing-accounts close [ID] [OPCIONES]
```

`POST /v1/billing_accounts/{id}/close`

Cierra la cuenta de cobro. El motivo (`reason`) se registra en el audit log.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `reason` | `string` | no | Motivo de la operación (registrado en el audit log). |

Otros campos y objetos anidados: `kobana schema billing.billing-accounts.close`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta de cobro cerrada. |
| `404` | Recurso no encontrado en la organización actual. |

## billing-accounts credits

Listar créditos de la cuenta

```text
kobana billing billing-accounts credits [ID] [OPCIONES]
```

`GET /v1/billing_accounts/{id}/credits`

Devuelve una lista paginada de los créditos de la cuenta de cobro.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `include_expired` | `string` | no | Incluye créditos expirados cuando es `true`. Valores: `true`, `false`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de créditos. |
| `404` | Recurso no encontrado en la organización actual. |

## billing-accounts entitlements

Listar productos contratados de la cuenta

```text
kobana billing billing-accounts entitlements [ID] [OPCIONES]
```

`GET /v1/billing_accounts/{id}/entitlements`

Devuelve una lista reducida de los productos a los que tiene derecho la cuenta de cobro, según las suscripciones en prueba, activas o en mora (gracia de cobro). Solo se devuelven los productos padre; los productos hijos (adicionales) se agrupan bajo el padre y se omiten. Ideal para habilitar o bloquear el uso de un producto en una app externa.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `product_group_id` | `string` | no | Filtra los productos contratados por familia (grupo de productos), por ID. |
| `product_group_slug` | `string` | no | Filtra los productos contratados por familia (grupo de productos), por slug. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de productos contratados. |
| `404` | Recurso no encontrado en la organización actual. |

## billing-accounts invoices

Listar facturas de la cuenta

```text
kobana billing billing-accounts invoices [ID] [OPCIONES]
```

`GET /v1/billing_accounts/{id}/invoices`

Devuelve una lista paginada de las facturas de la cuenta de cobro.

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
| `200` | Lista paginada de facturas. |
| `404` | Recurso no encontrado en la organización actual. |

## billing-accounts reactivate

Reactivar cuenta de cobro

```text
kobana billing billing-accounts reactivate [ID] [OPCIONES]
```

`POST /v1/billing_accounts/{id}/reactivate`

Reactiva una cuenta de cobro suspendida.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta de cobro reactivada. |
| `404` | Recurso no encontrado en la organización actual. |

## billing-accounts subscriptions

Listar suscripciones de la cuenta

```text
kobana billing billing-accounts subscriptions [ID] [OPCIONES]
```

`GET /v1/billing_accounts/{id}/subscriptions`

Devuelve una lista paginada de las suscripciones de la cuenta de cobro.

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
| `200` | Lista paginada de suscripciones. |
| `404` | Recurso no encontrado en la organización actual. |

## billing-accounts suspend

Suspender cuenta de cobro

```text
kobana billing billing-accounts suspend [ID] [OPCIONES]
```

`POST /v1/billing_accounts/{id}/suspend`

Suspende la cuenta. El motivo (`reason`) se registra en el audit log.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `reason` | `string` | no | Motivo de la operación (registrado en el audit log). |

Otros campos y objetos anidados: `kobana schema billing.billing-accounts.suspend`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta de cobro suspendida. |
| `404` | Recurso no encontrado en la organización actual. |
