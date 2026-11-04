---
id: data
title: "kobana banking data"
sidebar_label: "data"
description: "Consultas (boletos, códigos QR Pix)"
---

# kobana banking data

Consultas (boletos, códigos QR Pix)

Comandos de `kobana banking data`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`data bank-billet-queries list`](#data-bank-billet-queries-list) | `GET /v2/data/bank_billet_queries` | Listar Consultas de Boletos |
| [`data bank-billet-queries create`](#data-bank-billet-queries-create) | `POST /v2/data/bank_billet_queries` | Crear una Consulta de Boleto |
| [`data people list`](#data-people-list) | `GET /v2/data/people` | Buscar Personas |
| [`data people create`](#data-people-create) | `POST /v2/data/people` | Crear una Persona |
| [`data people get`](#data-people-get) | `GET /v2/data/people/{uid}` | Ver una Persona |
| [`data people delete`](#data-people-delete) | `DELETE /v2/data/people/{uid}` | Eliminar una Persona |
| [`data pix-qrcode-queries list`](#data-pix-qrcode-queries-list) | `GET /v2/data/pix_qrcode_queries` | Listar Consultas de Código QR de Pix |
| [`data pix-qrcode-queries create`](#data-pix-qrcode-queries-create) | `POST /v2/data/pix_qrcode_queries` | Crear una Consulta de Código QR de Pix |

## data bank-billet-queries list

Listar Consultas de Boletos

```text
kobana banking data bank-billet-queries list [OPCIONES]
```

`GET /v2/data/bank_billet_queries`

Lista todas las consultas de boletos realizadas por la cuenta autenticada. Permite filtrar por estado, etiquetas, fechas de creación y otros parámetros.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en el listado de consultas de boletos |

## data bank-billet-queries create

Crear una Consulta de Boleto

```text
kobana banking data bank-billet-queries create [OPCIONES]
```

`POST /v2/data/bank_billet_queries`

Crea una nueva consulta de boleto bancario a través de la línea digitada o código de barras. La consulta se procesa automáticamente y devuelve información detallada del boleto.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `uid` | `string` | no | ID único del registro en formato UUID |
| `status` | `string` | no | Situación de la consulta: * `pending` Pendiente (solo cuando se configura para no procesar automáticamente) * `success` Realizado con éxito * `error` Falla en la consulta Valores: `pending`, `success`, `error`. |
| `error_code` | `string` | no | Código de error cuando `status` es `error`: * `bank_billet_unavailable` Boleto no disponible: puede haber sido pagado, cancelado o dado de baja * `document_not_supported` El proveedor no confirmó el documento para pago: el convenio o la modalidad puede no estar soportado * `provider_error` Fallo del proveedor al consultar el boleto * `unavailable_environment` Consulta disponible solo en producción Valores: `null`, `bank_billet_unavailable`, `document_not_supported`, `provider_error`, `unavailable_environment`. |
| `error_message` | `string` | no | Mensaje de error cuando la consulta se procesa pero los datos no se devuelven. |
| `line_or_barcode` | `string` | no | Línea Verificable o Código de Barras, en los formatos: Línea Verificable: 34191.79001 01043.510047 91020.150008 7 75870000001000 Código de Barras: 34197758700000010001790010104351004791020150 |
| `line` | `string` | no | Línea Digitalizada |
| `barcode` | `string` | no | Código de Barras |
| `expire_at` | `string (date)` | no | Fecha de Vencimiento |
| `pay_due_date` | `string (date)` | no | Fecha límite para pago (plazo de tolerancia), cuando se proporciona por el origen |
| `amount` | `number` | no | Valor a pagar |
| `minimum_amount` | `number` | no | Valor mínimo |
| `maximum_amount` | `number` | no | Valor máximo |
| `original_amount` | `number` | no | Valor original |
| `valid_for_payment` | `boolean` | no | Indica si el boleto está disponible para pago en el momento de la consulta |
| `payment_status_reason` | `string` | no | Motivo retornado cuando el boleto no está disponible para pago |
| `allow_change_value` | `boolean` | no | Indica si el valor del pago puede ser alterado dentro de los límites retornados |
| `allow_partial_payment` | `boolean` | no | Indica si el origen permite pagos parciales |
| `issue_bank` | `object` | no | Banco emisor del boleto |
| `beneficiary` | `object` | no | Datos del Beneficiario |
| `guarantor` | `object` | no | Datos del Beneficiario Final |
| `guarantor_drawer` | `object` | no | Datos del Girador Avalista |
| `payer` | `object` | no | Datos del Pagador |
| `fine` | `object` | no | Multa |
| `interest` | `object` | no | Intereses |
| `discount` | `object` | no | Descuento |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |
| `created_via_api` | `boolean` | no | Indica si el registro fue creado por API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.data.bank-billet-queries.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Consulta de Boleto creada |
| `422` | Consulta de Boleto inválida |

## data people list

Buscar Personas

```text
kobana banking data people list [OPCIONES]
```

`GET /v2/data/people`

Lista todas las personas registradas en la cuenta autenticada. Permite filtrar por código externo, tipo de persona, documento, correo electrónico, etiquetas y período de creación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `external_code` | `string` | no | Filtro por Código Externo. Encuentra todos los registros cuyo campo sea igual al string. |
| `kind` | `string` | no | Filtro por Tipo de Persona (Natural o Jurídica). |
| `document_type` | `string` | no | Filtro por Tipo de Documento (CPF o CNPJ). |
| `document_number` | `string` | no | Filtro por Número de Documento. Encuentra todos los registros cuyo campo sea igual al string. |
| `tags` | `array` | no | Etiquetas asociadas al registro. |
| `created_from` | `string` | no | Fecha inicial del filtro de fecha de creación del registro. |
| `created_to` | `string` | no | Fecha final del filtro de fecha de creación del registro. |
| `per_page` | `integer` | no | Cantidad de registros por página. Máximo de 50. |
| `page` | `integer` | no | Número de página. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registros encontrados (página 2 con prev_url) |

## data people create

Crear una Persona

```text
kobana banking data people create [OPCIONES]
```

`POST /v2/data/people`

Crea una nueva persona (física o jurídica) con la información proporcionada. La persona quedará asociada a la cuenta autenticada.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre Completo o Razón Social |
| `uid` | `string (uuid)` | no | Identificador único del registro |
| `nickname` | `string` | no | Apodo o Nombre Fantasia |
| `kind` | `string` | no | Tipo de Persona |
| `external_code` | `string` | no | Código o ID en tu sistema de gestión. |
| `birthday` | `string (date)` | no | Fecha de Nacimiento. |
| `document_type` | `string` | no | Tipo del Documento. Valores válidos: CPF, CNPJ |
| `document_number` | `string` | no | Número del Documento. |
| `addresses` | `array<object>` | no | Lista de Direcciones |
| `phones` | `array<object>` | no | Lista de Teléfonos |
| `emails` | `array<object>` | no | Lista de Correos Electrónicos |
| `tags` | `array<string>` | no | Etiquetas asociadas al registro |
| `tag_list` | `string` | no | Lista de etiquetas separadas por coma |
| `custom_data` | `object` | no | Disponible para envío de un JSON con una estructura de datos libre |
| `notes` | `string` | no | Observaciones. Campo de texto libre |
| `created_via_api` | `boolean` | no | Indica si el registro fue creado por la API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro en la base de datos |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro en la base de datos |

Otros campos y objetos anidados: `kobana schema banking.data.people.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Persona creada |
| `422` | Error de validación |

## data people get

Ver una Persona

```text
kobana banking data people get [UID] [OPCIONES]
```

`GET /v2/data/people/{uid}`

Devuelve información detallada de una persona específica, incluidos datos personales, direcciones, teléfonos, correos electrónicos y datos personalizados.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Persona encontrada |
| `404` | Persona no encontrada |

## data people delete

Eliminar una Persona

```text
kobana banking data people delete [UID] [OPCIONES]
```

`DELETE /v2/data/people/{uid}`

Elimina permanentemente una persona del sistema. Esta acción no se puede deshacer.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Registro eliminado |
| `404` | Registro no encontrado |

## data pix-qrcode-queries list

Listar Consultas de Código QR de Pix

```text
kobana banking data pix-qrcode-queries list [OPCIONES]
```

`GET /v2/data/pix_qrcode_queries`

Lista todas las consultas de Código QR de Pix realizadas por la cuenta autenticada. Permite filtrar por etiquetas, external_id y otros parámetros.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `external_id` | `string` | no | Filtrar por ID externo |
| `tags` | `array` | no | Filtrar por etiquetas |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en el listado de consultas de Código QR de Pix |

## data pix-qrcode-queries create

Crear una Consulta de Código QR de Pix

```text
kobana banking data pix-qrcode-queries create [OPCIONES]
```

`POST /v2/data/pix_qrcode_queries`

Crea una nueva consulta de Código QR de Pix a través del qrcode. La consulta se procesa automáticamente y devuelve información detallada del Código QR de Pix.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `uid` | `string` | no | ID único del registro en formato UUID |
| `qrcode` | `string` | no | Código QR Pix para descodificación |
| `pix_key` | `string` | no | Clave Pix del beneficiario |
| `expires_at` | `string (date-time)` | no | Fecha de vencimiento del Código QR |
| `finality` | `string` | no | Propósito del pago (pago, retiro, cambio) |
| `amount` | `number` | no | Valor principal |
| `amount_change` | `number` | no | Valor del cambio |
| `interest` | `number` | no | Valor de los intereses |
| `fine` | `number` | no | Valor de la multa |
| `discount` | `number` | no | Valor del descuento |
| `total_amount` | `number` | no | Valor total a pagar |
| `beneficiary` | `object` | no | Datos del Beneficiario/Receptor |
| `debtor` | `object` | no | Datos del Deudor |
| `description` | `string` | no | Mensaje al pagador |
| `can_be_paid` | `boolean` | no | Indica si el Código QR puede ser pagado |
| `can_be_paid_with_different_value` | `boolean` | no | Indica si puede ser pagado con un valor diferente |
| `cannot_be_paid_reason` | `string` | no | Motivo por el cual no puede ser pagado |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas para categorización |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.data.pix-qrcode-queries.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Consulta de Código QR de Pix creada |
| `422` | Consulta de Código QR de Pix inválida |
