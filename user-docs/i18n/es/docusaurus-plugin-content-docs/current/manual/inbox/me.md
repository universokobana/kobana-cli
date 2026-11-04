---
id: me
title: "kobana inbox me"
sidebar_label: "me"
description: "Identidad detrás del token"
---

# kobana inbox me

Identidad detrás del token

Comandos de `kobana inbox me`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Identidad detrás del token |

## me

Identidad detrás del token

```text
kobana inbox me [OPCIONES]
```

`GET /v1/me`

A quién representa el token Bearer: la persona (`null` para un token de máquina), la organización y el alcance del propio token. Úsalo para probar si un token configurado es válido. No exige scope de recurso: cualquier token que se verifique recibe respuesta. El token en sí nunca se devuelve.

**Encabezados**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `x-origin-shared-secret` | `string` | no | Secreto compartido entre CloudFront y el origin. Obligatorio en producción; se ignora en desarrollo a menos que se configure `CF_ORIGIN_SHARED_SECRET`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Identidad. |
