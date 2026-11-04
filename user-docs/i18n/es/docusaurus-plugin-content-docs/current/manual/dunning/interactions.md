---
id: interactions
title: "kobana dunning interactions"
sidebar_label: "interactions"
---

# kobana dunning interactions


Comandos de `kobana dunning interactions`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`interactions list`](#interactions-list) | `GET /v1/interactions` | Listar interacciones |
| [`interactions create`](#interactions-create) | `POST /v1/interactions` | Registrar interacción |
| [`interactions get`](#interactions-get) | `GET /v1/interactions/{id}` | Consultar interacción |
| [`interactions update`](#interactions-update) | `PUT /v1/interactions/{id}` | Actualizar interacción |
| [`interactions delete`](#interactions-delete) | `DELETE /v1/interactions/{id}` | Eliminar interacción |

## interactions list

Listar interacciones

```text
kobana dunning interactions list [OPCIONES]
```

`GET /v1/interactions`

Lista las interacciones de la organización con paginación y filtros. Ordenable vía `sort_by` (`contacted_at` | `interaction_type` | `created_at` | `updated_at`; por defecto `contacted_at` desc).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `sort_by` | `string` | no | Campo de ordenación (nombre público en snake_case, ej.: `due_date`). |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `interaction_type` | `string` | no | Filtra por tipo (`call` \| `email` \| `whatsapp` \| `meeting` \| `note`). Valores: `call`, `email`, `whatsapp`, `meeting`, `note`. |
| `direction` | `string` | no | Filtra por dirección (`inbound` \| `outbound`). Valores: `inbound`, `outbound`. |
| `outcome` | `string` | no | Filtra por resultado (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Valores: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`. |
| `person_id` | `string` | no | Filtra por cliente (UUID). |
| `charge_id` | `string` | no | Filtra por cobro (UUID). |
| `include` | `string` | no | Relaciones a incluir, separadas por comas (`person`, `user`, `charge`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de interacciones. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## interactions create

Registrar interacción

```text
kobana dunning interactions create [OPCIONES]
```

`POST /v1/interactions`

Registra una interacción con un cliente, opcionalmente vinculada a un cobro. Admite `Idempotency-Key`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_id` | `string (uuid)` | sí | Cliente de la interacción. |
| `interaction_type` | `string` | sí | Tipo (`call` \| `email` \| `whatsapp` \| `meeting` \| `note`). Valores: `call`, `email`, `whatsapp`, `meeting`, `note`. |
| `direction` | `string` | sí | Dirección (`inbound` = el cliente contactó, `outbound` = contactamos al cliente). Valores: `inbound`, `outbound`. |
| `charge_id` | `string (uuid)` | no | Cobro relacionado con la interacción. |
| `summary` | `string` | no | Resumen/notas de la interacción. |
| `outcome` | `string` | no | Resultado (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Valores: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`, `null`. |
| `promise_date` | `string (date-time)` | no | Fecha prometida de pago. |
| `promised_amount` | `number` | no | Importe prometido (número positivo). |
| `contacted_at` | `string (date-time)` | no | Momento del contacto (ISO 8601; por defecto: ahora). |
| `duration_seconds` | `integer` | no | Duración del contacto en segundos. |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.interactions.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Interacción registrada (incluye los resúmenes de cliente, usuario y cobro). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## interactions get

Consultar interacción

```text
kobana dunning interactions get [ID] [OPCIONES]
```

`GET /v1/interactions/{id}`

Devuelve una interacción por ID, con resúmenes de cliente, usuario y cobro.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Interacción encontrada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## interactions update

Actualizar interacción

```text
kobana dunning interactions update [ID] [OPCIONES]
```

`PUT /v1/interactions/{id}`

Actualiza los campos editables de una interacción (parcial).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `summary` | `string` | no | Resumen/notas de la interacción. |
| `notes` | `string` | no | Alias de `summary` (se guarda en el mismo campo; si se envían ambos, `notes` prevalece). |
| `outcome` | `string` | no | Resultado (`promise_to_pay` \| `negotiation` \| `dispute` \| `no_contact` \| `callback_requested` \| `payment_confirmed`). Valores: `promise_to_pay`, `negotiation`, `dispute`, `no_contact`, `callback_requested`, `payment_confirmed`, `null`. |
| `promise_date` | `string (date-time)` | no | Fecha prometida de pago. |
| `promised_amount` | `number` | no | Importe prometido (número positivo). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_data_mode` | `string` | no | Cómo se aplica `custom_data` en el update: `replace` (default) sobrescribe el objeto entero; `merge` fusiona (deep) el payload sobre el valor almacenado — los objetos anidados se fusionan, escalares/arrays/null sobrescriben la clave. Valores: `replace`, `merge`. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.interactions.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Interacción actualizada. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## interactions delete

Eliminar interacción

```text
kobana dunning interactions delete [ID] [OPCIONES]
```

`DELETE /v1/interactions/{id}`

Elimina (borrado lógico) una interacción; el registro permanece para la pista de auditoría.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Interacción eliminada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
