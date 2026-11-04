---
id: audit-logs
title: "kobana dunning audit-logs"
sidebar_label: "audit-logs"
description: "Consultar pista de auditoría"
---

# kobana dunning audit-logs

Consultar pista de auditoría

Comandos de `kobana dunning audit-logs`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`audit-logs`](#audit-logs) | `GET /v1/audit-logs` | Consultar pista de auditoría |

## audit-logs

Consultar pista de auditoría

```text
kobana dunning audit-logs [OPCIONES]
```

`GET /v1/audit-logs`

Lista los registros de la pista de auditoría de la organización (más recientes primero), con filtros por acción, entidad, actor y período.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página (desde 1). |
| `per_page` | `integer` | no | Elementos por página (máx. 100). |
| `action` | `string` | no | Filtra por acción (búsqueda parcial, `contains`). |
| `entity_type` | `string` | no | Filtra por tipo de entidad. |
| `entity_id` | `string` | no | Filtra por entidad (UUID). |
| `actor_type` | `string` | no | Filtra por tipo de actor (`user` \| `system` \| `job`). Valores: `user`, `system`, `job`. |
| `actor_id` | `string` | no | Filtra por actor (ID del usuario, nombre del job o del sistema). |
| `from` | `string` | no | Inicio del período (ISO 8601, inclusive). |
| `to` | `string` | no | Fin del período (ISO 8601, inclusive). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de registros de auditoría. |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
