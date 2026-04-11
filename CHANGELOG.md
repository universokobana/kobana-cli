# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2026-04-11

### Added

- **`--development` environment flag** — connects to `http://localhost:5005` (OAuth + API)
- **Full OAuth scopes list** — all 58 Kobana scopes requested by default
- **Verbose OAuth token exchange logging** — `--verbose` now shows request/response details for debugging
- **Custom `User-Agent`** on OAuth token requests (`kobana-cli/<version>`) to bypass CloudFront WAF

## [0.2.1] - 2026-04-10

### Added

- **Nix flake** — `nix run github:universokobana/kobana-cli` for installation and dev shell

### Fixed

- **OAuth token endpoint** — Use `app.kobana.com.br/oauth/token` instead of `api.kobana.com.br` (404)
- **OAuth redirect URI** — Use `127.0.0.1` instead of `localhost` to allow wildcard ports
- **OAuth default scope** — Request `read` only by default (was sending an invalid list of 57 resource scopes)

## [0.2.0] - 2026-04-09

### Added

- **PKCE authentication** — `kobana auth login` works zero-config with embedded public client_id `kobana-cli`
- **OAuth scopes** — All 57 Kobana scopes supported, customizable via `--scopes`
- **Agent skills** — SKILL.md files for all API modules (charge, payment, transfer, financial, admin, mailbox, data, security, v1)
- **Homebrew distribution** — `brew tap universokobana/tap && brew install kobana`
- **Project documentation** — README, AGENTS.md, CONTEXT.md, SECURITY.md, CHANGELOG, LICENSE, CLAUDE.md
- **Demo video** — VHS-generated GIF showcasing CLI features
- **CI/CD** — GitHub Actions with cross-platform builds (Linux/macOS/Windows) and automated releases

### Fixed

- **OAuth authorize URL** — Separate URLs per environment (sandbox: `app-sandbox.kobana.com.br`, production: `app.kobana.com.br`)
- **Clippy warnings** — Resolved `derivable_impls`, `manual_split_once`, `too_many_arguments`
- **GitHub Actions** — Upgraded to v5 to fix Node.js 20 deprecation warnings

## [0.1.0] - 2026-04-08

### Added

- **Core CLI** — Dynamic command tree generated from embedded OpenAPI specs (v1 + v2)
- **Two-phase parsing** — Identifies service from argv, builds clap commands from spec, re-parses
- **HTTP client** — reqwest-based with Bearer auth, idempotency keys, structured error handling
- **Authentication** — Token via `KOBANA_TOKEN`, OAuth2 Authorization Code and Client Credentials flows
- **Credential store** — AES-256-GCM encrypted credentials with OS keyring (macOS Keychain, etc.) and file fallback
- **Auth commands** — `kobana auth login`, `logout`, `status`, `export`
- **Schema introspection** — `kobana schema --list` and `kobana schema <endpoint>` for runtime API discovery
- **Output formats** — JSON (default), table, CSV via `--output-format`
- **Field masks** — `--fields` to limit response fields and protect AI agent context windows
- **Dry-run** — `--dry-run` for all mutations, shows request without executing
- **Auto-pagination** — `--page-all` with NDJSON streaming, `--page-limit`, `--page-delay`
- **Input validation** — Path traversal, URL injection, control character, and double-encoding detection
- **Structured exit codes** — 0 (success), 1 (API), 2 (auth), 3 (validation), 4 (schema), 5 (internal)
- **Structured logging** — stderr via `KOBANA_LOG`, JSON file rotation via `KOBANA_LOG_FILE`
- **Environment support** — `--sandbox` / `--production` flags, `KOBANA_ENVIRONMENT` env var
- **Config** — `.env` file loading, `KOBANA_CONFIG_DIR` for custom config directory
- **Helpers** — `+emitir` (boleto), `+cobrar` (Pix), `+cancelar-lote` (batch cancel)
- **Shell completions** — bash, zsh, fish, powershell, elvish via `kobana completions <shell>`
- **API coverage** — Full v1 (boletos, clientes, webhooks) and v2 (charge, payment, transfer, financial, admin, mailbox, data, EDI, security)
