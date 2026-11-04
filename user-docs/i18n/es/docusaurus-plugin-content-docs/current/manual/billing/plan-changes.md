---
id: plan-changes
title: "kobana billing plan-changes"
sidebar_label: "plan-changes"
---

# kobana billing plan-changes


Comandos de `kobana billing plan-changes`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`plan-changes config`](#plan-changes-config) | `GET /v1/plan_changes/config` | Obtener configuración de cambio de plan |
| [`plan-changes config update`](#plan-changes-config-update) | `PATCH /v1/plan_changes/config` | Actualizar configuración de cambio de plan |
| [`plan-changes rules list`](#plan-changes-rules-list) | `GET /v1/plan_changes/rules` | Listar reglas de cambio de plan |
| [`plan-changes rules create`](#plan-changes-rules-create) | `POST /v1/plan_changes/rules` | Crear regla de cambio de plan |
| [`plan-changes rules get`](#plan-changes-rules-get) | `GET /v1/plan_changes/rules/{id}` | Obtener regla de cambio de plan |
| [`plan-changes rules update`](#plan-changes-rules-update) | `PATCH /v1/plan_changes/rules/{id}` | Actualizar regla de cambio de plan |
| [`plan-changes rules delete`](#plan-changes-rules-delete) | `DELETE /v1/plan_changes/rules/{id}` | Eliminar regla de cambio de plan |
| [`plan-changes stats`](#plan-changes-stats) | `GET /v1/plan_changes/stats` | Estadísticas de cambios de plan |
| [`plan-changes list`](#plan-changes-list) | `GET /v1/plan_changes` | Listar cambios de plan |
| [`plan-changes create`](#plan-changes-create) | `POST /v1/plan_changes` | Ejecutar cambio de plan |
| [`plan-changes get`](#plan-changes-get) | `GET /v1/plan_changes/{id}` | Obtener cambio de plan |

## plan-changes config

Obtener configuración de cambio de plan

```text
kobana billing plan-changes config [OPCIONES] [COMANDO]
```

`GET /v1/plan_changes/config`

Devuelve la configuración por defecto de la organización para upgrades y downgrades — comportamiento de prorrateo, timing y generación de crédito/refund.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Configuración actual. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-changes config update

Actualizar configuración de cambio de plan

```text
kobana billing plan-changes config update [OPCIONES]
```

`PATCH /v1/plan_changes/config`

Actualiza parcialmente (PATCH) la configuración de cambio de plan de la organización. Todos los campos son opcionales.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `allow_upgrade` | `boolean` | no | Habilita upgrades de plan para la organización. |
| `allow_downgrade` | `boolean` | no | Habilita downgrades de plan para la organización. |
| `upgrade_proration` | `string` | no | Método de prorrateo por defecto para upgrades: `full_proration`, `no_proration` o `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `downgrade_proration` | `string` | no | Método de prorrateo por defecto para downgrades. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `upgrade_timing` | `string` | no | Cuándo aplicar upgrades: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `downgrade_timing` | `string` | no | Cuándo aplicar downgrades: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `refund_on_downgrade` | `boolean` | no | Reembolsa la diferencia al hacer downgrade. |
| `credit_on_downgrade` | `boolean` | no | Genera crédito (en lugar de reembolso) al hacer downgrade. |
| `apply_discount_on_change` | `boolean` | no | Mantiene los descuentos activos al cambiar de plan. |

Otros campos y objetos anidados: `kobana schema billing.plan-changes.config.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Configuración actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plan-changes rules list

Listar reglas de cambio de plan

```text
kobana billing plan-changes rules list [OPCIONES]
```

`GET /v1/plan_changes/rules`

Lista las reglas personalizadas que sobrescriben el comportamiento por defecto para transiciones específicas entre planes.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de reglas de cambio de plan. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-changes rules create

Crear regla de cambio de plan

```text
kobana billing plan-changes rules create [OPCIONES]
```

`POST /v1/plan_changes/rules`

Crea una nueva regla para una transición entre planes. Permite bloquear/permitir la transición, definir timing, prorrateo, descuentos y días bonus.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `from_plan_id` | `string (uuid)` | no | UUID del plan de origen. Cuando se omite, la regla cubre cualquier plan de origen. |
| `to_plan_id` | `string (uuid)` | no | UUID del plan de destino. Cuando se omite, la regla cubre cualquier plan de destino. |
| `change_type` | `string` | no | Tipo del cambio: `upgrade`, `downgrade` o `lateral`. Valores: `upgrade`, `downgrade`, `lateral`. |
| `allowed` | `boolean` | no | Cuando es `false`, bloquea explícitamente la transición. |
| `timing` | `string` | no | Sobrescribe el timing del cambio: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `proration_method` | `string` | no | Sobrescribe el método de prorrateo: `full_proration`, `no_proration` o `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `discount_percent` | `number` | no | Descuento porcentual aplicado cuando esta regla coincide (0 a 100). |
| `bonus_days` | `integer` | no | Días bonus añadidos al período actual cuando se aplica esta regla. |
| `message` | `string` | no | Mensaje libre mostrado en el portal/checkout (ej.: motivo del bloqueo). |
| `priority` | `integer` | no | Prioridad para la resolución de conflictos cuando múltiples reglas coinciden (mayor gana). |

Otros campos y objetos anidados: `kobana schema billing.plan-changes.rules.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Regla creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plan-changes rules get

Obtener regla de cambio de plan

```text
kobana billing plan-changes rules get [ID] [OPCIONES]
```

`GET /v1/plan_changes/rules/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la regla. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-changes rules update

Actualizar regla de cambio de plan

```text
kobana billing plan-changes rules update [ID] [OPCIONES]
```

`PATCH /v1/plan_changes/rules/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `allowed` | `boolean` | no | Cuando es `false`, bloquea explícitamente la transición. |
| `timing` | `string` | no | Sobrescribe el timing del cambio: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `proration_method` | `string` | no | Sobrescribe el método de prorrateo: `full_proration`, `no_proration` o `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `discount_percent` | `number` | no | Descuento porcentual aplicado cuando esta regla coincide (0 a 100). |
| `bonus_days` | `integer` | no | Días bonus añadidos al período actual cuando se aplica esta regla. |
| `message` | `string` | no | Mensaje libre mostrado en el portal/checkout (ej.: motivo del bloqueo). |
| `priority` | `integer` | no | Prioridad para la resolución de conflictos cuando múltiples reglas coinciden (mayor gana). |
| `is_active` | `boolean` | no | Desactiva la regla sin eliminarla. |

Otros campos y objetos anidados: `kobana schema billing.plan-changes.rules.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Regla actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plan-changes rules delete

Eliminar regla de cambio de plan

```text
kobana billing plan-changes rules delete [ID] [OPCIONES]
```

`DELETE /v1/plan_changes/rules/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Regla eliminada. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-changes stats

Estadísticas de cambios de plan

```text
kobana billing plan-changes stats [OPCIONES]
```

`GET /v1/plan_changes/stats`

Devuelve agregados de cambios de plan (totales por tipo, por estado, créditos y cargos de prorrateo) para una ventana en días.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `days` | `integer` | no | Ventana en días para los agregados (por defecto: `30`). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Estadísticas agregadas de cambios de plan. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-changes list

Listar cambios de plan

```text
kobana billing plan-changes list [OPCIONES]
```

`GET /v1/plan_changes`

Devuelve el historial paginado de cambios de plan (upgrades, downgrades y laterales) de la organización actual.

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
| `subscription_id` | `string` | no | Filtra por los cambios de una suscripción. |
| `billing_account_id` | `string` | no | Filtra por los cambios de una cuenta de cobro. |
| `from_plan_id` | `string` | no | Filtra por el plan de origen (UUID). |
| `to_plan_id` | `string` | no | Filtra por el plan de destino (UUID). |
| `change_type` | `string` | no | Filtra por tipo: `upgrade`, `downgrade` o `lateral`. Valores: `upgrade`, `downgrade`, `lateral`. |
| `status` | `string` | no | Filtra por estado: `pending`, `scheduled`, `completed`, `canceled` o `failed`. Valores: `pending`, `scheduled`, `completed`, `canceled`, `failed`. |
| `timing` | `string` | no | Filtra por timing: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `proration_method` | `string` | no | Filtra por el método de prorrateo (ej.: `full_proration`, `no_proration`, `partial_proration`). |
| `price_difference_min` | `number,null` | no | Diferencia de precio mínima en R$. Ej.: `10.50`. |
| `price_difference_max` | `number,null` | no | Diferencia de precio máxima en R$. Ej.: `500.00`. |
| `proration_charge_min` | `number,null` | no | Cargo de prorrateo mínimo en R$. |
| `proration_charge_max` | `number,null` | no | Cargo de prorrateo máximo en R$. |
| `proration_credit_min` | `number,null` | no | Crédito de prorrateo mínimo en R$. |
| `proration_credit_max` | `number,null` | no | Crédito de prorrateo máximo en R$. |
| `effective_date_from` | `string` | no | Fecha de vigencia desde (inclusive). Formato: `YYYY-MM-DD`. |
| `effective_date_to` | `string` | no | Fecha de vigencia hasta (inclusive). Formato: `YYYY-MM-DD`. |
| `scheduled_date_from` | `string` | no | Fecha programada desde (inclusive). Formato: `YYYY-MM-DD`. |
| `scheduled_date_to` | `string` | no | Fecha programada hasta (inclusive). Formato: `YYYY-MM-DD`. |
| `completed_after` | `string` | no | Completada desde (inclusive). Formato: `YYYY-MM-DD`. |
| `completed_before` | `string` | no | Completada hasta (inclusive). Formato: `YYYY-MM-DD`. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de cambios de plan. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-changes create

Ejecutar cambio de plan

```text
kobana billing plan-changes create [OPCIONES]
```

`POST /v1/plan_changes`

Ejecuta un cambio de plan para una suscripción existente. `timing` controla si el cambio se aplica de inmediato (con prorrateo) o al final del período actual. En caso de éxito, devuelve los UUIDs del `plan_change`, la factura de prorrateo (si existe) y el crédito generado (si existe).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `subscription_id` | `string (uuid)` | sí | UUID de la suscripción que tendrá su plan cambiado. |
| `to_plan_id` | `string (uuid)` | sí | UUID del plan de destino. |
| `timing` | `string` | no | Cuándo aplicar el cambio: `immediate` (por defecto para upgrades) o `end_of_period` (por defecto para downgrades). Valores: `immediate`, `end_of_period`. |
| `reason` | `string` | no | Motivo del cambio (registrado en el audit log). |

Otros campos y objetos anidados: `kobana schema billing.plan-changes.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cambio de plan ejecutado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plan-changes get

Obtener cambio de plan

```text
kobana billing plan-changes get [ID] [OPCIONES]
```

`GET /v1/plan_changes/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del cambio de plan. |
| `404` | Recurso no encontrado en la organización actual. |
