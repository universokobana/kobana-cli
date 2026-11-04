---
id: webhook-deliveries
title: "kobana dashboards webhook-deliveries"
sidebar_label: "webhook-deliveries"
---

# kobana dashboards webhook-deliveries


Comandos de `kobana dashboards webhook-deliveries`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`webhook-deliveries list`](#webhook-deliveries-list) | `GET /v1/webhook_deliveries` | Listar entregas de webhook |
| [`webhook-deliveries get`](#webhook-deliveries-get) | `GET /v1/webhook_deliveries/{id}` | Consultar entrega de webhook |

## webhook-deliveries list

Listar entregas de webhook

```text
kobana dashboards webhook-deliveries list [OPCIONES]
```

`GET /v1/webhook_deliveries`

Entregas de todos los endpoints del workspace, de la más reciente a la más antigua, con el tipo de evento y el resumen del endpoint.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | no | `pending`, `delivered` o `failed`. Valores: `pending`, `delivered`, `failed`. |
| `webhook_id` | `string` | no | Solo entregas de este endpoint de webhook. |
| `date_from` | `string` | no | Límite inferior de `created_at` (fecha o fecha-hora ISO 8601). |
| `date_to` | `string` | no | Límite superior de `created_at` (fecha o fecha-hora ISO 8601). Una fecha sin hora cubre el día entero (UTC). |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Página de entregas. |
| `400` | Solicitud inválida: cuerpo malformado, campo obligatorio ausente o filtro con valor inválido. |
| `404` | Recurso no encontrado en la organización actual. |

## webhook-deliveries get

Consultar entrega de webhook

```text
kobana dashboards webhook-deliveries get [ID] [OPCIONES]
```

`GET /v1/webhook_deliveries/{id}`

Devuelve una entrega: la solicitud (redactada), la respuesta, los intentos y el endpoint de destino.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | La entrega. |
| `404` | Recurso no encontrado en la organización actual. |
