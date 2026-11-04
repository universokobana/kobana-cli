---
id: syntax
title: Sintaxis de los comandos
sidebar_label: Sintaxis
sidebar_position: 1
description: Cómo se arma un comando — producto, recurso, método — y cómo pasar parámetros, cuerpo y archivos.
---

# Sintaxis de los comandos

```bash
kobana <producto> <recurso> <método> [argumentos] [flags]
```

- **`<producto>`**: `banking`, `finance`, `billing`, `inbox`, `dunning`, `dashboards` o `kadu` ([Productos](/reference/products)).
- **`<recurso>`**: puede ser anidado — `bank-billets`, `charge pix`, `webhooks deliveries`.
- **`<método>`**: la acción — `list`, `get`, `create`, `update`, `delete` o una acción del recurso, como `cancel`, `pause`, `replay`.

Ningún comando se escribe a mano: el CLI lee las especificaciones OpenAPI incluidas en el binario y arma el árbol en cada ejecución. Por eso el [Manual](/manual) se genera del propio binario.

## Cómo se eligen los nombres

| Ruta de la API | Comando |
|---|---|
| `GET /v1/bank_billets` | `kobana banking bank-billets list` |
| `GET /v1/bank_billets/{id}` | `kobana banking bank-billets get 123` |
| `POST /v2/charge/pix` | `kobana banking charge pix create` |
| `PUT /v1/bank_billets/{id}/cancel` | `kobana banking bank-billets cancel 123` |

- **La versión de la API nunca aparece en el comando.** `/v1` y `/v2` quedan a cargo del CLI.
- **Los recursos que solo leen se ejecutan por su nombre**: `kobana banking me`, `kobana billing organization`.
- **Las acciones cuyo nombre ya es un verbo** se ejecutan sin el verbo HTTP: `kobana banking webhook-deliveries resend`.
- Un `delete` nunca queda implícito.
- Los nombres antiguos siguen valiendo como alias, fuera de la ayuda.

## Identificadores de la ruta

Los ids de la ruta van como **argumentos**, en el orden en que aparecen:

```bash
kobana banking bank-billets get 123
kobana banking charge pix commands get PIX_UID COMMAND_UID
```

`--params '{"id": "123"}'` sigue valiendo; cuando aparecen los dos, el argumento gana.

## Parámetros y cuerpo

Los parámetros de URL, de consulta y de encabezado van en `--params`; el cuerpo de la solicitud va en `--json`:

```bash
kobana billing subscriptions list --params '{"status": "active", "per_page": 50}'
kobana banking charge pix create --json '{"amount": 99.90, "pix_account_uid": "UID"}'
```

## Archivos

`--file campo=ruta`, repetible, envía la solicitud como `multipart/form-data`. Cada campo de `--json` se convierte en una parte de texto; el tipo del archivo se deduce de la extensión (`.eml` se envía como `message/rfc822`). Solo los endpoints que reciben formulario aceptan `--file`.

```bash
kobana inbox emails create --file file=./mensaje.eml --json '{"inbox_id": "INBOX_ID"}'
kobana finance attachments create --file file=./nota.pdf --json '{"title": "NF 123"}'
```

## Ayuda en cualquier nivel

```bash
kobana --help
kobana finance --help
kobana finance payables --help
kobana finance payables list --help
```

Consulta todas las flags en [Opciones globales](/manual/commands/global-options).
