---
id: edi
title: "kobana banking edi"
sidebar_label: "edi"
description: "EDI (buzones EDI)"
---

# kobana banking edi

EDI (buzones EDI)

Comandos de `kobana banking edi`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`edi boxes list`](#edi-boxes-list) | `GET /v2/edi/edi_boxes` | Listar Buzones de Correo |
| [`edi boxes create`](#edi-boxes-create) | `POST /v2/edi/edi_boxes` | Crear Buzón de Correo |
| [`edi boxes get`](#edi-boxes-get) | `GET /v2/edi/edi_boxes/{uid}` | Ver Buzón de Correo |
| [`edi boxes update`](#edi-boxes-update) | `PUT /v2/edi/edi_boxes/{uid}` | Actualizar Buzón de Correo |

## edi boxes list

Listar Buzones de Correo

```text
kobana banking edi boxes list [OPCIONES]
```

`GET /v2/edi/edi_boxes`

Lista Buzones de Correo.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `resource_type` | `string` | no | Filtro por Tipo de Recurso. Cartera de Cobranza (`charge.bank_billet_account`) o Cuenta Financiera (`financial.account`). |
| `per_page` | `integer` | no | Cantidad de registros por página. Máximo de 50. |
| `page` | `integer` | no | Número de página. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Buzones de Correo encontrados. |

## edi boxes create

Crear Buzón de Correo

```text
kobana banking edi boxes create [OPCIONES]
```

`POST /v2/edi/edi_boxes`

Crea un nuevo Buzón de Correo.

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.edi.boxes.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | Buzón de Correo creado |
| `422` | Error de validación |

## edi boxes get

Ver Buzón de Correo

```text
kobana banking edi boxes get [UID] [OPCIONES]
```

`GET /v2/edi/edi_boxes/{uid}`

Consulta información de un Buzón de Correo específico.

**Argumentos de la ruta:** `uid`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Buzón de Correo encontrado |
| `404` | Buzón de Correo no encontrado |

## edi boxes update

Actualizar Buzón de Correo

```text
kobana banking edi boxes update [UID] [OPCIONES]
```

`PUT /v2/edi/edi_boxes/{uid}`

Actualiza un Buzón de Correo específico.

**Argumentos de la ruta:** `uid`

**Cuerpo de la solicitud**

El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema banking.edi.boxes.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Buzón de Correo actualizado |
| `404` | Buzón de Correo no encontrado |
| `422` | Error de validación |
