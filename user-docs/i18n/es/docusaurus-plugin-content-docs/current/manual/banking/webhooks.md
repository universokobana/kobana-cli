---
id: webhooks
title: "kobana banking webhooks"
sidebar_label: "webhooks"
---

# kobana banking webhooks


Comandos de `kobana banking webhooks`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`webhooks list`](#webhooks-list) | `GET /v1/webhooks` | Listar Webhooks |
| [`webhooks create`](#webhooks-create) | `POST /v1/webhooks` | Crear Webhook |
| [`webhooks get`](#webhooks-get) | `GET /v1/webhooks/{id}` | Información del Webhook |
| [`webhooks update`](#webhooks-update) | `PUT /v1/webhooks/{id}` | Actualizar Webhook |
| [`webhooks delete`](#webhooks-delete) | `DELETE /v1/webhooks/{id}` | Eliminar Webhook |

## webhooks list

Listar Webhooks

```text
kobana banking webhooks list [OPCIONES]
```

`GET /v1/webhooks`

Listar webhooks.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de webhooks |

## webhooks create

Crear Webhook

```text
kobana banking webhooks create [OPCIONES]
```

`POST /v1/webhooks`

Crea un webhook para recibir notificaciones de eventos del sistema vía HTTP.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `url` | `string` | sí | Dirección donde se realizarán las solicitudes |
| `events` | `array<string>` | sí | Eventos que deben generar notificaciones para el webhook. Elija uno o más eventos. |
| `id` | `integer` | no | ID del webhook |
| `name` | `string` | no | Nombre de identificación del webhook |
| `content_type` | `string` | no | Content-type de la solicitud que se realizará. Valores válidos `application/json` (valor por defecto) o `application/x-www-form-urlencoded`. |
| `active` | `boolean` | no | Define si el webhook estará activo, recibiendo notificaciones. El valor por defecto es `true`. |
| `ssl_verification_enabled` | `boolean` | no | Define si la verificación SSL está habilitada en el webhook. |
| `subaccounts` | `boolean` | no | Define si el webhook estará activo para los eventos de las subcuentas. |
| `bank_billet_account_id` | `integer` | no | ID de la Cartera de Cobro. Si se proporciona, el webhook solo recibirá notificaciones de eventos relacionados con la cartera indicada. |
| `auth_kind` | `string` | no | Tipo de autenticación configurado. Devuelto en las respuestas. Para configurar, envíe el objeto `auth` en el body. Valores: `none`, `basic_auth`, `bearer`, `api_key`. |
| `auth` | `object` | no | Configuración de autenticación del endpoint receptor. Solo para creación/actualización — no devuelto en las respuestas. |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.webhooks.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Webhook creado con Basic Auth |
| `422` | Webhook inválido |

## webhooks get

Información del Webhook

```text
kobana banking webhooks get [ID] [OPCIONES]
```

`GET /v1/webhooks/{id}`

Consulta información específica de un Webhook.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Webhook encontrado |
| `404` | Webhook no encontrado |

## webhooks update

Actualizar Webhook

```text
kobana banking webhooks update [ID] [OPCIONES]
```

`PUT /v1/webhooks/{id}`

Actualiza información específica de un webhook.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `url` | `string` | sí | Dirección donde se realizarán las solicitudes |
| `events` | `array<string>` | sí | Eventos que deben generar notificaciones para el webhook. Elija uno o más eventos. |
| `id` | `integer` | no | ID del webhook |
| `name` | `string` | no | Nombre de identificación del webhook |
| `content_type` | `string` | no | Content-type de la solicitud que se realizará. Valores válidos `application/json` (valor por defecto) o `application/x-www-form-urlencoded`. |
| `active` | `boolean` | no | Define si el webhook estará activo, recibiendo notificaciones. El valor por defecto es `true`. |
| `ssl_verification_enabled` | `boolean` | no | Define si la verificación SSL está habilitada en el webhook. |
| `subaccounts` | `boolean` | no | Define si el webhook estará activo para los eventos de las subcuentas. |
| `bank_billet_account_id` | `integer` | no | ID de la Cartera de Cobro. Si se proporciona, el webhook solo recibirá notificaciones de eventos relacionados con la cartera indicada. |
| `auth_kind` | `string` | no | Tipo de autenticación configurado. Devuelto en las respuestas. Para configurar, envíe el objeto `auth` en el body. Valores: `none`, `basic_auth`, `bearer`, `api_key`. |
| `auth` | `object` | no | Configuración de autenticación del endpoint receptor. Solo para creación/actualización — no devuelto en las respuestas. |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.webhooks.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Actualización parcial de auth preserva campos secretos existentes |
| `404` | Webhook no encontrado |

## webhooks delete

Eliminar Webhook

```text
kobana banking webhooks delete [ID] [OPCIONES]
```

`DELETE /v1/webhooks/{id}`

Elimina un webhook específico

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Webhook eliminado |
| `404` | Webhook no encontrado |
