---
id: scripting
title: Scripts and AI agents
sidebar_label: Scripts and AI agents
sidebar_position: 8
description: Use kobana in shell scripts, CI and as a tool for AI agents.
---

# Scripts and AI agents

`kobana` was designed to be called by programs: structured output, errors with codes, introspection and simulation.

## In scripts and CI

```bash
#!/usr/bin/env bash
set -euo pipefail

export KOBANA_BILLING_TOKEN="$BILLING_TOKEN"

kobana billing invoices list --params '{"status": "open"}' --page-all --fields "id,total" \
  | jq -r '[.id, .total] | @tsv'
```

- The result goes to **stdout**, everything else to **stderr**.
- The [exit code](/reference/exit-codes) says what went wrong: `1` API error, `2` authentication, `3` validation, `4` schema, `5` internal.
- Use `KOBANA_<PRODUCT>_TOKEN` instead of interactive login, and `--env sandbox` to test.
- Fix `--idempotency-key` on calls that may be retried.
- Without a terminal, the CLI never asks anything nor shows the new-version notice; `--jq` filters the output with no `jq` installed. See [Accessibility](/usage/accessibility#scripts-and-ai-agents).
- A `.env` in the current folder **cannot** define tokens or credentials: the CLI warns and ignores them. See [Environment variables](/reference/environment-variables).

## With AI agents

An agent finds out what to do without reading long documentation:

1. `kobana schema --list` — which resources exist.
2. `kobana schema billing.subscriptions.create` — what the endpoint asks for.
3. `kobana billing subscriptions create --json '…' --dry-run` — the request, with no effect.
4. The same command without `--dry-run`.

Prefer `--fields` and `--page-all` to avoid filling the context, and treat every value from outside as untrusted: the CLI validates identifiers against `../`, control characters and URL injection, but the decision to send is yours.

### Skills

The repository ships ready-made [skills](https://github.com/universokobana/kobana-cli/tree/main/skills) per product (`kobana-billing`, `kobana-finance`, `kobana-kadu`…) for agents that support them.

## Documentation for machines

- Every page is available as Markdown by appending `.md` to the URL.
- [`/en/llms.txt`](https://cli.kobana.com.br/en/llms.txt) is the index; [`/en/llms-full.txt`](https://cli.kobana.com.br/en/llms-full.txt) has everything in a single read.
- The [Documentation MCP](/docs-mcp) lets the agent search and read pages on its own.
