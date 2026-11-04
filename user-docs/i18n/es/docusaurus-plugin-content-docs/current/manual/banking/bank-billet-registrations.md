---
id: bank-billet-registrations
title: "kobana banking bank-billet-registrations"
sidebar_label: "bank-billet-registrations"
---

# kobana banking bank-billet-registrations


Comandos de `kobana banking bank-billet-registrations`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`bank-billet-registrations list`](#bank-billet-registrations-list) | `GET /v1/bank_billet_registrations` | Listar Registros de Boleto. |
| [`bank-billet-registrations get`](#bank-billet-registrations-get) | `GET /v1/bank_billet_registrations/{id}` | Información del Registro de Boleto |

## bank-billet-registrations list

Listar Registros de Boleto.

```text
kobana banking bank-billet-registrations list [OPCIONES]
```

`GET /v1/bank_billet_registrations`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `registered_from` | `string` | no | A partir de la Fecha de registro. |
| `registered_to` | `string` | no | Hasta la Fecha de registro. |
| `bank_billet_id` | `string` | no | ID del Boleto. |
| `bank_billet_account_id` | `string` | no | ID de la Cartera. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de registros de boleto |

## bank-billet-registrations get

Información del Registro de Boleto

```text
kobana banking bank-billet-registrations get [ID] [OPCIONES]
```

`GET /v1/bank_billet_registrations/{id}`

Consulta información específica de un Registro de Boleto.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro de Boleto encontrado |
| `404` | Registro de retorno no encontrado |
