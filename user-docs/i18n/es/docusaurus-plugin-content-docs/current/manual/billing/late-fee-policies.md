---
id: late-fee-policies
title: "kobana billing late-fee-policies"
sidebar_label: "late-fee-policies"
---

# kobana billing late-fee-policies


Comandos de `kobana billing late-fee-policies`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`late-fee-policies list`](#late-fee-policies-list) | `GET /v1/late_fee_policies` | Listar políticas de intereses y multas |
| [`late-fee-policies create`](#late-fee-policies-create) | `POST /v1/late_fee_policies` | Crear política de intereses y multas |
| [`late-fee-policies simulate`](#late-fee-policies-simulate) | `POST /v1/late_fee_policies/simulate` | Simular intereses y multas |
| [`late-fee-policies get`](#late-fee-policies-get) | `GET /v1/late_fee_policies/{id}` | Consultar política de intereses y multas |
| [`late-fee-policies update`](#late-fee-policies-update) | `PATCH /v1/late_fee_policies/{id}` | Actualizar política de intereses y multas |
| [`late-fee-policies delete`](#late-fee-policies-delete) | `DELETE /v1/late_fee_policies/{id}` | Eliminar política de intereses y multas |
| [`late-fee-policies default`](#late-fee-policies-default) | `POST /v1/late_fee_policies/{id}/default` | Definir como política por defecto |

## late-fee-policies list

Listar políticas de intereses y multas

```text
kobana billing late-fee-policies list [OPCIONES]
```

`GET /v1/late_fee_policies`

Devuelve las políticas de intereses y multas de la organización. La política aplicada a una factura se resuelve en cascada: factura → suscripción → cuenta de cobro → política por defecto de la organización. Un nivel nulo significa heredar, nunca "sin cargo" — no cobrar nada es una política explícita con ambos modos en `none`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `include_deleted` | `string` | no | Incluye políticas eliminadas lógicamente. Valores: `true`, `false`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de políticas de intereses y multas. |
| `404` | Recurso no encontrado en la organización actual. |

## late-fee-policies create

Crear política de intereses y multas

```text
kobana billing late-fee-policies create [OPCIONES]
```

`POST /v1/late_fee_policies`

Crea una política. Multa e intereses por encima de los topes legales (2% de multa y 1% mensual de intereses, CDC art. 52) se rechazan salvo que `enforce_legal_caps` sea `false` — lo cual se audita por separado, al ser una decisión de riesgo jurídico.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la política, único en la organización. |
| `is_default` | `boolean` | no | Indica si es la política por defecto de la organización. |
| `fine_mode` | `string` | no | Cómo se calcula la multa. Se cobra una sola vez. `percentage` usa `fine_percent`, `fixed` usa `fine_amount`. Valores: `none`, `percentage`, `fixed`. |
| `fine_percent` | `number` | no | Porcentaje de la multa sobre el saldo, de 0 a 100. Solo se completa cuando `fine_mode` es `percentage`. |
| `fine_amount` | `number` | no | Valor fijo de la multa, en reales. Solo se completa cuando `fine_mode` es `fixed`. |
| `fine_days` | `integer` | no | Días después del vencimiento en que la multa comienza a aplicarse. El valor por defecto es 1 — el día siguiente al vencimiento. |
| `interest_mode` | `string` | no | Cómo corren los intereses. Los modos porcentuales usan `interest_percent`; `daily_amount` usa `interest_amount`. Los intereses se cobran solo sobre el principal, nunca sobre principal más multa. Valores: `none`, `daily_amount`, `daily_percentage`, `monthly_percentage`, `yearly_percentage`. |
| `interest_percent` | `number` | no | Porcentaje de intereses por día, mes o año, según `interest_mode`. La tasa mensual se divide pro rata die por 30; la anual, por 360. |
| `interest_amount` | `number` | no | Valor fijo de intereses por día, en reales. Solo se completa cuando `interest_mode` es `daily_amount`. |
| `interest_days` | `integer` | no | Días después del vencimiento en que los intereses comienzan a correr. El valor por defecto es 1. |
| `day_count` | `string` | no | Base de conteo de los días de atraso. Por ahora solo se acepta `calendar`: el cálculo local aún no tiene calendario de feriados, y contar días hábiles sin él sobrestimaría el cargo frente a lo que recauda el banco. Valores: `calendar`, `business`. |
| `max_interest_months` | `integer` | no | Límite de meses de intereses acumulados. Nulo significa sin límite. |
| `max_total_charge_percent` | `number` | no | Límite del cargo total (multa más intereses) como porcentaje del saldo. La multa se preserva y los intereses se recortan cuando el tope actúa. |
| `enforce_legal_caps` | `boolean` | no | Cuando es verdadero, rechaza multa superior al 2% e intereses superiores al 1% mensual. Desactivarlo es una decisión deliberada para contratos fuera de la relación de consumo, y se audita. |
| `partial_allocation` | `string` | no | Orden de imputación cuando un pago no cubre principal y cargos. `charges_first` sigue el art. 354 del Código Civil brasileño: intereses antes del capital. Valores: `charges_first`, `principal_first`. |
| `register_on_charge` | `boolean` | no | Registra la instrucción de multa e intereses en el banco al emitir boleto o Pix, para que el propio cobro recaude los cargos después del vencimiento. |
| `apply_on_dynamic_charge` | `boolean` | no | Recalcula el valor cobrado en tarjeta y en reemisiones, donde no existe instrucción bancaria que cobre el cargo. |
| `charge_instructions` | `string` | no | Texto libre impreso en las instrucciones del boleto. |

Otros campos y objetos anidados: `kobana schema billing.late-fee-policies.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Política creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## late-fee-policies simulate

Simular intereses y multas

```text
kobana billing late-fee-policies simulate [OPCIONES]
```

`POST /v1/late_fee_policies/simulate`

Calcula cuánto debe un saldo en multa e intereses en una fecha, sin crear nada. Informe `policy_id` o una `policy` inline. Sirve para conciliar el cálculo antes de que una factura venza de verdad.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `principal` | `number` | sí | Saldo sobre el cual calcular los cargos, en reales. |
| `due_date` | `string (date-time)` | sí | Vencimiento a partir del cual se cuenta el atraso. |
| `reference_date` | `string (date-time)` | no | Fecha de cálculo. Por defecto, ahora. Los días se cuentan en la fecha civil en America/Sao_Paulo. |
| `policy_id` | `string (uuid)` | no | Política existente a usar. Alternativa a `policy`. |
| `policy` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema billing.late-fee-policies.simulate`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cargos devengados en la fecha indicada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## late-fee-policies get

Consultar política de intereses y multas

```text
kobana billing late-fee-policies get [ID] [OPCIONES]
```

`GET /v1/late_fee_policies/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Política encontrada. |
| `404` | Recurso no encontrado en la organización actual. |

## late-fee-policies update

Actualizar política de intereses y multas

```text
kobana billing late-fee-policies update [ID] [OPCIONES]
```

`PATCH /v1/late_fee_policies/{id}`

Actualiza la política. Las facturas ya finalizadas no se ven afectadas: cada factura congela la política vigente al finalizar, de modo que una renegociación no cambie retroactivamente lo ya emitido.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre de la política, único en la organización. |
| `is_default` | `boolean` | no | Indica si es la política por defecto de la organización. |
| `fine_mode` | `string` | no | Cómo se calcula la multa. Se cobra una sola vez. `percentage` usa `fine_percent`, `fixed` usa `fine_amount`. Valores: `none`, `percentage`, `fixed`. |
| `fine_percent` | `number` | no | Porcentaje de la multa sobre el saldo, de 0 a 100. Solo se completa cuando `fine_mode` es `percentage`. |
| `fine_amount` | `number` | no | Valor fijo de la multa, en reales. Solo se completa cuando `fine_mode` es `fixed`. |
| `fine_days` | `integer` | no | Días después del vencimiento en que la multa comienza a aplicarse. El valor por defecto es 1 — el día siguiente al vencimiento. |
| `interest_mode` | `string` | no | Cómo corren los intereses. Los modos porcentuales usan `interest_percent`; `daily_amount` usa `interest_amount`. Los intereses se cobran solo sobre el principal, nunca sobre principal más multa. Valores: `none`, `daily_amount`, `daily_percentage`, `monthly_percentage`, `yearly_percentage`. |
| `interest_percent` | `number` | no | Porcentaje de intereses por día, mes o año, según `interest_mode`. La tasa mensual se divide pro rata die por 30; la anual, por 360. |
| `interest_amount` | `number` | no | Valor fijo de intereses por día, en reales. Solo se completa cuando `interest_mode` es `daily_amount`. |
| `interest_days` | `integer` | no | Días después del vencimiento en que los intereses comienzan a correr. El valor por defecto es 1. |
| `day_count` | `string` | no | Base de conteo de los días de atraso. Por ahora solo se acepta `calendar`: el cálculo local aún no tiene calendario de feriados, y contar días hábiles sin él sobrestimaría el cargo frente a lo que recauda el banco. Valores: `calendar`, `business`. |
| `max_interest_months` | `integer` | no | Límite de meses de intereses acumulados. Nulo significa sin límite. |
| `max_total_charge_percent` | `number` | no | Límite del cargo total (multa más intereses) como porcentaje del saldo. La multa se preserva y los intereses se recortan cuando el tope actúa. |
| `enforce_legal_caps` | `boolean` | no | Cuando es verdadero, rechaza multa superior al 2% e intereses superiores al 1% mensual. Desactivarlo es una decisión deliberada para contratos fuera de la relación de consumo, y se audita. |
| `partial_allocation` | `string` | no | Orden de imputación cuando un pago no cubre principal y cargos. `charges_first` sigue el art. 354 del Código Civil brasileño: intereses antes del capital. Valores: `charges_first`, `principal_first`. |
| `register_on_charge` | `boolean` | no | Registra la instrucción de multa e intereses en el banco al emitir boleto o Pix, para que el propio cobro recaude los cargos después del vencimiento. |
| `apply_on_dynamic_charge` | `boolean` | no | Recalcula el valor cobrado en tarjeta y en reemisiones, donde no existe instrucción bancaria que cobre el cargo. |
| `charge_instructions` | `string` | no | Texto libre impreso en las instrucciones del boleto. |

Otros campos y objetos anidados: `kobana schema billing.late-fee-policies.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Política actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## late-fee-policies delete

Eliminar política de intereses y multas

```text
kobana billing late-fee-policies delete [ID] [OPCIONES]
```

`DELETE /v1/late_fee_policies/{id}`

Eliminación lógica. Las facturas ya emitidas siguen valiendo por el snapshot que llevan. La política por defecto no puede eliminarse — defina otra como predeterminada antes.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Política eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## late-fee-policies default

Definir como política por defecto

```text
kobana billing late-fee-policies default [ID] [OPCIONES]
```

`POST /v1/late_fee_policies/{id}/default`

Convierte esta en la política por defecto de la organización — lo que hereda una factura cuando ni ella, ni su suscripción, ni su cuenta de cobro indican una. Exactamente una política lleva la marca.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Política definida como predeterminada. |
| `404` | Recurso no encontrado en la organización actual. |
