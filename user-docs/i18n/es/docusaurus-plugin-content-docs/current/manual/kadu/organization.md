---
id: organization
title: "kobana kadu organization"
sidebar_label: "organization"
description: "Organización del token"
---

# kobana kadu organization

Organización del token

Comandos de `kobana kadu organization`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`organization`](#organization) | `GET /v1/organization` | Organización del token |
| [`organization slack-installations list`](#organization-slack-installations-list) | `GET /v1/organization/slack_installations` | Instalaciones del bot de Slack de la organización |
| [`organization slack-installations update`](#organization-slack-installations-update) | `PATCH /v1/organization/slack_installations/{installationId}` | Configura una instalación de Slack |
| [`organization slack-installations delete`](#organization-slack-installations-delete) | `DELETE /v1/organization/slack_installations/{installationId}` | Quita una instalación de Slack |
| [`organization update`](#organization-update) | `PATCH /v1/organization` | Actualiza la organización del token |

## organization

Organización del token

```text
kobana kadu organization [OPCIONES] [COMANDO]
```

`GET /v1/organization`

Devuelve **solo la organización del token** (la del claim `org`). No recibe id y no lista: no hay forma de leer otra organización por esta API.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | not found |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## organization slack-installations list

Instalaciones del bot de Slack de la organización

```text
kobana kadu organization slack-installations list [OPCIONES]
```

`GET /v1/organization/slack_installations`

Workspaces de Slack vinculados a la organización del token (siempre la del token; sin id).

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## organization slack-installations update

Configura una instalación de Slack

```text
kobana kadu organization slack-installations update [INSTALLATIONID] [OPCIONES]
```

`PATCH /v1/organization/slack_installations/{installationId}`

Dónde responde el bot (`allowed_channel_types`), archivos de salida como enlace o adjunto (`files_mode`, `max_upload_bytes` hasta 100 MB) y detalle en el hilo (`log_detail`).

**Argumentos de la ruta:** `installationId`

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `allowed_channel_types` | `array<string>` | no |  |
| `files_mode` | `string` | no | Valores: `link`, `upload`. |
| `max_upload_bytes` | `integer` | no |  |
| `log_detail` | `string` | no | Valores: `milestones`, `verbose`. |

Otros campos y objetos anidados: `kobana schema kadu.organization.slack-installations.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `404` | NOT_FOUND |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## organization slack-installations delete

Quita una instalación de Slack

```text
kobana kadu organization slack-installations delete [INSTALLATIONID] [OPCIONES]
```

`DELETE /v1/organization/slack_installations/{installationId}`

Exige el nivel `delete` (o `all`). Revoca el token del bot en Slack y deshace las conexiones de cuenta.

**Argumentos de la ruta:** `installationId`

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `404` | NOT_FOUND |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## organization update

Actualiza la organización del token

```text
kobana kadu organization update [OPCIONES]
```

`PATCH /v1/organization`

Modifica **solo la organización del token** (la del claim `org`); no recibe id. `monthly_cost_limit` y `bypass_subscription` los define Kobana y se rechazan (403).

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | no |  |
| `timezone` | `string` | no |  |
| `default_locale` | `string` | no | Valores: `pt-BR`, `en-US`, `es-ES`. |
| `custom_metadata` | `object` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.organization.update`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
