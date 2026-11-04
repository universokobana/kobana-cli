---
id: notification-preferences
title: "kobana dunning notification-preferences"
sidebar_label: "notification-preferences"
---

# kobana dunning notification-preferences


Comandos de `kobana dunning notification-preferences`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`notification-preferences list`](#notification-preferences-list) | `GET /v1/notification-preferences` | Consultar preferencias de notificación |
| [`notification-preferences update`](#notification-preferences-update) | `PUT /v1/notification-preferences` | Actualizar preferencias de notificación |

## notification-preferences list

Consultar preferencias de notificación

```text
kobana dunning notification-preferences list [OPCIONES]
```

`GET /v1/notification-preferences`

Devuelve el gate efectivo por tipo de mensaje — los tipos sin registro guardado aparecen con todos los canales habilitados.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Preferencias efectivas por tipo. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## notification-preferences update

Actualizar preferencias de notificación

```text
kobana dunning notification-preferences update [OPCIONES]
```

`PUT /v1/notification-preferences`

Hace upsert de las preferencias enviadas (parcial: solo los tipos incluidos se modifican).

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `preferences` | `array<object>` | sí |  |

Otros campos y objetos anidados: `kobana schema dunning.notification-preferences.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Preferencias guardadas. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
