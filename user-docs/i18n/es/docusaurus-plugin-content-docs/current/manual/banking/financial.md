---
id: financial
title: "kobana banking financial"
sidebar_label: "financial"
description: "Financiero (cuentas, saldos, extractos)"
---

# kobana banking financial

Financiero (cuentas, saldos, extractos)

Comandos de `kobana banking financial`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`financial accounts balances list`](#financial-accounts-balances-list) | `GET /v2/financial/accounts/{financial_account_uid}/balances` | Listar saldos |
| [`financial accounts balances create`](#financial-accounts-balances-create) | `POST /v2/financial/accounts/{financial_account_uid}/balances` | Crear un saldo |
| [`financial accounts balances get`](#financial-accounts-balances-get) | `GET /v2/financial/accounts/{financial_account_uid}/balances/{balance_uid}` | Ver un Saldo |
| [`financial accounts commands get`](#financial-accounts-commands-get) | `GET /v2/financial/accounts/{financial_account_uid}/commands/{uid}` | Ver un Comando |
| [`financial accounts commands list`](#financial-accounts-commands-list) | `GET /v2/financial/accounts/{financial_account_uid}/commands` | Listar Comandos de una Cuenta |
| [`financial accounts statement-transactions imports list`](#financial-accounts-statement-transactions-imports-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports` | Listar Importaciones de estado de cuenta |
| [`financial accounts statement-transactions imports create`](#financial-accounts-statement-transactions-imports-create) | `POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports` | Importar Estado de Cuenta |
| [`financial accounts statement-transactions imports get`](#financial-accounts-statement-transactions-imports-get) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports/{uid}` | Ver Importación de Estado de Cuenta |
| [`financial accounts statement-transactions summary`](#financial-accounts-statement-transactions-summary) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/summary` | Resumir Estado de Cuenta |
| [`financial accounts statement-transactions syncs list`](#financial-accounts-statement-transactions-syncs-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs` | Listar Sincronizaciones de Estado de Cuenta |
| [`financial accounts statement-transactions syncs get`](#financial-accounts-statement-transactions-syncs-get) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs/{uid}` | Consultar Sincronización de Estado de Cuenta |
| [`financial accounts statement-transactions sync`](#financial-accounts-statement-transactions-sync) | `POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/sync` | Sincronizar Estado de Cuenta |
| [`financial accounts statement-transactions list`](#financial-accounts-statement-transactions-list) | `GET /v2/financial/accounts/{financial_account_uid}/statement_transactions` | Listar Transacciones |
| [`financial accounts list`](#financial-accounts-list) | `GET /v2/financial/accounts` | Listar Cuentas |
| [`financial accounts create`](#financial-accounts-create) | `POST /v2/financial/accounts` | Crear una Cuenta |
| [`financial accounts get`](#financial-accounts-get) | `GET /v2/financial/accounts/{uid}` | Ver una Cuenta |
| [`financial accounts update`](#financial-accounts-update) | `PUT /v2/financial/accounts/{uid}` | Actualizar Cuenta |
| [`financial accounts delete`](#financial-accounts-delete) | `DELETE /v2/financial/accounts/{uid}` | Eliminar una Cuenta |
| [`financial companies list`](#financial-companies-list) | `GET /v2/financial/companies` | Listar Empresas |
| [`financial companies create`](#financial-companies-create) | `POST /v2/financial/companies` | Crear una Empresa |
| [`financial companies get`](#financial-companies-get) | `GET /v2/financial/companies/{uid}` | Consultar uma Empresa |
| [`financial companies update`](#financial-companies-update) | `PUT /v2/financial/companies/{uid}` | Actualizar una Empresa |
| [`financial companies delete`](#financial-companies-delete) | `DELETE /v2/financial/companies/{uid}` | Eliminar una Empresa |
| [`financial providers`](#financial-providers) | `GET /v2/financial/providers` | Listar Proveedores |
| [`financial statement-categories`](#financial-statement-categories) | `GET /v2/financial/statement_categories` | Listar Categorías de Estado de Cuenta |

## financial accounts balances list

Listar saldos

```text
kobana banking financial accounts balances list [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/balances`

Lista todos los saldos de una cuenta financiera específica. Devuelve información sobre valores disponibles, bloqueados e invertidos automáticamente.

**Argumentos de la ruta:** `financial_account_uid`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Saldos encontrados |

## financial accounts balances create

Crear un saldo

```text
kobana banking financial accounts balances create [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`POST /v2/financial/accounts/{financial_account_uid}/balances`

Crea un nuevo saldo manual para una cuenta financiera. Permite registrar saldos iniciales o ajustes manuales de conciliación.

**Argumentos de la ruta:** `financial_account_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `uid` | `string (uuid)` | no | UID del Saldo |
| `amount` | `number` | no | Valor del saldo |
| `blocked_amount` | `number` | no | Valor bloqueado |
| `automatically_invested_amount` | `number` | no | Valor automáticamente invertido |
| `created_at` | `string (date-time)` | no | Fecha de creación. Formato iso8601 |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. Opcional para control interno. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `tags` | `array<string>` | no | Etiquetas asociadas a la importación. Opcional para control interno. |

Otros campos y objetos anidados: `kobana schema banking.financial.accounts.balances.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Saldo creado |

## financial accounts balances get

Ver un Saldo

```text
kobana banking financial accounts balances get [FINANCIAL_ACCOUNT_UID] [BALANCE_UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/balances/{balance_uid}`

Consulta información específica de una Cuenta.

**Argumentos de la ruta:** `financial_account_uid`, `balance_uid`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `field` | `string` | no | Campo para buscar el ID. Ejemplo: `uid` o `external_id` |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta encontrada |
| `404` | Cuenta no encontrada |

## financial accounts commands get

Ver un Comando

```text
kobana banking financial accounts commands get [FINANCIAL_ACCOUNT_UID] [UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/commands/{uid}`

Consulta información detallada de un comando específico ejecutado en una cuenta financiera.

**Argumentos de la ruta:** `financial_account_uid`, `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comando encontrado |
| `404` | Comando no encontrado |

## financial accounts commands list

Listar Comandos de una Cuenta

```text
kobana banking financial accounts commands list [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/commands`

Lista todos los comandos ejecutados en una cuenta financiera. Los comandos representan operaciones asincrónicas como sincronización de estado de cuenta y otras acciones automatizadas.

**Argumentos de la ruta:** `financial_account_uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Comandos encontrados |
| `404` | Cuenta no encontrada |

## financial accounts statement-transactions imports list

Listar Importaciones de estado de cuenta

```text
kobana banking financial accounts statement-transactions imports list [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports`

Lista todas las importaciones de estado de cuenta realizadas para una cuenta financiera. Permite filtrar por estado, fecha de procesamiento, etiquetas y nombre del archivo importado.

**Argumentos de la ruta:** `financial_account_uid`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `status` | `string` | no | Filtro por estado de la importación |
| `source_file_name` | `string` | no | Nombre del archivo importado |
| `processed_from` | `string` | no | Fecha inicial del filtro de fecha de procesamiento de la importación |
| `processed_to` | `string` | no | Fecha final del filtro de fecha de procesamiento de la importación |
| `created_from` | `string` | no | Fecha inicial del filtro de fecha de creación de la importación |
| `created_to` | `string` | no | Fecha final del filtro de fecha de creación de la importación |
| `tags` | `string` | no | Filtro por etiquetas de la importación |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Importaciones de Estado de Cuenta encontradas |

## financial accounts statement-transactions imports create

Importar Estado de Cuenta

```text
kobana banking financial accounts statement-transactions imports create [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports`

Importa un estado de cuenta bancario mediante archivo CNAB. El procesamiento se realiza de forma asincrónica y las transacciones se asocian a la cuenta financiera.
> **Procesamiento Asíncrono:** Este endpoint se procesa de forma asincrónica y crea un comando de actualización para la Cuenta Financiera.

La Importación de Estado de Cuenta de una Cuenta Financiera se realiza de forma asincrónica, es decir, al recibir la solicitud,
nuestra API guarda los parámetros de la solicitud y procesa la importación en segundo plano.
Después de que se procese la importación, las transacciones se asociarán a la Cuenta Financiera.
Recibe en la respuesta de esta solicitud el UID de la Importación creada y puede consultarlo en cualquier momento.

**Eventos de Webhook**

Cuando se realiza la solicitud, un evento `import.created` es disparado.
Nuestro servidor entonces comienza el procesamiento de la importación.
Después del fin del procesamiento, el evento `import.processed` es disparado.

**Argumentos de la ruta:** `financial_account_uid`

**Cuerpo de la solicitud**

Se envía como formulario `multipart/form-data`: los archivos van en `--file CAMPO=RUTA`.

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `source` | `string (binary)` | sí | Archivo para importación. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. |
| `tags` | `array<string>` | no | Etiquetas asociadas |

Otros campos y objetos anidados: `kobana schema banking.financial.accounts.statement-transactions.imports.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Importación creada |

## financial accounts statement-transactions imports get

Ver Importación de Estado de Cuenta

```text
kobana banking financial accounts statement-transactions imports get [FINANCIAL_ACCOUNT_UID] [UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/imports/{uid}`

Consulta información específica de una Importación de Estado de Cuenta.

**Argumentos de la ruta:** `financial_account_uid`, `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Importación de Estado de Cuenta encontrada |
| `404` | Cuenta no encontrada |

## financial accounts statement-transactions summary

Resumir Estado de Cuenta

```text
kobana banking financial accounts statement-transactions summary [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/summary`

Agrega las transacciones del estado de cuenta en totales por grupo, en lugar de devolver una línea por transacción.

Responde preguntas como "cuánto gasté en tarifas en julio?" en una única solicitud, con la
aritmética hecha en la base de datos — sin paginar el estado de cuenta completo.

> **La ventana aquí puede llegar a 730 días**, contra los 180 del listado. El tamaño de la respuesta es
> limitado por la cantidad de grupos, no por el historial, entonces una ventana larga es segura.

Acepta los mismos filtros del listado. Si la combinación de `group_by` y ventana genera más de
1000 grupos, la solicitud es rechazada con `422` en lugar de devolver un resumen truncado — un total
cortado por la mitad se ve completo y engaña silenciosamente.

La respuesta también trae los bloques `filters` y `meta`, idénticos a los del listado, con la adición
de `filters.group_by` informando el agrupamiento aplicado.

**Argumentos de la ruta:** `financial_account_uid`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `group_by` | `string` | no | Dimensiones de agrupación separadas por coma: `day`, `week`, `month`, `kind`, `category`. Predeterminado: `month`. |
| `occurrence_date_from` | `string` | no | Fecha inicial de la ventana. Misma regla de derivación del listado, con límite de 730 días. |
| `occurrence_date_to` | `string` | no | Fecha final de la ventana. |
| `kind` | `string` | no | Mismo filtro del listado. Valores: `credit`, `debit`. |
| `category` | `string` | no | Mismo filtro del listado. |
| `q` | `string` | no | Mismo filtro del listado. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Resumen calculado |

## financial accounts statement-transactions syncs list

Listar Sincronizaciones de Estado de Cuenta

```text
kobana banking financial accounts statement-transactions syncs list [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs`

Lista las sincronizaciones de estado de cuenta solicitadas para una Cuenta Financiera, de la más reciente a la más antigua.

Cada elemento representa una solicitud realizada en
Sincronizar Estado de Cuenta
e agrega los comandos creados para la ventana solicitada — ventanas que cruzan más de un mes generan un comando por mes.

El campo `commands_summary` trae el conteo de comandos por estado, lo que permite identificar cuántos meses
fallaron sin necesidad de abrir la solicitud. Para ver los comandos individualmente, use la consulta por `uid`.

> Solo las sincronizaciones solicitadas vía API aparecen aquí. Las sincronizaciones automáticas periódicas no generan registro.

**Argumentos de la ruta:** `financial_account_uid`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |
| `status` | `string` | no | Filtra por estado agregado de la sincronización. Valores: `pending`, `processing`, `done`, `partial`, `failed`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Sincronizaciones encontradas |

## financial accounts statement-transactions syncs get

Consultar Sincronización de Estado de Cuenta

```text
kobana banking financial accounts statement-transactions syncs get [FINANCIAL_ACCOUNT_UID] [UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs/{uid}`

Consulta el estado de una sincronización de estado de cuenta solicitada.

El estado se recalcula en cada consulta a partir de los comandos de la solicitud:

* `pending` - ningún comando comenzó a ser procesado
* `processing` - en curso (parte de los comandos ya terminó)
* `done` - todos los comandos fueron confirmados y las transacciones ya están disponibles en
  Listar Transacciones
* `partial` - todos terminaron, pero parte falló. El array `commands` muestra qué ventana falló y por qué
* `failed` - todos terminaron y ninguno fue confirmado

El array `commands` trae cada comando con su propia ventana en `params.start_at`/`params.end_at` y el
mensaje de error en `error_message`, cuando lo haya. Es por él que se identifica **cuál mes** falló en una
sincronización multi-mes.

**Argumentos de la ruta:** `financial_account_uid`, `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Sincronización encontrada |
| `404` | Sincronización no encontrada |

## financial accounts statement-transactions sync

Sincronizar Estado de Cuenta

```text
kobana banking financial accounts statement-transactions sync [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`POST /v2/financial/accounts/{financial_account_uid}/statement_transactions/sync`

Sincroniza el estado de cuenta bancario de una cuenta financiera. Busca transacciones directamente de la institución financiera mediante API. Requiere conexión activa.
> **Procesamiento Asíncrono:** Este endpoint se procesa de forma asincrónica y crea un comando de sincronización de estado de cuenta para la Cuenta Financiera.

La Sincronización de Estado de Cuenta de una Cuenta Financiera se realiza de forma asincrónica, es decir, al recibir la solicitud, nuestra
API guarda los parámetros de la solicitud y crea un Comando. Este comando se procesa en segundo plano y después se confirma
y las transacciones se importarán en la Cuenta Financiera. Recibe en la respuesta de esta solicitud el ID del comando creado
y puede consultarlo en cualquier momento.

Vea más en Listar Comandos de una Cuenta Financiera y
Ver un Comando.

Para que sea posible ejecutar la Sincronización de Estado de Cuenta, debe haber una Conexión activa (es decir, validada y habilitada)
asociada a la Cuenta Financiera.

**Período de la Sincronización**

Los campos `start_at` y `end_at` son opcionales y delimitan la ventana a sincronizar.
Cuando se omiten, la ventana predeterminada va del **primer día del mes corriente hasta hoy**.

* informando solo `start_at`, la fecha final asumida es hoy;
* informando solo `end_at`, la fecha inicial asumida es el primer día del mes de `end_at`;
* la fecha final no puede ser futura;
* el período máximo es de **2 años** hacia atrás a partir de hoy.

Ventanas que cruzan más de un mes-calendario se **dividen en un comando por mes**. Esto mantiene cada
solicitud a la institución financiera dentro de un volumen que el banco puede responder, y permite
reprocesar solo el mes que eventualmente falle. Una ventana dentro del mismo mes continúa creando
un único comando.

**Acompañando la Importación**

Además del comando en `data`, la respuesta trae en `sync` el registro de la importación solicitada, con un `uid`
propio. Use este `uid` para acompañar el conjunto completo — especialmente en ventanas multi-mes, donde
varios comandos se crean — en
Consultar Sincronización de Estado de Cuenta.

**Eventos de Webhook**

Cuando se realiza la solicitud, un evento `financial.account.statement_sync.requested` es disparado.
Nuestro servidor entonces comienza la comunicación con la API de la institución financiera para buscar transacciones.
Solo después de confirmado en la institución financiera, entonces el estado de cuenta se actualiza.
En este momento el evento `financial.account.statement_sync.confirmed` es disparado.

**Argumentos de la ruta:** `financial_account_uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `start_at` | `string (date)` | no | Fecha inicial de la sincronización. Predeterminado: primer día del mes corriente. Máximo de 2 años atrás. |
| `end_at` | `string (date)` | no | Fecha final de la sincronización. Predeterminado: hoy. No puede ser futura. |

Otros campos y objetos anidados: `kobana schema banking.financial.accounts.statement-transactions.sync`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Comando creado por mes para ventana multi-mes |
| `422` | Conexión para Estado de Cuenta no activa |

## financial accounts statement-transactions list

Listar Transacciones

```text
kobana banking financial accounts statement-transactions list [FINANCIAL_ACCOUNT_UID] [OPCIONES]
```

`GET /v2/financial/accounts/{financial_account_uid}/statement_transactions`

Lista las transacciones del estado de cuenta de una cuenta financiera, filtrando por período, tipo, valor y descripción.

**Ventana de fechas — cambio programado**

Hoy, una solicitud sin fechas devuelve el estado de cuenta completo de la cuenta. Este comportamiento
**cambiará el 05/02/2027**: la consulta pasará a ejecutarse siempre dentro de una ventana de no
más de 180 días, que es lo que mantiene el endpoint rápido en cuentas con historial largo.

Puede adoptar el comportamiento nuevo desde ya enviando `date_window=strict`, y debe hacerlo
antes de la fecha de corte. Con él:

| Parámetros enviados | Ventana consultada |
|---|---|
| ninguno | Los últimos 180 días |
| solo `occurrence_date_from` | 180 días a partir de ella |
| solo `occurrence_date_to` | 180 días antes de ella |
| ambos | El período informado, siempre que no supere 180 días |

Un intervalo mayor a 180 días es rechazado con `422` en lugar de ser reducido silenciosamente — un
estado de cuenta parcial con apariencia de completo es peor que un error. Para períodos más largos,
haga llamadas sucesivas de hasta 180 días cada una.

Mientras `date_window=strict` no se envíe, nada cambia. Las solicitudes cuyo resultado **sería
diferente** bajo la ventana estricta responden con los encabezados `Deprecation: true` y
`Sunset: Fri, 05 Feb 2027 00:00:00 GMT` — úselos para descubrir cuál de sus integraciones
necesita ajuste.

**Argumentos de la ruta:** `financial_account_uid`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página. Los valores por encima del máximo se reducen al máximo. |
| `date_window` | `string` | no | Envíe `strict` para adoptar desde ya la ventana de 180 días que pasa a ser el estándar el 05/02/2027. Sin él, el comportamiento actual se preserva. Valores: `strict`. |
| `occurrence_date_from` | `string` | no | Fecha inicial del filtro de fecha de ocurrencia. Con `date_window=strict`, si se omite se deriva de `occurrence_date_to` (180 días antes) o, en ausencia de ambas, de hoy. |
| `occurrence_date_to` | `string` | no | Fecha final del filtro de fecha de ocurrencia. Con `date_window=strict`, si se omite se deriva de `occurrence_date_from` (180 días después) o, en ausencia de ambas, es hoy. |
| `kind` | `string` | no | Filtra por tipo de transacción. Valores: `credit`, `debit`. |
| `amount_from` | `number` | no | Valor mínimo de la transacción, en reales y con signo — débitos son negativos, así como en el campo `amount` de la respuesta. Use punto como separador decimal. |
| `amount_to` | `number` | no | Valor máximo de la transacción, en reales y con signo — débitos son negativos, así como en el campo `amount` de la respuesta. Use punto como separador decimal. |
| `amount_abs_from` | `number` | no | Valor mínimo en módulo, ignorando el signo. Use para "más de X, entrada o salida". |
| `amount_abs_to` | `number` | no | Valor máximo en módulo, ignorando el signo. |
| `category` | `string` | no | Código de categoría, o lista separada por coma. Consulte el catálogo en `GET /v2/financial/statement_categories`. Los códigos desconocidos se ignoran. |
| `updated_since` | `string` | no | Devuelve solo transacciones cambiadas a partir de este instante (ISO 8601). Sirve para sincronización incremental. |
| `q` | `string` | no | Búsqueda textual en la descripción de la transacción enviada por el banco (`description`). Coincide con cualquier parte del texto, sin diferenciar mayúsculas, minúsculas o acentos — `mantenimiento` encuentra `MANTENIMIENTO`. |
| `view` | `string` | no | Use `compact` para recibir solo los campos esenciales de cada transacción. Valores: `compact`. |
| `fields` | `string` | no | Lista de campos separados por coma a devolver en cada transacción. Tiene precedencia sobre `view`. Un campo desconocido resulta en `422`. |
| `pagination` | `string` | no | Use `keyset` para paginación por cursor, estable cuando nuevas transacciones llegan durante el barrido. La respuesta pasa a traer `next_cursor` en lugar de las URLs. Valores: `keyset`. |
| `cursor` | `string` | no | Cursor devuelto en `pagination.next_cursor`. Solo tiene efecto con `pagination=keyset`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Transacciones encontradas |
| `422` | Intervalo de fechas mayor a 180 días (con date_window=strict) |

## financial accounts list

Listar Cuentas

```text
kobana banking financial accounts list [OPCIONES]
```

`GET /v2/financial/accounts`

Lista todas las cuentas financieras registradas. Devuelve información sobre las cuentas bancarias incluyendo proveedor, tipo de cuenta, datos bancarios y metadatos personalizados.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuentas encontradas |

## financial accounts create

Crear una Cuenta

```text
kobana banking financial accounts create [OPCIONES]
```

`POST /v2/financial/accounts`

Crea una nueva cuenta financiera para gestión de transacciones bancarias. Soporta cuentas corrientes y de ahorro.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `account_number` | `string` | sí | Número de Cuenta |
| `account_digit` | `string` | sí | Dígito de la Cuenta |
| `agency_number` | `string` | sí | Número de Agencia |
| `financial_provider_slug` | `string` | sí | Proveedor Financiero |
| `person_info` | `object` | sí | Datos del Titular |
| `uid` | `string (uuid)` | no | UID de la Cuenta Financiera |
| `kind` | `string` | no | Tipo de la Cuenta `checking`: Cuenta Corriente `savings`: Cuenta de Ahorros `payment`: Cuenta de Pago |
| `agency_digit` | `string` | no | Dígito da Agencia. Opcional: será calculado automaticamente usando Módulo 11 quando o banco exigir e não for fornecido. |
| `bank_number` | `integer` | no | Número COMPE del Banco. Obligatorio si bank_id no se informa. |
| `created_at` | `string (date-time)` | no | Fecha de creación. Formato iso8601 |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. Opcional para control interno. |
| `custom_name` | `string` | no | Nombre de la Cuenta Financiera para identificación dentro de Kobana. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `company_uid` | `string (uuid)` | no | UID de la Empresa (`/v2/financial/companies`) a la que pertenece la cuenta. Un UID inexistente, de otra cuenta o malformado se rechaza con 422 (`unresolvable_uid`). `null` o vacío desvincula la cuenta de la empresa; sin el campo, la empresa no cambia. |
| `payment_agreement_code` | `string` | no | Número del Acuerdo de Pago contratado junto al Banco. |
| `tags` | `array<string>` | no | Etiquetas asociadas a la importación. Opcional para control interno. |
| `updated_at` | `string (date-time)` | no | Fecha de la última actualización. Formato iso8601 |
| `service_accounts` | `array<object>` | no | Cuentas de servicio asociadas a esta cuenta financiera. |

Otros campos y objetos anidados: `kobana schema banking.financial.accounts.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Cuenta creada |
| `422` | Error de validación |

## financial accounts get

Ver una Cuenta

```text
kobana banking financial accounts get [UID] [OPCIONES]
```

`GET /v2/financial/accounts/{uid}`

Consulta información específica de una Cuenta.

**Argumentos de la ruta:** `uid`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `field` | `string` | no | Campo para buscar el ID. Ejemplo: `uid` o `external_id` |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta encontrada |
| `404` | Cuenta no encontrada |

## financial accounts update

Actualizar Cuenta

```text
kobana banking financial accounts update [UID] [OPCIONES]
```

`PUT /v2/financial/accounts/{uid}`

Actualiza una Cuenta Financiera específica.

**Argumentos de la ruta:** `uid`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `field` | `string` | no | Campo para buscar el ID. Ejemplo: `uid` o `external_id` |

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `kind` | `string` | no | Tipo de la Cuenta `checking`: Cuenta Corriente `savings`: Cuenta de Ahorros `payment`: Cuenta de Pago |
| `account_number` | `string` | no | Número de Cuenta |
| `account_digit` | `string` | no | Dígito de la Cuenta |
| `agency_number` | `string` | no | Número de Agencia |
| `agency_digit` | `string` | no | Dígito da Agencia. Opcional: será calculado automaticamente usando Módulo 11 quando o banco exigir e não for fornecido. |
| `custom_data` | `object` | no | Hash con clave y valor en formato JSON. Opcional para control interno. |
| `custom_name` | `string` | no | Nombre de la Cuenta Financiera para identificación dentro de Kobana. |
| `external_id` | `string` | no | ID en su sistema. Opcional para control interno. |
| `company_uid` | `string (uuid)` | no | UID de la Empresa (`/v2/financial/companies`) a la que pertenece la cuenta. Un UID inexistente, de otra cuenta o malformado se rechaza con 422 (`unresolvable_uid`). `null` o vacío desvincula la cuenta de la empresa; sin el campo, la empresa no cambia. |
| `payment_agreement_code` | `string` | no | Número del Acuerdo de Pago contratado junto al Banco. |
| `person_info` | `object` | no | Datos del Titular |
| `tags` | `array<string>` | no | Etiquetas asociadas a la importación. Opcional para control interno. |

Otros campos y objetos anidados: `kobana schema banking.financial.accounts.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Cuenta Financiera actualizada |
| `404` | Cuenta Financiera no encontrada |
| `422` | Error de validación |

## financial accounts delete

Eliminar una Cuenta

```text
kobana banking financial accounts delete [UID] [OPCIONES]
```

`DELETE /v2/financial/accounts/{uid}`

Elimina una Cuenta Financiera. La eliminación se procesa de forma asincrónica.

⚠️ Atención: esta operación elimina en cascada todos los registros
asociados a la Cuenta Financiera, incluyendo carteras de boleto,
transferencias, pagos, estados de cuenta y demás recursos vinculados.
Esta acción no se puede deshacer.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | Cuenta enviada para eliminación |
| `404` | Cuenta Financiera no encontrada |

## financial companies list

Listar Empresas

```text
kobana banking financial companies list [OPCIONES]
```

`GET /v2/financial/companies`

Lista las empresas (CNPJs) de la cuenta, las más recientes primero.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cnpj` | `string` | no | Filtra por el CNPJ (con o sin máscara). |
| `active` | `boolean` | no | Filtra por empresas activas o inactivas. |
| `external_id` | `string` | no | Filtra por el ID en su sistema. |
| `page` | `integer` | no | Número de Página |
| `per_page` | `integer` | no | Cantidad de registros por página |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Empresas encontradas |

## financial companies create

Crear una Empresa

```text
kobana banking financial companies create [OPCIONES]
```

`POST /v2/financial/companies`

Crea una empresa (CNPJ) en la cuenta. Contactos en formato de Persona: listas de direcciones, correos electrónicos y teléfonos.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Razón social. |
| `cnpj` | `string` | sí | CNPJ sin máscara, en mayúsculas (acepta el CNPJ alfanumérico). |
| `trade_name` | `string` | no | Nombre comercial. |
| `active` | `boolean` | no | Empresa activa. Solo una empresa activa por CNPJ en la cuenta. |
| `municipal_registration` | `string` | no | Inscripción municipal. |
| `tax_regime` | `string` | no | Régimen tributario. Valores: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`, `null`. |
| `addresses` | `array<object>` | no |  |
| `emails` | `array<object>` | no |  |
| `phones` | `array<object>` | no |  |
| `external_id` | `string` | no | ID en su sistema. Único por cuenta. |
| `tags` | `array<string>` | no |  |
| `custom_data` | `object` | no | Datos personalizados. |

Otros campos y objetos anidados: `kobana schema banking.financial.companies.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Empresa creada |
| `422` | Error de validación |

## financial companies get

Consultar uma Empresa

```text
kobana banking financial companies get [UID] [OPCIONES]
```

`GET /v2/financial/companies/{uid}`

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Empresa encontrada |
| `404` | Empresa no encontrada |

## financial companies update

Actualizar una Empresa

```text
kobana banking financial companies update [UID] [OPCIONES]
```

`PUT /v2/financial/companies/{uid}`

Actualiza la empresa. Sus cuentas financieras pasan a tener como titular los datos de la empresa.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | no | Razón social. |
| `trade_name` | `string` | no | Nombre comercial. |
| `cnpj` | `string` | no | CNPJ sin máscara, en mayúsculas (acepta el CNPJ alfanumérico). |
| `active` | `boolean` | no | Empresa activa. Solo una empresa activa por CNPJ en la cuenta. |
| `municipal_registration` | `string` | no | Inscripción municipal. |
| `tax_regime` | `string` | no | Régimen tributario. Valores: `SimplesNacional`, `SimplesNacionalExcessoSublimite`, `MicroempreendedorIndividual`, `LucroPresumido`, `LucroReal`, `Isento`, `null`. |
| `addresses` | `array<object>` | no |  |
| `emails` | `array<object>` | no |  |
| `phones` | `array<object>` | no |  |
| `external_id` | `string` | no | ID en su sistema. Único por cuenta. |
| `tags` | `array<string>` | no |  |
| `custom_data` | `object` | no | Datos personalizados. |

Otros campos y objetos anidados: `kobana schema banking.financial.companies.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Empresa actualizada |

## financial companies delete

Eliminar una Empresa

```text
kobana banking financial companies delete [UID] [OPCIONES]
```

`DELETE /v2/financial/companies/{uid}`

Elimina la empresa. Sus cuentas financieras se mantienen, sin empresa.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `204` | Empresa eliminada |

## financial providers

Listar Proveedores

```text
kobana banking financial providers [OPCIONES]
```

`GET /v2/financial/providers`

Este endpoint se utiliza para recibir los proveedores de servicio que Kobana está integrada.

Un proveedor de servicio es cualquier institución regulada por el BACEN, sea un banco comercial, institución financiera, institución de pagos, proveedor de servicio de pagos, etc.

El `slug` del proveedor de servicio se utiliza para la creación de una cuenta financiera.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Proveedores encontrados |

## financial statement-categories

Listar Categorías de Estado de Cuenta

```text
kobana banking financial statement-categories [OPCIONES]
```

`GET /v2/financial/statement_categories`

Catálogo de las categorías que una transacción de estado de cuenta puede llevar.

Necesario para usar el filtro `category` del listado y del resumen: los códigos son un dominio
cerrado, y sin el catálogo no hay forma de descubrir que las tarifas bancarias son el código `105`.

El catálogo tiene dos naturalezas: aproximadamente 50 códigos genéricos (`1xx` débitos, `2xx` créditos,
y `9999` para `otros`) y cientos de códigos específicos de banco, en el formato
`1 + número_del_banco + código_del_banco` (ej.: `107701`).

`kind` se deriva del slug y viene nulo cuando la categoría no declara un lado — que es el caso
de la mayoría de los códigos específicos de banco. Es una conveniencia, no una clasificación
exhaustiva: para sumar entradas y salidas, use el filtro `kind` del listado o el `group_by=kind`
del resumen, que operan sobre el tipo real de la transacción.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Catálogo de categorías |
