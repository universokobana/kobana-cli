---
id: users
title: "kobana kadu users"
sidebar_label: "users"
description: "Miembros de la organización (hasta 100)"
---

# kobana kadu users

Miembros de la organización (hasta 100)

Comandos de `kobana kadu users`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`users`](#users) | `GET /v1/users` | Miembros de la organización (hasta 100) |

## users

Miembros de la organización (hasta 100)

```text
kobana kadu users [OPCIONES]
```

`GET /v1/users`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
