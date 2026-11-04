---
id: data-sources
title: "kobana dashboards data-sources"
sidebar_label: "data-sources"
---

# kobana dashboards data-sources


Comandos de `kobana dashboards data-sources`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`data-sources catalog list`](#data-sources-catalog-list) | `GET /v1/data_sources/catalog` | Listar productos del catálogo Kobana |
| [`data-sources catalog create`](#data-sources-catalog-create) | `POST /v1/data_sources/catalog` | Conectar producto Kobana |
| [`data-sources logs get`](#data-sources-logs-get) | `GET /v1/data_sources/{id}/logs/{log_id}` | Consultar solicitud de la fuente |
| [`data-sources logs list`](#data-sources-logs-list) | `GET /v1/data_sources/{id}/logs` | Listar log de solicitudes de la fuente |
| [`data-sources openapi analyze`](#data-sources-openapi-analyze) | `POST /v1/data_sources/openapi/analyze` | Analizar spec OpenAPI |
| [`data-sources operations update`](#data-sources-operations-update) | `PATCH /v1/data_sources/{id}/operations/{operation_id}` | Activar o desactivar operación de la fuente |
| [`data-sources list`](#data-sources-list) | `GET /v1/data_sources` | Listar fuentes de datos |
| [`data-sources create`](#data-sources-create) | `POST /v1/data_sources` | Crear fuente de datos |
| [`data-sources test`](#data-sources-test) | `POST /v1/data_sources/test` | Probar conexión antes de guardar |
| [`data-sources get`](#data-sources-get) | `GET /v1/data_sources/{id}` | Consultar fuente de datos |
| [`data-sources update`](#data-sources-update) | `PATCH /v1/data_sources/{id}` | Actualizar fuente de datos |
| [`data-sources delete`](#data-sources-delete) | `DELETE /v1/data_sources/{id}` | Eliminar fuente de datos |
| [`data-sources authorize`](#data-sources-authorize) | `POST /v1/data_sources/{id}/authorize` | Autorizar fuente MCP |
| [`data-sources schedule`](#data-sources-schedule) | `PUT /v1/data_sources/{id}/schedule` | Agendar sincronización de la fuente |
| [`data-sources spec`](#data-sources-spec) | `PUT /v1/data_sources/{id}/spec` | Reemplazar spec de la fuente |
| [`data-sources sync`](#data-sources-sync) | `POST /v1/data_sources/{id}/sync` | Sincronizar fuente de datos |

## data-sources catalog list

Listar productos del catálogo Kobana

```text
kobana dashboards data-sources catalog list [OPCIONES]
```

`GET /v1/data_sources/catalog`

Productos Kobana que se pueden conectar como fuente de datos solo con un token. Lista completa, sin paginación.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Productos del catálogo. |
| `404` | Recurso no encontrado en la organización actual. |

## data-sources catalog create

Conectar producto Kobana

```text
kobana dashboards data-sources catalog create [OPCIONES]
```

`POST /v1/data_sources/catalog`

Guarda una fuente OpenAPI a partir de la spec del producto incluida en la aplicación y prueba la conexión.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `product` | `string` | sí | Clave del producto del catálogo Kobana (vea `GET /data_sources/catalog`). |
| `environment` | `string` | no | Valores: `production`, `sandbox`. |
| `token` | `string` | no | Token de API del producto. Se guarda cifrado. |
| `workspace_id` | `string` | no | Workspace dentro del producto, cuando la organización del token tiene más de uno. |
| `name` | `string` | no | Nombre visible, único por workspace y tipo. Si se omite, se deriva de la spec, del host o de la base. |

Otros campos y objetos anidados: `kobana schema dashboards.data-sources.catalog.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Fuente creada, con el resultado de la prueba de conexión. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Ya existe una fuente de datos con este nombre en el workspace. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources logs get

Consultar solicitud de la fuente

```text
kobana dashboards data-sources logs get [ID] [LOG_ID] [OPCIONES]
```

`GET /v1/data_sources/{id}/logs/{log_id}`

Una llamada completa: cabeceras y cuerpos de la solicitud y de la respuesta, con secretos enmascarados.

**Argumentos de la ruta:** `id`, `log_id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Entrada del log de solicitudes. |
| `404` | Recurso no encontrado en la organización actual. |

## data-sources logs list

Listar log de solicitudes de la fuente

```text
kobana dashboards data-sources logs list [ID] [OPCIONES]
```

`GET /v1/data_sources/{id}/logs`

Llamadas que hizo la fuente (solicitudes HTTP, consultas a la base), de las más nuevas a las más antiguas, sin cabeceras ni cuerpos.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `method` | `string` | no | Método HTTP (GET, POST…). |
| `success` | `string` | no | `true` solo las llamadas exitosas, `false` solo las que fallaron. Valores: `true`, `false`. |
| `search` | `string` | no | Fragmento del endpoint, sin distinguir mayúsculas. |
| `date_from` | `string` | no | Desde este día (YYYY-MM-DD, UTC), inclusive. |
| `date_to` | `string` | no | Hasta este día (YYYY-MM-DD, UTC), inclusive. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página del log de solicitudes. |
| `404` | Recurso no encontrado en la organización actual. |

## data-sources openapi analyze

Analizar spec OpenAPI

```text
kobana dashboards data-sources openapi analyze [OPCIONES]
```

`POST /v1/data_sources/openapi/analyze`

Lee la spec y devuelve lo que se importaría — título, servidores, esquemas de seguridad, autenticación sugerida y operaciones — sin guardar nada.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `spec` | `string` | sí | El documento OpenAPI 3.x o Swagger 2.0 como texto (JSON o YAML), hasta 5 MB — limitado también por los 2 MB de cuerpo de la API. |
| `spec_file_name` | `string` | no | Nombre del archivo de la spec (`openapi.yaml`). Si se omite, se deduce del contenido. |

Otros campos y objetos anidados: `kobana schema dashboards.data-sources.openapi.analyze`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Análisis de la spec. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources operations update

Activar o desactivar operación de la fuente

```text
kobana dashboards data-sources operations update [ID] [OPERATION_ID] [OPCIONES]
```

`PATCH /v1/data_sources/{id}/operations/{operation_id}`

Activa o desactiva una operación de la fuente (`enabled`). Las operaciones desactivadas no alimentan entidades.

**Argumentos de la ruta:** `id`, `operation_id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `enabled` | `boolean` | sí | Las operaciones desactivadas no alimentan entidades. |

Otros campos y objetos anidados: `kobana schema dashboards.data-sources.operations.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Operación actualizada. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources list

Listar fuentes de datos

```text
kobana dashboards data-sources list [OPCIONES]
```

`GET /v1/data_sources`

Fuentes de datos del workspace — APIs OpenAPI, servidores MCP y bases de solo lectura —, de las más nuevas a las más antiguas. Los secretos nunca se devuelven. Para crear una fuente, use la ruta de su tipo: `/data_sources/openapi`, `/mcp`, `/database` o `/catalog`.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de fuentes de datos. |
| `404` | Recurso no encontrado en la organización actual. |

## data-sources create

Crear fuente de datos

```text
kobana dashboards data-sources create [OPCIONES]
```

`POST /v1/data_sources`

Guarda una fuente de datos y prueba la conexión. `kind` dice cuál: `openapi` (API descrita por una spec OpenAPI — la spec va como texto en `spec`, el certificado mTLS en base64 en `certificate`; importa las operaciones), `mcp` (servidor MCP — `url`; sin OAuth conecta y descubre las tools al momento, con OAuth devuelve `authorization_url`, que una persona con sesión en el dashboard abre en el navegador para terminar) o `database` (base de datos de solo lectura — `engine` y conexión; cataloga sus tablas o colecciones). Envíe solo los campos del `kind` elegido. Los productos Kobana entran por `POST /data_sources/catalog`. Sustituye a `POST /data_sources/openapi`, `/mcp` y `/database`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema dashboards.data-sources.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Fuente creada, con el resultado de la prueba de conexión. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Ya existe una fuente de datos con este nombre en el workspace. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources test

Probar conexión antes de guardar

```text
kobana dashboards data-sources test [OPCIONES]
```

`POST /v1/data_sources/test`

Prueba una conexión aún no guardada. Informe `kind` (`openapi`, `mcp`, `database` o `catalog`) y el cuerpo de la ruta de creación correspondiente. No se persiste nada.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema dashboards.data-sources.test`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la prueba de conexión. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources get

Consultar fuente de datos

```text
kobana dashboards data-sources get [ID] [OPCIONES]
```

`GET /v1/data_sources/{id}`

La fuente de datos y sus operaciones (endpoints, tools o tablas que puede leer).

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Fuente de datos con sus operaciones. |
| `404` | Recurso no encontrado en la organización actual. |

## data-sources update

Actualizar fuente de datos

```text
kobana dashboards data-sources update [ID] [OPCIONES]
```

`PATCH /v1/data_sources/{id}`

Actualiza nombre, `is_active` y los campos de conexión del tipo de la fuente. Los campos secretos en blanco mantienen los guardados. Una fuente OpenAPI recibe un nuevo certificado en `certificate`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre visible, único por workspace y tipo. Si se omite, se deriva de la spec, del host o de la base. |
| `is_active` | `boolean` | no |  |
| `base_url` | `string` | no | Base URL de la API. Si se omite, usa el primer servidor absoluto de la spec. |
| `auth` | `object` | no | Cómo autentica la API: `none`, `header`, `query`, `bearer`, `basic` o `cert`. En `header`/`query`, informe los nombres y valores de las claves; un valor en blanco en una actualización mantiene el guardado. |
| `credentials` | `object` | no | Secretos de la autenticación elegida: `token` (bearer), `username`/`password` (basic), `certificate_password` (cert). Se guardan cifrados y nunca se devuelven. |
| `certificate` | `object` | no | Certificado de cliente para mTLS (`auth.kind = "cert"`). |
| `probe_operation_key` | `string` | no | OpenAPI: operación usada como sonda en la prueba de conexión; `null` elige una automáticamente. |
| `default_params` | `object` | no | OpenAPI: valores por defecto de parámetros enviados en cada llamada. |
| `response_format` | `string` | no | Valores: `json`, `xml`. |
| `url` | `string` | no | URL del servidor MCP (endpoint Streamable HTTP o SSE). Un token en la query string se guarda cifrado. |
| `transport` | `string` | no | Valores: `http`, `sse`. |
| `headers` | `anyOf` | no | Cabeceras extra enviadas en cada llamada: un objeto, o un `Nombre: valor` por línea. Se guardan cifradas. |
| `oauth` | `object` | no | OAuth del servidor MCP: `none`, `dcr` (registro dinámico de cliente) o `client` (su propio client id/secret). |
| `engine` | `string` | no | Valores: `postgres`, `mysql`, `mariadb`, `mssql`, `mongodb`. |
| `host` | `string` | no |  |
| `port` | `anyOf` | no |  |
| `database` | `string` | no |  |
| `username` | `string` | no |  |
| `password` | `string` | no |  |
| `connection_string` | `string` | no | Solo MongoDB: la cadena de conexión completa (en lugar de host, puerto, usuario y contraseña). Autenticación solo por usuario y contraseña (SCRAM); opciones aceptadas: authSource, authMechanism (SCRAM-SHA-1/SCRAM-SHA-256), replicaSet, tls/ssl, retryWrites, retryReads, w, journal, wtimeoutMS, readPreference, readConcernLevel, appName, directConnection, loadBalanced, compressors, zlibCompressionLevel y los tiempos/límites de pool (maxPoolSize, minPoolSize, maxIdleTimeMS, connectTimeoutMS, socketTimeoutMS, serverSelectionTimeoutMS, heartbeatFrequencyMS, localThresholdMS, maxStalenessSeconds). Cualquier otra opción se rechaza con 422. |
| `ssl` | `boolean` | no |  |
| `ssl_verify` | `boolean` | no |  |

Otros campos y objetos anidados: `kobana schema dashboards.data-sources.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Fuente de datos actualizada. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Ya existe una fuente de datos con este nombre en el workspace. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources delete

Eliminar fuente de datos

```text
kobana dashboards data-sources delete [ID] [OPCIONES]
```

`DELETE /v1/data_sources/{id}`

Elimina la fuente, sus operaciones y la agenda de sincronización. Rechazado (409) mientras una entidad lee la fuente: elimine las entidades antes. No se puede deshacer.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Fuente eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Una o más entidades leen esta fuente de datos. |

## data-sources authorize

Autorizar fuente MCP

```text
kobana dashboards data-sources authorize [ID] [OPCIONES]
```

`POST /v1/data_sources/{id}/authorize`

Reinicia la autorización OAuth de un servidor MCP. Devuelve `authorization_url` para que una persona con sesión en el dashboard la abra en el navegador, o `null` y la prueba de conexión cuando los tokens guardados aún funcionan.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | URL de autorización o resultado de la prueba. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources schedule

Agendar sincronización de la fuente

```text
kobana dashboards data-sources schedule [ID] [OPCIONES]
```

`PUT /v1/data_sources/{id}/schedule`

Define con qué frecuencia sincroniza la fuente: `{ cron, timezone }`, o `{ cron: null }` para solo bajo demanda. Devuelve las próximas ejecuciones.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cron` | `string` | sí | Cron de 5 campos (como máximo cada 5 minutos), o `null` para sincronizar solo bajo demanda. |
| `timezone` | `string` | no | Zona horaria IANA de la agenda. Por defecto: la actual de la fuente, o America/Sao_Paulo. |

Otros campos y objetos anidados: `kobana schema dashboards.data-sources.schedule`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Agenda guardada. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources spec

Reemplazar spec de la fuente

```text
kobana dashboards data-sources spec [ID] [OPCIONES]
```

`PUT /v1/data_sources/{id}/spec`

Reemplaza la spec OpenAPI y resincroniza las operaciones: las que mantienen la clave mantienen su configuración; las que desaparecieron de la spec se eliminan.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `spec` | `string` | sí | El documento OpenAPI 3.x o Swagger 2.0 como texto (JSON o YAML), hasta 5 MB — limitado también por los 2 MB de cuerpo de la API. |
| `spec_file_name` | `string` | no | Nombre del archivo de la spec (`openapi.yaml`). Si se omite, se deduce del contenido. |

Otros campos y objetos anidados: `kobana schema dashboards.data-sources.spec`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Spec reemplazada, con los avisos del análisis. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## data-sources sync

Sincronizar fuente de datos

```text
kobana dashboards data-sources sync [ID] [OPCIONES]
```

`POST /v1/data_sources/{id}/sync`

Encola la sincronización de todas las entidades alimentadas por la fuente. La ejecución es asíncrona.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `full` | `boolean` | no | `true` vuelve a leer todo en lugar de solo lo que cambió desde la última sincronización. |

Otros campos y objetos anidados: `kobana schema dashboards.data-sources.sync`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Sincronización encolada. |
| `400` | Cuerpo que no es JSON, o campo con tipo incorrecto. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
