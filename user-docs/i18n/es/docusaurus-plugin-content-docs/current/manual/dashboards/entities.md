---
id: entities
title: "kobana dashboards entities"
sidebar_label: "entities"
---

# kobana dashboards entities


Comandos de `kobana dashboards entities`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`entities join preview`](#entities-join-preview) | `POST /v1/entities/join/preview` | Vista previa de una relación |
| [`entities join create`](#entities-join-create) | `POST /v1/entities/join` | Crear entidad de relación |
| [`entities join update`](#entities-join-update) | `PUT /v1/entities/{id}/join` | Modificar relación |
| [`entities logs get`](#entities-logs-get) | `GET /v1/entities/{id}/logs/{log_id}` | Consultar llamada de la sincronización |
| [`entities logs list`](#entities-logs-list) | `GET /v1/entities/{id}/logs` | Registro de la sincronización |
| [`entities records list`](#entities-records-list) | `GET /v1/entities/{id}/records` | Consultar registros |
| [`entities records create`](#entities-records-create) | `POST /v1/entities/{id}/records` | Enviar registros |
| [`entities records delete`](#entities-records-delete) | `DELETE /v1/entities/{id}/records` | Limpiar registros |
| [`entities records import`](#entities-records-import) | `POST /v1/entities/{id}/records/import` | Importar registros desde CSV |
| [`entities records get`](#entities-records-get) | `GET /v1/entities/{id}/records/{record_id}` | Consultar registro |
| [`entities sync`](#entities-sync) | `GET /v1/entities/{id}/sync` | Estado de la sincronización |
| [`entities sync create`](#entities-sync-create) | `POST /v1/entities/{id}/sync` | Sincronizar entidad |
| [`entities tenant-syncs list`](#entities-tenant-syncs-list) | `GET /v1/entities/{id}/tenant_syncs` | Sincronizaciones por titular |
| [`entities tenant-syncs create`](#entities-tenant-syncs-create) | `POST /v1/entities/{id}/tenant_syncs` | Sincronizar un titular |
| [`entities list`](#entities-list) | `GET /v1/entities` | Listar entidades |
| [`entities create`](#entities-create) | `POST /v1/entities` | Crear entidad |
| [`entities preview`](#entities-preview) | `POST /v1/entities/preview` | Vista previa de un origen |
| [`entities get`](#entities-get) | `GET /v1/entities/{id}` | Consultar entidad |
| [`entities update`](#entities-update) | `PATCH /v1/entities/{id}` | Actualizar entidad |
| [`entities delete`](#entities-delete) | `DELETE /v1/entities/{id}` | Eliminar entidad |
| [`entities aggregate`](#entities-aggregate) | `GET /v1/entities/{id}/aggregate` | Calcular agregados de los registros |
| [`entities usage`](#entities-usage) | `GET /v1/entities/{id}/usage` | Uso de la entidad |

## entities join preview

Vista previa de una relación

```text
kobana dashboards entities join preview [OPCIONES]
```

`POST /v1/entities/join/preview`

Cómo coinciden dos entidades en los campos indicados (recuentos) y las primeras filas combinadas, antes de guardar la relación. Lee solo los registros ya sincronizados.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `base_entity_id` | `string (uuid)` | sí | Entidad base: todos sus registros entran en la relación. |
| `related_entity_id` | `string (uuid)` | sí | Entidad relacionada, emparejada con la base por los campos indicados. |
| `base_field` | `string` | sí | Clave del campo de la entidad base usado para emparejar. |
| `related_field` | `string` | sí | Clave del campo de la entidad relacionada usado para emparejar. |
| `type` | `string` | no | `left` (por defecto) mantiene los registros de la base sin pareja; `inner` solo los que coinciden. Valores: `inner`, `left`. |
| `match` | `string` | no | `exact` (por defecto) compara el texto; `digits` compara solo los dígitos (un CPF con y sin puntuación coinciden). Valores: `exact`, `digits`. |

Otros campos y objetos anidados: `kobana schema dashboards.entities.join.preview`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Vista previa de la relación. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities join create

Crear entidad de relación

```text
kobana dashboards entities join create [OPCIONES]
```

`POST /v1/entities/join`

Registra una entidad que relaciona otras dos (join) con los campos elegidos de cada una, y encola la primera sincronización.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | sí | Nombre de máquina que usan los bloques del portal; único en el workspace (hasta 63 caracteres). |
| `name` | `string` | sí |  |
| `name_plural` | `string` | sí |  |
| `join` | `object` | sí |  |
| `fields` | `array<object>` | sí | Los campos traídos de cada lado: `{ from: base\|related, field_key, key?, name? }`. |
| `description` | `string` | no |  |
| `label_field_key` | `string` | no | Campo de título; ausente = la copia del título de la entidad base, `null` = ninguno. |
| `owner_field_key` | `string` | no | Campo del titular; ausente = la copia del titular de la entidad base, `null` = ninguno. |

Otros campos y objetos anidados: `kobana schema dashboards.entities.join.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Entidad creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Conflicto: clave ya usada en el workspace, o la entidad es leída por una relación. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities join update

Modificar relación

```text
kobana dashboards entities join update [ID] [OPCIONES]
```

`PUT /v1/entities/{id}/join`

Cambia la relación de una entidad join y los campos que trae, y vuelve a leer sus registros. 422 cuando la entidad no es una relación.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | sí | Nombre de máquina que usan los bloques del portal; único en el workspace (hasta 63 caracteres). |
| `name` | `string` | sí |  |
| `name_plural` | `string` | sí |  |
| `join` | `object` | sí |  |
| `fields` | `array<object>` | sí | Los campos traídos de cada lado: `{ from: base\|related, field_key, key?, name? }`. |
| `description` | `string` | no |  |
| `label_field_key` | `string` | no | Campo de título; ausente = la copia del título de la entidad base, `null` = ninguno. |
| `owner_field_key` | `string` | no | Campo del titular; ausente = la copia del titular de la entidad base, `null` = ninguno. |

Otros campos y objetos anidados: `kobana schema dashboards.entities.join.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Entidad actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Conflicto: clave ya usada en el workspace, o la entidad es leída por una relación. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities logs get

Consultar llamada de la sincronización

```text
kobana dashboards entities logs get [ID] [LOG_ID] [OPCIONES]
```

`GET /v1/entities/{id}/logs/{log_id}`

Una llamada de la sincronización de la entidad, completa (secretos enmascarados).

**Argumentos de la ruta:** `id`, `log_id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La llamada. |
| `404` | Recurso no encontrado en la organización actual. |

## entities logs list

Registro de la sincronización

```text
kobana dashboards entities logs list [ID] [OPCIONES]
```

`GET /v1/entities/{id}/logs`

Las llamadas que la sincronización de la entidad hizo al conector, de las más nuevas a las más antiguas: el registro de solicitudes del conector restringido a la entidad.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `method` | `string` | no | Método HTTP de la llamada, como `GET`. |
| `success` | `string` | no | Solo llamadas exitosas (`true`) o fallidas (`false`). Valores: `true`, `false`. |
| `search` | `string` | no | Parte del endpoint llamado. |
| `date_from` | `string` | no | Primer día (AAAA-MM-DD). |
| `date_to` | `string` | no | Último día, inclusive (AAAA-MM-DD). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Llamadas de la sincronización. |
| `404` | Recurso no encontrado en la organización actual. |

## entities records list

Consultar registros

```text
kobana dashboards entities records list [ID] [OPCIONES]
```

`GET /v1/entities/{id}/records`

Los registros sincronizados de la entidad, con filtro por campo, ordenación y paginación por `limit`/`offset` (la misma consulta de los bloques del portal).

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `limit` | `integer` | no | Registros por llamada (por defecto 50, máximo 200). |
| `offset` | `integer,null` | no | Cuántos registros omitir. |
| `sort` | `string` | no | Claves de campo con dirección opcional, separadas por coma (`campo:desc,otro`); `asc` por defecto. |
| `filter` | `string` | no | Objeto JSON de condiciones por campo: `{"campo": valor}` o `{"campo": {"gte": 10}}`. Operadores: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `contains`, `is_null`. |
| `preview_as` | `string` | no | ID de un usuario del portal: solo sus registros, como los muestra el portal (en una entidad `on_access`, sincroniza ese titular antes). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registros de la entidad. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities records create

Enviar registros

```text
kobana dashboards entities records create [ID] [OPCIONES]
```

`POST /v1/entities/{id}/records`

Envía registros a una entidad sin conexión (`source_kind: "push"`): cada uno se guarda por su identificador — enviado de nuevo, se actualiza; sin cambios, no se reescribe. Las relaciones y los usuarios del portal que dependen de la entidad se actualizan después. 422 en una entidad que lee de una conexión.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `records` | `array<object>` | sí | Los registros (hasta 1000 por llamada), cada uno un objeto con las propiedades que leen los campos (`source_path`). Cada uno debe traer el identificador. |
| `mode` | `string` | no | `upsert` (predeterminado) mantiene los registros no enviados; `replace` elimina los que la llamada no trae — el envío pasa a ser el conjunto entero. Valores: `upsert`, `replace`. |

Otros campos y objetos anidados: `kobana schema dashboards.entities.records.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuántos registros se crearon, modificaron, mantuvieron y eliminaron, y los rechazados. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities records delete

Limpiar registros

```text
kobana dashboards entities records delete [ID] [OPCIONES]
```

`DELETE /v1/entities/{id}/records`

Elimina todos los registros sincronizados de la entidad y reinicia el estado de la sincronización, para que la próxima lea el origen desde cero.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registros eliminados. |
| `404` | Recurso no encontrado en la organización actual. |

## entities records import

Importar registros desde CSV

```text
kobana dashboards entities records import [ID] [OPCIONES]
```

`POST /v1/entities/{id}/records/import`

Importa un CSV a una entidad sin conexión: un registro por línea, cada columna que lee un campo convertida a su tipo (números, fechas, booleanos). Las líneas sin identificador se rechazan y se listan por número de línea (la 1 es el encabezado).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `content` | `string` | no | El texto del CSV: primera línea con los nombres de las columnas (el `source_path` de los campos), separador `,` o `;`, hasta 20.000 líneas. Envíe este o `content_base64`. |
| `content_base64` | `string` | no | El archivo CSV en base64 (UTF-8). |
| `mode` | `string` | no | `upsert` (predeterminado) mantiene los registros no enviados; `replace` elimina los que la llamada no trae — el envío pasa a ser el conjunto entero. Valores: `upsert`, `replace`. |

Otros campos y objetos anidados: `kobana schema dashboards.entities.records.import`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuántos registros se crearon, modificaron, mantuvieron y eliminaron, y los rechazados. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities records get

Consultar registro

```text
kobana dashboards entities records get [ID] [RECORD_ID] [OPCIONES]
```

`GET /v1/entities/{id}/records/{record_id}`

Un registro sincronizado — valores por campo y el registro tal como lo envió el origen — con su entidad.

**Argumentos de la ruta:** `id`, `record_id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El registro y su entidad. |
| `404` | Recurso no encontrado en la organización actual. |

## entities sync

Estado de la sincronización

```text
kobana dashboards entities sync [ID] [OPCIONES] [COMANDO]
```

`GET /v1/entities/{id}/sync`

Dónde está la sincronización de la entidad: estado, último error, últimas sincronizaciones, total de registros y estadísticas.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Estado de la sincronización. |
| `404` | Recurso no encontrado en la organización actual. |

## entities sync create

Sincronizar entidad

```text
kobana dashboards entities sync create [ID] [OPCIONES]
```

`POST /v1/entities/{id}/sync`

Encola una sincronización de la entidad y responde 202 con el ID; el avance está en `GET /entities/{id}/sync`. Una entidad `on_access` no tiene sincronización general (422): use `POST /entities/{id}/tenant_syncs`.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `full` | `string` | no | `true` (o `1`) vuelve a leer todo y descarta lo que el origen ya no tiene; sin él, la sincronización es incremental cuando el origen lo permite. Valores: `true`, `1`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Sincronización encolada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities tenant-syncs list

Sincronizaciones por titular

```text
kobana dashboards entities tenant-syncs list [ID] [OPCIONES]
```

`GET /v1/entities/{id}/tenant_syncs`

En una entidad `on_access`: dónde está la sincronización de cada titular, de los accedidos más recientemente a los más antiguos.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `status` | `string` | no | Estado de la sincronización del titular. Valores: `queued`, `running`, `completed`, `error`. |
| `search` | `string` | no | Parte del Tenant ID. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Sincronizaciones de los titulares. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## entities tenant-syncs create

Sincronizar un titular

```text
kobana dashboards entities tenant-syncs create [ID] [OPCIONES]
```

`POST /v1/entities/{id}/tenant_syncs`

Encola ahora la sincronización de un titular de una entidad `on_access`. 404 cuando ningún usuario del portal tiene ese Tenant ID; 422 cuando la entidad no es `on_access`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `tenant_id` | `string` | sí | Tenant ID (ID del titular) a sincronizar. Va en el cuerpo porque puede contener cualquier carácter. |
| `full` | `boolean` | no | Cuando es `true`, vuelve a leer todos los registros del titular. |

Otros campos y objetos anidados: `kobana schema dashboards.entities.tenant-syncs.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Sincronización encolada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities list

Listar entidades

```text
kobana dashboards entities list [OPCIONES]
```

`GET /v1/entities`

Todas las entidades del workspace, con campos, origen y estado de la sincronización, ordenadas por nombre. Sin paginación.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Entidades del workspace. |
| `404` | Recurso no encontrado en la organización actual. |

## entities create

Crear entidad

```text
kobana dashboards entities create [OPCIONES]
```

`POST /v1/entities`

Registra una entidad a partir de una operación de lectura (o consulta SQL) de la fuente de datos con los campos elegidos, y encola la primera sincronización completa.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | sí | Nombre de máquina que usan los bloques del portal; único en el workspace (hasta 63 caracteres). |
| `name` | `string` | sí |  |
| `name_plural` | `string` | sí |  |
| `fields` | `array<object>` | sí | Los campos que mantiene la entidad, en orden. Al actualizar, la lista completa (emparejada por `key`). |
| `integration_id` | `string (uuid)` | no | Fuente de datos (conector) de donde leer. Se omite con `source_kind: "push"`. |
| `operation_id` | `string (uuid)` | no | Operación de lectura (endpoint, tool o tabla) del origen. Envíe este campo o `sql`, no los dos. |
| `sql` | `string` | no | Consulta SQL en una fuente de base SQL. Envíe este campo o `operation_id`. |
| `params` | `object` | no | Parámetros con los que se llama a la operación (por defecto `{}`). |
| `pagination` | `object` | no |  |
| `sync_filters` | `array<object>` | no | Lo que la sincronización envía para filtrar los registros (hasta 20). Ignorado en una consulta SQL. |
| `source_kind` | `string` | no | `push`: entidad sin conexión — sin `integration_id`, `operation_id` ni `sql`. El `source_path` de cada campo vale la clave del campo cuando se omite (la propiedad que trae cada registro enviado) y `identifier_field_key` es obligatorio: un registro enviado de nuevo actualiza el existente por él. Valores: `push`. |
| `description` | `string` | no |  |
| `identifier_field_key` | `string` | no |  |
| `label_field_key` | `string` | no |  |
| `owner_field_key` | `string` | no | Campo con el ID del titular, emparejado con el `tenant_id` del usuario del portal. Sin él la entidad no aparece en el portal. |
| `owner_match` | `string` | no | Cómo se compara el campo del titular con el `tenant_id`: como texto (`external_id`) o como CPF/CNPJ (`document_number`, solo dígitos). Valores: `external_id`, `document_number`. |
| `sync_mode` | `string` | no | Cuándo se ejecuta la sincronización; ausente: `scheduled` al crear, se mantiene al actualizar. Valores: `scheduled`, `manual`, `on_access`. |
| `freshness_seconds` | `integer` | no | `on_access`: durante cuántos segundos los registros sincronizados de un titular cuentan como actuales. |
| `records_path` | `string` | no | Ruta de la lista de registros dentro de la respuesta, cuando no se detecta. |
| `sample` | `array<object>` | no | Los registros de la vista previa, guardados para volver a inferir campos después. |

Otros campos y objetos anidados: `kobana schema dashboards.entities.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Entidad creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Conflicto: clave ya usada en el workspace, o la entidad es leída por una relación. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities preview

Vista previa de un origen

```text
kobana dashboards entities preview [OPCIONES]
```

`POST /v1/entities/preview`

Llama a la operación de lectura elegida (o ejecuta la consulta SQL) y devuelve la muestra con la entidad inferida de ella. No se guarda nada; la llamada queda en el registro de solicitudes del conector.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `integration_id` | `string (uuid)` | sí | Fuente de datos (conector) de donde leer. Se omite con `source_kind: "push"`. |
| `operation_id` | `string (uuid)` | no | Operación de lectura (endpoint, tool o tabla) del origen. Envíe este campo o `sql`, no los dos. |
| `sql` | `string` | no | Consulta SQL en una fuente de base SQL. Envíe este campo o `operation_id`. |
| `params` | `object` | no | Parámetros con los que se llama a la operación (por defecto `{}`). |
| `pagination` | `object` | no |  |
| `sync_filters` | `array<object>` | no | Lo que la sincronización envía para filtrar los registros (hasta 20). Ignorado en una consulta SQL. |
| `limit` | `integer` | no | Tamaño de la muestra (por defecto 10, máximo 50). |

Otros campos y objetos anidados: `kobana schema dashboards.entities.preview`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Muestra y entidad inferida. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities get

Consultar entidad

```text
kobana dashboards entities get [ID] [OPCIONES]
```

`GET /v1/entities/{id}`

Una entidad con campos, origen, modo de sincronización, campo del titular y estado de la sincronización.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La entidad. |
| `404` | Recurso no encontrado en la organización actual. |

## entities update

Actualizar entidad

```text
kobana dashboards entities update [ID] [OPCIONES]
```

`PATCH /v1/entities/{id}`

Nombre, clave, campos especiales, paginación, filtros y modo de la sincronización, y la lista de campos (emparejada por clave). Cuando cambia el mapeo, los registros se vuelven a leer.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | sí | Nombre de máquina que usan los bloques del portal; único en el workspace (hasta 63 caracteres). |
| `name` | `string` | sí |  |
| `name_plural` | `string` | sí |  |
| `fields` | `array<object>` | sí | Los campos que mantiene la entidad, en orden. Al actualizar, la lista completa (emparejada por `key`). |
| `description` | `string` | no |  |
| `identifier_field_key` | `string` | no |  |
| `label_field_key` | `string` | no |  |
| `owner_field_key` | `string` | no | Campo con el ID del titular, emparejado con el `tenant_id` del usuario del portal. Sin él la entidad no aparece en el portal. |
| `owner_match` | `string` | no | Cómo se compara el campo del titular con el `tenant_id`: como texto (`external_id`) o como CPF/CNPJ (`document_number`, solo dígitos). Valores: `external_id`, `document_number`. |
| `sync_mode` | `string` | no | Cuándo se ejecuta la sincronización; ausente: `scheduled` al crear, se mantiene al actualizar. Valores: `scheduled`, `manual`, `on_access`. |
| `freshness_seconds` | `integer` | no | `on_access`: durante cuántos segundos los registros sincronizados de un titular cuentan como actuales. |
| `pagination` | `object` | no |  |
| `sync_filters` | `array<object>` | no | Filtros de la sincronización (hasta 20); ausente mantiene los actuales. |

Otros campos y objetos anidados: `kobana schema dashboards.entities.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Entidad actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Conflicto: clave ya usada en el workspace, o la entidad es leída por una relación. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities delete

Eliminar entidad

```text
kobana dashboards entities delete [ID] [OPCIONES]
```

`DELETE /v1/entities/{id}`

Elimina (soft delete) la entidad y sus campos. Rechazado (409) mientras una entidad de relación la lee.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Entidad eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Conflicto: clave ya usada en el workspace, o la entidad es leída por una relación. |

## entities aggregate

Calcular agregados de los registros

```text
kobana dashboards entities aggregate [ID] [OPCIONES]
```

`GET /v1/entities/{id}/aggregate`

Recuento, suma, promedio, mínimo o máximo sobre los registros sincronizados, opcionalmente agrupados por un campo — lo que muestran los bloques de KPI y gráfico.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `op` | `string` | no | Agregación: `count` (por defecto), `sum`, `avg`, `min` o `max`. Valores: `count`, `sum`, `avg`, `min`, `max`. |
| `field` | `string` | no | Clave del campo agregado (obligatoria excepto en `count`; numérico en `sum`/`avg`). |
| `group_by` | `string` | no | Clave del campo por el cual agrupar. |
| `filter` | `string` | no | Objeto JSON de condiciones por campo: `{"campo": valor}` o `{"campo": {"gte": 10}}`. Operadores: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `contains`, `is_null`. |
| `preview_as` | `string` | no | ID de un usuario del portal: solo sus registros, como los muestra el portal (en una entidad `on_access`, sincroniza ese titular antes). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la agregación. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## entities usage

Uso de la entidad

```text
kobana dashboards entities usage [ID] [OPCIONES]
```

`GET /v1/entities/{id}/usage`

Las páginas del portal (borrador y publicada) cuyos bloques leen la entidad — lo que se rompe si se elimina o cambia su clave.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Páginas que usan la entidad. |
| `404` | Recurso no encontrado en la organización actual. |
