---
id: userinfo
title: "kobana banking userinfo"
sidebar_label: "userinfo"
description: "Información del Usuario (descontinuado)"
---

# kobana banking userinfo

Información del Usuario (descontinuado)

Comandos de `kobana banking userinfo`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`userinfo`](#userinfo) | `GET /v1/userinfo` | Información del Usuario (descontinuado) |

## userinfo

Información del Usuario (descontinuado)

```text
kobana banking userinfo [OPCIONES]
```

`GET /v1/userinfo`

Consulta información de la cuenta accedida.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Usuario encontrado |
| `404` | Usuario no encontrado |
