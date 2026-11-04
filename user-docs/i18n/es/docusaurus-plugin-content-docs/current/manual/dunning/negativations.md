---
id: negativations
title: "kobana dunning negativations"
sidebar_label: "negativations"
---

# kobana dunning negativations


Comandos de `kobana dunning negativations`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`negativations list`](#negativations-list) | `GET /v1/negativations` | Listar negativaciones |
| [`negativations create`](#negativations-create) | `POST /v1/negativations` | Crear negativación |
| [`negativations get`](#negativations-get) | `GET /v1/negativations/{id}` | Consultar negativación |
| [`negativations update`](#negativations-update) | `PUT /v1/negativations/{id}` | Actualizar negativación |
| [`negativations remove`](#negativations-remove) | `POST /v1/negativations/{id}/remove` | Solicitar baja de la negativación |

## negativations list

Listar negativaciones

```text
kobana dunning negativations list [OPCIONES]
```

`GET /v1/negativations`

Lista las negativaciones de la organización con paginación y filtros. Usa `include=person,charge` para incluir los resúmenes de cliente y cobro.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `sort_by` | `string` | no | Campo de ordenación (`created_at` \| `updated_at` \| `amount` \| `registered_at`). Valores: `createdAt`, `updatedAt`, `amount`, `registeredAt`. |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra por estado (`pending` \| `active` \| `removed` \| `failed`). Valores: `pending`, `active`, `removed`, `failed`. |
| `bureau` | `string` | no | Filtra por buró de crédito (`serasa` \| `spc` \| `boa_vista`). Valores: `serasa`, `spc`, `boa_vista`. |
| `person_id` | `string` | no | Filtra por cliente (UUID). |
| `charge_id` | `string` | no | Filtra por cobro (UUID). |
| `include` | `string` | no | Relaciones a incluir, separadas por coma (`person`, `charge`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de negativaciones. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## negativations create

Crear negativación

```text
kobana dunning negativations create [OPCIONES]
```

`POST /v1/negativations`

Registra una negativación para un cobro. El registro nace `pending` y SOLO se envía al buró tras la revisión y aprobación humana (flujo legal; evento `negativation.review_required`). Devuelve 409 si ya existe una negativación `pending`/`active` para el mismo cobro en el mismo buró.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_id` | `string (uuid)` | sí | Cliente negativado (UUID). |
| `charge_id` | `string (uuid)` | sí | Cobro negativado (UUID). |
| `bureau` | `string` | sí | Buró de crédito (`serasa` \| `spc` \| `boa_vista`). Valores: `serasa`, `spc`, `boa_vista`. |
| `amount` | `number` | sí | Monto a negativar (número decimal positivo). |
| `external_id` | `string` | no | Identificador en TU sistema o protocolo en el buró. |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.negativations.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Negativación creada (esperando revisión humana). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## negativations get

Consultar negativación

```text
kobana dunning negativations get [ID] [OPCIONES]
```

`GET /v1/negativations/{id}`

Devuelve una negativación por ID, con los resúmenes de cliente y cobro.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Negativación encontrada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## negativations update

Actualizar negativación

```text
kobana dunning negativations update [ID] [OPCIONES]
```

`PUT /v1/negativations/{id}`

Actualiza una negativación (parcial). Es la etapa de aprobación del flujo legal: exige el permiso de revisión (`approve`) y es donde el estado pasa de `pending` a `active` tras la revisión humana.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Estado (`pending` = esperando revisión humana obligatoria, `active` = efectivada en el buró, `removed` = dada de baja, `failed` = falló). Valores: `pending`, `active`, `removed`, `failed`. |
| `external_id` | `string` | no | Identificador en TU sistema o protocolo en el buró. |
| `registered_at` | `string (date-time)` | no | Fecha en que la negativación se efectivó en el buró (ISO 8601). |
| `removed_at` | `string (date-time)` | no | Fecha de la baja en el buró (ISO 8601). |
| `removal_reason` | `string` | no | Motivo de la baja de la negativación. |
| `response_data` | `object` | no | Respuesta cruda del buró (payload de la integración). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.negativations.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Negativación actualizada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## negativations remove

Solicitar baja de la negativación

```text
kobana dunning negativations remove [ID] [OPCIONES]
```

`POST /v1/negativations/{id}/remove`

Solicita la baja de una negativación en el buró. Solo se permite para registros `active`; el estado pasa a `removed` y el motivo (si se envía) queda en `removal_reason`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `removal_reason` | `string` | no | Motivo de la baja de la negativación. |

Otros campos y objetos anidados: `kobana schema dunning.negativations.remove`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Baja registrada; negativación actualizada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
