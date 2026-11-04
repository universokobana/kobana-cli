---
id: ai-setup
title: Configurar con una IA
sidebar_label: Configurar con una IA
sidebar_position: 4
description: Deja que un agente de IA instale el MCP de la documentación, el CLI y las skills, y guíe el inicio de sesión.
---

# Configurar con una IA

Deja que un agente de IA prepare el entorno: instalar el MCP de la documentación, el CLI y las skills, y guiar el inicio de sesión de cada producto. Funciona mejor con agentes que **ejecutan comandos en tu máquina**, porque instalar el CLI e iniciar sesión solo ocurre ahí.

## Agentes con terminal (recomendado)

Ejecuta el comando de tu agente. Lee esta página y hace los pasos, pidiendo confirmación antes de instalar:

```bash
claude "Sigue las instrucciones de https://cli.kobana.com.br/es/getting-started/ai-setup.md para preparar mi entorno del Kobana CLI."
codex "Sigue las instrucciones de https://cli.kobana.com.br/es/getting-started/ai-setup.md para preparar mi entorno del Kobana CLI."
gemini "Sigue las instrucciones de https://cli.kobana.com.br/es/getting-started/ai-setup.md para preparar mi entorno del Kobana CLI."
```

En **Cursor**, usa el botón "Abrir en Cursor" de la [página de inicio](/). En otros agentes (agente de Copilot en VS Code, Windsurf, Cline…), pega el mismo mensaje: "Sigue las instrucciones de esta dirección".

## Chats en la web

ChatGPT, Claude y Gemini en la web **no ejecutan comandos en tu máquina**: solo explican. Úsalos para entender el CLI y resolver dudas — y, si quieres, conecta el [MCP de la Documentación](/docs-mcp) como conector para que respondan con base en estas páginas. La instalación en sí, ejecútala tú con los comandos que te den.

## Instrucciones para el agente

Estás ayudando a una persona a dejar el entorno listo para usar el Kobana CLI (el comando `kobana`, que accede a las APIs de todos los productos Kobana). Sigue los pasos en orden. Antes de cada comando que instale o cambie algo, explica qué hace y pide confirmación. Si no puedes ejecutar comandos, pásaselos a la persona para que los ejecute.

1. **MCP de la documentación.** Conecta a tu cliente el servidor MCP público y de solo lectura `https://cli.kobana.com.br/mcp`, para consultar la documentación. Instrucciones por cliente en [MCP de la Documentación](/docs-mcp). Si dudas: `npx add-mcp "https://cli.kobana.com.br/mcp"`.
2. **CLI.** En macOS y Linux: `curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash` (o `brew tap universokobana/tap && brew install kobana`). En Windows, usa el binario de la [página de Releases](https://github.com/universokobana/kobana-cli/releases/latest). Verifica con `kobana --version`.
3. **Skills de agente.** Pregunta qué productos usa la persona (`banking`, `finance`, `billing`, `inbox`, `dunning`, `dashboards`, `kadu`) e instala la skill `kobana-shared` y las de esos productos (en banking, empieza por `kobana-banking`). Están en [`skills/`](https://github.com/universokobana/kobana-cli/tree/main/skills); copia las carpetas a la carpeta de skills de este cliente. Mira [Skills para agentes](/guides/agent-skills).
4. **Autenticación.** Guía a la persona en el inicio de sesión de cada producto con `kobana <producto> auth login` (cada producto tiene su credencial). Nunca pidas tokens ni contraseñas en el chat. Mira [Autenticación](/getting-started/authentication).
5. **Prueba.** Para cada producto en el que la persona inició sesión, ejecuta `kobana <producto> me` (muestra a quién pertenece el token, sus scopes y su vencimiento, sin exponer el token) y di si funcionó. Si falla, usa [Solución de problemas](/guides/troubleshooting).

De aquí en adelante: usa siempre `--dry-run` antes de cualquier comando que cree, cambie o elimine, y `--env sandbox` para probar. La documentación en texto está en [`/es/llms.txt`](https://cli.kobana.com.br/es/llms.txt).
