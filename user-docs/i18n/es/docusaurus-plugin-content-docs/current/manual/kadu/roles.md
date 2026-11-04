---
id: roles
title: "kobana kadu roles"
sidebar_label: "roles"
---

# kobana kadu roles


Comandos de `kobana kadu roles`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.

| Comando | HTTP | Descripción |
|---|---|---|
| [`roles list`](#roles-list) | `GET /v1/roles` | Lista roles |
| [`roles create`](#roles-create) | `POST /v1/roles` | Crea rol |

## roles list

Lista roles

```text
kobana kadu roles list [OPCIONES]
```

`GET /v1/roles`

**Parámetros de consulta**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `cursor` | `string` | no |  |
| `limit` | `number` | no |  |
| `sort_by` | `string` | no | Campo en snake_case: created_at (predeterminado), updated_at o un campo filtrable. |
| `sort_order` | `string` | no | Predeterminado: desc. Valores: `asc`, `desc`. |

**Respuestas**

| HTTP | Descripción |
|---|---|
| `200` | OK |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |

## roles create

Crea rol

```text
kobana kadu roles create [OPCIONES]
```

`POST /v1/roles`

Permisos con el formato `*`, `agent.*`, `agent.<recurso>.*` o `agent.<recurso>.<acción>`. En un token de persona, ninguno puede ir más allá del rol de quien crea.

**Cuerpo de la solicitud**

| Nombre | Tipo | Obligatorio | Descripción |
|---|---|---|---|
| `label` | `string` | sí |  |
| `permissions` | `array<string>` | no |  |

Otros campos y objetos anidados: `kobana schema kadu.roles.create`.

**Respuestas**

| HTTP | Descripción |
|---|---|
| `201` | created |
| `400` | VALIDATION_ERROR |
| `409` | IDEMPOTENCY_KEY_REUSED / IDEMPOTENCY_REQUEST_IN_PROGRESS (o CONFLICT de la ruta) |
| `503` | AS_UNAVAILABLE — la verificación del token no está disponible (Authorization Server fuera de línea o configuración ausente). Intenta de nuevo (`Retry-After`). |
