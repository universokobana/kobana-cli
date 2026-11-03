# AGENTS.md

## Project Overview

`kobana` is a Rust CLI tool for interacting with the APIs of all seven Kobana products — Gateway Bancário (`banking`), Financeiro Inteligente (`finance`), Faturamento Automático (`billing`), Inbox Autônomo (`inbox`), Régua de Cobrança (`dunning`), Dashboard Mágico (`dashboards`) and Agente Kadu (`kadu`). Each is an independent API with its own hosts and credentials. The CLI dynamically generates its command surface at startup by parsing OpenAPI 3.1 specs embedded in the binary.

> [!IMPORTANT]
> **Dynamic Commands**: This project does NOT hardcode API endpoints as Rust structs. Instead, it embeds OpenAPI JSON specs and builds `clap` commands dynamically via two-phase parsing. When updating the API surface, replace the spec files in `crates/kobana-cli/specs/` (named `<product>-<version>.json`) and rebuild. Do NOT add new crates or modules per endpoint.

## Build & Test

```bash
cargo build          # Build in dev mode
cargo clippy -- -D warnings  # Lint check
cargo test           # Run tests
```

## CI/CD & Releases

CI runs on every push and PR to `main` via GitHub Actions (`.github/workflows/ci.yml`):

1. **Tests** — `cargo test --all` + `cargo clippy -- -D warnings` on Ubuntu
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

1. Load `product::REGISTRY` — parse each product's embedded specs and build their command trees
2. Build the dynamic `clap::Command` tree from those products, then parse argv against it

> [!IMPORTANT]
> clap panics at startup if a node exposes the same subcommand name twice, which
> a malformed tree can cause from spec data alone. `no_node_exposes_a_duplicate_subcommand_name`
> in `product.rs` guards every registered product against this — keep it passing.

### Workspace Layout

The repository is a Cargo workspace with two crates:

| Crate                          | Package            | Purpose                                      |
| ------------------------------ | ------------------ | -------------------------------------------- |
| `crates/kobana/`               | `kobana`           | Library — HTTP client, error types, spec parsing, validation |
| `crates/kobana-cli/`           | `kobana-cli`       | Binary crate — the `kobana` CLI              |

### Adding a Product

The CLI syntax is `kobana <product> <resource> <method>`. Products are
data, not code: everything product-specific lives in `REGISTRY` in
`crates/kobana-cli/src/product.rs`. Nothing else in the CLI is product-aware.

To add one:

1. Save the product's OpenAPI spec as JSON in `crates/kobana-cli/specs/<product>-v1.json`
2. Add a `Product` entry to `REGISTRY` with its hosts and a `SpecEntry`

Every Kobana product carries the version prefix in its own spec paths, so
`SpecEntry::version_prefix` is just the prefix to strip for tree placement —
the request URL keeps it:

| Product | `Hosts::production` | Spec paths | `version_prefix` |
|---------|---------------------|------------|------------------|
| `banking` | `https://api.kobana.com.br` | `/v1/bank_billets`, `/v2/charge/pix` | `/v1`, `/v2` |
| `inbox` | `https://api.inbox.kobana.com.br` | `/v1/workspaces` | `/v1` |
| `billing` | `https://api.billing.kobana.com.br` | `/v1/subscriptions` | `/v1` |
| `finance` | `https://api.finance.kobana.com.br` | `/v1/financial-accounts` | `/v1` |
| `dunning` | `https://api.dunning.kobana.com.br` | `/v1/charges` | `/v1` |
| `dashboards` | `https://api.dashboards.kobana.com.br` | `/v1/data_sources` | `/v1` |
| `kadu` | `https://api.kadu.ia.br` | `/v1/tasks` | `/v1` |

All seven are registered.

> [!IMPORTANT]
> **The published finance spec does not match that table yet.** It still ships
> `servers: https://api.finance.kobana.com.br/v1` with paths starting at the
> resource (`/financial-accounts`). That is a known bug being fixed upstream —
> the `/v1` moves into the paths, like every other product.
>
> `specs/finance-v1.json` is therefore normalized when converted: the `/v1` is
> stripped from `servers` and prepended to every path. Do not add a per-product
> switch in the code for this. When the upstream spec is corrected, re-convert
> it **without** the normalization step — the result is byte-identical in shape,
> and `every_endpoint_path_exists_in_its_spec` keeps the URLs honest either way.

`Hosts` must never include the version prefix.

A product's specs are merged into one command tree, so an API version is never
a CLI segment: `/v1/bank_billets` and `/v2/charge/pix` read as
`banking bank-billets` and `banking charge pix`. Adding a spec to a product
therefore adds its resources next to the existing ones — check the merged
`--help` for name clashes, which `merge()` resolves first-spec-wins.

Sandbox hosts follow `api.<product>.sandbox.kobana.com.br`, except banking,
which predates the convention (`api-sandbox.kobana.com.br`). Kadu lives on its
own domain (`api.kadu.ia.br`) and has staging, not a sandbox, so its `sandbox`
is `None`; `development` is its local dev server (`localhost:3030/api`).

`Product::client_cert_env` names the env var holding a PEM client certificate,
for products whose edge terminates mTLS. No product sets it today: `inbox` did
until its API dropped mTLS on 2026-10-31 (`KOBANA_INBOX_CLIENT_CERT` is no
longer read). The mechanism stays for a product that needs it again.
`product::client_for()` loads it and is the only place clients are built —
never call `KobanaClient::new` directly from a command.

`Product::header_env` fills spec-declared header parameters from env vars when
`--params` does not carry them. `finance` (where `X-Workspace-Id` is required on
every route), `dashboards` and `dunning` (optional while the organization has a
single workspace) map it to `KOBANA_WORKSPACE_ID`.
`header_env_names_a_header_parameter_of_the_spec` keeps the mapping honest.

#### Command names (`spec.rs`)

Each path segment is a resource (`_` → `-`), path parameters go in `--params`,
and the method name is inferred:

- `GET` → `list`, or `get` when it reads one record: a path ending in `{id}`,
  or a top-level route without pagination whose 2xx response is
  `{ data: <object> }` (the envelope may be a local `$ref`). A path with a GET
  item route right under it is that item's collection and stays `list`, so the
  `get` by id is not hidden: a top-level singleton (`/data_purges` and
  `/data_purges/{id}`), a collection under a parent (`/pix/{pix_uid}/commands`
  and `…/commands/{uid}`, also when it first reads as an action) or one keyed
  by a parameter (`/imports/{collection_name}` and `…/{id}`). Singletons keep
  `list` as a hidden alias.
- `POST` → `create`, `PUT`/`PATCH` → `update`, `DELETE` → `delete`.
- **Actions.** A path with a single operation whose last segment follows `{id}`
  (`/pix/{uid}/cancel`), or a POST-only path under another route
  (`/holidays/sync` next to `/holidays`), is a command of the parent resource
  named by that segment: `pix cancel`, `holidays sync`. The second kind was
  `holidays sync create` up to 0.8.0 and keeps answering to it as a hidden
  subcommand (`ResolvedEndpoint::legacy_create`; `kobana schema` accepts the
  old path too).
- A name that is also a child resource (`data-sources openapi` has
  `openapi analyze`) moves into that child and is inferred again there. Two
  paths meeting on one name are deduplicated only after that move.

- **Deprecated operations** (`deprecated: true` in the spec) stay runnable but
  leave the help (`.hide(true)`); `kobana schema` reports `deprecated`. That is
  how an API retires a path: the replacement shows, the old line still works.

Renaming a command breaks scripts, so a rule change keeps the old name as an
alias, and is checked by diffing the whole command tree of every product
before and after (a script walking `--help`).

#### Updating specs

Embed the **Portuguese** spec — its text is the source that the English and
Spanish translations in `specs/i18n/` are keyed by (see [Languages](#languages)). Sources, taken from each repo's `main` (not a working tree, which may
hold someone's uncommitted edits):

| Spec | Source |
|------|--------|
| `banking-v1.json`, `banking-v2.json` | `kobana-api-specs`: `swagger/v{1,2}_0/kobana-api-v{1,2}_0-openapi-3_1.json` (already JSON) |
| `billing`, `inbox`, `dunning`, `dashboards` | `<app>/user-docs/openapi/openapi-v1-pt.yaml` |
| `finance` | `kobana-finance/user-docs/static/openapi/openapi-v1-pt.yaml` (then normalize, see above) |
| `kadu` | `kobana-agent-kadu/user-docs/openapi/openapi-v1.yaml` (Portuguese; the translations are `openapi-v1-{en,es}.yaml`) |

The same specs are published at `docs.<product>.kobana.com.br/pt/api/overview/openapi.md`.
YAML must be converted to JSON before being embedded. macOS ships Ruby, which
needs no extra dependency — but the system Ruby (2.6) predates
`YAML.unsafe_load_file`, where `YAML.load_file` is already unsafe:

```bash
ruby -ryaml -rjson -e "y = YAML.respond_to?(:unsafe_load_file) ? YAML.unsafe_load_file('in.yaml') : YAML.load_file('in.yaml'); File.write('out.json', JSON.pretty_generate(y))"
```

After replacing specs, run `cargo test` — the tree, path and duplicate-name
guards catch most spec shapes the CLI cannot serve.

`resource_about()` is keyed by `(product, resource)`, not by resource name
alone: `payments` exists in banking and billing, `transfers` in banking and
finance, and they mean different things.

**After replacing a spec, rebuild its translations** — see
[Languages](#languages):

```bash
ruby scripts/build-spec-translations.rb
```

#### Library (`crates/kobana/src/`)

| File           | Purpose                                                    |
| -------------- | ---------------------------------------------------------- |
| `client.rs`    | HTTP client with Bearer auth and idempotency keys          |
| `error.rs`     | `KobanaError` enum, structured exit codes, JSON serialization |
| `i18n.rs`      | Language detection, `tr!`/`trf!`, spec text translation    |
| `spec.rs`      | OpenAPI spec parsing, command tree builder, method inference, `SpecLayout` |
| `validate.rs`  | Path/URL/identifier validators against injection attacks   |

#### CLI (`crates/kobana-cli/src/`)

| File                 | Purpose                                                        |
| -------------------- | -------------------------------------------------------------- |
| `main.rs`            | Entrypoint, two-phase CLI parsing, dispatch                    |
| `product.rs`         | Product registry — hosts, embedded specs, CLI service layout   |
| `commands.rs`        | Recursive `clap::Command` builder from OpenAPI spec            |
| `clap_i18n.rs`       | Localized clap help layout, `-h`/`-V`/`help`, parse errors, `--lang` |
| `executor.rs`        | HTTP request construction, response handling, dry-run           |
| `auth.rs`            | Token resolution chain (env var → file → saved credentials)    |
| `auth_commands.rs`   | `kobana <product> auth` subcommands: `login`, `logout`, `status`, `export` |
| `credential_store.rs`| AES-256-GCM encryption/decryption of credential files          |
| `oauth.rs`           | OAuth2 Authorization Code and Client Credentials flows         |
| `schema.rs`          | `kobana schema` command — introspect API endpoint schemas      |
| `formatter.rs`       | Output formatting: JSON (default), table, CSV                  |
| `pagination.rs`      | Auto-pagination with NDJSON streaming output — `cursor`/`meta.next_cursor` when the endpoint declares a `cursor` query parameter, `page`/`per_page` otherwise |
| `validate.rs`        | CLI-specific input validation for `--params` and `--json`      |
| `config.rs`          | Config directory, `.env` loading, environment management       |
| `logging.rs`         | Structured logging (stderr + JSON file rotation) via `tracing` |
| `completions.rs`     | Shell completion generation (bash, zsh, fish, powershell, elvish) |
| `update.rs`          | `kobana update` (self-update of standalone installs, checked against the published SHA-256) and the daily new-version notice |
| `helpers/mod.rs`     | Helper trait, registry, and dispatch                           |
| `helpers/boleto.rs`  | `+emitir` and `+cancelar-lote` helpers                         |
| `helpers/pix.rs`     | `+cobrar` helper                                               |

## Languages

Everything the CLI says to a person is in Portuguese, English or Spanish:
help, progress lines, prompts, error messages, the OAuth result page. The
language is picked once per run by `kobana::i18n::lang()` — `--lang` (read from
the raw arguments before clap, since commands are built with the language
already set), then `KOBANA_LANG`, `LC_ALL`, `LC_MESSAGES`, `LANG`, then the OS
(`sys-locale`; a bare `C`/`POSIX` locale falls through to it). `pt*` → pt,
`es*` → es, anything else → en.

**Every user-facing string is written in all three languages, in place:**

```rust
eprintln!("{}", tr!(pt: "Pronto", en: "Done", es: "Listo"));
KobanaError::Auth(trf!(pt: "falhou: {e}", en: "failed: {e}", es: "falló: {e}"))
```

`tr!` gives a `&'static str`, `trf!` a `String` via `format!` (inline captures
work). Fixed tables use `[pt, en, es]` arrays read with `i18n::pick` — product
`about`, `resource_about`, OAuth scope descriptions. Never translated: command
and flag names (including the Portuguese helpers like `+emitir --valor`), JSON
keys, exit codes, invariants that cannot fail at runtime (spec parse errors).

**clap is English-only**, so `clap_i18n` takes over its strings: every command
is built with `clap_i18n::command(name, usage)`, which sets a localized help
template, headings and the full usage line (the path is known while building,
so `[OPÇÕES]`/`<COMANDO>` are written out instead of clap's `[OPTIONS]`).
`clap_i18n::root` swaps clap's `-h`, `-V` and `help` subcommand for localized
ones (global settings, they reach every subcommand). Possible values and
defaults are hidden from clap and stated in the arg's own help. Parse errors go
through `clap_i18n::exit_with`, which rewrites the common kinds and falls back
to clap's text for the rest.

**Spec text** comes in Portuguese. `specs/i18n/<spec>.{en,es}.json` maps each
Portuguese `summary`/`description` to its translation, and
`SpecEntry::document` swaps them in at load time; text without a translation
stays in Portuguese. The maps are built by `scripts/build-spec-translations.rb`
from each product's per-language spec files — the ones its user docs are
generated from: `kobana-api-specs/swagger/v{1,2}_0/*.{en,es}.json` for banking
(kept current by that repo's translate-specs workflow), `user-docs/openapi/*-{en,es}.yaml`
for the others (`kobana-agent-kadu` ships `openapi-v1-{en,es}.yaml` with an unsuffixed Portuguese file; read from the repo's `main`) — walking both specs side by side — paths are matched
ignoring a `/vN` prefix and path parameter names. Finance's descriptions are in
English even in its Portuguese spec; only its summaries get translated.
`translated_specs_serve_the_same_commands` checks that translating never
changes the command tree and that most summaries do change.

The client sends `Accept-Language`, so APIs that localize their messages
answer in the same language.

## Install Script

`scripts/install.sh` is the `curl -fsSL .../scripts/install.sh | bash` installer.
It is served straight from `main` (raw.githubusercontent.com), so a change ships
on push — no release needed. It downloads the release asset for the platform into
`~/.kobana/bin`, checks it against the SHA-256 GitHub publishes as each asset's
`digest`, and links `~/.local/bin/kobana` to it. Output is in pt, en or es,
picked from `KOBANA_LANG`, then `LC_ALL`, `LC_MESSAGES`, `LANG` (a bare `C`/`POSIX`
falls back to macOS's `AppleLocale`; anything else is English). Every string
lives in `load_messages` as an `M_*` printf format — add a new one to all three
languages. The ASCII banner is in the brand lime (`#D3FD54`, 24-bit color; off
when stdout is not a TTY or `NO_COLOR` is set).

`kobana update` checks the same `digest` before replacing the binary and refuses
an asset whose SHA-256 is missing or does not match.

> [!IMPORTANT]
> `kobana update` on a standalone install replaces `std::env::current_exe()`
> **after `canonicalize()`**. Without it, running through the `~/.local/bin`
> symlink would replace the link with the new binary and leave
> `~/.kobana/bin/kobana` behind.

Keep it POSIX-tool-only (curl/wget, shasum/sha256sum, awk, sed): no jq or
python on the target machine. Run `shellcheck scripts/install.sh` after editing.

## Demo Videos

Demo recordings are generated with [VHS](https://github.com/charmbracelet/vhs) (`.tape` files).

> [!IMPORTANT]
> **Do not use VHS v0.12.0 — it silently produces nothing.** It starts ttyd and
> the browser, prints `Creating docs/demo.gif...`, captures zero frames, writes
> no file and still **exits 0**. Verified across the host install and the
> official Docker image, on the canonical `vhs new` example tape, so it is not a
> tape or environment problem. `brew install charmbracelet/tap/vhs` currently
> installs exactly this version. Use the pinned Docker image below instead.
>
> The official image also ships **without ffmpeg**, which is a separate defect —
> it does not matter for v0.10.0, which bundles a working encoder path.

Render with the pinned image. The tape runs `kobana` inside the container, so it
needs a **Linux** binary — build one in Docker rather than reusing `target/release`,
which is a macOS binary:

```bash
# 1. Build a Linux binary (kept out of the repo's target/)
docker run --rm -v "$PWD":/w -w /w -v /tmp/kobana-linux:/target \
  -e CARGO_TARGET_DIR=/target rust:1-slim-bookworm cargo build --release

# 2. Render the demo
docker run --rm --shm-size=1g -v "$PWD":/vhs \
  -v /tmp/kobana-linux/release/kobana:/usr/local/bin/kobana:ro \
  ghcr.io/charmbracelet/vhs:v0.10.0 docs/demo.tape
```

Re-test the plain `vhs docs/demo.tape` path when a version above v0.12.0 ships;
if it works, drop the Docker indirection.

### VHS quoting rules

- Use **double quotes** for simple strings: `Type "kobana --help" Enter`
- Use **backtick quotes** when the typed text contains JSON with double quotes:
  ```
  Type `kobana banking charge pix list --params '{"per_page": 5}'` Enter
  ```
  `\"` escapes inside double-quoted `Type` strings are **not supported** by VHS and will cause parse errors.
- **Keep `Type` text ASCII-only.** VHS drops non-ASCII characters from typed
  input: `"binário"` renders as `bin rio` and `"é"` disappears entirely. Write
  narration comments without diacritics (`"# Primeiro segmento do comando: o produto"`).
  This affects only `Type`; accented characters in **command output** render fine.
- Check the rendered width. `kobana --help` reaches ~173 columns, so the tape
  sets `Set Width 1500`; narrower values wrap the `--env` line.

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

Type `kobana banking charge pix create --dry-run --json '{"amount": 99.90, "pix_account_uid": "UID"}'` Enter
Sleep 3s

Type "kobana schema banking.charge.pix.create" Enter
Sleep 3s
```

## Input Validation & URL Safety

> [!IMPORTANT]
> This CLI is designed for use by AI/LLM agents. Always assume inputs can be adversarial — validate identifiers against path traversal (`../../`), reject control characters, reject URL injection (`?`, `#`), and reject double-encoding (`%`).

> [!NOTE]
> **Environment variables are trusted inputs.** The validation rules above apply to **CLI arguments** that may be passed by untrusted AI agents. Environment variables (e.g. `KOBANA_CONFIG_DIR`) are set by the user themselves and are not subject to these validations.

### Identifier Validation (`crates/kobana/src/validate.rs`)

All user-supplied values embedded in URL path segments are validated with `validate_identifier()`. A value must be exactly one path segment:

```rust
// Rejects: empty, `.`, `..`, / and \, control chars, ?, #, %
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

Helpers are commands prefixed with `+` that provide multi-step workflows the dynamic commands cannot: simplified interfaces, batch operations, or multi-API composition. Each helper belongs to one product (`Helper::product()`) and lives under it: `kobana <product> +verb`, never at the top level.

> [!IMPORTANT]
> **Do NOT add a helper that** wraps a single API call already available via the dynamic commands, adds flags to expose data already in the API response, or re-implements `--params`/`--json` parameters as custom flags. Helper flags must control orchestration logic.

Helpers send through the executor's pieces (`executor::Output`, `dry_run_report`,
`send`, `idempotency_key`), so the global options behave as in a dynamic
command: same `--dry-run` report, `--fields`/`--output-format`/`--output`/`--verbose`,
`--idempotency-key`. A global option a helper has no use for is refused with
`helpers::reject_options`, never silently dropped.

Current helpers:

| Helper | Description |
|--------|-------------|
| `kobana banking +emitir` | Simplified bank billet creation with named flags |
| `kobana banking +cancelar-lote` | Batch cancel multiple billets by ID |
| `kobana banking +cobrar` | Simplified Pix charge creation with named flags |

## Environment Variables

### Authentication

| Variable | Description |
|----------|-------------|
| `KOBANA_<PRODUCT>_TOKEN` | A product's Bearer token (highest priority): `KOBANA_BANKING_TOKEN`, `KOBANA_FINANCE_TOKEN`… Kadu also reads `KADU_TOKEN` (`Product::token_env_aliases`) |
| `KOBANA_CREDENTIALS_FILE` | Path to a credentials JSON file (the shape `auth export` prints). `product` is required; the file is used only for that product and for its `environment` (absent = any) |
| `KOBANA_CLIENT_ID` | OAuth client ID (for `kobana <product> auth login`, default `kobana-cli`) |
| `KOBANA_CLIENT_SECRET` | OAuth client secret. Like `--client-secret`, it selects the Client Credentials flow (banking only, with a client ID); the PKCE login sends no secret |
| `KOBANA_ACCOUNT_URL` | Overrides the Conta Kobana (authorization server) URL |
| `KOBANA_WORKSPACE_ID` | Workspace sent as `X-Workspace-Id` by `finance` (required), `dashboards` and `dunning` (optional while the organization has a single workspace). A value in `--params` wins |

> [!NOTE]
> Products do not share credentials. Every token is issued by the Conta Kobana
> (`account.kobana.com.br` in production, `account.<env>.kobana.com.br`
> otherwise, the local kobana-app in development) with the product's API URL as
> `aud`, so one token serves one product. `kobana <product> auth login` reads the scopes it publishes in
> `/.well-known/oauth-protected-resource` on its MCP host (falling back to the
> authorization server's catalogue filtered by `<product>/`, which is what Kadu
> needs today), adds the global access levels (`read`, `write`, `delete`,
> `import`, `export`) and sends `resource=<Product::oauth.*.api>`. Each
> `Product` carries its registry entry in `oauth` — the `api` URI must match
> `kobana-app/config/oauth_resources.yml` byte for byte, and in development it
> is not the CLI's base URL. Credentials are saved per product and environment
> (`credentials-<product>-<env>.enc`) and refreshed when expired; the legacy
> `credentials.enc` still counts as banking.

### Configuration

| Variable | Description |
|----------|-------------|
| `KOBANA_CONFIG_DIR` | Override the config directory (default: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `production` (default), `sandbox` or `development` |
| `KOBANA_LANG` | `pt`, `en` or `es` (default: `LC_ALL`/`LC_MESSAGES`/`LANG`, then the system language). `--lang` wins over it |

### Logging

| Variable | Description |
|----------|-------------|
| `KOBANA_LOG` | Log level filter for stderr (e.g., `kobana=debug`). Off by default. |
| `KOBANA_LOG_FILE` | Directory for JSON log files with daily rotation. Off by default. |

All variables can also live in a `.env` file (loaded via `dotenvy`): the
current directory's — never its parents' — and the config dir's; a variable
already set wins. The current directory's `.env` may come from a repository
someone else wrote, so it may not set `KOBANA_ACCOUNT_URL` or
`KOBANA_CONFIG_DIR` (`PROJECT_DOTENV_BLOCKED` in `config.rs`; the CLI warns and
ignores them). Saved tokens are refreshed only at the server that issued them
(`StoredCredentials::issuer`), never at the current `KOBANA_ACCOUNT_URL`.

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | API error (4xx/5xx) |
| `2` | Authentication error |
| `3` | Validation error, including a command line that does not parse (unknown command or flag, missing argument; the message goes to stderr) |
| `4` | Schema error |
| `5` | Internal error |
