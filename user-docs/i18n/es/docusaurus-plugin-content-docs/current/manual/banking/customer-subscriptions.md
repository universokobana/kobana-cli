---
id: customer-subscriptions
title: "kobana banking customer-subscriptions"
sidebar_label: "customer-subscriptions"
---

# kobana banking customer-subscriptions


Comandos de `kobana banking customer-subscriptions`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`customer-subscriptions list`](#customer-subscriptions-list) | `GET /v1/customer_subscriptions` | Listar Suscripciones |
| [`customer-subscriptions create`](#customer-subscriptions-create) | `POST /v1/customer_subscriptions` | Crear una Suscripción |
| [`customer-subscriptions get`](#customer-subscriptions-get) | `GET /v1/customer_subscriptions/{id}` | Información de la Suscripción |
| [`customer-subscriptions update`](#customer-subscriptions-update) | `PUT /v1/customer_subscriptions/{id}` | Actualizar la Suscripción |
| [`customer-subscriptions delete`](#customer-subscriptions-delete) | `DELETE /v1/customer_subscriptions/{id}` | Eliminar la Suscripción |
| [`customer-subscriptions next-charge`](#customer-subscriptions-next-charge) | `POST /v1/customer_subscriptions/{id}/next_charge` | Generar Próximo Cobro |

## customer-subscriptions list

Listar Suscripciones

```text
kobana banking customer-subscriptions list [OPCIONES]
```

`GET /v1/customer_subscriptions`

Listar suscripciones.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `customer_person_name` | `string` | no | Filtro por Nombre/Razón Social. |
| `customer_cnpj_cpf` | `string` | no | Filtro por CPF/CNPJ de la suscripción. Debe ser formateado con máscara. Encuentra todos los registros cuyo campo sea igual a la cadena. |
| `customer_email` | `string` | no | Filtro por correo electrónico del cliente. Encuentra todos los registros cuyo campo sea igual a la cadena. |
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de suscripciones |

## customer-subscriptions create

Crear una Suscripción

```text
kobana banking customer-subscriptions create [OPCIONES]
```

`POST /v1/customer_subscriptions`

Crea una nueva suscripción recurrente para cobro automático de clientes con boletos bancarios.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `customer_person_name` | `string` | sí | Nombre del cliente |
| `customer_cnpj_cpf` | `string` | sí | CPF/CNPJ del cliente |
| `customer_zipcode` | `string` | sí | CEP (formato 99999999) |
| `customer_address` | `string` | sí | Dirección |
| `customer_city_name` | `string` | sí | Ciudad (El nombre debe ser correcto y completo) |
| `customer_state` | `string` | sí | Estado |
| `customer_neighborhood` | `string` | sí | Barrio |
| `bank_billet_account_id` | `integer` | sí | ID de la Cartera de Cobranza. Si no se proporciona, usará la cartera predeterminada. |
| `amount` | `number` | sí | Valor de la Suscripción (R$) |
| `id` | `integer` | no | ID de la suscripción |
| `customer_id` | `integer` | no | ID del Cliente. Cuando se pasa este ID, los campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` y `customer_neighborhood` no son obligatorios. |
| `customer_nickname` | `string` | no | Apodo o Nombre Comercial del Pagador |
| `customer_email` | `string (email)` | no | Correo electrónico del Pagador |
| `customer_email_cc` | `string (email)` | no | Correo electrónico alternativo del Pagador |
| `customer_address_number` | `string` | no | Número |
| `customer_address_complement` | `string` | no | Complemento |
| `customer_phone_number` | `string` | no | Teléfono (con DDD, DDI es opcional) |
| `customer_person_type` | `string` | no | Tipo de Pagador. * `individual` Persona Física * `juridical` Persona Jurídica Valores: `individual`, `juridical`. |
| `customer_mobile_local_code` | `string` | no | DDD del Celular |
| `customer_mobile_number` | `string` | no | Celular |
| `customer_notes` | `string` | no | Observaciones del Pagador |
| `customer_ignore_email` | `boolean` | no | Nunca enviar correo electrónico a este cliente |
| `customer_ignore_sms` | `boolean` | no | Nunca enviar SMS a este cliente |
| `customer_contact_person` | `string` | no | Contacto |
| `customer_update` | `string` | no | Contacto |
| `cycle` | `string` | no | Ciclo de la suscripción. Predeterminado: monthly * `biweekly` Quincenal * `bimonthly` Bimestral * `monthly` Mensual * `quarterly` Trimestral * `semiannual` Semestral * `annual` Anual Valores: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `next_billing` | `string (date)` | no | Fecha del Primer o Próximo cobro. Si no se envía una fecha, este campo será calculado para tener el valor del día de creación de la suscripción más el ciclo elegido. Ej.: Mensual (Hoy + 30 días) |
| `end_at` | `string (date)` | no | Fecha en que desea detener los cobros. Si está en blanco, los cobros se generarán automáticamente hasta que se informe una fecha o se elimine la suscripción. |
| `description` | `string` | no | Descripción del producto o servicio |
| `instructions` | `string` | no | Instrucciones para el Cajero |
| `days_in_advance` | `integer` | no | Con cuántos días de anticipación a la fecha de vencimiento se generará el cobro. Predeterminado: 7. |
| `fine_type` | `integer` | no | Tipo de multa: * `0` Inexistente (Predeterminado) * `1` Para porcentaje del valor de boleto * `2` Para valor fijo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Cantidad de días después del vencimiento cuando la multa comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `fine_percentage` | `number` | no | Porcentaje de Multa por Atraso Ej: 2% x R$ 250,00 = R$ 5,00. Obligatorio si fine_type es igual a 1 |
| `fine_value` | `number` | no | Valor de la multa (R$). Obligatorio si fine_type es igual a 2. |
| `interest_type` | `integer` | no | Tipo de interés/mora: * `0` Inexistente (Predeterminado) * `1` Para porcentaje diario * `2` Para valor diario Valores: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | no | Cantidad de días después del vencimiento cuando la mora comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `interest_percentage` | `number` | no | Porcentaje diario de interés. De 0.0 a 100.0 (Ej 1.5% = 1.5) Obligatorio si interest_type es igual a 1. |
| `interest_value` | `number` | no | Valor diario de interés (R$). Obligatorio si interest_type es igual a 2. |
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
| `bank_billet_layout_id` | `integer` | no | ID del Modelo de boleto |
| `notes` | `string` | no | Observaciones |
| `tags` | `array<string>` | no | Etiquetas asociadas al boleto |
| `bank_billet_ids` | `array<integer>` | no | IDs de boletos vinculados al carnê |
| `prevent_registration` | `boolean` | no | Prevenir envío de registro al banco: Si `true`, impide que el boleto sea registrado. Para ser usado en los casos en que el boleto ya fue registrado fuera de KOBANA pero desea incluirse en el sistema. |
| `divergent_payment_type` | `integer` | no | Tipo de pago divergente: Válido solo para Itaú y Caixa. * `1` Acepta cualquier valor divergente * `2` Acepta pagos dentro de un rango de valores o porcentajes * `3` No acepta pago de valores divergentes * `4` Acepta pagos de valores superiores a un valor o porcentaje mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Tipo de valor a considerar para los límites de pagos: Válido solo para Itaú y Caixa. * `1` Informa pagos divergentes por valores * `2` Informa pagos divergentes por porcentajes Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Valor máximo para el rango de pagos divergentes. |
| `divergent_payment_minimum_value` | `number` | no | Valor mínimo para el rango de pagos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | no | Porcentaje máximo para el rango de pagos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | no | Porcentaje mínimo para el rango de pagos divergentes. |
| `divergent_payment_limit` | `integer` | no | Cantidad de pagos permitida. Obligatorio si se proporcionan datos para pago divergente. Usado solo por Caixa. |
| `custom_attachment_name` | `string` | no | Nombre para ser usado en los archivos de boleto enviados al cliente en notificaciones. Acepta uso de variables. Si se deja en blanco, el estándar es la palabra "boleto" acompañada del ID. |
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
| `days_for_revoke` | `integer` | no | Días corridos para Baja/Devolución: Nulo/En blanco: Obedece al estándar del banco. 0: Baja/Devolución el mismo día del vencimiento. 1 o más: Baja/Devolución después del vencimiento (Vencimiento + X días corridos). |
| `days_for_negativation` | `integer` | no | Días corridos para Negativación: Disponible solo para los siguientes bancos y formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sí \|	Sí	\| No * Itaú \| No \| Sí \| No |
| `days_for_sue` | `integer` | no | Días corridos para Protesto |
| `payment_count` | `integer` | no | Cantidad de pagos parciales aceptados para este boleto. |
| `import_id` | `integer` | no | ID de la Importación |
| `ignore_whatsapp` | `boolean` | no | Nunca enviar esta suscripción por WhatsApp |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.customer-subscriptions.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Suscripción creada |
| `422` | Suscripción inválida |

## customer-subscriptions get

Información de la Suscripción

```text
kobana banking customer-subscriptions get [ID] [OPCIONES]
```

`GET /v1/customer_subscriptions/{id}`

Consulta información específica de una Suscripción.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Suscripción encontrada |
| `404` | Suscripción no encontrada |

## customer-subscriptions update

Actualizar la Suscripción

```text
kobana banking customer-subscriptions update [ID] [OPCIONES]
```

`PUT /v1/customer_subscriptions/{id}`

Actualiza información específica de una Suscripción

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `customer_person_name` | `string` | sí | Nombre del cliente |
| `customer_cnpj_cpf` | `string` | sí | CPF/CNPJ del cliente |
| `customer_zipcode` | `string` | sí | CEP (formato 99999999) |
| `customer_address` | `string` | sí | Dirección |
| `customer_city_name` | `string` | sí | Ciudad (El nombre debe ser correcto y completo) |
| `customer_state` | `string` | sí | Estado |
| `customer_neighborhood` | `string` | sí | Barrio |
| `bank_billet_account_id` | `integer` | sí | ID de la Cartera de Cobranza. Si no se proporciona, usará la cartera predeterminada. |
| `amount` | `number` | sí | Valor de la Suscripción (R$) |
| `id` | `integer` | no | ID de la suscripción |
| `customer_id` | `integer` | no | ID del Cliente. Cuando se pasa este ID, los campos `customer_person_name`, `customer_cnpj_cpf`, `customer_zipcode`, `customer_address`, `customer_city_name`, `customer_state` y `customer_neighborhood` no son obligatorios. |
| `customer_nickname` | `string` | no | Apodo o Nombre Comercial del Pagador |
| `customer_email` | `string (email)` | no | Correo electrónico del Pagador |
| `customer_email_cc` | `string (email)` | no | Correo electrónico alternativo del Pagador |
| `customer_address_number` | `string` | no | Número |
| `customer_address_complement` | `string` | no | Complemento |
| `customer_phone_number` | `string` | no | Teléfono (con DDD, DDI es opcional) |
| `customer_person_type` | `string` | no | Tipo de Pagador. * `individual` Persona Física * `juridical` Persona Jurídica Valores: `individual`, `juridical`. |
| `customer_mobile_local_code` | `string` | no | DDD del Celular |
| `customer_mobile_number` | `string` | no | Celular |
| `customer_notes` | `string` | no | Observaciones del Pagador |
| `customer_ignore_email` | `boolean` | no | Nunca enviar correo electrónico a este cliente |
| `customer_ignore_sms` | `boolean` | no | Nunca enviar SMS a este cliente |
| `customer_contact_person` | `string` | no | Contacto |
| `customer_update` | `string` | no | Contacto |
| `cycle` | `string` | no | Ciclo de la suscripción. Predeterminado: monthly * `biweekly` Quincenal * `bimonthly` Bimestral * `monthly` Mensual * `quarterly` Trimestral * `semiannual` Semestral * `annual` Anual Valores: `biweekly`, `bimonthly`, `monthly`, `quarterly`, `semiannual`, `annual`. |
| `next_billing` | `string (date)` | no | Fecha del Primer o Próximo cobro. Si no se envía una fecha, este campo será calculado para tener el valor del día de creación de la suscripción más el ciclo elegido. Ej.: Mensual (Hoy + 30 días) |
| `end_at` | `string (date)` | no | Fecha en que desea detener los cobros. Si está en blanco, los cobros se generarán automáticamente hasta que se informe una fecha o se elimine la suscripción. |
| `description` | `string` | no | Descripción del producto o servicio |
| `instructions` | `string` | no | Instrucciones para el Cajero |
| `days_in_advance` | `integer` | no | Con cuántos días de anticipación a la fecha de vencimiento se generará el cobro. Predeterminado: 7. |
| `fine_type` | `integer` | no | Tipo de multa: * `0` Inexistente (Predeterminado) * `1` Para porcentaje del valor de boleto * `2` Para valor fijo Valores: `0`, `1`, `2`. |
| `days_for_fine` | `integer` | no | Cantidad de días después del vencimiento cuando la multa comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `fine_percentage` | `number` | no | Porcentaje de Multa por Atraso Ej: 2% x R$ 250,00 = R$ 5,00. Obligatorio si fine_type es igual a 1 |
| `fine_value` | `number` | no | Valor de la multa (R$). Obligatorio si fine_type es igual a 2. |
| `interest_type` | `integer` | no | Tipo de interés/mora: * `0` Inexistente (Predeterminado) * `1` Para porcentaje diario * `2` Para valor diario Valores: `0`, `1`, `2`. |
| `days_for_interest` | `integer` | no | Cantidad de días después del vencimiento cuando la mora comenzará a incidir. El valor predeterminado es 1 día (el día posterior al vencimiento). |
| `interest_percentage` | `number` | no | Porcentaje diario de interés. De 0.0 a 100.0 (Ej 1.5% = 1.5) Obligatorio si interest_type es igual a 1. |
| `interest_value` | `number` | no | Valor diario de interés (R$). Obligatorio si interest_type es igual a 2. |
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
| `bank_billet_layout_id` | `integer` | no | ID del Modelo de boleto |
| `notes` | `string` | no | Observaciones |
| `tags` | `array<string>` | no | Etiquetas asociadas al boleto |
| `bank_billet_ids` | `array<integer>` | no | IDs de boletos vinculados al carnê |
| `prevent_registration` | `boolean` | no | Prevenir envío de registro al banco: Si `true`, impide que el boleto sea registrado. Para ser usado en los casos en que el boleto ya fue registrado fuera de KOBANA pero desea incluirse en el sistema. |
| `divergent_payment_type` | `integer` | no | Tipo de pago divergente: Válido solo para Itaú y Caixa. * `1` Acepta cualquier valor divergente * `2` Acepta pagos dentro de un rango de valores o porcentajes * `3` No acepta pago de valores divergentes * `4` Acepta pagos de valores superiores a un valor o porcentaje mínimo Valores: `null`, `1`, `2`, `3`, `4`. |
| `divergent_payment_value_type` | `integer` | no | Tipo de valor a considerar para los límites de pagos: Válido solo para Itaú y Caixa. * `1` Informa pagos divergentes por valores * `2` Informa pagos divergentes por porcentajes Valores: `null`, `1`, `2`. |
| `divergent_payment_maximum_value` | `number` | no | Valor máximo para el rango de pagos divergentes. |
| `divergent_payment_minimum_value` | `number` | no | Valor mínimo para el rango de pagos divergentes. |
| `divergent_payment_maximum_percentage` | `number` | no | Porcentaje máximo para el rango de pagos divergentes. |
| `divergent_payment_minimum_percentage` | `number` | no | Porcentaje mínimo para el rango de pagos divergentes. |
| `divergent_payment_limit` | `integer` | no | Cantidad de pagos permitida. Obligatorio si se proporcionan datos para pago divergente. Usado solo por Caixa. |
| `custom_attachment_name` | `string` | no | Nombre para ser usado en los archivos de boleto enviados al cliente en notificaciones. Acepta uso de variables. Si se deja en blanco, el estándar es la palabra "boleto" acompañada del ID. |
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
| `days_for_revoke` | `integer` | no | Días corridos para Baja/Devolución: Nulo/En blanco: Obedece al estándar del banco. 0: Baja/Devolución el mismo día del vencimiento. 1 o más: Baja/Devolución después del vencimiento (Vencimiento + X días corridos). |
| `days_for_negativation` | `integer` | no | Días corridos para Negativación: Disponible solo para los siguientes bancos y formatos. * Banco \|	CNAB 240\|	CNAB 400 \|Webservice * Bradesco \|	Sí \|	Sí	\| No * Itaú \| No \| Sí \| No |
| `days_for_sue` | `integer` | no | Días corridos para Protesto |
| `payment_count` | `integer` | no | Cantidad de pagos parciales aceptados para este boleto. |
| `import_id` | `integer` | no | ID de la Importación |
| `ignore_whatsapp` | `boolean` | no | Nunca enviar esta suscripción por WhatsApp |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.customer-subscriptions.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Suscripción actualizada |
| `404` | Suscripción no encontrada |

## customer-subscriptions delete

Eliminar la Suscripción

```text
kobana banking customer-subscriptions delete [ID] [OPCIONES]
```

`DELETE /v1/customer_subscriptions/{id}`

Elimina una Suscripción.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Suscripción eliminada |
| `404` | Suscripción no encontrada |

## customer-subscriptions next-charge

Generar Próximo Cobro

```text
kobana banking customer-subscriptions next-charge [ID] [OPCIONES]
```

`POST /v1/customer_subscriptions/{id}/next_charge`

Genera el próximo cobro.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Próximo cobro generado |
| `404` | Suscripción no encontrada |
