---
id: tax-rules
title: "kobana billing tax-rules"
sidebar_label: "tax-rules"
---

# kobana billing tax-rules


Comandos de `kobana billing tax-rules`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`tax-rules list`](#tax-rules-list) | `GET /v1/tax_rules` | Listar reglas tributarias |
| [`tax-rules create`](#tax-rules-create) | `POST /v1/tax_rules` | Crear regla tributaria |
| [`tax-rules update`](#tax-rules-update) | `PUT /v1/tax_rules` | Crear o actualizar regla fiscal |
| [`tax-rules get`](#tax-rules-get) | `GET /v1/tax_rules/{id}` | Obtener regla tributaria |
| [`tax-rules delete`](#tax-rules-delete) | `DELETE /v1/tax_rules/{id}` | Eliminar regla tributaria |

## tax-rules list

Listar reglas tributarias

```text
kobana billing tax-rules list [OPCIONES]
```

`GET /v1/tax_rules`

Devuelve una lista paginada de las reglas tributarias de la organización actual. Usa los filtros `scope`, `company_id`, `service_item_id`, `billing_account_id` e `is_active` para acotar el resultado.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `scope` | `string` | no | Filtra por alcance: `global`, `organization`, `company`, `service`, `customer` o `municipality`. Valores: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `company_id` | `string` | no | Filtra por las reglas de una empresa específica. |
| `service_item_id` | `string` | no | Filtra por las reglas de un ítem de servicio específico. |
| `billing_account_id` | `string` | no | Filtra por las reglas de una cuenta de cobro específica. |
| `is_active` | `string` | no | Cuando es `true`, devuelve solo reglas activas; cuando es `false`, solo inactivas. Valores: `true`, `false`. |
| `search` | `string` | no | Búsqueda textual en `name` y `description`. |
| `customer_tax_type` | `string` | no | Filtra por tipo tributario del cliente: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` o `entidade_publica`. |
| `operation_type` | `string` | no | Filtra por tipo de operación: `interno` (mismo municipio) o `externo` (municipio diferente). |
| `municipality` | `string` | no | Filtra por el código IBGE del municipio (7 dígitos). |
| `priority_from` | `integer,null` | no | Prioridad mínima (inclusiva) para filtrar reglas. |
| `priority_to` | `integer,null` | no | Prioridad máxima (inclusiva) para filtrar reglas. |
| `effective_from_from` | `string` | no | Devuelve reglas cuya fecha de inicio de vigencia sea igual o posterior a este valor (ISO 8601). |
| `effective_from_to` | `string` | no | Devuelve reglas cuya fecha de inicio de vigencia sea igual o anterior a este valor (ISO 8601). |
| `effective_to_from` | `string` | no | Devuelve reglas cuya fecha de fin de vigencia sea igual o posterior a este valor (ISO 8601). |
| `effective_to_to` | `string` | no | Devuelve reglas cuya fecha de fin de vigencia sea igual o anterior a este valor (ISO 8601). |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de reglas tributarias. |
| `404` | Recurso no encontrado en la organización actual. |

## tax-rules create

Crear regla tributaria

```text
kobana billing tax-rules create [OPCIONES]
```

`POST /v1/tax_rules`

Crea una nueva regla tributaria con sus ítems (alícuotas por tributo). El `scope` define el nivel de aplicación y la `priority` define la precedencia — las reglas de mayor prioridad sobrescriben a las menores. Es obligatorio informar al menos un ítem en `items`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre legible de la regla. |
| `scope` | `string` | sí | Alcance de la regla: `global`, `organization`, `company`, `service`, `customer` o `municipality`. Valores: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `items` | `array<object>` | sí | Lista de ítems (alícuotas por tributo). Se requiere al menos 1 ítem. |
| `description` | `string` | no | Descripción opcional de la regla. |
| `priority` | `integer` | no | Prioridad de la regla. Mayor valor sobrescribe reglas de menor prioridad. Por defecto: `0`. |
| `company_id` | `string (uuid)` | no | UUID de la empresa cuando `scope = company`. La empresa debe pertenecer a la organización actual. |
| `service_item_id` | `string (uuid)` | no | UUID del ítem de servicio cuando `scope = service`. El ítem debe pertenecer a la organización actual. |
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro cuando `scope = customer`. La cuenta debe pertenecer a la organización actual. |
| `customer_tax_type` | `string` | no | Tipo tributario del cliente: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` o `entidade_publica`. Valores: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante`, `entidade_publica`. |
| `municipality` | `string` | no | Código IBGE del municipio (7 dígitos) cuando `scope = municipality`. |
| `operation_type` | `string` | no | Tipo de operación: `interno` (mismo municipio) o `externo` (municipio diferente). Valores: `interno`, `externo`. |
| `effective_from` | `string (date-time)` | no | Fecha/hora ISO 8601 en que la regla entra en vigor. |
| `effective_to` | `string (date-time)` | no | Fecha/hora ISO 8601 en que la regla deja de regir. `null` significa vigencia indeterminada. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.tax-rules.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Regla tributaria creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## tax-rules update

Crear o actualizar regla fiscal

```text
kobana billing tax-rules update [OPCIONES]
```

`PUT /v1/tax_rules`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre legible de la regla. |
| `description` | `string` | no | Descripción opcional de la regla. |
| `scope` | `string` | no | Alcance de la regla: `global`, `organization`, `company`, `service`, `customer` o `municipality`. Valores: `global`, `organization`, `company`, `service`, `customer`, `municipality`. |
| `priority` | `integer` | no | Prioridad de la regla. Mayor valor sobrescribe reglas de menor prioridad. Por defecto: `0`. |
| `company_id` | `string (uuid)` | no | UUID de la empresa cuando `scope = company`. La empresa debe pertenecer a la organización actual. |
| `service_item_id` | `string (uuid)` | no | UUID del ítem de servicio cuando `scope = service`. El ítem debe pertenecer a la organización actual. |
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro cuando `scope = customer`. La cuenta debe pertenecer a la organización actual. |
| `customer_tax_type` | `string` | no | Tipo tributario del cliente: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante` o `entidade_publica`. Valores: `pessoa_fisica`, `pessoa_juridica`, `mei`, `simples_optante`, `entidade_publica`. |
| `municipality` | `string` | no | Código IBGE del municipio (7 dígitos) cuando `scope = municipality`. |
| `operation_type` | `string` | no | Tipo de operación: `interno` (mismo municipio) o `externo` (municipio diferente). Valores: `interno`, `externo`. |
| `effective_from` | `string (date-time)` | no | Fecha/hora ISO 8601 en que la regla entra en vigor. |
| `effective_to` | `string (date-time)` | no | Fecha/hora ISO 8601 en que la regla deja de regir. `null` significa vigencia indeterminada. |
| `items` | `array<object>` | no | Lista de ítems (alícuotas por tributo). Se requiere al menos 1 ítem. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.tax-rules.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## tax-rules get

Obtener regla tributaria

```text
kobana billing tax-rules get [ID] [OPCIONES]
```

`GET /v1/tax_rules/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la regla tributaria. |
| `404` | Recurso no encontrado en la organización actual. |

## tax-rules delete

Eliminar regla tributaria

```text
kobana billing tax-rules delete [ID] [OPCIONES]
```

`DELETE /v1/tax_rules/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Regla tributaria eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
