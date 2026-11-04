---
id: plans
title: "kobana billing plans"
sidebar_label: "plans"
---

# kobana billing plans


Comandos de `kobana billing plans`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`plans bulk delete`](#plans-bulk-delete) | `DELETE /v1/plans/bulk` | Eliminar planes en lote |
| [`plans items`](#plans-items) | `GET /v1/plans/{id}/items` | Listar ítems del plan |
| [`plans items update`](#plans-items-update) | `PUT /v1/plans/{id}/items` | Reemplazar ítems del plan |
| [`plans list`](#plans-list) | `GET /v1/plans` | Listar planes |
| [`plans create`](#plans-create) | `POST /v1/plans` | Crear plan |
| [`plans update`](#plans-update) | `PUT /v1/plans` | Crear o actualizar plan |
| [`plans bulk-archive`](#plans-bulk-archive) | `POST /v1/plans/bulk-archive` | Archivar planes en lote |
| [`plans bulk-move-group`](#plans-bulk-move-group) | `POST /v1/plans/bulk-move-group` | Mover planes a otro grupo en lote |
| [`plans get`](#plans-get) | `GET /v1/plans/{id}` | Obtener plan |
| [`plans delete`](#plans-delete) | `DELETE /v1/plans/{id}` | Eliminar plan |
| [`plans archive`](#plans-archive) | `POST /v1/plans/{id}/archive` | Archivar plan |
| [`plans duplicate`](#plans-duplicate) | `POST /v1/plans/{id}/duplicate` | Duplicar plan |

## plans bulk delete

Eliminar planes en lote

```text
kobana billing plans bulk delete [OPCIONES]
```

`DELETE /v1/plans/bulk`

Elimina los planes informados (soft delete: `deleted_at`, el mismo efecto que `DELETE /v1/plans/{id}` en cada uno). Los planes con una suscripción que no esté cancelada se omiten y se devuelven en `skipped_ids`. Exige el nivel de acceso `delete`. Reemplaza `POST /v1/plans/bulk-delete`, eliminado.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | Lista de UUIDs de planes. Mínimo 1. |

Otros campos y objetos anidados: `kobana schema billing.plans.bulk.delete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resumen de la eliminación en lote. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plans items

Listar ítems del plan

```text
kobana billing plans items [ID] [OPCIONES] [COMANDO]
```

`GET /v1/plans/{id}/items`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de ítems del plan. |
| `404` | Recurso no encontrado en la organización actual. |

## plans items update

Reemplazar ítems del plan

```text
kobana billing plans items update [ID] [OPCIONES]
```

`PUT /v1/plans/{id}/items`

Reemplaza de forma atómica todos los ítems del plan. Solo se aceptan productos raíz y cada `product_id` debe ser único en la lista.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `items` | `array<object>` | sí |  |

Otros campos y objetos anidados: `kobana schema billing.plans.items.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Ítems del plan reemplazados. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plans list

Listar planes

```text
kobana billing plans list [OPCIONES]
```

`GET /v1/plans`

Devuelve una lista paginada de los planes de la organización actual.

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
| `status` | `string` | no | Filtra por estado del plan. Valores: `draft`, `active`, `archived`. |
| `plan_type` | `string` | no | Filtra por tipo del plan. Valores: `standard`, `custom`, `promotional`. |
| `visibility` | `string` | no | Filtra por visibilidad. Valores: `public`, `private`. |
| `plan_group_id` | `string` | no | Filtra por grupo de planes. |
| `company_id` | `string` | no | Filtra por empresa emisora. |
| `billing_account_id` | `string` | no | Filtra planes custom asociados a una cuenta de cobro. |
| `scoped_billing_account_id` | `string` | no | Devuelve planes disponibles para la cuenta de cobro indicada (estándar + planes custom de la cuenta). |
| `exclude_custom_plans` | `string` | no | Cuando es `true`, excluye los planes de tipo `custom` de los resultados. Valores: `true`, `false`. |
| `default_billing_cycle` | `string` | no | Filtra por la periodicidad de cobro por defecto del plan. Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `external_id` | `string` | no | Filtra por el identificador externo del plan. |
| `slug` | `string` | no | Filtra por el slug del plan. |
| `search` | `string` | no | Búsqueda textual libre. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `updated_after` | `string` | no | Inicio del rango de fecha de actualización (YYYY-MM-DD). |
| `updated_before` | `string` | no | Fin del rango de fecha de actualización (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de planes. |
| `404` | Recurso no encontrado en la organización actual. |

## plans create

Crear plan

```text
kobana billing plans create [OPCIONES]
```

`POST /v1/plans`

Crea un plan para la organización autenticada.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí |  |
| `slug` | `string` | sí | Solo letras minúsculas, dígitos y guiones. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no |  |
| `plan_group_id` | `string (uuid)` | no |  |
| `description` | `string` | no |  |
| `status` | `string` | no | Valores: `draft`, `active`, `archived`. |
| `plan_type` | `string` | no | Por defecto: `standard`. Valores: `standard`, `custom`, `promotional`. |
| `billing_account_id` | `string (uuid)` | no |  |
| `visibility` | `string` | no | Por defecto: `public`. Valores: `public`, `private`. |
| `features` | `array<string>` | no |  |
| `limits` | `object` | no |  |
| `trial_period_days` | `integer` | no |  |
| `default_billing_cycle` | `string` | no | Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `available_billing_cycles` | `array<string>` | no |  |
| `items` | `array<object>` | no |  |
| `custom_metadata` | `object` | no | Metadatos personalizados en formato libre (`object`). |

Otros campos y objetos anidados: `kobana schema billing.plans.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Plan creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plans update

Crear o actualizar plan

```text
kobana billing plans update [OPCIONES]
```

`PUT /v1/plans`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no |  |
| `plan_group_id` | `string (uuid)` | no |  |
| `name` | `string` | no |  |
| `slug` | `string` | no | Solo letras minúsculas, dígitos y guiones. |
| `description` | `string` | no |  |
| `status` | `string` | no | Valores: `draft`, `active`, `archived`. |
| `plan_type` | `string` | no | Por defecto: `standard`. Valores: `standard`, `custom`, `promotional`. |
| `billing_account_id` | `string (uuid)` | no |  |
| `visibility` | `string` | no | Por defecto: `public`. Valores: `public`, `private`. |
| `features` | `array<string>` | no |  |
| `limits` | `object` | no |  |
| `trial_period_days` | `integer` | no |  |
| `default_billing_cycle` | `string` | no | Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |
| `available_billing_cycles` | `array<string>` | no |  |
| `items` | `array<object>` | no |  |
| `custom_metadata` | `object` | no | Metadatos personalizados en formato libre (`object`). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.plans.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plans bulk-archive

Archivar planes en lote

```text
kobana billing plans bulk-archive [OPCIONES]
```

`POST /v1/plans/bulk-archive`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | Lista de UUIDs de planes. Mínimo 1. |

Otros campos y objetos anidados: `kobana schema billing.plans.bulk-archive`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resumen del archivado en lote. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plans bulk-move-group

Mover planes a otro grupo en lote

```text
kobana billing plans bulk-move-group [OPCIONES]
```

`POST /v1/plans/bulk-move-group`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí |  |
| `plan_group_id` | `string (uuid)` | sí | UUID del grupo destino. Usa `null` para quitar la agrupación. |

Otros campos y objetos anidados: `kobana schema billing.plans.bulk-move-group`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resumen del movimiento en lote. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plans get

Obtener plan

```text
kobana billing plans get [ID] [OPCIONES]
```

`GET /v1/plans/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del plan. |
| `404` | Recurso no encontrado en la organización actual. |

## plans delete

Eliminar plan

```text
kobana billing plans delete [ID] [OPCIONES]
```

`DELETE /v1/plans/{id}`

Elimina el plan (soft delete: `deleted_at` — desaparece de toda lectura; las reglas de cambio de plan que lo citan se eliminan). Exige el nivel de acceso `delete`. Devuelve 422 mientras haya una suscripción en el plan que no esté cancelada. Hasta octubre de 2026 esta ruta archivaba el plan, o eliminaba un borrador con `?hard=true` (el parámetro ya no existe) — archivar es `POST /v1/plans/{id}/archive`, reversible.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plan eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plans archive

Archivar plan

```text
kobana billing plans archive [ID] [OPCIONES]
```

`POST /v1/plans/{id}/archive`

Archiva el plan: sigue listado y puede restaurarse, pero no acepta nuevas suscripciones; las existentes siguen cobrando. Devuelve 422 mientras haya suscripciones activas en el plan. Exige el nivel de acceso `write`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plan archivado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plans duplicate

Duplicar plan

```text
kobana billing plans duplicate [ID] [OPCIONES]
```

`POST /v1/plans/{id}/duplicate`

Crea una copia del plan (incluyendo sus ítems) en estado de borrador.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Plan duplicado. |
| `404` | Recurso no encontrado en la organización actual. |
