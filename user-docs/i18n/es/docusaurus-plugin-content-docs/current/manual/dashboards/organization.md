---
id: organization
title: "kobana dashboards organization"
sidebar_label: "organization"
description: "Obtener organización actual"
---

# kobana dashboards organization

Obtener organización actual

Comandos de `kobana dashboards organization`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Obtener organización actual |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Actualizar organización actual |

## organization

Obtener organización actual

```text
kobana dashboards organization [OPCIONES] [COMANDO]
```

`GET /v1/organization`

Devuelve los datos de la organización a la que pertenece el token autenticado. No hay parámetro `:id` — la organización siempre se resuelve a partir de la clave de la API.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la organización actual. |
| `404` | Recurso no encontrado en la organización actual. |

## organization update

Actualizar organización actual

```text
kobana dashboards organization update [OPCIONES]
```

`PATCH /v1/organization`

Actualiza el perfil de la organización autenticada. Solo `addresses` y `custom_metadata` son editables (`custom_metadata: null` mantiene el valor actual). `name`, `email` y `phone` reflejan la Cuenta Kobana: solo la sincronización con la Cuenta Kobana los escribe, y un cuerpo que los trae responde 200 sin cambiarlos. La identidad (`id`, `public_id`, `external_id`, `status`, `slug`) también se ignora. Los campos comerciales (como `bypass_subscription`, `courtesy` y `monthly_cost_limit`) los define Kobana y responden 403.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `addresses` | `array<object>` | no | Lista de direcciones de la organización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema dashboards.organization.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Organización actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
