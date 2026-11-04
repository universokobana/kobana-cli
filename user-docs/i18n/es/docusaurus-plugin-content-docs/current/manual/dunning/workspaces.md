---
id: workspaces
title: "kobana dunning workspaces"
sidebar_label: "workspaces"
description: "Listar workspaces"
---

# kobana dunning workspaces

Listar workspaces

Comandos de `kobana dunning workspaces`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`workspaces`](#workspaces) | `GET /v1/workspaces` | Listar workspaces |

## workspaces

Listar workspaces

```text
kobana dunning workspaces [OPCIONES]
```

`GET /v1/workspaces`

Lista los workspaces que alcanza el token — todos los de la organización para un token de integración. Envíe el `id` (o `public_id`) en el header `X-Workspace-Id` de las demás llamadas; es opcional mientras la organización tenga un solo workspace. Esta ruta no exige el header.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | Workspaces alcanzables. |
| `404` | Recurso inexistente o fuera de tu organización. |
| `413` | Cuerpo de la solicitud mayor que 2 MB. |
