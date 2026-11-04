---
id: memberships
title: "kobana kadu memberships"
sidebar_label: "memberships"
description: "Lista vínculos"
---

# kobana kadu memberships

Lista vínculos

Comandos de `kobana kadu memberships`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`memberships`](#memberships) | `GET /v1/memberships` | Lista vínculos |

## memberships

Lista vínculos

```text
kobana kadu memberships [OPCIONES]
```

`GET /v1/memberships`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Campo en snake_case: created_at (predeterminado), updated_at o un campo filtrable. |
| `sort_order` | `string` | no | Predeterminado: desc. Valores: `asc`, `desc`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
