---
id: agreements
title: "kobana dunning agreements"
sidebar_label: "agreements"
---

# kobana dunning agreements


Comandos de `kobana dunning agreements`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`agreements installments`](#agreements-installments) | `GET /v1/agreements/{id}/installments` | Listar cuotas del acuerdo |
| [`agreements installments update`](#agreements-installments-update) | `PUT /v1/agreements/{id}/installments` | Actualizar cuota del acuerdo |
| [`agreements list`](#agreements-list) | `GET /v1/agreements` | Listar acuerdos |
| [`agreements create`](#agreements-create) | `POST /v1/agreements` | Crear acuerdo |
| [`agreements get`](#agreements-get) | `GET /v1/agreements/{id}` | Consultar acuerdo |
| [`agreements update`](#agreements-update) | `PUT /v1/agreements/{id}` | Actualizar acuerdo |
| [`agreements delete`](#agreements-delete) | `DELETE /v1/agreements/{id}` | Eliminar acuerdo |
| [`agreements accept`](#agreements-accept) | `POST /v1/agreements/{id}/accept` | Aceptar acuerdo |
| [`agreements cancel`](#agreements-cancel) | `POST /v1/agreements/{id}/cancel` | Cancelar acuerdo |
| [`agreements pdf`](#agreements-pdf) | `GET /v1/agreements/{id}/pdf` | Término de acuerdo (PDF) |

## agreements installments

Listar cuotas del acuerdo

```text
kobana dunning agreements installments [ID] [OPCIONES] [COMANDO]
```

`GET /v1/agreements/{id}/installments`

Lista las cuotas de un acuerdo (ordenadas por número) con un resumen agregado.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuotas del acuerdo con resumen. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements installments update

Actualizar cuota del acuerdo

```text
kobana dunning agreements installments update [ID] [OPCIONES]
```

`PUT /v1/agreements/{id}/installments`

Actualiza una cuota (pago/estado). Permitido solo en acuerdos `accepted` o `active`; las cuotas canceladas no pueden modificarse (409). El primer pago activa el acuerdo; con todas las cuotas pagadas, el acuerdo se completa (`completed`).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `installment_number` | `integer` | sí | Número de la cuota (desde 1). |
| `paid_amount` | `number` | no | Valor pagado de la cuota. |
| `paid_at` | `string` | no | Fecha de pago de la cuota. |
| `status` | `string` | no | Estado (`pending` \| `paid` \| `overdue` \| `cancelled`). Valores: `pending`, `paid`, `overdue`, `cancelled`. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.agreements.installments.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuota actualizada (devuelve la cuota y el acuerdo actualizado). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements list

Listar acuerdos

```text
kobana dunning agreements list [OPCIONES]
```

`GET /v1/agreements`

Lista los acuerdos de la organización con paginación y filtros.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `sort_by` | `string` | no | Campo de ordenación (nombre público en snake_case, ej.: `due_date`). |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra por estado del acuerdo. Valores: `proposed`, `accepted`, `active`, `completed`, `cancelled`, `defaulted`. |
| `person_id` | `string` | no | Filtra por los acuerdos de un cliente (UUID). |
| `include` | `string` | no | Relaciones a incluir en la respuesta (`person`, `installments`, `negotiation_offer`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de acuerdos. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements create

Crear acuerdo

```text
kobana dunning agreements create [OPCIONES]
```

`POST /v1/agreements`

Crea un acuerdo con estado `proposed`. `final_total` = `original_total` − `discount_amount` (debe ser mayor que cero) e `installment_value` = `final_total` / `number_of_installments`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_id` | `string (uuid)` | sí | Cliente (deudor) del acuerdo. |
| `original_total` | `number` | sí | Total original de la deuda renegociada. |
| `number_of_installments` | `integer` | sí | Número de cuotas (1 a 120). |
| `first_due_date` | `string` | sí | Vencimiento de la primera cuota (las demás vencen mensualmente). |
| `negotiation_offer_id` | `string (uuid)` | no | Oferta de negociación que originó el acuerdo. |
| `discount_amount` | `number` | no | Descuento concedido (predeterminado 0). |
| `terms_accepted` | `string` | no | Términos aceptados por el deudor (texto/versión). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.agreements.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Acuerdo creado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements get

Consultar acuerdo

```text
kobana dunning agreements get [ID] [OPCIONES]
```

`GET /v1/agreements/{id}`

Devuelve un acuerdo por ID (siempre incluye datos resumidos del cliente).

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `include` | `string` | no | Relaciones a incluir en la respuesta (`installments`, `negotiation_offer`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Acuerdo encontrado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements update

Actualizar acuerdo

```text
kobana dunning agreements update [ID] [OPCIONES]
```

`PUT /v1/agreements/{id}`

Actualiza un acuerdo (parcial). Permitido solo con estado `proposed` o `accepted` (409 en los demás).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `original_total` | `number` | no | Total original de la deuda renegociada. |
| `discount_amount` | `number` | no | Descuento concedido (predeterminado 0). |
| `number_of_installments` | `integer` | no | Número de cuotas (1 a 120). |
| `first_due_date` | `string` | no | Vencimiento de la primera cuota (las demás vencen mensualmente). |
| `terms_accepted` | `string` | no | Términos aceptados por el deudor (texto/versión). |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_data_mode` | `string` | no | Cómo se aplica `custom_data` en el update: `replace` (default) sobrescribe el objeto entero; `merge` fusiona (deep) el payload sobre el valor almacenado — los objetos anidados se fusionan, escalares/arrays/null sobrescriben la clave. Valores: `replace`, `merge`. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |
| `tags` | `array<string>` | no | Etiquetas libres. |

Otros campos y objetos anidados: `kobana schema dunning.agreements.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Acuerdo actualizado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements delete

Eliminar acuerdo

```text
kobana dunning agreements delete [ID] [OPCIONES]
```

`DELETE /v1/agreements/{id}`

Elimina (borrado lógico) un acuerdo, marcándolo como `cancelled`. Los acuerdos `completed` no pueden eliminarse (409).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Acuerdo eliminado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements accept

Aceptar acuerdo

```text
kobana dunning agreements accept [ID] [OPCIONES]
```

`POST /v1/agreements/{id}/accept`

Acepta un acuerdo `proposed` (409 en los demás estados): registra `accepted_at`/`accepted_ip` y crea las cuotas con vencimientos mensuales a partir de `first_due_date`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `accepted_ip` | `string` | no | IP registrada en la aceptación (pista de auditoría). |
| `terms_accepted` | `string` | no | Términos aceptados por el deudor (texto/versión). |

Otros campos y objetos anidados: `kobana schema dunning.agreements.accept`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Acuerdo aceptado (devuelve el acuerdo con las cuotas creadas). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements cancel

Cancelar acuerdo

```text
kobana dunning agreements cancel [ID] [OPCIONES]
```

`POST /v1/agreements/{id}/cancel`

Cancela un acuerdo, registrando `cancelled_at` y el motivo, y cancela las cuotas pendientes/vencidas. Los acuerdos `completed` o ya cancelados devuelven 409.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `reason` | `string` | sí | Motivo de la cancelación (obligatorio; guardado en `cancellation_reason`). |

Otros campos y objetos anidados: `kobana schema dunning.agreements.cancel`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Acuerdo cancelado (devuelve el acuerdo con sus cuotas). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## agreements pdf

Término de acuerdo (PDF)

```text
kobana dunning agreements pdf [ID] [OPCIONES]
```

`GET /v1/agreements/{id}/pdf`

Genera el término de acuerdo en PDF (documento jurídico) con acreedor, deudor, totales y cuotas, delimitado por la organización. Responde `application/pdf` como adjunto binario.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | PDF del término de acuerdo (adjunto binario `application/pdf`). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
