---
id: payment-methods
title: "kobana billing payment-methods"
sidebar_label: "payment-methods"
---

# kobana billing payment-methods


Comandos de `kobana billing payment-methods`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`payment-methods list`](#payment-methods-list) | `GET /v1/payment_methods` | Listar métodos de pago |
| [`payment-methods create`](#payment-methods-create) | `POST /v1/payment_methods` | Crear método de pago |
| [`payment-methods update`](#payment-methods-update) | `PUT /v1/payment_methods` | Crear o actualizar forma de pago |
| [`payment-methods get`](#payment-methods-get) | `GET /v1/payment_methods/{id}` | Obtener método de pago |
| [`payment-methods delete`](#payment-methods-delete) | `DELETE /v1/payment_methods/{id}` | Eliminar método de pago |
| [`payment-methods set-default`](#payment-methods-set-default) | `POST /v1/payment_methods/{id}/set-default` | Definir método como predeterminado |

## payment-methods list

Listar métodos de pago

```text
kobana billing payment-methods list [OPCIONES]
```

`GET /v1/payment_methods`

Devuelve una lista paginada de los métodos de pago de la organización actual. Filtros opcionales: `billing_account_id`, `type`, `status`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Filtra por UUID de la cuenta de cobro. |
| `type` | `string` | no | Filtra por tipo: `card`, `bank_slip`, `pix` o `bank_transfer`. Valores: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `status` | `string` | no | Filtra por estado: `active`, `expired` o `failed`. Valores: `active`, `expired`, `failed`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de métodos de pago. |
| `404` | Recurso no encontrado en la organización actual. |

## payment-methods create

Crear método de pago

```text
kobana billing payment-methods create [OPCIONES]
```

`POST /v1/payment_methods`

Crea un método de pago asociado a una cuenta de cobro. Para `type=card`, envía el bloque `card` con el `card_token` generado por la tokenización del lado del cliente. Para `type=bank_transfer`, envía el bloque `bank_account`. Para `bank_slip` y `pix`, basta con el `type` — el gateway predeterminado se resuelve por la organización.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sí | UUID de la cuenta de cobro propietaria del método de pago. |
| `type` | `string` | sí | Tipo del método: `card`, `bank_slip`, `pix` o `bank_transfer`. Valores: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `is_default` | `boolean` | no | Cuando es `true`, marca este método como predeterminado de la cuenta (desmarca los anteriores). El primer método creado para una cuenta también se vuelve predeterminado automáticamente. |
| `card` | `object` | no |  |
| `bank_account` | `object` | no |  |
| `billing_details` | `object` | no |  |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos personalizados en formato libre (`object`). |

Otros campos y objetos anidados: `kobana schema billing.payment-methods.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Método de pago creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payment-methods update

Crear o actualizar forma de pago

```text
kobana billing payment-methods update [OPCIONES]
```

`PUT /v1/payment_methods`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro propietaria del método de pago. |
| `type` | `string` | no | Tipo del método: `card`, `bank_slip`, `pix` o `bank_transfer`. Valores: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `is_default` | `boolean` | no | Cuando es `true`, marca este método como predeterminado de la cuenta (desmarca los anteriores). El primer método creado para una cuenta también se vuelve predeterminado automáticamente. |
| `card` | `object` | no |  |
| `bank_account` | `object` | no |  |
| `billing_details` | `object` | no |  |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos personalizados en formato libre (`object`). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.payment-methods.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payment-methods get

Obtener método de pago

```text
kobana billing payment-methods get [ID] [OPCIONES]
```

`GET /v1/payment_methods/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del método de pago. |
| `404` | Recurso no encontrado en la organización actual. |

## payment-methods delete

Eliminar método de pago

```text
kobana billing payment-methods delete [ID] [OPCIONES]
```

`DELETE /v1/payment_methods/{id}`

Elimina el método de pago (soft delete: `deleted_at`, y el estado pasa a `removed` donde todavía aparece por relación — en los pagos que pagó). Exige el nivel de acceso `delete`. Falla cuando hay pagos pendientes o en procesamiento. Si era el predeterminado, otro método activo se promueve automáticamente.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Método de pago eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payment-methods set-default

Definir método como predeterminado

```text
kobana billing payment-methods set-default [ID] [OPCIONES]
```

`POST /v1/payment_methods/{id}/set-default`

Marca el método como predeterminado de la cuenta de cobro y desmarca los anteriores. Falla cuando el método no está `active`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Método de pago definido como predeterminado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
