---
id: holidays
title: "kobana billing holidays"
sidebar_label: "holidays"
---

# kobana billing holidays


Comandos de `kobana billing holidays`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`holidays list`](#holidays-list) | `GET /v1/holidays` | Listar feriados |
| [`holidays create`](#holidays-create) | `POST /v1/holidays` | Crear feriado |
| [`holidays sync`](#holidays-sync) | `POST /v1/holidays/sync` | Importar feriados nacionales |
| [`holidays get`](#holidays-get) | `GET /v1/holidays/{id}` | Consultar feriado |
| [`holidays update`](#holidays-update) | `PATCH /v1/holidays/{id}` | Actualizar feriado |
| [`holidays delete`](#holidays-delete) | `DELETE /v1/holidays/{id}` | Eliminar feriado |

## holidays list

Listar feriados

```text
kobana billing holidays list [OPCIONES]
```

`GET /v1/holidays`

Devuelve los feriados de la organización — los días que el conteo en días hábiles salta. Una política de intereses y multas con `day_count = business` lee exactamente esta tabla.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `year` | `integer` | no | Filtra por año civil. |
| `date_from` | `string` | no | Fecha inicial, inclusiva, en formato `YYYY-MM-DD`. |
| `date_to` | `string` | no | Fecha final, inclusiva, en formato `YYYY-MM-DD`. |
| `type` | `string` | no | Filtra por el alcance del feriado. Valores: `national`, `state`, `municipal`, `optional`, `custom`. |
| `include_deleted` | `string` | no | Incluye feriados eliminados lógicamente. Valores: `true`, `false`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de feriados. |
| `404` | Recurso no encontrado en la organización actual. |

## holidays create

Crear feriado

```text
kobana billing holidays create [OPCIONES]
```

`POST /v1/holidays`

Crea un feriado. Úselo para feriados estatales, municipales y puntos facultativos — los nacionales llegan por la importación automática.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `date` | `string` | sí | Día civil del feriado, en formato `YYYY-MM-DD`. Sin hora: un feriado es una fecha del calendario, y aceptar un instante haría que el día guardado dependiera de la zona horaria del emisor. |
| `name` | `string` | sí | Nombre del feriado. Junto con la fecha identifica la fila — un feriado municipal y uno nacional pueden caer el mismo día. |
| `type` | `string` | no | Alcance: nacional, estatal, municipal, punto facultativo (`optional`) o propio de la organización (`custom`). Valores: `national`, `state`, `municipal`, `optional`, `custom`. |
| `notes` | `string` | no | Texto libre — la ciudad de un feriado municipal, o cualquier observación. |

Otros campos y objetos anidados: `kobana schema billing.holidays.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Feriado creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## holidays sync

Importar feriados nacionales

```text
kobana billing holidays sync [OPCIONES]
```

`POST /v1/holidays/sync`

Importa los feriados nacionales brasileños desde BrasilAPI para los años indicados (por omisión, el año corriente y los dos siguientes). Idempotente: importa solo lo que falta y nunca resucita un feriado eliminado a propósito.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `years` | `array<integer>` | no | Años a importar. Por omisión, el año corriente y los dos siguientes. |

Otros campos y objetos anidados: `kobana schema billing.holidays.sync`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resultado de la importación. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## holidays get

Consultar feriado

```text
kobana billing holidays get [ID] [OPCIONES]
```

`GET /v1/holidays/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Feriado encontrado. |
| `404` | Recurso no encontrado en la organización actual. |

## holidays update

Actualizar feriado

```text
kobana billing holidays update [ID] [OPCIONES]
```

`PATCH /v1/holidays/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `date` | `string` | no | Día civil del feriado, en formato `YYYY-MM-DD`. Sin hora: un feriado es una fecha del calendario, y aceptar un instante haría que el día guardado dependiera de la zona horaria del emisor. |
| `name` | `string` | no | Nombre del feriado. Junto con la fecha identifica la fila — un feriado municipal y uno nacional pueden caer el mismo día. |
| `type` | `string` | no | Alcance: nacional, estatal, municipal, punto facultativo (`optional`) o propio de la organización (`custom`). Valores: `national`, `state`, `municipal`, `optional`, `custom`. |
| `notes` | `string` | no | Texto libre — la ciudad de un feriado municipal, o cualquier observación. |

Otros campos y objetos anidados: `kobana schema billing.holidays.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Feriado actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## holidays delete

Eliminar feriado

```text
kobana billing holidays delete [ID] [OPCIONES]
```

`DELETE /v1/holidays/{id}`

Eliminación lógica. La fila permanece, y eso es lo que hace que la importación respete la remoción: un feriado quitado a propósito no vuelve en la próxima sincronización.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Feriado eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
