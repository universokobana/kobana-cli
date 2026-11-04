---
id: usage
title: "kobana kadu usage"
sidebar_label: "usage"
description: "Uso y costo de las ejecuciones, las conversaciones y la consolidación de memorias"
---

# kobana kadu usage

Uso y costo de las ejecuciones, las conversaciones y la consolidación de memorias

Comandos de `kobana kadu usage`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`usage`](#usage) | `GET /v1/usage` | Uso y costo de las ejecuciones, las conversaciones y la consolidación de memorias |
| [`usage performance`](#usage-performance) | `GET /v1/usage/performance` | Rendimiento: duración, éxito y error, memoria, archivos y volúmenes |

## usage

Uso y costo de las ejecuciones, las conversaciones y la consolidación de memorias

```text
kobana kadu usage [OPCIONES] [COMANDO]
```

`GET /v1/usage`

Agregado por día (en la zona horaria de la organización) × grupo, sumando ejecuciones, turnos de conversación del bot de Slack (origen `slack_chat`, sin workspace/actividad/Task) y consolidación de memorias (origen `memory_consolidation`, en el workspace/actividad/Task de la ejecución de origen). `by_kind` separa los tres: lo que es conversación y lo que no. Por defecto: últimos 30 días; período máximo de 366 días. `group` es el id (tier, workspace, activity, task) o el origen; null = sin valor.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `from` | `string` | no |  |
| `to` | `string` | no |  |
| `group_by` | `string` | no | Valores: `tier`, `workspace`, `activity`, `task`, `source`. |
| `workspace_id` | `string` | no |  |
| `activity_id` | `string` | no |  |
| `task_id` | `string` | no |  |
| `source` | `string` | no |  |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## usage performance

Rendimiento: duración, éxito y error, memoria, archivos y volúmenes

```text
kobana kadu usage performance [OPCIONES]
```

`GET /v1/usage/performance`

Serie diaria (en la zona horaria de la organización) con la duración media de las ejecuciones (total, bootstrap y ejecución efectiva), las ejecuciones terminadas por resultado, las memorias existentes al final del día (cantidad y bytes de contenido + resumen), el tamaño acumulado de los archivos de entrada y salida, filtrable por la extensión (`file_type`), y el de los volúmenes (`volume_id`). Predeterminado: últimos 30 días; período máximo de 366 días. Las memorias no tienen tarea ni origen: solo workspace y actividad filtran la memoria. Requiere Analytics › Rendimiento habilitado.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `from` | `string` | no |  |
| `to` | `string` | no |  |
| `workspace_id` | `string` | no |  |
| `activity_id` | `string` | no |  |
| `task_id` | `string` | no |  |
| `source` | `string` | no |  |
| `file_type` | `string` | no | Extensión de los archivos en el gráfico de archivos (ej.: `pdf`); `__none__` = sin extensión. Omitido = todas. |
| `volume_id` | `string` | no | Solo este volumen en la serie de volúmenes. Un id fuera de los volúmenes de la organización = todos. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
