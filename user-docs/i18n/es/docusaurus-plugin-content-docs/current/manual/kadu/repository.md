---
id: repository
title: "kobana kadu repository"
sidebar_label: "repository"
description: "Kadu Playbook de la organización"
---

# kobana kadu repository

Kadu Playbook de la organización

Comandos de `kobana kadu repository`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`repository`](#repository) | `GET /v1/repository` | Kadu Playbook de la organización |
| [`repository sync`](#repository-sync) | `POST /v1/repository/sync` | Sincroniza el Kadu Playbook |

## repository

Kadu Playbook de la organización

```text
kobana kadu repository [OPCIONES] [COMANDO]
```

`GET /v1/repository`

Conectar el repositorio es la instalación de la GitHub App, hecha en la consola (flujo OAuth en el navegador).

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## repository sync

Sincroniza el Kadu Playbook

```text
kobana kadu repository sync [OPCIONES]
```

`POST /v1/repository/sync`

Encola la sincronización (el worker clona el repositorio y actualiza el catálogo de Tasks). Sigue `synced_at` y `sync_error` en `GET /repository`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `202` | accepted |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `422` | NO_PLUGIN_REPO / NO_DEPLOY_KEY |
| `503` | QUEUE_UNAVAILABLE / AS_UNAVAILABLE |
