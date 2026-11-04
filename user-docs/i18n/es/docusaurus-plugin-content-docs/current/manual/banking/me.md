---
id: me
title: "kobana banking me"
sidebar_label: "me"
description: "Información de la cuenta"
---

# kobana banking me

Información de la cuenta

Comandos de `kobana banking me`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`me`](#me) | `GET /v2/me` | Obtener información del usuario autenticado |

## me

Obtener información del usuario autenticado

```text
kobana banking me [OPCIONES]
```

`GET /v2/me`

Devuelve información sobre el usuario autenticado, incluidos datos de la cuenta, usuario y token de acceso.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Información del usuario sin usuario asociado al token |
