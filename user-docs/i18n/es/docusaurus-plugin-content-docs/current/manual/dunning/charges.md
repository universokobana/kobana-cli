---
id: charges
title: "kobana dunning charges"
sidebar_label: "charges"
---

# kobana dunning charges


Comandos de `kobana dunning charges`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`charges bulk delete`](#charges-bulk-delete) | `DELETE /v1/charges/bulk` | Eliminar cobros en masa |
| [`charges list`](#charges-list) | `GET /v1/charges` | Listar cobros |
| [`charges create`](#charges-create) | `POST /v1/charges` | Crear cobro |
| [`charges bulk-assign-rule`](#charges-bulk-assign-rule) | `POST /v1/charges/bulk-assign-rule` | Aplicar regla en masa |
| [`charges bulk-cancel`](#charges-bulk-cancel) | `POST /v1/charges/bulk-cancel` | Cancelar cobros en masa |
| [`charges get`](#charges-get) | `GET /v1/charges/{id}` | Consultar cobro |
| [`charges update`](#charges-update) | `PUT /v1/charges/{id}` | Actualizar cobro |
| [`charges delete`](#charges-delete) | `DELETE /v1/charges/{id}` | Eliminar cobro |
| [`charges notify`](#charges-notify) | `POST /v1/charges/{id}/notify` | Notificar cobro |
| [`charges settlement-pdf`](#charges-settlement-pdf) | `GET /v1/charges/{id}/settlement-pdf` | Carta de finiquito (PDF) |

## charges bulk delete

Eliminar cobros en masa

```text
kobana dunning charges bulk delete [OPCIONES]
```

`DELETE /v1/charges/bulk`

Elimina (soft delete, irreversible) hasta 100 cobros en una llamada, con los ids en el cuerpo JSON, en cascada hacia tareas y ofertas relacionadas. Un cobro con registros asociados (notificaciones, interacciones, negativaciones, protestos) no se puede eliminar y vuelve en `failed` con `has_related_records`. Exige el nivel de acceso `delete` (o `all`) — `write` no basta. Sustituyó a `POST /charges/bulk-archive`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | IDs de los registros objetivo (1 a 100 por llamada). Los duplicados se ignoran. |

Otros campos y objetos anidados: `kobana schema dunning.charges.bulk.delete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado por ítem de la eliminación masiva. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges list

Listar cobros

```text
kobana dunning charges list [OPCIONES]
```

`GET /v1/charges`

Lista los cobros de la organización con paginación y filtros. Los cobros pendientes vencidos se promueven a `overdue` en la lectura.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `sort_by` | `string` | no | Campo de ordenación (nombre público en snake_case, ej.: `due_date`). |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda por número de documento, descripción, `external_id`, nombre o documento del cliente. |
| `person_id` | `string` | no | Filtra por los cobros de un cliente (UUID). |
| `status` | `string` | no | Filtra por estado. Acepta un valor o una lista separada por comas (`pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`). |
| `due_date_from` | `string` | no | Vencimiento desde (inclusive, `YYYY-MM-DD` o datetime ISO). |
| `due_date_to` | `string` | no | Vencimiento hasta (inclusive, `YYYY-MM-DD` o datetime ISO). |
| `include` | `string` | no | Relaciones a incluir en la respuesta (`person`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de cobros. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges create

Crear cobro

```text
kobana dunning charges create [OPCIONES]
```

`POST /v1/charges`

Crea un cobro para un cliente. Sin `collection_rule_id`, se aplica la regla de cobranza predeterminada de la organización. `current_amount` = original + intereses + multa − descuento; un cobro ya vencido entra como `overdue`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_id` | `string (uuid)` | sí | Cliente (deudor) del cobro. |
| `original_amount` | `number` | sí | Valor original del título. |
| `issue_date` | `string` | sí | Fecha de emisión. |
| `due_date` | `string` | sí | Fecha de vencimiento. |
| `collection_rule_id` | `string (uuid)` | no | Regla de cobranza que sigue el título. Si se omite al crear, se usa la regla predeterminada de la organización. |
| `document_number` | `string` | no | Número del documento de origen (factura, contrato, etc.). |
| `description` | `string` | no | Descripción libre del cobro. |
| `interest_amount` | `number` | no | Intereses acumulados (predeterminado 0). |
| `fine_amount` | `number` | no | Multa (predeterminado 0). |
| `discount_amount` | `number` | no | Descuento aplicado (predeterminado 0). |
| `status` | `string` | no | Estado (`pending` \| `paid` \| `overdue` \| `cancelled` \| `negotiated` \| `protested` \| `negatived` \| `written_off`). Valores: `pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`. |
| `payment_method` | `string` | no | Medio de pago (`boleto` \| `pix` \| `credit_card` \| `transfer`). Valores: `boleto`, `pix`, `credit_card`, `transfer`, `null`. |
| `barcode` | `string` | no | Código de barras/línea digitable del boleto. |
| `pix_emv` | `string` | no | Código Pix copia y pega (EMV). |
| `payment_url` | `string (uri)` | no | URL de la página de pago. |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `charge_policy` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema dunning.charges.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cobro creado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges bulk-assign-rule

Aplicar regla en masa

```text
kobana dunning charges bulk-assign-rule [OPCIONES]
```

`POST /v1/charges/bulk-assign-rule`

Aplica una regla de cobranza a hasta 100 cobros. La entrada en la regla respeta la idempotencia del motor: un cobro con enrollment activo/pausado mantiene el vigente. Solo los cobros `pending`/`overdue` son elegibles; el resto vuelve en `failed` con `invalid_status`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | IDs de los registros objetivo (1 a 100 por llamada). Los duplicados se ignoran. |
| `collection_rule_id` | `string (uuid)` | sí | ID de la regla de cobranza a aplicar (debe pertenecer a la organización). |

Otros campos y objetos anidados: `kobana schema dunning.charges.bulk-assign-rule`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado por ítem de la aplicación masiva de regla. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges bulk-cancel

Cancelar cobros en masa

```text
kobana dunning charges bulk-cancel [OPCIONES]
```

`POST /v1/charges/bulk-cancel`

Cancela hasta 100 cobros en una llamada. Cada cancelación pasa por el motor de la regla (valida la transición de estado, cierra el enrollment y suprime envíos pendientes). Un resultado parcial no es error: los ítems no cancelables vuelven en `failed` con el motivo (`not_found`, `invalid_status`).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | IDs de los registros objetivo (1 a 100 por llamada). Los duplicados se ignoran. |

Otros campos y objetos anidados: `kobana schema dunning.charges.bulk-cancel`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado por ítem de la cancelación masiva. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges get

Consultar cobro

```text
kobana dunning charges get [ID] [OPCIONES]
```

`GET /v1/charges/{id}`

Devuelve un cobro por ID. Usa `include=engine` para recibir la telemetría del motor (inscripción en la regla y ejecuciones de etapa).

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `include` | `string` | no | Relaciones a incluir en la respuesta (`person`, `engine` — telemetría de la regla). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cobro encontrado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges update

Actualizar cobro

```text
kobana dunning charges update [ID] [OPCIONES]
```

`PUT /v1/charges/{id}`

Actualiza los datos de un cobro (parcial). `status`, `paid_amount` y `paid_at` pertenecen al motor: reenviar el valor guardado se acepta, pero cambiarlos responde 422 `ENGINE_MANAGED_FIELD` — los pagos entran por `POST /charges/{id}/pay`. La única transición aceptada aquí es `status: cancelled` (409 si el estado actual no lo permite).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_id` | `string (uuid)` | no | Cliente (deudor) del cobro. |
| `collection_rule_id` | `string (uuid)` | no | Regla de cobranza que sigue el título. Si se omite al crear, se usa la regla predeterminada de la organización. |
| `document_number` | `string` | no | Número del documento de origen (factura, contrato, etc.). |
| `description` | `string` | no | Descripción libre del cobro. |
| `original_amount` | `number` | no | Valor original del título. |
| `interest_amount` | `number` | no | Intereses acumulados (predeterminado 0). |
| `fine_amount` | `number` | no | Multa (predeterminado 0). |
| `discount_amount` | `number` | no | Descuento aplicado (predeterminado 0). |
| `issue_date` | `string` | no | Fecha de emisión. |
| `due_date` | `string` | no | Fecha de vencimiento. |
| `status` | `string` | no | Estado (`pending` \| `paid` \| `overdue` \| `cancelled` \| `negotiated` \| `protested` \| `negatived` \| `written_off`). Valores: `pending`, `paid`, `overdue`, `cancelled`, `negotiated`, `protested`, `negatived`, `written_off`. |
| `payment_method` | `string` | no | Medio de pago (`boleto` \| `pix` \| `credit_card` \| `transfer`). Valores: `boleto`, `pix`, `credit_card`, `transfer`, `null`. |
| `barcode` | `string` | no | Código de barras/línea digitable del boleto. |
| `pix_emv` | `string` | no | Código Pix copia y pega (EMV). |
| `payment_url` | `string (uri)` | no | URL de la página de pago. |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `charge_policy` | `object` | no |  |
| `paid_amount` | `number` | no | Valor efectivamente pagado. |
| `paid_at` | `string` | no | Fecha del pago. |
| `custom_data_mode` | `string` | no | Cómo se aplica `custom_data` en el update: `replace` (default) sobrescribe el objeto entero; `merge` fusiona (deep) el payload sobre el valor almacenado — los objetos anidados se fusionan, escalares/arrays/null sobrescriben la clave. Valores: `replace`, `merge`. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.charges.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cobro actualizado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges delete

Eliminar cobro

```text
kobana dunning charges delete [ID] [OPCIONES]
```

`DELETE /v1/charges/{id}`

Elimina (borrado lógico) un cobro junto con las tareas y ofertas relacionadas. Un cobro con notificaciones, interacciones, negativaciones o protestos asociados no puede eliminarse (409).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Cobro eliminado (devuelve un mensaje de confirmación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges notify

Notificar cobro

```text
kobana dunning charges notify [ID] [OPCIONES]
```

`POST /v1/charges/{id}/notify`

Crea una notificación manual para el cobro y la envía a la cola de procesamiento. El destinatario debe ser un contacto registrado de la persona (422 `UNKNOWN_RECIPIENT` en caso contrario). Actualiza `last_notification_at`; el progreso de la regla (`current_step`) no cambia — solo el motor lo avanza.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `channel` | `string` | sí | Canal (`email` \| `sms` \| `whatsapp` \| `voice` \| `manual`). Valores: `email`, `sms`, `whatsapp`, `voice`, `manual`. |
| `recipient` | `string` | sí | Destinatario: correo para `email`, teléfono para `sms`/`whatsapp`/`voice` — siempre un contacto registrado de la persona del cobro; libre para `manual`. |
| `body` | `string` | sí | Cuerpo del mensaje. |
| `subject` | `string` | no | Asunto (usado en correo). |
| `collection_rule_step_id` | `string (uuid)` | no | Etapa de la regla del cobro a asociar a la notificación (etiqueta; no cambia el `current_step`). |

Otros campos y objetos anidados: `kobana schema dunning.charges.notify`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Notificación creada y encolada (estado `queued`). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## charges settlement-pdf

Carta de finiquito (PDF)

```text
kobana dunning charges settlement-pdf [ID] [OPCIONES]
```

`GET /v1/charges/{id}/settlement-pdf`

Genera la carta de finiquito del cobro en PDF (documento jurídico), delimitada por la organización. Solo se permite cuando el cobro está pagado (estado `paid`); de lo contrario devuelve 409 (`CHARGE_NOT_SETTLED`). Responde `application/pdf` como adjunto binario.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | PDF de la carta de finiquito (adjunto binario `application/pdf`). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | El cobro no está finiquitado — la carta solo puede generarse para el estado `paid` (código `CHARGE_NOT_SETTLED`). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
