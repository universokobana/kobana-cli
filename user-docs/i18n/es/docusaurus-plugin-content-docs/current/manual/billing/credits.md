---
id: credits
title: "kobana billing credits"
sidebar_label: "credits"
---

# kobana billing credits


Comandos de `kobana billing credits`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`credits list`](#credits-list) | `GET /v1/credits` | Listar créditos |
| [`credits create`](#credits-create) | `POST /v1/credits` | Agregar crédito |
| [`credits update`](#credits-update) | `PUT /v1/credits` | Crear o actualizar crédito |
| [`credits get`](#credits-get) | `GET /v1/credits/{id}` | Obtener crédito |
| [`credits delete`](#credits-delete) | `DELETE /v1/credits/{id}` | Eliminar crédito |

## credits list

Listar créditos

```text
kobana billing credits list [OPCIONES]
```

`GET /v1/credits`

Devuelve una lista paginada de los créditos de la organización actual. Por defecto, los créditos expirados se omiten — usa `usage` o `include_expired` para incluirlos.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `person_id` | `string` | no | Filtra por UUID de la persona asociada. |
| `person_document_number` | `string` | no | Filtra por CPF/CNPJ de la persona (coincidencia exacta). |
| `person_name_contains` | `string` | no | Filtra por nombre de la persona (búsqueda parcial, sin distinción de mayúsculas). |
| `billing_account_id` | `string` | no | Filtra créditos por cuenta de cobro. |
| `company_id` | `string` | no | Filtra créditos por la `company` vinculada. |
| `type` | `string` | no | Filtra por tipo de crédito: `promotional`, `adjustment`, `refund` o `manual`. Valores: `promotional`, `adjustment`, `refund`, `manual`. |
| `usage` | `string` | no | Filtra por estado de uso — `available` (saldo restante y no expirado), `used` (consumido por completo) o `expired` (vencido). Valores: `available`, `used`, `expired`. |
| `include_expired` | `string` | no | Incluye créditos expirados cuando es `true`. Se ignora si se informa `usage`. Valores: `true`, `false`. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `amount_min` | `number,null` | no | Monto mínimo del crédito en BRL (inclusivo). |
| `amount_max` | `number,null` | no | Monto máximo del crédito en BRL (inclusivo). |
| `amount_remaining_min` | `number,null` | no | Saldo restante mínimo en BRL (inclusivo). |
| `amount_remaining_max` | `number,null` | no | Saldo restante máximo en BRL (inclusivo). |
| `expires_after` | `string` | no | Límite inferior de `expires_at` (YYYY-MM-DD, inclusivo). |
| `expires_before` | `string` | no | Límite superior de `expires_at` (YYYY-MM-DD, inclusivo). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de créditos. |
| `404` | Recurso no encontrado en la organización actual. |

## credits create

Agregar crédito

```text
kobana billing credits create [OPCIONES]
```

`POST /v1/credits`

Agrega un crédito a una cuenta de cobro. El monto se acredita en `balance` de la `BillingAccount` y queda disponible para descontar facturas futuras. Cuando se informa `company_id`, el crédito solo puede aplicarse a facturas de la misma `company`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sí | UUID de la cuenta de cobro que recibirá el crédito. |
| `type` | `string` | sí | Tipo de crédito: `promotional`, `adjustment`, `refund` o `manual`. Valores: `promotional`, `adjustment`, `refund`, `manual`. |
| `amount` | `number` | sí | Monto del crédito en BRL (float, hasta 4 decimales). Debe ser un entero positivo. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no | UUID opcional de la `company` a la que está vinculado el crédito. Cuando se informa, restringe la aplicación del crédito a facturas de la misma `company`. |
| `description` | `string` | no | Descripción libre (hasta 500 caracteres) — se muestra en el extracto y en el audit log. |
| `expires_at` | `string (date-time)` | no | Fecha y hora de expiración (ISO8601). Después de esa fecha el saldo restante se descarta. |
| `custom_metadata` | `object` | no | Metadatos personalizados validados contra el esquema de la organización. |

Otros campos y objetos anidados: `kobana schema billing.credits.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Crédito creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## credits update

Crear o actualizar crédito

```text
kobana billing credits update [OPCIONES]
```

`PUT /v1/credits`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro que recibirá el crédito. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no | UUID opcional de la `company` a la que está vinculado el crédito. Cuando se informa, restringe la aplicación del crédito a facturas de la misma `company`. |
| `type` | `string` | no | Tipo de crédito: `promotional`, `adjustment`, `refund` o `manual`. Valores: `promotional`, `adjustment`, `refund`, `manual`. |
| `amount` | `number` | no | Monto del crédito en BRL (float, hasta 4 decimales). Debe ser un entero positivo. |
| `description` | `string` | no | Descripción libre (hasta 500 caracteres) — se muestra en el extracto y en el audit log. |
| `expires_at` | `string (date-time)` | no | Fecha y hora de expiración (ISO8601). Después de esa fecha el saldo restante se descarta. |
| `custom_metadata` | `object` | no | Metadatos personalizados validados contra el esquema de la organización. |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.credits.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## credits get

Obtener crédito

```text
kobana billing credits get [ID] [OPCIONES]
```

`GET /v1/credits/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del crédito. |
| `404` | Recurso no encontrado en la organización actual. |

## credits delete

Eliminar crédito

```text
kobana billing credits delete [ID] [OPCIONES]
```

`DELETE /v1/credits/{id}`

Elimina el crédito y revierte el saldo correspondiente en la cuenta de cobro. Solo se permite si no se ha consumido ningún centavo del crédito — de lo contrario la API responde `400`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Crédito eliminado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
