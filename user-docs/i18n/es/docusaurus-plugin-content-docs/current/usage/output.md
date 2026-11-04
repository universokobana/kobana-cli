---
id: output
title: Salida
sidebar_label: Salida
sidebar_position: 2
description: JSON por defecto, tabla y CSV cuando lo necesites, selección de campos y escritura en archivo.
---

# Salida

La respuesta va al **stdout**; los mensajes, avisos y el `--verbose` van al **stderr**. Así, `kobana … | jq` y `kobana … > archivo` funcionan sin ruido.

## Formato

```bash
kobana banking bank-billets list                         # JSON (por defecto)
kobana banking bank-billets list --output-format table   # tabla
kobana banking bank-billets list --output-format csv     # CSV
```

## Solo los campos que importan

`--fields` limita la respuesta a una lista separada por comas. Es la forma más barata de leer datos, y de no llenar la ventana de contexto de un agente:

```bash
kobana banking bank-billets list --fields "id,amount,status,due_at"
```

## Filtrar con `--jq` {#jq}

`--jq` aplica una expresión [jq](https://jqlang.github.io/jq/manual/) a la salida JSON, como el `--jq` de `gh` — sin tener `jq` instalado. Cada resultado sale en una línea: un texto sale tal cual (como `jq -r`), el resto en JSON, con sangría solo en el terminal.

```bash
kobana banking bank-billets list --jq '.[].id'
kobana billing invoices list --page-all --jq 'select(.status == "open") | [.id, .total] | @tsv'
kobana banking auth status --jq .authenticated
```

- Vale para toda salida JSON: respuestas de la API (incluso cada elemento de `--page-all`), `--dry-run`, `kobana schema`, `auth status` y `kobana update` (que, con `--jq`, solo informa).
- Se aplica después de `--fields`, y no combina con `--output-format table` o `csv`.
- La expresión se comprueba antes de cualquier solicitud: un error de sintaxis sale con el código `3` sin enviar nada.
- Si la expresión falla en la respuesta de una escritura (`create`, `update`, `delete`, helpers), el CLI avisa en el stderr e imprime la respuesta sin el filtro, con código `0`: la operación se realizó, y repetirla no sería seguro.
- En el terminal, los caracteres de control de un texto se vuelven `�`, para que un texto de la API no mande secuencias de escape a la pantalla.
- `env` y `$ENV` no existen en `--jq`, para que una expresión — escrita por un agente, por ejemplo — no lea los tokens del entorno; tampoco los módulos (`import`, `include`) ni `input`.

## Guardar en un archivo

```bash
kobana banking bank-billets get 12345 --output boleto.json
```

`--output` rechaza rutas con `..`, enlaces simbólicos, carpetas y dispositivos.

## Ver la conversación con la API

`--verbose` imprime en el stderr la solicitud y la respuesta, sin el token:

```bash
kobana billing plans list --verbose
```

Para un registro estructurado, usa `KOBANA_LOG=kobana=debug` (stderr) o `KOBANA_LOG_FILE=./logs` (archivos JSON con rotación diaria). Consulta [Variables de entorno](/reference/environment-variables).
