---
id: bank-accounts
title: "kobana billing bank-accounts"
sidebar_label: "bank-accounts"
---

# kobana billing bank-accounts


Comandos de `kobana billing bank-accounts`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`bank-accounts list`](#bank-accounts-list) | `GET /v1/bank_accounts` | Listar cuentas bancarias |
| [`bank-accounts create`](#bank-accounts-create) | `POST /v1/bank_accounts` | Registrar cuenta bancaria |
| [`bank-accounts get`](#bank-accounts-get) | `GET /v1/bank_accounts/{id}` | Consultar cuenta bancaria |
| [`bank-accounts update`](#bank-accounts-update) | `PATCH /v1/bank_accounts/{id}` | Actualizar cuenta bancaria |
| [`bank-accounts delete`](#bank-accounts-delete) | `DELETE /v1/bank_accounts/{id}` | Eliminar cuenta bancaria |

## bank-accounts list

Listar cuentas bancarias

```text
kobana billing bank-accounts list [OPCIONES]
```

`GET /v1/bank_accounts`

Cuentas bancarias de la organización a las que los clientes transfieren en un cobro `bank_transfer` (`POST /invoices/{id}/pay`).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `active` | `string` | no | Filtra por cuentas activas (`true`) o inactivas (`false`). Valores: `true`, `false`. |
| `bank_id` | `string` | no | Filtra por banco (UUID de `/banks`). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de cuentas bancarias. |
| `404` | Recurso no encontrado en la organización actual. |

## bank-accounts create

Registrar cuenta bancaria

```text
kobana billing bank-accounts create [OPCIONES]
```

`POST /v1/bank_accounts`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Alias de la cuenta, para el operador. |
| `bank_id` | `string (uuid)` | sí | UUID del banco (vea `/banks`). |
| `agency` | `string` | sí | Agencia, solo dígitos. |
| `account_number` | `string` | sí | Número de cuenta, solo dígitos. |
| `type` | `string` | no | `checking` (corriente) o `savings` (ahorro). Valores: `checking`, `savings`. |
| `agency_digit` | `string` | no | Dígito de la agencia, cuando lo haya. |
| `account_digit` | `string` | no | Dígito de la cuenta, cuando lo haya. |
| `holder_name` | `string` | no | Titular mostrado al cliente. Vacío usa la razón social de la empresa emisora de la factura (o de la organización). |
| `holder_document` | `string` | no | CPF/CNPJ del titular, solo dígitos. Vacío usa el documento de la empresa emisora (o de la organización). |
| `company_id` | `string (uuid)` | no | Empresa emisora a la que pertenece (opcional). |
| `active` | `boolean` | no | Solo los destinos activos pueden elegirse en un nuevo cobro. |

Otros campos y objetos anidados: `kobana schema billing.bank-accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cuenta bancaria creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## bank-accounts get

Consultar cuenta bancaria

```text
kobana billing bank-accounts get [ID] [OPCIONES]
```

`GET /v1/bank_accounts/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta bancaria encontrada. |
| `404` | Recurso no encontrado en la organización actual. |

## bank-accounts update

Actualizar cuenta bancaria

```text
kobana billing bank-accounts update [ID] [OPCIONES]
```

`PATCH /v1/bank_accounts/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Alias de la cuenta, para el operador. |
| `bank_id` | `string (uuid)` | no | UUID del banco (vea `/banks`). |
| `type` | `string` | no | `checking` (corriente) o `savings` (ahorro). Valores: `checking`, `savings`. |
| `agency` | `string` | no | Agencia, solo dígitos. |
| `agency_digit` | `string` | no | Dígito de la agencia, cuando lo haya. |
| `account_number` | `string` | no | Número de cuenta, solo dígitos. |
| `account_digit` | `string` | no | Dígito de la cuenta, cuando lo haya. |
| `holder_name` | `string` | no | Titular mostrado al cliente. Vacío usa la razón social de la empresa emisora de la factura (o de la organización). |
| `holder_document` | `string` | no | CPF/CNPJ del titular, solo dígitos. Vacío usa el documento de la empresa emisora (o de la organización). |
| `company_id` | `string (uuid)` | no | Empresa emisora a la que pertenece (opcional). |
| `active` | `boolean` | no | Solo los destinos activos pueden elegirse en un nuevo cobro. |

Otros campos y objetos anidados: `kobana schema billing.bank-accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta bancaria actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## bank-accounts delete

Eliminar cuenta bancaria

```text
kobana billing bank-accounts delete [ID] [OPCIONES]
```

`DELETE /v1/bank_accounts/{id}`

Eliminación lógica. Los cobros ya enviados siguen mostrando los datos que recibió el cliente.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta bancaria eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
