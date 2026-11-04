---
id: subscription-changes
title: "kobana billing subscription-changes"
sidebar_label: "subscription-changes"
---

# kobana billing subscription-changes


Comandos de `kobana billing subscription-changes`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`subscription-changes config list`](#subscription-changes-config-list) | `GET /v1/subscription_changes/config` | Obtener configuración de cambios de suscripción |
| [`subscription-changes config update`](#subscription-changes-config-update) | `PUT /v1/subscription_changes/config` | Actualizar configuración de cambios de suscripción |

## subscription-changes config list

Obtener configuración de cambios de suscripción

```text
kobana billing subscription-changes config list [OPCIONES]
```

`GET /v1/subscription_changes/config`

Devuelve la configuración de la organización para cambios en ítems de suscripción — operaciones permitidas (agregar/eliminar/cambiar cantidad/cambiar precio), método de prorrateo por operación, momento de aplicación (`immediate` o `end_of_period`), reembolso/crédito al eliminar y self-service en el portal. Si aún no existe, se crea con los valores por defecto en la primera lectura.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Configuración de la organización. |
| `404` | Recurso no encontrado en la organización actual. |

## subscription-changes config update

Actualizar configuración de cambios de suscripción

```text
kobana billing subscription-changes config update [OPCIONES]
```

`PUT /v1/subscription_changes/config`

Actualiza parcialmente la configuración. Solo se modifican los campos enviados. `refund_on_remove` requiere que `credit_on_remove` también esté habilitado — el reembolso es una variante del crédito.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `allow_item_add` | `boolean` | no | Permite agregar nuevos ítems a una suscripción existente. |
| `allow_item_remove` | `boolean` | no | Permite eliminar ítems de una suscripción existente. |
| `allow_quantity_change` | `boolean` | no | Permite cambiar la cantidad de ítems ya presentes en la suscripción. |
| `allow_price_swap` | `boolean` | no | Permite cambiar el precio (variante) de un ítem existente sin eliminarlo. |
| `add_proration` | `string` | no | Método de prorrateo al agregar ítems: `full_proration`, `no_proration` o `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `remove_proration` | `string` | no | Método de prorrateo al eliminar ítems: `full_proration`, `no_proration` o `partial_proration`. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `quantity_up_proration` | `string` | no | Método de prorrateo al aumentar la cantidad de un ítem. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `quantity_down_proration` | `string` | no | Método de prorrateo al reducir la cantidad de un ítem. Valores: `full_proration`, `no_proration`, `partial_proration`. |
| `add_timing` | `string` | no | Momento de aplicación al agregar ítems: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `remove_timing` | `string` | no | Momento de aplicación al eliminar ítems: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `quantity_up_timing` | `string` | no | Momento de aplicación al aumentar cantidad: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `quantity_down_timing` | `string` | no | Momento de aplicación al reducir cantidad: `immediate` o `end_of_period`. Valores: `immediate`, `end_of_period`. |
| `refund_on_remove` | `boolean` | no | Cuando es `true`, emite un reembolso al cliente al eliminar ítems. Requiere `credit_on_remove` habilitado. |
| `credit_on_remove` | `boolean` | no | Cuando es `true`, emite un crédito a favor del cliente al eliminar ítems. |
| `portal_self_service` | `boolean` | no | Cuando es `true`, permite que el cliente cambie su propia suscripción desde el portal. |
| `portal_require_confirmation` | `boolean` | no | Cuando es `true`, exige confirmación explícita del cliente antes de aplicar el cambio en el portal. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.subscription-changes.config.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Configuración actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
