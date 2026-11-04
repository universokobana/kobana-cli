---
id: protests
title: "kobana dunning protests"
sidebar_label: "protests"
---

# kobana dunning protests


Comandos de `kobana dunning protests`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`protests list`](#protests-list) | `GET /v1/protests` | Listar protestos |
| [`protests create`](#protests-create) | `POST /v1/protests` | Crear protesto |
| [`protests get`](#protests-get) | `GET /v1/protests/{id}` | Consultar protesto |
| [`protests update`](#protests-update) | `PUT /v1/protests/{id}` | Actualizar protesto |
| [`protests cancel`](#protests-cancel) | `POST /v1/protests/{id}/cancel` | Solicitar cancelación del protesto |

## protests list

Listar protestos

```text
kobana dunning protests list [OPCIONES]
```

`GET /v1/protests`

Lista los protestos de la organización con paginación y filtros. Usa `include=person,charge` para incluir los resúmenes de cliente y cobro.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `sort_by` | `string` | no | Campo de ordenación (`created_at` \| `updated_at` \| `amount` \| `protest_date` \| `sent_at`). Valores: `createdAt`, `updatedAt`, `amount`, `protestDate`, `sentAt`. |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra por estado (`pending` \| `sent` \| `intimated` \| `protested` \| `paid` \| `cancelled`). Valores: `pending`, `sent`, `intimated`, `protested`, `paid`, `cancelled`. |
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
| `200` | Lista paginada de protestos. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## protests create

Crear protesto

```text
kobana dunning protests create [OPCIONES]
```

`POST /v1/protests`

Registra un protesto para un cobro. El registro nace `pending` y SOLO se envía a la notaría tras la revisión y aprobación humana (flujo legal; evento `protest.review_required`). Devuelve 409 si ya existe un protesto activo (`pending`/`sent`/`intimated`/`protested`) para el mismo cobro.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_id` | `string (uuid)` | sí | Cliente protestado (UUID). |
| `charge_id` | `string (uuid)` | sí | Cobro protestado (UUID). |
| `amount` | `number` | sí | Monto a protestar (número decimal positivo). |
| `notary` | `string` | no | Nombre de la notaría de protesto. |
| `notary_code` | `string` | no | Código/identificador de la notaría. |
| `fees` | `number` | no | Aranceles/costas de la notaría (número decimal). |
| `external_id` | `string` | no | Identificador en TU sistema o protocolo en la notaría. |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.protests.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Protesto creado (esperando revisión humana). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## protests get

Consultar protesto

```text
kobana dunning protests get [ID] [OPCIONES]
```

`GET /v1/protests/{id}`

Devuelve un protesto por ID, con los resúmenes de cliente y cobro.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Protesto encontrado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## protests update

Actualizar protesto

```text
kobana dunning protests update [ID] [OPCIONES]
```

`PUT /v1/protests/{id}`

Actualiza un protesto (parcial). Es la etapa de aprobación del flujo legal: exige el permiso de revisión (`approve`) y es donde el estado avanza de `pending` a `sent`/`intimated`/`protested` tras la revisión humana.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Estado (`pending` = esperando revisión humana obligatoria, `sent` = enviado a la notaría, `intimated` = deudor intimado, `protested` = protestado, `paid` = pagado, `cancelled` = cancelado). Valores: `pending`, `sent`, `intimated`, `protested`, `paid`, `cancelled`. |
| `notary` | `string` | no | Nombre de la notaría de protesto. |
| `notary_code` | `string` | no | Código/identificador de la notaría. |
| `fees` | `number` | no | Aranceles/costas de la notaría (número decimal). |
| `external_id` | `string` | no | Identificador en TU sistema o protocolo en la notaría. |
| `sent_at` | `string (date-time)` | no | Fecha de envío a la notaría (ISO 8601). |
| `intimation_date` | `string (date-time)` | no | Fecha de intimación del deudor (ISO 8601). |
| `protest_date` | `string (date-time)` | no | Fecha de registro del protesto (ISO 8601). |
| `payment_date` | `string (date-time)` | no | Fecha de pago en la notaría (ISO 8601). |
| `cancellation_date` | `string (date-time)` | no | Fecha de cancelación del protesto (ISO 8601). |
| `response_data` | `object` | no | Respuesta cruda de la notaría (payload de la integración). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.protests.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Protesto actualizado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## protests cancel

Solicitar cancelación del protesto

```text
kobana dunning protests cancel [ID] [OPCIONES]
```

`POST /v1/protests/{id}/cancel`

Solicita la cancelación de un protesto. No se permite para protestos ya `cancelled` ni `paid`; el estado pasa a `cancelled` y el motivo (si se envía) queda en `responseData.cancellationReason`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cancellation_reason` | `string` | no | Motivo de la cancelación. |

Otros campos y objetos anidados: `kobana schema dunning.protests.cancel`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cancelación registrada; protesto actualizado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
