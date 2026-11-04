---
id: me
title: "kobana finance me"
sidebar_label: "me"
description: "Obtener la identidad del token"
---

# kobana finance me

Obtener la identidad del token

Comandos de `kobana finance me`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`me`](#me) | `GET /v1/me` | Obtener la identidad del token |

## me

Obtener la identidad del token

```text
kobana finance me [OPCIONES]
```

`GET /v1/me`

Returns the person, organization and token details behind the Bearer token, so an integrator can test that the configured token is valid. Any valid token works: no scope and no `X-Workspace-Id` header are needed. Never returns the token itself.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | The identity behind the token. |
