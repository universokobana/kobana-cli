---
id: proposals
title: "kobana billing proposals"
sidebar_label: "proposals"
---

# kobana billing proposals


Comandos de `kobana billing proposals`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`proposals bulk delete`](#proposals-bulk-delete) | `DELETE /v1/proposals/bulk` | Eliminar propuestas en lote |
| [`proposals items list`](#proposals-items-list) | `GET /v1/proposals/{id}/items` | Listar ítems de la propuesta |
| [`proposals items create`](#proposals-items-create) | `POST /v1/proposals/{id}/items` | Agregar ítem a la propuesta |
| [`proposals items get`](#proposals-items-get) | `GET /v1/proposals/{id}/items/{item_id}` | Obtener ítem de la propuesta |
| [`proposals items update`](#proposals-items-update) | `PATCH /v1/proposals/{id}/items/{item_id}` | Actualizar ítem de la propuesta |
| [`proposals items delete`](#proposals-items-delete) | `DELETE /v1/proposals/{id}/items/{item_id}` | Eliminar ítem de la propuesta |
| [`proposals stats`](#proposals-stats) | `GET /v1/proposals/stats` | Obtener estadísticas de propuestas |
| [`proposals list`](#proposals-list) | `GET /v1/proposals` | Listar propuestas |
| [`proposals create`](#proposals-create) | `POST /v1/proposals` | Crear propuesta |
| [`proposals update`](#proposals-update) | `PUT /v1/proposals` | Crear o actualizar propuesta |
| [`proposals bulk-cancel`](#proposals-bulk-cancel) | `POST /v1/proposals/bulk-cancel` | Cancelar propuestas en lote |
| [`proposals bulk-send`](#proposals-bulk-send) | `POST /v1/proposals/bulk-send` | Enviar propuestas en lote |
| [`proposals get`](#proposals-get) | `GET /v1/proposals/{id}` | Obtener propuesta |
| [`proposals delete`](#proposals-delete) | `DELETE /v1/proposals/{id}` | Eliminar propuesta |
| [`proposals accept`](#proposals-accept) | `POST /v1/proposals/{id}/accept` | Aceptar propuesta |
| [`proposals cancel`](#proposals-cancel) | `POST /v1/proposals/{id}/cancel` | Cancelar propuesta |
| [`proposals duplicate`](#proposals-duplicate) | `POST /v1/proposals/{id}/duplicate` | Duplicar propuesta |
| [`proposals generate-pdf`](#proposals-generate-pdf) | `POST /v1/proposals/{id}/generate-pdf` | Generar PDF de la propuesta |
| [`proposals json`](#proposals-json) | `GET /v1/proposals/{id}/json` | Obtener snapshot JSON de la propuesta |
| [`proposals pricing`](#proposals-pricing) | `GET /v1/proposals/{id}/pricing` | Obtener cálculo de precio de la propuesta |
| [`proposals publish`](#proposals-publish) | `POST /v1/proposals/{id}/publish` | Publicar propuesta |
| [`proposals send`](#proposals-send) | `POST /v1/proposals/{id}/send` | Enviar propuesta por correo |
| [`proposals undo-accept`](#proposals-undo-accept) | `POST /v1/proposals/{id}/undo-accept` | Deshacer aceptación de la propuesta |
| [`proposals unpublish`](#proposals-unpublish) | `POST /v1/proposals/{id}/unpublish` | Despublicar propuesta |

## proposals bulk delete

Eliminar propuestas en lote

```text
kobana billing proposals bulk delete [OPCIONES]
```

`DELETE /v1/proposals/bulk`

Elimina varias propuestas a la vez (hasta 100). Devuelve conteos de éxito/fallo y errores por id. Exige el nivel de acceso `delete`. Reemplaza `POST /v1/proposals/bulk-delete`, eliminado.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | Lista de UUIDs de las propuestas objetivo. Mínimo 1, máximo 100. |

Otros campos y objetos anidados: `kobana schema billing.proposals.bulk.delete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la eliminación en lote. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals items list

Listar ítems de la propuesta

```text
kobana billing proposals items list [ID] [OPCIONES]
```

`GET /v1/proposals/{id}/items`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de ítems. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals items create

Agregar ítem a la propuesta

```text
kobana billing proposals items create [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/items`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `product_id` | `string (uuid)` | sí | UUID del producto. **Obligatorio**. |
| `price_id` | `string (uuid)` | sí | UUID del precio. **Obligatorio**. |
| `unit_amount` | `number` | sí | Valor unitario en BRL (float, hasta 4 decimales). |
| `quantity` | `number` | no | Cantidad. Por defecto: 1. |
| `metadata` | `object` | no | Metadatos arbitrarios del ítem. |
| `custom_metadata` | `object` | no | Campos personalizados del ítem. |

Otros campos y objetos anidados: `kobana schema billing.proposals.items.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Ítem creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals items get

Obtener ítem de la propuesta

```text
kobana billing proposals items get [ID] [ITEM_ID] [OPCIONES]
```

`GET /v1/proposals/{id}/items/{item_id}`

**Argumentos de la ruta:** `id`, `item_id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del ítem. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals items update

Actualizar ítem de la propuesta

```text
kobana billing proposals items update [ID] [ITEM_ID] [OPCIONES]
```

`PATCH /v1/proposals/{id}/items/{item_id}`

Actualiza un ítem. Solo permitido en propuestas en borrador (`status = draft`).

**Argumentos de la ruta:** `id`, `item_id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `quantity` | `integer` | no | Actualiza la cantidad. |
| `unit_amount` | `number` | no | Actualiza el valor unitario en BRL (float, hasta 4 decimales). |
| `setup_amount` | `number` | no | Actualiza el setup unitario en BRL (float, hasta 4 decimales). |
| `custom_metadata` | `object` | no | proposals.items.update.body.custom_metadata |

Otros campos y objetos anidados: `kobana schema billing.proposals.items.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Ítem actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals items delete

Eliminar ítem de la propuesta

```text
kobana billing proposals items delete [ID] [ITEM_ID] [OPCIONES]
```

`DELETE /v1/proposals/{id}/items/{item_id}`

Elimina un ítem. Solo permitido en propuestas en borrador (`status = draft`).

**Argumentos de la ruta:** `id`, `item_id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Ítem eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals stats

Obtener estadísticas de propuestas

```text
kobana billing proposals stats [OPCIONES]
```

`GET /v1/proposals/stats`

Devuelve conteos de propuestas por estado (`draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired`, `canceled`) para la organización actual.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Estadísticas de propuestas. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals list

Listar propuestas

```text
kobana billing proposals list [OPCIONES]
```

`GET /v1/proposals`

Devuelve una lista paginada de las propuestas comerciales de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `person_id` | `string` | no | Filtra por UUID de la persona asociada. |
| `person_document_number` | `string` | no | Filtra por CPF/CNPJ de la persona (coincidencia exacta). |
| `person_name_contains` | `string` | no | Filtra por nombre de la persona (búsqueda parcial, sin distinción de mayúsculas). |
| `status` | `string` | no | Filtra por estado: `draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired` o `canceled`. Valores: `draft`, `sent`, `viewed`, `accepted`, `rejected`, `expired`, `canceled`. |
| `billing_account_id` | `string` | no | Filtra por `billing_account_id` vinculado. |
| `prospect_id` | `string` | no | Filtra por `prospect_id`. |
| `contact_id` | `string` | no | Filtra por `contact_id` vinculado. |
| `coupon_id` | `string` | no | Filtra por `coupon_id` aplicado. |
| `company_id` | `string` | no | Filtra por `company_id` emisor. |
| `template_id` | `string` | no | Filtra por `template_id` usado para generar el PDF. |
| `code` | `string` | no | Filtra por código corto de la propuesta (ej.: `PROP-0001`). |
| `external_id` | `string` | no | Filtra por identificador externo. |
| `search` | `string` | no | Búsqueda textual libre. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `expires_after` | `string` | no | Filtra por `expires_at` inicial (ISO 8601, inclusivo). |
| `expires_before` | `string` | no | Filtra por `expires_at` final (ISO 8601, inclusivo). |
| `accepted_after` | `string` | no | Filtra por `accepted_at` inicial (ISO 8601, inclusivo). |
| `accepted_before` | `string` | no | Filtra por `accepted_at` final (ISO 8601, inclusivo). |
| `sent_after` | `string` | no | Filtra por `sent_at` inicial (ISO 8601, inclusivo). |
| `sent_before` | `string` | no | Filtra por `sent_at` final (ISO 8601, inclusivo). |
| `viewed_after` | `string` | no | Filtra por `viewed_at` inicial (ISO 8601, inclusivo). |
| `viewed_before` | `string` | no | Filtra por `viewed_at` final (ISO 8601, inclusivo). |
| `rejected_after` | `string` | no | Filtra por `rejected_at` inicial (ISO 8601, inclusivo). |
| `rejected_before` | `string` | no | Filtra por `rejected_at` final (ISO 8601, inclusivo). |
| `setup_amount_min` | `number,null` | no | Filtra por `setup_amount` mínimo (en BRL). |
| `setup_amount_max` | `number,null` | no | Filtra por `setup_amount` máximo (en BRL). |
| `monthly_amount_min` | `number,null` | no | Filtra por `monthly_amount` mínimo (en BRL). |
| `monthly_amount_max` | `number,null` | no | Filtra por `monthly_amount` máximo (en BRL). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de propuestas. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals create

Crear propuesta

```text
kobana billing proposals create [OPCIONES]
```

`POST /v1/proposals`

Crea una propuesta comercial. Es obligatorio informar `billing_account_id`, `prospect_id` o `prospect_data` (creación inline del prospect).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora. Usa `null` para limpiar. |
| `title` | `string` | no | Título de la propuesta. Por defecto: "Proposta Comercial". |
| `description` | `string` | no | Descripción libre de la propuesta. |
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro destino (cliente existente). Obligatorio si no se informan `prospect_id` ni `prospect_data`. |
| `prospect_id` | `string (uuid)` | no | UUID de un prospect existente. Alternativa a `billing_account_id`/`prospect_data`. |
| `contact_id` | `string (uuid)` | no | UUID de un contacto existente vinculado al prospect. |
| `prospect_data` | `object` | no | Datos para crear un prospect inline (persona jurídica). Alternativa a `prospect_id`/`billing_account_id`. |
| `contact_data` | `object` | no | Datos para crear un contacto inline (persona física responsable). |
| `coupon_id` | `string (uuid)` | no | UUID del cupón a aplicar. |
| `template_id` | `string (uuid)` | no | UUID del template para generar el PDF. |
| `discount_percent` | `number` | no | Porcentaje de descuento negociado (0–100), aplicado sobre el valor mensual; el descuento por ciclo se aplica sobre el valor con descuento. No lo envíe junto con `discount_amount`. |
| `discount_amount` | `number` | no | Descuento fijo negociado por mes en BRL (float, hasta 4 decimales), aplicado sobre el valor mensual; el descuento por ciclo se aplica sobre el valor con descuento. No lo envíe junto con `discount_percent`. |
| `commitment_months` | `integer` | no | Permanencia en meses (1–120). Use `null` para quitar. Copiado a la suscripción cuando se acepta la propuesta. |
| `expires_at` | `string` | no | Fecha de expiración. |
| `notes` | `string` | no | Notas internas. |
| `items` | `array<object>` | no | Ítems de la propuesta. Mínimo 1. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Campos personalizados (validados contra el schema de la organización). |

Otros campos y objetos anidados: `kobana schema billing.proposals.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Propuesta creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals update

Crear o actualizar propuesta

```text
kobana billing proposals update [OPCIONES]
```

`PUT /v1/proposals`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora. Usa `null` para limpiar. |
| `title` | `string` | no | Título de la propuesta. Por defecto: "Proposta Comercial". |
| `description` | `string` | no | Descripción libre de la propuesta. |
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro destino (cliente existente). Obligatorio si no se informan `prospect_id` ni `prospect_data`. |
| `prospect_id` | `string (uuid)` | no | UUID de un prospect existente. Alternativa a `billing_account_id`/`prospect_data`. |
| `contact_id` | `string (uuid)` | no | UUID de un contacto existente vinculado al prospect. |
| `prospect_data` | `object` | no | Datos para crear un prospect inline (persona jurídica). Alternativa a `prospect_id`/`billing_account_id`. |
| `contact_data` | `object` | no | Datos para crear un contacto inline (persona física responsable). |
| `coupon_id` | `string (uuid)` | no | UUID del cupón a aplicar. |
| `template_id` | `string (uuid)` | no | UUID del template para generar el PDF. |
| `discount_percent` | `number` | no | Porcentaje de descuento negociado (0–100), aplicado sobre el valor mensual; el descuento por ciclo se aplica sobre el valor con descuento. No lo envíe junto con `discount_amount`. |
| `discount_amount` | `number` | no | Descuento fijo negociado por mes en BRL (float, hasta 4 decimales), aplicado sobre el valor mensual; el descuento por ciclo se aplica sobre el valor con descuento. No lo envíe junto con `discount_percent`. |
| `commitment_months` | `integer` | no | Permanencia en meses (1–120). Use `null` para quitar. Copiado a la suscripción cuando se acepta la propuesta. |
| `expires_at` | `string` | no | Fecha de expiración. |
| `notes` | `string` | no | Notas internas. |
| `items` | `array<object>` | no | Ítems de la propuesta. Mínimo 1. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no | Campos personalizados (validados contra el schema de la organización). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.proposals.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals bulk-cancel

Cancelar propuestas en lote

```text
kobana billing proposals bulk-cancel [OPCIONES]
```

`POST /v1/proposals/bulk-cancel`

Cancela varias propuestas a la vez (hasta 100). Devuelve conteos de éxito/fallo y errores por id.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | Lista de UUIDs de las propuestas objetivo. Mínimo 1, máximo 100. |

Otros campos y objetos anidados: `kobana schema billing.proposals.bulk-cancel`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la cancelación en lote. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals bulk-send

Enviar propuestas en lote

```text
kobana billing proposals bulk-send [OPCIONES]
```

`POST /v1/proposals/bulk-send`

Envía varias propuestas a la vez (hasta 100). Devuelve conteos de éxito/fallo y errores por id.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | Lista de UUIDs de las propuestas objetivo. Mínimo 1, máximo 100. |

Otros campos y objetos anidados: `kobana schema billing.proposals.bulk-send`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado del envío en lote. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals get

Obtener propuesta

```text
kobana billing proposals get [ID] [OPCIONES]
```

`GET /v1/proposals/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la propuesta. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals delete

Eliminar propuesta

```text
kobana billing proposals delete [ID] [OPCIONES]
```

`DELETE /v1/proposals/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Propuesta eliminada. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals accept

Aceptar propuesta

```text
kobana billing proposals accept [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/accept`

Acepta la propuesta y crea (o confirma) la suscripción correspondiente en el ciclo informado.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cycle` | `string` | sí | Ciclo de cobro de la suscripción creada al aceptar: `monthly`, `quarterly`, `semiannual` o `annual`. Valores: `monthly`, `quarterly`, `semiannual`, `annual`. |

Otros campos y objetos anidados: `kobana schema billing.proposals.accept`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Propuesta aceptada — devuelve la propuesta, `subscription_id` e `invoice_id` cuando aplican. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals cancel

Cancelar propuesta

```text
kobana billing proposals cancel [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/cancel`

Cancela la propuesta (`status → canceled`).

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Propuesta cancelada. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals duplicate

Duplicar propuesta

```text
kobana billing proposals duplicate [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/duplicate`

Crea una nueva propuesta a partir de la original (borrador), preservando ítems y configuración.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Propuesta duplicada. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals generate-pdf

Generar PDF de la propuesta

```text
kobana billing proposals generate-pdf [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/generate-pdf`

Genera (o regenera) el PDF de la propuesta vía Google App Scripts. Requiere un `template` con `app_script_url` configurado.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | PDF generado — devuelve la propuesta actualizada y las URLs del documento/PDF. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals json

Obtener snapshot JSON de la propuesta

```text
kobana billing proposals json [ID] [OPCIONES]
```

`GET /v1/proposals/{id}/json`

Devuelve el snapshot completo de los datos usados para renderizar el template del PDF (espeja lo que recibe AppScript).

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Snapshot JSON de la propuesta. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals pricing

Obtener cálculo de precio de la propuesta

```text
kobana billing proposals pricing [ID] [OPCIONES]
```

`GET /v1/proposals/{id}/pricing`

Devuelve el detalle de precios por ciclo de cobro (mensual, trimestral, semestral, anual), con setup y descuentos aplicados.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de precios. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals publish

Publicar propuesta

```text
kobana billing proposals publish [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/publish`

Publica la propuesta (`draft → sent`) sin enviar el correo, dejándola disponible en el portal.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Propuesta publicada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals send

Enviar propuesta por correo

```text
kobana billing proposals send [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/send`

Envía (o reenvía) la propuesta por correo al destinatario. Estados permitidos: `draft`, `sent` o `viewed`. Opcionalmente personaliza `subject` y `message` (HTML).

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `subject` | `string` | no | Asunto personalizado del correo. Por defecto: `Proposta {code}`. |
| `message` | `string` | no | Mensaje HTML personalizado del cuerpo del correo. |

Otros campos y objetos anidados: `kobana schema billing.proposals.send`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Propuesta enviada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## proposals undo-accept

Deshacer aceptación de la propuesta

```text
kobana billing proposals undo-accept [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/undo-accept`

Deshace la aceptación: elimina la suscripción vinculada y vuelve el estado a `sent`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Aceptación deshecha. |
| `404` | Recurso no encontrado en la organización actual. |

## proposals unpublish

Despublicar propuesta

```text
kobana billing proposals unpublish [ID] [OPCIONES]
```

`POST /v1/proposals/{id}/unpublish`

Vuelve la propuesta a borrador (`sent`/`viewed` → `draft`).

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Propuesta despublicada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
