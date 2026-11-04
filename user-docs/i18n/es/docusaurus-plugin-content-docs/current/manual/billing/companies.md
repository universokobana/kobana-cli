---
id: companies
title: "kobana billing companies"
sidebar_label: "companies"
---

# kobana billing companies


Comandos de `kobana billing companies`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`companies fiscal`](#companies-fiscal) | `GET /v1/companies/{id}/fiscal` | Obtener perfil fiscal |
| [`companies fiscal update`](#companies-fiscal-update) | `PATCH /v1/companies/{id}/fiscal` | Actualizar perfil fiscal |
| [`companies fiscal-profile`](#companies-fiscal-profile) | `GET /v1/companies/{id}/fiscal-profile` | Obtener perfil fiscal (alias) |
| [`companies fiscal-profile update`](#companies-fiscal-profile-update) | `PATCH /v1/companies/{id}/fiscal-profile` | Actualizar perfil fiscal (alias) |
| [`companies matriz`](#companies-matriz) | `GET /v1/companies/matriz` | Listar matrices |
| [`companies list`](#companies-list) | `GET /v1/companies` | Listar empresas |
| [`companies create`](#companies-create) | `POST /v1/companies` | Crear empresa |
| [`companies update`](#companies-update) | `PUT /v1/companies` | Crear o actualizar empresa |
| [`companies get`](#companies-get) | `GET /v1/companies/{id}` | Obtener empresa |
| [`companies delete`](#companies-delete) | `DELETE /v1/companies/{id}` | Eliminar empresa |
| [`companies deactivate`](#companies-deactivate) | `POST /v1/companies/{id}/deactivate` | Desactivar empresa |
| [`companies reactivate`](#companies-reactivate) | `POST /v1/companies/{id}/reactivate` | Reactivar empresa |
| [`companies set-default`](#companies-set-default) | `POST /v1/companies/{id}/set-default` | Definir empresa por defecto |

## companies fiscal

Obtener perfil fiscal

```text
kobana billing companies fiscal [ID] [OPCIONES] [COMANDO]
```

`GET /v1/companies/{id}/fiscal`

Devuelve el perfil fiscal de la empresa, o `null` cuando aún no se ha configurado.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Perfil fiscal de la empresa (o `null`). |
| `404` | Recurso no encontrado en la organización actual. |

## companies fiscal update

Actualizar perfil fiscal

```text
kobana billing companies fiscal update [ID] [OPCIONES]
```

`PATCH /v1/companies/{id}/fiscal`

Crea o actualiza (upsert) el perfil fiscal de la empresa. Incluye régimen tributario, alícuotas (ISS, PIS, COFINS, CSLL, IRPJ) y configuración de CBS/IBS.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `taxation_regime` | `string` | sí | Régimen tributario: `simple_national`, `presumed_profit`, `real_profit` o `mei`. Valores: `simple_national`, `presumed_profit`, `real_profit`, `mei`. |
| `simples_annex` | `string` | no | Anexo del Simples Nacional: `annex_iii`, `annex_iv` o `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `rbt12` | `number` | no | Ingreso bruto acumulado en los últimos 12 meses, en BRL (float, hasta 4 decimales). |
| `factor_r` | `number` | no | Factor R (nómina ÷ ingreso), usado en la clasificación del Simples Nacional. |
| `cpp_inside_das` | `boolean` | no | Indica si la CPP (contribución previsional patronal) está incluida en el DAS. |
| `presumption_rate` | `number` | no | Alícuota de presunción (lucro presunto), en decimal (ej.: 0.32). |
| `iss_municipality` | `string` | no | Municipio donde se aplica el ISS. |
| `iss_rate` | `number` | no | Alícuota del ISS, en decimal. |
| `pis_rate` | `number` | no | Alícuota del PIS, en decimal. |
| `cofins_rate` | `number` | no | Alícuota de COFINS, en decimal. |
| `csll_rate` | `number` | no | Alícuota de CSLL, en decimal. |
| `irpj_rate` | `number` | no | Alícuota de IRPJ, en decimal. |
| `cbs_ibs_enabled` | `boolean` | no | Habilita los campos CBS/IBS (Reforma Tributaria brasileña). |
| `cbs_rate` | `number` | no | Alícuota de la CBS, en decimal. |
| `ibs_rate` | `number` | no | Alícuota del IBS, en decimal. |
| `cbs_ibs_mode` | `string` | no | Modo de cálculo CBS/IBS: `informative` (solo demostrativo) o `compositive` (compone el precio). Valores: `informative`, `compositive`. |

Otros campos y objetos anidados: `kobana schema billing.companies.fiscal.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Perfil fiscal actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## companies fiscal-profile

Obtener perfil fiscal (alias)

```text
kobana billing companies fiscal-profile [ID] [OPCIONES] [COMANDO]
```

`GET /v1/companies/{id}/fiscal-profile`

Alias de `GET /companies/{id}/fiscal` — misma representación y comportamiento.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Perfil fiscal de la empresa (o `null`). |
| `404` | Recurso no encontrado en la organización actual. |

## companies fiscal-profile update

Actualizar perfil fiscal (alias)

```text
kobana billing companies fiscal-profile update [ID] [OPCIONES]
```

`PATCH /v1/companies/{id}/fiscal-profile`

Alias de `PATCH /companies/{id}/fiscal` — misma representación y comportamiento.

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `taxation_regime` | `string` | sí | Régimen tributario: `simple_national`, `presumed_profit`, `real_profit` o `mei`. Valores: `simple_national`, `presumed_profit`, `real_profit`, `mei`. |
| `simples_annex` | `string` | no | Anexo del Simples Nacional: `annex_iii`, `annex_iv` o `annex_v`. Valores: `annex_iii`, `annex_iv`, `annex_v`. |
| `rbt12` | `number` | no | Ingreso bruto acumulado en los últimos 12 meses, en BRL (float, hasta 4 decimales). |
| `factor_r` | `number` | no | Factor R (nómina ÷ ingreso), usado en la clasificación del Simples Nacional. |
| `cpp_inside_das` | `boolean` | no | Indica si la CPP (contribución previsional patronal) está incluida en el DAS. |
| `presumption_rate` | `number` | no | Alícuota de presunción (lucro presunto), en decimal (ej.: 0.32). |
| `iss_municipality` | `string` | no | Municipio donde se aplica el ISS. |
| `iss_rate` | `number` | no | Alícuota del ISS, en decimal. |
| `pis_rate` | `number` | no | Alícuota del PIS, en decimal. |
| `cofins_rate` | `number` | no | Alícuota de COFINS, en decimal. |
| `csll_rate` | `number` | no | Alícuota de CSLL, en decimal. |
| `irpj_rate` | `number` | no | Alícuota de IRPJ, en decimal. |
| `cbs_ibs_enabled` | `boolean` | no | Habilita los campos CBS/IBS (Reforma Tributaria brasileña). |
| `cbs_rate` | `number` | no | Alícuota de la CBS, en decimal. |
| `ibs_rate` | `number` | no | Alícuota del IBS, en decimal. |
| `cbs_ibs_mode` | `string` | no | Modo de cálculo CBS/IBS: `informative` (solo demostrativo) o `compositive` (compone el precio). Valores: `informative`, `compositive`. |

Otros campos y objetos anidados: `kobana schema billing.companies.fiscal-profile.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Perfil fiscal actualizado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## companies matriz

Listar matrices

```text
kobana billing companies matriz [OPCIONES]
```

`GET /v1/companies/matriz`

Lista las empresas matrices (sin `parent_company_id`) activas de la organización. Útil para seleccionar la matriz al crear una sucursal. Solo aparecen empresas con CNPJ.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista de empresas matrices. |
| `404` | Recurso no encontrado en la organización actual. |

## companies list

Listar empresas

```text
kobana billing companies list [OPCIONES]
```

`GET /v1/companies`

Devuelve una lista paginada de las empresas (matrices y sucursales) de la organización actual.

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
| `status` | `string` | no | Filtra por estado: `active` o `inactive`. Valores: `active`, `inactive`. |
| `search` | `string` | no | Búsqueda textual por nombre, razón social, correo o documento. |
| `is_default` | `string` | no | Filtra solo la empresa marcada como predeterminada de la organización. Valores: `true`, `false`. |
| `parent_company_id` | `string` | no | Filtra sucursales por matriz. Usa el UUID de la matriz para listar sus sucursales. |
| `email_layout_id` | `string` | no | Filtra empresas por el UUID del diseño de correo asociado. |
| `created_after` | `string` | no | Inicio del rango de fecha de creación (YYYY-MM-DD). |
| `created_before` | `string` | no | Fin del rango de fecha de creación (YYYY-MM-DD). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de empresas. |
| `404` | Recurso no encontrado en la organización actual. |

## companies create

Crear empresa

```text
kobana billing companies create [OPCIONES]
```

`POST /v1/companies`

Crea una empresa (matriz o sucursal). Para crear una sucursal, informa `parent_company_id` — el CNPJ debe compartir el mismo prefijo que la matriz. Solo las empresas con CNPJ pueden tener sucursales. La primera empresa creada en la organización se marca como `is_default` automáticamente.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `name` | `string` | sí | Nombre comercial de la empresa. |
| `document_type` | `string` | sí | Tipo de documento: `cpf` o `cnpj`. Solo `cnpj` puede tener sucursales. Valores: `cpf`, `cnpj`. |
| `document_number` | `string` | sí | CPF o CNPJ (con o sin máscara — se normalizará a solo dígitos). |
| `email` | `string (email)` | sí | Correo principal de la empresa. |
| `parent_company_id` | `string (uuid)` | no | UUID de la matriz al crear una sucursal. Omite para crear una matriz. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `legal_name` | `string` | no | Razón social. |
| `phone` | `string` | no | Teléfono principal de la empresa. Cuando se informa, se añade al array `phones`. |
| `addresses` | `array<object>` | no | Lista de direcciones de la empresa. |
| `emails` | `array<object>` | no | Lista de correos adicionales. Si se omite, se usa el `email` principal. |
| `phones` | `array<object>` | no | Lista de teléfonos de la empresa. |
| `websites` | `array<object>` | no | Lista de sitios web de la empresa. |
| `is_default` | `boolean` | no | Cuando es `true`, marca esta empresa como predeterminada de la organización. Solo las matrices pueden ser predeterminadas. |
| `custom_metadata` | `object` | no | Metadatos personalizados (clave/valor libre). |

Otros campos y objetos anidados: `kobana schema billing.companies.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Empresa creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## companies update

Crear o actualizar empresa

```text
kobana billing companies update [OPCIONES]
```

`PUT /v1/companies`

Recibe el mismo cuerpo del POST, más `id` y `public_id`, y deduplica por `id`, `public_id`, `external_id` y las claves naturales del recurso, en ese orden, deteniéndose en la primera que encuentre un registro. Si lo encuentra, lo actualiza y responde 200; si no, lo crea y responde 201. Los campos obligatorios del POST solo se exigen cuando no se encuentra ningún registro, y la actualización es parcial.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `parent_company_id` | `string (uuid)` | no | UUID de la matriz al crear una sucursal. Omite para crear una matriz. |
| `name` | `string` | no | Nombre comercial de la empresa. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |
| `legal_name` | `string` | no | Razón social. |
| `document_type` | `string` | no | Tipo de documento: `cpf` o `cnpj`. Solo `cnpj` puede tener sucursales. Valores: `cpf`, `cnpj`. |
| `document_number` | `string` | no | CPF o CNPJ (con o sin máscara — se normalizará a solo dígitos). |
| `email` | `string (email)` | no | Correo principal de la empresa. |
| `phone` | `string` | no | Teléfono principal de la empresa. Cuando se informa, se añade al array `phones`. |
| `addresses` | `array<object>` | no | Lista de direcciones de la empresa. |
| `emails` | `array<object>` | no | Lista de correos adicionales. Si se omite, se usa el `email` principal. |
| `phones` | `array<object>` | no | Lista de teléfonos de la empresa. |
| `websites` | `array<object>` | no | Lista de sitios web de la empresa. |
| `is_default` | `boolean` | no | Cuando es `true`, marca esta empresa como predeterminada de la organización. Solo las matrices pueden ser predeterminadas. |
| `custom_metadata` | `object` | no | Metadatos personalizados (clave/valor libre). |
| `id` | `string (uuid)` | no | UUID del registro. Se usa solo para localizarlo — nunca se guarda. |
| `public_id` | `string (uuid)` | no | UUID público del registro, usado para localizarlo. |

Otros campos y objetos anidados: `kobana schema billing.companies.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Registro actualizado. |
| `201` | Registro creado. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## companies get

Obtener empresa

```text
kobana billing companies get [ID] [OPCIONES]
```

`GET /v1/companies/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la empresa. |
| `404` | Recurso no encontrado en la organización actual. |

## companies delete

Eliminar empresa

```text
kobana billing companies delete [ID] [OPCIONES]
```

`DELETE /v1/companies/{id}`

Elimina la empresa (soft delete: `deleted_at` — desaparece de toda lectura; las facturas y NFS-e emitidas siguen citándola). Exige el nivel de acceso `delete`. No es posible eliminar la única empresa activa ni una empresa con suscripciones activas o en período de prueba. Hasta octubre de 2026 esta ruta solo desactivaba la empresa — desactivar es `POST /v1/companies/{id}/deactivate`, reversible.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Empresa eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## companies deactivate

Desactivar empresa

```text
kobana billing companies deactivate [ID] [OPCIONES]
```

`POST /v1/companies/{id}/deactivate`

Desactiva la empresa. No es posible desactivar la única empresa activa ni una empresa con suscripciones activas o en período de prueba. Si la empresa desactivada era la predeterminada, otra empresa activa se promueve automáticamente.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Empresa desactivada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## companies reactivate

Reactivar empresa

```text
kobana billing companies reactivate [ID] [OPCIONES]
```

`POST /v1/companies/{id}/reactivate`

Reactiva una empresa previamente desactivada. Devuelve **422** cuando la empresa ya está activa.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Empresa reactivada. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## companies set-default

Definir empresa por defecto

```text
kobana billing companies set-default [ID] [OPCIONES]
```

`POST /v1/companies/{id}/set-default`

Marca la empresa como predeterminada de la organización. La empresa predeterminada anterior se desmarca en la misma transacción.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Empresa definida como predeterminada. |
| `404` | Recurso no encontrado en la organización actual. |
