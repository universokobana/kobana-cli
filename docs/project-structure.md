# Kobana CLI — Estrutura do Projeto

Linguagem: **Rust**
Referencia de arquitetura: **gws-cli**

---

## Diferenca Fundamental vs gws-cli

O gws-cli gera comandos **dinamicamente** a partir de Discovery Documents em runtime. O Kobana CLI gera comandos **a partir de um OpenAPI spec embutido no binario** em compile-time ou startup. Isso porque:

- A Kobana nao tem Discovery Service — tem um OpenAPI spec fixo
- O spec esta versionado no repo (`kobana-api-specs/`)
- Atualizar o CLI = atualizar o spec embutido + rebuild

---

## Workspace Layout

```
kobana-cli/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── kobana/                   # [LIBRARY] Tipos core, HTTP client, validacao
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── client.rs         # HTTP client com retry e rate limiting
│   │       ├── error.rs          # KobanaError enum, exit codes, JSON serialization
│   │       ├── spec.rs           # OpenAPI spec parsing, tipos derivados de serde
│   │       ├── services.rs       # Registry de servicos v1/v2 e seus recursos
│   │       └── validate.rs       # Path safety, URL encoding, input validation
│   │
│   └── kobana-cli/               # [BINARY] O CLI `kobana`
│       ├── Cargo.toml
│       ├── specs/                # OpenAPI specs embutidos (include_str! ou embed)
│       │   ├── v1.json
│       │   └── v2.json
│       └── src/
│           ├── main.rs           # Entrypoint, two-phase parsing, dispatch
│           ├── commands.rs       # Builder de clap::Command a partir do OpenAPI spec
│           ├── executor.rs       # Construcao de HTTP request, response handling
│           ├── schema.rs         # `kobana schema` — introspecao de endpoints
│           ├── auth.rs           # Token resolution (env var -> file -> OAuth)
│           ├── auth_commands.rs  # `kobana auth login/logout/status/export`
│           ├── credential_store.rs # AES-256-GCM encrypt/decrypt de credenciais
│           ├── oauth.rs          # OAuth2 flows (authorization code, client credentials)
│           ├── formatter.rs      # Output: JSON (default), table, CSV
│           ├── pagination.rs     # --page-all, NDJSON streaming
│           ├── validate.rs       # CLI-specific input validation
│           ├── logging.rs        # Structured logging (tracing) para stderr
│           ├── config.rs         # Config dir, .env loading, environment management
│           └── helpers/          # Helper commands (+verb) para workflows multi-step
│               ├── mod.rs        # Helper trait + registry
│               ├── boleto.rs     # +emitir, +cancelar-lote (atalhos v1)
│               └── pix.rs        # +cobrar (atalho cobranca pix)
│
├── kobana-api-specs/             # [GIT SUBMODULE] OpenAPI specs da Kobana
├── gws-cli/                      # [GIT SUBMODULE] Referencia de arquitetura
├── docs/                         # Documentacao do projeto
└── scripts/                      # Build, release, CI
```

---

## Dependencias Principais

### Library (`kobana`)

| Crate | Versao | Uso |
|-------|--------|-----|
| `serde` | 1 | Serialization (derive) |
| `serde_json` | 1 | JSON parsing |
| `reqwest` | 0.12 | HTTP client (rustls-tls, json) |
| `thiserror` | 2 | Error types |
| `anyhow` | 1 | Error handling |
| `tokio` | 1 | Async runtime (time, fs) |
| `tracing` | 0.1 | Observability |
| `percent-encoding` | 2 | URL encoding |

### CLI (`kobana-cli`)

| Crate | Versao | Uso |
|-------|--------|-----|
| `kobana` | path | Library crate |
| `clap` | 4 | CLI parsing (derive, string) |
| `reqwest` | 0.12 | HTTP client (stream, json, rustls-tls) |
| `tokio` | 1 | Async runtime (full) |
| `serde` / `serde_json` | 1 | JSON |
| `aes-gcm` | 0.10 | Credential encryption |
| `sha2` | 0.10 | Key derivation |
| `keyring` | 3 | OS keyring (platform-specific) |
| `zeroize` | 1 | Secure memory clearing |
| `dirs` | 5 | Home directory |
| `dotenvy` | 0.15 | .env file loading |
| `base64` | 0.22 | Base64 encoding |
| `uuid` | 1 | Idempotency keys |
| `tracing` | 0.1 | Structured logging |
| `tracing-subscriber` | 0.3 | Log output (stderr + file) |
| `chrono` | 0.4 | Date/time |

**Nao necessarios (vs gws-cli):**
- `yup-oauth2` — gws-cli usa para Google OAuth. Kobana usa OAuth2 padrao, implementado com `reqwest` direto
- `ratatui` / `crossterm` — TUI interativa. Pode ser adicionado depois se necessario
- `mail-builder` — Especifico de Gmail
- `chrono-tz` / `iana-time-zone` — Especifico de Calendar

---

## Fluxo de Parsing (Two-Phase)

Adaptado do gws-cli para funcionar com OpenAPI spec estatico:

### Fase 1: Identificar Servico

```
argv: ["kobana", "charge", "pix", "list", "--params", '{"page":1}']
                    ^
                    servico = "charge"
```

1. Ler `argv[1]` para identificar servico/versao
2. Comandos especiais interceptados antes: `auth`, `schema`, `--help`, `--version`

### Fase 2: Build Dinamico + Parse

1. Carregar OpenAPI spec (v1 ou v2, inferido do servico)
2. Construir arvore `clap::Command` a partir dos paths do spec
3. Re-parsear argumentos restantes com clap
4. Resolver metodo HTTP + URL a partir do match
5. Autenticar, construir request, executar

### Mapeamento Path -> Comando

```
/v2/charge/pix              -> kobana charge pix list/create
/v2/charge/pix/{uid}        -> kobana charge pix get/update
/v2/charge/pix/{uid}/cancel -> kobana charge pix cancel
```

Regras de inferencia de metodo:
- `GET` sem `{id}` no path -> `list`
- `GET` com `{id}` no path -> `get`
- `POST` -> `create`
- `PUT`/`PATCH` -> `update`
- `DELETE` -> `delete`
- Paths com acao no final (`/cancel`, `/approve`) -> metodo nomeado

---

## Autenticacao

### Precedencia

```
1. KOBANA_TOKEN (env var)           -> Bearer token direto
2. KOBANA_CREDENTIALS_FILE (env var) -> Arquivo JSON com tokens OAuth
3. Client Credentials (env vars)     -> KOBANA_CLIENT_ID + KOBANA_CLIENT_SECRET
4. Credenciais salvas                -> ~/.config/kobana/credentials.enc
```

### OAuth2 Flows

**Authorization Code** (para acessar contas de terceiros):
```
1. CLI abre browser -> https://app.kobana.com.br/oauth/authorize?client_id=...&redirect_uri=...&response_type=code
2. Usuario autoriza
3. Callback em http://localhost:PORT com ?code=...
4. CLI troca code por access_token via POST /oauth/token
5. Credenciais criptografadas e salvas
```

**Client Credentials** (para apps OAuth):
```
1. POST /oauth/token com grant_type=client_credentials&client_id=...&client_secret=...
2. Recebe access_token
3. Criptografa e salva
```

### Storage

```
~/.config/kobana/
├── credentials.enc        # AES-256-GCM encrypted OAuth tokens
├── config.json            # environment, defaults
└── client_secret.json     # OAuth client config (opcional)
```

Chave de encriptacao no OS keyring (macOS Keychain, Windows Credential Manager, Linux Secret Service) com fallback para arquivo.

---

## Schema Introspection

```bash
kobana schema charge.pix.create
```

Output:
```json
{
  "method": "POST",
  "path": "/v2/charge/pix",
  "parameters": [...],
  "request_body": {
    "required": ["amount", "pix_account_uid"],
    "properties": {
      "amount": {"type": "number"},
      "pix_account_uid": {"type": "string", "format": "uuid"}
    }
  },
  "responses": {
    "201": {"description": "Cobranca criada"},
    "422": {"description": "Validacao falhou"}
  }
}
```

Alimentado pelo OpenAPI spec embutido. Sem chamadas de rede.

---

## Ordem de Implementacao

### Fase 1 — Core (MVP)
1. Workspace Cargo.toml + crates scaffold
2. `kobana` lib: error types, HTTP client, spec parsing, validation
3. `kobana-cli`: main.rs com two-phase parsing
4. `commands.rs`: build clap tree a partir do OpenAPI spec
5. `executor.rs`: executar requests, formatar response
6. `auth.rs`: Bearer token via env var (`KOBANA_TOKEN`)
7. `schema.rs`: introspecao de endpoints
8. Flags globais: `--params`, `--json`, `--fields`, `--dry-run`, `--sandbox/--production`
9. Exit codes estruturados

### Fase 2 — Auth Completa
10. `credential_store.rs`: AES-256-GCM encryption
11. `auth_commands.rs`: `kobana auth login/logout/status/export`
12. `oauth.rs`: Authorization Code + Client Credentials flows
13. Config dir management (`~/.config/kobana/`)

### Fase 3 — Produtividade
14. `pagination.rs`: `--page-all`, `--page-limit`, NDJSON
15. `formatter.rs`: output table, CSV
16. `logging.rs`: structured logging
17. `validate.rs`: input validation completa
18. `config.rs`: .env loading, environment management

### Fase 4 — Helpers & Polish
19. Helper trait + registry
20. Helpers basicos: `+emitir` (boleto), `+cobrar` (pix)
21. Shell completions (bash, zsh, fish, powershell)
22. Man pages
23. CI/CD + release binaries
