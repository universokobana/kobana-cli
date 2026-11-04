---
id: me
title: "kobana kadu me"
sidebar_label: "me"
description: "Identidad detrás del token"
---

# kobana kadu me

Identidad detrás del token

Comandos de `kobana kadu me`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidad detrás del token |

## me

Identidad detrás del token

```text
kobana kadu me [OPCIONES]
```

`GET /v1/me`

Devuelve la persona, la organización y el token (tipo, alcances, nivel de acceso, expiración) de la credencial Bearer. Sirve para probar si el token configurado es válido: es la única ruta exenta de la verificación de alcance, así que cualquier token válido responde. `user` es la persona del `sub` cuando el token trae correo, y null en un token de máquina. Nunca devuelve el token.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
