---
id: scripting
title: Scripts e agentes de IA
sidebar_label: Scripts e agentes de IA
sidebar_position: 8
description: Use o kobana em shell scripts, CI e como ferramenta de agentes de IA.
---

# Scripts e agentes de IA

O `kobana` foi desenhado para ser chamado por programas: saída estruturada, erros com código, introspecção e simulação.

## Em scripts e CI

```bash
#!/usr/bin/env bash
set -euo pipefail

export KOBANA_BILLING_TOKEN="$BILLING_TOKEN"

kobana billing invoices list --params '{"status": "open"}' --page-all --fields "id,total" \
  | jq -r '[.id, .total] | @tsv'
```

- O resultado vai para o **stdout**, o resto para o **stderr**.
- O [código de saída](/reference/exit-codes) diz o que deu errado: `1` erro de API, `2` autenticação, `3` validação, `4` schema, `5` interno.
- Use `KOBANA_<PRODUTO>_TOKEN` em vez de login interativo, e `--env sandbox` para testar.
- Fixe `--idempotency-key` em chamadas que podem ser repetidas.
- Sem terminal, o CLI nunca pergunta nem mostra o aviso de versão nova; `--jq` filtra a saída sem precisar do `jq` instalado. Veja [Acessibilidade](/usage/accessibility#scripts-e-agentes-de-ia).
- Um `.env` na pasta atual **não** pode definir tokens nem credenciais: o CLI avisa e ignora. Veja [Variáveis de ambiente](/reference/environment-variables).

## Com agentes de IA

Um agente descobre o que fazer sem ler documentação longa:

1. `kobana schema --list` — que recursos existem.
2. `kobana schema billing.subscriptions.create` — o que o endpoint pede.
3. `kobana billing subscriptions create --json '…' --dry-run` — a requisição, sem efeito.
4. O mesmo comando sem `--dry-run`.

Prefira `--fields` e `--page-all` para não encher o contexto, e trate todo valor vindo de fora como não confiável: o CLI valida identificadores contra `../`, caracteres de controle e injeção de URL, mas a decisão de enviar é sua.

### Skills

O repositório traz [skills](https://github.com/universokobana/kobana-cli/tree/main/skills) prontas por produto (`kobana-billing`, `kobana-finance`, `kobana-kadu`…) para agentes que as suportam.

## Documentação para máquinas

- Cada página está em Markdown acrescentando `.md` à URL.
- [`/pt/llms.txt`](https://cli.kobana.com.br/pt/llms.txt) é o índice; [`/pt/llms-full.txt`](https://cli.kobana.com.br/pt/llms-full.txt) traz tudo numa leitura só.
- O [MCP da Documentação](/docs-mcp) deixa o agente buscar e ler páginas por conta própria.
