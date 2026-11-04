---
id: disputes
title: "kobana dunning disputes"
sidebar_label: "disputes"
---

# kobana dunning disputes


Comandos de `kobana dunning disputes`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`disputes list`](#disputes-list) | `GET /v1/disputes` | Listar disputas |
| [`disputes create`](#disputes-create) | `POST /v1/disputes` | Abrir disputa |
| [`disputes get`](#disputes-get) | `GET /v1/disputes/{id}` | Consultar disputa |
| [`disputes update`](#disputes-update) | `PATCH /v1/disputes/{id}` | Ejecutar acción en la disputa |

## disputes list

Listar disputas

```text
kobana dunning disputes list [OPCIONES]
```

`GET /v1/disputes`

Lista las disputas de la organización, ordenadas por apertura (la más reciente primero). No admite ordenación personalizada.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `status` | `string` | no | Filtra por estado (`open` \| `under_review` \| `resolved_valid` \| `resolved_invalid` \| `canceled`). Valores: `open`, `under_review`, `resolved_valid`, `resolved_invalid`, `canceled`. |
| `charge_id` | `string` | no | Filtra por cobro (UUID). |
| `person_id` | `string` | no | Filtra por cliente (UUID). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de disputas. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## disputes create

Abrir disputa

```text
kobana dunning disputes create [OPCIONES]
```

`POST /v1/disputes`

Abre una disputa sobre un cobro y pausa la regla de inmediato. Si el cobro ya tiene una disputa abierta o en revisión, devuelve la existente (no duplica). Admite `Idempotency-Key`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `charge_id` | `string (uuid)` | sí | Cobro en disputa. |
| `type` | `string` | sí | Tipo de impugnación (`already_paid` \| `not_recognized` \| `wrong_amount` \| `service_not_provided` \| `incorrect_invoice` \| `wrong_recipient` \| `other`). Valores: `already_paid`, `not_recognized`, `wrong_amount`, `service_not_provided`, `incorrect_invoice`, `wrong_recipient`, `other`. |
| `reason` | `string` | no | Relato del deudor/operador. |
| `disputed_amount` | `number` | no | Importe impugnado (número). Omite para impugnar el importe total. |
| `documents` | `array<object>` | no | Adjuntos/referencias de evidencia. |
| `assigned_to_id` | `string (uuid)` | no | Usuario responsable de la revisión (debe pertenecer a la organización). |
| `sla_days` | `integer` | no | Días de SLA para resolver (máx. 60; valor por defecto del sistema si se omite). |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |

Otros campos y objetos anidados: `kobana schema dunning.disputes.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Disputa abierta (o la disputa abierta existente del cobro). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## disputes get

Consultar disputa

```text
kobana dunning disputes get [ID] [OPCIONES]
```

`GET /v1/disputes/{id}`

Devuelve una disputa por ID, con el cobro completo, el cliente y el responsable.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Disputa encontrada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## disputes update

Ejecutar acción en la disputa

```text
kobana dunning disputes update [ID] [OPCIONES]
```

`PATCH /v1/disputes/{id}`

Ejecuta una acción de flujo en la disputa: `start_review`, `resolve` o `cancel`. Las transiciones inválidas (p. ej., `start_review` cuando el estado no es `open`) devuelven 409.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema dunning.disputes.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Disputa actualizada por la acción. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
