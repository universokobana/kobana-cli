---
id: subscription-item-changes
title: "kobana billing subscription-item-changes"
sidebar_label: "subscription-item-changes"
---

# kobana billing subscription-item-changes


Comandos de `kobana billing subscription-item-changes`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`subscription-item-changes list`](#subscription-item-changes-list) | `GET /v1/subscription_item_changes` | Listar cambios de ítems de suscripción |
| [`subscription-item-changes get`](#subscription-item-changes-get) | `GET /v1/subscription_item_changes/{id}` | Obtener cambio de ítem de suscripción |
| [`subscription-item-changes cancel`](#subscription-item-changes-cancel) | `POST /v1/subscription_item_changes/{id}/cancel` | Cancela un cambio agendado |

## subscription-item-changes list

Listar cambios de ítems de suscripción

```text
kobana billing subscription-item-changes list [OPCIONES]
```

`GET /v1/subscription_item_changes`

Devuelve una lista paginada de los cambios aplicados (o agendados) sobre ítems de suscripciones de la organización — agregados, eliminaciones, cambios de cantidad y cambios de precio. Cada registro incluye el snapshot de las operaciones, los montos de prorrateo en BRL (float, hasta 4 decimales) y los punteros a la factura/credit generados.

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
| `subscription_id` | `string` | no | Filtra por los cambios de una suscripción específica. |
| `billing_account_id` | `string` | no | Filtra por los cambios de una cuenta de cobro específica. |
| `status` | `string` | no | Filtra por estado: `pending`, `scheduled`, `completed`, `canceled` o `failed`. Valores: `pending`, `scheduled`, `completed`, `canceled`, `failed`. |
| `timing` | `string` | no | Filtra por el momento de aplicación del cambio: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `initiated_by_role` | `string` | no | Filtra por el origen del cambio: `dashboard`, `portal` o `api`. Valores: `dashboard`, `portal`, `api`. |
| `proration_charge_min` | `number,null` | no | Monto mínimo del cargo de prorrateo, en reales (R$). Almacenado internamente en BRL (float, hasta 4 decimales). |
| `proration_charge_max` | `number,null` | no | Monto máximo del cargo de prorrateo, en reales (R$). Almacenado internamente en BRL (float, hasta 4 decimales). |
| `proration_credit_min` | `number,null` | no | Monto mínimo del crédito de prorrateo, en reales (R$). Almacenado internamente en BRL (float, hasta 4 decimales). |
| `proration_credit_max` | `number,null` | no | Monto máximo del crédito de prorrateo, en reales (R$). Almacenado internamente en BRL (float, hasta 4 decimales). |
| `effective_date_from` | `string` | no | Fecha efectiva mínima del cambio (inclusive), en formato `YYYY-MM-DD`. |
| `effective_date_to` | `string` | no | Fecha efectiva máxima del cambio (inclusive), en formato `YYYY-MM-DD`. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de cambios de ítems. |
| `404` | Recurso no encontrado en la organización actual. |

## subscription-item-changes get

Obtener cambio de ítem de suscripción

```text
kobana billing subscription-item-changes get [ID] [OPCIONES]
```

`GET /v1/subscription_item_changes/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del cambio de ítem. |
| `404` | Recurso no encontrado en la organización actual. |

## subscription-item-changes cancel

Cancela un cambio agendado

```text
kobana billing subscription-item-changes cancel [ID] [OPCIONES]
```

`POST /v1/subscription_item_changes/{id}/cancel`

Cancela un cambio de ítems aún agendado (`timing=end_of_period`). Los cambios ya aplicados no pueden cancelarse. Devuelve el registro con su nuevo estado. Exige el nivel de acceso `write`. Reemplaza `DELETE /v1/subscription_item_changes/{id}`, eliminado.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cambio cancelado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
