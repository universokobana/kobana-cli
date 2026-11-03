---
name: kobana-shared
description: "Kobana CLI: Shared patterns for authentication, global flags, and output formatting."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
---

# kobana — Shared Reference

## Installation

```bash
brew tap universokobana/tap
brew install kobana
```

Or download from [Releases](https://github.com/universokobana/kobana-cli/releases/latest).

## Authentication

Each product has its own token, issued by the Conta Kobana with the product's
API as `aud`. A token never serves another product.

```bash
# Token per product (highest priority)
export KOBANA_BANKING_TOKEN=...
export KOBANA_FINANCE_TOKEN=...
export KOBANA_KADU_TOKEN=...      # or KADU_TOKEN

# Browser login (OAuth + PKCE): the scopes the product publishes, then the
# access levels (read, write, delete, import, export)
kobana billing auth login
kobana billing auth login --scopes "billing/customers,read,write"

# Client credentials — banking only
kobana banking auth login --client-id <ID> --client-secret <SECRET>

kobana billing auth status
```

## Global Flags

| Flag | Description |
|------|-------------|
| `--output-format <FORMAT>` | Output format: `json` (default), `table`, `csv` |
| `--dry-run` | Validate and show request without calling the API |
| `--fields <FIELDS>` | Limit response fields (protects AI context windows) |
| `--env <ENV>` | API environment: `production` (default), `sandbox`, or `development` |
| `--verbose` | Show request/response details on stderr |

## CLI Syntax

```bash
kobana <product> <resource> <method> [flags]
```

`<product>` is the Kobana product: `banking` (Gateway Bancário), `inbox`
(Inbox Autônomo), `billing` (Faturamento Automático), `finance` (Financeiro
Inteligente), `dunning` (Régua de Cobrança), `dashboards` (Dashboard Mágico) or
`kadu` (Agente Kadu).
Each is its own API with its own credentials — see that product's skill.

### Method Flags

| Flag | Description |
|------|-------------|
| `--params '{"key": "val"}'` | URL/query/header parameters (id, uid, page, filters, `X-Workspace-Id`) |
| `--json '{"key": "val"}'` | Request body for POST/PUT/PATCH |
| `--output <PATH>` | Save response to file |
| `--page-all` | Auto-paginate (NDJSON output); follows `meta.next_cursor` on routes that declare `cursor` |
| `--page-limit <N>` | Max pages when using --page-all (default: 10) |
| `--page-delay <MS>` | Delay between pages in ms (default: 100) |
| `--idempotency-key <KEY>` | Custom idempotency key for mutations |

## Security Rules

- **Never** output tokens or credentials directly
- **Always** use `--dry-run` before executing write/delete operations
- **Always** use `--fields` to limit response size and protect context windows
- Use `kobana schema <endpoint>` to inspect payloads before constructing them

## JSON Tips

Wrap `--params` and `--json` values in single quotes so the shell does not interpret the inner double quotes:

```bash
kobana banking charge pix list --params '{"per_page": 5}'
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `KOBANA_<PRODUCT>_TOKEN` | A product's Bearer token (highest priority), e.g. `KOBANA_BILLING_TOKEN`; Kadu also reads `KADU_TOKEN` |
| `KOBANA_CREDENTIALS_FILE` | Path to a credentials JSON file (the shape `auth export` prints), used only for the `product` (absent = `banking`) and `environment` it names |
| `KOBANA_CLIENT_ID` | OAuth client ID |
| `KOBANA_CLIENT_SECRET` | OAuth client secret |
| `KOBANA_WORKSPACE_ID` | Workspace sent as `X-Workspace-Id` by `finance` (required) and `dashboards` |
| `KOBANA_ENVIRONMENT` | `production` (default), `sandbox` or `development` |
| `KOBANA_CONFIG_DIR` | Override config directory (default: `~/.config/kobana`) |

## Community & Feedback

- For bugs or feature requests: https://github.com/universokobana/kobana-cli/issues
- Security vulnerabilities: https://www.kobana.com.br/white-hat
