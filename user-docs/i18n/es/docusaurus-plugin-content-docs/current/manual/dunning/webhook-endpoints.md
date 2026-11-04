---
id: webhook-endpoints
title: "kobana dunning webhook-endpoints"
sidebar_label: "webhook-endpoints"
---

# kobana dunning webhook-endpoints


Comandos de `kobana dunning webhook-endpoints`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`webhook-endpoints deliveries retry`](#webhook-endpoints-deliveries-retry) | `POST /v1/webhook-endpoints/{id}/deliveries/{deliveryId}/retry` | Reenviar entrega |
| [`webhook-endpoints deliveries list`](#webhook-endpoints-deliveries-list) | `GET /v1/webhook-endpoints/{id}/deliveries` | Listar entregas del endpoint |
| [`webhook-endpoints list`](#webhook-endpoints-list) | `GET /v1/webhook-endpoints` | Listar endpoints de webhook |
| [`webhook-endpoints create`](#webhook-endpoints-create) | `POST /v1/webhook-endpoints` | Crear endpoint de webhook |
| [`webhook-endpoints get`](#webhook-endpoints-get) | `GET /v1/webhook-endpoints/{id}` | Consultar endpoint de webhook |
| [`webhook-endpoints update`](#webhook-endpoints-update) | `PATCH /v1/webhook-endpoints/{id}` | Actualizar endpoint de webhook |
| [`webhook-endpoints delete`](#webhook-endpoints-delete) | `DELETE /v1/webhook-endpoints/{id}` | Eliminar endpoint de webhook |
| [`webhook-endpoints rotate-secret`](#webhook-endpoints-rotate-secret) | `POST /v1/webhook-endpoints/{id}/rotate-secret` | Rotar secret |
| [`webhook-endpoints test`](#webhook-endpoints-test) | `POST /v1/webhook-endpoints/{id}/test` | Disparar evento de prueba |

## webhook-endpoints deliveries retry

Reenviar entrega

```text
kobana dunning webhook-endpoints deliveries retry [ID] [DELIVERYID] [OPCIONES]
```

`POST /v1/webhook-endpoints/{id}/deliveries/{deliveryId}/retry`

Reencola una entrega para un reintento inmediato (el estado vuelve a `pending`). Falla con 409 si el endpoint está inactivo o eliminado.

**Argumentos de la ruta:** `id`, `deliveryId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Entrega reencolada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## webhook-endpoints deliveries list

Listar entregas del endpoint

```text
kobana dunning webhook-endpoints deliveries list [ID] [OPCIONES]
```

`GET /v1/webhook-endpoints/{id}/deliveries`

Lista las entregas del endpoint (paginado, más recientes primero) con la captura de request/response para inspección y debug.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `status` | `string` | no | Filtra por el estado de la entrega. Valores: `pending`, `success`, `failed`, `exhausted`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de entregas. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## webhook-endpoints list

Listar endpoints de webhook

```text
kobana dunning webhook-endpoints list [OPCIONES]
```

`GET /v1/webhook-endpoints`

Lista los endpoints de webhook de la organización con contadores de entrega por estado. `secret` y `auth_config` nunca aparecen en el listado.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de endpoints. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## webhook-endpoints create

Crear endpoint de webhook

```text
kobana dunning webhook-endpoints create [OPCIONES]
```

`POST /v1/webhook-endpoints`

Crea un endpoint de webhook. La URL exige HTTPS y se valida contra direcciones internas (SSRF). El `secret` se devuelve solo en esta respuesta — guárdalo con seguridad.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `url` | `string (uri)` | sí | URL HTTPS de destino de las entregas (validada contra direcciones internas — SSRF). |
| `description` | `string` | no | Descripción libre del endpoint. |
| `events` | `array<string>` | no | Tipos de evento suscritos. Lista vacía = todos los eventos. |
| `auth_type` | `string` | no | Autenticación del request contra tu endpoint (`none` \| `basic` \| `bearer` \| `header`), además de la firma HMAC (siempre enviada). Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | no | Credenciales del `auth_type`: `basic` → `{"username","password"}`; `bearer` → `{"token"}`; `header` → `{"key","value"}`. Cifrado en reposo; nunca devuelto en las consultas. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |

Otros campos y objetos anidados: `kobana schema dunning.webhook-endpoints.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Endpoint creado (incluye `secret` — se muestra una única vez). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## webhook-endpoints get

Consultar endpoint de webhook

```text
kobana dunning webhook-endpoints get [ID] [OPCIONES]
```

`GET /v1/webhook-endpoints/{id}`

Devuelve el endpoint con sus últimas 20 entregas. El `secret` nunca se devuelve.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Endpoint con entregas recientes. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## webhook-endpoints update

Actualizar endpoint de webhook

```text
kobana dunning webhook-endpoints update [ID] [OPCIONES]
```

`PATCH /v1/webhook-endpoints/{id}`

Actualiza el endpoint (parcial). La URL, si se envía, se revalida (HTTPS + SSRF); `auth_config` se cifra y nunca se devuelve.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `url` | `string (uri)` | no | URL HTTPS de destino de las entregas (validada contra direcciones internas — SSRF). |
| `description` | `string` | no | Descripción libre del endpoint. |
| `events` | `array<string>` | no | Tipos de evento suscritos. Lista vacía = todos los eventos. |
| `is_active` | `boolean` | no | Un endpoint activo recibe entregas; uno inactivo es ignorado por el worker. |
| `auth_type` | `string` | no | Autenticación del request contra tu endpoint (`none` \| `basic` \| `bearer` \| `header`), además de la firma HMAC (siempre enviada). Valores: `none`, `basic`, `bearer`, `header`. |
| `auth_config` | `object` | no | Credenciales del `auth_type`: `basic` → `{"username","password"}`; `bearer` → `{"token"}`; `header` → `{"key","value"}`. Cifrado en reposo; nunca devuelto en las consultas. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.webhook-endpoints.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Endpoint actualizado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## webhook-endpoints delete

Eliminar endpoint de webhook

```text
kobana dunning webhook-endpoints delete [ID] [OPCIONES]
```

`DELETE /v1/webhook-endpoints/{id}`

Elimina (borrado lógico) el endpoint; las entregas pendientes dejan de procesarse.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Endpoint eliminado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## webhook-endpoints rotate-secret

Rotar secret

```text
kobana dunning webhook-endpoints rotate-secret [ID] [OPCIONES]
```

`POST /v1/webhook-endpoints/{id}/rotate-secret`

Genera un nuevo secret HMAC; el anterior deja de validar de inmediato. El nuevo secret se devuelve solo en esta respuesta — guárdalo con seguridad.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Nuevo secret (se muestra una única vez). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## webhook-endpoints test

Disparar evento de prueba

```text
kobana dunning webhook-endpoints test [ID] [OPCIONES]
```

`POST /v1/webhook-endpoints/{id}/test`

Entrega un evento `webhook.test` REAL al endpoint, por el mismo pipeline de los eventos de negocio (cola, firma HMAC, captura de request/response, retry) — valida tu consumidor de punta a punta sin esperar un evento real. Entregado solo al endpoint objetivo, independiente de la lista de eventos suscritos. Falla con 409 si el endpoint está inactivo.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Evento de prueba encolado para entrega. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
