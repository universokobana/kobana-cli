---
id: bank-billet-batches
title: "kobana banking bank-billet-batches"
sidebar_label: "bank-billet-batches"
---

# kobana banking bank-billet-batches


Comandos de `kobana banking bank-billet-batches`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`bank-billet-batches list`](#bank-billet-batches-list) | `GET /v1/bank_billet_batches` | Listar Lotes |
| [`bank-billet-batches create`](#bank-billet-batches-create) | `POST /v1/bank_billet_batches` | Crear Lote |
| [`bank-billet-batches get`](#bank-billet-batches-get) | `GET /v1/bank_billet_batches/{id}/` | Información del Lote |
| [`bank-billet-batches delete`](#bank-billet-batches-delete) | `DELETE /v1/bank_billet_batches/{id}/` | Eliminar el lote |
| [`bank-billet-batches add-bank-billets`](#bank-billet-batches-add-bank-billets) | `PUT /v1/bank_billet_batches/{id}/add_bank_billets` | Incluir Boletos en el Lote |
| [`bank-billet-batches pdf`](#bank-billet-batches-pdf) | `POST /v1/bank_billet_batches/{id}/pdf` | Exportar Lote en Formato .pdf |
| [`bank-billet-batches remove-bank-billet`](#bank-billet-batches-remove-bank-billet) | `DELETE /v1/bank_billet_batches/{id}/remove_bank_billet` | Eliminar Boleto del Lote |
| [`bank-billet-batches zip`](#bank-billet-batches-zip) | `POST /v1/bank_billet_batches/{id}/zip` | Exportar Lote en Formato .zip |

## bank-billet-batches list

Listar Lotes

```text
kobana banking bank-billet-batches list [OPCIONES]
```

`GET /v1/bank_billet_batches`

Listar Lotes.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de lotes |

## bank-billet-batches create

Crear Lote

```text
kobana banking bank-billet-batches create [OPCIONES]
```

`POST /v1/bank_billet_batches`

Crear Lote.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre del lote. |
| `bank_billet_ids` | `string` | no | IDs de los Boletos separados por coma. |

Otros campos y objetos anidados: `kobana schema banking.bank-billet-batches.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Lote creado con éxito |
| `422` | Datos inválidos |

## bank-billet-batches get

Información del Lote

```text
kobana banking bank-billet-batches get [ID] [OPCIONES]
```

`GET /v1/bank_billet_batches/{id}/`

Consulta información específica de un Lote.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lote encontrado |
| `404` | Lote no encontrado |

## bank-billet-batches delete

Eliminar el lote

```text
kobana banking bank-billet-batches delete [ID] [OPCIONES]
```

`DELETE /v1/bank_billet_batches/{id}/`

Eliminar lote.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Lote eliminado |
| `404` | Lote no encontrado |

## bank-billet-batches add-bank-billets

Incluir Boletos en el Lote

```text
kobana banking bank-billet-batches add-bank-billets [ID] [OPCIONES]
```

`PUT /v1/bank_billet_batches/{id}/add_bank_billets`

Incluir boletos en el lote.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `bank_billet_ids` | `string` | sí | IDs de los Boletos separados por coma. |

Otros campos y objetos anidados: `kobana schema banking.bank-billet-batches.add-bank-billets`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Boletos agregados con éxito |
| `404` | Lote no encontrado |
| `422` | Datos inválidos |

## bank-billet-batches pdf

Exportar Lote en Formato .pdf

```text
kobana banking bank-billet-batches pdf [ID] [OPCIONES]
```

`POST /v1/bank_billet_batches/{id}/pdf`

Exportar lote en formato .pdf.

Después de la solicitud la exportación entra en una cola para generar el archivo del lote. La URL del archivo del lote será devuelta por el endpoint de Exportación del Lote.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Solicitud de exportación del lote creada con éxito |
| `404` | Lote no encontrado |

## bank-billet-batches remove-bank-billet

Eliminar Boleto del Lote

```text
kobana banking bank-billet-batches remove-bank-billet [ID] [OPCIONES]
```

`DELETE /v1/bank_billet_batches/{id}/remove_bank_billet`

Eliminar boleto del lote.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `bank_billet_id` | `integer` | sí | ID de los Boleto separados por coma. |

Otros campos y objetos anidados: `kobana schema banking.bank-billet-batches.remove-bank-billet`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Boleto removido con éxito |
| `404` | Lote no encontrado |

## bank-billet-batches zip

Exportar Lote en Formato .zip

```text
kobana banking bank-billet-batches zip [ID] [OPCIONES]
```

`POST /v1/bank_billet_batches/{id}/zip`

Exportar lote en formato .zip.

Después de la solicitud la exportación entra en una cola para generar el archivo del lote. La URL del archivo del lote será devuelta por el endpoint de Exportación del Lote.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Solicitud de exportación del lote creada con éxito |
| `404` | Lote no encontrado |
