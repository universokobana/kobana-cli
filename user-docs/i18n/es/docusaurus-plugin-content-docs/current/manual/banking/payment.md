---
id: payment
title: "kobana banking payment"
sidebar_label: "payment"
description: "Pagos (boletos, Pix, impuestos, servicios públicos)"
---

# kobana banking payment

Pagos (boletos, Pix, impuestos, servicios públicos)

Comandos de `kobana banking payment`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`payment accounts list`](#payment-accounts-list) | `GET /v2/payment/accounts` | Listar cuentas de pago |
| [`payment accounts create`](#payment-accounts-create) | `POST /v2/payment/accounts` | Crear cuenta de pago |
| [`payment accounts get`](#payment-accounts-get) | `GET /v2/payment/accounts/{uid}` | Buscar cuenta de pago |
| [`payment accounts update`](#payment-accounts-update) | `PATCH /v2/payment/accounts/{uid}` | Actualizar cuenta de pago |
| [`payment bank-billet-batches create`](#payment-bank-billet-batches-create) | `POST /v2/payment/bank_billet_batches` | Crear um Lote de pago de Boletos |
| [`payment bank-billets list`](#payment-bank-billets-list) | `GET /v2/payment/bank_billets` | Listar Pagos de Boleto |
| [`payment bank-billets create`](#payment-bank-billets-create) | `POST /v2/payment/bank_billets` | Crear um pago de Boleto |
| [`payment bank-billets get`](#payment-bank-billets-get) | `GET /v2/payment/bank_billets/{uid}` | Ver um pago de Boleto |
| [`payment bank-billets cancel`](#payment-bank-billets-cancel) | `PUT /v2/payment/bank_billets/{uid}/cancel` | Cancelar um pago de Boleto |
| [`payment batches list`](#payment-batches-list) | `GET /v2/payment/batches` | Listar Lotes de pagos |
| [`payment batches get`](#payment-batches-get) | `GET /v2/payment/batches/{uid}` | Ver um Lote de pago |
| [`payment batches approve`](#payment-batches-approve) | `PUT /v2/payment/batches/{uid}/approve` | Aprobar um Lote de pago |
| [`payment batches cancel`](#payment-batches-cancel) | `PUT /v2/payment/batches/{uid}/cancel` | Cancelar um Lote de pago |
| [`payment batches reprove`](#payment-batches-reprove) | `PUT /v2/payment/batches/{uid}/reprove` | Rechazar um Lote de pago |
| [`payment dda-accounts list`](#payment-dda-accounts-list) | `GET /v2/payment/dda_accounts` | Listar cuentas de DDA |
| [`payment dda-accounts create`](#payment-dda-accounts-create) | `POST /v2/payment/dda_accounts` | Crear cuenta de DDA |
| [`payment dda-accounts get`](#payment-dda-accounts-get) | `GET /v2/payment/dda_accounts/{uid}` | Buscar cuenta de DDA |
| [`payment dda-accounts update`](#payment-dda-accounts-update) | `PUT /v2/payment/dda_accounts/{uid}` | Actualizar cuenta de DDA |
| [`payment dda-accounts disable`](#payment-dda-accounts-disable) | `PUT /v2/payment/dda_accounts/{uid}/disable` | Deshabilitar cuenta de DDA |
| [`payment dda-accounts enable`](#payment-dda-accounts-enable) | `PUT /v2/payment/dda_accounts/{uid}/enable` | Habilitar cuenta de DDA |
| [`payment dda-bank-billets list`](#payment-dda-bank-billets-list) | `GET /v2/payment/dda_bank_billets` | Listar Boletos DDA |
| [`payment dda-bank-billets get`](#payment-dda-bank-billets-get) | `GET /v2/payment/dda_bank_billets/{uid}` | Buscar Boleto DDA |
| [`payment dda-bank-billets delete`](#payment-dda-bank-billets-delete) | `DELETE /v2/payment/dda_bank_billets/{uid}` | Eliminar Boleto DDA |
| [`payment dda-bank-billets reject`](#payment-dda-bank-billets-reject) | `PUT /v2/payment/dda_bank_billets/{uid}/reject` | Rechazar Boleto DDA |
| [`payment dda-bank-billets release`](#payment-dda-bank-billets-release) | `PUT /v2/payment/dda_bank_billets/{uid}/release` | Liberar Boleto DDA para pago |
| [`payment pix list`](#payment-pix-list) | `GET /v2/payment/pix` | Listar pagos Pix |
| [`payment pix create`](#payment-pix-create) | `POST /v2/payment/pix` | Crear um pago Pix |
| [`payment pix get`](#payment-pix-get) | `GET /v2/payment/pix/{uid}` | Ver um pago Pix |
| [`payment pix cancel`](#payment-pix-cancel) | `PUT /v2/payment/pix/{uid}/cancel` | Cancelar um pago Pix |
| [`payment pix-batches create`](#payment-pix-batches-create) | `POST /v2/payment/pix_batches` | Crear um Lote de pago Pix |
| [`payment tax-batches create`](#payment-tax-batches-create) | `POST /v2/payment/tax_batches` | Crear Lote de pagos de Tributo |
| [`payment taxes list`](#payment-taxes-list) | `GET /v2/payment/taxes` | Listar pagos de Tributo |
| [`payment taxes create`](#payment-taxes-create) | `POST /v2/payment/taxes` | Crear pago de Tributo |
| [`payment taxes get`](#payment-taxes-get) | `GET /v2/payment/taxes/{uid}` | Exibir pago de Tributo |
| [`payment taxes cancel`](#payment-taxes-cancel) | `PUT /v2/payment/taxes/{uid}/cancel` | Cancelar um pago de Tributo |
| [`payment utilities list`](#payment-utilities-list) | `GET /v2/payment/utilities` | Listar pagos de cuentas de Consumo |
| [`payment utilities create`](#payment-utilities-create) | `POST /v2/payment/utilities` | Crear um pago de cuenta de Consumo |
| [`payment utilities get`](#payment-utilities-get) | `GET /v2/payment/utilities/{uid}` | Ver um pago de cuenta de Consumo |
| [`payment utilities cancel`](#payment-utilities-cancel) | `PUT /v2/payment/utilities/{uid}/cancel` | Cancelar um pago de cuenta de Consumo |
| [`payment utility-batches create`](#payment-utility-batches-create) | `POST /v2/payment/utility_batches` | Crear un Lote de Pago de Cuentas de Consumo |

## payment accounts list

Listar cuentas de pago

```text
kobana banking payment accounts list [OPCIONES]
```

`GET /v2/payment/accounts`

Lista todas las cuentas de pago registradas en la cuenta. Soporta paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `financial_account_uid` | `string` | no | Filtrar por UID de la cuenta financiera |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cuentas de pago acessadas con escopo read |

## payment accounts create

Crear cuenta de pago

```text
kobana banking payment accounts create [OPCIONES]
```

`POST /v2/payment/accounts`

Cria uma nova cuenta de pago associada a uma cuenta financeira. Use quando a cuenta financeira precisa de mais de uma cuenta de pago (por exemplo, múltiplos convênios en lo mesmo banco). A cuenta é creada sin conexão bancária (`connection_uid` retorna `null`); associe uma conexão antes de Habilitar remessas/registro. Requer escopo `write` + `payment.accounts`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string` | sí | UUID de la Cuenta Financiera que recibirá la nueva cuenta de pago |
| `agreement_code` | `string` | no | Código del convenio |
| `external_id` | `string` | no | Identificador externo de la cuenta de pago |
| `remittance_enabled` | `boolean` | no | Habilita o deshabilita la generación de remesas EDI para esta cuenta. |
| `provider_data` | `object` | no | Atributos específicos del proveedor bancario. |

Otros campos y objetos anidados: `kobana schema banking.payment.accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | cuenta de pago creada |
| `404` | Cuenta Financiera no encontrada |
| `422` | Erro de validação. `agreement_code` acima del limite. |

## payment accounts get

Buscar cuenta de pago

```text
kobana banking payment accounts get [UID] [OPCIONES]
```

`GET /v2/payment/accounts/{uid}`

Retorna los detalhes de uma cuenta de pago específica pelo UID

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cuenta de pago acessada con escopo read |
| `404` | cuenta de pago en lo encontrada |

## payment accounts update

Actualizar cuenta de pago

```text
kobana banking payment accounts update [UID] [OPCIONES]
```

`PATCH /v2/payment/accounts/{uid}`

Atualiza atributos editáveis de uma cuenta de pago. Aceita `remittance_enabled` (booleano, controla a geração de remessas EDI), `provider_data.service_point_number` (string, ponto de atendimento — por exemplo `numeroPa` en lo Sicoob) y `provider_data.cooperative_cnpj` (string, CNPJ de la cooperativa Sicoob). O corpo aceita PATCH parcial: envie apenas los campos que deseja Actualizar.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `uid` | `string` | no | UUID de la cuenta de pago |
| `financial_account_uid` | `string` | no | UUID de la cuenta financiera asociada |
| `connection_uid` | `string (uuid)` | no | Conexión bancaria de la cuenta. |
| `agreement_code` | `string` | no | Código del convenio |
| `provider_data` | `object` | no | Datos específicos del proveedor. |
| `external_id` | `string` | no | Identificador externo de la cuenta de pago |
| `balance_validation_enabled_at` | `string (date-time)` | no | Fecha y hora en que la validación de saldo fue habilitada |
| `remittance_enabled_at` | `string (date-time)` | no | Fecha y hora en que la remesa EDI fue habilitada |
| `remittance_enabled` | `boolean` | no | Habilita o deshabilita la generación de remesas EDI para esta cuenta. |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.payment.accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | PATCH parcial — apenas service_point_number (sin remittance_enabled) |
| `404` | cuenta de pago en lo encontrada |
| `422` | Erro de validação. `remittance_enabled` deve ser verdadeiro o falso. |

## payment bank-billet-batches create

Crear um Lote de pago de Boletos

```text
kobana banking payment bank-billet-batches create [OPCIONES]
```

`POST /v2/payment/bank_billet_batches`

Este endpoint se utiliza para la generación de un lote de pago.

**Requisito previo**

Para emitir um lote de pago é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de pago aplicada ao lote é definida em `payment_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de pago, o sistema cria um comando para registrá-lo en la instituição financeira. Após o registro del lote ser confirmado en la instituição, o evento `payment.bank_billet_batch.confirmed` é disparado.

O evento `payment.db.created` também é disparado nesta operação caso o lote contenha um novo pago.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `payments` | `array<anyOf>` | sí | Lista de Pagos de Boletos nuevos o existentes. |
| `payment_account_uid` | `string (uuid)` | no | Cuenta de pago aplicada a todo el lote (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se proporciona, debe pertenecer a la cuenta financiera del lote. |

Otros campos y objetos anidados: `kobana schema banking.payment.bank-billet-batches.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Lote de pago creado |
| `422` | Error de validación — external_id duplicado |

## payment bank-billets list

Listar Pagos de Boleto

```text
kobana banking payment bank-billets list [OPCIONES]
```

`GET /v2/payment/bank_billets`

Lista todos los pagos de boletos bancários realizados. Soporta paginación y retorna información detalhadas sobre cada pago.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pagos encontrados |

## payment bank-billets create

Crear um pago de Boleto

```text
kobana banking payment bank-billets create [OPCIONES]
```

`POST /v2/payment/bank_billets`

Este endpoint se utiliza para la generación de un pago.

**Requisito previo**

Para emitir um pago é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de pago que registra a operação é definida em `payment_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.db.created` é disparado nesta operação.

> **Próximos passos:** Após Crear um pago, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/payment/bank_billet_batches`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Cantidad |
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `code` | `string` | sí | Código de Barras o Línea Digitible |
| `scheduled_to` | `string (date)` | no | Fecha de Programación |
| `payment_account_uid` | `string (uuid)` | no | Cuenta de pago de origen (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se proporciona, debe pertenecer a `financial_account_uid`. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |
| `transfer_purpose` | `string` | no | Objetivo del pago. Define el tipo de servicio del lote en la remesa enviada al banco. Opcional. Cuando no se informa, la remesa sale como `98` (Pagos Diversos). Informe solo si el banco exige un tipo de lote específico: Itaú, por ejemplo, rechaza remesas cuyo tipo de lote la empresa no haya contratado. Los pagos con objetivos diferentes se envían en remesas separadas, porque el tipo de servicio describe el lote entero. En pagos, hoy solo Itaú lee este campo en la remesa. En los demás bancos el tipo de lote es fijo: el campo se acepta y se devuelve en la respuesta, pero no altera el archivo enviado. `20`: Pago Proveedor. `98`: Pagos Diversos Valores: `20`, `98`. |
| `beneficiary` | `object` | no | Datos del Beneficiario. Obligatorio cuando la cuenta financiera sea BB; de lo contrario, opcional — cuando se proporciona, reemplaza los datos extraídos del código de barras. |

Otros campos y objetos anidados: `kobana schema banking.payment.bank-billets.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | pago creado |
| `422` | Error de validación |

## payment bank-billets get

Ver um pago de Boleto

```text
kobana banking payment bank-billets get [UID] [OPCIONES]
```

`GET /v2/payment/bank_billets/{uid}`

Consulta información específicas de um pago de Boleto.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago encontrado |
| `404` | pago en lo encontrado |

## payment bank-billets cancel

Cancelar um pago de Boleto

```text
kobana banking payment bank-billets cancel [UID] [OPCIONES]
```

`PUT /v2/payment/bank_billets/{uid}/cancel`

> **Procesamiento Asincrónico:** Este endpoint é processado assincronamente y cria um comando de cancelamento para o pago.
O cancelamento de um pago é feito assincronamente, o seja, ao receber a requisição, a nossa API guarda a solicitação y cria um Comando. Esse comando é processado em segundo plano y depois de confirmado, o pago tem seu status alterado para Cancelado. Você recebe en la resposta desta requisição o ID del comando creado y pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `payment.bank_billet.cancel.requested` é disparado.
O nosso servidor então começa las tentativas de envio del cancelamento para a instituição financeira.
Somente depois de confirmado en la instituição financeira, aí então o pago é marcado como cancelado em nosso banco de dados.
Neste momento o evento `payment.bank_billet.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. O pago en lo podrá más ser processado após o cancelamento.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago cancelado sin interacción con proveedor |
| `422` | El pago ya fue confirmado, rechazado o cancelado |

## payment batches list

Listar Lotes de pagos

```text
kobana banking payment batches list [OPCIONES]
```

`GET /v2/payment/batches`

Retorna uma lista paginada de todos los lotes de pagos de la cuenta. los lotes são retornados em ordem decrescente de data de criação, con los mais recentes primeiro. Este endpoint permite filtrar y paginar los resultados para facilitar a navegação através de grandes volumes de lotes.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lotes encontrados |

## payment batches get

Ver um Lote de pago

```text
kobana banking payment batches get [UID] [OPCIONES]
```

`GET /v2/payment/batches/{uid}`

Consulta información específicas de um pago de Boleto.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lote de pago encontrado |
| `404` | Lote de pago en lo encontrado |

## payment batches approve

Aprobar um Lote de pago

```text
kobana banking payment batches approve [UID] [OPCIONES]
```

`PUT /v2/payment/batches/{uid}/approve`

> **Procesamiento Asincrónico:** Este endpoint é processado assincronamente y cria um comando de aprovação para o Lote de pago.
  A aprovação de um Lote de pago é feito assincronamente, o seja, ao receber a requisição, a nossa API guarda a solicitação y cria um Comando. Esse comando é processado em segundo plano y depois de confirmado, o Lote tem seu status alterado para Aprovado. Você recebe en la resposta desta requisição o ID del comando creado y pode consultá-lo a qualquer momento.

  ## Eventos de Webhook

  Quando a requisição é realizada, um evento `payment.payment_batch.approve.requested` é disparado.
  O nosso servidor então começa las tentativas de envio de la liberação para a instituição financeira.
  Somente depois de confirmado en la instituição financeira, aí então o lote é marcado como aprovado em nosso banco de dados.
  Neste momento o evento `payment.payment_batch.approve.confirmed` é disparado.

  > ⚠️ **Atenção ao status Aprovado:** O status de Aprovado não é terminal. Quando a instituição financeira confirmar los pagos, los mesmos terão seus status alterados para Confirmado

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `404` | Lote de pago en lo encontrado |
| `422` | Lote de Transferencia no está esperando liberación |

## payment batches cancel

Cancelar um Lote de pago

```text
kobana banking payment batches cancel [UID] [OPCIONES]
```

`PUT /v2/payment/batches/{uid}/cancel`

> **Procesamiento Asincrónico:** cria um comando de cancelamento para o Lote de pago.

  Quando o proveedor financeiro expõe API de cancelamento de lote (ex.: Banco del Brasil),
  é creado 1 Comando de cancelamento en lo lote. Para provedores sin API de lote, o
  cancelamento é feito pago a pago (fallback per-item). Antes del registro, o
  lote é cancelado localmente sin interacción con proveedor.

  ## Eventos de Webhook

  Quando a requisição é realizada, um evento `payment.{type}_batch.cancel.requested` é disparado.
  Somente depois de confirmado en la instituição financeira o lote é marcado como cancelado,
  disparando `payment.{type}_batch.cancel.confirmed`.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando de cancelamento creado con sucesso |
| `422` | Lote de pago en lo puede ser cancelado en lo estado atual |

## payment batches reprove

Rechazar um Lote de pago

```text
kobana banking payment batches reprove [UID] [OPCIONES]
```

`PUT /v2/payment/batches/{uid}/reprove`

> **Procesamiento Asincrónico:** Este endpoint é processado assincronamente y cria um comando de reprovação para o Lote de pago.
  A reprovação de um Lote de pago é feita assincronamente, o seja, ao receber a requisição, a nossa API guarda a solicitação y cria um Comando. Esse comando é processado em segundo plano y depois de confirmado, o Lote tem seu status alterado para Reprovado. Você recebe en la resposta desta requisição o ID del comando creado y pode consultá-lo a qualquer momento.

  ## Eventos de Webhook

  Quando a requisição é realizada, um evento `payment.payment_batch.reprove.requested` é disparado.
  O nosso servidor então começa las tentativas de envio de la liberação para a instituição financeira.
  Somente depois de confirmado en la instituição financeira, aí então o lote é marcado como aprovado em nosso banco de dados.
  Neste momento o evento `payment.payment_batch.reprove.confirmed` é disparado.

  > ⚠️ **Atenção ao status Reprovado:** O status de Reprovado é terminal. Quando a instituição financeira Cancelar los pagos, los mesmos terão seus status alterados para Reprovado

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando creado exitosamente |
| `404` | Lote de pago en lo encontrado |
| `422` | Lote de Transferencia no está esperando liberación |

## payment dda-accounts list

Listar cuentas de DDA

```text
kobana banking payment dda-accounts list [OPCIONES]
```

`GET /v2/payment/dda_accounts`

Lista todas las cuentas DDA registradas en la cuenta. Soporta paginación.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista vazia quando en lo hay cuentas DDA |

## payment dda-accounts create

Crear cuenta de DDA

```text
kobana banking payment dda-accounts create [OPCIONES]
```

`POST /v2/payment/dda_accounts`

Cria uma nova cuenta DDA.

**Eventos de Webhook**

O evento `payment.dda.account.db.created` é disparado nesta operação.

Após a criação, o sistema gera automaticamente a carta DDA (PDF), disparando los eventos `payment.dda.account.letter.pending`, `payment.dda.account.letter.waiting_signatures` y `payment.dda.account.letter.signed` à medida que o fluxo de assinatura progride.

Após aprovação pelo Manager, los eventos `payment.dda.account.confirmed` o `payment.dda.account.rejected` são disparados.

Após `confirmed`, o sistema inicia o registro de la cuenta en lo banco de forma assíncrona, disparando `payment.dda.account.register.requested` y `payment.dda.account.register.confirmed` (o `failed`/`error`) conforme a resposta del provider.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cnpj` | `string` | sí | CNPJ o CPF de la empresa/persona para la cual se habilitará el DDA. Debe ser un documento válido. Cada cuenta Kobana puede tener solo una cuenta DDA activa. |
| `signers` | `array<object>` | no | Lista de 1 a 3 signatarios de la carta de adhesión al DDA. Cada signatario recibirá un correo electrónico con enlace para firma digital. |
| `business_legal_name` | `string` | no | Razón social de la empresa asociada al CNPJ de la cuenta DDA. Campo opcional. |
| `phone_number` | `string` | no | Teléfono de contacto de la empresa asociada al CNPJ de la cuenta DDA. Campo opcional. |
| `address` | `object` | no | Dirección de la empresa asociada al CNPJ de la cuenta DDA. Campo opcional; todos los subcampos también son opcionales. |

Otros campos y objetos anidados: `kobana schema banking.payment.dda-accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | cuenta de DDA creada con sucesso |
| `422` | Subcampo de address con valor en lo escalar (array) |

## payment dda-accounts get

Buscar cuenta de DDA

```text
kobana banking payment dda-accounts get [UID] [OPCIONES]
```

`GET /v2/payment/dda_accounts/{uid}`

Retorna los detalhes de uma cuenta de DDA específica pelo UID.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cuenta de DDA encontrada |
| `404` | cuenta de DDA en lo encontrada |

## payment dda-accounts update

Actualizar cuenta de DDA

```text
kobana banking payment dda-accounts update [UID] [OPCIONES]
```

`PUT /v2/payment/dda_accounts/{uid}`

Atualiza los signatários de uma cuenta DDA. Somente permitido quando a cuenta é editável (editable=true).

**Eventos de Webhook**

O evento `payment.dda.account.db.updated` é disparado nesta operação.

Se a cuenta possuía carta em `external_letter_status: waiting_signatures`, o sistema sincroniza los novos signatários con o provider de assinatura de forma assíncrona. Caso o provider reinicie o fluxo de assinatura, los eventos `payment.dda.account.letter.pending` y subsequentes serão disparados.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `signers` | `array<object>` | no | Lista de 1 a 3 signatarios de la carta de adhesión al DDA. Cada signatario recibirá un correo electrónico con enlace para firma digital. |
| `business_legal_name` | `string` | no | Razón social de la empresa asociada al CNPJ de la cuenta DDA. Campo opcional. |
| `phone_number` | `string` | no | Teléfono de contacto de la empresa asociada al CNPJ de la cuenta DDA. Campo opcional. |
| `address` | `object` | no | Dirección de la empresa asociada al CNPJ de la cuenta DDA. Campo opcional; todos los subcampos también son opcionales. Reemplaza el hash completo guardado anteriormente — no realiza merge parcial. |

Otros campos y objetos anidados: `kobana schema banking.payment.dda-accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cuenta de DDA atualizada con sucesso |
| `422` | cuenta no editable |

## payment dda-accounts disable

Deshabilitar cuenta de DDA

```text
kobana banking payment dda-accounts disable [UID] [OPCIONES]
```

`PUT /v2/payment/dda_accounts/{uid}/disable`

Solicita a desabilitação de uma cuenta DDA existente. Não remove `enabled_at` diretamente.

**Eventos de Webhook**

O evento `payment.dda.account.db.updated` é disparado nesta operação, refletindo a mudança em `disable_requested_at`. A desabilitação em si só é confirmada de forma assíncrona, quando o proveedor responde: `payment.dda.account.disabled` (sucesso) o `payment.dda.account.failed` (falha).

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cuenta de DDA desabilitada con sucesso |
| `404` | cuenta de DDA en lo encontrada |

## payment dda-accounts enable

Habilitar cuenta de DDA

```text
kobana banking payment dda-accounts enable [UID] [OPCIONES]
```

`PUT /v2/payment/dda_accounts/{uid}/enable`

Solicita a habilitação de uma cuenta DDA previamente desabilitada. Sujeito a período de cooldown de 21 minutos após desabilitação.

**Eventos de Webhook**

O evento `payment.dda.account.db.updated` é disparado nesta operação, refletindo a mudança em `enable_requested_at`. A habilitação em si só é confirmada de forma assíncrona, quando o proveedor responde: `payment.dda.account.enabled` (sucesso) o `payment.dda.account.failed` (falha/resposta incompleta del proveedor).

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | cuenta de DDA habilitada con sucesso |
| `404` | cuenta de DDA en lo encontrada |
| `422` | cuenta de DDA no puede ser habilitada sin carta firmada y estado aprobado |

## payment dda-bank-billets list

Listar Boletos DDA

```text
kobana banking payment dda-bank-billets list [OPCIONES]
```

`GET /v2/payment/dda_bank_billets`

Lista todos los boletos DDA vinculados à cuenta. Soporta paginación.

**Eventos de Webhook**

Boletos DDA chegam exclusivamente via sincronização automática (en lo hay criação manual). los eventos de webhook relacionados são:

- `payment.dda.bank_billet.db.created` — disparado quando um novo boleto é sincronizado del provider
- `payment.dda.bank_billet.external_status_changed` — disparado quando o status externo del boleto muda (ex: `open` → `paid`)

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista vazia quando en lo hay boletos DDA |

## payment dda-bank-billets get

Buscar Boleto DDA

```text
kobana banking payment dda-bank-billets get [UID] [OPCIONES]
```

`GET /v2/payment/dda_bank_billets/{uid}`

Retorna los detalhes de um boleto DDA específico pelo UID.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Boleto DDA encontrado |
| `404` | Boleto DDA en lo encontrado |

## payment dda-bank-billets delete

Eliminar Boleto DDA

```text
kobana banking payment dda-bank-billets delete [UID] [OPCIONES]
```

`DELETE /v2/payment/dda_bank_billets/{uid}`

Elimina (soft delete) un boleto DDA existente.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Boleto DDA excluído con sucesso |
| `404` | Boleto DDA en lo encontrado |

## payment dda-bank-billets reject

Rechazar Boleto DDA

```text
kobana banking payment dda-bank-billets reject [UID] [OPCIONES]
```

`PUT /v2/payment/dda_bank_billets/{uid}/reject`

Rejeita um boleto DDA con status `pending`. Boletos rejeitados não serão processados para pago.

**Eventos de Webhook**

O evento `payment.dda.bank_billet.db.updated` é disparado nesta operação con las alterações de status (`pending` → `rejected`) y o campo `rejected_at`. Este evento é disparado em qualquer atualização del boleto DDA, não apenas nesta operação.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Boleto DDA rejeitado con sucesso |
| `404` | Boleto DDA en lo encontrado |
| `422` | Transición de estado inválida |

## payment dda-bank-billets release

Liberar Boleto DDA para pago

```text
kobana banking payment dda-bank-billets release [UID] [OPCIONES]
```

`PUT /v2/payment/dda_bank_billets/{uid}/release`

Libera um boleto DDA con status `pending` para pago. Somente boletos con `external_status: open` podem ser liberados.

**Eventos de Webhook**

O evento `payment.dda.bank_billet.db.updated` é disparado nesta operação con las alterações de status (`pending` → `released`) y o campo `released_at`. Este evento é disparado em qualquer atualização del boleto DDA, não apenas nesta operação.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Boleto DDA liberado para pago con sucesso |
| `404` | Boleto DDA en lo encontrado |
| `422` | Transición de estado inválida |

## payment pix list

Listar pagos Pix

```text
kobana banking payment pix list [OPCIONES]
```

`GET /v2/payment/pix`

Lista todos los pagos realizados via Pix. Soporta paginación y retorna información detalhadas sobre cada pago.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pagos encontrados |

## payment pix create

Crear um pago Pix

```text
kobana banking payment pix create [OPCIONES]
```

`POST /v2/payment/pix`

Este endpoint se utiliza para la generación de un pago.

**Requisito previo**

Para emitir um pago é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de pago que registra a operação é definida em `payment_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.db.created` é disparado nesta operação.

> **Próximos passos:** Após Crear um pago, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/payment/pix_batches`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `qrcode` | `string` | sí | Código QR Copiar y Pegar |
| `amount` | `number` | no | Cantidad. Opcional, el valor se extraerá automáticamente de `qrcode`. |
| `scheduled_to` | `string (date)` | no | Fecha de Programación |
| `payment_account_uid` | `string (uuid)` | no | Cuenta de pago de origen (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se proporciona, debe pertenecer a `financial_account_uid`. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `transfer_purpose` | `string` | no | Objetivo del pago. Define el tipo de servicio del lote en la remesa enviada al banco. Opcional. Cuando no se informa, la remesa sale como `98` (Pagos Diversos). Informe solo si el banco exige un tipo de lote específico: Itaú, por ejemplo, rechaza remesas cuyo tipo de lote la empresa no haya contratado. Los pagos con objetivos diferentes se envían en remesas separadas, porque el tipo de servicio describe el lote entero. En pagos, hoy solo Itaú lee este campo en la remesa. En los demás bancos el tipo de lote es fijo: el campo se acepta y se devuelve en la respuesta, pero no altera el archivo enviado. En Itaú el lote de Pix QR-Code usa el tipo de servicio fijo `20`: el valor informado se acepta y se devuelve, pero no altera la remesa. `01`: Cobranza. `03`: Boleto de Pago Electrónico. `04`: Conciliación Bancaria. `05`: Débitos. `06`: Custodia de Cheques. `07`: Gestión de Caja. `08`: Consulta/Información de Margen. `09`: Registro de la Consignación/Retención. `10`: Pago de Dividendos. `11`: Mantenimiento de la Consignación. `12`: Consignación de Cuotas. `13`: Glosa de la Consignación (INSS). `14`: Consulta de Tributos a pagar. `20`: Pago Proveedor. `22`: Pago de Cuentas, Tributos e Impuestos. `23`: Interoperabilidad entre Cuentas de Instituciones de Pago. `25`: Compror. `26`: Compror Rotativo. `29`: Alegación del Pagador. `30`: Pago de Salarios. `32`: Pago de honorarios. `33`: Pago de beca de ayuda. `34`: Pago de prebenda (remuneración a padres y sacerdotes). `40`: Vendor. `41`: Vendor a Plazo. `50`: Pago de Siniestros Asegurados. `60`: Pago de Gastos de Viajero en Tránsito. `70`: Pago Autorizado. `75`: Pago a Acreditados. `77`: Pago de Remuneración. `80`: Pago a Representantes / Vendedores Autorizados. `90`: Pago de Beneficios. `98`: Pagos Diversos Valores: `01`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `20`, `22`, `23`, `25`, `26`, `29`, `30`, `32`, `33`, `34`, `40`, `41`, `50`, `60`, `70`, `75`, `77`, `80`, `90`, `98`. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |
| `identifier` | `string` | no | Identificación del pago en el comprobante del proveedor financiero. (Uso exclusivo del banco) Bancos que aceptan este campo (vía API): Itaú |
| `beneficiary` | `object` | no | Datos del beneficiario. |

Otros campos y objetos anidados: `kobana schema banking.payment.pix.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | pago creado |
| `422` | Error de validación |

## payment pix get

Ver um pago Pix

```text
kobana banking payment pix get [UID] [OPCIONES]
```

`GET /v2/payment/pix/{uid}`

Consulta información específicas de um pago Pix.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago encontrado |
| `404` | pago en lo encontrado |

## payment pix cancel

Cancelar um pago Pix

```text
kobana banking payment pix cancel [UID] [OPCIONES]
```

`PUT /v2/payment/pix/{uid}/cancel`

> **Procesamiento Asincrónico:** Este endpoint é processado assincronamente y cria um comando de cancelamento para o pago.
O cancelamento de um pago é feito assincronamente, o seja, ao receber a requisição, a nossa API guarda a solicitação y cria um Comando. Esse comando é processado em segundo plano y depois de confirmado, o pago tem seu status alterado para Cancelado. Você recebe en la resposta desta requisição o ID del comando creado y pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `payment.pix.cancel.requested` é disparado.
O nosso servidor então começa las tentativas de envio del cancelamento para a instituição financeira.
Somente depois de confirmado en la instituição financeira, aí então o pago é marcado como cancelado em nosso banco de dados.
Neste momento o evento `payment.pix.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. O pago en lo podrá más ser processado após o cancelamento.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago cancelado sin interacción con proveedor |
| `422` | El pago ya fue confirmado, rechazado o cancelado |

## payment pix-batches create

Crear um Lote de pago Pix

```text
kobana banking payment pix-batches create [OPCIONES]
```

`POST /v2/payment/pix_batches`

Este endpoint se utiliza para la generación de un lote de pago.

**Requisito previo**

Para emitir um lote de pago é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de pago aplicada ao lote é definida em `payment_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de pago, o sistema cria um comando para registrá-lo en la instituição financeira. Após o registro del lote ser confirmado en la instituição, o evento `payment.pix_batch.confirmed` é disparado.

O evento `payment.db.created` também é disparado nesta operação caso o lote contenha um novo pago.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `payments` | `array<anyOf>` | sí | Lista de Pagos de Pix nuevos o existentes. |
| `payment_account_uid` | `string (uuid)` | no | Cuenta de pago aplicada a todo el lote (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se proporciona, debe pertenecer a la cuenta financiera del lote. |

Otros campos y objetos anidados: `kobana schema banking.payment.pix-batches.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Lote de pago creado |
| `422` | Error de validación — external_id duplicado |

## payment tax-batches create

Crear Lote de pagos de Tributo

```text
kobana banking payment tax-batches create [OPCIONES]
```

`POST /v2/payment/tax_batches`

Este endpoint se utiliza para la generación de un lote de pago de tributos municipais, estaduais y federais.

> ⚠️ **Novidade em Desenvolvimento!** Esta API está em versão beta - isso significa que estamos constantemente melhorando y ajustando suas funcionalidades. Embora seja totalmente funcional, algumas coisas podem mudar conforme recebemos feedback dos desenvolvedores.
>
> Para dúvidas sobre esta funcionalidade, entre em contato conosco.

**Requisito previo**

Para emitir um lote de pago é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de pago aplicada ao lote é definida em `payment_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.batch.db.created` é disparado nesta operação.

Ao cadastrar um Lote de pago, o sistema cria um comando para registrá-lo en la instituição financeira. Após o registro del lote ser confirmado en la instituição, o evento `payment.tax_batch.confirmed` é disparado.

O evento `payment.db.created` também é disparado nesta operação caso o lote contenha um novo pago.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `payments` | `array<anyOf>` | sí | Lista de Pagos de Tributo nuevos o existentes. |
| `payment_account_uid` | `string (uuid)` | no | Cuenta de pago aplicada a todo el lote (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se proporciona, debe pertenecer a la cuenta financiera del lote. |

Otros campos y objetos anidados: `kobana schema banking.payment.tax-batches.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Lote creado con format=data (GRU) |
| `422` | Error de validación — external_id duplicado |

## payment taxes list

Listar pagos de Tributo

```text
kobana banking payment taxes list [OPCIONES]
```

`GET /v2/payment/taxes`

Este endpoint se utiliza para la generación de un pago de tributo municipal, estadual o federal.

> ⚠️ **Novidade em Desenvolvimento!** Esta API está em versão beta - isso significa que estamos constantemente melhorando y ajustando suas funcionalidades. Embora seja totalmente funcional, algumas coisas podem mudar conforme recebemos feedback dos desenvolvedores.
>
> Para dúvidas sobre esta funcionalidade, entre em contato conosco.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pagos encontrados |

## payment taxes create

Crear pago de Tributo

```text
kobana banking payment taxes create [OPCIONES]
```

`POST /v2/payment/taxes`

**Requisitos previos**

Para emitir um pago é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de pago que registra a operação é definida em `payment_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Cantidad |
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `kind` | `string` | sí | Tipo de Tributo Valores: `itbi`, `icms`, `iss`, `iptu`, `fgts`, `dare`, `darf`, `gru`, `gps`. |
| `scheduled_to` | `string (date)` | no | Fecha de Programación |
| `payment_account_uid` | `string (uuid)` | no | Cuenta de pago de origen (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se proporciona, debe pertenecer a `financial_account_uid`. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `transfer_purpose` | `string` | no | Objetivo del pago. Define el tipo de servicio del lote en la remesa enviada al banco. Opcional. Cuando no se informa, la remesa sale como `98` (Pagos Diversos). Informe solo si el banco exige un tipo de lote específico: Itaú, por ejemplo, rechaza remesas cuyo tipo de lote la empresa no haya contratado. Los pagos con objetivos diferentes se envían en remesas separadas, porque el tipo de servicio describe el lote entero. En pagos, hoy solo Itaú lee este campo en la remesa. En los demás bancos el tipo de lote es fijo: el campo se acepta y se devuelve en la respuesta, pero no altera el archivo enviado. Cuando `format` es `code`, solo se aceptan `20` y `98`: el tributo con código de barras viaja en el mismo lote que las cuentas de consumo. `01`: Cobranza. `03`: Boleto de Pago Electrónico. `04`: Conciliación Bancaria. `05`: Débitos. `06`: Custodia de Cheques. `07`: Gestión de Caja. `08`: Consulta/Información de Margen. `09`: Registro de la Consignación/Retención. `10`: Pago de Dividendos. `11`: Mantenimiento de la Consignación. `12`: Consignación de Cuotas. `13`: Glosa de la Consignación (INSS). `14`: Consulta de Tributos a pagar. `20`: Pago Proveedor. `22`: Pago de Cuentas, Tributos e Impuestos. `23`: Interoperabilidad entre Cuentas de Instituciones de Pago. `25`: Compror. `26`: Compror Rotativo. `29`: Alegación del Pagador. `30`: Pago de Salarios. `32`: Pago de honorarios. `33`: Pago de beca de ayuda. `34`: Pago de prebenda (remuneración a padres y sacerdotes). `40`: Vendor. `41`: Vendor a Plazo. `50`: Pago de Siniestros Asegurados. `60`: Pago de Gastos de Viajero en Tránsito. `70`: Pago Autorizado. `75`: Pago a Acreditados. `77`: Pago de Remuneración. `80`: Pago a Representantes / Vendedores Autorizados. `90`: Pago de Beneficios. `98`: Pagos Diversos Valores: `01`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `20`, `22`, `23`, `25`, `26`, `29`, `30`, `32`, `33`, `34`, `40`, `41`, `50`, `60`, `70`, `75`, `77`, `80`, `90`, `98`. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |
| `format` | `string` | no | Formato del tributo. `code`: Informar código de barras/línea digitible (predeterminado). `data`: Informar campos estructurados (disponible para kinds: darf, gru, gps, fgts). Valores: `code`, `data`. |
| `code` | `string` | no | Código de Barras o Línea Digitible del Tributo. Obligatorio cuando format=code. |
| `data` | `object` | no | Campos estructurados del tributo. Obligatorio cuando format=data. |

Otros campos y objetos anidados: `kobana schema banking.payment.taxes.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | pago creado con format=data (DARF) |
| `422` | Parámetros inválidos |

## payment taxes get

Exibir pago de Tributo

```text
kobana banking payment taxes get [UID] [OPCIONES]
```

`GET /v2/payment/taxes/{uid}`

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago encontrado |
| `404` | pago en lo encontrado |

## payment taxes cancel

Cancelar um pago de Tributo

```text
kobana banking payment taxes cancel [UID] [OPCIONES]
```

`PUT /v2/payment/taxes/{uid}/cancel`

> **Procesamiento Asincrónico:** Este endpoint é processado assincronamente y cria um comando de cancelamento para o pago.
O cancelamento de um pago é feito assincronamente, o seja, ao receber a requisição, a nossa API guarda a solicitação y cria um Comando. Esse comando é processado em segundo plano y depois de confirmado, o pago tem seu status alterado para Cancelado. Você recebe en la resposta desta requisição o ID del comando creado y pode consultá-lo a qualquer momento.

**Eventos de Webhook**

Quando a requisição é realizada, um evento `payment.tax.cancel.requested` é disparado.
O nosso servidor então começa las tentativas de envio del cancelamento para a instituição financeira.
Somente depois de confirmado en la instituição financeira, aí então o pago é marcado como cancelado em nosso banco de dados.
Neste momento o evento `payment.tax.cancel.confirmed` é disparado.

> ⚠️ **Atenção ao status Cancelado:** O status de Cancelado é terminal. O pago en lo podrá más ser processado após o cancelamento.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago cancelado sin interacción con proveedor |
| `422` | El pago ya fue confirmado, rechazado o cancelado |

## payment utilities list

Listar pagos de cuentas de Consumo

```text
kobana banking payment utilities list [OPCIONES]
```

`GET /v2/payment/utilities`

Lista todos los pagos de cuentas de consumo (água, luz, gas, teléfono, etc.) realizados. Soporta paginación y retorna información detalhadas.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pagos encontrados |

## payment utilities create

Crear um pago de cuenta de Consumo

```text
kobana banking payment utilities create [OPCIONES]
```

`POST /v2/payment/utilities`

Este endpoint se utiliza para la generación de un pago.

**Requisito previo**

Para emitir um pago é necessário ter uma cuenta Financeira, informada em `financial_account_uid`. A cuenta de pago que registra a operação é definida em `payment_account_uid`; si se omite, se usa la más antiga vinculada à cuenta financeira; si se informa, debe pertenecer à `financial_account_uid`.

**Eventos de Webhook**

O evento `payment.db.created` é disparado nesta operação.

> **Próximos passos:** Após Crear um pago, você desejará em algum momento enviar um Lote para a instituição financeira. Para isso, utilize o endpoint `POST /api/v2/payment/utility_batches`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Cantidad |
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `code` | `string` | sí | Código de Barras o Línea Digitible |
| `scheduled_to` | `string (date)` | no | Fecha de Programación |
| `payment_account_uid` | `string (uuid)` | no | Cuenta de pago de origen (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se proporciona, debe pertenecer a `financial_account_uid`. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |
| `transfer_purpose` | `string` | no | Objetivo del pago. Define el tipo de servicio del lote en la remesa enviada al banco. Opcional. Cuando no se informa, la remesa sale como `98` (Pagos Diversos). Informe solo si el banco exige un tipo de lote específico: Itaú, por ejemplo, rechaza remesas cuyo tipo de lote la empresa no haya contratado. Los pagos con objetivos diferentes se envían en remesas separadas, porque el tipo de servicio describe el lote entero. En pagos, hoy solo Itaú lee este campo en la remesa. En los demás bancos el tipo de lote es fijo: el campo se acepta y se devuelve en la respuesta, pero no altera el archivo enviado. `20`: Pago Proveedor. `98`: Pagos Diversos Valores: `20`, `98`. |

Otros campos y objetos anidados: `kobana schema banking.payment.utilities.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | pago creado |
| `422` | Error de validación |

## payment utilities get

Ver um pago de cuenta de Consumo

```text
kobana banking payment utilities get [UID] [OPCIONES]
```

`GET /v2/payment/utilities/{uid}`

Consulta información específicas de um pago de cuenta de Consumo.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago encontrado |
| `404` | pago en lo encontrado |

## payment utilities cancel

Cancelar um pago de cuenta de Consumo

```text
kobana banking payment utilities cancel [UID] [OPCIONES]
```

`PUT /v2/payment/utilities/{uid}/cancel`

> **Procesamiento Asincrónico:** Este endpoint se procesa de forma asincrónica y crea un comando de cancelación para el Pago.
La cancelación de un Pago se realiza de forma asincrónica, es decir, al recibir la solicitud, nuestra API guarda la solicitud y crea un Comando. Este comando se procesa en segundo plano y después de ser confirmado, el Pago cambia su estado a Cancelado. Recibe en la respuesta de esta solicitud el ID del comando creado y puede consultarlo en cualquier momento.

**Eventos de Webhook**

Cuando se realiza la solicitud, se dispara un evento `payment.utility.cancel.requested`.
Nuestro servidor entonces comienza los intentos de envío de la cancelación a la institución financiera.
Solo después de ser confirmado en la institución financiera, entonces el pago se marca como cancelado en nuestra base de datos.
En este momento se dispara el evento `payment.utility.cancel.confirmed`.

> ⚠️ **Atención al estado Cancelado:** El estado de Cancelado es terminal. El pago no podrá ser procesado después de la cancelación.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | pago cancelado sin interacción con proveedor |
| `422` | El pago ya fue confirmado, rechazado o cancelado |

## payment utility-batches create

Crear un Lote de Pago de Cuentas de Consumo

```text
kobana banking payment utility-batches create [OPCIONES]
```

`POST /v2/payment/utility_batches`

Este endpoint se utiliza para la generación de un lote de pago.

**Requisito previo**

Para emitir un lote de pago, es necesario tener una Cuenta Financiera, informada en `financial_account_uid`. La cuenta de pago aplicada al lote se define en `payment_account_uid`; si se omite, se usa la más antigua vinculada a la cuenta financiera; si se especifica, debe pertenecer a `financial_account_uid`.

**Eventos de Webhook**

El evento `payment.batch.db.created` se dispara en esta operación.

Al registrar un Lote de Pago, el sistema crea un comando para registrarlo en la institución financiera. Después de que el registro del lote sea confirmado en la institución, se dispara el evento `payment.utility_batch.confirmed`.

El evento `payment.db.created` también se dispara en esta operación si el lote contiene un nuevo pago.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `financial_account_uid` | `string (uuid)` | sí | UID de la Cuenta Financiera de Origen |
| `payments` | `array<anyOf>` | sí | Lista de Pagos de Cuentas de Consumo nuevas o existentes. |
| `payment_account_uid` | `string (uuid)` | no | Cuenta de pago aplicada a todo el lote (opcional). Si se omite, se usa la más antigua de la cuenta financiera. Si se proporciona, debe pertenecer a la cuenta financiera del lote. |

Otros campos y objetos anidados: `kobana schema banking.payment.utility-batches.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Lote de pago creado |
| `422` | Error de validación — external_id duplicado |
