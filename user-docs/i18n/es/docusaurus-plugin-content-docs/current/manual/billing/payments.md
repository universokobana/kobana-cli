---
id: payments
title: "kobana billing payments"
sidebar_label: "payments"
---

# kobana billing payments


Comandos de `kobana billing payments`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`payments list`](#payments-list) | `GET /v1/payments` | Listar pagos |
| [`payments create`](#payments-create) | `POST /v1/payments` | Procesar pago |
| [`payments bulk-update-invoice`](#payments-bulk-update-invoice) | `POST /v1/payments/bulk-update-invoice` | Reasignar pagos a otra factura |
| [`payments get`](#payments-get) | `GET /v1/payments/{id}` | Obtener pago |
| [`payments delete`](#payments-delete) | `DELETE /v1/payments/{id}` | Eliminar pago |
| [`payments cancel`](#payments-cancel) | `POST /v1/payments/{id}/cancel` | Cancelar pago |
| [`payments change-card`](#payments-change-card) | `POST /v1/payments/{id}/change-card` | Cambiar tarjeta del pago |
| [`payments change-due-date`](#payments-change-due-date) | `POST /v1/payments/{id}/change-due-date` | Cambiar vencimiento del boleto |
| [`payments refund`](#payments-refund) | `POST /v1/payments/{id}/refund` | Reembolsar pago |
| [`payments retry`](#payments-retry) | `POST /v1/payments/{id}/retry` | Reintentar pago |
| [`payments send-email`](#payments-send-email) | `POST /v1/payments/{id}/send-email` | Enviar boleto por correo |
| [`payments sync`](#payments-sync) | `POST /v1/payments/{id}/sync` | Sincronizar pago con el gateway |

## payments list

Listar pagos

```text
kobana billing payments list [OPCIONES]
```

`GET /v1/payments`

Devuelve una lista paginada de los pagos de la organización actual.

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
| `status` | `string` | no | Filtra por estado del pago. Valores: `pending`, `processing`, `succeeded`, `failed`, `canceled`, `partial`, `refunded`, `partially_refunded`. |
| `invoice_id` | `string` | no | Filtra por los pagos de una factura específica. |
| `billing_account_id` | `string` | no | Filtra por los pagos de una cuenta de cobro. |
| `company_id` | `string` | no | Filtra por los pagos de una empresa específica. |
| `gateway_id` | `string` | no | Filtra por los pagos procesados por un gateway específico. |
| `payment_method_id` | `string` | no | Filtra por los pagos de un medio de pago específico. |
| `payment_method_type` | `string` | no | Filtra por medio de pago: `card`, `bank_slip`, `pix` o `bank_transfer`. Valores: `card`, `bank_slip`, `pix`, `bank_transfer`. |
| `external_id` | `string` | no | Filtra por ID externo (campo libre para identificadores del sistema legado). |
| `gateway_payment_id` | `string` | no | Filtra por el identificador del pago en el gateway upstream. |
| `invoice_filter` | `string` | no | Filtra por presencia de factura: `all`, `with_invoice` o `without_invoice`. Valores: `all`, `with_invoice`, `without_invoice`. |
| `search` | `string` | no | Búsqueda textual libre. |
| `amount_min` | `number,null` | no | Monto mínimo cobrado (en BRL). |
| `amount_max` | `number,null` | no | Monto máximo cobrado (en BRL). |
| `paid_amount_min` | `number,null` | no | Monto mínimo efectivamente recibido (en BRL). |
| `paid_amount_max` | `number,null` | no | Monto máximo efectivamente recibido (en BRL). |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `paid_after` | `string` | no | Fecha inicial de confirmación del pago — formato ISO 8601. |
| `paid_before` | `string` | no | Fecha final de confirmación del pago — formato ISO 8601. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de pagos. |
| `404` | Recurso no encontrado en la organización actual. |

## payments create

Procesar pago

```text
kobana billing payments create [OPCIONES]
```

`POST /v1/payments`

Procesa un pago para una factura existente. Acepta un `payment_method_id` previamente registrado **o** un bloque `payment_method` con datos de tarjeta/PIX/boleto. Cuando se omite `amount`, cobra el saldo abierto de la factura.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `invoice_id` | `string (uuid)` | sí | UUID de la factura a pagar. |
| `payment_method_id` | `string (uuid)` | no | UUID de un medio de pago previamente registrado para la cuenta. |
| `payment_method` | `object` | no |  |
| `amount` | `number` | no | Monto a cobrar en BRL (float, hasta 4 decimales). Por defecto: saldo abierto de la factura. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos libres definidos por el cliente para correlación interna. |

Otros campos y objetos anidados: `kobana schema billing.payments.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Pago creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payments bulk-update-invoice

Reasignar pagos a otra factura

```text
kobana billing payments bulk-update-invoice [OPCIONES]
```

`POST /v1/payments/bulk-update-invoice`

Actualiza en bloque la factura asociada a los pagos indicados. Útil para corregir vínculos cuando los pagos llegaron apuntando a la factura equivocada. Un pago que ya liquida más de una factura no se mueve: vuelve en `failed` (con `id` y `error`), contado por `failed_count`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `payment_ids` | `array<string (uuid)>` | sí | Lista de UUIDs de los pagos a actualizar. |
| `invoice_id` | `string (uuid)` | sí | UUID de la factura de destino. |

Otros campos y objetos anidados: `kobana schema billing.payments.bulk-update-invoice`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pagos reasignados. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payments get

Obtener pago

```text
kobana billing payments get [ID] [OPCIONES]
```

`GET /v1/payments/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del pago. |
| `404` | Recurso no encontrado en la organización actual. |

## payments delete

Eliminar pago

```text
kobana billing payments delete [ID] [OPCIONES]
```

`DELETE /v1/payments/{id}`

Elimina un pago. Los pagos ya confirmados no pueden eliminarse — use `cancel` o `refund`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pago eliminado. |
| `404` | Recurso no encontrado en la organización actual. |

## payments cancel

Cancelar pago

```text
kobana billing payments cancel [ID] [OPCIONES]
```

`POST /v1/payments/{id}/cancel`

Cancela un pago abierto. Cuando `cancel_on_gateway` es `true` (por defecto), la cancelación se propaga al gateway.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `reason` | `string` | no | Motivo de la cancelación (registrado en el audit log). |
| `cancel_on_gateway` | `boolean` | no | Cuando es `true` (por defecto), también cancela en el gateway upstream. |

Otros campos y objetos anidados: `kobana schema billing.payments.cancel`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pago cancelado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payments change-card

Cambiar tarjeta del pago

```text
kobana billing payments change-card [ID] [OPCIONES]
```

`POST /v1/payments/{id}/change-card`

Reemplaza la tarjeta de crédito de un pago fallido procesado por Pagar.me. La nueva tarjeta debe estar registrada previamente en `payment_methods`.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `credit_card_id` | `string (uuid)` | sí | UUID de la tarjeta de crédito activa de la cuenta de cobro. |

Otros campos y objetos anidados: `kobana schema billing.payments.change-card`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Tarjeta cambiada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payments change-due-date

Cambiar vencimiento del boleto

```text
kobana billing payments change-due-date [ID] [OPCIONES]
```

`POST /v1/payments/{id}/change-due-date`

Cambia el vencimiento de un pago en boleto pendiente emitido vía gateway Kobana. La nueva fecha debe ser futura.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `due_date` | `string` | sí | Nueva fecha de vencimiento en formato `YYYY-MM-DD`. |

Otros campos y objetos anidados: `kobana schema billing.payments.change-due-date`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Vencimiento actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payments refund

Reembolsar pago

```text
kobana billing payments refund [ID] [OPCIONES]
```

`POST /v1/payments/{id}/refund`

Reembolsa un pago exitoso. Sin `amount`, reembolsa el monto total; con un monto parcial, emite un reembolso parcial.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | no | Monto a reembolsar en BRL (float, hasta 4 decimales). Por defecto: monto total del pago. |
| `reason` | `string` | no | Motivo del reembolso (registrado en el audit log). |

Otros campos y objetos anidados: `kobana schema billing.payments.refund`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pago reembolsado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payments retry

Reintentar pago

```text
kobana billing payments retry [ID] [OPCIONES]
```

`POST /v1/payments/{id}/retry`

Reprocesa un pago fallido. Opcionalmente cambia a una tarjeta nueva (vía `card`) o a un medio de pago guardado (`payment_method_id`).

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `card` | `object` | no |  |
| `payment_method_id` | `string (uuid)` | no | UUID de un medio de pago guardado para usar en el reintento. |

Otros campos y objetos anidados: `kobana schema billing.payments.retry`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pago reprocesado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payments send-email

Enviar boleto por correo

```text
kobana billing payments send-email [ID] [OPCIONES]
```

`POST /v1/payments/{id}/send-email`

Envía el enlace del boleto por correo al destinatario indicado. Disponible solo para pagos en boleto pendientes.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string (email)` | sí | Dirección de correo del destinatario. |
| `message` | `string` | no | Mensaje opcional incluido en el cuerpo del correo. |

Otros campos y objetos anidados: `kobana schema billing.payments.send-email`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Correo enviado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## payments sync

Sincronizar pago con el gateway

```text
kobana billing payments sync [ID] [OPCIONES]
```

`POST /v1/payments/{id}/sync`

Sincroniza el estado del pago con el gateway upstream (PIX/boleto Kobana o reembolso Pagar.me). Devuelve el pago actualizado.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pago sincronizado. |
| `404` | Recurso no encontrado en la organización actual. |
