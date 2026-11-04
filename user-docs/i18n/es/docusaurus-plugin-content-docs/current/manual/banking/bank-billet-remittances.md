---
id: bank-billet-remittances
title: "kobana banking bank-billet-remittances"
sidebar_label: "bank-billet-remittances"
---

# kobana banking bank-billet-remittances


Comandos de `kobana banking bank-billet-remittances`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`bank-billet-remittances occurrence create`](#bank-billet-remittances-occurrence-create) | `POST /v1/bank_billet_remittances/{id}/occurrence/{our_code}` | Crear Pendencias |
| [`bank-billet-remittances pending`](#bank-billet-remittances-pending) | `GET /v1/bank_billet_remittances/pending` | Listar Registros de Remesa Pendientes |
| [`bank-billet-remittances list`](#bank-billet-remittances-list) | `GET /v1/bank_billet_remittances` | Listar Registros de Remesa |
| [`bank-billet-remittances get`](#bank-billet-remittances-get) | `GET /v1/bank_billet_remittances/{id}` | Información del Registro de Remesa |
| [`bank-billet-remittances delete`](#bank-billet-remittances-delete) | `DELETE /v1/bank_billet_remittances/{id}` | Eliminar Pendencias |

## bank-billet-remittances occurrence create

Crear Pendencias

```text
kobana banking bank-billet-remittances occurrence create [ID] [OUR_CODE] [OPCIONES]
```

`POST /v1/bank_billet_remittances/{id}/occurrence/{our_code}`

Crear pendencias.

**Argumentos de la ruta:** `id`, `our_code`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Pendencias creada |
| `422` | No fue posible procesar |

## bank-billet-remittances pending

Listar Registros de Remesa Pendientes

```text
kobana banking bank-billet-remittances pending [OPCIONES]
```

`GET /v1/bank_billet_remittances/pending`

Listar Registros de Remesa pendientes

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `our_code` | `string` | no | Código de operación de registro. Consulte los valores posibles para cada banco. |
| `expire_from` | `string` | no | A partir de Fecha de vencimiento. |
| `expire_to` | `string` | no | Hasta Fecha de vencimiento. |
| `bank_billet_id` | `string` | no | ID del Boleto. |
| `bank_billet_account_id` | `string` | no | ID de la Cartera. |
| `remittance_id` | `string` | no | ID de la Remesa. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de registros de remesa |

## bank-billet-remittances list

Listar Registros de Remesa

```text
kobana banking bank-billet-remittances list [OPCIONES]
```

`GET /v1/bank_billet_remittances`

Listar Registros de Remesa.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `our_code` | `string` | no | Código de operación de registro. Consulte los valores posibles para cada banco. |
| `expire_from` | `string` | no | A partir de Fecha de vencimiento. |
| `expire_to` | `string` | no | Hasta Fecha de vencimiento. |
| `bank_billet_id` | `string` | no | ID del Boleto. |
| `bank_billet_account_id` | `string` | no | ID de la Cartera. |
| `remittance_id` | `string` | no | ID de la Remesa. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de registros de remesa |

## bank-billet-remittances get

Información del Registro de Remesa

```text
kobana banking bank-billet-remittances get [ID] [OPCIONES]
```

`GET /v1/bank_billet_remittances/{id}`

Consulta información específica de un Registro de Remesa.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro de Remesa encontrado |
| `404` | Registro de Remesa no encontrado |

## bank-billet-remittances delete

Eliminar Pendencias

```text
kobana banking bank-billet-remittances delete [ID] [OPCIONES]
```

`DELETE /v1/bank_billet_remittances/{id}`

Eliminar pendencias.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Pendencias eliminada |
| `404` | Retorno de remesa no encontrado |
