---
id: pix-keys
title: "kobana billing pix-keys"
sidebar_label: "pix-keys"
---

# kobana billing pix-keys


Comandos de `kobana billing pix-keys`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`pix-keys list`](#pix-keys-list) | `GET /v1/pix_keys` | Listar claves Pix |
| [`pix-keys create`](#pix-keys-create) | `POST /v1/pix_keys` | Registrar clave Pix |
| [`pix-keys get`](#pix-keys-get) | `GET /v1/pix_keys/{id}` | Consultar clave Pix |
| [`pix-keys update`](#pix-keys-update) | `PATCH /v1/pix_keys/{id}` | Actualizar clave Pix |
| [`pix-keys delete`](#pix-keys-delete) | `DELETE /v1/pix_keys/{id}` | Eliminar clave Pix |

## pix-keys list

Listar claves Pix

```text
kobana billing pix-keys list [OPCIONES]
```

`GET /v1/pix_keys`

Claves Pix de la organización a las que los clientes transfieren en un cobro `bank_transfer` (`POST /invoices/{id}/pay`).

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `active` | `string` | no | Filtra por claves activas (`true`) o inactivas (`false`). Valores: `true`, `false`. |
| `key_type` | `string` | no | Filtra por tipo de clave. Valores: `cpf`, `cnpj`, `email`, `phone`, `random`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de claves Pix. |
| `404` | Recurso no encontrado en la organización actual. |

## pix-keys create

Registrar clave Pix

```text
kobana billing pix-keys create [OPCIONES]
```

`POST /v1/pix_keys`

El tipo se identifica por la propia clave (CPF, CNPJ, correo, teléfono con +55 o aleatoria); `key_type`, si se envía, debe coincidir. La clave se guarda sin máscara y es única en la organización.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | sí | La clave, sin máscara (documentos solo con dígitos, teléfono con `+55`, correo y aleatoria en minúsculas). |
| `key_type` | `string` | no | Tipo de clave: `cpf`, `cnpj`, `email`, `phone` o `random`. Valores: `cpf`, `cnpj`, `email`, `phone`, `random`. |
| `description` | `string` | no | Descripción libre para el operador (ej.: "Pix principal"). |
| `holder_name` | `string` | no | Titular mostrado al cliente. Vacío usa la razón social de la empresa emisora de la factura (o de la organización). |
| `holder_document` | `string` | no | CPF/CNPJ del titular, solo dígitos. Vacío usa el documento de la empresa emisora (o de la organización). |
| `company_id` | `string (uuid)` | no | Empresa emisora a la que pertenece (opcional). |
| `active` | `boolean` | no | Solo los destinos activos pueden elegirse en un nuevo cobro. |

Otros campos y objetos anidados: `kobana schema billing.pix-keys.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Clave Pix creada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## pix-keys get

Consultar clave Pix

```text
kobana billing pix-keys get [ID] [OPCIONES]
```

`GET /v1/pix_keys/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Clave Pix encontrada. |
| `404` | Recurso no encontrado en la organización actual. |

## pix-keys update

Actualizar clave Pix

```text
kobana billing pix-keys update [ID] [OPCIONES]
```

`PATCH /v1/pix_keys/{id}`

**Argumentos de la ruta:** `id`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `key` | `string` | no | La clave, sin máscara (documentos solo con dígitos, teléfono con `+55`, correo y aleatoria en minúsculas). |
| `key_type` | `string` | no | Tipo de clave: `cpf`, `cnpj`, `email`, `phone` o `random`. Valores: `cpf`, `cnpj`, `email`, `phone`, `random`. |
| `description` | `string` | no | Descripción libre para el operador (ej.: "Pix principal"). |
| `holder_name` | `string` | no | Titular mostrado al cliente. Vacío usa la razón social de la empresa emisora de la factura (o de la organización). |
| `holder_document` | `string` | no | CPF/CNPJ del titular, solo dígitos. Vacío usa el documento de la empresa emisora (o de la organización). |
| `company_id` | `string (uuid)` | no | Empresa emisora a la que pertenece (opcional). |
| `active` | `boolean` | no | Solo los destinos activos pueden elegirse en un nuevo cobro. |

Otros campos y objetos anidados: `kobana schema billing.pix-keys.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Clave Pix actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## pix-keys delete

Eliminar clave Pix

```text
kobana billing pix-keys delete [ID] [OPCIONES]
```

`DELETE /v1/pix_keys/{id}`

Eliminación lógica. Los cobros ya enviados siguen mostrando los datos que recibió el cliente, y la clave puede registrarse de nuevo.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Clave Pix eliminada. |
| `404` | Recurso no encontrado en la organización actual. |
