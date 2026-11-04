---
id: installments
title: "kobana banking installments"
sidebar_label: "installments"
---

# kobana banking installments


Comandos de `kobana banking installments`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`installments list`](#installments-list) | `GET /v1/installments` | Listar Carnés |
| [`installments create`](#installments-create) | `POST /v1/installments` | Crear un Carné |
| [`installments get`](#installments-get) | `GET /v1/installments/{id}` | Información del Carné |
| [`installments delete`](#installments-delete) | `DELETE /v1/installments/{id}` | Eliminar el Carné |

## installments list

Listar Carnés

```text
kobana banking installments list [OPCIONES]
```

`GET /v1/installments`

Listar Carnés.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de carnés |

## installments create

Crear un Carné

```text
kobana banking installments create [OPCIONES]
```

`POST /v1/installments`

Crea un carné para generación de múltiples boletos con vencimientos periódicos.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `bank_billet_account_id` | `integer` | sí | ID de la Cartera de Cobranza. Si no se proporciona, usará la cartera predeterminada. |
| `amount` | `number` | sí | Cantidad |
| `customer_person_name` | `string` | sí | Nombre del cliente |
| `customer_cnpj_cpf` | `string` | sí | CPF/CNPJ del cliente |
| `customer_state` | `string` | sí | Estado |
| `customer_city_name` | `string` | sí | Ciudad (El nombre debe ser correcto y completo) |
| `customer_zipcode` | `string` | sí | CEP (formato 99999999) |
| `customer_address` | `string` | sí | Dirección |
| `customer_neighborhood` | `string` | sí | Barrio |
| `start_at` | `string (date)` | sí | Fecha del primer cobro |
| `total` | `integer` | sí | Cantidad de parcelas |
| `document_type` | `string` | sí | Tipo de Documento: * `Código` \| `Sigla` \| Descripción * `01` \| `CH` \| Cheque * `02` \| `DM` \| Duplicata Mercantil (Predeterminado) * `03` \| `DMI` \| Duplicata Mercantil p/ Indicación * `04` \| `DS` \| Duplicata de Servicio * `05` \| `DSI` \| Duplicata de Servicio p/ Indicación * `06` \| `DR` \| Duplicata Rural * `07` \| `LC` \| Letra de Cambio * `08` \| `NCC` \| Nota de Crédito Comercial * `09` \| `NCE` \| Nota de Crédito a Exportación * `10` \| `NCI` \| Nota de Crédito Industrial * `11` \| `NCR` \| Nota de Crédito Rural * `12` \| `NP` \| Nota Promissória * `13` \| `NPR` \| Nota Promissória Rural * `14` \| `TM` \| Triplicata Mercantil * `15` \| `TS` \| Triplicata de Servicio * `16` \| `NS` \| Nota de Seguro * `17` \| `RC` \| Recibo * `18` \| `FAT` \| Factura * `19` \| `ND` \| Nota de Débito * `20` \| `AP` \| Póliza de Seguro * `21` \| `ME` \| Mensualidad Escolar * `22` \| `PC` \| Parcela de Consorcio * `23` \| `NF` \| Nota Fiscal * `24` \| `DD` \| Documento de Deuda * `25` \| `CPR` \| Cédula de Producto Rural * `26` \| `CTR` \| Contrato * `27` \| `CSG` \| Coaseguros * `28` \| `EC` \| Gastos Condominiales * `29` \| `CPS` \| Cuenta de Prestación de Servicios * `30` \| `WR` \| Warrant * `31` \| `DP` \| Duplicata Prestación * `32` \| `CSR` \| Cobranza Seriada * `33` \| `CAR` \| Carnê * `34` \| `ARE` \| Póliza Ramos Elementales * `35` \| `CC` \| Tarjeta de Crédito * `36` \| `BDP` \| Boleto de Propuesta * `37` \| `NPD` \| Nota Promissória Directa * `38` \| `DAE` \| Deuda Activa de Estado * `39` \| `DAM` \| Deuda Activa de Municipio * `40` \| `DAU` \| Deuda Activa Unión * `41` \| `CCB` \| Célula de Crédito Bancario * `42` \| `FI` \| Financiamiento * `43` \| `RD` \| Prorrateo de Gastos * `44` \| `DRI` \| Duplicata Rural p/ Indicación * `45` \| `ECI` \| Gastos Condominiales p/ Indicación * `99` \| `Otros` \| Otros Valores: `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `25`, `26`, `27`, `28`, `29`, `30`, `31`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `43`, `44`, `45`, `99`. |
| `id` | `integer` | no | ID del carnê |
| `bank_billet_layout_id` | `integer` | no | ID del Modelo de boleto |
| `customer_id` | `integer` | no | ID del Cliente. Cuando se pasa este ID, los campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` y `customer_neighborhood` no son obligatorios. |
| `customer_address_complement` | `string` | no | Complemento |
| `customer_address_number` | `string` | no | Número |
| `customer_email` | `string (email)` | no | Correo electrónico del Pagador |
| `customer_email_cc` | `string (email)` | no | Correo electrónico alternativo del Pagador |
| `customer_phone_number` | `string` | no | Teléfono (com DDD) |
| `customer_ignore_email` | `boolean` | no | Nunca enviar correo electrónico a este cliente |
| `customer_ignore_sms` | `boolean` | no | Nunca enviar SMS a este cliente |
| `customer_mobile_local_code` | `string` | no | DDD del Celular |
| `customer_mobile_number` | `string` | no | Celular |
| `customer_nickname` | `string` | no | Apodo o Nombre Comercial del Pagador |
| `customer_notes` | `string` | no | Observaciones del Pagador |
| `customer_contact_person` | `string` | no | Contacto |
| `customer_update` | `string` | no | Contacto |
| `end_at` | `string (date)` | no | Fecha del último cobro |
| `cycle` | `string` | no | Ciclo del carnê. Predeterminado: monthly * `biweekly` Quincenal * `bimonthly` Bimestral * `monthly` Mensual * `quarterly` Trimestral * `semiannual` Semestral * `annual` Anual Valores: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `custom_bank_billets` | `array<object>` | no | Carnê personalizado. Cuando se envía, los valores de amount, start_at y end_at serán ignorados. |
| `bank_billet_ids` | `array<integer>` | no | IDs de boletos vinculados al carnê |
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
| `days_for_discount` | `integer` | no | Días para descuento. Obligatorio si discount_type es diferente de 0 (cero), No puede resultar en una fecha pasada referente a la creación. |
| `discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al descuento. Obligatorio si discount_type es igual a 2 |
| `discount_value` | `number` | no | Valor del descuento (R$). Obligatorio si discount_type es igual a 1. |
| `days_for_second_discount` | `integer` | no | Días para segundo descuento. |
| `second_discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al segundo descuento. |
| `second_discount_value` | `number` | no | Valor del segundo descuento (R$). |
| `days_for_third_discount` | `integer` | no | Días para tercer descuento. |
| `third_discount_percentage` | `number` | no | Porcentaje del valor de boleto equivalente al tercer descuento. |
| `third_discount_value` | `number` | no | Valor del tercer descuento (R$). |
| `tags` | `array<string>` | no | Etiquetas asociadas al boleto |
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
| `document_number` | `string` | no | Número del Documento, El tamaño depende del banco y formato del archivo Remesa: * Banco \|	Formato\| Tamaño \| * Abc \|`Cnab240`\|	15	\| * Ailos \|`Cnab240`\|	15  \| * Arbi \|`Cnab240`\|	15	\| * Banese \|`Cnab240`\|	15	\| * Banestes \|`Cnab400`\|	10	\| * Banrisul \|`Cnab240`\|	15	\| * Banrisul \|`Cnab400`\|	10	\| * Bb \|`Cnab240`\|	15	\| * Bb \|`Cnab400`\|	10	\| * Bib \|`Cnab240`\|	15	\| * Bnb \|`Cnab400`\|	10	\| * Bnpparibas \|`Cnab400`\|	10	\| * Bradesco \|`Cnab240`\|	15	\| * Bradesco \|`Cnab400`\|	10	\| * Brb \|`Cnab400`\|	10	\| * Caixa \|`Cnab240`\|	11	\| * Caixa \|`Cnab400`\|	10	\| * Caruana \|`Cnab400`\|	10	\| * Citibank \|`Cnab400`\|	10	\| * Credisis \|`Cnab240`\|	15	\| * Cresol \|`Cnab240`\|	10	\| * Cresol \|`Cnab400`\|	10	\| * Cresol Bradesco \|`Cnab240`\|	15	\| * Cresol Bradesco \|`Cnab400`\|	10	\| * Daycoval \|`Cnab400`\|	10	\| * Itau \|`Cnab240`\|	10	\| * Itau \|`Cnab400`\|	10	\| * Mercantil \|`Cnab240`\|	10	\| * Moneyplus \|`Cnab240`\|	10	\| * Rendimiento \|`Cnab400`\|	10	\| * Safra \|`Cnab400`\|	102	\| * Santander \|`Cnab240`\|	15	\| * Santander \|`Cnab400`\|	10	\| * Semear \|`Cnab400`\|	10	\| * Sicoob \|`Cnab240`\|	15	\| * Sicoob \|`Cnab400`\|	10	\| * Sicredi \|`Cnab240`\|	15	\| * Sicredi \|`Cnab400`\|	10	\| * Sofisa \|`Cnab240`\|	15	\| * Unicred \|`Cnab240`\|	15	\| * Uniprime \|`Cnab400`\|	10	\| * Uniprime99 \|`Cnab400`\|	10	\| * Santander \|`Cnab400`\|	10 |
| `days_for_revoke` | `integer` | no | Días corridos para Baja/Devolución: Nulo/En blanco: Obedece al estándar del banco. 0: Baja/Devolución el mismo día del vencimiento. 1 o más: Baja/Devolución después del vencimiento (Vencimiento + X días corridos). |
| `days_for_negativation` | `integer` | no | Días corridos para Negativación: Disponible solo para los siguientes bancos y formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sí \|	Sí	\| No * Itaú \| No \| Sí \| No |
| `days_for_sue` | `integer` | no | Días corridos para Protesto |
| `divergent_payment_type` | `integer` | no | Tipo de pago divergente: Válido solo para Itaú y Caixa. * `1` Acepta cualquier valor divergente * `2` Acepta pagos dentro de un rango de valores o porcentajes * `3` No acepta pago de valores divergentes * `4` Acepta pagos de valores superiores a un valor o porcentaje mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Tipo de valor a considerar para los límites de pagos: Válido solo para Itaú y Caixa. * `1` Informa pagos divergentes por valores * `2` Informa pagos divergentes por porcentajes Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Valor máximo para el rango de pagos divergentes. |
| `divergent_payment_minimum_value` | `number` | no | Valor mínimo para el rango de pagos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | no | Porcentaje máximo para el rango de pagos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | no | Porcentaje mínimo para el rango de pagos divergentes. |
| `divergent_payment_limit` | `integer` | no | Cantidad de pagos permitida. Obligatorio si se proporcionan datos para pago divergente. Usado solo por Caixa. |
| `prevent_registration` | `boolean` | no | Prevenir envío de registro al banco: Si `true`, impide que el boleto sea registrado. Para ser usado en los casos en que el boleto ya fue registrado fuera de KOBANA pero desea incluirse en el sistema. |
| `notes` | `string` | no | Observaciones |
| `custom_attachment_name` | `string` | no | Nombre para ser usado en los archivos de boleto enviados al cliente en notificaciones. Acepta uso de variables. Si se deja en blanco, el estándar es la palabra "boleto" acompañada del ID. |
| `created_via_api` | `boolean` | no | Indica si el boleto fue creado por API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del boleto |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del boleto |
| `payment_count` | `integer` | no | Cantidad de pagos parciales aceptados para este boleto. |
| `url` | `string` | no | URL para visualización del carnê |
| `status` | `string` | no | Situación del carnê: * `created` Generando * `processed` Parcelas creadas * `generated` Abierto Valores: `created`, `processed`, `generated`. |
| `import_id` | `integer` | no | ID de la Importación |
| `ignore_whatsapp` | `boolean` | no | Nunca enviar este pagaré por WhatsApp |

Otros campos y objetos anidados: `kobana schema banking.installments.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Carné Creado |
| `422` | Carné inválido |

## installments get

Información del Carné

```text
kobana banking installments get [ID] [OPCIONES]
```

`GET /v1/installments/{id}`

Consulta información específica de un Carné.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Carné encontrado con éxito |
| `404` | Carné no encontrado |

## installments delete

Eliminar el Carné

```text
kobana banking installments delete [ID] [OPCIONES]
```

`DELETE /v1/installments/{id}`

Elimina un Carné específico. Ningún boleto generado por el carné será eliminado.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Carné cancelado |
| `404` | Carné no encontrado |
