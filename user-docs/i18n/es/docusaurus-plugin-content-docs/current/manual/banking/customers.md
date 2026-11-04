---
id: customers
title: "kobana banking customers"
sidebar_label: "customers"
---

# kobana banking customers


Comandos de `kobana banking customers`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`customers cnpj-cpf`](#customers-cnpj-cpf) | `GET /v1/customers/cnpj_cpf` | Buscar Cliente por CNPJ/CPF |
| [`customers email`](#customers-email) | `GET /v1/customers/email` | Buscar Cliente por Correo Electrónico |
| [`customers people-sync`](#customers-people-sync) | `PUT /v1/customers/people_sync` | Habilitar o deshabilitar el espejo Cliente &lt;-> Persona |
| [`customers list`](#customers-list) | `GET /v1/customers` | Listar Clientes |
| [`customers create`](#customers-create) | `POST /v1/customers` | Crear un Cliente |
| [`customers get`](#customers-get) | `GET /v1/customers/{id}` | Ver el Cliente |
| [`customers update`](#customers-update) | `PUT /v1/customers/{id}` | Actualizar Cliente |

## customers cnpj-cpf

Buscar Cliente por CNPJ/CPF

```text
kobana banking customers cnpj-cpf [OPCIONES]
```

`GET /v1/customers/cnpj_cpf`

Ver un cliente buscando por cnpj o cpf.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `q` | `string` | sí | CNPJ/CPF del cliente |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cliente encontrado |
| `404` | Cliente no encontrado |

## customers email

Buscar Cliente por Correo Electrónico

```text
kobana banking customers email [OPCIONES]
```

`GET /v1/customers/email`

Ver un cliente buscando por correo electrónico.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `q` | `string` | sí | Correo electrónico del cliente |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cliente encontrado |
| `404` | Cliente no encontrado |

## customers people-sync

Habilitar o deshabilitar el espejo Cliente &lt;-> Persona

```text
kobana banking customers people-sync [OPCIONES]
```

`PUT /v1/customers/people_sync`

Activa o desactiva, para la cuenta, el espejo automático entre Clientes y Personas (cada Cliente obtiene una Persona equivalente y los cambios se propagan en ambos sentidos). Solo cambia la configuración: los Clientes ya existentes no obtienen una Persona hasta que se ejecute el backfill. En una subcuenta, `enabled` en la respuesta también refleja la herencia de la cuenta principal.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `enabled` | `boolean` | sí | true activa el espejo; false lo desactiva. |

Otros campos y objetos anidados: `kobana schema banking.customers.people-sync`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Configuración actualizada |
| `422` | Parámetro enabled ausente o inválido |

## customers list

Listar Clientes

```text
kobana banking customers list [OPCIONES]
```

`GET /v1/customers`

Listar clientes.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de clientes |

## customers create

Crear un Cliente

```text
kobana banking customers create [OPCIONES]
```

`POST /v1/customers`

Crea un nuevo cliente para asociación con cobros, boletos y suscripciones.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_name` | `string` | sí | Nombre o Razón Social |
| `address` | `string` | sí | Dirección |
| `neighborhood` | `string` | sí | Barrio |
| `zipcode` | `string` | sí | CEP (formato 99999999) |
| `city_name` | `string` | sí | Ciudad (El nombre debe ser correcto y completo) |
| `id` | `integer` | no | ID |
| `external_code` | `string` | no | Código Externo |
| `cnpj_cpf` | `string` | no | CPF/CNPJ |
| `contact_person` | `string` | no | Persona de Contacto (cuando es Empresa) |
| `nickname` | `string` | no | Apodo o Nombre Comercial del Pagador |
| `birthday` | `string (date)` | no | Fecha de Nacimiento |
| `email` | `string (email)` | no | Correo electrónico principal |
| `email_cc` | `string (email)` | no | Correo electrónico alternativo |
| `address_complement` | `string` | no | Complemento de la Dirección |
| `address_number` | `string` | no | Número |
| `state` | `string` | no | Estado |
| `phone_number` | `string` | no | Teléfono (con DDD, DDI es opcional) |
| `mobile_local_code` | `string` | no | DDD del Celular |
| `mobile_number` | `string` | no | Celular |
| `ignore_email` | `boolean` | no | Nunca enviar correo electrónico a este cliente |
| `ignore_sms` | `boolean` | no | Nunca enviar SMS a este cliente |
| `ignore_whatsapp` | `boolean` | no | Nunca enviar Whatsapp a este cliente |
| `notes` | `string` | no | Observaciones |
| `tags` | `array<string>` | no | Etiquetas asociadas al registro |
| `tag_list` | `string` | no | Etiquetas asociadas al registro |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `created_via_api` | `boolean` | no | Indica si el registro fue creado por API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.customers.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cliente creado |
| `422` | Cliente inválido |

## customers get

Ver el Cliente

```text
kobana banking customers get [ID] [OPCIONES]
```

`GET /v1/customers/{id}`

Consulta información específica de un Cliente.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cliente encontrado |
| `404` | Cliente no encontrado |

## customers update

Actualizar Cliente

```text
kobana banking customers update [ID] [OPCIONES]
```

`PUT /v1/customers/{id}`

Actualiza información específica de un Cliente.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `person_name` | `string` | sí | Nombre o Razón Social |
| `address` | `string` | sí | Dirección |
| `neighborhood` | `string` | sí | Barrio |
| `zipcode` | `string` | sí | CEP (formato 99999999) |
| `city_name` | `string` | sí | Ciudad (El nombre debe ser correcto y completo) |
| `id` | `integer` | no | ID |
| `external_code` | `string` | no | Código Externo |
| `contact_person` | `string` | no | Persona de Contacto (cuando es Empresa) |
| `nickname` | `string` | no | Apodo o Nombre Comercial del Pagador |
| `birthday` | `string (date)` | no | Fecha de Nacimiento |
| `email` | `string (email)` | no | Correo electrónico principal |
| `email_cc` | `string (email)` | no | Correo electrónico alternativo |
| `address_complement` | `string` | no | Complemento de la Dirección |
| `address_number` | `string` | no | Número |
| `state` | `string` | no | Estado |
| `phone_number` | `string` | no | Teléfono (con DDD, DDI es opcional) |
| `mobile_local_code` | `string` | no | DDD del Celular |
| `mobile_number` | `string` | no | Celular |
| `ignore_email` | `boolean` | no | Nunca enviar correo electrónico a este cliente |
| `ignore_sms` | `boolean` | no | Nunca enviar SMS a este cliente |
| `ignore_whatsapp` | `boolean` | no | Nunca enviar Whatsapp a este cliente |
| `notes` | `string` | no | Observaciones |
| `tags` | `array<string>` | no | Etiquetas asociadas al registro |
| `tag_list` | `string` | no | Etiquetas asociadas al registro |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `created_via_api` | `boolean` | no | Indica si el registro fue creado por API |
| `created_at` | `string (date-time)` | no | Fecha y hora de creación del registro |
| `updated_at` | `string (date-time)` | no | Fecha y hora de la última actualización del registro |

Otros campos y objetos anidados: `kobana schema banking.customers.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Cliente actualizado |
| `404` | Cliente no encontrado |
