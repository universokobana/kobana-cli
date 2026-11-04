---
id: service-items
title: "kobana billing service-items"
sidebar_label: "service-items"
---

# kobana billing service-items


Comandos de `kobana billing service-items`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`service-items list`](#service-items-list) | `GET /v1/service_items` | Listar ítems de servicio |
| [`service-items create`](#service-items-create) | `POST /v1/service_items` | Crear ítem de servicio |
| [`service-items update`](#service-items-update) | `PUT /v1/service_items` | Crear o actualizar ítem de servicio |
| [`service-items get`](#service-items-get) | `GET /v1/service_items/{id}` | Obtener ítem de servicio |
| [`service-items delete`](#service-items-delete) | `DELETE /v1/service_items/{id}` | Eliminar ítem de servicio |

## service-items list

Listar ítems de servicio

```text
kobana billing service-items list [OPCIONES]
```

`GET /v1/service_items`

Devuelve una lista paginada de los ítems de servicio (catálogo LC 116/2003) de la organización actual. Usa los filtros para restringir por empresa, código LC 116, NBS, clasificación tributaria, indicador de operación o anexo del Simples.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `company_id` | `string` | no | Filtra por UUID de la empresa emisora asociada al ítem. |
| `search` | `string` | no | Búsqueda textual libre en `name`, `internal_code` y `description`. |
| `is_active` | `string` | no | Filtra por estado: `true` devuelve solo ítems activos; `false`, solo inactivos. Valores: `true`, `false`. |
| `lc116_code` | `string` | no | Filtra por el código LC 116/2003 (p. ej. `01.05`). |
| `nbs_code` | `string` | no | Filtra por el código NBS (Nomenclatura Brasileña de Servicios). |
| `c_class_trib` | `string` | no | Filtra por la clasificación tributaria IBS/CBS (`c_class_trib`). |
| `indop_code` | `string` | no | Filtra por el Indicador de Operación (`indop_code`) usado en la NFS-e SNNFSE. |
| `simples_annex` | `string` | no | Filtra por el anexo del Simples Nacional aplicable: `annex_iii`, `annex_iv` o `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `cnae_code` | `string` | no | Filtra por el código CNAE relacionado con el servicio. |
| `municipal_code` | `string` | no | Filtra por el código municipal del servicio. |
| `internal_code` | `string` | no | Filtra por el código interno del ítem. |
| `cbs_ibs_category` | `string` | no | Filtra por la categoría CBS/IBS del ítem según la reforma tributaria. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de ítems de servicio. |
| `404` | Recurso no encontrado en la organización actual. |

## service-items create

Crear ítem de servicio

```text
kobana billing service-items create [OPCIONES]
```

`POST /v1/service_items`

Crea un ítem de servicio en el catálogo LC 116/2003. `internal_code` es el identificador interno y `lc116_code` debe seguir el catálogo de la Ley Complementaria 116/2003 (p. ej. `01.05`). Cuando se informa `company_id`, la empresa debe pertenecer a la organización autenticada.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del ítem de servicio. |
| `internal_code` | `string` | sí | Código interno del ítem — inmutable después de la creación. |
| `lc116_code` | `string` | sí | Código LC 116/2003 del servicio (p. ej. `01.05`). |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora asociada. Omite o envía `null` para un ítem aplicable a todas las empresas. |
| `description` | `string` | no | Descripción opcional. |
| `municipal_code` | `string` | no | Código municipal del servicio, cuando corresponda. |
| `nbs_code` | `string` | no | Código NBS. |
| `cnae_code` | `string` | no | Código CNAE relacionado. |
| `simples_annex` | `string` | no | Anexo del Simples Nacional aplicable: `annex_iii`, `annex_iv` o `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `default_iss_rate` | `number` | no | Alícuota predeterminada de ISS en puntos porcentuales (p. ej. `2.5`). |
| `c_class_trib` | `string` | no | Clasificación tributaria CST IBS/CBS. |
| `cbs_ibs_category` | `string` | no | Categoría CBS/IBS del ítem. |
| `indop_code` | `string` | no | Indicador de Operación (`indOp`) de la NFS-e SNNFSE. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.service-items.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Ítem de servicio creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## service-items update

Crear o actualizar ítem de servicio

```text
kobana billing service-items update [OPCIONES]
```

`PUT /v1/service_items`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora asociada. Omite o envía `null` para un ítem aplicable a todas las empresas. |
| `name` | `string` | no | Nombre del ítem de servicio. |
| `description` | `string` | no | Descripción opcional. |
| `internal_code` | `string` | no | Código interno del ítem — inmutable después de la creación. |
| `lc116_code` | `string` | no | Código LC 116/2003 del servicio (p. ej. `01.05`). |
| `municipal_code` | `string` | no | Código municipal del servicio, cuando corresponda. |
| `nbs_code` | `string` | no | Código NBS. |
| `cnae_code` | `string` | no | Código CNAE relacionado. |
| `simples_annex` | `string` | no | Anexo del Simples Nacional aplicable: `annex_iii`, `annex_iv` o `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `default_iss_rate` | `number` | no | Alícuota predeterminada de ISS en puntos porcentuales (p. ej. `2.5`). |
| `c_class_trib` | `string` | no | Clasificación tributaria CST IBS/CBS. |
| `cbs_ibs_category` | `string` | no | Categoría CBS/IBS del ítem. |
| `indop_code` | `string` | no | Indicador de Operación (`indOp`) de la NFS-e SNNFSE. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.service-items.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## service-items get

Obtener ítem de servicio

```text
kobana billing service-items get [ID] [OPCIONES]
```

`GET /v1/service_items/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del ítem de servicio. |
| `404` | Recurso no encontrado en la organización actual. |

## service-items delete

Eliminar ítem de servicio

```text
kobana billing service-items delete [ID] [OPCIONES]
```

`DELETE /v1/service_items/{id}`

Elimina el ítem de servicio (soft delete: `deleted_at` — desaparece de toda lectura; las reglas tributarias, productos y NFS-e emitidas que lo citan guardan la referencia). Exige el nivel de acceso `delete`. Para solo dejar de usarlo, desactívalo (`PATCH` con `is_active=false`). Hasta octubre de 2026 esta ruta eliminaba el ítem definitivamente, o solo lo desactivaba cuando una regla tributaria lo usaba.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Ítem de servicio eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
