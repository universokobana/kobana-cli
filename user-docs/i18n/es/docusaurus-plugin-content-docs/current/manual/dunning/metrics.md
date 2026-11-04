---
id: metrics
title: "kobana dunning metrics"
sidebar_label: "metrics"
---

# kobana dunning metrics


Comandos de `kobana dunning metrics`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`metrics recovery`](#metrics-recovery) | `GET /v1/metrics/recovery` | Métricas de recuperación |

## metrics recovery

Métricas de recuperación

```text
kobana dunning metrics recovery [OPCIONES]
```

`GET /v1/metrics/recovery`

Devuelve las métricas de recuperación del período (por defecto: últimos 6 meses, contados desde el primer día del mes inicial).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `months` | `integer` | no | Ventana en meses (1–24; por defecto 6). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Métricas calculadas. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
