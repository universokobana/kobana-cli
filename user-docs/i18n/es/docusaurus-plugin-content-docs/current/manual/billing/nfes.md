---
id: nfes
title: "kobana billing nfes"
sidebar_label: "nfes"
---

# kobana billing nfes


Comandos de `kobana billing nfes`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`nfes bulk create`](#nfes-bulk-create) | `POST /v1/nfes/bulk` | Actualizar NF-e en lote |
| [`nfes bulk delete`](#nfes-bulk-delete) | `DELETE /v1/nfes/bulk` | Eliminar NF-e en lote |
| [`nfes request-tries list`](#nfes-request-tries-list) | `GET /v1/nfes/request-tries` | Listar reintentos de solicitud (organización) |
| [`nfes request-tries get`](#nfes-request-tries-get) | `GET /v1/nfes/{id}/request-tries` | Listar reintentos de solicitud de la NF-e |
| [`nfes requests get`](#nfes-requests-get) | `GET /v1/nfes/{id}/requests/{request_id}` | Obtener solicitud externa |
| [`nfes requests list`](#nfes-requests-list) | `GET /v1/nfes/{id}/requests` | Listar solicitudes externas de la NF-e |
| [`nfes list`](#nfes-list) | `GET /v1/nfes` | Listar NF-e |
| [`nfes create`](#nfes-create) | `POST /v1/nfes` | Crear NF-e (borrador) |
| [`nfes update`](#nfes-update) | `PUT /v1/nfes` | Crear o actualizar NF-e |
| [`nfes fetch-barueri`](#nfes-fetch-barueri) | `POST /v1/nfes/fetch-barueri` | Buscar NFS-e en Barueri |
| [`nfes fetch-from-sefaz`](#nfes-fetch-from-sefaz) | `POST /v1/nfes/fetch-from-sefaz` | Buscar NFS-e en ADN (SEFAZ) |
| [`nfes import-barueri`](#nfes-import-barueri) | `POST /v1/nfes/import-barueri` | Importar NFS-e Barueri |
| [`nfes sync`](#nfes-sync) | `POST /v1/nfes/sync` | Sincronizar NF-e de la organización |
| [`nfes get`](#nfes-get) | `GET /v1/nfes/{id}` | Obtener NF-e |
| [`nfes delete`](#nfes-delete) | `DELETE /v1/nfes/{id}` | Eliminar NF-e |
| [`nfes cancel`](#nfes-cancel) | `POST /v1/nfes/{id}/cancel` | Cancelar NF-e |
| [`nfes issue`](#nfes-issue) | `POST /v1/nfes/{id}/issue` | Emitir NF-e |
| [`nfes pdf`](#nfes-pdf) | `GET /v1/nfes/{id}/pdf` | Descargar PDF de la NF-e |
| [`nfes retry-cancel`](#nfes-retry-cancel) | `POST /v1/nfes/{id}/retry-cancel` | Reintentar cancelación de la NF-e |
| [`nfes validate-barueri`](#nfes-validate-barueri) | `POST /v1/nfes/{id}/validate-barueri` | Validar NFS-e Barueri |
| [`nfes xml`](#nfes-xml) | `GET /v1/nfes/{id}/xml` | Descargar XML de la NF-e |

## nfes bulk create

Actualizar NF-e en lote

```text
kobana billing nfes bulk create [OPCIONES]
```

`POST /v1/nfes/bulk`

Actualiza varias NF-e a la vez. Hoy soporta vincular o desvincular una factura (`invoice_id`) en hasta 100 NF-e por solicitud.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `nfe_ids` | `array<string (uuid)>` | sí | UUIDs de las NF-e a actualizar. Mínimo 1. |
| `invoice_id` | `string (uuid)` | sí | UUID de la factura a vincular. Envía `null` para desvincular. |
| `invoice_withholding` | `string` | no | Qué hacer si la retención de las NF-e difiere de la retención de la factura: `block` (predeterminado) rechaza con 409 `NFE_WITHHOLDING_MISMATCH`; `update_invoice` actualiza la factura con la retención de la NF-e. Valores: `block`, `update_invoice`. |

Otros campos y objetos anidados: `kobana schema billing.nfes.bulk.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cantidad de NF-e actualizadas. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `409` | La retención de la NF-e difiere de la retención de la factura vinculada (`NFE_WITHHOLDING_MISMATCH`). `details` trae ambos lados; repite con `invoice_withholding: "update_invoice"` para actualizar la factura. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes bulk delete

Eliminar NF-e en lote

```text
kobana billing nfes bulk delete [OPCIONES]
```

`DELETE /v1/nfes/bulk`

Elimina hasta 100 NF-e a la vez. Todas deben estar en estado `draft` y pertenecer a la organización actual. Exige el nivel de acceso `delete`. Reemplaza `POST /v1/nfes/bulk-delete`, eliminado.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `ids` | `array<string (uuid)>` | sí | UUIDs de las NF-e a eliminar. Mínimo 1, máximo 100. |

Otros campos y objetos anidados: `kobana schema billing.nfes.bulk.delete`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cantidad de NF-e eliminadas. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes request-tries list

Listar reintentos de solicitud (organización)

```text
kobana billing nfes request-tries list [OPCIONES]
```

`GET /v1/nfes/request-tries`

Lista todos los intentos de solicitud al proveedor de NF-e de la organización, paginados y ordenados por fecha descendente.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de intentos. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes request-tries get

Listar reintentos de solicitud de la NF-e

```text
kobana billing nfes request-tries get [ID] [OPCIONES]
```

`GET /v1/nfes/{id}/request-tries`

Lista los intentos (`NfeRequestTry`) realizados al proveedor para esta NF-e — una entrada por llamada, con endpoint, estado HTTP, duración y mensajes de error.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de intentos. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes requests get

Obtener solicitud externa

```text
kobana billing nfes requests get [ID] [REQUEST_ID] [OPCIONES]
```

`GET /v1/nfes/{id}/requests/{request_id}`

**Argumentos de la ruta:** `id`, `request_id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la solicitud externa, incluyendo cabeceras y cuerpos. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes requests list

Listar solicitudes externas de la NF-e

```text
kobana billing nfes requests list [ID] [OPCIONES]
```

`GET /v1/nfes/{id}/requests`

Lista las solicitudes HTTP capturadas para esta NF-e (llamadas al proveedor y webhooks recibidos), con paginación. Usa `direction` para filtrar entre `outbound` e `inbound`.

**Argumentos de la ruta:** `id`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `direction` | `string` | no | Filtra solicitudes por dirección: `outbound` (llamadas salientes) o `inbound` (webhooks recibidos). Valores: `outbound`, `inbound`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de solicitudes. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes list

Listar NF-e

```text
kobana billing nfes list [OPCIONES]
```

`GET /v1/nfes`

Devuelve una lista paginada de NF-e (facturas electrónicas) de la organización actual.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `person_id` | `string` | no | Filtra por UUID de la persona asociada. |
| `person_document_number` | `string` | no | Filtra por CPF/CNPJ de la persona (coincidencia exacta). |
| `person_name_contains` | `string` | no | Filtra por nombre de la persona (búsqueda parcial, sin distinción de mayúsculas). |
| `billing_account_id` | `string` | no | Filtra por `billing_account_id`. |
| `company_id` | `string` | no | Filtra por `company_id` emisor. |
| `invoice_id` | `string` | no | Filtra por `invoice_id` vinculado. |
| `payment_id` | `string` | no | Filtra por `payment_id` vinculado. |
| `transaction_id` | `string` | no | Filtra por `transaction_id` vinculado. |
| `status` | `string` | no | Filtra por estado: `draft`, `pending`, `processing`, `issued`, `pending_cancel`, `canceling`, `canceled` o `error`. Valores: `draft`, `pending`, `processing`, `issued`, `pending_cancel`, `canceling`, `canceled`, `error`. |
| `invoice_filter` | `string` | no | Filtra por presencia de factura: `with` (solo con factura) o `without` (solo sueltas). Valores: `with`, `without`. |
| `nfe_number` | `integer,null` | no | Filtra por número de la NF-e. |
| `nfe_series` | `string` | no | Filtra por serie de la NF-e. |
| `nfe_key` | `string` | no | Filtra por clave de acceso de la NF-e. |
| `external_id` | `string` | no | Filtra por identificador externo de la NF-e. |
| `lc116_code` | `string` | no | Filtra por código LC 116. |
| `service_code` | `string` | no | Filtra por código del servicio municipal. |
| `amount_min` | `number,null` | no | Monto mínimo de la NF-e en reales (filtra `amount`). |
| `amount_max` | `number,null` | no | Monto máximo de la NF-e en reales (filtra `amount`). |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |
| `issue_date_from` | `string` | no | Filtra por `issue_date` inicial (inclusivo, formato YYYY-MM-DD). |
| `issue_date_to` | `string` | no | Filtra por `issue_date` final (inclusivo, formato YYYY-MM-DD). |
| `cancelled_after` | `string` | no | Filtra por `cancelled_at` inicial (inclusivo, formato YYYY-MM-DD). |
| `cancelled_before` | `string` | no | Filtra por `cancelled_at` final (inclusivo, formato YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de NF-e. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes create

Crear NF-e (borrador)

```text
kobana billing nfes create [OPCIONES]
```

`POST /v1/nfes`

Crea una NF-e en borrador (`draft`) para emisión posterior vía `POST /nfes/{id}/issue`. Acepta datos del tomador (`customer`) y alícuotas opcionales.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | sí | UUID de la cuenta de cobro (tomador). **Obligatorio**. |
| `amount` | `number` | sí | Monto de la NF-e en BRL (float, hasta 4 decimales). **Obligatorio**, debe ser positivo. |
| `customer` | `object` | sí | Datos del tomador al momento de la emisión. **Obligatorio**. |
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora (prestador). Por defecto: empresa por defecto de la organización. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `invoice_id` | `string (uuid)` | no | UUID de la factura vinculada (opcional). |
| `transaction_id` | `string (uuid)` | no | UUID de la transacción de origen (opcional). |
| `service_code` | `string` | no | Código del servicio según la tabla municipal. |
| `service_description` | `string` | no | Descripción libre del servicio prestado. |
| `lc116_code` | `string` | no | Código LC 116. |
| `cnae_code` | `string` | no | Código CNAE. |
| `nbs_code` | `string` | no | Código NBS. |
| `c_class_trib` | `string` | no | Clasificación Tributaria (cClassTrib) del IBS/CBS. |
| `ind_op` | `string` | no | Indicador de Operación (indOp) del IBS/CBS. |
| `rps_number` | `integer` | no | Número del RPS (cuando la numeración es gestionada por la aplicación). |
| `rps_serial_number` | `string` | no | Serie del RPS. |
| `taxes` | `object` | no | Alícuotas de impuestos (en decimal, ej.: `0.05` para 5%). Aplica los defaults de la empresa cuando se omite. |
| `custom_metadata` | `object` | no | Campos personalizados (validados contra el schema de la organización). |

Otros campos y objetos anidados: `kobana schema billing.nfes.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | NF-e creada en borrador. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes update

Crear o actualizar NF-e

```text
kobana billing nfes update [OPCIONES]
```

`PUT /v1/nfes`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `billing_account_id` | `string (uuid)` | no | UUID de la cuenta de cobro (tomador). **Obligatorio**. |
| `company_id` | `string (uuid)` | no | UUID de la empresa emisora (prestador). Por defecto: empresa por defecto de la organización. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `invoice_id` | `string (uuid)` | no | UUID de la factura vinculada (opcional). |
| `transaction_id` | `string (uuid)` | no | UUID de la transacción de origen (opcional). |
| `amount` | `number` | no | Monto de la NF-e en BRL (float, hasta 4 decimales). **Obligatorio**, debe ser positivo. |
| `service_code` | `string` | no | Código del servicio según la tabla municipal. |
| `service_description` | `string` | no | Descripción libre del servicio prestado. |
| `lc116_code` | `string` | no | Código LC 116. |
| `cnae_code` | `string` | no | Código CNAE. |
| `nbs_code` | `string` | no | Código NBS. |
| `c_class_trib` | `string` | no | Clasificación Tributaria (cClassTrib) del IBS/CBS. |
| `ind_op` | `string` | no | Indicador de Operación (indOp) del IBS/CBS. |
| `rps_number` | `integer` | no | Número del RPS (cuando la numeración es gestionada por la aplicación). |
| `rps_serial_number` | `string` | no | Serie del RPS. |
| `customer` | `object` | no | Datos del tomador al momento de la emisión. **Obligatorio**. |
| `taxes` | `object` | no | Alícuotas de impuestos (en decimal, ej.: `0.05` para 5%). Aplica los defaults de la empresa cuando se omite. |
| `custom_metadata` | `object` | no | Campos personalizados (validados contra el schema de la organización). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.nfes.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes fetch-barueri

Buscar NFS-e en Barueri

```text
kobana billing nfes fetch-barueri [OPCIONES]
```

`POST /v1/nfes/fetch-barueri`

Busca una NFS-e en el endpoint municipal de Barueri usando el `codigo_autenticidade` impreso en el DANFSE y el CPF/CNPJ del tomador. Devuelve el XML crudo y, cuando se encuentra, la NF-e ya existente en la organización.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `codigo_autenticidade` | `string` | sí | Código de autenticidad impreso en el DANFSE de la NFS-e. |
| `cnpj_tomador` | `string` | sí | CPF o CNPJ del tomador de la NFS-e (solo dígitos o con formato). |
| `ambiente` | `string` | no | Ambiente municipal: `production` o `homologation`. Por defecto: `production`. Valores: `production`, `homologation`. |

Otros campos y objetos anidados: `kobana schema billing.nfes.fetch-barueri`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la búsqueda con XML y referencia a la NF-e existente, cuando exista. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes fetch-from-sefaz

Buscar NFS-e en ADN (SEFAZ)

```text
kobana billing nfes fetch-from-sefaz [OPCIONES]
```

`POST /v1/nfes/fetch-from-sefaz`

Busca una NFS-e en el Ambiente de Dados Nacional (ADN) usando una clave de acceso de 50 dígitos y un certificado digital previamente registrado (autenticación mTLS).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `access_key` | `string` | sí | Clave de acceso de la NFS-e (50 dígitos). |
| `certificate_id` | `string` | sí | UUID del certificado digital registrado en la organización (usado para autenticación mTLS). |
| `ambiente` | `anyOf` | no | Ambiente SEFAZ: `1` (producción) o `2` (homologación). Por defecto: `2`. |

Otros campos y objetos anidados: `kobana schema billing.nfes.fetch-from-sefaz`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la búsqueda en el ADN. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes import-barueri

Importar NFS-e Barueri

```text
kobana billing nfes import-barueri [OPCIONES]
```

`POST /v1/nfes/import-barueri`

Importa una NFS-e de Barueri en la base de datos. Acepta un XML crudo (devuelto por `POST /nfes/fetch-barueri`) o el `codigo_autenticidade` + `cnpj_tomador` para re-buscar antes de importar. Crea la `BillingAccount` del tomador cuando aún no existe. Exige el nivel de acceso `import` — `write` no basta.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `codigo_autenticidade` | `string` | sí | Código de autenticidad impreso en el DANFSE de la NFS-e. |
| `cnpj_tomador` | `string` | sí | CPF o CNPJ del tomador de la NFS-e (solo dígitos o con formato). |
| `xml` | `string` | no | XML crudo devuelto por `POST /nfes/fetch-barueri`. Opcional — cuando se omite, la NFS-e se vuelve a buscar con `codigo_autenticidade` + `cnpj_tomador`. |
| `ambiente` | `string` | no | Ambiente municipal: `production` o `homologation`. Por defecto: `production`. Valores: `production`, `homologation`. |

Otros campos y objetos anidados: `kobana schema billing.nfes.import-barueri`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la importación con flags `created`/`updated` y la NF-e persistida. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes sync

Sincronizar NF-e de la organización

```text
kobana billing nfes sync [OPCIONES]
```

`POST /v1/nfes/sync`

Dispara un job en background para sincronizar todas las NF-e de la organización con el proveedor (actualiza estado, descarga PDFs/XMLs pendientes). Devuelve el `job_id` para seguimiento. Rechaza si ya hay una sync activa.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Sync en cola. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes get

Obtener NF-e

```text
kobana billing nfes get [ID] [OPCIONES]
```

`GET /v1/nfes/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la NF-e. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes delete

Eliminar NF-e

```text
kobana billing nfes delete [ID] [OPCIONES]
```

`DELETE /v1/nfes/{id}`

Elimina una NF-e. Permitido solo para estados `draft`, `pending`, `processing`, `error` o `canceled`. NF-e `issued` deben cancelarse vía `POST /nfes/{id}/cancel`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | NF-e eliminada. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes cancel

Cancelar NF-e

```text
kobana billing nfes cancel [ID] [OPCIONES]
```

`POST /v1/nfes/{id}/cancel`

Cancela una NF-e emitida. Encola la cancelación de forma asíncrona con el proveedor. El `reason` debe tener al menos 15 caracteres y se registra en el audit log.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `reason` | `string` | sí | Motivo de la cancelación. **Mínimo 15 caracteres**. Registrado en el audit log y enviado al proveedor. |

Otros campos y objetos anidados: `kobana schema billing.nfes.cancel`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cancelación en cola. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes issue

Emitir NF-e

```text
kobana billing nfes issue [ID] [OPCIONES]
```

`POST /v1/nfes/{id}/issue`

Emite una NF-e en borrador (`draft`) o reintenta una NF-e en `pending`/`error`. Envía la solicitud al proveedor configurado en la organización.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | NF-e en cola/emitida. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes pdf

Descargar PDF de la NF-e

```text
kobana billing nfes pdf [ID] [OPCIONES]
```

`GET /v1/nfes/{id}/pdf`

Descarga el PDF de la NF-e (DANFE) como adjunto binario (`application/pdf`). Servido desde la base de datos cuando está disponible; en caso contrario, se busca en el proveedor.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | PDF de la NF-e. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes retry-cancel

Reintentar cancelación de la NF-e

```text
kobana billing nfes retry-cancel [ID] [OPCIONES]
```

`POST /v1/nfes/{id}/retry-cancel`

Reintenta la cancelación de una NF-e atascada en `pending_cancel`, `canceling` o `error`. Re-encola el job de cancelación.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Reintento de cancelación en cola. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## nfes validate-barueri

Validar NFS-e Barueri

```text
kobana billing nfes validate-barueri [ID] [OPCIONES]
```

`POST /v1/nfes/{id}/validate-barueri`

Re-busca la NFS-e en la municipalidad de Barueri y la compara con el registro local, devolviendo la lista de diferencias y el XML actualizado para reaplicación vía `POST /nfes/import-barueri`.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la validación con diferencias y XML actual. |
| `404` | Recurso no encontrado en la organización actual. |

## nfes xml

Descargar XML de la NF-e

```text
kobana billing nfes xml [ID] [OPCIONES]
```

`GET /v1/nfes/{id}/xml`

Descarga el XML de la NF-e como adjunto binario (`application/xml`). Servido desde la base de datos cuando está disponible; en caso contrario, se busca en el proveedor.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | XML de la NF-e. |
| `404` | Recurso no encontrado en la organización actual. |
