---
id: negotiation-campaigns
title: "kobana dunning negotiation-campaigns"
sidebar_label: "negotiation-campaigns"
---

# kobana dunning negotiation-campaigns


Comandos de `kobana dunning negotiation-campaigns`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`negotiation-campaigns list`](#negotiation-campaigns-list) | `GET /v1/negotiation-campaigns` | Listar campañas |
| [`negotiation-campaigns create`](#negotiation-campaigns-create) | `POST /v1/negotiation-campaigns` | Crear campaña |
| [`negotiation-campaigns end`](#negotiation-campaigns-end) | `POST /v1/negotiation-campaigns/{id}/end` | Finalizar campaña |

## negotiation-campaigns list

Listar campañas

```text
kobana dunning negotiation-campaigns list [OPCIONES]
```

`GET /v1/negotiation-campaigns`

Lista las campañas de negociación de la organización (activas y finalizadas), de la más reciente a la más antigua. Sin paginación.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Campañas de la organización (envelope `campaigns`). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## negotiation-campaigns create

Crear campaña

```text
kobana dunning negotiation-campaigns create [OPCIONES]
```

`POST /v1/negotiation-campaigns`

Crea y activa una campaña de descuento. El portal del deudor comienza a ofrecerla a los cobros dentro del rango de atraso configurado.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre de la campaña. |
| `discount_percentage` | `number` | sí | Porcentaje de descuento (0-100). Devuelto como string decimal. |
| `max_installments` | `integer` | no | Máximo de cuotas permitido (1 = solo pago único). |
| `min_days_overdue` | `integer` | no | Inicio del rango de días de atraso en que la campaña aplica (null = sin mínimo). |
| `max_days_overdue` | `integer` | no | Fin del rango de días de atraso en que la campaña aplica (null = sin máximo). |
| `valid_until` | `string (date)` | no | Fecha límite de validez de la campaña (null = sin plazo). |
| `terms` | `string` | no | Condiciones mostradas al deudor. |
| `custom_metadata` | `object` | no | Metadatos del usuario (objeto JSON libre, lectura y escritura). Ver `custom_data` para el campo legado de los recursos núcleo. |

Otros campos y objetos anidados: `kobana schema dunning.negotiation-campaigns.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Campaña creada y activada (envelope `campaign`). |
| `400` | Solicitud inválida (error de validación). |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |

## negotiation-campaigns end

Finalizar campaña

```text
kobana dunning negotiation-campaigns end [ID] [OPCIONES]
```

`POST /v1/negotiation-campaigns/{id}/end`

Finaliza la campaña (desactiva; `activated_at` pasa a null). El registro se mantiene y el portal deja de ofrecerla. Es una escritura (`write`), no una eliminación — sustituyó a `DELETE /negotiation-campaigns/{id}`.

**Argumentos de la ruta:** `id`

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `X-Workspace-Id` | `string` | no | Workspace en el que actúa la llamada (`id` o `public_id`, listados en `GET /v1/workspaces`). Opcional mientras la organización tenga un solo workspace; con dos o más, sin él la respuesta es `400 WORKSPACE_REQUIRED`. Un workspace de otra organización es `403 UNKNOWN_WORKSPACE`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Campaña finalizada. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
