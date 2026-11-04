---
id: customers
title: "kobana billing customers"
sidebar_label: "customers"
---

# kobana billing customers


Comandos de `kobana billing customers`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`customers statement sync`](#customers-statement-sync) | `POST /v1/customers/{id}/statement/sync` | Reconstruir el extracto a partir del historial |
| [`customers statement list`](#customers-statement-list) | `GET /v1/customers/{id}/statement` | Listar extracto financiero del cliente |
| [`customers users list`](#customers-users-list) | `GET /v1/customers/{id}/users` | Listar usuarios del portal del cliente |
| [`customers users create`](#customers-users-create) | `POST /v1/customers/{id}/users` | Invitar usuario del portal |
| [`customers users invite`](#customers-users-invite) | `POST /v1/customers/{id}/users/invite` | Invitar usuario del portal (verbo) |
| [`customers users get`](#customers-users-get) | `GET /v1/customers/{id}/users/{user_id}` | Obtener usuario del portal |
| [`customers users update`](#customers-users-update) | `PATCH /v1/customers/{id}/users/{user_id}` | Actualizar usuario del portal |
| [`customers users delete`](#customers-users-delete) | `DELETE /v1/customers/{id}/users/{user_id}` | Eliminar usuario del portal |
| [`customers users email-preview`](#customers-users-email-preview) | `GET /v1/customers/{id}/users/{user_id}/email-preview` | Previsualizar correo del usuario del portal |
| [`customers users resend`](#customers-users-resend) | `POST /v1/customers/{id}/users/{user_id}/resend` | Reenviar correo de invitación o verificación |
| [`customers list`](#customers-list) | `GET /v1/customers` | Listar clientes |
| [`customers create`](#customers-create) | `POST /v1/customers` | Crear cliente |
| [`customers update`](#customers-update) | `PUT /v1/customers` | Crear o actualizar cliente |
| [`customers get`](#customers-get) | `GET /v1/customers/{id}` | Obtener cliente |
| [`customers delete`](#customers-delete) | `DELETE /v1/customers/{id}` | Eliminar cliente (soft delete) |
| [`customers recalculate-mrr`](#customers-recalculate-mrr) | `POST /v1/customers/{id}/recalculate-mrr` | Recalcular el MRR del cliente |

## customers statement sync

Reconstruir el extracto a partir del historial

```text
kobana billing customers statement sync [ID] [OPCIONES]
```

`POST /v1/customers/{id}/statement/sync`

Recrea los ítems del extracto a partir de las facturas, pagos y aplicaciones de crédito existentes. Devuelve el resumen de lo creado.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | Necesario cuando el cliente tiene múltiples cuentas de cobro. |

Otros campos y objetos anidados: `kobana schema billing.customers.statement.sync`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Extracto reconstruido. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## customers statement list

Listar extracto financiero del cliente

```text
kobana billing customers statement list [ID] [OPCIONES]
```

`GET /v1/customers/{id}/statement`

Devuelve los movimientos del extracto (facturas, pagos, aplicaciones de crédito) de la cuenta de cobro del cliente. Cuando el cliente tiene múltiples cuentas, informa `billing_account_id`.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Necesario cuando el cliente tiene múltiples cuentas de cobro. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de ítems del extracto. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## customers users list

Listar usuarios del portal del cliente

```text
kobana billing customers users list [ID] [OPCIONES]
```

`GET /v1/customers/{id}/users`

Devuelve los usuarios con acceso al portal vinculados a la cuenta de cobro del cliente.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `status` | `string` | no | Filtra usuarios del portal por estado. Valores: `active`, `pending_verification`, `pending_invitation`, `suspended`, `locked`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de usuarios del portal. |
| `404` | Recurso no encontrado en la organización actual. |

## customers users create

Invitar usuario del portal

```text
kobana billing customers users create [ID] [OPCIONES]
```

`POST /v1/customers/{id}/users`

Crea una invitación de acceso al portal para el correo informado. La invitación expira en 7 días.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string` | sí | Correo del invitado (obligatorio). |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.customers.users.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Invitación creada y correo encolado para envío. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | Ya existe un usuario o invitación para este correo. |

## customers users invite

Invitar usuario del portal (verbo)

```text
kobana billing customers users invite [ID] [OPCIONES]
```

`POST /v1/customers/{id}/users/invite`

Equivalente a `POST /customers/{id}/users`. Disponible como ruta de verbo para SDKs que prefieren URLs descriptivas.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string` | sí | Correo del invitado (obligatorio). |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.customers.users.invite`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Invitación creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## customers users get

Obtener usuario del portal

```text
kobana billing customers users get [ID] [USER_ID] [OPCIONES]
```

`GET /v1/customers/{id}/users/{user_id}`

El parámetro `{user_id}` acepta el UUID o el `external_id` del usuario.

**Argumentos de la ruta:** `id`, `user_id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del usuario del portal. |
| `404` | Recurso no encontrado en la organización actual. |

## customers users update

Actualizar usuario del portal

```text
kobana billing customers users update [ID] [USER_ID] [OPCIONES]
```

`PATCH /v1/customers/{id}/users/{user_id}`

Actualiza `email`, `external_id`, `sso_only` y `status`. Transiciones de estado permitidas: `active` ↔ `suspended`.

**Argumentos de la ruta:** `id`, `user_id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `email` | `string` | no |  |
| `external_id` | `string` | no |  |
| `sso_only` | `boolean` | no |  |
| `status` | `string` | no | Transiciones permitidas: `active` ↔ `suspended`. Valores: `active`, `suspended`. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). |

Otros campos y objetos anidados: `kobana schema billing.customers.users.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Usuario actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## customers users delete

Eliminar usuario del portal

```text
kobana billing customers users delete [ID] [USER_ID] [OPCIONES]
```

`DELETE /v1/customers/{id}/users/{user_id}`

Elimina el usuario del portal (soft delete: `deleted_at`) — una invitación pendiente y un usuario activo por igual: pierde el acceso al instante, las sesiones y los enlaces pendientes dejan de valer, y el correo puede invitarse de nuevo. Exige el nivel de acceso `delete`. Hasta octubre de 2026 esta ruta borraba la invitación pendiente y solo suspendía al usuario activo.

**Argumentos de la ruta:** `id`, `user_id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Usuario del portal eliminado. |
| `404` | Recurso no encontrado en la organización actual. |

## customers users email-preview

Previsualizar correo del usuario del portal

```text
kobana billing customers users email-preview [ID] [USER_ID] [OPCIONES]
```

`GET /v1/customers/{id}/users/{user_id}/email-preview`

Devuelve el asunto y el mensaje del correo de invitación o verificación que se enviaría al usuario, sin disparar el envío. Disponible solo para usuarios `pending_invitation` o `pending_verification`.

**Argumentos de la ruta:** `id`, `user_id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Previsualización del correo. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## customers users resend

Reenviar correo de invitación o verificación

```text
kobana billing customers users resend [ID] [USER_ID] [OPCIONES]
```

`POST /v1/customers/{id}/users/{user_id}/resend`

Reenvía el correo de invitación (para `pending_invitation`, con rotación de token) o de verificación (para `pending_verification`). Otros estados devuelven 400.

**Argumentos de la ruta:** `id`, `user_id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Correo reenviado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## customers list

Listar clientes

```text
kobana billing customers list [OPCIONES]
```

`GET /v1/customers`

Devuelve una lista paginada de los clientes de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda textual libre por nombre, documento o correo. |
| `kind` | `string` | no | Filtra por tipo de persona: `natural` (PF) o `juridical` (PJ). Valores: `natural`, `juridical`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de clientes. |
| `404` | Recurso no encontrado en la organización actual. |

## customers create

Crear cliente

```text
kobana billing customers create [OPCIONES]
```

`POST /v1/customers`

Crea un cliente y, por defecto, una cuenta de cobro asociada. Usa `create_billing_account=false` para crear solo el cliente.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del cliente (obligatorio). |
| `public_id` | `string (uuid)` | no | UUID público del cliente. Se genera cuando se omite; envíelo solo para conservar un identificador de su sistema. Debe ser único. |
| `external_id` | `string` | no | Identificador externo del cliente en el sistema de origen. |
| `nickname` | `string` | no |  |
| `legal_name` | `string` | no | Razón social (para persona jurídica). |
| `document_number` | `string` | no | CPF o CNPJ. El `document_type` se infiere por la longitud (≤11 → cpf, si no cnpj). |
| `kind` | `string` | no | Valores: `natural`, `juridical`. |
| `birthday` | `string` | no | Fecha de nacimiento (ISO 8601). |
| `email` | `string` | no | Correo principal de contacto. |
| `phone` | `string` | no |  |
| `notes` | `string` | no |  |
| `addresses` | `array<object>` | no |  |
| `phones` | `array<object>` | no |  |
| `emails` | `array<object>` | no |  |
| `websites` | `array<object>` | no |  |
| `tags` | `array<string>` | no |  |
| `custom_metadata` | `object` | no |  |
| `create_billing_account` | `boolean` | no | Cuando es `false`, no crea cuenta de cobro automáticamente. Por defecto: `true`. |
| `billing_email` | `string` | no | Correo para envío de facturas. Por defecto: usa `email` cuando se omite. |
| `payment_terms_days` | `integer` | no | Plazo de vencimiento de factura en días. Por defecto: el plazo predeterminado configurado en la organización (10 cuando no está configurado). |
| `auto_collection` | `boolean` | no | Cobro automático en la cuenta creada. Por defecto: `true`. |

Otros campos y objetos anidados: `kobana schema billing.customers.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cliente creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## customers update

Crear o actualizar cliente

```text
kobana billing customers update [OPCIONES]
```

`PUT /v1/customers`

Recibe el mismo cuerpo del POST y deduplica por `id`, `public_id`, `external_id` y `document_number`, en ese orden, deteniéndose en la primera clave que encuentre un cliente. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. La actualización es parcial: los campos ausentes conservan su valor.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del cliente. Obligatorio solo cuando ningún cliente es encontrado por las claves de deduplicación. |
| `public_id` | `string (uuid)` | no | UUID público del cliente. Localiza el registro y, cuando no se encuentra ninguno, se guarda en el cliente creado. |
| `external_id` | `string` | no | Identificador externo del cliente en el sistema de origen. |
| `nickname` | `string` | no |  |
| `legal_name` | `string` | no | Razón social (para persona jurídica). |
| `document_number` | `string` | no | CPF o CNPJ. El `document_type` se infiere por la longitud (≤11 → cpf, si no cnpj). |
| `kind` | `string` | no | Valores: `natural`, `juridical`. |
| `birthday` | `string` | no | Fecha de nacimiento (ISO 8601). |
| `email` | `string` | no | Correo principal de contacto. |
| `phone` | `string` | no |  |
| `notes` | `string` | no |  |
| `addresses` | `array<object>` | no |  |
| `phones` | `array<object>` | no |  |
| `emails` | `array<object>` | no |  |
| `websites` | `array<object>` | no |  |
| `tags` | `array<string>` | no |  |
| `custom_metadata` | `object` | no |  |
| `create_billing_account` | `boolean` | no | Cuando es `false`, no crea cuenta de cobro automáticamente. Por defecto: `true`. |
| `billing_email` | `string` | no | Correo para envío de facturas. Por defecto: usa `email` cuando se omite. |
| `payment_terms_days` | `integer` | no | Plazo de vencimiento de factura en días. Por defecto: el plazo predeterminado configurado en la organización (10 cuando no está configurado). |
| `auto_collection` | `boolean` | no | Cobro automático en la cuenta creada. Por defecto: `true`. |
| `id` | `string (uuid)` | no | UUID del cliente. Se usa solo para localizar el registro — nunca se guarda. |
| `custom_metadata_mode` | `string` | no | Cómo se reconcilia el `custom_metadata` enviado con el valor almacenado. `replace` (por defecto) reemplaza todo el objeto; `merge` hace un deep merge sobre el valor actual — los objetos anidados se combinan recursivamente, los escalares y arrays reemplazan la clave, y las claves `null` se mantienen (no eliminan la clave). Aceptado solo en la actualización. Valores: `replace`, `merge`. |

Otros campos y objetos anidados: `kobana schema billing.customers.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cliente actualizado. |
| `201` | Cliente creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |

## customers get

Obtener cliente

```text
kobana billing customers get [ID] [OPCIONES]
```

`GET /v1/customers/{id}`

El parámetro `{id}` acepta el UUID del cliente o el `external_id`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del cliente. |
| `404` | Recurso no encontrado en la organización actual. |

## customers delete

Eliminar cliente (soft delete)

```text
kobana billing customers delete [ID] [OPCIONES]
```

`DELETE /v1/customers/{id}`

Marca el cliente como eliminado sin borrar registros históricos.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Confirmación de soft delete. |
| `404` | Recurso no encontrado en la organización actual. |

## customers recalculate-mrr

Recalcular el MRR del cliente

```text
kobana billing customers recalculate-mrr [ID] [OPCIONES]
```

`POST /v1/customers/{id}/recalculate-mrr`

Recalcula el MRR (Monthly Recurring Revenue) de las cuentas de cobro del cliente a partir de las suscripciones activas.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | MRR recalculado. |
| `404` | Recurso no encontrado en la organización actual. |
