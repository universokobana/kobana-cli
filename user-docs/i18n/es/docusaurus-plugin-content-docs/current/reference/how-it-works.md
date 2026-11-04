---
id: how-it-works
title: Cómo funciona
sidebar_label: Cómo funciona
sidebar_position: 4
description: Cómo kobana arma los comandos a partir de las especificaciones OpenAPI incluidas en el binario.
---

# Cómo funciona

`kobana` **no tiene una lista de endpoints escrita a mano**. Cada producto Kobana publica una especificación OpenAPI 3.1, y el binario carga esas especificaciones y arma el árbol de comandos a partir de ellas en cada ejecución.

## Dos fases

1. **Descubrir el producto.** El CLI lee el primer argumento (`billing`, `finance`…) y carga la especificación de ese producto, ya incluida en el binario.
2. **Armar y ejecutar.** De ella sale el árbol de comandos: cada ruta se vuelve un recurso, cada operación se vuelve un comando. Solo entonces se leen los argumentos restantes, se resuelve la credencial del producto, y se arma y envía la solicitud.

El resultado es que `--help` en cualquier nivel, el [Manual](/manual) y [`kobana schema`](/usage/schema) responden desde la misma fuente: la especificación.

## Qué significa esto para ti

- **Ningún comando queda fuera de la API.** Cuando un producto publica un endpoint nuevo, la próxima versión del binario ya lo incluye — sin escribir código por endpoint.
- **Sin red para descubrir comandos.** La especificación está en el binario: `--help`, `schema` y `--dry-run` funcionan sin conexión.
- **La versión de la API no aparece en el comando.** `kobana banking bank-billets list` va a `/v1/bank_billets` y `kobana banking charge pix list` va a `/v2/charge/pix`.
- **Los nombres siguen la forma en que hablas.** Un recurso que solo lee se ejecuta por su nombre (`kobana banking me`), y los ids de la ruta son argumentos (`bank-billets get 123`). Consulta [Sintaxis](/usage/syntax).
- **Los nombres antiguos no rompen scripts.** Cuando cambia una regla de nombres, el nombre anterior sigue valiendo como alias.
- **Todo el texto viene traducido.** Las descripciones de la especificación se reemplazan por la traducción del idioma elegido ([Idiomas](/usage/languages)).

## Los helpers quedan aparte

Los pocos comandos escritos a mano son los [helpers](/usage/helpers) (`+emitir`, `+cobrar`…). Empiezan con `+` precisamente para no chocar nunca con un comando generado.

## Código abierto

El CLI es de código abierto (licencia MIT) en [github.com/universokobana/kobana-cli](https://github.com/universokobana/kobana-cli), y está hecho de dos partes: una biblioteca (cliente HTTP, errores, lectura de las especificaciones, validación de entradas) y el binario (comandos, autenticación, salida, paginación, helpers).
