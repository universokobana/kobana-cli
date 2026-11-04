---
id: bank-billet-accounts
title: "kobana banking bank-billet-accounts"
sidebar_label: "bank-billet-accounts"
---

# kobana banking bank-billet-accounts


Comandos de `kobana banking bank-billet-accounts`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`bank-billet-accounts list`](#bank-billet-accounts-list) | `GET /v1/bank_billet_accounts` | Listar Carteras |
| [`bank-billet-accounts create`](#bank-billet-accounts-create) | `POST /v1/bank_billet_accounts` | Crear Cartera de Cobro |
| [`bank-billet-accounts get`](#bank-billet-accounts-get) | `GET /v1/bank_billet_accounts/{id}` | Información de la Cartera |
| [`bank-billet-accounts update`](#bank-billet-accounts-update) | `PUT /v1/bank_billet_accounts/{id}` | Actualizar la Cartera |
| [`bank-billet-accounts delete`](#bank-billet-accounts-delete) | `DELETE /v1/bank_billet_accounts/{id}` | Eliminar la Cartera de Cobro |
| [`bank-billet-accounts ask`](#bank-billet-accounts-ask) | `GET /v1/bank_billet_accounts/{id}/ask` | Solicitar Homologación de la Cartera de Cobro |
| [`bank-billet-accounts set-default`](#bank-billet-accounts-set-default) | `PUT /v1/bank_billet_accounts/{id}/set_default` | Cambiar Cartera de Cobro predeterminada |
| [`bank-billet-accounts validate`](#bank-billet-accounts-validate) | `PUT /v1/bank_billet_accounts/{id}/validate` | Validar Cartera de Cobro |

## bank-billet-accounts list

Listar Carteras

```text
kobana banking bank-billet-accounts list [OPCIONES]
```

`GET /v1/bank_billet_accounts`

Listar Carteras.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Carteras Encontradas |

## bank-billet-accounts create

Crear Cartera de Cobro

```text
kobana banking bank-billet-accounts create [OPCIONES]
```

`POST /v1/bank_billet_accounts`

Crea una nueva cartera de cobro para la emisión de boletos bancarios. La cartera debe ser homologada antes de poder ser utilizada en producción.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `bank_contract_slug` | `string` | sí | Slug de la Cartera |
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera. Cuando se proporciona, los datos de agencia y cuenta se completarán automáticamente desde la cuenta financiera. |
| `beneficiary_name` | `string` | sí | Nombre del Beneficiario |
| `beneficiary_cnpj_cpf` | `string` | sí | CNPJ/CPF del Beneficiario |
| `beneficiary_address_street` | `string` | sí | Calle del Beneficiario |
| `beneficiary_address_street_number` | `string` | sí | Número de la calle del Beneficiario |
| `beneficiary_address_complement` | `string` | sí | Complemento de la dirección del Beneficiario |
| `beneficiary_address_neighborhood` | `string` | sí | Barrio del Beneficiario |
| `beneficiary_address_city` | `string` | sí | Ciudad del Beneficiario |
| `beneficiary_address_state` | `string` | sí | Estado del Beneficiario |
| `beneficiary_address_zipcode` | `string` | sí | CEP del Beneficiario (formato 99999999) |
| `beneficiary_address` | `string` | sí | Dirección completa del Beneficiario. |
| `id` | `integer` | no | ID de la cartera |
| `next_our_number` | `integer` | no | Siguiente Nuestro Número. Predeterminado: 1 |
| `next_remittance_number` | `integer` | no | Último secuencial de la remesa Predeterminado: 1 |
| `agency_number` | `string` | no | Agencia. Completado automáticamente cuando se proporciona financial_account_uid. |
| `agency_digit` | `string` | no | Dígito de la Agencia. Se completa automáticamente cuando se proporciona financial_account_uid. |
| `account_number` | `string` | no | Cuenta. Completada automáticamente cuando se proporciona financial_account_uid. |
| `account_digit` | `string` | no | Dígito de la Cuenta. Completado automáticamente cuando se proporciona financial_account_uid. |
| `extra1` | `string` | no | Campo extra 1. Dependiendo de la cartera elegida podría ser obligatorio. |
| `extra1_digit` | `string` | no | Dígito del Campo extra 1. Dependiendo de la cartera elegida podría ser obligatorio. |
| `extra2` | `string` | no | Campo extra 2. Dependiendo de la cartera elegida podría ser obligatorio. |
| `extra2_digit` | `string` | no | Dígito del Campo extra 2. Dependiendo de la cartera elegida podría ser obligatorio. |
| `extra3` | `string` | no | Código de Remesa |
| `payment_place` | `string` | no | Lugar de Pago |
| `contract` | `string` | no | Contrato |
| `contract_type` | `string` | no | Tipo del Contrato |
| `financial_account` | `object` | no | Cuenta Financiera asociada a la cartera |
| `bolepix_enabled_at` | `string` | no | Fecha y Hora en que el boleto híbrido fue habilitado. |
| `bolepix` | `object` | no | Configuración de Boleto Híbrido |
| `name` | `string` | no | Nombre de la Cuenta. No se recibe en la creación ni en la actualización, solo se devuelve en la consulta y listado |
| `status` | `string` | no | Situación de la cartera: Cuando la cartera acaba de ser registrada, obtiene el estado `pending`. En este momento el usuario debe aceptar los términos e iniciar la homologación. Al hacer clic en "Proceder con la Homologación", el estado pasa a `homologating`. Se generará un boleto y una remesa de Homologación para ser enviada al banco. Después de completarse, el estado pasa a `validating`. A partir de este momento el usuario debe informar el valor del boleto que fue generado o enviar el retorno procesado por el banco. Cuando el valor es informado correctamente, o en el retorno procesado, el banco dice que todo está bien, el estado pasa a `active`. * `pending` Homologación no iniciada * `homologating` En homologación, aguardando pago del boleto * `validating` Boleto pagado, aguardando validación * `active` Activa y lista para usar Valores: `pending`, `homologating`, `validating`, `active`. |
| `default` | `boolean` | no | Predeterminado. El campo `default` determina la cartera de cobranza que se utilizará en la creación de boleto cuando no se informe ninguna cartera. En el momento en que la primera cartera es homologada (pasa al `status` = `active`), recibe el valor `default` = `true` |
| `configuration` | `object` | no | Configuración de datos predeterminados para boleto |
| `bank_contract` | `object` | no | Datos de la Cartera. No se recibe en la creación ni en la actualización, solo se devuelve en la consulta y listado. |
| `custom_name` | `string` | no | Nombre de la Cartera para identificación dentro de Kobana. |
| `kind` | `string` | no | Tipo de CNAB: * `cnab400` CNAB 400 * `cnab240` CNAB 240 Valores: `cnab400`, `cnab240`. |
| `remittance_disabled` | `boolean` | no | Desabilitar generación de remesas. |
| `remittance_disabled_at` | `string (date-time)` | no | Fecha y hora en que se deshabilitó la generación de remesa. |
| `allow_expiration_on_weekends` | `boolean` | no | Permitir vencimiento en fin de semana y feriados |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación de la cartera |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización de la cartera |
| `created_via_api` | `boolean` | no | Indica si la cartera fue creada por API |

Otros campos y objetos anidados: `kobana schema banking.bank-billet-accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cartera de cobro creada con financial_account_uid |
| `422` | Cartera inválida - Beneficiario CPF |

## bank-billet-accounts get

Información de la Cartera

```text
kobana banking bank-billet-accounts get [ID] [OPCIONES]
```

`GET /v1/bank_billet_accounts/{id}`

Consulta información específica de una Cartera

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cartera encontrada |
| `404` | Cartera no encontrada |

## bank-billet-accounts update

Actualizar la Cartera

```text
kobana banking bank-billet-accounts update [ID] [OPCIONES]
```

`PUT /v1/bank_billet_accounts/{id}`

Actualiza información específica de una Cartera.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `kind` | `string` | no | Tipo de CNAB: * `cnab400` CNAB 400 * `cnab240` CNAB 240 Valores: `cnab400`, `cnab240`. |
| `bank_contract_slug` | `string` | no | Slug de la Cartera. Las carteras activas no podrán tener el campo bank_contract_slug alterados. |
| `financial_account` | `object` | no | Cuenta Financiera asociada a la cartera |
| `payment_place` | `string` | no | Lugar de Pago |
| `custom_name` | `string` | no | Nombre de la Cartera para identificación dentro de Kobana. |
| `allow_expiration_on_weekends` | `boolean` | no | Permitir vencimiento en fin de semana y feriados |
| `beneficiary_name` | `string` | no | Nombre del Beneficiario |
| `beneficiary_cnpj_cpf` | `string` | no | CNPJ/CPF del Beneficiario |
| `agency_number` | `string` | no | Agencia. Completado automáticamente cuando se proporciona financial_account_uid. |
| `agency_digit` | `string` | no | Dígito de la Agencia. Opcional: será calculado automáticamente usando Módulo 11 cuando el banco lo requiera y no se proporcione. Completado automáticamente cuando se proporciona financial_account_uid. |
| `account_number` | `string` | no | Cuenta. Completada automáticamente cuando se proporciona financial_account_uid. |
| `account_digit` | `string` | no | Dígito de la Cuenta. Completado automáticamente cuando se proporciona financial_account_uid. |
| `extra1` | `string` | no | Campo extra 1. |
| `extra1_digit` | `string` | no | Dígito del Campo extra 1. |
| `extra2` | `string` | no | Campo extra 2. |
| `extra2_digit` | `string` | no | Dígito del Campo extra 2. |
| `contract` | `string` | no | Contrato |
| `contract_type` | `string` | no | Tipo del Contrato |
| `extra3` | `string` | no | Código de Remesa |
| `beneficiary_address_street` | `string` | no | Calle del Beneficiario |
| `beneficiary_address_street_number` | `string` | no | Número de la calle del Beneficiario |
| `beneficiary_address_complement` | `string` | no | Complemento de la dirección del Beneficiario |
| `beneficiary_address_neighborhood` | `string` | no | Barrio del Beneficiario |
| `beneficiary_address_city` | `string` | no | Ciudad del Beneficiario |
| `beneficiary_address_state` | `string` | no | Estado del Beneficiario |
| `beneficiary_address_zipcode` | `string` | no | CEP del Beneficiario (formato 99999999) |
| `beneficiary_address` | `string` | no | Dirección completa del Beneficiario. |
| `bolepix_enabled` | `boolean` | no | Habilitar Bolepix (legado — prefiera el objeto `bolepix`) |
| `bolepix` | `object` | no | Configuración de Boleto Híbrido |
| `bolepix_key` | `string` | no | Clave Pix que será usada en el Bolepix |
| `default` | `boolean` | no | Predeterminado. El campo `default` determina la cartera de cobranza que se utilizará en la creación del boleto cuando ninguna cartera sea informada. En el momento que la primera cartera es homologada (cambia a `status` = `active`), recibe el valor `default` = `true` |
| `remittance_disabled` | `boolean` | no | Desabilitar generación de remesas. |
| `next_our_number` | `integer` | no | Último nuestro número |
| `next_remittance_number` | `integer` | no | Último secuencial de la remesa Predeterminado: 1 |
| `bank_billet_account_next_our_number_attributes` | `object` | no |  |
| `bank_billet_account_next_remittance_number_attributes` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema banking.bank-billet-accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Cartera actualizada |
| `404` | Cartera no encontrada |
| `422` | Entidad No Procesable |

## bank-billet-accounts delete

Eliminar la Cartera de Cobro

```text
kobana banking bank-billet-accounts delete [ID] [OPCIONES]
```

`DELETE /v1/bank_billet_accounts/{id}`

Elimina una Cartera de Cobro. Opcionalmente, también elimina la Cuenta Financiera asociada.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `delete_financial_account` | `boolean` | no | Cuando `true`, también elimina la Cuenta Financiera asociada a la Cartera. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Cartera y Cuenta Financiera eliminadas (delete_financial_account=true) |
| `204` | Cartera encolada para eliminación (delete_financial_account=false) |
| `404` | Cartera no encontrada |

## bank-billet-accounts ask

Solicitar Homologación de la Cartera de Cobro

```text
kobana banking bank-billet-accounts ask [ID] [OPCIONES]
```

`GET /v1/bank_billet_accounts/{id}/ask`

Solicitar homologación de la Cartera de Cobro

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Solicitud de homologación realizada con éxito |
| `404` | Cartera no encontrada |
| `422` | Cartera ya homologada |

## bank-billet-accounts set-default

Cambiar Cartera de Cobro predeterminada

```text
kobana banking bank-billet-accounts set-default [ID] [OPCIONES]
```

`PUT /v1/bank_billet_accounts/{id}/set_default`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `default` | `boolean` | sí | Define la Cartera predeterminada (Default: true) o dejar de ser predeterminada (Default: false) |

Otros campos y objetos anidados: `kobana schema banking.bank-billet-accounts.set-default`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Cartera definida como predeterminada con éxito |
| `404` | Cartera no encontrada |
| `422` | Entidad No Procesable |

## bank-billet-accounts validate

Validar Cartera de Cobro

```text
kobana banking bank-billet-accounts validate [ID] [OPCIONES]
```

`PUT /v1/bank_billet_accounts/{id}/validate`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `homologation_amount` | `string` | sí | Valor recibido por el boleto (R$) Ej.: 1,87 |

Otros campos y objetos anidados: `kobana schema banking.bank-billet-accounts.validate`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Cartera Homologada con éxito |
| `404` | Cartera no encontrada |
| `422` | Entidad No Procesable |
