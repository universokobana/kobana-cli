# AGENTS.md

## Project Overview

`kobana` is a Rust CLI tool for interacting with the Kobana financial API (v1 and v2). It dynamically generates its command surface at startup by parsing OpenAPI 3.1 specs embedded in the binary.

> [!IMPORTANT]
> **Dynamic Commands**: This project does NOT hardcode API endpoints as Rust structs. Instead, it embeds OpenAPI JSON specs and builds `clap` commands dynamically via two-phase parsing. When updating the API surface, replace the spec files in `crates/kobana-cli/specs/` and rebuild. Do NOT add new crates or modules per endpoint.

## Build & Test

```bash
cargo build          # Build in dev mode
cargo clippy -- -D warnings  # Lint check
cargo test           # Run tests
```

## CI/CD & Releases

CI runs on every push and PR to `main` via GitHub Actions (`.github/workflows/ci.yml`):

1. **Test** — `cargo test --all` + `cargo clippy -- -D warnings` on Ubuntu
2. **Build** — cross-platform matrix: Linux (amd64/arm64), macOS (amd64/arm64), Windows (amd64)
3. **Release** — triggered when a commit on `main` starts with `release:`. Creates a GitHub Release with binaries for all 5 targets.

### How to create a release

1. Update version in `crates/kobana-cli/Cargo.toml` and `crates/kobana/Cargo.toml`
2. Update `CHANGELOG.md` with the new version section
3. Commit with the `release:` prefix:

```bash
git add -A
git commit -m "release: v0.2.0"
git push
```

> [!IMPORTANT]
> The release job keys off the commit message prefix `release:`. Without this prefix, CI will build and test but will **not** create a GitHub Release. Do not use this prefix for non-release commits.

## Architecture

The CLI uses a **two-phase argument parsing** strategy:

1. Parse argv to extract the service name (e.g., `charge`, `v1`)
2. Load the embedded OpenAPI spec, build a dynamic `clap::Command` tree, then re-parse

### Workspace Layout

The repository is a Cargo workspace with two crates:

| Crate                          | Package            | Purpose                                      |
| ------------------------------ | ------------------ | -------------------------------------------- |
| `crates/kobana/`               | `kobana`           | Library — HTTP client, error types, spec parsing, validation |
| `crates/kobana-cli/`           | `kobana-cli`       | Binary crate — the `kobana` CLI              |

#### Library (`crates/kobana/src/`)

| File           | Purpose                                                    |
| -------------- | ---------------------------------------------------------- |
| `client.rs`    | HTTP client with Bearer auth and idempotency keys          |
| `error.rs`     | `KobanaError` enum, structured exit codes, JSON serialization |
| `spec.rs`      | OpenAPI spec parsing, command tree builder, method inference |
| `validate.rs`  | Path/URL/identifier validators against injection attacks   |

#### CLI (`crates/kobana-cli/src/`)

| File                 | Purpose                                                        |
| -------------------- | -------------------------------------------------------------- |
| `main.rs`            | Entrypoint, two-phase CLI parsing, dispatch                    |
| `commands.rs`        | Recursive `clap::Command` builder from OpenAPI spec            |
| `executor.rs`        | HTTP request construction, response handling, dry-run           |
| `auth.rs`            | Token resolution chain (env var → file → saved credentials)    |
| `auth_commands.rs`   | `kobana auth` subcommands: `login`, `logout`, `status`, `export` |
| `credential_store.rs`| AES-256-GCM encryption/decryption of credential files          |
| `oauth.rs`           | OAuth2 Authorization Code and Client Credentials flows         |
| `schema.rs`          | `kobana schema` command — introspect API endpoint schemas      |
| `formatter.rs`       | Output formatting: JSON (default), table, CSV                  |
| `pagination.rs`      | Auto-pagination with NDJSON streaming output                   |
| `validate.rs`        | CLI-specific input validation for `--params` and `--json`      |
| `config.rs`          | Config directory, `.env` loading, environment management       |
| `logging.rs`         | Structured logging (stderr + JSON file rotation) via `tracing` |
| `completions.rs`     | Shell completion generation (bash, zsh, fish, powershell, elvish) |
| `helpers/mod.rs`     | Helper trait, registry, and dispatch                           |
| `helpers/boleto.rs`  | `+emitir` and `+cancelar-lote` helpers                         |
| `helpers/pix.rs`     | `+cobrar` helper                                               |

## Demo Videos

Demo recordings are generated with [VHS](https://github.com/charmbracelet/vhs) (`.tape` files).

```bash
# Install VHS (macOS)
brew install charmbracelet/tap/vhs

# Record a demo
vhs docs/demo.tape
```

### VHS quoting rules

- Use **double quotes** for simple strings: `Type "kobana --help" Enter`
- Use **backtick quotes** when the typed text contains JSON with double quotes:
  ```
  Type `kobana charge pix list --params '{"per_page": 5}'` Enter
  ```
  `\"` escapes inside double-quoted `Type` strings are **not supported** by VHS and will cause parse errors.

### Creating a new demo

1. Create a `.tape` file in `docs/` (e.g., `docs/demo-pix.tape`)
2. Use `Set Shell "bash"` and `Set FontSize 14` for consistency
3. Keep demos short (under 30 seconds) and focused on one feature
4. Run `vhs docs/<file>.tape` to generate the `.gif`

Example `.tape` file:

```tape
Output docs/demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 600

Type "kobana --help" Enter
Sleep 3s

Type `kobana charge pix create --dry-run --json '{"amount": 99.90, "pix_account_uid": "UID"}'` Enter
Sleep 3s

Type "kobana schema charge.pix.create" Enter
Sleep 3s
```

## Input Validation & URL Safety

> [!IMPORTANT]
> This CLI is designed for use by AI/LLM agents. Always assume inputs can be adversarial — validate identifiers against path traversal (`../../`), reject control characters, reject URL injection (`?`, `#`), and reject double-encoding (`%`).

> [!NOTE]
> **Environment variables are trusted inputs.** The validation rules above apply to **CLI arguments** that may be passed by untrusted AI agents. Environment variables (e.g. `KOBANA_CONFIG_DIR`) are set by the user themselves and are not subject to these validations.

### Identifier Validation (`crates/kobana/src/validate.rs`)

All user-supplied values embedded in URL path segments are validated with `validate_identifier()`:

```rust
// Rejects: ../, control chars, ?, #, %
kobana::validate::validate_identifier(&value, "uid")?;
```

### Query Parameters

Query parameters are handled by reqwest's `.query()` builder, which encodes values automatically. User-supplied `--params` JSON is parsed and passed as key-value pairs.

### Checklist for New Features

When adding a new feature:

1. **URL path segments** → Validate with `validate_identifier()`
2. **Query parameters** → Use reqwest `.query()` builder (via `--params`)
3. **Request bodies** → Validate structure with `validate::validate_body()`
4. **File paths** → Validate for path traversal before writing
5. **Write tests** for both the happy path AND the rejection path

## Helper Commands (`+verb`)

Helpers are commands prefixed with `+` that provide multi-step workflows the dynamic commands cannot: simplified interfaces, batch operations, or multi-API composition.

> [!IMPORTANT]
> **Do NOT add a helper that** wraps a single API call already available via the dynamic commands, adds flags to expose data already in the API response, or re-implements `--params`/`--json` parameters as custom flags. Helper flags must control orchestration logic.

Current helpers:

| Helper | Description |
|--------|-------------|
| `+emitir` | Simplified bank billet creation with named flags |
| `+cancelar-lote` | Batch cancel multiple billets by ID |
| `+cobrar` | Simplified Pix charge creation with named flags |

## Environment Variables

### Authentication

| Variable | Description |
|----------|-------------|
| `KOBANA_TOKEN` | Bearer access token (highest priority) |
| `KOBANA_CREDENTIALS_FILE` | Path to OAuth credentials JSON file |
| `KOBANA_CLIENT_ID` | OAuth client ID (for `kobana auth login`) |
| `KOBANA_CLIENT_SECRET` | OAuth client secret |

### Configuration

| Variable | Description |
|----------|-------------|
| `KOBANA_CONFIG_DIR` | Override the config directory (default: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `sandbox` (default) or `production` |

### Logging

| Variable | Description |
|----------|-------------|
| `KOBANA_LOG` | Log level filter for stderr (e.g., `kobana=debug`). Off by default. |
| `KOBANA_LOG_FILE` | Directory for JSON log files with daily rotation. Off by default. |

All variables can also live in a `.env` file (loaded via `dotenvy`).

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | API error (4xx/5xx) |
| `2` | Authentication error |
| `3` | Validation error |
| `4` | Schema error |
| `5` | Internal error |
