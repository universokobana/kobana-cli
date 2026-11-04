---
id: subscriptions
title: "kobana billing subscriptions"
sidebar_label: "subscriptions"
---

# kobana billing subscriptions


Comandos de `kobana billing subscriptions`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`subscriptions items`](#subscriptions-items) | `GET /v1/subscriptions/{id}/items` | Listar ítems de la suscripción |
| [`subscriptions items create`](#subscriptions-items-create) | `POST /v1/subscriptions/{id}/items` | Aplica cambios en los ítems de la suscripción |
| [`subscriptions list`](#subscriptions-list) | `GET /v1/subscriptions` | Listar suscripciones |
| [`subscriptions create`](#subscriptions-create) | `POST /v1/subscriptions` | Crear suscripción |
| [`subscriptions update`](#subscriptions-update) | `PUT /v1/subscriptions` | Crear o actualizar suscripción |
| [`subscriptions get`](#subscriptions-get) | `GET /v1/subscriptions/{id}` | Obtener suscripción |
| [`subscriptions delete`](#subscriptions-delete) | `DELETE /v1/subscriptions/{id}` | Cancelar y eliminar suscripción (soft delete) |
| [`subscriptions activate`](#subscriptions-activate) | `POST /v1/subscriptions/{id}/activate` | Activar suscripción en borrador/confirmada |
| [`subscriptions cancel`](#subscriptions-cancel) | `POST /v1/subscriptions/{id}/cancel` | Cancelar suscripción |
| [`subscriptions change-anniversary`](#subscriptions-change-anniversary) | `POST /v1/subscriptions/{id}/change-anniversary` | Cambiar el aniversario (día de inicio del ciclo) de la suscripción |
| [`subscriptions change-plan`](#subscriptions-change-plan) | `POST /v1/subscriptions/{id}/change-plan` | Cambiar el plan de la suscripción |
| [`subscriptions create-invoice`](#subscriptions-create-invoice) | `POST /v1/subscriptions/{id}/create-invoice` | Generar una factura puntual para la suscripción |
| [`subscriptions pause`](#subscriptions-pause) | `POST /v1/subscriptions/{id}/pause` | Pausar suscripción |
| [`subscriptions resume`](#subscriptions-resume) | `POST /v1/subscriptions/{id}/resume` | Reanudar suscripción pausada |
| [`subscriptions revert-to-confirmed`](#subscriptions-revert-to-confirmed) | `POST /v1/subscriptions/{id}/revert-to-confirmed` | Revertir suscripción al estado confirmado |
| [`subscriptions sync-plan-items`](#subscriptions-sync-plan-items) | `POST /v1/subscriptions/{id}/sync-plan-items` | Resincronizar ítems con el plan actual |

## subscriptions items

Listar ítems de la suscripción

```text
kobana billing subscriptions items [ID] [OPCIONES] [COMANDO]
```

`GET /v1/subscriptions/{id}/items`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de ítems. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions items create

Aplica cambios en los ítems de la suscripción

```text
kobana billing subscriptions items create [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/items`

Ejecuta un lote de hasta 50 operaciones sobre los ítems: agregar, eliminar, cambiar cantidad, cambiar de precio o definir un valor unitario puntual. Las operaciones se aplican en conjunto y generan un único registro de cambio, con la factura o el crédito de prorrateo resultante.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `changes` | `array<oneOf>` | sí | Operaciones a aplicar, de 1 a 50. Cada ítem se discrimina por `op`. |
| `timing` | `string` | no | Cuándo aplicar: `immediate` (predeterminado) o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `reason` | `string` | no | Motivo libre, guardado junto al registro de cambio. |
| `idempotency_key` | `string` | no | Clave de idempotencia. El encabezado `Idempotency-Key` tiene precedencia sobre este campo. |

Otros campos y objetos anidados: `kobana schema billing.subscriptions.items.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cambios aplicados. `item_change_id` apunta al registro generado; `invoice_id` y `credit_id` vienen completos cuando el prorrateo generó factura o crédito. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## subscriptions list

Listar suscripciones

```text
kobana billing subscriptions list [OPCIONES]
```

`GET /v1/subscriptions`

Devuelve una lista paginada de las suscripciones de la organización actual.

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
| `status` | `string` | no | Filtra por estado de la suscripción. Valores: `draft`, `trialing`, `active`, `past_due`, `canceled`, `paused`, `suspended`. |
| `plan_id` | `string` | no | Filtra por plan. |
| `billing_cycle` | `string` | no | Filtra por periodicidad de cobro. Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `billing_account_id` | `string` | no | Filtra por cuenta de cobro. |
| `search` | `string` | no | Búsqueda textual libre. |
| `company_id` | `string` | no | Filtra por empresa (UUID). |
| `collection_method` | `string` | no | Filtra por el método de cobro: `charge_automatically`, `manual_charge` o `manual_invoice`. Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `cancel_at_period_end` | `string` | no | Filtra suscripciones programadas para cancelarse al fin del período (`true`) o no (`false`). Valores: `true`, `false`. |
| `external_id` | `string` | no | Filtra por el identificador externo. |
| `product_group_id` | `string` | no | Filtra por grupo de producto (UUID). |
| `proposal_id` | `string` | no | Filtra por propuesta (UUID). |
| `recurring_amount_min` | `number,null` | no | Importe recurrente mínimo en reales (ej.: `50.00`). |
| `recurring_amount_max` | `number,null` | no | Importe recurrente máximo en reales (ej.: `500.00`). |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `current_period_start_from` | `string` | no | Inicio del período actual a partir de esta fecha (YYYY-MM-DD). |
| `current_period_start_to` | `string` | no | Inicio del período actual hasta esta fecha (YYYY-MM-DD). |
| `current_period_end_from` | `string` | no | Fin del período actual a partir de esta fecha (YYYY-MM-DD). |
| `current_period_end_to` | `string` | no | Fin del período actual hasta esta fecha (YYYY-MM-DD). |
| `trial_end_from` | `string` | no | Fin del trial a partir de esta fecha (YYYY-MM-DD). |
| `trial_end_to` | `string` | no | Fin del trial hasta esta fecha (YYYY-MM-DD). |
| `canceled_after` | `string` | no | Cancelado a partir de esta fecha (YYYY-MM-DD). |
| `canceled_before` | `string` | no | Cancelado hasta esta fecha (YYYY-MM-DD). |
| `activated_after` | `string` | no | Activado a partir de esta fecha (YYYY-MM-DD). |
| `activated_before` | `string` | no | Activado hasta esta fecha (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de suscripciones. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions create

Crear suscripción

```text
kobana billing subscriptions create [OPCIONES]
```

`POST /v1/subscriptions`

Crea una suscripción. Es obligatorio enviar `plan_id` o `items`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sí | UUID de la cuenta de cobro que recibirá esta suscripción. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no |  |
| `plan_id` | `string (uuid)` | no | UUID del plan. Obligatorio cuando no se envía `items`. |
| `product_group_id` | `string (uuid)` | no |  |
| `billing_cycle` | `string` | no | Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `collection_method` | `string` | no | Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `trial_end` | `string (date-time)` | no | Fecha de finalización del trial (ISO 8601). |
| `trial_period_days` | `integer` | no | Alternativa a `trial_end`. Días de trial a partir de la creación. |
| `proration_behavior` | `string` | no | Valores: `create_prorations`, `none`, `always_invoice`. |
| `nfe_issuance_policy` | `string` | no | Valores: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `items` | `array<object>` | no |  |
| `commitment_months` | `integer` | no | Permanencia: número de meses contratados (copiado de la propuesta aceptada). Dentro de la ventana de permanencia — inicio de la suscripción + meses — el cliente no puede cancelar por el portal. Use `null` para quitar. |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema billing.subscriptions.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Suscripción creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## subscriptions update

Crear o actualizar suscripción

```text
kobana billing subscriptions update [OPCIONES]
```

`PUT /v1/subscriptions`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro que recibirá esta suscripción. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no |  |
| `plan_id` | `string (uuid)` | no | UUID del plan. Obligatorio cuando no se envía `items`. |
| `product_group_id` | `string (uuid)` | no |  |
| `billing_cycle` | `string` | no | Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `collection_method` | `string` | no | Valores: `charge_automatically`, `manual_charge`, `manual_invoice`. |
| `trial_end` | `string (date-time)` | no | Fecha de finalización del trial (ISO 8601). |
| `trial_period_days` | `integer` | no | Alternativa a `trial_end`. Días de trial a partir de la creación. |
| `proration_behavior` | `string` | no | Valores: `create_prorations`, `none`, `always_invoice`. |
| `nfe_issuance_policy` | `string` | no | Valores: `disabled`, `on_finalization`, `on_full_payment`, `per_installment`. |
| `items` | `array<object>` | no |  |
| `commitment_months` | `integer` | no | Permanencia: número de meses contratados (copiado de la propuesta aceptada). Dentro de la ventana de permanencia — inicio de la suscripción + meses — el cliente no puede cancelar por el portal. Use `null` para quitar. |
| `notes` | `string` | no |  |
| `custom_metadata` | `object` | no |  |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.subscriptions.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## subscriptions get

Obtener suscripción

```text
kobana billing subscriptions get [ID] [OPCIONES]
```

`GET /v1/subscriptions/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la suscripción. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions delete

Cancelar y eliminar suscripción (soft delete)

```text
kobana billing subscriptions delete [ID] [OPCIONES]
```

`DELETE /v1/subscriptions/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Confirmación de soft delete. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions activate

Activar suscripción en borrador/confirmada

```text
kobana billing subscriptions activate [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/activate`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Suscripción activada. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions cancel

Cancelar suscripción

```text
kobana billing subscriptions cancel [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/cancel`

Cancela inmediatamente, o programa para el fin del período cuando `cancel_at_period_end=true`.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cancel_at_period_end` | `boolean` | no | Cuando es `true`, programa la cancelación para el fin del período actual. Por defecto: `false` (cancela inmediatamente). |
| `reason` | `string` | no |  |

Otros campos y objetos anidados: `kobana schema billing.subscriptions.cancel`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Suscripción cancelada. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions change-anniversary

Cambiar el aniversario (día de inicio del ciclo) de la suscripción

```text
kobana billing subscriptions change-anniversary [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/change-anniversary`

Mueve el día del mes en que empiezan los ciclos. El ciclo en curso mantiene su inicio y se extiende hasta la víspera del nuevo día; los días añadidos se cobran a prorrata (importe del ciclo × días ÷ días del ciclo) en una factura `subscription_update`. El cambio siempre avanza la fecha, nunca la retrocede. Use `dry_run` para calcular sin aplicar.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `anniversary_day` | `integer` | sí | Nuevo día del mes (1 a 28) en que empiezan los ciclos. Use 1 para mes calendario. |
| `charge_proration` | `boolean` | no | Cobrar los días añadidos. Por defecto: `true`, salvo cuando el `proration_behavior` de la suscripción es `none`. |
| `dry_run` | `boolean` | no | Cuando es `true`, solo calcula y devuelve el resultado, sin cambiar nada. |
| `reason` | `string` | no | Motivo del cambio (opcional, registrado en el audit log). |

Otros campos y objetos anidados: `kobana schema billing.subscriptions.change-anniversary`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado del cambio (o de la simulación, con `dry_run`). |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## subscriptions change-plan

Cambiar el plan de la suscripción

```text
kobana billing subscriptions change-plan [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/change-plan`

Upgrade o downgrade. La prorrata sigue el `proration_behavior` de la suscripción.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `plan_id` | `string (uuid)` | sí | UUID del nuevo plan. |
| `timing` | `string` | no | Cuándo entra en vigor el cambio de plan. Por defecto: `immediate`. Valores: `immediate`, `end_of_period`. |
| `reason` | `string` | no | Motivo del cambio (opcional, registrado en el audit log). |

Otros campos y objetos anidados: `kobana schema billing.subscriptions.change-plan`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Suscripción actualizada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## subscriptions create-invoice

Generar una factura puntual para la suscripción

```text
kobana billing subscriptions create-invoice [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/create-invoice`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Factura generada. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions pause

Pausar suscripción

```text
kobana billing subscriptions pause [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/pause`

Suspende el cobro manteniendo la suscripción abierta. `behavior` controla cómo se tratan las facturas generadas durante la pausa.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `behavior` | `string` | no | Cómo tratar facturas generadas durante la pausa. Por defecto: `mark_uncollectible`. Valores: `mark_uncollectible`, `keep_as_draft`, `void`. |
| `resumes_at` | `string (date-time)` | no | Fecha para reanudación automática. Omita para reanudación manual. |

Otros campos y objetos anidados: `kobana schema billing.subscriptions.pause`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Suscripción pausada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## subscriptions resume

Reanudar suscripción pausada

```text
kobana billing subscriptions resume [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/resume`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Suscripción reanudada. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions revert-to-confirmed

Revertir suscripción al estado confirmado

```text
kobana billing subscriptions revert-to-confirmed [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/revert-to-confirmed`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Suscripción revertida. |
| `404` | Recurso no encontrado en la organización actual. |

## subscriptions sync-plan-items

Resincronizar ítems con el plan actual

```text
kobana billing subscriptions sync-plan-items [ID] [OPCIONES]
```

`POST /v1/subscriptions/{id}/sync-plan-items`

Realinea los ítems de la suscripción con la plantilla de plan-items (reconciliación manual).

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Suscripción sincronizada. |
| `404` | Recurso no encontrado en la organización actual. |
