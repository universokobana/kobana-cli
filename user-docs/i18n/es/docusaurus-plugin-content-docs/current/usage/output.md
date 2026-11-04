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
