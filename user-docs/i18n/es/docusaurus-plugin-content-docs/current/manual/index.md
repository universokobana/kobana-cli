---
id: index
title: Manual
sidebar_label: Visión general
sidebar_position: 1
slug: /manual
description: Todos los comandos de kobana, por producto y recurso, generados desde el propio binario.
---

# Manual

El manual lista **todos los comandos** de `kobana`: qué hace cada uno, el método y la ruta HTTP, los parámetros, el cuerpo de la solicitud y las respuestas. Se genera desde el propio binario — `--help`, `kobana schema` y las especificaciones OpenAPI incluidas —, así que acompaña cada versión y cada idioma.

```bash
kobana <producto> <recurso> <método> [flags]
```

## Comandos del CLI

| Comando | Para qué |
|---|---|
| [Opciones globales](/manual/commands/global-options) | Las flags que acepta todo comando: `--params`, `--json`, `--fields`, `--dry-run`… |
| [`kobana schema`](/manual/commands/schema) | Parámetros, cuerpo y respuestas de un endpoint |
| [`kobana completions`](/manual/commands/completions) | Autocompletado para bash, zsh, fish, PowerShell y elvish |
| [`kobana update`](/manual/commands/update) | Busca una versión nueva y actualiza el CLI |

## Productos

| Producto | Comando |
|---|---|
| Gateway Bancario | [`kobana banking`](/manual/banking) |
| Financiero Inteligente | [`kobana finance`](/manual/finance) |
| Facturación Automática | [`kobana billing`](/manual/billing) |
| Inbox Autónomo | [`kobana inbox`](/manual/inbox) |
| Cobranza | [`kobana dunning`](/manual/dunning) |
| Dashboard Mágico | [`kobana dashboards`](/manual/dashboards) |
| Agente Kadu | [`kobana kadu`](/manual/kadu) |

Cada producto tiene una página por recurso, con una sección por comando.

## Cómo leer una página

- **El bloque `text`** es la línea de uso, tal como la imprime `--help`. `[ID]` son los identificadores de la ruta, en el orden de la ruta; `[OPCIONES]`, las [opciones globales](/manual/commands/global-options).
- **`GET /v1/…`** es la llamada que hace el comando. La versión de la API aparece solo ahí, nunca en el comando.
- **Los parámetros de consulta, encabezados y cuerpo** van en `--params` y `--json`. La columna *Obligatorio* indica lo que exige la API.
- **Cuerpo de la solicitud** muestra los campos del primer nivel. Para objetos anidados, ejecuta el `kobana schema` indicado en la página.

## Errores comunes a todos los comandos

Todos los endpoints pueden responder con estos códigos, que el manual no repite en cada comando:

| HTTP | Significado |
|---|---|
| `401` | Token ausente, inválido o vencido — consulta [Autenticación](/getting-started/authentication) |
| `403` | Token válido, pero sin permiso (o sin el [nivel de acceso](/getting-started/authentication#inicio-de-sesión-desde-el-navegador-oauth--pkce) necesario) para la operación |
| `429` | Límite de solicitudes excedido |
| `500` | Error inesperado en el servidor |

En el CLI, una respuesta 4xx o 5xx termina con el [código de salida](/reference/exit-codes) `1`.
