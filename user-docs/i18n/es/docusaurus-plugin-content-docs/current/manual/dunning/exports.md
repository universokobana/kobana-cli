---
id: exports
title: "kobana dunning exports"
sidebar_label: "exports"
---

# kobana dunning exports


Comandos de `kobana dunning exports`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`exports charges`](#exports-charges) | `GET /v1/exports/charges` | Exportar cobros (CSV síncrono) |
| [`exports jobs list`](#exports-jobs-list) | `GET /v1/exports/jobs` | Listar exportaciones asíncronas |
| [`exports jobs create`](#exports-jobs-create) | `POST /v1/exports/jobs` | Crear exportación asíncrona |
| [`exports jobs download`](#exports-jobs-download) | `GET /v1/exports/jobs/{id}/download` | Descargar CSV de la exportación |
| [`exports people`](#exports-people) | `GET /v1/exports/people` | Exportar clientes (CSV síncrono) |

## exports charges

Exportar cobros (CSV síncrono)

```text
kobana dunning exports charges [OPCIONES]
```

`GET /v1/exports/charges`

Genera y responde al instante el CSV de cobros (`text/csv; charset=utf-8` con BOM, `Content-Disposition: attachment`), con filtros opcionales de estado y período de vencimiento. Columnas: cliente, documento_cliente, numero_documento, descricao, valor_original, valor_atual, vencimento, dias_atraso, status, origem, external_id. Tope de 10 mil líneas; por encima usa la exportación asíncrona (POST /exports/jobs). Exige el nivel de acceso `export` (o `all`) — `read` y `write` no bastan.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Filtra los cobros por estado (ej.: `pending`, `overdue`, `paid`). |
| `from` | `string` | no | Vencimiento desde (fecha ISO `YYYY-MM-DD`). |
| `to` | `string` | no | Vencimiento hasta (fecha ISO `YYYY-MM-DD`). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivo CSV (`text/csv; charset=utf-8` con BOM para que Excel muestre los acentos correctamente; `Content-Disposition: attachment`). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## exports jobs list

Listar exportaciones asíncronas

```text
kobana dunning exports jobs list [OPCIONES]
```

`GET /v1/exports/jobs`

Historial de las 50 exportaciones asíncronas más recientes de la organización (sin paginación). Usa esta ruta para seguir el estado de los jobs; el CSV en sí no viene en el listado. Es una lectura (`read`): el archivo en sí exige `export`.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de las exportaciones más recientes. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## exports jobs create

Crear exportación asíncrona

```text
kobana dunning exports jobs create [OPCIONES]
```

`POST /v1/exports/jobs`

Agenda la generación del CSV en un worker, sin el tope de 10 mil líneas del modo síncrono y sin bloquear la request. Sigue el estado en GET /exports/jobs y descarga el archivo en GET /exports/jobs/\{id\}/download antes de `expires_at`. Responde 202. Exige el nivel de acceso `export` (o `all`) — `read` y `write` no bastan.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `resource_type` | `string` | sí | Tipo de recurso exportado (`people` = clientes \| `charges` = cobros). Valores: `people`, `charges`. |
| `file_name` | `string` | no | Nombre del archivo generado (por defecto: `{resourceType}.csv`). |
| `filters` | `object` | no | Filtros aplicados a la exportación (solo `charges`); quedan guardados en el job para auditoría. |

Otros campos y objetos anidados: `kobana schema dunning.exports.jobs.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Exportación aceptada y encolada para procesamiento. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## exports jobs download

Descargar CSV de la exportación

```text
kobana dunning exports jobs download [ID] [OPCIONES]
```

`GET /v1/exports/jobs/{id}/download`

Entrega el CSV ya generado de una exportación asíncrona (`text/csv; charset=utf-8` con BOM). Solo disponible cuando el job está `completed` y dentro de la validez (`expires_at`). Exige el nivel de acceso `export` (o `all`) — `read` y `write` no bastan.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivo CSV (`text/csv; charset=utf-8` con BOM para que Excel muestre los acentos correctamente; `Content-Disposition: attachment`). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `409` | Exportación aún no concluida (código `NOT_READY`; el campo `status` informa el estado actual). |
| `410` | Exportación expirada — genera una nueva (código `EXPIRED`). |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## exports people

Exportar clientes (CSV síncrono)

```text
kobana dunning exports people [OPCIONES]
```

`GET /v1/exports/people`

Genera y responde al instante el CSV de clientes (`text/csv; charset=utf-8` con BOM, `Content-Disposition: attachment`). Columnas: nome, documento, tipo_documento, email, telefone, classificacao, external_id, tags, criado_em. Tope de 10 mil líneas; por encima usa la exportación asíncrona (POST /exports/jobs). Exige el nivel de acceso `export` (o `all`) — `read` y `write` no bastan.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Archivo CSV (`text/csv; charset=utf-8` con BOM para que Excel muestre los acentos correctamente; `Content-Disposition: attachment`). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
