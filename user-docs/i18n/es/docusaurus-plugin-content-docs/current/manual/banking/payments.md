---
id: payments
title: "kobana banking payments"
sidebar_label: "payments"
description: "Pagos (unificado)"
---

# kobana banking payments

Pagos (unificado)

Comandos de `kobana banking payments`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`payments list`](#payments-list) | `GET /v2/payments` | Listar Pagos |
| [`payments create`](#payments-create) | `POST /v2/payments` | Crear un Pago |
| [`payments barcode`](#payments-barcode) | `POST /v2/payments/barcode` | Crear un Pago por código de barras |
| [`payments get`](#payments-get) | `GET /v2/payments/{uid}` | Ver un Pago |
| [`payments approve`](#payments-approve) | `PUT /v2/payments/{uid}/approve` | Liberar (aprobar) un Pago |
| [`payments cancel`](#payments-cancel) | `PUT /v2/payments/{uid}/cancel` | Cancelar un Pago |

## payments list

Listar Pagos

```text
kobana banking payments list [OPCIONES]
```

`GET /v2/payments`

Lista todos los pagos de cualquier tipo (PIX, Boleto, DARF, Tributos, Cuentas). Admite filtros y paginación. Este es un endpoint unificado que devuelve pagos de todos los tipos a través de una sola consulta.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `kind` | `string` | no | Filtrar por tipo de pago Valores: `bank_billet`, `darf`, `utility`, `gru`, `pix`, `tax`. |
| `status` | `string` | no | Filtrar por estado Valores: `pending`, `scheduled`, `confirmed`, `rejected`, `canceled`. |
| `created_from` | `string` | no | Filtrar por fecha de creación inicial (formato: YYYY-MM-DD, inclusivo) |
| `created_to` | `string` | no | Filtrar por fecha de creación final (formato: YYYY-MM-DD, incluye el día completo) |
| `updated_from` | `string` | no | Filtrar por fecha de actualización inicial (formato: YYYY-MM-DD, inclusivo) |
| `updated_to` | `string` | no | Filtrar por fecha de actualización final (formato: YYYY-MM-DD, incluye el día completo) |
| `scheduled_from` | `string` | no | Filtrar por fecha de programación inicial (formato: YYYY-MM-DD) |
| `scheduled_to` | `string` | no | Filtrar por fecha de programación final (formato: YYYY-MM-DD) |
| `scheduled` | `boolean` | no | Filtrar solo pagos programados (true) o no programados (false) |
| `identifier` | `string` | no | Buscar por identificador (búsqueda parcial) |
| `external_id` | `string` | no | Filtrar por ID externo |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pagos encontrados |

## payments create

Crear un Pago

```text
kobana banking payments create [OPCIONES]
```

`POST /v2/payments`

Endpoint genérico para la creación de pagos de cualquier tipo. El tipo debe especificarse en el campo `kind`.

**Requisitos previos**

Para emitir un pago, es necesario tener una Cuenta Financiera, informada en `financial_account_uid`. La cuenta de pago que registra la operación se define en `payment_account_uid`; si se omite, se usa la más antigua vinculada a la cuenta financiera; si se especifica, debe pertenecer a `financial_account_uid`.

**Tipos Soportados**

**PIX**
Utilizar `kind: "pix"` y proporcionar la clave `pix`:
```json
{
  "kind": "pix",
  "financial_account_uid": "...",
  "amount": 100.0,
  "pix": {
    "qrcode": "00020126330014br.gov.bcb.pix..."
  }
}
```

**Boleto Bancario**
Utilizar `kind: "bank_billet"` y proporcionar la clave `bank_billet`:
```json
{
  "kind": "bank_billet",
  "financial_account_uid": "...",
  "amount": 200.0,
  "bank_billet": {
    "code": "81770000001-8 00005659970-7 41131079703-9 0014334-8"
  }
}
```

**DARF**
Utilizar `kind: "darf"` y proporcionar la clave `darf`:
```json
{
  "kind": "darf",
  "financial_account_uid": "...",
  "amount": 300.0,
  "darf": {
    "tax_code": "0107",
    "calculation_date": "2024-12-31",
    "expire_at": "2025-01-10"
  }
}
```

**Tributos (Impuesto)**
Utilizar `kind: "tax"` y proporcionar la clave `tax` con código de barras:
```json
{
  "kind": "tax",
  "financial_account_uid": "...",
  "amount": 400.0,
  "tax": {
    "kind": "iptu",
    "code": "81770000001-8 00005659970-7..."
  }
}
```

**Cuentas de Consumo (Servicio)**
Utilizar `kind: "utility"` y proporcionar la clave `utility`:
```json
{
  "kind": "utility",
  "financial_account_uid": "...",
  "amount": 500.0,
  "utility": {
    "code": "81770000001-8 00005659970-7..."
  }
}
```

**Eventos de Webhook**

El evento `payment.db.created` se dispara en esta operación.

> **Endpoint Unificado:** Este endpoint unifica la creación de todos los tipos de pago. Para crear pagos de un tipo específico, también puede usar los endpoints dedicados en `/api/v2/payment/{type}`.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.payments.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Pago de Cuenta de Consumo creado |
| `422` | Tributo con formato=code y código ausente |

## payments barcode

Crear un Pago por código de barras

```text
kobana banking payments barcode [OPCIONES]
```

`POST /v2/payments/barcode`

Crea un pago a partir solo del código de barras o de la línea digitable. El tipo del pago se resuelve por el propio código, siguiendo el layout FEBRABAN v08, y se devuelve en el campo `kind` de la respuesta:

| Código | `kind` |
|---|---|
| 47 posiciones, o 44 que no comienzan con `8` | `bank_billet` |
| comienza con `8`, segmento `2`, `3`, `4`, `6` o `7` | `utility` |
| comienza con `8`, segmento `1` o `5` | `tax`, con `tax.kind = "unknown"` y `tax.format = "code"` |

El código se rechaza con `422` cuando un dígito verificador no coincide (el general, en el boleto y en la recaudación, y los de campo o de bloque de la línea digitable), cuando usa el segmento `9`, `0` u `8`, cuando el tamaño no corresponde con el primer dígito (47 posiciones comenzando con `8`, 48 que no comienzan con `8`), o cuando es una guía de FGTS (segmento `5` con convenio `0181` o `0182` en las posiciones 16 a 19).

**Requisitos previos**

`payment_account_uid` es obligatorio y define la cuenta financiera de origen. `financial_account_uid` no se acepta en este endpoint.

`amount` es opcional cuando el código trae el valor a pagar: recaudación con identificador de valor `6` u `8` (3ª posición) y boleto con valor distinto de cero. Sin `amount`, el pago usa el valor del código; en el boleto, es el valor nominal, sin intereses, multa ni descuento. Es obligatorio en la recaudación con identificador `7` o `9`, en la que el valor del código es solo de referencia, y cuando el código viene con valor cero.

**Eventos de Webhook**

El evento `payment.db.created` se dispara en esta operación.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `code` | `string` | sí | Código de barras (44 posiciones) o línea digitable (47 o 48 posiciones). Se descartan espacios, puntos, guiones y barras. |
| `payment_account_uid` | `string (uuid)` | sí | UID de la Cuenta de Pago de origen. Obligatorio: no hay selección automática. |
| `amount` | `number` | no | Cantidad. Opcional cuando el código trae el valor a pagar (recaudación con identificador de valor `6` u `8`, o boleto con valor distinto de cero): sin ella, vale el valor del código, que en el boleto es el nominal, sin intereses, multa ni descuento. Obligatoria en la recaudación con identificador `7` o `9` y cuando el código viene con valor cero. Cuando se envía, siempre prevalece sobre el valor del código: para pagar un boleto vencido con cargos, envíe la cantidad con ellos. |
| `scheduled_to` | `string (date)` | no | Fecha de Programación |
| `beneficiary` | `object` | no | Información del beneficiario. Obligatorio cuando el código sea de boleto y la cuenta financiera sea BB. |
| `identifier` | `string` | no | Identificación del pago en el comprobante del proveedor financiero. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |

Otros campos y objetos anidados: `kobana schema banking.payments.barcode`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Pago de boleto creado |
| `422` | Código con valor de referencia, sin amount |

## payments get

Ver un Pago

```text
kobana banking payments get [UID] [OPCIONES]
```

`GET /v2/payments/{uid}`

Consulta información específica de un Pago de cualquier tipo. El tipo se identifica automáticamente y la respuesta se serializa adecuadamente.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Tributo GRU con formato=data |
| `404` | pago en lo encontrado |

## payments approve

Liberar (aprobar) un Pago

```text
kobana banking payments approve [UID] [OPCIONES]
```

`PUT /v2/payments/{uid}/approve`

> **Procesamiento Asincrónico:** crea un comando de liberación (approve) para un Pago registrado que está esperando aprobación.

La liberación se realiza de forma asincrónica: al recibir la solicitud, creamos un Comando que se procesa en segundo plano. Recibe en la respuesta el ID del comando creado y puede consultarlo en cualquier momento.

> ⚠️ **Soporte por banco:** no todos los proveedores exponen la liberación individual de pago a través de API. Cuando el proveedor no admite la operación, la respuesta es `422`.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando de liberación creado con éxito |
| `422` | El pago no está en un estado que permita liberación o el proveedor no lo admite |

## payments cancel

Cancelar un Pago

```text
kobana banking payments cancel [UID] [OPCIONES]
```

`PUT /v2/payments/{uid}/cancel`

> **Procesamiento Asincrónico:** Este endpoint se procesa de forma asincrónica y crea un comando de cancelación para el Pago.
La cancelación de un Pago se realiza de forma asincrónica, es decir, al recibir la solicitud, nuestra API guarda la solicitud y crea un Comando. Este comando se procesa en segundo plano y después de ser confirmado, el Pago cambia su estado a Cancelado. Recibe en la respuesta de esta solicitud el ID del comando creado y puede consultarlo en cualquier momento.

**Eventos de Webhook**

Cuando se realiza la solicitud, se dispara un evento `payment.{type}.cancel.requested` (donde \{type\} puede ser pix, bank_billet, etc).
Nuestro servidor entonces comienza los intentos de envío de la cancelación a la institución financiera.
Solo después de ser confirmado en la institución financiera, entonces el pago se marca como cancelado en nuestra base de datos.
En este momento se dispara el evento `payment.{type}.cancel.confirmed`.

> ⚠️ **Atención al estado Cancelado:** El estado de Cancelado es terminal. El pago no podrá ser procesado después de la cancelación.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago cancelado sin interacción con proveedor |
| `422` | El pago ya fue confirmado, rechazado o cancelado |
