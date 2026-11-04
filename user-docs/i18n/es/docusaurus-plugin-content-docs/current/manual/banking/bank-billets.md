---
id: bank-billets
title: "kobana banking bank-billets"
sidebar_label: "bank-billets"
---

# kobana banking bank-billets


Comandos de `kobana banking bank-billets`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`bank-billets protest update`](#bank-billets-protest-update) | `PUT /v1/bank_billets/{id}/protest/{type}` | Protestar Boleto |
| [`bank-billets list`](#bank-billets-list) | `GET /v1/bank_billets` | Listar Boletos |
| [`bank-billets create`](#bank-billets-create) | `POST /v1/bank_billets` | Crear un Boleto |
| [`bank-billets cancel-all`](#bank-billets-cancel-all) | `POST /v1/bank_billets/cancel_all` | Cancelar Boletos en Lote |
| [`bank-billets get`](#bank-billets-get) | `GET /v1/bank_billets/{id}` | Ver el Boleto |
| [`bank-billets update`](#bank-billets-update) | `PUT /v1/bank_billets/{id}` | Actualizar el Boleto |
| [`bank-billets cancel`](#bank-billets-cancel) | `PUT /v1/bank_billets/{id}/cancel` | Cancelar el Boleto |
| [`bank-billets duplicate`](#bank-billets-duplicate) | `POST /v1/bank_billets/{id}/duplicate` | Duplicar Boleto |
| [`bank-billets pay`](#bank-billets-pay) | `PUT /v1/bank_billets/{id}/pay` | Marcar Boleto Como Pagado |
| [`bank-billets send-email`](#bank-billets-send-email) | `POST /v1/bank_billets/{id}/send_email` | Enviar Boleto por Correo Electrónico |
| [`bank-billets send-sms`](#bank-billets-send-sms) | `POST /v1/bank_billets/{id}/send_sms` | Enviar Boleto por SMS |

## bank-billets protest update

Protestar Boleto

```text
kobana banking bank-billets protest update [ID] [TYPE] [OPCIONES]
```

`PUT /v1/bank_billets/{id}/protest/{type}`

Protesta o cancela protesta de un boleto vencido. El parámetro :type en la ruta es obligatorio: use "send" para protestar el boleto o "cancel" para cancelar un protesta existente.

**Argumentos de la ruta:** `id`, `type`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Boleto protestado con éxito |
| `404` | Boleto no encontrado |
| `422` | Tipo de protesta inválido |

## bank-billets list

Listar Boletos

```text
kobana banking bank-billets list [OPCIONES]
```

`GET /v1/bank_billets`

Listar boletos.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `bank_billet_account_id` | `string` | no | Filtro por ID de la Cartera. Encuentra todos los registros que tengan el ID. |
| `status` | `string` | no | Filtro por Situación. Encuentra todos los registros con el estado. Valores: `generating`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `our_number` | `string` | no | Filtro por Nuestro Número |
| `processed_our_number_raw` | `string` | no | Filtro por Nuestro Número con DV (limpio). |
| `cnpj_cpf` | `string` | no | Filtro por CPF/CNPJ del cliente. Debe ser formateado con máscara. Encuentra todos los registros cuyo campo sea igual a la cadena. |
| `created_from` | `string` | no | Fecha inicial del filtro de fecha de creación del registro. |
| `created_to` | `string` | no | Fecha final del filtro de fecha de creación del registro. |
| `expire_from` | `string` | no | Fecha inicial del filtro de fecha de vencimiento |
| `expire_to` | `string` | no | Fecha final del filtro de fecha de vencimiento. |
| `paid_from` | `string` | no | Fecha inicial del filtro de fecha de pago. |
| `paid_to` | `string` | no | Fecha final del filtro de fecha de pago. |
| `registered_to` | `string` | no | Fecha inicial del filtro de fecha de registro. |
| `registered_from` | `string` | no | Fecha final del filtro de fecha de registro. |
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `updated_from` | `string` | no | Fecha inicial del filtro de fecha de actualización del registro. |
| `updated_to` | `string` | no | Fecha final del filtro de fecha de actualización del registro. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de boletos |

## bank-billets create

Crear un Boleto

```text
kobana banking bank-billets create [OPCIONES]
```

`POST /v1/bank_billets`

Crea un nuevo boleto bancario para cobro. El boleto será procesado y registrado en la institución financiera de acuerdo con la configuración de la cartera de cobro utilizada.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | sí | Cantidad |
| `expire_at` | `string (date)` | sí | Fecha de vencimiento |
| `customer_person_name` | `string` | sí | Nombre del cliente |
| `customer_cnpj_cpf` | `string` | sí | CPF/CNPJ del cliente |
| `customer_state` | `string` | sí | Estado |
| `customer_city_name` | `string` | sí | Ciudad (El nombre debe ser correcto y completo) |
| `customer_zipcode` | `string` | sí | CEP (formato 99999999) |
| `customer_address` | `string` | sí | Dirección |
| `customer_neighborhood` | `string` | sí | Barrio |
| `id` | `integer` | no | ID de boleto |
| `bank_billet_account_id` | `integer` | no | ID de la Cartera de Cobranza. Si no se proporciona, usará la cartera predeterminada. |
| `bank_billet_layout_id` | `integer` | no | ID del Modelo de boleto |
| `customer_id` | `integer` | no | ID del Cliente. Cuando se pasa este ID, los campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` y `customer_neighborhood` no son obligatorios. |
| `customer_address_complement` | `string` | no | Complemento |
| `customer_address_number` | `string` | no | Número |
| `customer_email` | `string (email)` | no | Correo electrónico del Pagador |
| `customer_email_cc` | `string (email)` | no | Correo electrónico alternativo del Pagador |
| `customer_phone_number` | `string` | no | Teléfono (con DDD, DDI es opcional) |
| `customer_ignore_email` | `boolean` | no | Nunca enviar correo electrónico a este cliente |
| `customer_ignore_sms` | `boolean` | no | Nunca enviar SMS a este cliente |
| `customer_mobile_local_code` | `string` | no | DDD del Celular |
| `customer_mobile_number` | `string` | no | Celular |
| `customer_nickname` | `string` | no | Apodo o Nombre Comercial del Pagador |
| `customer_notes` | `string` | no | Observaciones del Pagador |
| `customer_contact_person` | `string` | no | Contacto |
| `interest_type` | `integer` | no | Tipo de interés/mora: * `0` Inexistente (Predeterminado) * `1` Para porcentaje diario * `2` Para valor diario * `7` Para porcentaje mensual - Bancos compatibles: Bradesco, BB, BTG, Caixa, Inter, Itaú, Safra, Santander, Sicoob y Sicredi Valores: `0`, `1`, `2`, `7`. |
| `days_for_interest` | `integer` | no | Cantidad de días después del vencimiento cuando la mora comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `interest_percentage` | `number` | no | Porcentaje de interés. De 0.0 a 100.0 (Ej 1.5% = 1.5) Obligatorio si interest_type es igual a 1 o 7. Hasta 2 decimales. |
| `interest_value` | `number` | no | Valor diario de interés (R$). Obligatorio si interest_type es igual a 2. Hasta 2 decimales. |
| `interest_days_type` | `integer` | no | Tipo de Días para interés: * `0` Corridos * `1` Hábiles Valores: `0`, `1`. |
| `fine_type` | `integer` | no | Tipo de multa: * `0` Inexistente (Predeterminado) * `1` Para porcentaje del valor de boleto * `2` Para valor fijo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Cantidad de días después del vencimiento cuando la multa comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `fine_percentage` | `number` | no | Porcentaje de Multa por Atraso Ej: 2% x R$ 250,00 = R$ 5,00. Obligatoria si fine_type es igual a 1. Hasta 2 decimales. |
| `fine_value` | `number` | no | Valor de la multa (R$). Obligatorio si fine_type es igual a 2. Hasta 2 decimales. |
| `discount_type` | `integer` | no | Tipo de descuento: El tipo de descuento será el mismo para los tres descuentos, si existen. : * `0` Inexistente (Predeterminado) * `1` Para valor fijo * `2` Para porcentaje del valor de boleto Valores: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | no | Días para descuento. Obligatorio si discount_type es diferente de 0 (cero) |
| `discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al descuento. Obligatorio si discount_type es igual a 2 |
| `discount_value` | `number` | no | Valor del descuento (R$). Obligatorio si discount_type es igual a 1. |
| `days_for_second_discount` | `integer` | no | Días para segundo descuento. |
| `second_discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al segundo descuento. |
| `second_discount_value` | `number` | no | Valor del segundo descuento (R$). |
| `days_for_third_discount` | `integer` | no | Días para tercer descuento. |
| `third_discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al tercer descuento. |
| `third_discount_value` | `number` | no | Valor del tercer descuento (R$). |
| `tags` | `array<string>` | no | Etiquetas asociadas al boleto |
| `tag_list` | `string` | no | Etiquetas asociadas al boleto |
| `charge_type` | `integer` | no | Tipo de Cobranza: * `1` Simple * `2` Vinculada * `3` Descontada * `4` Vendor Valores: `1`, `2`, `3`, `4`. |
| `dispatch_type` | `integer` | no | Tipo de Cobranza: Cuando el boleto necesita ser enviado por correo. Es necesario contratar el servicio con el banco y pagará una tarifa. * `1` Cliente * `2` Banco Valores: `1`, `2`. |
| `guarantor_name` | `string` | no | Nombre del Beneficiario final (Sacador/Avalista) |
| `guarantor_cnpj_cpf` | `string` | no | CNPJ/CPF del Beneficiario final (Sacador/Avalista) |
| `guarantor_address_number` | `string` | no | Número del Beneficiario final (Sacador/Avalista) |
| `guarantor_neighborhood` | `string` | no | Barrio del Beneficiario final (Sacador/Avalista) |
| `guarantor_phone_number` | `string` | no | Teléfono (con DDD) del Beneficiario final (Sacador/Avalista) |
| `guarantor_city_name` | `string` | no | Ciudad (El nombre debe ser correcto y completo) del Beneficiario final (Sacador/Avalista) |
| `guarantor_state` | `string` | no | Estado del Beneficiario final (Sacador/Avalista) |
| `guarantor_zipcode` | `string` | no | CEP (formato 99999999) del Beneficiario final (Sacador/Avalista) |
| `guarantor_address` | `string` | no | Dirección del Beneficiario final (Sacador/Avalista) |
| `guarantor_address_complement` | `string` | no | Complemento del Beneficiario final (Sacador/Avalista) |
| `description` | `string` | no | Descripción del producto o servicio |
| `instructions` | `string` | no | Instrucciones para el Cajero |
| `document_date` | `string (date)` | no | Fecha del Documento |
| `document_type` | `string` | no | Tipo de Documento: * `Código` \| `Sigla` \| Descripción * `01` \| `CH` \| Cheque * `02` \| `DM` \| Duplicata Mercantil (Predeterminado) * `03` \| `DMI` \| Duplicata Mercantil p/ Indicación * `04` \| `DS` \| Duplicata de Servicio * `05` \| `DSI` \| Duplicata de Servicio p/ Indicación * `06` \| `DR` \| Duplicata Rural * `07` \| `LC` \| Letra de Cambio * `08` \| `NCC` \| Nota de Crédito Comercial * `09` \| `NCE` \| Nota de Crédito a Exportación * `10` \| `NCI` \| Nota de Crédito Industrial * `11` \| `NCR` \| Nota de Crédito Rural * `12` \| `NP` \| Nota Promissória * `13` \| `NPR` \| Nota Promissória Rural * `14` \| `TM` \| Triplicata Mercantil * `15` \| `TS` \| Triplicata de Servicio * `16` \| `NS` \| Nota de Seguro * `17` \| `RC` \| Recibo * `18` \| `FAT` \| Factura * `19` \| `ND` \| Nota de Débito * `20` \| `AP` \| Póliza de Seguro * `21` \| `ME` \| Mensualidad Escolar * `22` \| `PC` \| Parcela de Consorcio * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Documento de Deuda * `25` \| `CPR` \| Cédula de Producto Rural * `26` \| `CTR` \| Contrato * `27` \| `CSG` \| Coaseguros * `28` \| `EC` \| Gastos Condominiales * `29` \| `CPS` \| Cuenta de Prestación de Servicios * `30` \| `WR` \| Warrant * `31` \| `DP` \| Duplicata Prestación * `32` \| `CSR` \| Cobranza Seriada * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Póliza Ramos Elementales * `35` \| `CC` \| Tarjeta de Crédito * `36` \| `BDP` \| Boleto de Propuesta * `37` \| `NPD` \| Nota Promissória Directa * `38` \| `DAE` \| Deuda Activa de Estado * `39` \| `DAM` \| Deuda Activa de Municipio * `40` \| `DAU` \| Deuda Activa Unión * `41` \| `CCB` \| Célula de Crédito Bancario * `42` \| `FI` \| Financiamiento * `43` \| `RD` \| Prorrateo de Gastos * `44` \| `DRI` \| Duplicata Rural p/ Indicación * `45` \| `ECI` \| Gastos Condominiales p/ Indicación * `99` \| `Otros` \| Otros Valores: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `document_type_label` | `string` | no | Tipo de Documento (Sigla) |
| `document_number` | `string` | no | Número del Documento, El tamaño depende del banco y formato del archivo Remesa: * Banco \|	Formato\| Tamaño \| * Abc \|`Cnab240`\|	15	\| * Ailos \|`Cnab240`\|	15  \| * Arbi \|`Cnab240`\|	15	\| * Banese \|`Cnab240`\|	15	\| * Banestes \|`Cnab400`\|	10	\| * Banrisul \|`Cnab240`\|	15	\| * Banrisul \|`Cnab400`\|	10	\| * Bb \|`Cnab240`\|	15	\| * Bb \|`Cnab400`\|	10	\| * Bib \|`Cnab240`\|	15	\| * Bnb \|`Cnab400`\|	10	\| * Bnpparibas \|`Cnab400`\|	10	\| * Bradesco \|`Cnab240`\|	15	\| * Bradesco \|`Cnab400`\|	10	\| * Brb \|`Cnab400`\|	10	\| * Caixa \|`Cnab240`\|	11	\| * Caixa \|`Cnab400`\|	10	\| * Caruana \|`Cnab400`\|	10	\| * Citibank \|`Cnab400`\|	10	\| * Credisis \|`Cnab240`\|	15	\| * Cresol \|`Cnab240`\|	10	\| * Cresol \|`Cnab400`\|	10	\| * Cresol Bradesco \|`Cnab240`\|	15	\| * Cresol Bradesco \|`Cnab400`\|	10	\| * Daycoval \|`Cnab400`\|	10	\| * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| * Mercantil \|`Cnab240`\|	10	\| * Moneyplus \|`Cnab240`\|	10	\| * Rendimiento \|`Cnab400`\|	10	\| * Safra \|`Cnab400`\|	102	\| * Santander \|`Cnab240`\|	15	\| * Santander \|`Cnab400`\|	10	\| * Semear \|`Cnab400`\|	10	\| * Sicoob \|`Cnab240`\|	15	\| * Sicoob \|`Cnab400`\|	10	\| * Sicredi \|`Cnab240`\|	15	\| * Sicredi \|`Cnab400`\|	10	\| * Sofisa \|`Cnab240`\|	15	\| * Unicred \|`Cnab240`\|	15	\| * Uniprime \|`Cnab400`\|	10	\| * Uniprime99 \|`Cnab400`\|	10	\| * Santander \|`Cnab400`\|	10 |
| `acceptance` | `string` | no | Aceptación: * `N` No (Predeterminado) * `S` Sí Valores: `N`, `S`. |
| `our_number` | `string` | no | Nuestro Número. Si no se proporciona, usará el Siguiente Nuestro Número de la Cartera de Cobranza. |
| `processed_our_number` | `string` | no | Nuestro Número calculado con DV (formateado) |
| `processed_our_number_raw` | `string` | no | Nuestro Número calculado con DV (limpio) |
| `paid_amount` | `number` | no | Valor pagado |
| `paid_at` | `string (date)` | no | Fecha del pago |
| `days_for_revoke` | `integer` | no | Días corridos para Baja/Devolución: Nulo/En blanco: Obedece al estándar del banco. 0: Baja/Devolución el mismo día del vencimiento. 1 o más: Baja/Devolución después del vencimiento (Vencimiento + X días corridos). |
| `credit_bureau` | `integer` | no | Buró de Crédito/Órgano Negativador. `0`: Serasa. `1`: Quod. `2`: SPC Opciones disponibles para cada banco compatible: * Banco do Brasil: Serasa y Quod. Valores: `null`, `0`, `1`, `2`. |
| `days_for_negativation` | `integer` | no | Días corridos para Negativación: Disponible solo para los siguientes bancos y formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sí \|	Sí	\| No * Itaú \| No \| Sí \| No |
| `days_for_sue_type` | `integer` | no | Tipo de días para protesto: * `0` Corridos * `1` Hábiles Valores: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | no | Días corridos para Protesto |
| `sue_history` | `array<object>` | no | Historial de eventos de protesto de boleto |
| `sue_code` | `string` | no | Código de Protesto (CNAB 240). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `revoke_code` | `string` | no | Código de Baja (CNAB 240). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `first_instruction` | `string` | no | Primera Instrucción (CNAB 400). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `second_instruction` | `string` | no | Segunda Instrucción (CNAB 400). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `watermark` | `boolean` | no | Dirección |
| `payment_count` | `integer` | no | Cantidad de pagos parciales aceptados para este boleto. |
| `divergent_payment_type` | `integer` | no | Tipo de pago divergente: Válido solo para Itaú y Caixa. * `1` Acepta cualquier valor divergente * `2` Acepta pagos dentro de un rango de valores o porcentajes * `3` No acepta pago de valores divergentes * `4` Acepta pagos de valores superiores a un valor o porcentaje mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Tipo de valor a considerar para los límites de pagos: Válido solo para Itaú y Caixa. * `1` Informa pagos divergentes por valores * `2` Informa pagos divergentes por porcentajes Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Valor máximo para el rango de pagos divergentes. |
| `divergent_payment_minimum_value` | `number` | no | Valor mínimo para el rango de pagos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | no | Porcentaje máximo para el rango de pagos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | no | Porcentaje mínimo para el rango de pagos divergentes. |
| `divergent_payment_limit` | `integer` | no | Cantidad de pagos permitida. Obligatorio si se proporcionan datos para pago divergente. Usado solo por Caixa. |
| `prevent_registration` | `boolean` | no | Prevenir envío de registro al banco: Si `true`, impide que el boleto sea registrado. Para ser usado en los casos en que el boleto ya fue registrado fuera de KOBANA pero desea incluirse en el sistema. |
| `issued_at` | `string (date-time)` | no | Fecha de emisión de boleto. Aceptado solo cuando `prevent_registration: true`. |
| `control_number` | `string` | no | Número de control: Puede contener cualquier información de interés de la Empresa. La información contenida en este campo siempre se devolverá con el título respectivo en el archivo de retorno. |
| `ignore_email` | `boolean` | no | No enviar este boleto por correo electrónico |
| `ignore_sms` | `boolean` | no | Nunca enviar este boleto por SMS |
| `ignore_whatsapp` | `boolean` | no | Nunca enviar este boleto por WhatsApp |
| `addons` | `string (jsonb)` | no | Dirección |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `meta` | `object` | no | Hash con clave y valor en formato JSON. |
| `notes` | `string` | no | Observaciones |
| `custom_attachment_name` | `string` | no | Nombre para ser usado en los archivos de boleto enviados al cliente en notificaciones. Acepta uso de variables. Si se deja en blanco, el estándar es la palabra "boleto" acompañada del ID. |
| `split_payment` | `boolean` | no | Split de Pago. Válido solo para ABC Brasil, Bradesco e Itaú. Si es true, el prorrateo del boleto será registrado. Informe las cuentas para prorrateo en `split_accounts`. Para Itaú, es necesario informar el tipo de split de pago en `split_payment_type` |
| `split_payment_type` | `integer` | no | Tipo de Split de Pago: Válido solo para Itau. Usado solo con Split de Pago true. * `1` Prorrateo de crédito por porcentaje (%) – Valor nominal del título * `2` Prorrateo de crédito en valor (R$) – Valor nominal del título * `3` Prorrateo de crédito por porcentaje (%) – Valor neto recibido * `4` Prorrateo de crédito en valor (R$) – Valor neto recibido, prorrateado proporcionalmente Valores: `1`, `2`, `3`, `4`. |
| `split_accounts` | `array<object>` | no | Cuentas para Split de pago. |
| `payment_place` | `string` | no | Lugar de Pago |
| `installment_id` | `integer` | no | ID del Carné |
| `installment_number` | `integer` | no | Número de la parcela del carnê |
| `installment_total` | `integer` | no | Total de parcelas del carnê |
| `customer_subscription_id` | `integer` | no | ID de la Suscripción |
| `beneficiary_name` | `string` | no | Nombre del Beneficiario |
| `beneficiary_cnpj_cpf` | `string` | no | CNPJ/CPF del Beneficiario |
| `beneficiary_address` | `string` | no | Dirección del Beneficiario |
| `beneficiary_assignor_code` | `string` | no | Agencia/Código del Beneficiario |
| `bank_contract_slug` | `string` | no | Slug de la Cartera |
| `agency_number` | `string` | no | Agencia |
| `agency_digit` | `string` | no | Dígito de la Agencia |
| `account_number` | `string` | no | Cuenta |
| `account_digit` | `string` | no | Dígito de la Cuenta |
| `extra1` | `string` | no | Campo extra 1 |
| `extra1_digit` | `string` | no | Dígito del Campo extra 1 |
| `extra2` | `string` | no | Campo extra 2 |
| `extra2_digit` | `string` | no | Dígito del Campo extra 2 |
| `created_via_api` | `boolean` | no | Indica si el boleto fue creado por API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del boleto |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del boleto |
| `registration_status` | `string` | no | Situación del registro en el banco: * `pending` Pendiente * `skipped` Ignorado * `requested` Requisitado * `confirmed` Confirmado * `rejected` Rechazado (aún se intentará nuevamente) * `failed` Fallo (no se intentará nuevamente) Valores: `pending`, `skipped`, `requested`, `confirmed`, `rejected`, `failed`. |
| `registered_at` | `string (date-time)` | no | Fecha y hora del registro (cuando se confirma) |
| `register_type` | `integer` | no | Tipo de Registro: * `1` API * `2` Banco Valores: `1`, `2`. |
| `cancel_type` | `integer` | no | Tipo de Cancelación: * `1` Cliente * `2` Banco Valores: `null`, `1`, `2`. |
| `cancellation_reason` | `integer` | no | Motivo de Cancelación: * `1` Otro * `2` Fraude * `3` Muerte * `4` Error operacional * `5` Pago liquidado Valores: `null`, `1`, `2`, `3`, `4`, `5`. |
| `line` | `string` | no | Línea Digitalizada |
| `barcode` | `string` | no | Código de Barras |
| `shorten_url` | `string` | no | URL para visualización del boleto |
| `url` | `string` | no | URL para visualización del boleto |
| `carne_url` | `string` | no | URL para visualización del carnê (Cuando sea parcela) |
| `formats` | `object` | no | URLs con formatos disponibles. Ej.: PDF, Imagen, Pix, etc. |
| `pix_enabled` | `boolean` | no | Indica si el boleto es híbrido y tiene QRcode Pix |
| `pix_qrcode` | `string` | no | QRcode Pix del boleto híbrido |
| `pix_txid` | `string` | no | Campo txid del Pix. Generado automáticamente por defecto si no se proporciona. |
| `prevent_pix` | `boolean` | no | Si es verdadero, impide la creación del Pix para carteras con Pix habilitado. No se considera si la cartera no tiene Pix habilitado. |
| `status` | `string` | no | Situación del boleto: * `generating` Generando * `draft` Borrador * `generation_failed` Error al generar * `opened` Abierto * `canceled` Cancelado * `paid` Pagado * `overdue` Vencido * `validation_failed` Inválido * `chargeback` Revertido Valores: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `recipient_account` | `string` | no | Cuenta Destinataria + Dígito |
| `reduction_type` | `integer` | no | Tipo de abatimiento: `1`: Valor. `2`: Porcentaje. `3`: Sin abatimiento Valores: `1`, `2`, `3`. |
| `reduction_amount` | `number` | no | Valor del abatimiento. Obligatorio si reduction_type es igual a 1. |
| `reduction_percentage` | `number` | no | Porcentaje de Abatimiento. Ej: 2% x R$ 250,00 = R$ 5,00. Obligatorio si reduction_type es igual a 2. Hasta 2 decimales. |
| `instructions_mode` | `integer` | no | Cálculo de fechas en la Instrucción para el Cajero: * `0` No completar las instrucciones para el cajero * `1` Calcular fecha por la vía de registro (API/CNAB) * `2` Usar la fecha configurada en el boleto Valores: `0`, `1`, `2`. |
| `import_id` | `integer` | no | ID de la Importación |
| `virtual_bank_billet_id` | `integer` | no | ID de Boleto generado por miembro de contrato BackOffice. (BackOffice debe estar habilitado). |
| `external_id` | `string` | no | ID del boleto en el sistema del cliente. Opcional para control y búsqueda interna. |
| `financial_provider_external_id` | `string` | no | ID en la institución financiera |
| `after_create` | `array<string>` | no | Ejecución automática de comandos después de que se cree el boleto. Valores permitidos: * `sync`: Sincronización con el proveedor financiero. |

Otros campos y objetos anidados: `kobana schema banking.bank-billets.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Boleto creado |
| `422` | Pago parcial de Itaú requiere tipo de autorización específico |

## bank-billets cancel-all

Cancelar Boletos en Lote

```text
kobana banking bank-billets cancel-all [OPCIONES]
```

`POST /v1/bank_billets/cancel_all`

Cancelar boletos en lote

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `status` | `string` | no | Situación del boleto: * `generating` Generando * `draft` Borrador * `generation_failed` Error al generar * `opened` Abierto * `canceled` Cancelado * `paid` Pagado * `overdue` Vencido * `validation_failed` Inválido * `chargeback` Revertido Valores: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `expire_from` | `string (date)` | no | A partir de Fecha de vencimiento |
| `expire_to` | `string (date)` | no | Hasta la Fecha de vencimiento |
| `cnpj_cpf` | `string` | no | CNPJ o CPF del Pagador |
| `bank_billet_ids` | `array<integer>` | no | IDs de boletos a ser cancelados |

Otros campos y objetos anidados: `kobana schema banking.bank-billets.cancel-all`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Solicitud realizada con éxito |
| `422` | Entidad No Procesable |

## bank-billets get

Ver el Boleto

```text
kobana banking bank-billets get [ID] [OPCIONES]
```

`GET /v1/bank_billets/{id}`

Consulta información específica de un Boleto.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Boleto encontrado |
| `404` | Boleto no encontrado |

## bank-billets update

Actualizar el Boleto

```text
kobana banking bank-billets update [ID] [OPCIONES]
```

`PUT /v1/bank_billets/{id}`

Actualiza información específica de un Boleto.

Puede cambiar boletos con estado Abierto(`opened`) o Vencido(`overdue`)

Recibirá webhooks `generating` y `opened` para el boleto modificado.

En carteras registradas, el cambio entrará en el envío y se pueden cobrar cargos bancarios.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | no | Valor del Boleto |
| `expire_at` | `string (date)` | no | Fecha de vencimiento |
| `tags` | `array<string>` | no | Etiquetas asociadas al boleto |
| `notes` | `string` | no | Observaciones |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. Puede modificarse en cualquier estado del boleto. |
| `days_for_sue_type` | `integer` | no | Tipo de días para protesto: * `0` Corridos * `1` Hábiles Valores: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | no | Días corridos para Protesto |
| `days_for_revoke` | `integer` | no | Días corridos para Baja Bancos Compatibles: Itaú |
| `sue_code` | `string` | no | Código de Protesto (Solo por CNAB 240). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `instructions` | `string` | no | Instrucciones para el Cajero |
| `description` | `string` | no | Descripción del producto o servicio |
| `interest_type` | `integer` | no | Tipo de interés/mora: * `0` Inexistente (Predeterminado) * `1` Para porcentaje diario * `2` Para valor diario * `7` Para porcentaje mensual - Bancos compatibles: Bradesco, BB, BTG, Caixa, Inter, Itaú, Safra, Santander, Sicoob y Sicredi Valores: `0`, `1`, `2`, `7`. |
| `days_for_interest` | `integer` | no | Cantidad de días después del vencimiento cuando la mora comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `interest_percentage` | `number` | no | Porcentaje de interés. De 0.0 a 100.0 (Ej 1.5% = 1.5) Obligatorio si interest_type es igual a 1 o 7. Hasta 2 decimales. |
| `interest_value` | `number` | no | Valor diario de interés (R$). Obligatorio si interest_type es igual a 2. Hasta 2 decimales. |
| `discount_type` | `integer` | no | Tipo de descuento: El tipo de descuento será el mismo para los tres descuentos, si existen. : * `0` Inexistente (Predeterminado) * `1` Para valor fijo * `2` Para porcentaje del valor de boleto Valores: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | no | Días para descuento. Obligatorio si discount_type es diferente de 0 (cero) |
| `discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al descuento. Obligatorio si discount_type es igual a 2 |
| `discount_value` | `number` | no | Valor del descuento (R$). Obligatorio si discount_type es igual a 1. |
| `days_for_second_discount` | `integer` | no | Días para segundo descuento. |
| `second_discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al segundo descuento. |
| `second_discount_value` | `number` | no | Valor del segundo descuento (R$). |
| `days_for_third_discount` | `integer` | no | Días para tercer descuento. |
| `third_discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al tercer descuento. |
| `third_discount_value` | `number` | no | Valor del tercer descuento (R$). |
| `reduction_type` | `integer` | no | Tipo de abatimiento: `1`: Valor. `2`: Porcentaje. `3`: Sin abatimiento Valores: `1`, `2`, `3`. |
| `reduction_amount` | `number` | no | Valor del abatimiento. Obligatorio si reduction_type es igual a 1. |
| `reduction_percentage` | `number` | no | Porcentaje de Abatimiento. Ej: 2% x R$ 250,00 = R$ 5,00. Obligatorio si reduction_type es igual a 2. Hasta 2 decimales. |
| `divergent_payment_type` | `integer` | no | Tipo de pago divergente: * `1` Acepta cualquier valor divergente * `2` Acepta pagos dentro de un rango de valores o porcentajes * `3` No acepta pago de valores divergentes * `4` Acepta pagos de valores superiores a un valor o porcentaje mínimo Bancos compatibles: Itaú Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Tipo de valor a considerar para los límites de pagos: Válido solo para Itaú y Caixa. * `1` Informa pagos divergentes por valores * `2` Informa pagos divergentes por porcentajes Bancos compatibles: Itaú Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Valor máximo para el rango de pagos divergentes. Bancos compatibles: Itaú |
| `divergent_payment_minimum_value` | `number` | no | Valor mínimo para el rango de pagos divergentes. Bancos compatibles: Itaú |
| `divergent_payment_maximum_percentage` | `number` | no | Porcentaje máximo para el rango de pagos divergentes. Bancos compatibles: Itaú |
| `divergent_payment_minimum_percentage` | `number` | no | Porcentaje mínimo para el rango de pagos divergentes. Bancos compatibles: Itaú |
| `fine_type` | `integer` | no | Tipo de multa: * `0` Inexistente (Predeterminado) * `1` Para porcentaje del valor de boleto * `2` Para valor fijo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Cantidad de días después del vencimiento cuando la multa comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `credit_bureau` | `integer` | no | Buró de Crédito/Órgano Negativador. `0`: Serasa. `1`: Quod. `2`: SPC Opciones disponibles para cada banco compatible: * Banco do Brasil: Serasa y Quod. Valores: `null`, `0`, `1`, `2`. |
| `days_for_negativation` | `integer` | no | Cantidad de días después del vencimiento para negativar el título. |
| `fine_percentage` | `number` | no | Porcentaje de Multa por Atraso Ej: 2% x R$ 250,00 = R$ 5,00. Obligatoria si fine_type es igual a 1. Hasta 2 decimales. |
| `fine_value` | `number` | no | Valor de la multa (R$). Obligatorio si fine_type es igual a 2. Hasta 2 decimales. |
| `document_number` | `string` | no | Número del Documento, El tamaño depende del banco, formato y forma de envío (API / EDI): * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| |
| `customer_person_name` | `string` | no | Nombre del cliente. Bancos Compatibles: Santander |
| `customer_cnpj_cpf` | `string` | no | CPF/CNPJ del cliente. Bancos Compatibles: Santander |
| `customer_state` | `string` | no | Estado. Bancos Compatibles: Santander |
| `customer_city_name` | `string` | no | Ciudad (El nombre debe ser correcto y completo). Bancos Compatibles: Santander |
| `customer_zipcode` | `string` | no | CEP (formato 99999999). Bancos Compatibles: Santander |
| `customer_address` | `string` | no | Dirección. Bancos Compatibles: Santander |
| `customer_neighborhood` | `string` | no | Barrio. Bancos Compatibles: Santander |
| `guarantor_name` | `string` | no | Nombre del Beneficiario final (Sacador/Avalista). Bancos Compatibles: Santander |
| `guarantor_cnpj_cpf` | `string` | no | CNPJ/CPF del Beneficiario final (Sacador/Avalista). Bancos Compatibles: Santander |
| `document_type` | `string` | no | Tipo de Documento: * `Código` \| `Sigla` \| Descripción * `01` \| `CH` \| Cheque * `02` \| `DM` \| Duplicata Mercantil (Predeterminado) * `03` \| `DMI` \| Duplicata Mercantil p/ Indicación * `04` \| `DS` \| Duplicata de Servicio * `05` \| `DSI` \| Duplicata de Servicio p/ Indicación * `06` \| `DR` \| Duplicata Rural * `07` \| `LC` \| Letra de Cambio * `08` \| `NCC` \| Nota de Crédito Comercial * `09` \| `NCE` \| Nota de Crédito a Exportación * `10` \| `NCI` \| Nota de Crédito Industrial * `11` \| `NCR` \| Nota de Crédito Rural * `12` \| `NP` \| Nota Promissória * `13` \| `NPR` \| Nota Promissória Rural * `14` \| `TM` \| Triplicata Mercantil * `15` \| `TS` \| Triplicata de Servicio * `16` \| `NS` \| Nota de Seguro * `17` \| `RC` \| Recibo * `18` \| `FAT` \| Factura * `19` \| `ND` \| Nota de Débito * `20` \| `AP` \| Póliza de Seguro * `21` \| `ME` \| Mensualidad Escolar * `22` \| `PC` \| Parcela de Consorcio * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Documento de Deuda * `25` \| `CPR` \| Cédula de Producto Rural * `26` \| `CTR` \| Contrato * `27` \| `CSG` \| Coaseguros * `28` \| `EC` \| Gastos Condominiales * `29` \| `CPS` \| Cuenta de Prestación de Servicios * `30` \| `WR` \| Warrant * `31` \| `DP` \| Duplicata Prestación * `32` \| `CSR` \| Cobranza Seriada * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Póliza Ramos Elementales * `35` \| `CC` \| Tarjeta de Crédito * `36` \| `BDP` \| Boleto de Propuesta * `37` \| `NPD` \| Nota Promissória Directa * `38` \| `DAE` \| Deuda Activa de Estado * `39` \| `DAM` \| Deuda Activa de Municipio * `40` \| `DAU` \| Deuda Activa Unión * `41` \| `CCB` \| Célula de Crédito Bancario * `42` \| `FI` \| Financiamiento * `43` \| `RD` \| Prorrateo de Gastos * `44` \| `DRI` \| Duplicata Rural p/ Indicación * `45` \| `ECI` \| Gastos Condominiales p/ Indicación * `99` \| `Otros` \| Otros Bancos Compatibles: Santander Valores: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `control_number` | `string` | no | Número de control: Puede contener cualquier información de interés de la Empresa. La información contenida en este campo siempre se devolverá con el título respectivo en el archivo de retorno. Bancos Compatibles: Santander |
| `payment_count` | `integer` | no | Cantidad de pagos parciales aceptados para este boleto. Bancos Compatibles: Santander |
| `issued_at` | `string (date)` | no | Fecha de emisión. Bancos Compatibles: Santander |
| `bolepix_key` | `string` | no | Clave del bolepix. Bancos Compatibles: Santander |
| `pix_txid` | `string` | no | TxId del pix. Bancos Compatibles: Santander |

Otros campos y objetos anidados: `kobana schema banking.bank-billets.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Boleto actualizado |
| `404` | Boleto no encontrado |
| `422` | Campos no soportados por el banco |

## bank-billets cancel

Cancelar el Boleto

```text
kobana banking bank-billets cancel [ID] [OPCIONES]
```

`PUT /v1/bank_billets/{id}/cancel`

Cancela boletos con estado Abierto (opened) o Vencido (overdue)

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cancellation_reason` | `integer` | no | Motivo de la cancelación: * `1` Otro * `2` Fraude * `3` Muerte * `4` Error operacional * `5` Pago liquidado Valores: `1`, `2`, `3`, `4`, `5`. |

Otros campos y objetos anidados: `kobana schema banking.bank-billets.cancel`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Boleto cancelado |
| `404` | Boleto no encontrado |
| `422` | Boleto no puede ser cancelado |

## bank-billets duplicate

Duplicar Boleto

```text
kobana banking bank-billets duplicate [ID] [OPCIONES]
```

`POST /v1/bank_billets/{id}/duplicate`

Duplica boleto permitiendo cambiar la información del nuevo boleto o cancelar el boleto duplicado

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `expire_at_in_days` | `integer` | no | Nº de días para vencimiento a partir de hoy. Predeterminado: 7 |
| `amount` | `number` | no | Valor del nuevo Boleto |
| `cancel` | `boolean` | no | Cancelar el boleto que está siendo duplicado. Predeterminado: true |
| `with_fines` | `boolean` | no | Actualizar el valor del nuevo boleto con interés y multa. Predeterminado: false |
| `bank_billet_account_id` | `integer` | no | ID de la Cartera de Cobranza. Si no se proporciona, usará la cartera predeterminada. |
| `bank_billet_layout_id` | `integer` | no | ID del Modelo de boleto |
| `customer_id` | `integer` | no | ID del Cliente. Cuando se pasa este ID, los campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` y `customer_neighborhood` no son obligatorios. |
| `customer_person_name` | `string` | no | Nombre del cliente |
| `customer_cnpj_cpf` | `string` | no | CPF/CNPJ del cliente |
| `customer_state` | `string` | no | Estado |
| `customer_city_name` | `string` | no | Ciudad (El nombre debe ser correcto y completo) |
| `customer_zipcode` | `string` | no | CEP (formato 99999999) |
| `customer_address` | `string` | no | Dirección |
| `customer_address_complement` | `string` | no | Complemento |
| `customer_address_number` | `string` | no | Número |
| `customer_email` | `string (email)` | no | Correo electrónico del Pagador |
| `customer_email_cc` | `string (email)` | no | Correo electrónico alternativo del Pagador |
| `customer_neighborhood` | `string` | no | Barrio |
| `customer_phone_number` | `string` | no | Teléfono (con DDD, DDI es opcional) |
| `customer_ignore_email` | `boolean` | no | Nunca enviar correo electrónico a este cliente |
| `customer_ignore_sms` | `boolean` | no | Nunca enviar SMS a este cliente |
| `customer_mobile_local_code` | `string` | no | DDD del Celular |
| `customer_mobile_number` | `string` | no | Celular |
| `customer_nickname` | `string` | no | Apodo o Nombre Comercial del Pagador |
| `customer_notes` | `string` | no | Observaciones del Pagador |
| `customer_contact_person` | `string` | no | Contacto |
| `interest_type` | `integer` | no | Tipo de interés/mora: * `0` Inexistente (Predeterminado) * `1` Para porcentaje diario * `2` Para valor diario Valores: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | no | Cantidad de días después del vencimiento cuando la mora comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `interest_percentage` | `number` | no | Porcentaje diario de interés. De 0.0 a 100.0 (Ej 1.5% = 1.5) Obligatorio si interest_type es igual a 1. |
| `interest_value` | `number` | no | Valor diario de interés (R$). Obligatorio si interest_type es igual a 2. |
| `interest_days_type` | `integer` | no | Tipo de Días para interés: * `0` Corridos * `1` Hábiles Valores: `0`, `1`. |
| `fine_type` | `integer` | no | Tipo de multa: * `0` Inexistente (Predeterminado) * `1` Para porcentaje del valor de boleto * `2` Para valor fijo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Cantidad de días después del vencimiento cuando la multa comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `fine_percentage` | `number` | no | Porcentaje de Multa por Atraso Ej: 2% x R$ 250,00 = R$ 5,00. Obligatorio si fine_type es igual a 1 |
| `fine_value` | `number` | no | Valor de la multa (R$). Obligatorio si fine_type es igual a 2. |
| `discount_type` | `integer` | no | Tipo de descuento: El tipo de descuento será el mismo para los tres descuentos, si existen. : * `0` Inexistente (Predeterminado) * `1` Para valor fijo * `2` Para porcentaje del valor de boleto Valores: `0`, `1`, `2`. |
| `days_for_discount` | `integer` | no | Días para descuento. Obligatorio si discount_type es diferente de 0 (cero) |
| `discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al descuento. Obligatorio si discount_type es igual a 2 |
| `discount_value` | `number` | no | Valor del descuento (R$). Obligatorio si discount_type es igual a 1. |
| `days_for_second_discount` | `integer` | no | Días para segundo descuento. |
| `second_discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al segundo descuento. |
| `second_discount_value` | `number` | no | Valor del segundo descuento (R$). |
| `days_for_third_discount` | `integer` | no | Días para tercer descuento. |
| `third_discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al tercer descuento. |
| `third_discount_value` | `number` | no | Valor del tercer descuento (R$). |
| `tags` | `array<string>` | no | Etiquetas asociadas al boleto |
| `tag_list` | `string` | no | Etiquetas asociadas al boleto |
| `charge_type` | `integer` | no | Tipo de Cobranza: * `1` Simple * `2` Vinculada * `3` Descontada * `4` Vendor Valores: `1`, `2`, `3`, `4`. |
| `dispatch_type` | `integer` | no | Tipo de Cobranza: Cuando el boleto necesita ser enviado por correo. Es necesario contratar el servicio con el banco y pagará una tarifa. * `1` Cliente * `2` Banco Valores: `1`, `2`. |
| `guarantor_name` | `string` | no | Nombre del Beneficiario final (Sacador/Avalista) |
| `guarantor_cnpj_cpf` | `string` | no | CNPJ/CPF del Beneficiario final (Sacador/Avalista) |
| `guarantor_address_number` | `string` | no | Número del Beneficiario final (Sacador/Avalista) |
| `guarantor_neighborhood` | `string` | no | Barrio del Beneficiario final (Sacador/Avalista) |
| `guarantor_phone_number` | `string` | no | Teléfono (con DDD) del Beneficiario final (Sacador/Avalista) |
| `guarantor_city_name` | `string` | no | Ciudad (El nombre debe ser correcto y completo) del Beneficiario final (Sacador/Avalista) |
| `guarantor_state` | `string` | no | Estado del Beneficiario final (Sacador/Avalista) |
| `guarantor_zipcode` | `string` | no | CEP (formato 99999999) del Beneficiario final (Sacador/Avalista) |
| `guarantor_address` | `string` | no | Dirección del Beneficiario final (Sacador/Avalista) |
| `guarantor_address_complement` | `string` | no | Complemento del Beneficiario final (Sacador/Avalista) |
| `description` | `string` | no | Descripción del producto o servicio |
| `instructions` | `string` | no | Instrucciones para el Cajero |
| `document_date` | `string (date)` | no | Fecha del Documento |
| `document_type` | `string` | no | Tipo de Documento: * `Código` \| `Sigla` \| Descripción * `01` \| `CH` \| Cheque * `02` \| `DM` \| Duplicata Mercantil (Predeterminado) * `03` \| `DMI` \| Duplicata Mercantil p/ Indicación * `04` \| `DS` \| Duplicata de Servicio * `05` \| `DSI` \| Duplicata de Servicio p/ Indicación * `06` \| `DR` \| Duplicata Rural * `07` \| `LC` \| Letra de Cambio * `08` \| `NCC` \| Nota de Crédito Comercial * `09` \| `NCE` \| Nota de Crédito a Exportación * `10` \| `NCI` \| Nota de Crédito Industrial * `11` \| `NCR` \| Nota de Crédito Rural * `12` \| `NP` \| Nota Promissória * `13` \| `NPR` \| Nota Promissória Rural * `14` \| `TM` \| Triplicata Mercantil * `15` \| `TS` \| Triplicata de Servicio * `16` \| `NS` \| Nota de Seguro * `17` \| `RC` \| Recibo * `18` \| `FAT` \| Factura * `19` \| `ND` \| Nota de Débito * `20` \| `AP` \| Póliza de Seguro * `21` \| `ME` \| Mensualidad Escolar * `22` \| `PC` \| Parcela de Consorcio * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Documento de Deuda * `25` \| `CPR` \| Cédula de Producto Rural * `26` \| `CTR` \| Contrato * `27` \| `CSG` \| Coaseguros * `28` \| `EC` \| Gastos Condominiales * `29` \| `CPS` \| Cuenta de Prestación de Servicios * `30` \| `WR` \| Warrant * `31` \| `DP` \| Duplicata Prestación * `32` \| `CSR` \| Cobranza Seriada * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Póliza Ramos Elementales * `35` \| `CC` \| Tarjeta de Crédito * `36` \| `BDP` \| Boleto de Propuesta * `37` \| `NPD` \| Nota Promissória Directa * `38` \| `DAE` \| Deuda Activa de Estado * `39` \| `DAM` \| Deuda Activa de Municipio * `40` \| `DAU` \| Deuda Activa Unión * `41` \| `CCB` \| Célula de Crédito Bancario * `42` \| `FI` \| Financiamiento * `43` \| `RD` \| Prorrateo de Gastos * `44` \| `DRI` \| Duplicata Rural p/ Indicación * `45` \| `ECI` \| Gastos Condominiales p/ Indicación * `99` \| `Otros` \| Otros Valores: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `document_type_label` | `string` | no | Tipo de Documento (Sigla) |
| `document_number` | `string` | no | Número del Documento |
| `acceptance` | `string` | no | Aceptación: * `N` No (Predeterminado) * `S` Sí Valores: `N`, `S`. |
| `our_number` | `string` | no | Nuestro Número. Si no se proporciona, usará el Siguiente Nuestro Número de la Cartera de Cobranza. |
| `processed_our_number` | `string` | no | Nuestro Número calculado con DV (formateado) |
| `processed_our_number_raw` | `string` | no | Nuestro Número calculado con DV (limpio) |
| `paid_amount` | `number` | no | Valor pagado |
| `paid_at` | `string (date)` | no | Fecha del pago |
| `days_for_revoke` | `integer` | no | Días corridos para Baja/Devolución: Nulo/En blanco: Obedece al estándar del banco. 0: Baja/Devolución el mismo día del vencimiento. 1 o más: Baja/Devolución después del vencimiento (Vencimiento + X días corridos). |
| `days_for_negativation` | `integer` | no | Días corridos para Negativación: Disponible solo para los siguientes bancos y formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sí \|	Sí	\| No * Itaú \| No \| Sí \| No |
| `days_for_sue_type` | `integer` | no | Tipo de días para protesto: * `0` Corridos * `1` Hábiles Valores: `null`, `0`, `1`. |
| `days_for_sue` | `integer` | no | Días corridos para Protesto |
| `sue_code` | `string` | no | Código de Protesto (CNAB 240). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `revoke_code` | `string` | no | Código de Baja (CNAB 240). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `first_instruction` | `string` | no | Primera Instrucción (CNAB 400). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `second_instruction` | `string` | no | Segunda Instrucción (CNAB 400). Consulte los valores posibles &lt;a href="https://developers.kobana.com.br/reference/bancos-suportados" target="_blank">para cada banco&lt;/a>. |
| `watermark` | `boolean` | no | Dirección |
| `divergent_payment_type` | `integer` | no | Tipo de pago divergente: Válido solo para Itaú y Caixa. * `1` Acepta cualquier valor divergente * `2` Acepta pagos dentro de un rango de valores o porcentajes * `3` No acepta pago de valores divergentes * `4` Acepta pagos de valores superiores a un valor o porcentaje mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Tipo de valor a considerar para los límites de pagos: Válido solo para Itaú y Caixa. * `1` Informa pagos divergentes por valores * `2` Informa pagos divergentes por porcentajes Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Valor mínimo para el rango de pagos divergentes. |
| `divergent_payment_minimum_value` | `number` | no | Valor máximo para el rango de pagos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | no | Porcentaje mínimo para el rango de pagos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | no | Porcentaje máximo para el rango de pagos divergentes. |
| `divergent_payment_limit` | `integer` | no | Cantidad de pagos permitida. Obligatorio si se proporcionan datos para pago divergente. Usado solo por Caixa. |
| `prevent_registration` | `boolean` | no | Prevenir envío de registro al banco: Si `true`, impide que el boleto sea registrado. Para ser usado en los casos en que el boleto ya fue registrado fuera de KOBANA pero desea incluirse en el sistema. |
| `control_number` | `string` | no | Número de control: Puede contener cualquier información de interés de la Empresa. La información contenida en este campo siempre se devolverá con el título respectivo en el archivo de retorno. |
| `ignore_email` | `boolean` | no | No enviar este boleto por correo electrónico |
| `ignore_sms` | `boolean` | no | Nunca enviar este boleto por SMS |
| `ignore_whatsapp` | `boolean` | no | Nunca enviar este boleto por WhatsApp |
| `addons` | `string (jsonb)` | no | Dirección |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `meta` | `object` | no | Hash con clave y valor en formato JSON. |
| `notes` | `string` | no | Observaciones |
| `custom_attachment_name` | `string` | no | Nombre para ser usado en los archivos de boleto enviados al cliente en notificaciones. Acepta uso de variables. Si se deja en blanco, el estándar es la palabra "boleto" acompañada del ID. |
| `split_payment` | `boolean` | no | División de Pago. Válido solo para ABC Brasil (remesa CNAB 240), Itaú y Bradesco (remesa CNAB 400). Si true, la división del boleto será registrada. Informar las cuentas para división en `split_accounts` |
| `split_payment_type` | `integer` | no | Tipo de Split de Pago: Válido solo para Itau. Usado solo con Split de Pago true. * `1` Prorrateo de crédito por porcentaje (%) – Valor nominal del título * `2` Prorrateo de crédito en valor (R$) – Valor nominal del título * `3` Prorrateo de crédito por porcentaje (%) – Valor neto recibido * `4` Prorrateo de crédito en valor (R$) – Valor neto recibido, prorrateado proporcionalmente Valores: `1`, `2`, `3`, `4`. |
| `split_accounts` | `array<object>` | no | Cuentas para Split de pago. |
| `payment_place` | `string` | no | Lugar de Pago |
| `installment_id` | `integer` | no | ID del Carné |
| `installment_number` | `integer` | no | Número de la parcela del carnê |
| `installment_total` | `integer` | no | Total de parcelas del carnê |
| `customer_subscription_id` | `integer` | no | ID de la Suscripción |
| `beneficiary_name` | `string` | no | Nombre del Beneficiario |
| `beneficiary_cnpj_cpf` | `string` | no | CNPJ/CPF del Beneficiario |
| `beneficiary_address` | `string` | no | Dirección del Beneficiario |
| `beneficiary_assignor_code` | `string` | no | Agencia/Código del Beneficiario |
| `bank_contract_slug` | `string` | no | Slug de la Cartera |
| `agency_number` | `string` | no | Agencia |
| `agency_digit` | `string` | no | Dígito de la Agencia |
| `account_number` | `string` | no | Cuenta |
| `account_digit` | `string` | no | Dígito de la Cuenta |
| `extra1` | `string` | no | Campo extra 1 |
| `extra1_digit` | `string` | no | Dígito del Campo extra 1 |
| `extra2` | `string` | no | Campo extra 2 |
| `extra2_digit` | `string` | no | Dígito del Campo extra 2 |
| `created_via_api` | `boolean` | no | Indica si el boleto fue creado por API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del boleto |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del boleto |
| `registered_at` | `string (date-time)` | no | Fecha y hora del registro |
| `register_type` | `integer` | no | Tipo de Registro: * `1` API * `2` Banco Valores: `1`, `2`. |
| `cancel_type` | `integer` | no | Tipo de Cancelación: * `1` Cliente * `2` Banco Valores: `1`, `2`. |
| `payment_count` | `integer` | no | Cantidad de pagos parciales aceptados para este boleto. |
| `line` | `string` | no | Línea Digitalizada |
| `barcode` | `string` | no | Dirección |
| `shorten_url` | `string` | no | URL para visualización del boleto |
| `url` | `string` | no | URL para visualización del boleto |
| `carne_url` | `string` | no | URL para visualización del carnê (Cuando sea parcela) |
| `formats` | `object` | no | URLs con formatos disponibles. Ej.: PDF, Imagen, Pix, etc. |
| `pix_enabled` | `boolean` | no | Indica si el boleto es híbrido y tiene QRcode Pix |
| `pix_qrcode` | `string` | no | QRcode Pix del boleto híbrido |
| `status` | `string` | no | Situación del boleto: * `generating` Generando * `draft` Borrador * `generation_failed` Error al generar * `opened` Abierto * `canceled` Cancelado * `paid` Pagado * `overdue` Vencido * `validation_failed` Inválido * `chargeback` Revertido Valores: `generating`, `draft`, `generation_failed`, `validation_failed`, `opened`, `canceled`, `paid`, `overdue`, `blocked`, `chargeback`. |
| `recipient_account` | `string` | no | Cuenta Destinataria + Dígito |

Otros campos y objetos anidados: `kobana schema banking.bank-billets.duplicate`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Boleto duplicado |
| `404` | Boleto no encontrado |

## bank-billets pay

Marcar Boleto Como Pagado

```text
kobana banking bank-billets pay [ID] [OPCIONES]
```

`PUT /v1/bank_billets/{id}/pay`

Marcar boleto como pagado

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `paid_amount` | `number` | sí | Valor pagado del boleto. |
| `paid_at` | `string (date)` | sí | Fecha de pago del boleto |
| `bank_rate` | `string` | no | Valor de la comisión bancaria. |
| `direct_payment` | `boolean` | no | Informa si el pago fue realizado directamente al beneficiario. |

Otros campos y objetos anidados: `kobana schema banking.bank-billets.pay`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Boleto pagado |
| `404` | Boleto no encontrado |

## bank-billets send-email

Enviar Boleto por Correo Electrónico

```text
kobana banking bank-billets send-email [ID] [OPCIONES]
```

`POST /v1/bank_billets/{id}/send_email`

Enviar boleto por correo electrónico

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `form` | `object` | sí | Formulario de Correo electrónico. |

Otros campos y objetos anidados: `kobana schema banking.bank-billets.send-email`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Correo electrónico enviado con éxito |
| `400` | Fallo al enviar el correo electrónico |

## bank-billets send-sms

Enviar Boleto por SMS

```text
kobana banking bank-billets send-sms [ID] [OPCIONES]
```

`POST /v1/bank_billets/{id}/send_sms`

Enviar boleto por SMS

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `form` | `object` | sí | Formulario de SMS. |

Otros campos y objetos anidados: `kobana schema banking.bank-billets.send-sms`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | SMS enviado con éxito |
| `400` | Fallo al enviar el sms |
