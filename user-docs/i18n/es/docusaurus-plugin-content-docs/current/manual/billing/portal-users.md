---
id: portal-users
title: "kobana billing portal-users"
sidebar_label: "portal-users"
description: "Lista usuarios del portal"
---

# kobana billing portal-users

Lista usuarios del portal

Comandos de `kobana billing portal-users`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`portal-users`](#portal-users) | `GET /v1/portal_users` | Lista usuarios del portal |

## portal-users

Lista usuarios del portal

```text
kobana billing portal-users [OPCIONES]
```

`GET /v1/portal_users`

Devuelve los usuarios del portal del cliente en la organización autenticada, con paginación. Los campos sensibles (contraseña, tokens) nunca se exponen.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `billing_account_id` | `string` | no | Restringe a una cuenta de cobro específica. |
| `customer_id` | `string` | no | Restringe a todas las cuentas de cobro de un cliente. Acepta el `id` del cliente o su `external_id`. |
| `status` | `string` | no | Filtra por estado: `active`, `pending_invitation`, `pending_verification`, `suspended` o `locked`. Valores: `active`, `pending_invitation`, `pending_verification`, `suspended`, `locked`. |
| `email` | `string` | no | Busca por fragmento del correo. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de usuarios del portal. |
| `404` | Recurso no encontrado en la organización actual. |
