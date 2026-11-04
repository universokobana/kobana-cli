---
id: remittances
title: "kobana banking remittances"
sidebar_label: "remittances"
---

# kobana banking remittances


Comandos de `kobana banking remittances`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`remittances list`](#remittances-list) | `GET /v1/remittances` | Listar CNABs |
| [`remittances create`](#remittances-create) | `POST /v1/remittances` | Crear CNAB |
| [`remittances bulk`](#remittances-bulk) | `POST /v1/remittances/bulk` | Crear CNABs en Lote |
| [`remittances get`](#remittances-get) | `GET /v1/remittances/{id}` | Información del CNAB |
| [`remittances delete`](#remittances-delete) | `DELETE /v1/remittances/{id}` | Eliminar CNAB |
| [`remittances raw`](#remittances-raw) | `GET /v1/remittances/{id}/raw` | Raw (text/plain) del CNAB |

## remittances list

Listar CNABs

```text
kobana banking remittances list [OPCIONES]
```

`GET /v1/remittances`

Listar CNABs.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `expire_from` | `string` | no | A partir de la Fecha de vencimiento del boleto. |
| `expire_to` | `string` | no | Hasta la Fecha de vencimiento del boleto. |
| `created_from` | `string` | no | A partir de la Fecha de creación de la Remesa. |
| `created_to` | `string` | no | Hasta la Fecha de creación de la Remesa. |
| `bank_billet_account_id` | `string` | no | Filtro por ID de la Cartera. |
| `our_code` | `string` | no | Filtro por Nuestro número. |
| `tags` | `string` | no | Filtro por tags (separados por coma). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Éxito en la lista de CNABs |

## remittances create

Crear CNAB

```text
kobana banking remittances create [OPCIONES]
```

`POST /v1/remittances`

Crear CNAB.

Se realizará una solicitud de Remesa. Después de la creación de la Remesa, será procesada y al final del procesamiento se emitirán notificaciones a través de Webhooks.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `bank_billet_account_id` | `integer` | no | ID de la Cartera de Cobro. |
| `bank_billet_remittance_ids` | `array<integer>` | no | IDs de Registros de Remesa. |
| `external_id` | `string` | no | ID externo. |
| `custom_data` | `object` | no | Datos personalizados. |
| `tags` | `array<string>` | no | Etiquetas. |

Otros campos y objetos anidados: `kobana schema banking.remittances.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | CNAB creado con custom_data y tags |
| `404` | Remesa no encontrada |

## remittances bulk

Crear CNABs en Lote

```text
kobana banking remittances bulk [OPCIONES]
```

`POST /v1/remittances/bulk`

Crear CNABs en Lote.

Se realizarán solicitudes de Remesa de acuerdo con IDs de las ocurrencias. Se creará una solicitud de Remesa para cada cartera asociada a las ocurrencias. Después de la creación de las Remesas, serán procesadas y al final del procesamiento se emitirán notificaciones a través de Webhooks.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `bank_billet_remittance_ids` | `array<integer>` | no | IDs de Registros de Remesa. |

Otros campos y objetos anidados: `kobana schema banking.remittances.bulk`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | CNABs creadas |
| `404` | Ocurrencia no encontrada |

## remittances get

Información del CNAB

```text
kobana banking remittances get [ID] [OPCIONES]
```

`GET /v1/remittances/{id}`

Consulta información específica de un CNAB.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | CNAB encontrado |
| `404` | CNAB no encontrado |

## remittances delete

Eliminar CNAB

```text
kobana banking remittances delete [ID] [OPCIONES]
```

`DELETE /v1/remittances/{id}`

Eliminar CNAB.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | CNAB eliminado |
| `404` | CNAB no encontrado |

## remittances raw

Raw (text/plain) del CNAB

```text
kobana banking remittances raw [ID] [OPCIONES]
```

`GET /v1/remittances/{id}/raw`

Raw (text/plain) del CNAB.

Se disparará el evento remittance.downloaded.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Raw del CNAB |
| `404` | CNAB no encontrado |
| `422` | Remesa esperando procesamiento |
