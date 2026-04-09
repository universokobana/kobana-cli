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

```bash
# Token direto (mais simples)
export KOBANA_TOKEN=seu_token_aqui

# OAuth — Client Credentials
kobana auth login --client-id <ID> --client-secret <SECRET>

# OAuth — Authorization Code (abre browser)
export KOBANA_CLIENT_ID=seu_client_id
export KOBANA_CLIENT_SECRET=seu_client_secret
kobana auth login
```

## Global Flags

| Flag | Description |
|------|-------------|
| `--output-format <FORMAT>` | Output format: `json` (default), `table`, `csv` |
| `--dry-run` | Validate and show request without calling the API |
| `--fields <FIELDS>` | Limit response fields (protects AI context windows) |
| `--sandbox` | Use sandbox environment (default) |
| `--production` | Use production environment |
| `--verbose` | Show request/response details on stderr |

## CLI Syntax

```bash
kobana <service> <resource> <method> [flags]
```

### Method Flags

| Flag | Description |
|------|-------------|
| `--params '{"key": "val"}'` | URL/query parameters (id, uid, page, filters) |
| `--json '{"key": "val"}'` | Request body for POST/PUT/PATCH |
| `--output <PATH>` | Save response to file |
| `--page-all` | Auto-paginate (NDJSON output) |
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
kobana charge pix list --params '{"per_page": 5}'
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `KOBANA_TOKEN` | Bearer access token (highest priority) |
| `KOBANA_CREDENTIALS_FILE` | Path to OAuth credentials JSON file |
| `KOBANA_CLIENT_ID` | OAuth client ID |
| `KOBANA_CLIENT_SECRET` | OAuth client secret |
| `KOBANA_ENVIRONMENT` | `sandbox` (default) or `production` |
| `KOBANA_CONFIG_DIR` | Override config directory (default: `~/.config/kobana`) |

## Community & Feedback

- For bugs or feature requests: https://github.com/universokobana/kobana-cli/issues
- Security vulnerabilities: https://www.kobana.com.br/white-hat
