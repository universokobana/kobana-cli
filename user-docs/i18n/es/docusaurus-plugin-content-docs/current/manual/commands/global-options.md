---
id: global-options
title: "Opciones globales"
sidebar_label: "Opciones globales"
sidebar_position: 1
---

# Opciones globales

Valen para todos los comandos, antes o después del nombre del recurso.

| Opciones | Descripción |
|---|---|
| `--params <JSON>` | Parámetros de URL y de consulta, en JSON |
| `--json <JSON>` | Cuerpo de la solicitud, en JSON |
| `--file <CAMPO=RUTA>` | Archivo enviado como parte de un formulario multipart, campo=ruta (repetible; los campos de --json se vuelven partes de texto) |
| `--fields <CAMPOS>` | Campos a incluir en la respuesta, separados por comas |
| `--dry-run` | Muestra la solicitud sin ejecutarla |
| `--env <ENTORNO>` | Entorno de la API: production, sandbox o development (cada producto tiene sus hosts). También puede venir de KOBANA_ENVIRONMENT. |
| `--verbose` | Muestra los detalles de la solicitud y la respuesta en stderr |
| `--output <RUTA>` | Guarda la respuesta en un archivo |
| `--output-format <FORMATO>` | Formato de salida: json (predeterminado), table, csv |
| `--idempotency-key <CLAVE>` | Clave de idempotencia propia para modificaciones |
| `--page-all` | Recorre todas las páginas e imprime NDJSON |
| `--page-limit <N>` | Máximo de páginas a obtener (predeterminado: 10) |
| `--page-delay <MS>` | Intervalo entre páginas en ms (predeterminado: 100) |
| `--jq <EXPRESIÓN>` | Filtra la salida JSON con una expresión jq, ej.: '.data[].id' (no necesita jq instalado) |
| `--no-color` | Desactiva los colores (también: NO_COLOR o CLICOLOR=0); vea `kobana accessibility` |
| `--lang <IDIOMA>` | Idioma de los mensajes: pt, en o es (predeterminado: KOBANA_LANG o el del sistema) |
| `-h, --help` | Muestra la ayuda |
| `-V, --version` | Muestra la versión |
