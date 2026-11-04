---
id: discharges
title: "kobana banking discharges"
sidebar_label: "discharges"
---

# kobana banking discharges


Comandos de `kobana banking discharges`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`discharges list`](#discharges-list) | `GET /v1/discharges` | Listar CNABs |
| [`discharges create`](#discharges-create) | `POST /v1/discharges` | Enviar CNAB |
| [`discharges get`](#discharges-get) | `GET /v1/discharges/{id}` | Información del CNAB |
| [`discharges download`](#discharges-download) | `GET /v1/discharges/{id}/download` | Descargar CNAB |
| [`discharges pay-off`](#discharges-pay-off) | `PUT /v1/discharges/{id}/pay_off` | Pagar Boletos |
| [`discharges reprocess`](#discharges-reprocess) | `PUT /v1/discharges/{id}/reprocess` | Reprocesar CNAB |

## discharges list

Listar CNABs

```text
kobana banking discharges list [OPCIONES]
```

`GET /v1/discharges`

Listar CNABs.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `created_from` | `string` | no | A partir de la Fecha de creación del Retorno. |
| `created_to` | `string` | no | Hasta la Fecha de creación del Retorno. |
| `bank_billet_account_id` | `string` | no | Filtro por ID de la Cartera. |
| `tags` | `string` | no | Filtro por tags (separados por coma). |
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de CNABs |

## discharges create

Enviar CNAB

```text
kobana banking discharges create [OPCIONES]
```

`POST /v1/discharges`

Envía un archivo CNAB de retorno bancario para procesamiento de pagos y bajas de boletos.

**Cuerpo de la solicitud**

Se envía como formulario `multipart/form-data`: los archivos van en `--file CAMPO=RUTA`.

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `file` | `string (binary)` | no | Archivo |
| `filename` | `string` | no | Nombre del archivo |
| `bank_billet_account_id` | `integer` | no | ID de la Cartera de Cobro. |
| `external_id` | `string` | no | ID externo. |
| `custom_data` | `object` | no | Datos personalizados. |
| `tags` | `array<string>` | no | Etiquetas. |

Otros campos y objetos anidados: `kobana schema banking.discharges.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | CNAB enviado con custom_data y tags |
| `422` | Datos inválidos |

## discharges get

Información del CNAB

```text
kobana banking discharges get [ID] [OPCIONES]
```

`GET /v1/discharges/{id}`

Consulta información específica de un CNAB.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | CNAB encontrado |
| `404` | CNAB no encontrado |

## discharges download

Descargar CNAB

```text
kobana banking discharges download [ID] [OPCIONES]
```

`GET /v1/discharges/{id}/download`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Descarga con éxito |
| `404` | CNAB no encontrado |

## discharges pay-off

Pagar Boletos

```text
kobana banking discharges pay-off [ID] [OPCIONES]
```

`PUT /v1/discharges/{id}/pay_off`

Pagar boletos.

Ya es necesario haber enviado el CNAB. Todos los boletos que se identifiquen dentro del CNAB serán marcados como PAGADO y los Webhooks serán disparados.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Boletos pagados |
| `404` | CNAB no encontrado |

## discharges reprocess

Reprocesar CNAB

```text
kobana banking discharges reprocess [ID] [OPCIONES]
```

`PUT /v1/discharges/{id}/reprocess`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | CNAB reprocesada |
| `404` | CNAB no encontrado |
