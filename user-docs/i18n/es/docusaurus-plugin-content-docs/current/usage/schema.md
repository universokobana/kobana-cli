---
id: schema
title: Schema
sidebar_label: Schema
sidebar_position: 5
description: Consulta parámetros, cuerpo y respuestas de cualquier endpoint sin salir de la terminal.
---

# Schema

`kobana schema` muestra, en JSON, todo lo que dice la especificación sobre un endpoint: método y ruta HTTP, parámetros de ruta, de consulta y de encabezado, cuerpo y respuestas.

```bash
# Todos los productos y recursos
kobana schema --list

# Un endpoint — la ruta tiene la forma del comando
kobana schema banking.charge.pix.create
kobana schema finance.payables.list
kobana schema kadu.tasks.run
```

La ruta es `<producto>.<recurso>.<método>`. En los comandos que se ejecutan por su nombre, el método se puede omitir: `kobana schema banking.me`.

## Qué devuelve

| Campo | Contenido |
|---|---|
| `method`, `path` | Verbo y ruta HTTP, con la versión de la API |
| `summary`, `description` | Texto de la especificación, en el idioma del CLI |
| `path_params` | Identificadores de la ruta, en el orden de los argumentos |
| `parameters` | Parámetros de consulta, con tipo, obligatoriedad y valores aceptados |
| `header_params` | Encabezados, como `X-Workspace-Id` |
| `request_body` | Esquema del cuerpo |
| `responses` | Códigos de respuesta y sus esquemas |
| `deprecated` | `true` cuando el endpoint está en desuso |

## Para agentes

El schema es la forma más corta de que un agente descubra cómo llamar a un endpoint sin leer toda la documentación. Consulta [Scripts y agentes de IA](/usage/scripting). La misma información, ya formateada, está en el [Manual](/manual).
