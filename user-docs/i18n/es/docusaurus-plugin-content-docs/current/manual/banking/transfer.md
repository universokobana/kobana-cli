---
id: transfer
title: "kobana banking transfer"
sidebar_label: "transfer"
description: "Transferencias (Pix, TED, internas)"
---

# kobana banking transfer

Transferencias (Pix, TED, internas)

Comandos de `kobana banking transfer`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`transfer accounts list`](#transfer-accounts-list) | `GET /v2/transfer/accounts` | Listar cuentas de transferencia |
| [`transfer accounts create`](#transfer-accounts-create) | `POST /v2/transfer/accounts` | Crear cuenta de transferencia |
| [`transfer accounts get`](#transfer-accounts-get) | `GET /v2/transfer/accounts/{uid}` | Buscar cuenta de transferencia |
| [`transfer accounts update`](#transfer-accounts-update) | `PATCH /v2/transfer/accounts/{uid}` | Actualizar cuenta de transferencia |
| [`transfer batches list`](#transfer-batches-list) | `GET /v2/transfer/batches` | Listar Lotes de Transferencia |
| [`transfer batches get`](#transfer-batches-get) | `GET /v2/transfer/batches/{uid}` | Visualizar un Lote de Transferencia |
| [`transfer batches approve`](#transfer-batches-approve) | `PUT /v2/transfer/batches/{uid}/approve` | Aprobar un Lote de Transferencia |
| [`transfer batches cancel`](#transfer-batches-cancel) | `PUT /v2/transfer/batches/{uid}/cancel` | Cancelar un Lote de Transferencia |
| [`transfer batches reprove`](#transfer-batches-reprove) | `PUT /v2/transfer/batches/{uid}/reprove` | Rechazar un Lote de Transferencia |
| [`transfer internal list`](#transfer-internal-list) | `GET /v2/transfer/internal` | Listar Transferencias Entre Cuentas |
| [`transfer internal create`](#transfer-internal-create) | `POST /v2/transfer/internal` | Crear una Transferencia Entre Cuentas |
| [`transfer internal get`](#transfer-internal-get) | `GET /v2/transfer/internal/{uid}` | Visualizar una Transferencia Entre Cuentas |
| [`transfer internal approve`](#transfer-internal-approve) | `PUT /v2/transfer/internal/{uid}/approve` | Liberar (aprobar) una Transferencia interna |
| [`transfer internal cancel`](#transfer-internal-cancel) | `PUT /v2/transfer/internal/{uid}/cancel` | Cancelar una Transferencia Entre Cuentas |
| [`transfer internal reschedule`](#transfer-internal-reschedule) | `PUT /v2/transfer/internal/{uid}/reschedule` | Reagendar (cambiar la fecha) una Transferencia interna |
| [`transfer internal-batches create`](#transfer-internal-batches-create) | `POST /v2/transfer/internal_batches` | Crear un Lote de Transferencia Entre Cuentas |
| [`transfer pix list`](#transfer-pix-list) | `GET /v2/transfer/pix` | Listar Transferencias Pix |
| [`transfer pix create`](#transfer-pix-create) | `POST /v2/transfer/pix` | Crear una Transferencia Pix |
| [`transfer pix get`](#transfer-pix-get) | `GET /v2/transfer/pix/{uid}` | Visualizar una Transferencia Pix |
| [`transfer pix approve`](#transfer-pix-approve) | `PUT /v2/transfer/pix/{uid}/approve` | Liberar (aprobar) una Transferencia Pix |
| [`transfer pix cancel`](#transfer-pix-cancel) | `PUT /v2/transfer/pix/{uid}/cancel` | Cancelar una Transferencia Pix |
| [`transfer pix reschedule`](#transfer-pix-reschedule) | `PUT /v2/transfer/pix/{uid}/reschedule` | Reagendar (cambiar la fecha) una Transferencia Pix |
| [`transfer pix-batches create`](#transfer-pix-batches-create) | `POST /v2/transfer/pix_batches` | Crear un Lote de Transferencia Pix |
| [`transfer ted list`](#transfer-ted-list) | `GET /v2/transfer/ted` | Listar transferencias TED |
| [`transfer ted create`](#transfer-ted-create) | `POST /v2/transfer/ted` | Crear uma transferencia TED |
| [`transfer ted get`](#transfer-ted-get) | `GET /v2/transfer/ted/{uid}` | Ver uma transferencia TED |
| [`transfer ted approve`](#transfer-ted-approve) | `PUT /v2/transfer/ted/{uid}/approve` | Liberar (Aprobar) uma transferencia TED |
| [`transfer ted cancel`](#transfer-ted-cancel) | `PUT /v2/transfer/ted/{uid}/cancel` | Cancelar uma transferencia TED |
| [`transfer ted reschedule`](#transfer-ted-reschedule) | `PUT /v2/transfer/ted/{uid}/reschedule` | Reagendar (mudar a data) uma transferencia TED |
| [`transfer ted-batches create`](#transfer-ted-batches-create) | `POST /v2/transfer/ted_batches` | Crear un Lote de Transferencia TED |

## transfer accounts list

Listar cuentas de transferencia

```text
kobana banking transfer accounts list [OPCIONES]
```

`GET /v2/transfer/accounts`

Lista todas las cuentas de transferencia registradas en la cuenta. Soporta paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `financial_account_uid` | `string` | no | Filtrar por UID de la cuenta financiera |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cuentas de transferencia acessadas con escopo read |

## transfer accounts create

Crear cuenta de transferencia

```text
kobana banking transfer accounts create [OPCIONES]
```

`POST /v2/transfer/accounts`

Cria uma nova cuenta de transferencia associada a uma cuenta financeira. Use quando a cuenta financeira precisa de mais de uma cuenta de transferencia. A cuenta é creada sin conexão bancária (`connection_uid` retorna `null`); associe uma conexão antes de Habilitar remessas/registro. Requer escopo `write` + `transfer.accounts`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string` | sí | UUID de la Cuenta Financiera que recibirá la nueva cuenta de transferencia |
| `agreement_code` | `string` | no | Código del convenio |
| `external_id` | `string` | no | Identificador externo de la cuenta de transferencia |
| `remittance_enabled` | `boolean` | no | Habilita o deshabilita la generación de remesas EDI para esta cuenta. |
| `provider_data` | `object` | no | Atributos específicos del proveedor bancario. |

Otros campos y objetos anidados: `kobana schema banking.transfer.accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | cuenta de transferencia creada |
| `404` | Cuenta Financiera no encontrada |
| `422` | Erro de validação. `agreement_code` acima del limite. |

## transfer accounts get

Buscar cuenta de transferencia

```text
kobana banking transfer accounts get [UID] [OPCIONES]
```

`GET /v2/transfer/accounts/{uid}`

Retorna los detalhes de uma cuenta de transferencia específica pelo UID

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cuenta de transferencia acessada con escopo read |
| `404` | cuenta de transferencia en lo encontrada |

## transfer accounts update

Actualizar cuenta de transferencia

```text
kobana banking transfer accounts update [UID] [OPCIONES]
```

`PATCH /v2/transfer/accounts/{uid}`

Atualiza atributos editáveis de uma cuenta de transferencia. Aceita `remittance_enabled` (booleano, controla a geração de remessas EDI), `provider_data.service_point_number` (string, ponto de atendimento — por exemplo `numeroPa` em TED Sicoob) y `provider_data.cooperative_cnpj` (string, CNPJ de la cooperativa Sicoob). O corpo aceita PATCH parcial: envie apenas los campos que deseja Actualizar.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `uid` | `string` | no | UUID de la cuenta de transferencia |
| `financial_account_uid` | `string` | no | UUID de la cuenta financiera asociada |
| `connection_uid` | `string (uuid)` | no | Conexión bancaria de la cuenta. |
| `agreement_code` | `string` | no | Código del convenio |
| `provider_data` | `object` | no | Datos específicos del proveedor. Para Sicoob, expone `service_point_number` y `cooperative_cnpj`. |
| `external_id` | `string` | no | Identificador externo de la cuenta de transferencia |
| `balance_validation_enabled_at` | `string (date-time)` | no | Fecha y hora en que la validación de saldo fue habilitada |
| `remittance_enabled_at` | `string (date-time)` | no | Fecha y hora en que la remesa EDI fue habilitada |
| `remittance_enabled` | `boolean` | no | Habilita o deshabilita la generación de remesas EDI para esta cuenta. |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.transfer.accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | PATCH parcial — apenas service_point_number (sin remittance_enabled) |
| `404` | cuenta de transferencia en lo encontrada |
| `422` | Erro de validação. `remittance_enabled` deve ser verdadeiro o falso. |

## transfer batches list

Listar Lotes de Transferencia

```text
kobana banking transfer batches list [OPCIONES]
```

`GET /v2/transfer/batches`

Listar Lotes de Transferencia.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de lotes |

## transfer batches get

Visualizar un Lote de Transferencia

```text
kobana banking transfer batches get [UID] [OPCIONES]
```

`GET /v2/transfer/batches/{uid}`

Consulta información específica de un Lote de Transferencia.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lote de Transferencia encontrado |
| `404` | Lote de Transferencia no encontrado |

## transfer batches approve

Aprobar un Lote de Transferencia

```text
kobana banking transfer batches approve [UID] [OPCIONES]
```

`PUT /v2/transfer/batches/{uid}/approve`

> **Procesamiento Asíncrono:** Este endpoint es procesado asincrónicamente y crea un comando de aprobación para el Lote de Transferencia.
  La aprobación de un Lote de Transferencia se realiza asincrónicamente, es decir, al recibir la solicitud, nuestra API guarda la solicitación y crea un Comando. Este comando es procesado en segundo plano y después de confirmado, el Lote tiene su estado alterado a Aprobado. Recibes en la respuesta de esta solicitud el ID del comando creado y puedes consultarlo en cualquier momento.

  ## Eventos de Webhook

  Cuando la solicitud se realiza, se dispara un evento `transfer.transfer_batch.approve.requested`.
  Nuestro servidor luego comienza los intentos de envío de la liberación a la institución financiera.
  Solo después de confirmado en la institución financiera, entonces el lote es marcado como aprobado en nuestra base de datos.
  En este momento se dispara el evento `transfer.transfer_batch.approve.confirmed`.

  > ⚠️ **Atención al estado Aprobado:** El estado de Aprobado no es terminal. Cuando la institución financiera confirme las transferencias, los mismos tendrán sus estados alterados a Confirmado

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `404` | Lote de Transferencia no encontrado |
| `422` | Lote de Transferencia no está esperando liberación |

## transfer batches cancel

Cancelar un Lote de Transferencia

```text
kobana banking transfer batches cancel [UID] [OPCIONES]
```

`PUT /v2/transfer/batches/{uid}/cancel`

> **Cancelación:** Este endpoint cancela el Lote de Transferencia. El comportamiento depende del estado de registro del lote.

  - **No registrado en el proveedor:** El lote es cancelado inmediatamente de forma local, sin interacción con la institución financiera. La respuesta contiene el lote actualizado.
  - **Registrado en el proveedor (API de cancelación soportada):** Un Comando de cancelación es creado y procesado asincrónicamente. La respuesta contiene el Comando creado.
  - **Registrado en el proveedor (API de cancelación no soportada):** Un Comando de cancelación es creado por Transferencia individualmente. La respuesta contiene el lote actualizado.

  La cancelación es posible en los estados: `pending`, `awaiting_approval`, `confirmed` y `approved`.

  > ⚠️ **Atención al estado Cancelado:** El estado de Cancelado es terminal.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cancelado con éxito |
| `422` | Lote de Transferencia no puede ser cancelado en el estado actual |

## transfer batches reprove

Rechazar un Lote de Transferencia

```text
kobana banking transfer batches reprove [UID] [OPCIONES]
```

`PUT /v2/transfer/batches/{uid}/reprove`

> **Procesamiento Asíncrono:** Este endpoint es procesado asincrónicamente y crea un comando de rechazo para el Lote de Transferencia.
  El rechazo de un Lote de Transferencia se realiza asincrónicamente, es decir, al recibir la solicitud, nuestra API guarda la solicitación y crea un Comando. Este comando es procesado en segundo plano y después de confirmado, el Lote tiene su estado alterado a Rechazado. Recibes en la respuesta de esta solicitud el ID del comando creado y puedes consultarlo en cualquier momento.

  ## Eventos de Webhook

  Cuando la solicitud se realiza, se dispara un evento `transfer.transfer_batch.reprove.requested`.
  Nuestro servidor luego comienza los intentos de envío de la liberación a la institución financiera.
  Solo después de confirmado en la institución financiera, entonces el lote es marcado como aprobado en nuestra base de datos.
  En este momento se dispara el evento `transfer.transfer_batch.reprove.confirmed`.

  > ⚠️ **Atención al estado Rechazado:** El estado de Rechazado es terminal. Cuando la institución financiera cancele las transferencias, los mismos tendrán sus estados alterados a Rechazado

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `404` | Lote de Transferencia no encontrado |
| `422` | Lote de Transferencia no está esperando liberación |

## transfer internal list

Listar Transferencias Entre Cuentas

```text
kobana banking transfer internal list [OPCIONES]
```

`GET /v2/transfer/internal`

Listar Transferencias Entre Cuentas.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la listación de las transferencias |

## transfer internal create

Crear una Transferencia Entre Cuentas

```text
kobana banking transfer internal create [OPCIONES]
```

`POST /v2/transfer/internal`

Este endpoint se utiliza para la generación de una transferencia Entre Cuentas.

**Requisito Previo**

Para emitir una transferencia es necesario tener una Cuenta Financiera, informada en `financial_account_uid`. La cuenta de transferencia que registra la operación se define en `transfer_account_uid`; si se omite, se utiliza la más antigua vinculada a la cuenta financiera; si se informa, debe pertenecer a `financial_account_uid`.

**Eventos de Webhook**

El evento `transfer.db.created` se dispara en esta operación.

> **Próximos pasos:** Después de crear una transferencia, querrás en algún momento enviar un Lote a la institución financiera. Para esto, utiliza el endpoint `POST /api/v2/transfer/internal_batches`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Cantidad |
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `beneficiary` | `object` | sí | Datos del Receptor |
| `internal` | `object` | sí | Datos bancarios |
| `scheduled_to` | `string (date)` | no | Fecha de Programación |
| `transfer_purpose` | `string` | no | Objetivo de la Transferencia |
| `transfer_account_uid` | `string (uuid)` | no | Cuenta de transferencia de origen (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se informa, debe pertenecer a `financial_account_uid`. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |

Otros campos y objetos anidados: `kobana schema banking.transfer.internal.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Transferencia Entre Cuentas creada |
| `422` | Error de validación |

## transfer internal get

Visualizar una Transferencia Entre Cuentas

```text
kobana banking transfer internal get [UID] [OPCIONES]
```

`GET /v2/transfer/internal/{uid}`

Consulta información específica de una Transferencia Entre Cuentas.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Transferencia Entre Cuentas encontrada |
| `404` | Transferencia Entre Cuentas no encontrada |

## transfer internal approve

Liberar (aprobar) una Transferencia interna

```text
kobana banking transfer internal approve [UID] [OPCIONES]
```

`PUT /v2/transfer/internal/{uid}/approve`

> **Procesamiento Asíncrono:** crea un comando de liberación (approve) para la Transferencia esperando aprobación.

> ⚠️ **Soporte por banco:** ningún proveedor financiero soporta liberación de transferencia interna vía API en este momento — la llamada responde `422` ("no posee soporte a esta operación vía API para este proveedor financiero"). El endpoint existe para simetría y habilita el soporte automáticamente cuando algún proveedor lo implemente.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `422` | Proveedor sin soporte a liberación vía API |

## transfer internal cancel

Cancelar una Transferencia Entre Cuentas

```text
kobana banking transfer internal cancel [UID] [OPCIONES]
```

`PUT /v2/transfer/internal/{uid}/cancel`

> **Procesamiento Asíncrono:** Este endpoint es procesado asincrónicamente y crea un comando de cancelación para la Transferencia.
El cancelamiento de una Transferencia se realiza asincrónicamente, es decir, al recibir la solicitud, nuestra API guarda la solicitación y crea un Comando. Este comando es procesado en segundo plano y después de confirmado, la Transferencia tiene su estado alterado a Cancelado. Recibes en la respuesta de esta solicitud el ID del comando creado y puedes consultarlo en cualquier momento.

**Eventos de Webhook**

Cuando la solicitud se realiza, se dispara un evento `transfer.internal.cancel.requested`.
Nuestro servidor luego comienza los intentos de envío del cancelamiento a la institución financiera.
Solo después de confirmado en la institución financiera, entonces la transferencia es marcada como cancelada en nuestra base de datos.
En este momento se dispara el evento `transfer.internal.cancel.confirmed`.

> ⚠️ **Atención al estado Cancelado:** El estado de Cancelado es terminal. La transferencia no podrá ser procesada más después del cancelamiento.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `422` | La transferencia ya fue confirmada, rechazada o cancelada |

## transfer internal reschedule

Reagendar (cambiar la fecha) una Transferencia interna

```text
kobana banking transfer internal reschedule [UID] [OPCIONES]
```

`PUT /v2/transfer/internal/{uid}/reschedule`

> **Procesamiento Asíncrono:** crea un comando de reagendamiento (cambio de la fecha de pago) para la Transferencia.

> ⚠️ **Soporte por banco:** ningún proveedor financiero soporta reagendamiento de transferencia interna vía API en este momento — la llamada responde `422` ("no posee soporte a esta operación vía API para este proveedor financiero"). El endpoint existe para simetría y habilita el soporte automáticamente cuando algún proveedor lo implemente.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `date` | `string (date)` | sí | Nueva fecha de pago (YYYY-MM-DD). |

Otros campos y objetos anidados: `kobana schema banking.transfer.internal.reschedule`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `422` | Proveedor sin soporte al reagendamiento vía API |

## transfer internal-batches create

Crear un Lote de Transferencia Entre Cuentas

```text
kobana banking transfer internal-batches create [OPCIONES]
```

`POST /v2/transfer/internal_batches`

Este endpoint se utiliza para la generación de un lote de transferencia Entre Cuentas.

**Requisito Previo**

Para emitir un lote de transferencia es necesario tener una Cuenta Financiera, informada en `financial_account_uid`. La cuenta de transferencia aplicada al lote se define en `transfer_account_uid`; si se omite, se utiliza la más antigua vinculada a la cuenta financiera; si se informa, debe pertenecer a `financial_account_uid`.

**Eventos de Webhook**

El evento `transfer.batch.db.created` se dispara en esta operación.

Al registrar un Lote de Transferencia, el sistema crea un comando para registrarlo en la institución financiera. Después de que el registro del lote sea confirmado en la institución, el evento `transfer.internal_batch.confirmed` se dispara.

El evento `transfer.db.created` también se dispara en esta operación si el lote contiene una nueva transferencia.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `transfers` | `array<anyOf>` | sí | Lista de Transferencias Entre Cuentas nuevas o existentes. |
| `transfer_account_uid` | `string (uuid)` | no | Cuenta de transferencia aplicada a todo el lote (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se informa, debe pertenecer a la cuenta financiera del lote. |

Otros campos y objetos anidados: `kobana schema banking.transfer.internal-batches.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Lote de Transferencia Entre Cuentas creado |
| `422` | Error de validación — external_id duplicado |

## transfer pix list

Listar Transferencias Pix

```text
kobana banking transfer pix list [OPCIONES]
```

`GET /v2/transfer/pix`

Listar Transferencias Pix.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la listación de las transferencias |

## transfer pix create

Crear una Transferencia Pix

```text
kobana banking transfer pix create [OPCIONES]
```

`POST /v2/transfer/pix`

Este endpoint se utiliza para la generación de una transferencia Pix.

**Requisito Previo**

Para emitir una transferencia es necesario tener una Cuenta Financiera, informada en `financial_account_uid`. La cuenta de transferencia que registra la operación se define en `transfer_account_uid`; si se omite, se utiliza la más antigua vinculada a la cuenta financiera; si se informa, debe pertenecer a `financial_account_uid`.

**Eventos de Webhook**

El evento `transfer.db.created` se dispara en esta operación.

> **Próximos pasos:** Después de crear una transferencia, querrás en algún momento enviar un Lote a la institución financiera. Para esto, utiliza el endpoint `POST /api/v2/transfer/pix_batches`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Cantidad |
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `beneficiary` | `object` | sí | Datos del Receptor |
| `type` | `string` | sí | Tipo del Pix `key`: Clave. `bank_account`: Datos bancarios |
| `scheduled_to` | `string (date)` | no | Fecha de Programación |
| `transfer_purpose` | `string` | no | Objetivo de la Transferencia |
| `transfer_account_uid` | `string (uuid)` | no | Cuenta de transferencia de origen (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se informa, debe pertenecer a `financial_account_uid`. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |
| `key_type` | `string` | no | Tipo de la Clave (Obligatorio si el campo `type` es `key`) `cpf`: CPF. `cnpj`: CNPJ. `random`: Aleatoria. `email`: Correo. `phone`: Celular |
| `key` | `string` | no | Clave Pix (Obligatorio si el campo `type` es `key`) |
| `bank_account` | `object` | no | Datos bancarios (Obligatorio si el campo `type` es `bank_account`) |
| `identifier` | `string` | no | Identificación del pago en el comprobante del proveedor financiero. (Uso exclusivo del banco) Bancos que aceptan este campo (vía API): Itaú |

Otros campos y objetos anidados: `kobana schema banking.transfer.pix.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Transferencia Pix creada |
| `422` | Error de validación |

## transfer pix get

Visualizar una Transferencia Pix

```text
kobana banking transfer pix get [UID] [OPCIONES]
```

`GET /v2/transfer/pix/{uid}`

Consulta información específica de una Transferencia Pix.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Transferencia Pix encontrada |
| `404` | Transferencia Pix no encontrada |

## transfer pix approve

Liberar (aprobar) una Transferencia Pix

```text
kobana banking transfer pix approve [UID] [OPCIONES]
```

`PUT /v2/transfer/pix/{uid}/approve`

> **Procesamiento Asíncrono:** crea un comando de liberación (approve) para la Transferencia Pix esperando aprobación.

> ⚠️ **Soporte por banco:** no todo proveedor financiero soporta liberación vía API. Cuando no se soporta, la API responde `422` ("no posee soporte a esta operación vía API para este proveedor financiero").

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `422` | Transferencia no puede ser liberada |

## transfer pix cancel

Cancelar una Transferencia Pix

```text
kobana banking transfer pix cancel [UID] [OPCIONES]
```

`PUT /v2/transfer/pix/{uid}/cancel`

> **Procesamiento Asíncrono:** Este endpoint es procesado asincrónicamente y crea un comando de cancelación para la Transferencia.
El cancelamiento de una Transferencia se realiza asincrónicamente, es decir, al recibir la solicitud, nuestra API guarda la solicitación y crea un Comando. Este comando es procesado en segundo plano y después de confirmado, la Transferencia tiene su estado alterado a Cancelado. Recibes en la respuesta de esta solicitud el ID del comando creado y puedes consultarlo en cualquier momento.

**Eventos de Webhook**

Cuando la solicitud se realiza, se dispara un evento `transfer.pix.cancel.requested`.
Nuestro servidor luego comienza los intentos de envío del cancelamiento a la institución financiera.
Solo después de confirmado en la institución financiera, entonces la transferencia es marcada como cancelada en nuestra base de datos.
En este momento se dispara el evento `transfer.pix.cancel.confirmed`.

> ⚠️ **Atención al estado Cancelado:** El estado de Cancelado es terminal. La transferencia no podrá ser procesada más después del cancelamiento.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `422` | La transferencia ya fue confirmada, rechazada o cancelada |

## transfer pix reschedule

Reagendar (cambiar la fecha) una Transferencia Pix

```text
kobana banking transfer pix reschedule [UID] [OPCIONES]
```

`PUT /v2/transfer/pix/{uid}/reschedule`

> **Procesamiento Asíncrono:** crea un comando de reagendamiento (cambio de la fecha de pago) para la Transferencia Pix.

> ⚠️ **Soporte por banco:** no todo proveedor financiero soporta reagendamiento vía API. Cuando no se soporta, la API responde `422` ("no posee soporte a esta operación vía API para este proveedor financiero").

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `date` | `string (date)` | sí | Nueva fecha de pago (YYYY-MM-DD). |

Otros campos y objetos anidados: `kobana schema banking.transfer.pix.reschedule`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `422` | Transferencia no puede ser reagendada |

## transfer pix-batches create

Crear un Lote de Transferencia Pix

```text
kobana banking transfer pix-batches create [OPCIONES]
```

`POST /v2/transfer/pix_batches`

Este endpoint se utiliza para la generación de un lote de transferencia pix.

**Requisito Previo**

Para emitir un lote de transferencia es necesario tener una Cuenta Financiera, informada en `financial_account_uid`. La cuenta de transferencia aplicada al lote se define en `transfer_account_uid`; si se omite, se utiliza la más antigua vinculada a la cuenta financiera; si se informa, debe pertenecer a `financial_account_uid`.

**Eventos de Webhook**

El evento `transfer.batch.db.created` se dispara en esta operación.

Al registrar un Lote de Transferencia, el sistema crea un comando para registrarlo en la institución financiera. Después de que el registro del lote sea confirmado en la institución, el evento `transfer.pix_batch.confirmed` se dispara.

El evento `transfer.db.created` también se dispara en esta operación si el lote contiene una nueva transferencia.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `transfers` | `array<anyOf>` | sí | Lista de Transferencias Pix nuevas o existentes. |
| `transfer_account_uid` | `string (uuid)` | no | Cuenta de transferencia aplicada a todo el lote (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se informa, debe pertenecer a la cuenta financiera del lote. |

Otros campos y objetos anidados: `kobana schema banking.transfer.pix-batches.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Lote de Transferencia Pix creado |
| `422` | Error de validación — external_id duplicado |

## transfer ted list

Listar transferencias TED

```text
kobana banking transfer ted list [OPCIONES]
```

`GET /v2/transfer/ted`

Listar transferencias TED.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la listación de las transferencias |

## transfer ted create

Crear uma transferencia TED

```text
kobana banking transfer ted create [OPCIONES]
```

`POST /v2/transfer/ted`

Este endpoint se utiliza para la generación de una transferencia TED.

**Requisito previo**

Para emitir uma transferencia é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de transferencia que registra a operação é definida em `transfer_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Eventos de Webhook**

O evento `transfer.db.created` é disparado nesta operação.

> **Próximos passos:** Após Crear uma transferencia, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/transfer/ted_batches`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Cantidad |
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `scheduled_to` | `string (date)` | no | Fecha de Programación |
| `transfer_purpose` | `string` | no | Objetivo de la Transferencia |
| `transfer_account_uid` | `string (uuid)` | no | Cuenta de transferencia de origen (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se informa, debe pertenecer a `financial_account_uid`. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |
| `beneficiary` | `object` | no | Datos del Receptor |
| `bank_account` | `object` | no | Datos bancarios |
| `judicial_deposit_number` | `string` | no | Número del depósito judicial (TED de depósito judicial, exclusivo de Banco do Brasil — mutuamente excluyente con `bank_account`) |
| `judicial_deposit_bank_code` | `string` | no | Código COMPE del banco de destino del depósito judicial (obligatorio con `judicial_deposit_number`). Use `001` para depósito en el propio Banco do Brasil |

Otros campos y objetos anidados: `kobana schema banking.transfer.ted.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | transferencia TED creada |
| `422` | Error de validación |

## transfer ted get

Ver uma transferencia TED

```text
kobana banking transfer ted get [UID] [OPCIONES]
```

`GET /v2/transfer/ted/{uid}`

Consulta información específicas de uma transferencia TED.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | transferencia TED encontrada |
| `404` | transferencia TED en lo encontrada |

## transfer ted approve

Liberar (Aprobar) uma transferencia TED

```text
kobana banking transfer ted approve [UID] [OPCIONES]
```

`PUT /v2/transfer/ted/{uid}/approve`

> **Procesamiento Asincrónico:** cria um comando de liberação (approve) para a transferencia aguardando aprovação.

A liberação é processada assincronamente: a API guarda a solicitação, cria um Comando y responde con o ID dele. Após confirmado en la instituição financeira, a transferencia é liberada.

> ⚠️ **Suporte por banco:** nem todo proveedor financeiro Soporta liberação via API. Quando não suportado, a API responde `422` con a mensagem de "en lo tiene suporte a esta operação via API para este proveedor financeiro".

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `422` | Transferencia no puede ser liberada |

## transfer ted cancel

Cancelar uma transferencia TED

```text
kobana banking transfer ted cancel [UID] [OPCIONES]
```

`PUT /v2/transfer/ted/{uid}/cancel`

> **Procesamiento Asincrónico:** Este endpoint é processado assincronamente y cria um comando de cancelamento para a transferencia.
O cancelamento de uma transferencia é feito assincronamente, o seja, ao receber a requisição, a nossa API guarda a solicitação y cria um Comando. Esse comando é processado em segundo plano y depois de confirmado, a transferencia tem seu status alterado para Cancelado. Você recebe en la resposta desta requisição o ID del comando creado y pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `transfer.ted.cancel.requested` é disparado.
O nosso servidor então começa las tentativas de envio del cancelamento para a instituição financeira.
Somente depois de confirmado en la instituição financeira, aí então a transferencia é marcada como cancelada em nosso banco de dados.
Neste momento o evento `transfer.ted.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. A transferencia en lo podrá más ser processada após o cancelamento.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `422` | La transferencia ya fue confirmada, rechazada o cancelada |

## transfer ted reschedule

Reagendar (mudar a data) uma transferencia TED

```text
kobana banking transfer ted reschedule [UID] [OPCIONES]
```

`PUT /v2/transfer/ted/{uid}/reschedule`

> **Procesamiento Asincrónico:** cria um comando de reagendamento (mudança de la data de pago) para a transferencia.

> ⚠️ **Suporte por banco:** nem todo proveedor financeiro Soporta reagendamento via API. Quando não suportado, a API responde `422` ("en lo tiene suporte a esta operação via API para este proveedor financeiro").

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `date` | `string (date)` | sí | Nueva fecha de pago (YYYY-MM-DD). |

Otros campos y objetos anidados: `kobana schema banking.transfer.ted.reschedule`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `422` | Transferencia no puede ser reagendada |

## transfer ted-batches create

Crear un Lote de Transferencia TED

```text
kobana banking transfer ted-batches create [OPCIONES]
```

`POST /v2/transfer/ted_batches`

Este endpoint se utiliza para la generación de un lote de transferencia TED.

**Requisito previo**

Para emitir um lote de transferencia é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de transferencia aplicada ao lote é definida em `transfer_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Eventos de Webhook**

O evento `transfer.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de transferencia, o sistema cria um comando para registrá-lo en la instituição financeira. Após o registro del lote ser confirmado en la instituição, o evento `transfer.ted_batch.confirmed` é disparado.

O evento `transfer.db.created` também é disparado nesta operação caso o lote contenha uma nova transferencia.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `transfers` | `array<anyOf>` | sí | Lista de Transferencias TED nuevas o existentes. |
| `transfer_account_uid` | `string (uuid)` | no | Cuenta de transferencia aplicada a todo el lote (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se informa, debe pertenecer a la cuenta financiera del lote. |

Otros campos y objetos anidados: `kobana schema banking.transfer.ted-batches.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Lote de transferencia TED creado |
| `422` | Error de validación — external_id duplicado |
