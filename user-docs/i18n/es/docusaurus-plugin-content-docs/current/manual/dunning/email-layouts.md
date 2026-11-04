---
id: email-layouts
title: "kobana dunning email-layouts"
sidebar_label: "email-layouts"
description: "Listar layouts de correo"
---

# kobana dunning email-layouts

Listar layouts de correo

Comandos de `kobana dunning email-layouts`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`email-layouts`](#email-layouts) | `GET /v1/email-layouts` | Listar layouts de correo |
| [`email-layouts create`](#email-layouts-create) | `POST /v1/email-layouts` | Crear layout de correo |

## email-layouts

Listar layouts de correo

```text
kobana dunning email-layouts [OPCIONES] [COMANDO]
```

`GET /v1/email-layouts`

Lista los layouts de correo de la organización (predeterminado primero) y el layout integrado de referencia.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de layouts. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## email-layouts create

Crear layout de correo

```text
kobana dunning email-layouts create [OPCIONES]
```

`POST /v1/email-layouts`

Crea un layout de correo. El `html_body` debe contener el placeholder `{{content}}` (si no, 400 `MISSING_CONTENT_PLACEHOLDER`). `isDefault: true` desmarca el predeterminado anterior.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del layout. |
| `html_body` | `string` | sí | HTML completo del layout; debe contener el placeholder `{{content}}`. |
| `is_default` | `boolean` | no | Layout predeterminado de la organización (solo uno; definir uno nuevo desmarca el anterior). |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |

Otros campos y objetos anidados: `kobana schema dunning.email-layouts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Layout creado. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
