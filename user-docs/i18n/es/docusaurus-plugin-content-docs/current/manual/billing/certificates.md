---
id: certificates
title: "kobana billing certificates"
sidebar_label: "certificates"
---

# kobana billing certificates


Comandos de `kobana billing certificates`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`certificates list`](#certificates-list) | `GET /v1/certificates` | Listar certificados digitales |
| [`certificates create`](#certificates-create) | `POST /v1/certificates` | Subir certificado digital |
| [`certificates get`](#certificates-get) | `GET /v1/certificates/{id}` | Obtener certificado |
| [`certificates delete`](#certificates-delete) | `DELETE /v1/certificates/{id}` | Eliminar certificado |

## certificates list

Listar certificados digitales

```text
kobana billing certificates list [OPCIONES]
```

`GET /v1/certificates`

Devuelve una lista paginada de los certificados digitales (A1) de la organización actual. Por seguridad, la respuesta **nunca** incluye la clave privada ni la contraseña del PFX — solo metadatos extraídos del certificado.

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
| `status` | `string` | no | Filtra por estado del certificado. `expiring_soon` devuelve certificados activos que vencen en los próximos 30 días. Valores: `active`, `expired`, `revoked`, `expiring_soon`. |
| `search` | `string` | no | Búsqueda textual en `name`, `common_name`, `document` u `organization_name`. |
| `format` | `string` | no | Filtra por el formato del certificado almacenado (`pfx`, `pem`). |
| `document` | `string` | no | Filtra por el CPF o CNPJ extraído del certificado (solo dígitos). |
| `serial_number` | `string` | no | Filtra por el número de serie del certificado. |
| `not_after_from` | `string` | no | Devuelve certificados cuya fecha de expiración (`not_after`) es mayor o igual a la fecha indicada (ISO 8601). |
| `not_after_to` | `string` | no | Devuelve certificados cuya fecha de expiración (`not_after`) es menor o igual a la fecha indicada (ISO 8601). |
| `not_before_from` | `string` | no | Devuelve certificados cuya fecha de inicio de validez (`not_before`) es mayor o igual a la fecha indicada (ISO 8601). |
| `not_before_to` | `string` | no | Devuelve certificados cuya fecha de inicio de validez (`not_before`) es menor o igual a la fecha indicada (ISO 8601). |
| `created_after` | `string` | no | Devuelve certificados creados a partir de la fecha indicada (ISO 8601). |
| `created_before` | `string` | no | Devuelve certificados creados hasta la fecha indicada (ISO 8601). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de certificados. |
| `404` | Recurso no encontrado en la organización actual. |

## certificates create

Subir certificado digital

```text
kobana billing certificates create [OPCIONES]
```

`POST /v1/certificates`

Sube un certificado digital A1 vía `multipart/form-data`. La clave privada y la contraseña se cifran en reposo y **nunca** son devueltas por la API. El endpoint detecta duplicados por `thumbprint` y asocia automáticamente el certificado a un `person` por el CNPJ/CPF extraído.

Valores aceptados para `type`:
- `pfx` — enviar `file` (PFX/P12 binario) y `password`.
- `cert_key` — enviar `cert_file` (PEM del certificado) y `key_file` (PEM de la clave).
- `combined_pem` — enviar `file` (PEM con certificado + clave).
- `cert_key_password` — enviar `cert_file`, `key_file` y `password` (clave protegida por contraseña).

**Cuerpo de la solicitud**

Se envía como formulario `multipart/form-data`: los archivos van en `--file CAMPO=RUTA`.

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `type` | `string` | sí | Formato del envío: `pfx`, `cert_key`, `combined_pem` o `cert_key_password`. Valores: `pfx`, `cert_key`, `combined_pem`, `cert_key_password`. |
| `name` | `string` | no | Nombre amigable del certificado. Cuando se omite, se deriva de `common_name` o del documento. |
| `file` | `string (binary)` | no | Archivo binario (PFX/P12) cuando `type=pfx`, o PEM combinado cuando `type=combined_pem`. |
| `cert_file` | `string (binary)` | no | Archivo PEM del certificado (solo para `type=cert_key` o `type=cert_key_password`). |
| `key_file` | `string (binary)` | no | Archivo PEM de la clave privada (solo para `type=cert_key` o `type=cert_key_password`). |
| `password` | `string` | no | Contraseña del PFX (`type=pfx`) o de la clave privada (`type=cert_key_password`). Se almacena cifrada y nunca se devuelve. |
| `external_id` | `string` | no | Identificador en su sistema externo, para idempotencia/sincronización. |

Otros campos y objetos anidados: `kobana schema billing.certificates.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Certificado subido y almacenado con éxito. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |

## certificates get

Obtener certificado

```text
kobana billing certificates get [ID] [OPCIONES]
```

`GET /v1/certificates/{id}`

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle del certificado (sin datos sensibles). |
| `404` | Recurso no encontrado en la organización actual. |

## certificates delete

Eliminar certificado

```text
kobana billing certificates delete [ID] [OPCIONES]
```

`DELETE /v1/certificates/{id}`

Elimina el certificado mediante **soft delete**. Los datos cifrados permanecen en la base solo para fines de auditoría.

**Argumentos de la ruta:** `id`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Certificado eliminado. |
| `404` | Recurso no encontrado en la organización actual. |
