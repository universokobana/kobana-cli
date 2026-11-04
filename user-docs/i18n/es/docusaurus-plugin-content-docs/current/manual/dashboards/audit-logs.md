---
id: audit-logs
title: "kobana dashboards audit-logs"
sidebar_label: "audit-logs"
---

# kobana dashboards audit-logs


Comandos de `kobana dashboards audit-logs`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`audit-logs summary`](#audit-logs-summary) | `GET /v1/audit_logs/summary` | Resumen de la actividad auditada |
| [`audit-logs list`](#audit-logs-list) | `GET /v1/audit_logs` | Listar registros de auditoría |
| [`audit-logs get`](#audit-logs-get) | `GET /v1/audit_logs/{id}` | Consultar registro de auditoría |

## audit-logs summary

Resumen de la actividad auditada

```text
kobana dashboards audit-logs summary [OPCIONES]
```

`GET /v1/audit_logs/summary`

Actividad de las últimas `hours` horas: total de registros, las 10 acciones más frecuentes y los 20 registros más recientes. Requiere el permiso de exportar auditoría.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `hours` | `number` | no | Tamaño de la ventana en horas, de 1 a 8760. Por defecto 24. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resumen de la actividad. |
| `400` | Solicitud inválida: cuerpo malformado, campo obligatorio ausente o filtro con valor inválido. |
| `404` | Recurso no encontrado en la organización actual. |

## audit-logs list

Listar registros de auditoría

```text
kobana dashboards audit-logs list [OPCIONES]
```

`GET /v1/audit_logs`

Auditoría del workspace y de la organización, del más reciente al más antiguo. La auditoría no se puede cambiar.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `user_id` | `string` | no | Identificador exacto del usuario. |
| `action` | `string` | no | Acción exacta (ej.: `webhook.created`). |
| `resource_type` | `string` | no | Tipo exacto del recurso (ej.: `webhook`). |
| `resource_id` | `string` | no | ID exacto del recurso. |
| `search` | `string` | no | Fragmento de la acción, del tipo de recurso, del ID o del correo del usuario, sin distinguir mayúsculas. |
| `date_from` | `string` | no | Límite inferior de `created_at` (fecha o fecha-hora ISO 8601). |
| `date_to` | `string` | no | Límite superior de `created_at` (fecha o fecha-hora ISO 8601). Una fecha sin hora cubre el día entero (UTC). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de registros de auditoría. |
| `400` | Solicitud inválida: cuerpo malformado, campo obligatorio ausente o filtro con valor inválido. |
| `404` | Recurso no encontrado en la organización actual. |

## audit-logs get

Consultar registro de auditoría

```text
kobana dashboards audit-logs get [ID] [OPCIONES]
```

`GET /v1/audit_logs/{id}`

Devuelve un registro de auditoría con los valores anteriores y nuevos.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El registro de auditoría. |
| `404` | Recurso no encontrado en la organización actual. |
