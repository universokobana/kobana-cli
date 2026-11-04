---
id: people
title: "kobana dunning people"
sidebar_label: "people"
---

# kobana dunning people


Comandos de `kobana dunning people`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`people bulk delete`](#people-bulk-delete) | `DELETE /v1/people/bulk` | Eliminar clientes en masa |
| [`people contact-points list`](#people-contact-points-list) | `GET /v1/people/{id}/contact-points` | Listar puntos de contacto |
| [`people contact-points create`](#people-contact-points-create) | `POST /v1/people/{id}/contact-points` | Crear punto de contacto |
| [`people contact-points update`](#people-contact-points-update) | `PATCH /v1/people/{id}/contact-points/{cpId}` | Actualizar punto de contacto |
| [`people contact-points delete`](#people-contact-points-delete) | `DELETE /v1/people/{id}/contact-points/{cpId}` | Eliminar punto de contacto |
| [`people list`](#people-list) | `GET /v1/people` | Listar clientes |
| [`people create`](#people-create) | `POST /v1/people` | Crear cliente |
| [`people get`](#people-get) | `GET /v1/people/{id}` | Consultar cliente |
| [`people update`](#people-update) | `PUT /v1/people/{id}` | Actualizar cliente |
| [`people delete`](#people-delete) | `DELETE /v1/people/{id}` | Eliminar cliente |
| [`people statement-pdf`](#people-statement-pdf) | `GET /v1/people/{id}/statement-pdf` | Estado de cuenta del deudor (PDF) |

## people bulk delete

Eliminar clientes en masa

```text
kobana dunning people bulk delete [OPCIONES]
```

`DELETE /v1/people/bulk`

Elimina (soft delete, irreversible) hasta 100 clientes en una llamada, con los ids en el cuerpo JSON. Un cliente con cobros abiertos (`pending`/`overdue`/`negotiated`) no se puede eliminar y vuelve en `failed` con `has_active_charges`. Un resultado parcial no es error. Exige el nivel de acceso `delete` (o `all`) — `write` no basta. Sustituyó a `POST /people/bulk-archive`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | IDs de los registros objetivo (1 a 100 por llamada). Los duplicados se ignoran. |

Otros campos y objetos anidados: `kobana schema dunning.people.bulk.delete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado por ítem de la eliminación masiva. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people contact-points list

Listar puntos de contacto

```text
kobana dunning people contact-points list [ID] [OPCIONES]
```

`GET /v1/people/{id}/contact-points`

Lista los puntos de contacto de un cliente, con paginación y filtros por canal y estado.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `channel` | `string` | no | Filtra por canal (`email` \| `sms` \| `whatsapp` \| `voice` \| `postal`). Valores: `email`, `sms`, `whatsapp`, `voice`, `postal`. |
| `status` | `string` | no | Filtra por estado (`active` \| `deleted`). Valores: `active`, `deleted`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de puntos de contacto. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people contact-points create

Crear punto de contacto

```text
kobana dunning people contact-points create [ID] [OPCIONES]
```

`POST /v1/people/{id}/contact-points`

Crea un punto de contacto para el cliente. Devuelve 409 si ya existe un contacto activo con el mismo canal y valor.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `channel` | `string` | sí | Canal (`email` \| `sms` \| `whatsapp` \| `voice` \| `postal`). Valores: `email`, `sms`, `whatsapp`, `voice`, `postal`. |
| `value` | `string` | sí | Valor NORMALIZADO del contacto: correo en minúsculas; teléfono y CP solo dígitos. |
| `label` | `string` | no | Etiqueta libre (ej.: personal/commercial/mobile/whatsapp). |
| `is_primary` | `boolean` | no | Indica el contacto primario del canal (usado primero en la resolución del destinatario). |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |

Otros campos y objetos anidados: `kobana schema dunning.people.contact-points.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Punto de contacto creado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Conflicto con el estado actual del recurso (o clave de idempotencia reutilizada). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people contact-points update

Actualizar punto de contacto

```text
kobana dunning people contact-points update [ID] [CPID] [OPCIONES]
```

`PATCH /v1/people/{id}/contact-points/{cpId}`

Actualiza etiqueta/primario/verificación de un punto de contacto y permite marcar/desmarcar opt-out y tercero manualmente.

**Argumentos de la ruta:** `id`, `cpId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no | Etiqueta libre (ej.: personal/commercial/mobile/whatsapp). |
| `is_primary` | `boolean` | no | Indica el contacto primario del canal (usado primero en la resolución del destinatario). |
| `verified` | `boolean` | no | `true` marca el contacto como verificado ahora; `false` elimina la verificación. |
| `opt_out` | `boolean` | no | `true` registra opt-out por contacto ahora; `false` lo revierte. |
| `third_party` | `boolean` | no | `true` marca el contacto como de tercero ahora; `false` lo revierte. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.people.contact-points.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Punto de contacto actualizado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people contact-points delete

Eliminar punto de contacto

```text
kobana dunning people contact-points delete [ID] [CPID] [OPCIONES]
```

`DELETE /v1/people/{id}/contact-points/{cpId}`

Elimina (borrado lógico) un punto de contacto. Las supresiones y el historial permanecen para la pista de auditoría.

**Argumentos de la ruta:** `id`, `cpId`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Punto de contacto eliminado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people list

Listar clientes

```text
kobana dunning people list [OPCIONES]
```

`GET /v1/people`

Lista los clientes de la organización con paginación y filtros.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `sort_by` | `string` | no | Campo de ordenación (nombre público en snake_case, ej.: `due_date`). |
| `sort_order` | `string` | no | Dirección de ordenación (`asc` \| `desc`). Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda por nombre, documento o correo. |
| `classification_id` | `string` | no | Filtra por clasificación (UUID). |
| `status` | `string` | no | Filtra por estado del registro (`active` \| `deleted`). Valores: `active`, `deleted`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de clientes. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people create

Crear cliente

```text
kobana dunning people create [OPCIONES]
```

`POST /v1/people`

Crea un cliente en la cartera. `external_id` y `document_number` se usan para deduplicación en importaciones e integraciones.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del cliente. |
| `classification_id` | `string (uuid)` | no | Clasificación del cliente (segmentación de la regla). |
| `document_number` | `string` | no | CPF/CNPJ (solo dígitos). |
| `document_type` | `string` | no | Tipo de documento (`cpf` \| `cnpj`). Valores: `cpf`, `cnpj`, `null`. |
| `kind` | `string` | no | Naturaleza (`natural` = persona física, `juridical` = jurídica). Valores: `natural`, `juridical`, `null`. |
| `legal_name` | `string` | no | Razón social. |
| `nickname` | `string` | no | Alias/nombre de fantasía. |
| `birthday` | `string (date-time)` | no | Fecha de nacimiento (ISO 8601). |
| `addresses` | `array<object>` | no | Direcciones. |
| `phones` | `array<object>` | no | Teléfonos. |
| `emails` | `array<object>` | no | Correos electrónicos. |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `notes` | `string` | no | Notas internas. |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |

Otros campos y objetos anidados: `kobana schema dunning.people.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cliente creado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people get

Consultar cliente

```text
kobana dunning people get [ID] [OPCIONES]
```

`GET /v1/people/{id}`

Devuelve un cliente por ID.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cliente encontrado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people update

Actualizar cliente

```text
kobana dunning people update [ID] [OPCIONES]
```

`PUT /v1/people/{id}`

Actualiza los datos de un cliente (parcial).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `classification_id` | `string (uuid)` | no | Clasificación del cliente (segmentación de la regla). |
| `document_number` | `string` | no | CPF/CNPJ (solo dígitos). |
| `document_type` | `string` | no | Tipo de documento (`cpf` \| `cnpj`). Valores: `cpf`, `cnpj`, `null`. |
| `kind` | `string` | no | Naturaleza (`natural` = persona física, `juridical` = jurídica). Valores: `natural`, `juridical`, `null`. |
| `name` | `string` | no | Nombre del cliente. |
| `legal_name` | `string` | no | Razón social. |
| `nickname` | `string` | no | Alias/nombre de fantasía. |
| `birthday` | `string (date-time)` | no | Fecha de nacimiento (ISO 8601). |
| `addresses` | `array<object>` | no | Direcciones. |
| `phones` | `array<object>` | no | Teléfonos. |
| `emails` | `array<object>` | no | Correos electrónicos. |
| `external_id` | `string` | no | Identificador en TU sistema (clave de las integraciones). |
| `tags` | `array<string>` | no | Etiquetas libres. |
| `notes` | `string` | no | Notas internas. |
| `custom_data` | `object` | no | Metadatos personalizables por el consumidor de la API. Límites: 64 KB serializados y 16 niveles de anidamiento. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |
| `custom_data_mode` | `string` | no | Cómo se aplica `custom_data` en el update: `replace` (default) sobrescribe el objeto entero; `merge` fusiona (deep) el payload sobre el valor almacenado — los objetos anidados se fusionan, escalares/arrays/null sobrescriben la clave. Valores: `replace`, `merge`. |
| `custom_metadata_mode` | `string` | no | Cómo el `custom_metadata` enviado se concilia con el valor guardado: `replace` (por defecto) reemplaza el objeto entero; `merge` hace deep merge — los objetos anidados se mezclan, escalares y arrays reemplazan la clave, y una clave enviada con `null` se guarda como `null` (no se elimina). Solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema dunning.people.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cliente actualizado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people delete

Eliminar cliente

```text
kobana dunning people delete [ID] [OPCIONES]
```

`DELETE /v1/people/{id}`

Elimina (borrado lógico) un cliente. Los cobros y el historial permanecen para la pista de auditoría.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Cliente eliminado. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## people statement-pdf

Estado de cuenta del deudor (PDF)

```text
kobana dunning people statement-pdf [ID] [OPCIONES]
```

`GET /v1/people/{id}/statement-pdf`

Genera el estado de cuenta del deudor en PDF: todos los cobros de la persona (documento, vencimiento, estado, importe original y actual) con los totales pendiente, vencido y ya pagado. Delimitado por la organización. Responde `application/pdf` como adjunto binario.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | PDF del estado de cuenta del deudor (adjunto binario `application/pdf`). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
