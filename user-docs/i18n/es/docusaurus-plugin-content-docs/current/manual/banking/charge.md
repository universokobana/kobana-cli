---
id: charge
title: "kobana banking charge"
sidebar_label: "charge"
description: "Cobros (Pix, boletos, Pix automático)"
---

# kobana banking charge

Cobros (Pix, boletos, Pix automático)

Comandos de `kobana banking charge`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`charge automatic-pix accounts list`](#charge-automatic-pix-accounts-list) | `GET /v2/charge/automatic_pix/accounts` | Lista todas las cuentas PIX automáticas |
| [`charge automatic-pix accounts create`](#charge-automatic-pix-accounts-create) | `POST /v2/charge/automatic_pix/accounts` | Crear nueva cuenta PIX automática |
| [`charge automatic-pix accounts get`](#charge-automatic-pix-accounts-get) | `GET /v2/charge/automatic_pix/accounts/{uid}` | Ver cuenta PIX automática |
| [`charge automatic-pix accounts update`](#charge-automatic-pix-accounts-update) | `PATCH /v2/charge/automatic_pix/accounts/{uid}` | Actualizar cuenta PIX automática |
| [`charge automatic-pix accounts delete`](#charge-automatic-pix-accounts-delete) | `DELETE /v2/charge/automatic_pix/accounts/{uid}` | Eliminar cuenta PIX automática |
| [`charge automatic-pix locations list`](#charge-automatic-pix-locations-list) | `GET /v2/charge/automatic_pix/locations` | Lista todas las Ubicaciones del Pix Automático |
| [`charge automatic-pix locations create`](#charge-automatic-pix-locations-create) | `POST /v2/charge/automatic_pix/locations` | Crear nueva Ubicación del PIX Automático |
| [`charge automatic-pix locations get`](#charge-automatic-pix-locations-get) | `GET /v2/charge/automatic_pix/locations/{uid}` | Consulta una Ubicación específica del PIX Automático |
| [`charge automatic-pix pix list`](#charge-automatic-pix-pix-list) | `GET /v2/charge/automatic_pix/pix` | Lista todas las cobrancias PIX automáticas |
| [`charge automatic-pix pix get`](#charge-automatic-pix-pix-get) | `GET /v2/charge/automatic_pix/pix/{uid}` | Información de la cobrancia PIX automática |
| [`charge automatic-pix pix update`](#charge-automatic-pix-pix-update) | `PATCH /v2/charge/automatic_pix/pix/{uid}` | Actualizar cobrancia PIX automática |
| [`charge automatic-pix pix cancel`](#charge-automatic-pix-pix-cancel) | `PUT /v2/charge/automatic_pix/pix/{uid}/cancel` | Cancelar cobrancia PIX automática |
| [`charge automatic-pix pix retry`](#charge-automatic-pix-pix-retry) | `PUT /v2/charge/automatic_pix/pix/{uid}/retry` | Reintentar cobrancia PIX automática |
| [`charge automatic-pix recurrences list`](#charge-automatic-pix-recurrences-list) | `GET /v2/charge/automatic_pix/recurrences` | Lista todas las recurrencias PIX automáticas |
| [`charge automatic-pix recurrences create`](#charge-automatic-pix-recurrences-create) | `POST /v2/charge/automatic_pix/recurrences` | Crear nueva recurrencia PIX automática |
| [`charge automatic-pix recurrences pix`](#charge-automatic-pix-recurrences-pix) | `POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/pix` | Crear nueva cobrancia PIX automática |
| [`charge automatic-pix recurrences requests`](#charge-automatic-pix-recurrences-requests) | `POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/requests` | Crear nueva solicitud PIX automática |
| [`charge automatic-pix recurrences get`](#charge-automatic-pix-recurrences-get) | `GET /v2/charge/automatic_pix/recurrences/{uid}` | Información de la recurrencia PIX automática |
| [`charge automatic-pix recurrences update`](#charge-automatic-pix-recurrences-update) | `PATCH /v2/charge/automatic_pix/recurrences/{uid}` | Actualizar recurrencia PIX automática |
| [`charge automatic-pix recurrences cancel`](#charge-automatic-pix-recurrences-cancel) | `PUT /v2/charge/automatic_pix/recurrences/{uid}/cancel` | Cancelar recurrencia PIX automática |
| [`charge automatic-pix requests list`](#charge-automatic-pix-requests-list) | `GET /v2/charge/automatic_pix/requests` | Lista todas las solicitudes PIX automáticas |
| [`charge automatic-pix requests get`](#charge-automatic-pix-requests-get) | `GET /v2/charge/automatic_pix/requests/{uid}` | Información de la solicitud PIX automática |
| [`charge automatic-pix requests update`](#charge-automatic-pix-requests-update) | `PATCH /v2/charge/automatic_pix/requests/{uid}` | Actualizar solicitud PIX automática |
| [`charge automatic-pix requests cancel`](#charge-automatic-pix-requests-cancel) | `PUT /v2/charge/automatic_pix/requests/{uid}/cancel` | Cancelar solicitud PIX automática |
| [`charge payments list`](#charge-payments-list) | `GET /v2/charge/payments` | Listar Recibos |
| [`charge payments create`](#charge-payments-create) | `POST /v2/charge/payments` | Informar Recibo |
| [`charge payments get`](#charge-payments-get) | `GET /v2/charge/payments/{uid}` | Información del Recibo |
| [`charge payments delete`](#charge-payments-delete) | `DELETE /v2/charge/payments/{uid}` | Eliminar Recibo |
| [`charge pix commands get`](#charge-pix-commands-get) | `GET /v2/charge/pix/{pix_uid}/commands/{uid}` | Ver un Comando |
| [`charge pix commands list`](#charge-pix-commands-list) | `GET /v2/charge/pix/{pix_uid}/commands` | Listar Comandos de un Pix |
| [`charge pix list`](#charge-pix-list) | `GET /v2/charge/pix` | Listar Pix |
| [`charge pix create`](#charge-pix-create) | `POST /v2/charge/pix` | Crear un Pix |
| [`charge pix cancel`](#charge-pix-cancel) | `PUT /v2/charge/pix/{pix_uid}/cancel` | Cancelar un Pix |
| [`charge pix update`](#charge-pix-update) | `PUT /v2/charge/pix/{pix_uid}/update` | Actualizar un Pix |
| [`charge pix get`](#charge-pix-get) | `GET /v2/charge/pix/{uid}` | Ver un Pix |
| [`charge pix delete`](#charge-pix-delete) | `DELETE /v2/charge/pix/{uid}` | Eliminar un Pix |
| [`charge pix-accounts list`](#charge-pix-accounts-list) | `GET /v2/charge/pix_accounts` | Listar Cuentas Pix |
| [`charge pix-accounts create`](#charge-pix-accounts-create) | `POST /v2/charge/pix_accounts` | Crear Cuenta Pix |
| [`charge pix-accounts get`](#charge-pix-accounts-get) | `GET /v2/charge/pix_accounts/{uid}` | Ver una cuenta Pix |
| [`charge pix-accounts update`](#charge-pix-accounts-update) | `PUT /v2/charge/pix_accounts/{uid}` | Actualizar Cuenta Pix |
| [`charge pix-accounts delete`](#charge-pix-accounts-delete) | `DELETE /v2/charge/pix_accounts/{uid}` | Eliminar Cuenta Pix |

## charge automatic-pix accounts list

Lista todas las cuentas PIX automáticas

```text
kobana banking charge automatic-pix accounts list [OPCIONES]
```

`GET /v2/charge/automatic_pix/accounts`

Lista todas las cuentas PIX automáticas del usuario con paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página para paginación |
| `per_page` | `integer` | no | Cantidad de elementos por página (predeterminado: 50, máximo: 50) |
| `external_id` | `string` | no | Filtrar por ID externo |
| `financial_account_uid` | `string` | no | Filtrar por UID de la cuenta financiera |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de cuentas PIX automáticas filtrada por financial_account_uid |

## charge automatic-pix accounts create

Crear nueva cuenta PIX automática

```text
kobana banking charge automatic-pix accounts create [OPCIONES]
```

`POST /v2/charge/automatic_pix/accounts`

Crea una nueva cuenta PIX automática.

**Importante**

Después de crear la cuenta, es necesario asociar una conexión a ella para que el PIX Automático funcione.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string` | sí | UID de la cuenta financiera asociada |
| `custom_name` | `string` | no | Nombre personalizado de la cuenta |
| `external_id` | `string` | no | ID externo para integración |
| `enabled` | `boolean` | no | Define si la cuenta está habilitada. |
| `custom_data` | `object` | no | Datos personalizados |
| `tags` | `array<string>` | no | Etiquetas de la cuenta |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cuenta PIX automática creada exitosamente |
| `422` | Datos inválidos |

## charge automatic-pix accounts get

Ver cuenta PIX automática

```text
kobana banking charge automatic-pix accounts get [UID] [OPCIONES]
```

`GET /v2/charge/automatic_pix/accounts/{uid}`

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalles de la cuenta PIX automática |
| `404` | Cuenta PIX automática no encontrada |

## charge automatic-pix accounts update

Actualizar cuenta PIX automática

```text
kobana banking charge automatic-pix accounts update [UID] [OPCIONES]
```

`PATCH /v2/charge/automatic_pix/accounts/{uid}`

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `custom_name` | `string` | no | Nombre personalizado de la cuenta |
| `external_id` | `string` | no | ID externo para integración |
| `enabled` | `boolean` | no | Define si la cuenta está habilitada. |
| `custom_data` | `object` | no | Datos personalizados |
| `tags` | `array<string>` | no | Etiquetas de la cuenta |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta PIX automática actualizada |

## charge automatic-pix accounts delete

Eliminar cuenta PIX automática

```text
kobana banking charge automatic-pix accounts delete [UID] [OPCIONES]
```

`DELETE /v2/charge/automatic_pix/accounts/{uid}`

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta PIX automática eliminada |
| `422` | La cuenta PIX automática tiene registros asociados (ej: recurrencia) |

## charge automatic-pix locations list

Lista todas las Ubicaciones del Pix Automático

```text
kobana banking charge automatic-pix locations list [OPCIONES]
```

`GET /v2/charge/automatic_pix/locations`

Lista todas las Ubicaciones del Pix Automático del usuario con paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página para paginación |
| `per_page` | `integer` | no | Cantidad de elementos por página (predeterminado: 50, máximo: 50) |
| `external_id` | `string` | no | Filtrar por ID externo |
| `charge_automatic_pix_account_uid` | `string` | no | Filtrar por UID de la cuenta Pix Automático |
| `financial_provider_external_id` | `string` | no | Filtrar por ID externo del proveedor financiero |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de Ubicaciones del Pix Automático |

## charge automatic-pix locations create

Crear nueva Ubicación del PIX Automático

```text
kobana banking charge automatic-pix locations create [OPCIONES]
```

`POST /v2/charge/automatic_pix/locations`

Crea una nueva Ubicación del PIX Automático.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `charge_automatic_pix_account_uid` | `string` | sí | UID de la cuenta Pix Automático |
| `external_id` | `string` | no | ID externo de la ubicación |
| `tags` | `array<string>` | no | Etiquetas para organización |
| `custom_data` | `object` | no | Datos personalizados |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.locations.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Ubicación del PIX Automático creada exitosamente |
| `422` | Datos inválidos para creación |

## charge automatic-pix locations get

Consulta una Ubicación específica del PIX Automático

```text
kobana banking charge automatic-pix locations get [UID] [OPCIONES]
```

`GET /v2/charge/automatic_pix/locations/{uid}`

Devuelve los detalles de una Ubicación específica del PIX Automático.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Ubicación del PIX Automático encontrada |
| `404` | Ubicación del PIX Automático no encontrada |

## charge automatic-pix pix list

Lista todas las cobrancias PIX automáticas

```text
kobana banking charge automatic-pix pix list [OPCIONES]
```

`GET /v2/charge/automatic_pix/pix`

Lista todas las cobrancias PIX automáticas con paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `automatic_pix_recurrence_uid` | `string` | no | Filtrar por UID de la recurrencia PIX automática |
| `external_id` | `string` | no | Filtrar por ID externo |
| `page` | `integer` | no | Número de página para paginación |
| `per_page` | `integer` | no | Cantidad de elementos por página (predeterminado: 50, máximo: 50) |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de cobrancias PIX automáticas |

## charge automatic-pix pix get

Información de la cobrancia PIX automática

```text
kobana banking charge automatic-pix pix get [UID] [OPCIONES]
```

`GET /v2/charge/automatic_pix/pix/{uid}`

Consulta información específica de una cobrancia PIX automática.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cobrancia PIX automática encontrada |
| `404` | Cobrancia PIX automática no encontrada |

## charge automatic-pix pix update

Actualizar cobrancia PIX automática

```text
kobana banking charge automatic-pix pix update [UID] [OPCIONES]
```

`PATCH /v2/charge/automatic_pix/pix/{uid}`

Actualiza información de una cobrancia PIX automática existente.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `external_id` | `string` | no | ID externo del cobro. |
| `tags` | `array<string>` | no | Etiquetas. |
| `custom_data` | `object` | no | Datos personalizados. |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.pix.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cobrancia PIX automática actualizada exitosamente |
| `404` | Cobrancia PIX automática no encontrada |
| `422` | ID Externo duplicado al actualizar |

## charge automatic-pix pix cancel

Cancelar cobrancia PIX automática

```text
kobana banking charge automatic-pix pix cancel [UID] [OPCIONES]
```

`PUT /v2/charge/automatic_pix/pix/{uid}/cancel`

Cancela una cobrancia PIX automática existente.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cobrancia PIX automática cancelada exitosamente |
| `404` | Cobrancia PIX automática no encontrada |
| `422` | Error al cancelar |

## charge automatic-pix pix retry

Reintentar cobrancia PIX automática

```text
kobana banking charge automatic-pix pix retry [UID] [OPCIONES]
```

`PUT /v2/charge/automatic_pix/pix/{uid}/retry`

Reintentar una cobrancia PIX automática que falló o expiró.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `date` | `string (date)` | sí | Fecha para reintentar la cobrancia. |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.pix.retry`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cobrancia PIX automática programada para reintento exitosamente |
| `404` | Cobrancia PIX automática no encontrada |
| `422` | Fecha inválida |

## charge automatic-pix recurrences list

Lista todas las recurrencias PIX automáticas

```text
kobana banking charge automatic-pix recurrences list [OPCIONES]
```

`GET /v2/charge/automatic_pix/recurrences`

Lista todas las recurrencias PIX automáticas del usuario con paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página para paginación |
| `per_page` | `integer` | no | Cantidad de elementos por página (predeterminado: 50, máximo: 50) |
| `external_id` | `string` | no | Filtrar por ID externo |
| `charge_automatic_pix_account_uid` | `string` | no | Filtrar por UID de la cuenta Pix Automático |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de recurrencias PIX automáticas |

## charge automatic-pix recurrences create

Crear nueva recurrencia PIX automática

```text
kobana banking charge automatic-pix recurrences create [OPCIONES]
```

`POST /v2/charge/automatic_pix/recurrences`

Crea una nueva recurrencia PIX automática.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `charge_automatic_pix_account_uid` | `string` | sí | UID de la cuenta Pix Automático. La cuenta financiera y la conexión se heredan de esta cuenta. |
| `document` | `string` | sí | Número, identificador o código que representa el objeto de la autorización (contrato, pedido, etc.). |
| `start_at` | `string (date_time)` | sí | Fecha de inicio de la recurrencia. |
| `recurrence_term` | `string` | sí | Plazo de la recurrencia. Valores: `indefinite`, `fixed`. |
| `cycle` | `string` | sí | Ciclo de la recurrencia. Valores: `weekly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `amount_term` | `string` | sí | Tipo del valor de la recurrencia. Valores: `fixed`, `variable`. |
| `retry_policy` | `string` | sí | Política de reintento. Valores: `disallow`, `allow_3r7d`. |
| `debtor` | `object` | sí | Información de la persona deudora |
| `pix_uid` | `string` | no | UID del Pix (opcional). Cuando se informa junto con location_uid: el código QR generado será de la jornada instant (Jornada 3) para Pix instantáneo u offer (Jornada 4) para Pix con vencimiento. |
| `location_uid` | `string` | no | UID de la ubicación. Cuando se informa, el código QR se generará conforme a la jornada (recurrent / instant / offer). |
| `description` | `string` | no | Descripción de la recurrencia. |
| `finish_at` | `string (date_time)` | no | Fecha de fin de la recurrencia. |
| `amount` | `number` | no | Valor (cuando amount_term es fixed). |
| `minimum_amount` | `number` | no | Valor mínimo (cuando amount_term es variable). |
| `agreement_code` | `string` | no | Número del Acuerdo de Pago. |
| `external_id` | `string` | no | ID externo de la recurrencia. |
| `custom_data` | `object` | no | Datos personalizados. |
| `tags` | `array<string>` | no | Etiquetas. |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.recurrences.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Recurrencia PIX automática creada con pix_uid y location_uid (JORNADA_3) |
| `422` | ID Externo ya existe |

## charge automatic-pix recurrences pix

Crear nueva cobrancia PIX automática

```text
kobana banking charge automatic-pix recurrences pix [RECURRENCE_UID] [OPCIONES]
```

`POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/pix`

Crea una nueva cobrancia PIX automática para una recurrencia.

**Argumentos de la ruta:** `recurrence_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Valor del cobro. |
| `retry_policy` | `string` | sí | Política de reintento. Valores: `disallow`, `allow_3r7d`. |
| `next_business_day` | `boolean` | sí | Si debe considerar próximo día hábil. |
| `expire_at` | `string (date_time)` | sí | Fecha de expiración del cobro. |
| `additional_info` | `string` | no | Información adicional del cobro. |
| `txid` | `string` | no | Identificador de la transacción (opcional; generado automáticamente si se omite). |
| `external_id` | `string` | no | ID externo del cobro. |
| `tags` | `array<string>` | no | Etiquetas. |
| `custom_data` | `object` | no | Datos personalizados. |
| `debtor` | `object` | no | Datos adicionales del deudor |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.recurrences.pix`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cobrancia PIX automática creada con txid personalizado |
| `422` | ID Externo duplicado |

## charge automatic-pix recurrences requests

Crear nueva solicitud PIX automática

```text
kobana banking charge automatic-pix recurrences requests [RECURRENCE_UID] [OPCIONES]
```

`POST /v2/charge/automatic_pix/recurrences/{recurrence_uid}/requests`

Crea una nueva solicitud PIX automática para una recurrencia.

**Argumentos de la ruta:** `recurrence_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `expire_at` | `string (date_time)` | sí | Fecha de expiración de la solicitud. |
| `addressee_financial_account` | `object` | sí | Cuenta Financiera del Destinatario (usuario pagador) |
| `external_id` | `string` | no | ID externo de la solicitud. |
| `custom_data` | `object` | no | Datos personalizados. |
| `tags` | `array<string>` | no | Etiquetas. |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.recurrences.requests`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Solicitud PIX automática creada exitosamente |
| `422` | ID Externo duplicado |

## charge automatic-pix recurrences get

Información de la recurrencia PIX automática

```text
kobana banking charge automatic-pix recurrences get [UID] [OPCIONES]
```

`GET /v2/charge/automatic_pix/recurrences/{uid}`

Consulta información específica de una recurrencia PIX automática.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Recurrencia PIX automática encontrada |
| `404` | Recurrencia PIX automática no encontrada |

## charge automatic-pix recurrences update

Actualizar recurrencia PIX automática

```text
kobana banking charge automatic-pix recurrences update [UID] [OPCIONES]
```

`PATCH /v2/charge/automatic_pix/recurrences/{uid}`

Actualiza información de una recurrencia PIX automática existente.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `external_id` | `string` | no | ID externo de la recurrencia. |
| `tags` | `array<string>` | no | Etiquetas. |
| `custom_data` | `object` | no | Datos personalizados. |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.recurrences.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Recurrencia PIX automática actualizada exitosamente |
| `404` | Recurrencia PIX automática no encontrada |
| `422` | ID Externo ya existe al actualizar |

## charge automatic-pix recurrences cancel

Cancelar recurrencia PIX automática

```text
kobana banking charge automatic-pix recurrences cancel [UID] [OPCIONES]
```

`PUT /v2/charge/automatic_pix/recurrences/{uid}/cancel`

Cancela una recurrencia PIX automática existente.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Recurrencia PIX automática cancelada exitosamente |
| `422` | Error al cancelar |

## charge automatic-pix requests list

Lista todas las solicitudes PIX automáticas

```text
kobana banking charge automatic-pix requests list [OPCIONES]
```

`GET /v2/charge/automatic_pix/requests`

Lista todas las solicitudes PIX automáticas con paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de página para paginación |
| `per_page` | `integer` | no | Cantidad de elementos por página (predeterminado: 50, máximo: 50) |
| `automatic_pix_recurrence_uid` | `string` | no | Filtrar por UID de la recurrencia PIX automática |
| `charge_automatic_pix_account_uid` | `string` | no | Filtrar por UID de la cuenta Pix Automático |
| `external_id` | `string` | no | Filtrar por ID externo |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de solicitudes PIX automáticas |

## charge automatic-pix requests get

Información de la solicitud PIX automática

```text
kobana banking charge automatic-pix requests get [UID] [OPCIONES]
```

`GET /v2/charge/automatic_pix/requests/{uid}`

Consulta información específica de una solicitud PIX automática.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Solicitud PIX automática encontrada |
| `404` | Solicitud PIX automática no encontrada |

## charge automatic-pix requests update

Actualizar solicitud PIX automática

```text
kobana banking charge automatic-pix requests update [UID] [OPCIONES]
```

`PATCH /v2/charge/automatic_pix/requests/{uid}`

Actualiza información de una solicitud PIX automática existente.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `external_id` | `string` | no | ID externo de la solicitud. |
| `tags` | `array<string>` | no | Etiquetas. |
| `custom_data` | `object` | no | Datos personalizados. |

Otros campos y objetos anidados: `kobana schema banking.charge.automatic-pix.requests.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Solicitud PIX automática actualizada exitosamente |
| `404` | Solicitud PIX automática no encontrada |
| `422` | ID Externo duplicado al actualizar |

## charge automatic-pix requests cancel

Cancelar solicitud PIX automática

```text
kobana banking charge automatic-pix requests cancel [UID] [OPCIONES]
```

`PUT /v2/charge/automatic_pix/requests/{uid}/cancel`

Cancela una solicitud PIX automática existente.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Solicitud PIX automática cancelada exitosamente |
| `404` | Solicitud PIX automática no encontrada |
| `422` | Error al cancelar |

## charge payments list

Listar Recibos

```text
kobana banking charge payments list [OPCIONES]
```

`GET /v2/charge/payments`

Listar Recibos.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página. |
| `per_page` | `integer` | no | Cantidad de registros por página. |
| `paid_from` | `string` | no | A partir de la Fecha de Recibo. |
| `paid_to` | `string` | no | Hasta la Fecha de Recibo. |
| `amount_from` | `number` | no | A partir del Valor del Recibo. |
| `amount_to` | `number` | no | Hasta el Valor del Recibo. |
| `kind` | `string` | no | Tipo de Recibo. Directo al beneficiario/En manos, Boleto (Confirmado por el banco mediante archivo de retorno o API) o PIX (Confirmado por el banco). Valores: `direct`, `bank_billet`, `pix`. |
| `payable_type` | `string` | no | Tipo de Recurso Pagadero. Valores: `BankBillet`, `Pix`. |
| `payable_public_id` | `string` | no | UID del Recurso Pagadero. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Buscando por tipo de recibo |

## charge payments create

Informar Recibo

```text
kobana banking charge payments create [OPCIONES]
```

`POST /v2/charge/payments`

Informa un Recibo.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Valor del Recibimiento. |
| `kind` | `string` | sí | Tipo de Recibimiento. Valores: `direct`, `bank_billet`, `pix`. |
| `paid_at` | `string (date)` | sí | Fecha do Recepción. |
| `payable` | `object` | sí | Recurso asociado al pago (Boleto o Pix). |
| `uid` | `string (uuid)` | no | UID del Recibimiento. |
| `debtor` | `object` | no | Información del deudor (deudor) del boleto. |
| `payer` | `object` | no | Información de quién realizó efectivamente el pago (portador). Se rellena automáticamente mediante consulta diaria al banco, disponible D+2 días hábiles después del pago. Nulo cuando la consulta aún no se ha realizado o el banco no retornó los datos del pagador. |
| `bank` | `object` | no | Información bancaria. |
| `receiving` | `object` | no | Información de recibimiento. |
| `credit_at` | `string (date)` | no | Fecha de crédito. |
| `created_via_api` | `boolean` | no | Enviado por API. |
| `transaction_identifier` | `string` | no | Identificador de la Transacción. |
| `status` | `string` | no | Situación. |
| `duplicated` | `object` | no | Presente cuando el recibimiento es un duplicado. Contiene el UID del recibimiento original. |
| `created_at` | `string (date)` | no | Fecha y hora de creación. |
| `updated_at` | `string (date)` | no | Fecha y hora de la alteración. |

Otros campos y objetos anidados: `kobana schema banking.charge.payments.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Recibo de pix informado exitosamente |
| `422` | Error de validación |

## charge payments get

Información del Recibo

```text
kobana banking charge payments get [UID] [OPCIONES]
```

`GET /v2/charge/payments/{uid}`

Consulta información específica de un Recibo.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Recibo encontrado |
| `404` | Recibo no encontrado |

## charge payments delete

Eliminar Recibo

```text
kobana banking charge payments delete [UID] [OPCIONES]
```

`DELETE /v2/charge/payments/{uid}`

Elimina un Recibo.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Recibo eliminado |
| `404` | Recibo no encontrado |

## charge pix commands get

Ver un Comando

```text
kobana banking charge pix commands get [PIX_UID] [UID] [OPCIONES]
```

`GET /v2/charge/pix/{pix_uid}/commands/{uid}`

Consulta información detallada de un comando específico ejecutado en un cobro Pix.

**Argumentos de la ruta:** `pix_uid`, `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Actualización del Pix encontrada |
| `404` | Actualización del Pix no encontrada |

## charge pix commands list

Listar Comandos de un Pix

```text
kobana banking charge pix commands list [PIX_UID] [OPCIONES]
```

`GET /v2/charge/pix/{pix_uid}/commands`

Lista todos los comandos (operaciones) ejecutados en un cobro Pix específico, incluyendo actualizaciones, cancelaciones y otras operaciones.

**Argumentos de la ruta:** `pix_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pix encontrado |
| `404` | Pix no encontrado |

## charge pix list

Listar Pix

```text
kobana banking charge pix list [OPCIONES]
```

`GET /v2/charge/pix`

Lista todos los cobros Pix registrados. Permite filtrar por estado, TXID, fecha de vencimiento, fecha de pago, CNPJ/CPF del pagador y etiquetas.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `txid` | `string` | no | Filtro por TXID del Pix |
| `status` | `string` | no | Filtro por estado del Pix |
| `pix_account_id` | `integer` | no | Filtro por ID de la cuenta del Pix |
| `pix_account_uid` | `string` | no | Filtro por UID de la cuenta del Pix |
| `expire_from` | `string` | no | Fecha inicial del filtro de fecha de vencimiento del Pix |
| `expire_to` | `string` | no | Fecha final del filtro de fecha de vencimiento del Pix |
| `paid_from` | `string` | no | Fecha inicial del filtro de fecha de pago del Pix |
| `paid_to` | `string` | no | Fecha final del filtro de fecha de pago del Pix |
| `created_from` | `string` | no | Fecha inicial del filtro de fecha de creación del Pix |
| `created_to` | `string` | no | Fecha final del filtro de fecha de creación del Pix |
| `cnpj_cpf` | `string` | no | Filtro por CNPJ/CPF del pagador |
| `tags` | `string` | no | Filtro por etiquetas del pix |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Buscando Pix por etiquetas |
| `422` | cnpj_cpf inválido |

## charge pix create

Crear un Pix

```text
kobana banking charge pix create [OPCIONES]
```

`POST /v2/charge/pix`

Este endpoint se utiliza para la generación de un cobro Pix.

La creación del Pix es síncrona, pero el registro en la institución financiera se realiza en segundo plano. Por lo tanto, la respuesta de la solicitud no contendrá el campo `qrcode`.

Para recibir el qrcode es necesario suscribirse a un webhook para recibir el payload completo del Pix cuando el registro sea confirmado.

**Requisito previo**

Para emitir un cobro Pix es necesario tener registrada en su cuenta una Cuenta Pix. Todo Pix pertenece a una Cuenta Pix que representa la institución de pagos y la clave que se utilizará para la emisión del Pix.
La creación de la Cuenta Pix necesita hacerse a través de la interfaz web del sistema o mediante el endpoint `POST /api/v2/charge/pix_accounts`.

**Eventos de Webhook**

Al registrar un Pix el sistema crea un comando para registrar el PIX en la institución financiera y el evento `pix.register.requested` es disparado. Después de que el registro del Pix sea confirmado en la institución el evento `pix.register.confirmed` es disparado.

El evento `pix.db.created` también es disparado en esta operación.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Cantidad en reales |
| `pix_account_uid` | `string (uuid)` | sí | UID de la Cuenta Pix |
| `expire_at` | `string (date-time)` | sí | Fecha y hora de vencimiento. Formato iso8601 |
| `id` | `integer` | no | ID del Pix (DEPRECATED: usa uid) |
| `uid` | `string (uuid)` | no | UID del Pix |
| `payer` | `object` | no | Datos del Pagador |
| `status` | `string` | no | Estado del Pix. * `opened`: Cobranza Pix abierta (no pagada). [Estado Inicial] * `paid`: Cobranza Pix pagada. * `overdue`: Cobranza Pix vencida. * `canceled`: Cobranza Pix cancelada. * `generation_failed`: Cobranza Pix creada, pero no se registró en el banco. Valores: `opened`, `paid`, `overdue`, `canceled`, `generation_failed`. |
| `pix_account_id` | `integer` | no | ID de la Cuenta Pix (DEPRECATED: usa pix_account_uid) |
| `txid` | `anyOf` | no | TXID del Pix - Identificador único de la transacción |
| `canceled_at` | `string (date-time)` | no | Fecha y hora de cancelación del Pix |
| `registered_at` | `string (date-time)` | no | Fecha y hora de registro en el proveedor |
| `paid_at` | `string (date-time)` | no | Fecha y hora del pago |
| `paid_amount` | `number` | no | Cantidad efectivamente pagada |
| `registration_status` | `string` | no | Estado del registro en el proveedor Valores: `pending`, `skipped`, `requested`, `confirmed`, `rejected`, `failed`. |
| `revoke_days` | `anyOf` | no | Número de días activo después del vencimiento. OBLIGATORIO y debe ser mayor que cero cuando el Pix tiene intereses (interest) o multa (fine) configurados. |
| `message` | `anyOf` | no | Mensaje de solicitud al pagador |
| `message_payer` | `anyOf` | no | Mensaje para el pagador (campo solo lectura) |
| `beneficiary` | `object` | no | Datos del beneficiario del Pix |
| `url` | `string` | no | URL para visualización del Pix |
| `formats` | `object` | no | URLs con formatos disponibles para el Pix |
| `additional_info` | `anyOf` | no | Información adicional para el pagador (máximo 10 pares clave-valor) |
| `qrcode` | `object` | no | Código QR del Pix |
| `registration_kind` | `string` | no | Tipo del Pix. * `instant`: Inmediato (predeterminado) * `billing`: Cobranza Valores: `instant`, `billing`. |
| `kind` | `string` | no | Tipo del Pix. * `instant`: Inmediato (predeterminado) * `billing`: Cobranza Valores: `instant`, `billing`. |
| `command_ids` | `array<integer>` | no |  |
| `payments` | `array<string (uuid)>` | no | Lista de UIDs de los Recibimientos vinculados al Pix |
| `end_to_end_id` | `anyOf` | no | End To End Id del Pix - Identificador único de la transferencia |
| `custom_data` | `anyOf` | no | Hash con clave y valor en formato JSON. |
| `external_id` | `string` | no | Identificador externo para el Pix |
| `virtual_pix_uid` | `string` | no | UID del Pix generado por miembro de BackOffice. (BackOffice debe estar habilitado). |
| `fine_type` | `integer` | no | Tipo de la multa: * `0` Sin multa (`none`) * `1` Por valor (`amount`) * `2` Por porcentaje (`percentage`) En el retorno los datos vienen en el objeto `fine`. Valores: `0`, `1`, `2`. |
| `fine_amount` | `number` | no | Valor de la multa en reales. Obligatorio cuando `fine_type` = `amount`. |
| `fine_percentage` | `number` | no | Porcentaje de la multa (0.0 a 100.0). Obligatorio cuando `fine_type` = `percentage`. |
| `interest_type` | `integer` | no | Tipo de interés: * `0` No aplica / exento (`none`) * `1` Valor diario — días corridos (`daily_amount_calendar`) * `2` Porcentaje diario — días corridos (`daily_percentage_calendar`) * `3` Porcentaje mensual — días corridos (`monthly_percentage_calendar`) * `4` Porcentaje anual — días corridos (`yearly_percentage_calendar`) * `5` Valor diario — días útiles (`daily_amount_business`) * `6` Porcentaje diario — días útiles (`daily_percentage_business`) * `7` Porcentaje mensual — días útiles (`monthly_percentage_business`) * `8` Porcentaje anual — días útiles (`yearly_percentage_business`) En el retorno los datos vienen en el objeto `interest`. Valores: `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`. |
| `interest_amount` | `number` | no | Valor diario de interés en reales. Obligatorio cuando `interest_type` sea por valor (`daily_amount_*`). |
| `interest_percentage` | `number` | no | Porcentaje de interés. Obligatorio cuando `interest_type` sea por porcentaje. |
| `reduction_type` | `integer` | no | Tipo de descuento: * `1` Por valor (`amount`) * `2` Por porcentaje (`percentage`) * `3` No aplica — elimina el descuento (`none`) En el retorno los datos vienen en el objeto `reduction`. Valores: `1`, `2`, `3`. |
| `reduction_amount` | `number` | no | Valor del descuento en reales. Obligatorio cuando `reduction_type` = `amount`. |
| `reduction_percentage` | `number` | no | Porcentaje del descuento. Obligatorio cuando `reduction_type` = `percentage`. |
| `discount_type` | `integer` | no | Tipo de descuento aplicado a los campos `discount_*` (nulo/ausente = sin descuento): * `1` Valor fijo (`fixed_amount`) * `2` Porcentaje fijo (`fixed_percentage`) * `3` Anticipación por valor — días corridos (`advance_amount_calendar`) * `4` Anticipación por valor — días útiles (`advance_amount_business`) * `5` Anticipación por porcentaje — días corridos (`advance_percentage_calendar`) * `6` Anticipación por porcentaje — días útiles (`advance_percentage_business`) Las modalidades de anticipación (`3`–`6`) aceptan solo un rango. En el retorno el tipo viene en `discounts.type`. Valores: `1`, `2`, `3`, `4`, `5`, `6`. |
| `discount_amount` | `number` | no | Valor del 1º descuento en reales. Obligatorio cuando `discount_type` sea por valor. |
| `discount_percentage` | `number` | no | Porcentaje del 1º descuento. Obligatorio cuando `discount_type` sea por porcentaje. |
| `discount_days` | `integer` | no | Días antes del vencimiento en que el 1º descuento es válido. |
| `discount_second_amount` | `number` | no | Valor del 2º descuento en reales. |
| `discount_second_percentage` | `number` | no | Porcentaje del 2º descuento. |
| `discount_second_days` | `integer` | no | Días antes del vencimiento en que el 2º descuento es válido. |
| `discount_third_amount` | `number` | no | Valor del 3º descuento en reales. |
| `discount_third_percentage` | `number` | no | Porcentaje del 3º descuento. |
| `discount_third_days` | `integer` | no | Días antes del vencimiento en que el 3º descuento es válido. |
| `fine` | `object` | no | Configuración de multa (RETORNO/response como objeto anidado). En el ENVÍO/request utilice los campos planos `fine_type`, `fine_amount` y `fine_percentage`. |
| `reduction` | `object` | no | Configuración de descuento (RETORNO/response como objeto anidado). En el ENVÍO/request utilice los campos planos `reduction_type`, `reduction_amount` y `reduction_percentage`. |
| `interest` | `object` | no | Configuración de interés (RETORNO/response como objeto anidado). En el ENVÍO/request utilice los campos planos `interest_type`, `interest_amount` e `interest_percentage`. |
| `discounts` | `object` | no | Configuración de descuentos (RETORNO/response como objeto anidado con hasta 3 descuentos). En el ENVÍO/request utilice los campos planos `discount_*`, `discount_second_*` y `discount_third_*`. |
| `ignore_whatsapp` | `boolean` | no | Nunca envíe este Pix por WhatsApp |
| `password_protected_mode` | `integer` | no | Modo de protección por contraseña: 0=deshabilitado, 1=últimos 4 dígitos, 2=primeros 5 dígitos Valores: `0`, `1`, `2`. |
| `tags` | `array<string>` | no | Etiquetas asociadas al pix |
| `tag_list` | `string` | no | Etiquetas asociadas al pix |
| `created_via_api` | `boolean` | no | Indica si el Pix fue creado por API |
| `financial_provider_external_id` | `string` | no | ID en la institución financiera |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.charge.pix.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Pix creado con datos de cliente |
| `422` | Error de validación |

## charge pix cancel

Cancelar un Pix

```text
kobana banking charge pix cancel [PIX_UID] [OPCIONES]
```

`PUT /v2/charge/pix/{pix_uid}/cancel`

> **Procesamiento Asíncrono:** Este endpoint se procesa de forma asincrónica y crea un comando de cancelación para el PIX.
La cancelación de un PIX se realiza de forma asincrónica, es decir, al recibir la solicitud, nuestra API guarda la solicitud y crea un Comando. Este comando se procesa en segundo plano y después de confirmarse, el Pix es cancelado. Recibe en la respuesta de esta solicitud el ID del comando creado y puede consultarlo en cualquier momento. Vea más en Listar Comandos de un Pix y Ver un Comando.

**Eventos de Webhook**

Cuando se realiza la solicitud, un evento `pix.command.cancel.started` es disparado.
Nuestro servidor entonces comienza los intentos de envío de la cancelación a la institución financiera.
Solo después de confirmado en la institución financiera, entonces el pix es cancelado en nuestra base de datos.
En este momento el evento `pix.command.cancel.confirmed` es disparado.

El evento `pix.db.updated` también es disparado en la confirmación de esta operación.
> ⚠️ **Atención al payload de retorno:** La respuesta de la llamada de cancelación de un PIX es el payload de un Comando. Dentro del payload del comando está la clave `pix` con los datos del PIX en cuestión.

**Argumentos de la ruta:** `pix_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `422` | Pix ya cancelado |

## charge pix update

Actualizar un Pix

```text
kobana banking charge pix update [PIX_UID] [OPCIONES]
```

`PUT /v2/charge/pix/{pix_uid}/update`

> **Procesamiento Asíncrono:** Este endpoint se procesa de forma asincrónica y crea un comando de actualización para el PIX.
La actualización de un PIX se realiza de forma asincrónica, es decir, al recibir la solicitud, nuestra API guarda los cambios que está solicitando y crea un Comando. Este comando se procesa en segundo plano y después de confirmarse, el Pix es actualizado. Recibe en la respuesta de esta solicitud el ID del comando creado y puede consultarlo en cualquier momento. Vea más en Listar Comandos de un Pix y Ver un Comando.

**Eventos de Webhook**

Cuando se realiza la solicitud, un evento `pix.update.requested` es disparado.
Nuestro servidor entonces comienza los intentos de envío del cambio a la institución financiera.
Solo después de confirmado en la institución financiera, entonces el pix es actualizado.
En este momento el evento `pix.update.confirmed` es disparado.

El evento `pix.db.updated` también es disparado en la confirmación de esta operación.
> ⚠️ **Atención al payload de retorno:** La respuesta de la llamada de actualización de un PIX es el payload de un Comando. Dentro del payload del comando está la clave `pix` con los datos del PIX en cuestión.

**Argumentos de la ruta:** `pix_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | no | Cantidad |
| `pix_account_uid` | `string (uuid)` | no | UID de la Cuenta Pix para cambiar la clave Pix del receptor. Este campo es útil cuando desea cambiar la clave Pix en el proveedor financiero. **Importante**: Solo se aceptan cuentas Pix que estén activas (estado `active`). |
| `payer` | `object` | no | Datos del Pagador |
| `status` | `string` | no | Estado del Pix. `opened`: Cobranza Pix abierta (no pagada). [Estado Inicial] `paid`: Cobranza Pix pagada. `overdue`: Cobranza Pix vencida. `canceled`: Cobranza Pix cancelada. `generation_failed`: Cobranza Pix creada, pero no se registró en el banco. |
| `expire_at` | `string (date-time)` | no | Fecha y hora de vencimiento. Formato iso8601 |
| `revoke_days` | `integer` | no | Número de días activo después del vencimiento |
| `message` | `string` | no | Mensaje de solicitud al pagador |
| `additional_info` | `object` | no | Información adicional para el pagador |
| `fine_type` | `integer` | no | Tipo de la multa: * `0` Sin multa (`none`) * `1` Por valor (`amount`) * `2` Por porcentaje (`percentage`) En el retorno los datos vienen en el objeto `fine`. Valores: `0`, `1`, `2`. |
| `fine_amount` | `number` | no | Valor de la multa en reales. Obligatorio cuando `fine_type` = `amount`. |
| `fine_percentage` | `number` | no | Porcentaje de la multa (0.0 a 100.0). Obligatorio cuando `fine_type` = `percentage`. |
| `interest_type` | `integer` | no | Tipo de interés: * `0` No aplica / exento (`none`) * `1` Valor diario — días corridos (`daily_amount_calendar`) * `2` Porcentaje diario — días corridos (`daily_percentage_calendar`) * `3` Porcentaje mensual — días corridos (`monthly_percentage_calendar`) * `4` Porcentaje anual — días corridos (`yearly_percentage_calendar`) * `5` Valor diario — días útiles (`daily_amount_business`) * `6` Porcentaje diario — días útiles (`daily_percentage_business`) * `7` Porcentaje mensual — días útiles (`monthly_percentage_business`) * `8` Porcentaje anual — días útiles (`yearly_percentage_business`) En el retorno los datos vienen en el objeto `interest`. Valores: `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`. |
| `interest_amount` | `number` | no | Valor diario de interés en reales. Obligatorio cuando `interest_type` sea por valor (`daily_amount_*`). |
| `interest_percentage` | `number` | no | Porcentaje de interés. Obligatorio cuando `interest_type` sea por porcentaje. |
| `reduction_type` | `integer` | no | Tipo de descuento: * `1` Por valor (`amount`) * `2` Por porcentaje (`percentage`) * `3` No aplica — elimina el descuento (`none`) En el retorno los datos vienen en el objeto `reduction`. Valores: `1`, `2`, `3`. |
| `reduction_amount` | `number` | no | Valor del descuento en reales. Obligatorio cuando `reduction_type` = `amount`. |
| `reduction_percentage` | `number` | no | Porcentaje del descuento. Obligatorio cuando `reduction_type` = `percentage`. |
| `discount_type` | `integer` | no | Tipo de descuento aplicado a los campos `discount_*` (nulo/ausente = sin descuento): * `1` Valor fijo (`fixed_amount`) * `2` Porcentaje fijo (`fixed_percentage`) * `3` Anticipación por valor — días corridos (`advance_amount_calendar`) * `4` Anticipación por valor — días útiles (`advance_amount_business`) * `5` Anticipación por porcentaje — días corridos (`advance_percentage_calendar`) * `6` Anticipación por porcentaje — días útiles (`advance_percentage_business`) Las modalidades de anticipación (`3`–`6`) aceptan solo un rango. En el retorno el tipo viene en `discounts.type`. Valores: `1`, `2`, `3`, `4`, `5`, `6`. |
| `discount_amount` | `number` | no | Valor del 1º descuento en reales. Obligatorio cuando `discount_type` sea por valor. |
| `discount_percentage` | `number` | no | Porcentaje del 1º descuento. Obligatorio cuando `discount_type` sea por porcentaje. |
| `discount_days` | `integer` | no | Días antes del vencimiento en que el 1º descuento es válido. |
| `discount_second_amount` | `number` | no | Valor del 2º descuento en reales. |
| `discount_second_percentage` | `number` | no | Porcentaje del 2º descuento. |
| `discount_second_days` | `integer` | no | Días antes del vencimiento en que el 2º descuento es válido. |
| `discount_third_amount` | `number` | no | Valor del 3º descuento en reales. |
| `discount_third_percentage` | `number` | no | Porcentaje del 3º descuento. |
| `discount_third_days` | `integer` | no | Días antes del vencimiento en que el 3º descuento es válido. |
| `created_via_api` | `boolean` | no | Indica si el Pix fue creado por API |
| `financial_provider_external_id` | `string` | no | ID en la institución financiera |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.charge.pix.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente - con pix_account_uid |
| `422` | Pix pagado |

## charge pix get

Ver un Pix

```text
kobana banking charge pix get [UID] [OPCIONES]
```

`GET /v2/charge/pix/{uid}`

Consulta información específica de un Pix.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pix encontrado |
| `404` | Pix no encontrado |

## charge pix delete

Eliminar un Pix

```text
kobana banking charge pix delete [UID] [OPCIONES]
```

`DELETE /v2/charge/pix/{uid}`

Elimina un Pix cancelado.

Esta operación elimina el PIX definitivamente de nuestra base de datos. Incluso realizando la eliminación, el PIX se contabiliza en el volumen para el plan contratado

**Eventos de Webhook**

El evento `pix.db.deleted` es disparado en esta operación.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Pix eliminado |
| `404` | Pix no encontrado |
| `422` | Pix abierto |

## charge pix-accounts list

Listar Cuentas Pix

```text
kobana banking charge pix-accounts list [OPCIONES]
```

`GET /v2/charge/pix_accounts`

Lista las cuentas Pix registradas en la cuenta.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuentas Pix encontradas |

## charge pix-accounts create

Crear Cuenta Pix

```text
kobana banking charge pix-accounts create [OPCIONES]
```

`POST /v2/charge/pix_accounts`

Este endpoint se utiliza para registrar una nueva cuenta pix.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string` | sí | UID de la cuenta financiera. El proveedor y datos del beneficiario se derivan automáticamente. |
| `key` | `string` | sí | Clave pix |
| `custom_name` | `string` | no | Nombre personalizado de la cuenta pix |
| `enabled` | `boolean` | no | Habilitar Cuenta Pix |
| `default` | `boolean` | no | Cuenta predeterminada |

Otros campos y objetos anidados: `kobana schema banking.charge.pix-accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cuenta Pix creada |
| `422` | Cuenta Pix inválida |

## charge pix-accounts get

Ver una cuenta Pix

```text
kobana banking charge pix-accounts get [UID] [OPCIONES]
```

`GET /v2/charge/pix_accounts/{uid}`

Consulta información específica de una cuenta Pix.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta Pix encontrada |
| `404` | Cuenta Pix no encontrada |

## charge pix-accounts update

Actualizar Cuenta Pix

```text
kobana banking charge pix-accounts update [UID] [OPCIONES]
```

`PUT /v2/charge/pix_accounts/{uid}`

Este endpoint se utiliza para actualizar una nueva cuenta pix.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `custom_name` | `string` | no | Nombre personalizado de la cuenta pix |
| `key` | `string` | no | Clave pix |
| `enabled` | `boolean` | no | Habilitar Cuenta Pix |
| `default` | `boolean` | no | Cuenta predeterminada |
| `financial_account_uid` | `string` | no | UID de la cuenta financiera (para asociar a una cuenta existente) |

Otros campos y objetos anidados: `kobana schema banking.charge.pix-accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta Pix actualizada |
| `404` | Cuenta Pix no encontrada |
| `422` | Clave Pix inválida |

## charge pix-accounts delete

Eliminar Cuenta Pix

```text
kobana banking charge pix-accounts delete [UID] [OPCIONES]
```

`DELETE /v2/charge/pix_accounts/{uid}`

Elimina una cuenta Pix.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Cuenta Pix eliminada |
| `404` | Cuenta Pix no encontrada |
