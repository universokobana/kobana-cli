---
id: banks
title: "kobana billing banks"
sidebar_label: "banks"
description: "Listar bancos"
---

# kobana billing banks

Listar bancos

Comandos de `kobana billing banks`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`banks`](#banks) | `GET /v1/banks` | Listar bancos |

## banks

Listar bancos

```text
kobana billing banks [OPCIONES]
```

`GET /v1/banks`

Bancos brasileños con código COMPE (BACEN), usados en el registro de cuentas bancarias. La lista es la misma para todas las organizaciones y se actualiza semanalmente desde BrasilAPI.

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `page` | `integer` | no | Página (comienza en 1). Por defecto: `1`. |
| `per_page` | `integer` | no | Ítems por página. Por defecto: `20`. Máximo: `100`. |
| `sort_by` | `string` | no | Campo de ordenación. El valor por defecto depende del recurso. |
| `sort_order` | `string` | no | Dirección de ordenación. Por defecto: `desc`. Valores: `asc`, `desc`. |
| `search` | `string` | no | Búsqueda por parte del nombre o del código. |
| `code` | `string` | no | Código COMPE exacto (ej.: `341`). |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Lista paginada de bancos. |
| `404` | Recurso no encontrado en la organización actual. |
