---
id: credit-purchases
title: "kobana billing credit-purchases"
sidebar_label: "credit-purchases"
---

# kobana billing credit-purchases


Comandos de `kobana billing credit-purchases`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`credit-purchases create`](#credit-purchases-create) | `POST /v1/credit_purchases` | Comprar créditos |

## credit-purchases create

Comprar créditos

```text
kobana billing credit-purchases create [OPCIONES]
```

`POST /v1/credit_purchases`

Genera y finaliza una factura independiente (`billing_reason = credit_purchase`) con la cantidad de créditos de un producto de tipo `credits`, sin depender de una suscripción. El saldo vale cuando la factura se paga: el webhook `invoice.paid` trae `line_items[].product_type` y `quantity`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `product_id` | `string (uuid)` | sí | Producto de tipo `credits`. |
| `quantity` | `integer` | sí | Cantidad de créditos (entera; múltiplo del incremento del producto). |
| `billing_account_id` | `string (uuid)` | no | ID de la cuenta de cobro. Envía este **o** `billing_account_public_id`. |
| `billing_account_public_id` | `string (uuid)` | no | Identificador público de la cuenta de cobro. |
| `price_id` | `string (uuid)` | no | Precio a cobrar. Por defecto: el precio activo por defecto del producto. |
| `collection_method` | `string` | no | `manual_charge` (por defecto): el cliente paga la factura. `charge_automatically`: cobra la tarjeta por defecto al finalizar. Valores: `charge_automatically`, `manual_charge`. |
| `due_date` | `string` | no | Vencimiento. Por defecto: el plazo de pago de la cuenta. |

Otros campos y objetos anidados: `kobana schema billing.credit-purchases.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Factura de la compra, con sus ítems. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
