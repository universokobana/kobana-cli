---
id: organization
title: "kobana billing organization"
sidebar_label: "organization"
description: "Obtener organización actual"
---

# kobana billing organization

Obtener organización actual

Comandos de `kobana billing organization`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Obtener organización actual |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Actualizar organización actual |

## organization

Obtener organización actual

```text
kobana billing organization [OPCIONES] [COMANDO]
```

`GET /v1/organization`

Devuelve los datos de la organización a la que pertenece el token autenticado. No hay parámetro `:id` — la organización siempre se resuelve a partir de la clave de la API.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Detalle de la organización actual. |
| `404` | Recurso no encontrado en la organización actual. |

## organization update

Actualizar organización actual

```text
kobana billing organization update [OPCIONES]
```

`PATCH /v1/organization`

Actualiza el perfil que controla la organización autenticada: `addresses` y `custom_metadata` (`null` mantiene el valor actual). La identidad (`id`, `external_id`, `status`, documento) y los campos reflejados de la Cuenta Kobana (`name`, `email`, `phone`) no se pueden modificar por este endpoint: si se envían, se ignoran — para corregirlos, cambie la Cuenta Kobana. Los campos comerciales (como `courtesy` y `bypass_subscription`) se rechazan con `403`.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `addresses` | `array<object>` | no | Lista de direcciones de la organización. |
| `custom_metadata` | `object` | no | Metadatos arbitrarios definidos por el cliente (clave/valor). `null` se ignora y mantiene el valor actual. |

Otros campos y objetos anidados: `kobana schema billing.organization.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Organización actualizada. |
| `400` | El cuerpo de la solicitud falló en la validación. |
| `404` | Recurso no encontrado en la organización actual. |
| `422` | El cuerpo de la solicitud falló en la validación. |
