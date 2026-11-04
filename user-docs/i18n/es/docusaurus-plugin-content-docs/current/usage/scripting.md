---
id: scripting
title: Scripts y agentes de IA
sidebar_label: Scripts y agentes de IA
sidebar_position: 8
description: Usa kobana en shell scripts, CI y como herramienta de agentes de IA.
---

# Scripts y agentes de IA

`kobana` fue diseñado para ser llamado por programas: salida estructurada, errores con código, introspección y simulación.

## En scripts y CI

```bash
#!/usr/bin/env bash
set -euo pipefail

export KOBANA_BILLING_TOKEN="$BILLING_TOKEN"

kobana billing invoices list --params '{"status": "open"}' --page-all --fields "id,total" \
  | jq -r '[.id, .total] | @tsv'
```

- El resultado va al **stdout**, el resto al **stderr**.
- El [código de salida](/reference/exit-codes) dice qué falló: `1` error de API, `2` autenticación, `3` validación, `4` schema, `5` interno.
- Usa `KOBANA_<PRODUCTO>_TOKEN` en lugar del inicio de sesión interactivo, y `--env sandbox` para probar.
- Fija `--idempotency-key` en las llamadas que pueden repetirse.
- Un `.env` en la carpeta actual **no** puede definir tokens ni credenciales: el CLI avisa y lo ignora. Consulta [Variables de entorno](/reference/environment-variables).

## Con agentes de IA

Un agente descubre qué hacer sin leer documentación extensa:

1. `kobana schema --list` — qué recursos existen.
2. `kobana schema billing.subscriptions.create` — qué pide el endpoint.
3. `kobana billing subscriptions create --json '…' --dry-run` — la solicitud, sin efecto.
4. El mismo comando sin `--dry-run`.

Prefiere `--fields` y `--page-all` para no llenar el contexto, y trata todo valor que venga de afuera como no confiable: el CLI valida los identificadores contra `../`, caracteres de control e inyección de URL, pero la decisión de enviar es tuya.

### Skills

El repositorio incluye [skills](https://github.com/universokobana/kobana-cli/tree/main/skills) listas por producto (`kobana-billing`, `kobana-finance`, `kobana-kadu`…) para los agentes que las soportan.

## Documentación para máquinas

- Cada página está disponible en Markdown agregando `.md` a la URL.
- [`/es/llms.txt`](https://cli.kobana.com.br/es/llms.txt) es el índice; [`/es/llms-full.txt`](https://cli.kobana.com.br/es/llms-full.txt) trae todo en una sola lectura.
- El [MCP de la Documentación](/docs-mcp) permite que el agente busque y lea páginas por su cuenta.
