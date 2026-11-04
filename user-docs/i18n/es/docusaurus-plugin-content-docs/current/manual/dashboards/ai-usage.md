---
id: ai-usage
title: "kobana dashboards ai-usage"
sidebar_label: "ai-usage"
description: "Consultar uso de IA"
---

# kobana dashboards ai-usage

Consultar uso de IA

Comandos de `kobana dashboards ai-usage`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`ai-usage`](#ai-usage) | `GET /v1/ai_usage` | Consultar uso de IA |

## ai-usage

Consultar uso de IA

```text
kobana dashboards ai-usage [OPCIONES]
```

`GET /v1/ai_usage`

Consumo de IA, solo precio (costo y margen nunca salen): totales, detalle diario y últimos turnos del workspace de la llamada, más el acumulado del mes, franquicias, límites, saldo de crédito y si el próximo turno está permitido — estos de toda la organización, pues el límite y el crédito son compartidos. Período por defecto: mes actual (America/Sao_Paulo), como máximo 366 días.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `start_date` | `string` | no | Primer día (AAAA-MM-DD). |
| `end_date` | `string` | no | Último día (AAAA-MM-DD). |
| `days` | `integer` | no | Los últimos N días, en lugar de un rango de fechas. |

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada: el `id` o el `public_id` que se muestran en Workspaces en el panel. Dentro de la organización del token, los datos viven en workspaces. Puede omitirse mientras la organización tenga un único workspace; con dos o más, omitirlo devuelve `400 workspace_required`. Un workspace que no es de la organización devuelve `403 unknown_workspace`. Con un token de persona, solo los workspaces a los que ella tiene acceso (los propietarios de la organización alcanzan todos); los demás devuelven `403 workspace_access_denied`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | El uso de IA. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
