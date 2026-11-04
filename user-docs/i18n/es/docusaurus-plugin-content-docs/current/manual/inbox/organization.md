---
id: organization
title: "kobana inbox organization"
sidebar_label: "organization"
description: "Obtener organización"
---

# kobana inbox organization

Obtener organización

Comandos de `kobana inbox organization`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Obtener organización |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Actualizar organización |

## organization

Obtener organización

```text
kobana inbox organization [OPCIONES] [COMANDO]
```

`GET /v1/organization`

La organización sale solo del token (el claim `org`, como en `GET /v1/me`): no hay lista ni `{id}`, y ningún header, query o cuerpo elige otra. Nivel de organización: no hace falta `X-Workspace-Id`. Requiere el permiso `settings.show` (todos los roles de sistema lo tienen).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |

## organization update

Actualizar organización

```text
kobana inbox organization update [OPCIONES]
```

`PATCH /v1/organization`

La organización sale solo del token (el claim `org`, como en `GET /v1/me`): no hay lista ni `{id}`, y ningún header, query o cuerpo elige otra. Nivel de organización: no hace falta `X-Workspace-Id`. Cambia el perfil común a todos los workspaces: `custom_metadata`. Las claves desconocidas, los campos de identidad (`id`, `public_id`, `external_id`, `slug`, `document_type`, `document_number`) y los espejos de la cuenta Kobana (`name`, `email`, `phone` — sincronizados desde ella, por eso no se editan aquí) se ignoran. Mencionar una configuración comercial de Kobana (cortesía, liberación sin suscripción, límites, socio, entitlements) se rechaza con 403 `FORBIDDEN` (`error.details.fields`). Requiere el permiso `settings.update` (propietario y admin). Cada cambio queda en el registro de auditoría de la organización.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `custom_metadata` | `object` | no | Metadatos del usuario, formato libre. `null` conserva el valor guardado. Máximo 64 KB serializado, 16 niveles de profundidad. |
| `custom_metadata_mode` | `string` | no | Cómo se reconcilia el `custom_metadata` enviado con el valor almacenado. `replace` (por defecto) sobrescribe el objeto completo; `merge` hace deep-merge sobre el valor actual — los objetos anidados se fusionan recursivamente, escalares y arrays sobrescriben la clave, y las claves `null` se conservan (no eliminan la clave). Solo en update. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema inbox.organization.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito. |
| `400` | El cuerpo no superó la validación de Zod. `error.details` contiene `formErrors`/`fieldErrors`. |
