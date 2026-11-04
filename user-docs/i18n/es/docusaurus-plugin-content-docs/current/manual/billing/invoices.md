---
id: invoices
title: "kobana billing invoices"
sidebar_label: "invoices"
---

# kobana billing invoices


Comandos de `kobana billing invoices`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`invoices installments`](#invoices-installments) | `GET /v1/invoices/{id}/installments` | Listar parcelas de la factura |
| [`invoices installments create`](#invoices-installments-create) | `POST /v1/invoices/{id}/installments` | Crear/actualizar cronograma de parcelas |
| [`invoices installments mark-paid`](#invoices-installments-mark-paid) | `POST /v1/invoices/{id}/installments/{installment_id}/mark-paid` | Marcar parcela como pagada |
| [`invoices installments pay`](#invoices-installments-pay) | `POST /v1/invoices/{id}/installments/{installment_id}/pay` | Pagar parcela |
| [`invoices pending-nfe`](#invoices-pending-nfe) | `GET /v1/invoices/pending-nfe` | Listar facturas con NF-e pendiente |
| [`invoices pending-payment`](#invoices-pending-payment) | `GET /v1/invoices/pending-payment` | Listar facturas con pago pendiente |
| [`invoices list`](#invoices-list) | `GET /v1/invoices` | Listar facturas |
| [`invoices create`](#invoices-create) | `POST /v1/invoices` | Crear factura |
| [`invoices update`](#invoices-update) | `PUT /v1/invoices` | Crear o actualizar factura |
| [`invoices get`](#invoices-get) | `GET /v1/invoices/{id}` | Obtener factura |
| [`invoices delete`](#invoices-delete) | `DELETE /v1/invoices/{id}` | Eliminar factura |
| [`invoices apply-credits`](#invoices-apply-credits) | `POST /v1/invoices/{id}/apply-credits` | Aplicar créditos a la factura |
| [`invoices finalize`](#invoices-finalize) | `POST /v1/invoices/{id}/finalize` | Finalizar factura |
| [`invoices issue-nfe`](#invoices-issue-nfe) | `POST /v1/invoices/{id}/issue-nfe` | Emitir NF-e para la factura |
| [`invoices pay`](#invoices-pay) | `POST /v1/invoices/{id}/pay` | Pagar factura |
| [`invoices payment-methods`](#invoices-payment-methods) | `GET /v1/invoices/{id}/payment-methods` | Listar métodos de pago disponibles |
| [`invoices pdf`](#invoices-pdf) | `GET /v1/invoices/{id}/pdf` | Descargar PDF de la factura |
| [`invoices remove-credits`](#invoices-remove-credits) | `POST /v1/invoices/{id}/remove-credits` | Eliminar créditos aplicados |
| [`invoices send-reminder`](#invoices-send-reminder) | `POST /v1/invoices/{id}/send-reminder` | Enviar recordatorio de cobro |
| [`invoices undo-payment`](#invoices-undo-payment) | `POST /v1/invoices/{id}/undo-payment` | Deshacer pago de la factura |
| [`invoices void`](#invoices-void) | `POST /v1/invoices/{id}/void` | Cancelar (void) factura |

## invoices installments

Listar parcelas de la factura

```text
kobana billing invoices installments [ID] [OPCIONES] [COMANDO]
```

`GET /v1/invoices/{id}/installments`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de parcelas. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices installments create

Crear/actualizar cronograma de parcelas

```text
kobana billing invoices installments create [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/installments`

Crea o reemplaza el cronograma de parcelas de la factura. Mínimo de 2 parcelas.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `installments` | `array<object>` | sí | Lista de parcelas. Mínimo 2. |

Otros campos y objetos anidados: `kobana schema billing.invoices.installments.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cronograma de parcelas creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## invoices installments mark-paid

Marcar parcela como pagada

```text
kobana billing invoices installments mark-paid [ID] [INSTALLMENT_ID] [OPCIONES]
```

`POST /v1/invoices/{id}/installments/{installment_id}/mark-paid`

Marca manualmente una parcela como pagada (reconciliación, sin llamada al gateway).

**Argumentos de la ruta:** `id`, `installment_id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Parcela marcada como pagada. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices installments pay

Pagar parcela

```text
kobana billing invoices installments pay [ID] [INSTALLMENT_ID] [OPCIONES]
```

`POST /v1/invoices/{id}/installments/{installment_id}/pay`

Procesa el pago de una parcela vía gateway. La parcela anterior debe estar pagada.

**Argumentos de la ruta:** `id`, `installment_id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `payment_method` | `string` | sí | Cuando se informa, dispara el gateway (`pix`, `bank_slip` o `card`) o crea un cobro por transferencia (`bank_transfer`, exige `bank_account_id` o `pix_key_id`). Omite para marcar como pagada manualmente. Valores: `pix`, `bank_slip`, `card`, `bank_transfer`. |
| `payment_method_id` | `string (uuid)` | no | UUID de un método de pago guardado (ej.: tarjeta tokenizada). |
| `card` | `object` | no | Datos de la tarjeta cuando `payment_method = card` y no hay `payment_method_id`. |
| `bank_account_id` | `string (uuid)` | no | Con `payment_method = bank_transfer`: UUID de la cuenta bancaria de la organización a la que el cliente debe transferir (vea `/bank_accounts`). Informe este o `pix_key_id`, nunca los dos. |
| `pix_key_id` | `string (uuid)` | no | Con `payment_method = bank_transfer`: UUID de la clave Pix de la organización a la que el cliente debe transferir (vea `/pix_keys`). Informe este o `bank_account_id`, nunca los dos. |

Otros campos y objetos anidados: `kobana schema billing.invoices.installments.pay`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado del pago de la parcela. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## invoices pending-nfe

Listar facturas con NF-e pendiente

```text
kobana billing invoices pending-nfe [OPCIONES]
```

`GET /v1/invoices/pending-nfe`

Devuelve facturas pagadas pero con emisión de NF-e pendiente — sin NF-e, o con NF-e en estado `pending`, `processing` o `error`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Filtra por `billing_account_id`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de facturas con NF-e pendiente. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices pending-payment

Listar facturas con pago pendiente

```text
kobana billing invoices pending-payment [OPCIONES]
```

`GET /v1/invoices/pending-payment`

Devuelve facturas finalizadas (`status = open`) cuya `due_date` ya pasó.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Filtra por `billing_account_id`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de facturas con pago pendiente. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices list

Listar facturas

```text
kobana billing invoices list [OPCIONES]
```

`GET /v1/invoices`

Devuelve una lista paginada de las facturas de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra por estado: `draft`, `open`, `paid`, `void` o `uncollectible`. Valores: `draft`, `open`, `paid`, `void`, `uncollectible`. |
| `billing_account_id` | `string` | no | Filtra por `billing_account_id`. |
| `company_id` | `string` | no | Filtra por `company_id` emisor. |
| `subscription_id` | `string` | no | Filtra por `subscription_id` de origen. |
| `due_date_from` | `string` | no | Filtra por `due_date` inicial (inclusive). |
| `due_date_to` | `string` | no | Filtra por `due_date` final (inclusive). |
| `date_from` | `string` | no | Filtra por `created_at` inicial (inclusive). |
| `date_to` | `string` | no | Filtra por `created_at` final (inclusive). |
| `search` | `string` | no | Búsqueda textual libre. |
| `total_min` | `number,null` | no | Filtra por valor total mínimo, en BRL (float, hasta 4 decimales). |
| `total_max` | `number,null` | no | Filtra por valor total máximo, en BRL (float, hasta 4 decimales). |
| `document_number` | `string` | no | Filtra por CPF/CNPJ del cliente. |
| `without_payments` | `string` | no | Cuando es `true`, devuelve solo facturas sin pagos asociados. Valores: `true`, `false`. |
| `plan_filter` | `string` | no | Filtra por origen: `all`, `with_plan` (con suscripción) o `without_plan` (suelta). Valores: `all`, `with_plan`, `without_plan`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de facturas. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices create

Crear factura

```text
kobana billing invoices create [OPCIONES]
```

`POST /v1/invoices`

Crea una factura para una cuenta de cobro. Acepta ítems de línea, parcelas opcionales (`installments`, mínimo 2) y política de emisión de NF-e.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sí | UUID de la cuenta de cobro destino. **Obligatorio**. |
| `line_items` | `array<object>` | sí | Ítems de línea. Mínimo 1. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora. Por defecto: empresa por defecto de la organización. |
| `subscription_id` | `string (uuid)` | no | UUID de la suscripción de origen. |
| `product_group_id` | `string (uuid)` | no | UUID del grupo de productos. |
| `collection_method` | `string` | no | Método de cobro. Por defecto: `manual_invoice`. Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `due_date` | `string` | no | Fecha de vencimiento. Por defecto: hoy + `payment_terms_days` de la cuenta. |
| `description` | `string` | no | Descripción libre mostrada al inicio de la factura. |
| `footer` | `string` | no | Pie libre mostrado al final de la factura. |
| `discount` | `number` | no | Descuento fijo en BRL. |
| `discount_description` | `string` | no | Descripción del descuento (ej.: "Descuento por pago anual (20%)"). |
| `installments` | `array<object>` | no | Cronograma de parcelas (mínimo 2). Cada ítem tiene `amount` y `due_date`. |
| `nfe_issuance_policy` | `string` | no | Política de emisión de NF-e para facturas en cuotas. Valores: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `period_start` | `string` | no | Inicio del período de competencia (autocompletado en facturas de suscripción). |
| `period_end` | `string` | no | Fin del período de competencia (autocompletado en facturas de suscripción). |
| `custom_metadata` | `object` | no | Campos personalizados (validados contra el schema de la organización). |

Otros campos y objetos anidados: `kobana schema billing.invoices.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Factura creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## invoices update

Crear o actualizar factura

```text
kobana billing invoices update [OPCIONES]
```

`PUT /v1/invoices`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro destino. **Obligatorio**. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora. Por defecto: empresa por defecto de la organización. |
| `subscription_id` | `string (uuid)` | no | UUID de la suscripción de origen. |
| `product_group_id` | `string (uuid)` | no | UUID del grupo de productos. |
| `collection_method` | `string` | no | Método de cobro. Por defecto: `manual_invoice`. Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `due_date` | `string` | no | Fecha de vencimiento. Por defecto: hoy + `payment_terms_days` de la cuenta. |
| `description` | `string` | no | Descripción libre mostrada al inicio de la factura. |
| `footer` | `string` | no | Pie libre mostrado al final de la factura. |
| `line_items` | `array<object>` | no | Ítems de línea. Mínimo 1. |
| `discount` | `number` | no | Descuento fijo en BRL. |
| `discount_description` | `string` | no | Descripción del descuento (ej.: "Descuento por pago anual (20%)"). |
| `installments` | `array<object>` | no | Cronograma de parcelas (mínimo 2). Cada ítem tiene `amount` y `due_date`. |
| `nfe_issuance_policy` | `string` | no | Política de emisión de NF-e para facturas en cuotas. Valores: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `period_start` | `string` | no | Inicio del período de competencia (autocompletado en facturas de suscripción). |
| `period_end` | `string` | no | Fin del período de competencia (autocompletado en facturas de suscripción). |
| `custom_metadata` | `object` | no | Campos personalizados (validados contra el schema de la organización). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.invoices.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## invoices get

Obtener factura

```text
kobana billing invoices get [ID] [OPCIONES]
```

`GET /v1/invoices/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la factura. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices delete

Eliminar factura

```text
kobana billing invoices delete [ID] [OPCIONES]
```

`DELETE /v1/invoices/{id}`

Elimina una factura `draft`, `open` o `void` (los créditos aplicados a una factura `open` vuelven al saldo del cliente). Una factura `paid` o `uncollectible` no puede eliminarse — deshaga el pago o anúlela antes.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Factura eliminada. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices apply-credits

Aplicar créditos a la factura

```text
kobana billing invoices apply-credits [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/apply-credits`

Aplica créditos de la cuenta de cobro en una factura abierta (`status = open`).

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `credit_ids` | `array<string (uuid)>` | sí | UUIDs de los créditos a aplicar (mínimo 1). |
| `amount` | `number` | no | Valor a aplicar en BRL. Por defecto: cubre toda la factura. |

Otros campos y objetos anidados: `kobana schema billing.invoices.apply-credits`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Créditos aplicados. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## invoices finalize

Finalizar factura

```text
kobana billing invoices finalize [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/finalize`

Finaliza una factura en borrador (`draft` → `open`).

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Factura finalizada. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices issue-nfe

Emitir NF-e para la factura

```text
kobana billing invoices issue-nfe [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/issue-nfe`

Dispara la emisión de la NF-e (Factura Electrónica) para la factura.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | NF-e en cola/emitida. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices pay

Pagar factura

```text
kobana billing invoices pay [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/pay`

Dos modos: (a) cuando se informa `payment_method`, crea un pago vía gateway (`pix`, `bank_slip` o `card`) y devuelve los artefactos del método (QR code, código de barras o estado de la tarjeta) — o, con `bank_transfer`, un cobro por transferencia a la cuenta bancaria (`bank_account_id`) o clave Pix (`pix_key_id`) de la organización, sin gateway, devolviendo los datos en `bank_transfer` y quedando pendiente hasta la baja manual; (b) cuando se omite, marca la factura como pagada manualmente (reconciliación).

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `payment_method` | `string` | no | Cuando se informa, dispara el gateway (`pix`, `bank_slip` o `card`) o crea un cobro por transferencia (`bank_transfer`, exige `bank_account_id` o `pix_key_id`). Omite para marcar como pagada manualmente. Valores: `pix`, `bank_slip`, `card`, `bank_transfer`. |
| `payment_method_id` | `string (uuid)` | no | UUID de un método de pago guardado (ej.: tarjeta tokenizada). |
| `card` | `object` | no | Datos de la tarjeta cuando `payment_method = card` y no hay `payment_method_id`. |
| `bank_account_id` | `string (uuid)` | no | Con `payment_method = bank_transfer`: UUID de la cuenta bancaria de la organización a la que el cliente debe transferir (vea `/bank_accounts`). Informe este o `pix_key_id`, nunca los dos. |
| `pix_key_id` | `string (uuid)` | no | Con `payment_method = bank_transfer`: UUID de la clave Pix de la organización a la que el cliente debe transferir (vea `/pix_keys`). Informe este o `bank_account_id`, nunca los dos. |
| `amount` | `number` | no | Valor a cobrar en BRL. Por defecto: valor restante de la factura. Obligatorio cuando se informa `installment_ids`, y debe ser igual a la suma del `net` de las parcelas seleccionadas. |
| `installment_ids` | `array<string (uuid)>` | no | UUIDs de las parcelas que el pago cubre (facturas en cuotas). |

Otros campos y objetos anidados: `kobana schema billing.invoices.pay`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado del pago o factura marcada como pagada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## invoices payment-methods

Listar métodos de pago disponibles

```text
kobana billing invoices payment-methods [ID] [OPCIONES]
```

`GET /v1/invoices/{id}/payment-methods`

Lista los métodos de pago disponibles para pagar esta factura, según los gateways activos de la organización y la configuración del portal.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Métodos de pago disponibles. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices pdf

Descargar PDF de la factura

```text
kobana billing invoices pdf [ID] [OPCIONES]
```

`GET /v1/invoices/{id}/pdf`

Genera y descarga el PDF de la factura como adjunto binario (`application/pdf`).

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | PDF de la factura. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices remove-credits

Eliminar créditos aplicados

```text
kobana billing invoices remove-credits [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/remove-credits`

Elimina todos los créditos aplicados a la factura, devolviéndolos a la cuenta de cobro. Solo funciona en facturas abiertas (`status = open`).

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Créditos eliminados. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## invoices send-reminder

Enviar recordatorio de cobro

```text
kobana billing invoices send-reminder [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/send-reminder`

Envía por correo un recordatorio de cobro de la factura al `billing_email` de la cuenta.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Recordatorio enviado. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices undo-payment

Deshacer pago de la factura

```text
kobana billing invoices undo-payment [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/undo-payment`

Revierte el pago de la factura (`paid` → `open`). El motivo (`reason`) se registra en el audit log.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `reason` | `string` | no | Motivo de la reversión (registrado en el audit log). |

Otros campos y objetos anidados: `kobana schema billing.invoices.undo-payment`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pago deshecho. |
| `404` | Recurso no encontrado en la organización actual. |

## invoices void

Cancelar (void) factura

```text
kobana billing invoices void [ID] [OPCIONES]
```

`POST /v1/invoices/{id}/void`

Cancela una factura abierta (`open` → `void`). El motivo (`reason`) se registra en el audit log.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `reason` | `string` | no | Motivo de la cancelación (registrado en el audit log). |

Otros campos y objetos anidados: `kobana schema billing.invoices.void`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Factura cancelada. |
| `404` | Recurso no encontrado en la organización actual. |
