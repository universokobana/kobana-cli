---
id: withholdings
title: "kobana billing withholdings"
sidebar_label: "withholdings"
---

# kobana billing withholdings


Comandos de `kobana billing withholdings`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`withholdings list`](#withholdings-list) | `GET /v1/withholdings` | Listar retenciones |
| [`withholdings create`](#withholdings-create) | `POST /v1/withholdings` | Crea una retención |
| [`withholdings update`](#withholdings-update) | `PUT /v1/withholdings` | Crea o actualiza una retención |
| [`withholdings convert-to-credits`](#withholdings-convert-to-credits) | `POST /v1/withholdings/convert-to-credits` | Convertir retenciones pendientes en créditos (en lote) |
| [`withholdings get`](#withholdings-get) | `GET /v1/withholdings/{id}` | Obtener retención |
| [`withholdings delete`](#withholdings-delete) | `DELETE /v1/withholdings/{id}` | Elimina una retención |
| [`withholdings convert-to-credit`](#withholdings-convert-to-credit) | `POST /v1/withholdings/{id}/convert-to-credit` | Convertir retención en crédito |

## withholdings list

Listar retenciones

```text
kobana billing withholdings list [OPCIONES]
```

`GET /v1/withholdings`

Devuelve una lista paginada de las retenciones tributarias de la organización actual. Admite filtros por tipo de tributo, estado, retenedor, factura, NF-e y competencia.

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
| `tax_type` | `string` | no | Filtra por tipo de tributo: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs` o `ibs`. Valores: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `status` | `string` | no | Filtra por estado: `retained`, `compensated` o `partially_compensated`. Valores: `retained`, `compensated`, `partially_compensated`. |
| `withholder_id` | `string` | no | Filtra por las retenciones de un retenedor (cuenta de cobro) específico. |
| `invoice_id` | `string` | no | Filtra por las retenciones asociadas a una factura. |
| `nfe_id` | `string` | no | Filtra por las retenciones asociadas a una NF-e. |
| `competence_year` | `integer` | no | Filtra por el año de competencia (2000–3000). |
| `competence_month` | `integer` | no | Filtra por el mes de competencia (1–12). |
| `amount_min` | `number,null` | no | Monto mínimo retenido en BRL (ej.: `10.5` para R$ 10,50). |
| `amount_max` | `number,null` | no | Monto máximo retenido en BRL (ej.: `500.0` para R$ 500,00). |
| `base_min` | `number,null` | no | Base de cálculo mínima en BRL. |
| `base_max` | `number,null` | no | Base de cálculo máxima en BRL. |
| `compensated_min` | `number,null` | no | Monto mínimo ya compensado en BRL. |
| `compensated_max` | `number,null` | no | Monto máximo ya compensado en BRL. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de retenciones. |
| `404` | Recurso no encontrado en la organización actual. |

## withholdings create

Crea una retención

```text
kobana billing withholdings create [OPCIONES]
```

`POST /v1/withholdings`

Registra una retención en la fuente para el retenedor indicado. `base` y `amount` son valores en BRL (float, hasta 4 decimales).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `tax_type` | `string` | sí | Tributo retenido. Valores: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `base` | `number` | sí | Base de cálculo, en BRL (float, hasta 4 decimales). |
| `rate` | `number` | sí | Alícuota aplicada, en porcentaje. |
| `amount` | `number` | sí | Monto retenido, en BRL (float, hasta 4 decimales). |
| `withholder_id` | `string (uuid)` | sí | Cuenta de cobro que efectuó la retención. |
| `competence_month` | `integer` | sí | Mes de competencia (1–12). |
| `competence_year` | `integer` | sí | Año de competencia. |
| `invoice_id` | `string (uuid)` | no | Factura de origen, cuando exista. |
| `nfe_id` | `string (uuid)` | no | Nota fiscal relacionada, cuando exista. |
| `metadata` | `object` | no |  |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema billing.withholdings.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Retención creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## withholdings update

Crea o actualiza una retención

```text
kobana billing withholdings update [OPCIONES]
```

`PUT /v1/withholdings`

Localiza la retención por `id`, `public_id` o `external_id`. Si no la encuentra, la crea con el cuerpo completo y devuelve `201`. **Si la encuentra, solo aplica `competence_month`, `competence_year`, `metadata`, `external_id` y `custom_metadata`**: `base`, `rate`, `amount` y `withholder_id` son inmutables tras el registro y se ignoran en el cuerpo de actualización.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `tax_type` | `string` | no | Tributo retenido. Valores: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `base` | `number` | no | Base de cálculo, en BRL (float, hasta 4 decimales). |
| `rate` | `number` | no | Alícuota aplicada, en porcentaje. |
| `amount` | `number` | no | Monto retenido, en BRL (float, hasta 4 decimales). |
| `withholder_id` | `string (uuid)` | no | Cuenta de cobro que efectuó la retención. |
| `competence_month` | `integer` | no | Mes de competencia (1–12). |
| `competence_year` | `integer` | no | Año de competencia. |
| `invoice_id` | `string (uuid)` | no | Factura de origen, cuando exista. |
| `nfe_id` | `string (uuid)` | no | Nota fiscal relacionada, cuando exista. |
| `metadata` | `object` | no |  |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `custom_metadata` | `object` | no |  |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.withholdings.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## withholdings convert-to-credits

Convertir retenciones pendientes en créditos (en lote)

```text
kobana billing withholdings convert-to-credits [OPCIONES]
```

`POST /v1/withholdings/convert-to-credits`

Convierte todas las retenciones pendientes de la organización — opcionalmente acotadas por `tax_type` y/o competencia — en créditos. Devuelve la lista de créditos generados, la cantidad total convertida y el monto total en BRL.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `tax_type` | `string` | no | Restringe la conversión a un tipo de tributo específico (ej.: `irrf`, `pis`, `cofins`). Valores: `iss`, `pis`, `cofins`, `csll`, `irpj`, `irrf`, `csrf`, `inss`, `cbs`, `ibs`. |
| `competence_year` | `integer` | no | Restringe la conversión al año de competencia indicado. |
| `competence_month` | `integer` | no | Restringe la conversión al mes de competencia indicado (1–12). |

Otros campos y objetos anidados: `kobana schema billing.withholdings.convert-to-credits`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lote de retenciones convertido en créditos. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## withholdings get

Obtener retención

```text
kobana billing withholdings get [ID] [OPCIONES]
```

`GET /v1/withholdings/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la retención. |
| `404` | Recurso no encontrado en la organización actual. |

## withholdings delete

Elimina una retención

```text
kobana billing withholdings delete [ID] [OPCIONES]
```

`DELETE /v1/withholdings/{id}`

Rechaza la eliminación cuando la retención ya fue compensada, total o parcialmente.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Retención eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## withholdings convert-to-credit

Convertir retención en crédito

```text
kobana billing withholdings convert-to-credit [ID] [OPCIONES]
```

`POST /v1/withholdings/{id}/convert-to-credit`

Convierte una retención (total o parcialmente) en un crédito en la cuenta de cobro derivada de la factura asociada — o, en su ausencia, del propio retenedor. Cuando se omite `amount`, convierte el saldo pendiente de la retención.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `amount` | `number` | no | Monto a convertir en BRL (float, hasta 4 decimales). Por defecto: saldo pendiente de la retención. |

Otros campos y objetos anidados: `kobana schema billing.withholdings.convert-to-credit`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Crédito generado a partir de la retención. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
