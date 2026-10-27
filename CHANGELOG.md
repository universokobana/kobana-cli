# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2026-10-26

### Fixed

- **Coleções sob um recurso pai viravam `get`** — um `GET` com parâmetro de caminho era sempre `get`, então `GET /webhooks/{id}/deliveries` (a lista) e `GET /webhooks/{id}/deliveries/{delivery_id}` (uma entrega) disputavam o mesmo nome e um dos dois sumia. Agora um `GET` que não termina em parâmetro e aceita paginação (`page`, `per_page`, `cursor`, `limit`) é `list`; recursos singulares sob um pai (`companies fiscal-profile`, `balances latest`) continuam `get`
  - **BREAKING:** listagens que se chamavam `get` passam a `list` — `banking financial accounts balances`, `statement-transactions`, `statement-transactions imports` e `statement-transactions syncs`; `billing customers statement` e `customers users`; `dashboards entities records`, `entities tenant-syncs`, `portal-users activity timeline` e `webhooks deliveries`; `dunning people contact-points` e `webhook-endpoints deliveries`; `finance financial-accounts balances`
  - Voltaram comandos que estavam inacessíveis: `banking financial accounts balances get`, `statement-transactions imports get` e `statement-transactions syncs get` (agora leem **um** item); `billing customers users get` e `billing nfes requests list`; `dashboards data-sources logs list`, `entities logs list` e `entities records get`; `inbox webhooks deliveries list`
- **Exemplos de `payables aggregates`** no README e na skill do `finance` — o comando é `payables aggregates list`

### Changed

- Specs do `dunning` e do `dashboards` atualizadas: a descrição da API do dunning fala no access token da Conta Kobana, e os resumos ambíguos do dashboards foram corrigidos

## [0.5.0] - 2026-10-26

Seis produtos no mesmo binário: Gateway Bancário (`banking`), Inbox Autônomo (`inbox`), Faturamento Automático (`billing`), Financeiro Inteligente (`finance`), Régua de Cobrança (`dunning`) e Dashboard Mágico (`dashboards`).

### Added

- **Produto `dunning` (Régua de Cobrança)** — `kobana dunning <recurso> <metodo>`, com clientes, cobranças, réguas, templates, notificações, acordos, disputas, negativações, protestos, tarefas, importações/exportações e webhooks. Spec OpenAPI v1 embutida (67 paths)
- **Produto `dashboards` (Dashboard Mágico)** — `kobana dashboards <recurso> <metodo>`, com fontes de dados, entidades, workspaces, portal do cliente, builder de páginas, backups e webhooks. Spec OpenAPI v1 embutida (153 paths)
- Skills `kobana-dunning` e `kobana-dashboards`
- **Parâmetros de cabeçalho** — cabeçalhos declarados na spec (como `X-Workspace-Id`) são passados em `--params` e enviados como cabeçalho, fora da query string. `KOBANA_WORKSPACE_ID` preenche o `X-Workspace-Id` do `finance` (obrigatório) e do `dashboards`. O `--dry-run` passa a mostrar os cabeçalhos e `kobana schema` os lista em `header_params`
- **Produto `billing` (Faturamento Automático)** — `kobana billing <recurso> <metodo>`, com assinaturas, planos, produtos, faturas, NF-e, propostas, cupons, pagamentos e contas de faturamento. Spec OpenAPI v1 embutida (173 paths)
- **Produto `finance` (Financeiro Inteligente)** — `kobana finance <recurso> <metodo>`, com contas financeiras, lançamentos, contas a pagar e receber, fluxo de caixa, conciliação e classificação. Spec OpenAPI v1 embutida (45 paths)
- Skills `kobana-billing` e `kobana-finance`
- **Produto `inbox` (Inbox Autônomo)** — `kobana inbox <recurso> <metodo>`, com workspaces, inboxes, e-mails, agentes, agent runs, webhooks (incluindo entregas e replay) e system events. Spec OpenAPI v1 embutida
- **Suporte a mTLS** — `KobanaClient::with_client_cert` apresenta um certificado de cliente no handshake TLS. A API do Inbox fica atrás de mTLS terminado no edge; aponte `KOBANA_INBOX_CLIENT_CERT` para um PEM com a cadeia e a chave privada. Sem ele o CLI avisa em stderr e segue (o dry-run não avisa, já que não faz requisição)
- Skill `kobana-inbox` documentando os recursos e as credenciais próprias do produto

### Changed

- **Specs atualizadas para as versões atuais das APIs** — `banking` v1 (67 paths) e v2 (158; novos `admin certificates`, `admin commands`, `data people`, `financial companies`, `financial statement-categories`, sincronização e resumo de extrato), `billing` (190; `late-fee-policies`, `bank-accounts`, `pix-keys`, `holidays`, `banks`, `credit-purchases`, `usage credits`, `me`), `finance` (55; `custom-classifiers`, `fiscal-documents`, `financial-accounts balances`, `me`) e `inbox` (31; `workflow-runs`, `secrets`, `models`, `usage`, `agents versions`, `me`). Os produtos fora do `banking` agora autenticam com token RS256 emitido pela Conta Kobana (`account.kobana.com.br`)
- **BREAKING (`finance`):** toda rota exige o cabeçalho `X-Workspace-Id` — defina `KOBANA_WORKSPACE_ID`
- **BREAKING (`banking`):** parâmetros de path renomeados na v2 — `entry_id` → `entry_uid` em `mailbox entries`, `id` → `uid` em `financial accounts` e nos `commands` de Pix e contas. `entry_uid` ainda aceita `uid`/`id` pelo alias; nos demais, passe `uid` em vez de `id`
- **Removido (`billing`):** `proposal-templates set-default`, que saiu da API
- **BREAKING: comandos agora exigem o produto** — a sintaxe mudou de `kobana <servico> <recurso> <metodo>` para `kobana <produto> <servico> <recurso> <metodo>`. A API original da Kobana passou a ser o produto Gateway Bancário (`banking`), então `kobana charge pix list` vira `kobana banking charge pix list` e `kobana v1 bank-billets list` vira `kobana banking v1 bank-billets list`
- **BREAKING: a versão da API saiu do comando** — `kobana banking v1 bank-billets list` vira `kobana banking bank-billets list` e `kobana inbox v1 emails list` vira `kobana inbox emails list`. A versão é resolvida a partir da spec e continua na URL da requisição; os domínios do v2 (`charge`, `payment`, …) seguem inalterados, porque são domínios e não versões. As specs de um produto passam a ser mescladas em uma árvore só
- **BREAKING: `kobana schema <endpoint>`** exige o produto e não aceita mais a versão: `kobana schema banking.charge.pix.create`, `kobana schema banking.bank-billets.list`. Caminhos com versão recebem um erro explicando a mudança e sugerindo o caminho novo. A saída de `kobana schema --list` passa a ser agrupada por produto

### Fixed

- **Descrições de recursos vazando entre produtos** — o texto de ajuda era mapeado só pelo nome do recurso, então `kobana billing payments` exibia "Pagamentos (unificado)" e `kobana finance transfers` exibia "Transferências (unificado)", ambos do Gateway Bancário. Agora o mapa é chaveado por `(produto, recurso)`
- **Panic na inicialização em paths de ação com mais de um verbo HTTP** — `/v2/admin/connections/{uid}/associations` (POST+DELETE), os canais de `/v2/mailbox/entries/{entry_id}/*` e `/v1/inboxes/{id}/recipients` geravam o mesmo nome de subcomando duas vezes, e o clap aborta nesse caso. Isso quebrava `kobana completions <shell>` e o `--help` desses recursos. Agora a ação vira um nó de recurso e cada verbo recebe seu método (`inboxes recipients create` / `... delete`)
- **User-Agent** — as requisições se identificavam como `kobana-cli/0.1.0` em qualquer versão; agora enviam a versão real

### Internal

- **Registro de produtos orientado a dados** (`crates/kobana-cli/src/product.rs`) — hosts, specs embutidas e layout de serviços de cada produto passam a ser uma entrada em `REGISTRY`. Adicionar um produto agora é adicionar uma spec em `specs/` e uma entrada na tabela; nenhum outro módulo do CLI conhece produtos
- **URL base por produto** — `Environment::base_url()` (que fixava `api.kobana.com.br`) foi removido; o host vem do produto resolvido, já que cada produto tem API própria
- **Prefixo de versão configurável** (`crates/kobana/src/spec.rs`) — `build_command_tree` recebe o prefixo a remover em vez de ter `/v1/`/`/v2/` hardcoded, que era uma amarra do banking dentro da biblioteca
- **`kobana schema`** passou a percorrer a mesma árvore de comandos usada no dispatch, em vez de reimplementar casamento de paths e inferência de método — elimina a duplicação que podia divergir do CLI real. A saída ganhou os campos `product` e `path_params`
- Specs renomeadas para `specs/banking-v1.json` e `specs/banking-v2.json`

## [0.4.1] - 2026-04-11

### Changed

- **`kobana update` output** — defaults to a single human-readable line; when an update is available and stdin is a terminal, prompts `(Y/n)` before running the upgrade command inline (`brew upgrade`, `cargo install`, or the standalone self-update)
- **`--json` flag** added to `kobana update` to restore the machine-readable JSON output (never prompts, safe in pipes)

## [0.4.0] - 2026-04-11

### Added

- **`kobana update` command** — checks GitHub Releases for a newer version and prints status (current, latest, install method, install command). With `--check`, only prints; without, performs an atomic self-update for standalone Unix binaries or prints the correct upgrade command for Homebrew/Cargo installs
- **Daily auto-update check** — every normal command performs a best-effort (3s timeout) check at most once per day and prints a yellow stderr warning when a newer release exists. Respects TTY (no ANSI codes when piped/logged)

### Changed

- **BREAKING: environment flags** — removed `--sandbox`, `--production`, `--development`. Use a single `--env <production|sandbox|development>` flag. The default is now **production** (was sandbox). `KOBANA_ENVIRONMENT` continues to work and accepts the same values
- **BREAKING: default environment** — now `production` instead of `sandbox`

## [0.3.0] - 2026-04-11

### Added

- **Interactive OAuth scope selector** — TUI drill-down for `auth login` when `--scopes` is not provided: first pick top-level groups, then sub-scopes per group, then read/write permission level
- **Human-readable scope descriptions** — pt-BR descriptions embedded for all 58 Kobana OAuth scopes and for top-level groups
- **Friendly OAuth denial page** — when the user declines on the consent screen, the browser shows a styled error page with the decoded reason instead of a connection-closed error; the CLI returns a descriptive auth error
- **UTF-8 URL decoding** on the OAuth callback query string

### Fixed

- **Credential key persistence** — enable `keyring` backend features (`apple-native`, `windows-native`, `sync-secret-service`); without features, keyring v3 silently "succeeded" and the encryption key was lost between runs, breaking every follow-up command after `auth login`
- **`resolve_token` error visibility** — propagate credential store errors instead of masking them behind a generic "No authentication configured" message

### Changed

- **`auth login` output** — richer JSON payload matching gws-cli style (credentials_file, encryption backend, scopes list)

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
