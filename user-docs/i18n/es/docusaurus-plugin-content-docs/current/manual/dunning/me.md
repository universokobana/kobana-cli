---
id: me
title: "kobana dunning me"
sidebar_label: "me"
description: "Identidad del token"
---

# kobana dunning me

Identidad del token

Comandos de `kobana dunning me`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidad del token |

## me

Identidad del token

```text
kobana dunning me [OPCIONES]
```

`GET /v1/me`

Devuelve quién es el dueño del token Bearer enviado. Sirve para probar si el token configurado es válido. Nunca devuelve el token ni ningún secreto.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Identidad del token. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
