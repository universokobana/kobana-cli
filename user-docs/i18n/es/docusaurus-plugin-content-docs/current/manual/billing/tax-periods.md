---
id: tax-periods
title: "kobana billing tax-periods"
sidebar_label: "tax-periods"
---

# kobana billing tax-periods


Comandos de `kobana billing tax-periods`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`tax-periods list`](#tax-periods-list) | `GET /v1/tax_periods` | Listar períodos tributarios |
| [`tax-periods create`](#tax-periods-create) | `POST /v1/tax_periods` | Procesar período tributario mensual |
| [`tax-periods get`](#tax-periods-get) | `GET /v1/tax_periods/{id}` | Obtener período tributario |
| [`tax-periods update`](#tax-periods-update) | `PATCH /v1/tax_periods/{id}` | Ejecutar acción en el período tributario |
| [`tax-periods delete`](#tax-periods-delete) | `DELETE /v1/tax_periods/{id}` | Eliminar período tributario |

## tax-periods list

Listar períodos tributarios

```text
kobana billing tax-periods list [OPCIONES]
```

`GET /v1/tax_periods`

Devuelve una lista paginada de los períodos tributarios (mensuales o trimestrales) de la organización. Permite filtrar por empresa, tipo de período, año y estado.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `company_id` | `string` | no | Filtra por empresa emisora. |
| `period_type` | `string` | no | Filtra por tipo de período: `monthly` o `quarterly`. Valores: `monthly`, `quarterly`. |
| `year` | `string` | no | Filtra por año de competencia (ej.: `2026`). |
| `status` | `string` | no | Filtra por estado: `open`, `calculated` o `closed`. Valores: `open`, `calculated`, `closed`. |
| `month` | `integer` | no | Filtra por mes de competencia (1 a 12). |
| `quarter` | `integer` | no | Filtra por trimestre de competencia (1 a 4). |
| `gross_revenue_min` | `number,null` | no | Ingreso bruto mínimo del período, en BRL (ej.: `1000.00`). |
| `gross_revenue_max` | `number,null` | no | Ingreso bruto máximo del período, en BRL (ej.: `50000.00`). |
| `das_estimated_min` | `number,null` | no | DAS estimado mínimo del período, en BRL (ej.: `100.00`). |
| `das_estimated_max` | `number,null` | no | DAS estimado máximo del período, en BRL (ej.: `5000.00`). |
| `closed_after` | `string` | no | Devuelve períodos cerrados desde esta fecha (formato `YYYY-MM-DD`). |
| `closed_before` | `string` | no | Devuelve períodos cerrados hasta esta fecha (formato `YYYY-MM-DD`). |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de períodos tributarios. |
| `404` | Recurso no encontrado en la organización actual. |

## tax-periods create

Procesar período tributario mensual

```text
kobana billing tax-periods create [OPCIONES]
```

`POST /v1/tax_periods`

Crea (o recupera) el período tributario mensual indicado y calcula los impuestos debidos (ISS, PIS, COFINS, IRPJ, CSLL, CBS, IBS) con base en las NFes emitidas y las retenciones a compensar. Cuando se omite `company_id`, usa la empresa por defecto de la organización.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `year` | `integer` | sí | Año de competencia (ej.: `2026`). |
| `month` | `integer` | sí | Mes de competencia (1 a 12). |
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora. Omita para usar la empresa por defecto de la organización. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.tax-periods.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Período tributario procesado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## tax-periods get

Obtener período tributario

```text
kobana billing tax-periods get [ID] [OPCIONES]
```

`GET /v1/tax_periods/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del período tributario. |
| `404` | Recurso no encontrado en la organización actual. |

## tax-periods update

Ejecutar acción en el período tributario

```text
kobana billing tax-periods update [ID] [OPCIONES]
```

`PATCH /v1/tax_periods/{id}`

Aplica una acción sobre el período: `recalculate` recalcula los impuestos (solo períodos mensuales), `close` cierra el período y `reopen` lo reabre.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `action` | `string` | sí | Acción a ejecutar: `recalculate`, `close` o `reopen`. Valores: `recalculate`, `close`, `reopen`. |

Otros campos y objetos anidados: `kobana schema billing.tax-periods.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Período tributario actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## tax-periods delete

Eliminar período tributario

```text
kobana billing tax-periods delete [ID] [OPCIONES]
```

`DELETE /v1/tax_periods/{id}`

Elimina permanentemente el período tributario. Use con precaución — los períodos cerrados normalmente no deben eliminarse.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Período tributario eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
