---
id: rate-limits
title: "kobana dashboards rate-limits"
sidebar_label: "rate-limits"
description: "Consultar límites de solicitudes"
---

# kobana dashboards rate-limits

Consultar límites de solicitudes

Comandos de `kobana dashboards rate-limits`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`rate-limits`](#rate-limits) | `GET /v1/rate_limits` | Consultar límites de solicitudes |

## rate-limits

Consultar límites de solicitudes

```text
kobana dashboards rate-limits [OPCIONES]
```

`GET /v1/rate_limits`

Los límites de solicitudes vigentes para la organización, si son personalizados (definidos por el equipo Kobana) y los valores por defecto de la plataforma. Solo lectura.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Los límites. |
| `404` | Recurso no encontrado en la organización actual. |
