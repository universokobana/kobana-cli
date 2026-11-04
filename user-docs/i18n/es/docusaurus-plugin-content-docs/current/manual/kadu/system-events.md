---
id: system-events
title: "kobana kadu system-events"
sidebar_label: "system-events"
description: "Eventos de sistema (los 100 más recientes)"
---

# kobana kadu system-events

Eventos de sistema (los 100 más recientes)

Comandos de `kobana kadu system-events`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`system-events`](#system-events) | `GET /v1/system_events` | Eventos de sistema (los 100 más recientes) |

## system-events

Eventos de sistema (los 100 más recientes)

```text
kobana kadu system-events [OPCIONES]
```

`GET /v1/system_events`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `slug` | `string` | no |  |
| `workspace_id` | `string` | no |  |
| `task_id` | `string` | no |  |
| `execution_id` | `string` | no |  |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR — filtro inválido |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
