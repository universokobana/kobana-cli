---
id: me
title: "kobana billing me"
sidebar_label: "me"
description: "Identidad del token actual"
---

# kobana billing me

Identidad del token actual

Comandos de `kobana billing me`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidad del token actual |

## me

Identidad del token actual

```text
kobana billing me [OPCIONES]
```

`GET /v1/me`

Devuelve a quién pertenece el token Bearer (persona y organización) y los alcances y la expiración del propio token. Sirve para probar si el token configurado es válido. Nunca devuelve el token ni secretos. `user` es `null` cuando el token no es de una persona.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Identidad del token. |
| `404` | Recurso no encontrado en la organización actual. |
