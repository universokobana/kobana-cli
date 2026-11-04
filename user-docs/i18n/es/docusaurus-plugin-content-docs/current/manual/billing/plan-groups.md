---
id: plan-groups
title: "kobana billing plan-groups"
sidebar_label: "plan-groups"
---

# kobana billing plan-groups


Comandos de `kobana billing plan-groups`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`plan-groups list`](#plan-groups-list) | `GET /v1/plan_groups` | Listar grupos de planes |
| [`plan-groups create`](#plan-groups-create) | `POST /v1/plan_groups` | Crear grupo de planes |
| [`plan-groups update`](#plan-groups-update) | `PUT /v1/plan_groups` | Crear o actualizar grupo de planes |
| [`plan-groups reorder`](#plan-groups-reorder) | `POST /v1/plan_groups/reorder` | Reordenar grupos de planes |
| [`plan-groups get`](#plan-groups-get) | `GET /v1/plan_groups/{id}` | Obtener grupo de planes |
| [`plan-groups delete`](#plan-groups-delete) | `DELETE /v1/plan_groups/{id}` | Eliminar grupo de planes |
| [`plan-groups archive`](#plan-groups-archive) | `POST /v1/plan_groups/{id}/archive` | Archivar grupo de planes |

## plan-groups list

Listar grupos de planes

```text
kobana billing plan-groups list [OPCIONES]
```

`GET /v1/plan_groups`

Devuelve todos los grupos de planes de la organización actual ordenados por `display_order`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda textual libre por nombre o slug. |
| `include_archived` | `string` | no | Cuando es `true`, incluye grupos archivados en la lista. Valores: `true`, `false`. |
| `company_id` | `string` | no | Filtra grupos de planes por la empresa indicada (UUID). |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `archived_after` | `string` | no | Devuelve solo grupos archivados a partir de esta fecha (YYYY-MM-DD). |
| `archived_before` | `string` | no | Devuelve solo grupos archivados hasta esta fecha (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de grupos de planes. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-groups create

Crear grupo de planes

```text
kobana billing plan-groups create [OPCIONES]
```

`POST /v1/plan_groups`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del grupo de planes. |
| `slug` | `string` | no | Identificador legible (opcional). Generado automáticamente si se omite. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.plan-groups.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Grupo de planes creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plan-groups update

Crear o actualizar grupo de planes

```text
kobana billing plan-groups update [OPCIONES]
```

`PUT /v1/plan_groups`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre del grupo de planes. |
| `slug` | `string` | no | Identificador legible (opcional). Generado automáticamente si se omite. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.plan-groups.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plan-groups reorder

Reordenar grupos de planes

```text
kobana billing plan-groups reorder [OPCIONES]
```

`POST /v1/plan_groups/reorder`

Actualiza el `display_order` de los grupos según el orden del array `group_ids`. Todos los grupos deben pertenecer a la organización actual.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `group_ids` | `array<string (uuid)>` | sí | Lista de UUIDs de los grupos en el nuevo orden de visualización. Todos deben pertenecer a la organización actual. |

Otros campos y objetos anidados: `kobana schema billing.plan-groups.reorder`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Orden actualizado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## plan-groups get

Obtener grupo de planes

```text
kobana billing plan-groups get [ID] [OPCIONES]
```

`GET /v1/plan_groups/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del grupo de planes. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-groups delete

Eliminar grupo de planes

```text
kobana billing plan-groups delete [ID] [OPCIONES]
```

`DELETE /v1/plan_groups/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Confirmación de eliminación. |
| `404` | Recurso no encontrado en la organización actual. |

## plan-groups archive

Archivar grupo de planes

```text
kobana billing plan-groups archive [ID] [OPCIONES]
```

`POST /v1/plan_groups/{id}/archive`

Marca el grupo como archivado completando `archived_at`. Los grupos archivados se ocultan de los listados por defecto.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Grupo de planes archivado. |
| `404` | Recurso no encontrado en la organización actual. |
