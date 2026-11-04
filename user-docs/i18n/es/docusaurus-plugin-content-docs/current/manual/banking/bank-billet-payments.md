---
id: bank-billet-payments
title: "kobana banking bank-billet-payments"
sidebar_label: "bank-billet-payments"
---

# kobana banking bank-billet-payments


Comandos de `kobana banking bank-billet-payments`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`bank-billet-payments list`](#bank-billet-payments-list) | `GET /v1/bank_billet_payments` | Listar Pagos de Boleto |
| [`bank-billet-payments create`](#bank-billet-payments-create) | `POST /v1/bank_billet_payments` | Realizar Pago de Boleto |
| [`bank-billet-payments get`](#bank-billet-payments-get) | `GET /v1/bank_billet_payments/{id}` | Información del Pago de Boleto |
| [`bank-billet-payments delete`](#bank-billet-payments-delete) | `DELETE /v1/bank_billet_payments/{id}` | Eliminar Pago de Boleto |

## bank-billet-payments list

Listar Pagos de Boleto

```text
kobana banking bank-billet-payments list [OPCIONES]
```

`GET /v1/bank_billet_payments`

Listar Pagos de Boleto.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `paid_from` | `string` | no | A partir de la Fecha de pago. |
| `paid_to` | `string` | no | Hasta la Fecha de pago. |
| `bank_billet_id` | `string` | no | ID del Boleto. |
| `bank_billet_account_id` | `string` | no | ID de la Cartera. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de pagos de boletos |

## bank-billet-payments create

Realizar Pago de Boleto

```text
kobana banking bank-billet-payments create [OPCIONES]
```

`POST /v1/bank_billet_payments`

Realizar Pago de Boleto.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `bank_billet_id` | `integer` | sí | ID del Boleto. |
| `bank_billet_account_id` | `integer` | sí | ID de la Cartera. |
| `paid_amount` | `number` | sí | Valor Pagado |
| `paid_at` | `string (date)` | sí | Fecha do Recepción. |
| `kind` | `integer` | sí | Tipo de Recepción: * `0` Directo al beneficiario/En manos * `1` Boleto (Confirmado por el banco vía archivo de devolución o API) * `2` Pix (Confirmado por el banco vía Pix) Valores: `0`, `1`, `2`. |
| `id` | `integer` | no | ID de la Recepción de Boleto |
| `discharge_id` | `integer` | no | ID del Retorno. |
| `amount` | `number` | no | Valor del Boleto |
| `paid_bank` | `string` | no | Banco de Recepción. |
| `paid_agency` | `string` | no | Agencia de Recepción (com dígito quando disponível). |
| `paid_agency_number` | `string` | no | Número da Agencia de Recepción (sem dígito). |
| `paid_agency_digit` | `string` | no | Dígito da Agencia de Recepción. |
| `bank_rate` | `number` | no | Tasa bancaria |
| `credit_at` | `string (date)` | no | Fecha de crédito. |
| `direct` | `boolean` | no | Recepción directo al beneficiario |
| `expire_at` | `string (date)` | no | Fecha de vencimiento. |
| `document_number` | `string` | no | Número del Documento. |
| `our_number` | `string` | no | Nuestro Número. |
| `created_via_api` | `boolean` | no | Enviado por la API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación. |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización. |
| `customer_person_name` | `string` | no | Nombre del cliente |
| `paid_by_pix` | `boolean` | no | Recepción vía Pix |
| `financial_provider_external_id` | `string` | no | ID de la recepción en la institución financiera |
| `line_parsed` | `string (json)` | no | Línea parseada con información proveniente del banco: Contenido de la devolución bancaria. Puede cambiar de acuerdo con el banco y el formato 240 o 400. Si alguno de los campos está en blanco es porque su banco no proporciona esa información. \| Valor \| Descripción \| \|---\|---\| \| paid_at \| Fecha de recepción \| \| paid_amount \| Valor pagado \| \| bank_rate \| Tasa del boleto pagada al banco \| \| credit_at \| Fecha de crédito \| \| credit_amount \| Valor del Crédito \| \| our_number \| Nuestro número \| \| processed_our_number_raw \| Nuestro número formateado \| \| discount \| Descuento \| \| billet_fine \| Incrementos(Multa/Intereses cobrados) \| \| other_fines \| Otros cargos adicionales \| \| other_credit \| Otros valores pagados \| \| value_rebate \| Otras tarifas \| \| error_code \| Código de error \| \| event_type \| Ocurrencia bancaria \| \| agency_number \| Agencia \| \| account_number \| Cuenta \| \| banco_recebedor \| Banco receptor \| \| agencia_recebedora \| Agencia receptora \| \| control_number \| Número de Control \| \| document_number \| Número del Documento \| |

Otros campos y objetos anidados: `kobana schema banking.bank-billet-payments.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Pago realizado con éxito |
| `422` | Datos inválidos |

## bank-billet-payments get

Información del Pago de Boleto

```text
kobana banking bank-billet-payments get [ID] [OPCIONES]
```

`GET /v1/bank_billet_payments/{id}`

Consulta información específicas de um pago de Boleto.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pago de boleto encontrado |
| `404` | Pago de boleto no encontrado |

## bank-billet-payments delete

Eliminar Pago de Boleto

```text
kobana banking bank-billet-payments delete [ID] [OPCIONES]
```

`DELETE /v1/bank_billet_payments/{id}`

Eliminar Pago de Boleto.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Pago de Boleto eliminado |
| `404` | Pago de Boleto no encontrado |
