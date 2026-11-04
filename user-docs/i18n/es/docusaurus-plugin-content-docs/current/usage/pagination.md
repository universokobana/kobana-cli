---
id: pagination
title: Paginación
sidebar_label: Paginación
sidebar_position: 3
description: Recorre todas las páginas de un listado con --page-all y recibe NDJSON.
---

# Paginación

Un listado devuelve una página por llamada. Con `--page-all` el CLI recorre las páginas e imprime **un registro por línea**, en [NDJSON](https://github.com/ndjson/ndjson-spec):

```bash
kobana billing invoices list --page-all --fields "id,status,total"
```

Cada línea es un objeto JSON completo, así que la salida se puede procesar mientras llega:

```bash
kobana kadu executions list --page-all | jq -r 'select(.status == "failed") | .id'
```

## Cómo pagina

- Si el endpoint declara un parámetro `cursor`, el CLI sigue `meta.next_cursor`.
- En caso contrario, usa `page` y `per_page`.

## Límites

| Flag | Valor por defecto | Para qué |
|---|---|---|
| `--page-limit <N>` | `10` | Máximo de páginas consultadas |
| `--page-delay <MS>` | `100` | Intervalo entre páginas, en milisegundos |

Aumenta el `--page-limit` a propósito: existe para que un listado enorme no se convierta en miles de llamadas sin que te des cuenta.

```bash
kobana banking charge pix list --page-all --page-limit 50 --page-delay 250
```
