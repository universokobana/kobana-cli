---
id: proposal-templates
title: "kobana billing proposal-templates"
sidebar_label: "proposal-templates"
---

# kobana billing proposal-templates


Comandos de `kobana billing proposal-templates`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`proposal-templates list`](#proposal-templates-list) | `GET /v1/proposal_templates` | Listar plantillas de propuesta |
| [`proposal-templates create`](#proposal-templates-create) | `POST /v1/proposal_templates` | Crear plantilla de propuesta |
| [`proposal-templates update`](#proposal-templates-update) | `PUT /v1/proposal_templates` | Crear o actualizar plantilla de propuesta |
| [`proposal-templates get`](#proposal-templates-get) | `GET /v1/proposal_templates/{id}` | Obtener plantilla de propuesta |
| [`proposal-templates delete`](#proposal-templates-delete) | `DELETE /v1/proposal_templates/{id}` | Eliminar plantilla de propuesta |
| [`proposal-templates activate`](#proposal-templates-activate) | `POST /v1/proposal_templates/{id}/activate` | Activar plantilla de propuesta |
| [`proposal-templates deactivate`](#proposal-templates-deactivate) | `POST /v1/proposal_templates/{id}/deactivate` | Desactivar plantilla de propuesta |
| [`proposal-templates duplicate`](#proposal-templates-duplicate) | `POST /v1/proposal_templates/{id}/duplicate` | Duplicar plantilla de propuesta |

## proposal-templates list

Listar plantillas de propuesta

```text
kobana billing proposal-templates list [OPCIONES]
```

`GET /v1/proposal_templates`

Devuelve una lista paginada de las plantillas de propuesta de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda textual por nombre o descripción. |
| `is_active` | `boolean` | no | Filtra por plantillas activas (`true`) o desactivadas (`false`). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de plantillas de propuesta. |
| `404` | Recurso no encontrado en la organización actual. |

## proposal-templates create

Crear plantilla de propuesta

```text
kobana billing proposal-templates create [OPCIONES]
```

`POST /v1/proposal_templates`

Crea una plantilla de propuesta vinculada a un Google Doc o Google Slides. El `google_doc_id` debe ser único por organización.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la plantilla (único por organización). |
| `google_doc_id` | `string` | sí | ID del Google Doc/Slides usado como plantilla (único por organización). |
| `description` | `string` | no | Descripción opcional de la plantilla. |
| `document_type` | `string` | no | Tipo de documento base: `google_doc` o `google_slides`. Por defecto: `google_doc`. Valores: `google_doc`, `google_slides`. |
| `app_script_url` | `string (uri)` | no | URL del Apps Script publicado que renderiza la plantilla, cuando aplique. |
| `google_drive_folder_id` | `string` | no | ID de la carpeta de Google Drive donde se guardarán las copias generadas. |
| `is_active` | `boolean` | no | Cuando es `false`, crea la plantilla ya desactivada. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos libres definidos por el cliente para correlación interna. |

Otros campos y objetos anidados: `kobana schema billing.proposal-templates.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Plantilla de propuesta creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposal-templates update

Crear o actualizar plantilla de propuesta

```text
kobana billing proposal-templates update [OPCIONES]
```

`PUT /v1/proposal_templates`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Nombre de la plantilla (único por organización). |
| `description` | `string` | no | Descripción opcional de la plantilla. |
| `document_type` | `string` | no | Tipo de documento base: `google_doc` o `google_slides`. Por defecto: `google_doc`. Valores: `google_doc`, `google_slides`. |
| `google_doc_id` | `string` | no | ID del Google Doc/Slides usado como plantilla (único por organización). |
| `app_script_url` | `string (uri)` | no | URL del Apps Script publicado que renderiza la plantilla, cuando aplique. |
| `google_drive_folder_id` | `string` | no | ID de la carpeta de Google Drive donde se guardarán las copias generadas. |
| `is_active` | `boolean` | no | Cuando es `false`, crea la plantilla ya desactivada. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos libres definidos por el cliente para correlación interna. |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.proposal-templates.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposal-templates get

Obtener plantilla de propuesta

```text
kobana billing proposal-templates get [ID] [OPCIONES]
```

`GET /v1/proposal_templates/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la plantilla de propuesta. |
| `404` | Recurso no encontrado en la organización actual. |

## proposal-templates delete

Eliminar plantilla de propuesta

```text
kobana billing proposal-templates delete [ID] [OPCIONES]
```

`DELETE /v1/proposal_templates/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plantilla de propuesta eliminada. |
| `404` | Recurso no encontrado en la organización actual. |

## proposal-templates activate

Activar plantilla de propuesta

```text
kobana billing proposal-templates activate [ID] [OPCIONES]
```

`POST /v1/proposal_templates/{id}/activate`

Activa una plantilla de propuesta previamente desactivada, dejándola disponible para usar en nuevas propuestas.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plantilla de propuesta activada. |
| `404` | Recurso no encontrado en la organización actual. |

## proposal-templates deactivate

Desactivar plantilla de propuesta

```text
kobana billing proposal-templates deactivate [ID] [OPCIONES]
```

`POST /v1/proposal_templates/{id}/deactivate`

Desactiva una plantilla de propuesta. Las plantillas desactivadas no pueden usarse en nuevas propuestas, pero las propuestas ya creadas permanecen sin cambios.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Plantilla de propuesta desactivada. |
| `404` | Recurso no encontrado en la organización actual. |

## proposal-templates duplicate

Duplicar plantilla de propuesta

```text
kobana billing proposal-templates duplicate [ID] [OPCIONES]
```

`POST /v1/proposal_templates/{id}/duplicate`

Crea una copia de la plantilla indicada, manteniendo la configuración y los metadatos. La nueva plantilla recibe un nombre derivado.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Plantilla de propuesta duplicada. |
| `404` | Recurso no encontrado en la organización actual. |
