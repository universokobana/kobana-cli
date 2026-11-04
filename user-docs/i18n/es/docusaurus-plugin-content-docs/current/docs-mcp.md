---
id: docs-mcp
title: MCP de la Documentación
sidebar_label: MCP de la Documentación
sidebar_position: 3
slug: /docs-mcp
---

# MCP de la Documentación

Esta documentación tiene **su propio servidor MCP**. Con él, un asistente de IA
(Claude, ChatGPT, Cursor, VS Code, Claude Code) busca y lee las páginas de este
sitio sin que tengas que copiar y pegar nada. Así, las respuestas sobre
los comandos y flags de `kobana` salen del contenido publicado, en el idioma que elijas.

```text
https://cli.kobana.com.br/mcp
```


## Cómo funciona

- **Público y de solo lectura:** no pide inicio de sesión ni token. Expone el
  mismo contenido que cualquier persona ve en el sitio.
- **Siempre actualizado:** lee la versión publicada de la documentación. Una
  página nueva o modificada aparece en el MCP en cuanto se publica el sitio.
- **Tres idiomas:** portugués (`pt`), inglés (`en`) y español (`es`). Si no se
  indica idioma, usa el portugués.
- **Transporte:** Streamable HTTP, sin sesión. Funciona con cualquier cliente
  MCP que se conecte a un servidor remoto por URL.

## Tools

| Tool | Qué hace | Parámetros |
|---|---|---|
| `search_docs` | Busca páginas por palabras clave en el título, la ruta y el resumen, sin distinguir acentos ni mayúsculas. Devuelve las mejores páginas con ruta y resumen | `query` (obligatorio), `locale`, `limit` (1 a 25, por defecto 8) |
| `get_page` | Devuelve el contenido completo de una página en Markdown | `path`: la ruta que devolvió `search_docs` o `list_pages` (por ejemplo, `/es/getting-started/installation`) o la URL completa |
| `list_pages` | Lista todas las páginas, con ruta y resumen | `locale` |

El asistente decide solo cuándo llamar a cada tool. Normalmente busca con
`search_docs` y lee con `get_page` las páginas que le interesan.

## Cómo instalarlo

La forma más rápida es el menú **Copiar página**, arriba de cada página de este
sitio. En **MCP de la Documentación** encontrarás:

- **Conectar a Cursor** y **Conectar a VS Code:** instalan el servidor con un
  clic.
- **Copiar config del MCP:** copia el bloque JSON para clientes configurados
  por archivo.
- **Copiar comando del MCP:** copia el comando de instalación para la terminal.

Si prefieres configurarlo a mano, los pasos de cada cliente están abajo.

### Cualquier cliente, desde la terminal

[`add-mcp`](https://www.npmjs.com/package/add-mcp) detecta los clientes
instalados en tu equipo y les agrega el servidor:

```bash
npx add-mcp "https://cli.kobana.com.br/mcp"
```

### Claude Code

```bash
claude mcp add --transport http kobana-cli-docs https://cli.kobana.com.br/mcp
```

### Cursor

Usa **Conectar a Cursor** en el menú de la página, o agrega esto a
`~/.cursor/mcp.json` (para todos los proyectos) o al `.cursor/mcp.json` del
proyecto:

```json
{
  "mcpServers": {
    "kobana-cli-docs": {
      "url": "https://cli.kobana.com.br/mcp"
    }
  }
}
```

### VS Code

Usa **Conectar a VS Code** en el menú de la página, o crea `.vscode/mcp.json` en
el proyecto. En VS Code la clave es `servers`, no `mcpServers`:

```json
{
  "servers": {
    "kobana-cli-docs": {
      "type": "http",
      "url": "https://cli.kobana.com.br/mcp"
    }
  }
}
```

### Claude y ChatGPT

En **Configuración › Conectores**, agrega un conector personalizado y pega la
URL `https://cli.kobana.com.br/mcp`. No hay inicio de sesión.

### Otros clientes configurados por archivo

```json
{
  "mcpServers": {
    "kobana-cli-docs": {
      "type": "http",
      "url": "https://cli.kobana.com.br/mcp"
    }
  }
}
```

## Ejemplos de preguntas

Con el servidor conectado, pregunta con normalidad. El asistente consulta la
documentación antes de responder:

- "¿Cómo instalo kobana y me autentico en Facturación Automática?"
- "¿Cómo obtengo todas las páginas de un listado y lo guardo como CSV?"
- "¿Cuáles son los códigos de salida y cómo los uso en un script?"
- "Arma un comando que cree un cobro Pix con `--dry-run`."

## Probar sin cliente

```bash
curl -s https://cli.kobana.com.br/mcp \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"search_docs","arguments":{"query":"webhook","locale":"es"}}}'
```

## Límites

- Los parámetros de cada endpoint están en el [Manual](/manual); para la API en sí, usa la documentación de cada producto (`docs.<producto>.kobana.com.br`).
- La búsqueda es por palabras clave, no semántica. Si no encuentras nada, prueba
  otros términos o pide al asistente que use `list_pages`.
- También puedes leer cualquier página en Markdown agregando `.md` a su URL, o
  descargar toda la documentación en
  [`llms-full.txt`](https://cli.kobana.com.br/es/llms-full.txt).
