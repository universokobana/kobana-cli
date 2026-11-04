---
id: usage
title: "kobana dashboards usage"
sidebar_label: "usage"
description: "Consultar consumo de la plataforma"
---

# kobana dashboards usage

Consultar consumo de la plataforma

Comandos de `kobana dashboards usage`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`usage`](#usage) | `GET /v1/usage` | Consultar consumo de la plataforma |

## usage

Consultar consumo de la plataforma

```text
kobana dashboards usage [OPCIONES]
```

`GET /v1/usage`

Para cada métrica (miembros, clientes, usuarios del portal, integraciones…), la serie diaria en el período, el valor más reciente y el pico. Por defecto: mes actual; como máximo 12 meses atrás. Recurso de la organización.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `start_date` | `string` | no | Primer día (AAAA-MM-DD). Por defecto: día 1 del mes actual. |
| `end_date` | `string` | no | Último día (AAAA-MM-DD). Por defecto: hoy. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El consumo. |
| `404` | Recurso no encontrado en la organización actual. |
