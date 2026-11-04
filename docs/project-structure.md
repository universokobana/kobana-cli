# Kobana CLI — Estrutura do Projeto

Linguagem: **Rust**
Referencia de arquitetura: **gws-cli**

---

## Diferenca Fundamental vs gws-cli

O gws-cli gera comandos **dinamicamente** a partir de Discovery Documents em runtime. O Kobana CLI gera comandos **a partir de um OpenAPI spec embutido no binario** em compile-time ou startup. Isso porque:

- A Kobana nao tem Discovery Service — cada produto publica um OpenAPI spec
- Os specs ficam versionados neste repo, em `crates/kobana-cli/specs/` (copiados do repo de cada produto; ver AGENTS.md)
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
│   │       ├── client.rs         # HTTP client: Bearer, idempotencia, Accept-Language, multipart
│   │       ├── error.rs          # KobanaError enum, exit codes, JSON serialization
│   │       ├── i18n.rs           # Idioma (pt/en/es), tr!/trf!, traducao do texto dos specs
│   │       ├── spec.rs           # OpenAPI spec parsing, arvore de comandos, inferencia de metodos
│   │       └── validate.rs       # Path safety, URL encoding, input validation
│   │
│   └── kobana-cli/               # [BINARY] O CLI `kobana`
│       ├── Cargo.toml
│       ├── specs/                # OpenAPI specs embutidos (include_str!), em portugues
│       │   ├── <produto>-<versao>.json  # banking-v1, banking-v2, finance-v1, billing-v1,
│       │   │                            # inbox-v1, dunning-v1, dashboards-v1, kadu-v1
│       │   └── i18n/             # <spec>.en.json, <spec>.es.json: traducoes do texto
│       └── src/
│           ├── main.rs           # Entrypoint, two-phase parsing, dispatch
│           ├── product.rs        # Registro de produtos: hosts, specs, OAuth, variaveis
│           ├── commands.rs       # Builder de clap::Command a partir do OpenAPI spec
│           ├── clap_i18n.rs      # Ajuda, erros de parse e --lang no idioma do usuario
│           ├── executor.rs       # Construcao de HTTP request, response handling
│           ├── schema.rs         # `kobana schema` — introspecao de endpoints
│           ├── auth.rs           # Token resolution (env var -> file -> OAuth)
│           ├── auth_commands.rs  # `kobana <produto> auth login/logout/status/export`
│           ├── credential_store.rs # AES-256-GCM encrypt/decrypt de credenciais
│           ├── oauth.rs          # OAuth2 flows (authorization code, client credentials)
│           ├── formatter.rs      # Output: JSON (default), table, CSV
│           ├── pagination.rs     # --page-all, NDJSON streaming
│           ├── validate.rs       # CLI-specific input validation
│           ├── logging.rs        # Structured logging (tracing) para stderr
│           ├── config.rs         # Config dir, .env loading, environment management
│           ├── completions.rs    # `kobana completions <shell>`
│           ├── update.rs         # `kobana update` e o aviso de versao nova
│           └── helpers/          # Helper commands (+verb) para workflows multi-step
│               ├── mod.rs        # Helper trait + registry
│               ├── boleto.rs     # +emitir, +cancelar-lote (atalhos v1)
│               └── pix.rs        # +cobrar (atalho cobranca pix)
│
├── docs/                         # Documentacao do projeto
├── skills/                       # Skills para agentes (uma por area da API)
└── scripts/                      # install.sh, traducao dos specs
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
| `sys-locale` | 0.3 | Idioma do sistema |

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

### Fase 1: Carregar os produtos

```
argv: ["kobana", "banking", "charge", "pix", "list", "--params", '{"page":1}']
                    ^
                    produto = "banking"
```

1. `product::load_all()` le os specs embutidos de cada produto do `REGISTRY` e
   monta a arvore de comandos de cada um (os specs de um produto, v1 e v2 no
   banking, viram uma arvore so)
2. O `--lang` e lido dos argumentos crus antes, porque os comandos ja nascem no
   idioma escolhido

### Fase 2: Build + Parse

1. Construir a arvore `clap::Command`: `kobana <produto> <recurso…> <metodo>`,
   mais `auth` e os helpers (`+verb`) dentro de cada produto, e `schema`,
   `completions`, `update` e `help` no topo
2. Parsear os argumentos com clap
3. Comandos especiais (`help`, `schema`, `update`, `<produto> auth`,
   `completions`, helpers) sao atendidos direto
4. Resolver metodo HTTP + URL a partir do match
5. Autenticar, construir request, executar

### Mapeamento Path -> Comando

```
/v2/charge/pix              -> kobana banking charge pix list/create
/v2/charge/pix/{uid}        -> kobana banking charge pix get/update
/v2/charge/pix/{uid}/cancel -> kobana banking charge pix cancel
```

Regras de inferencia de metodo (detalhes no AGENTS.md):
- `GET` sem `{id}` no fim do path -> `list` (ou `get`, num singleton como `/v2/me`)
- `GET` com `{id}` no fim do path -> `get`
- `GET` com uma rota de item logo abaixo dele -> `list`
- `POST` -> `create`
- `PUT`/`PATCH` -> `update`
- `DELETE` -> `delete`
- Paths com acao no final (`/cancel`, `/approve`) -> metodo nomeado

---

## Autenticacao

### Precedencia

```
1. KOBANA_<PRODUTO>_TOKEN (env var)  -> Bearer token do produto
2. KOBANA_CREDENTIALS_FILE (env var) -> Arquivo JSON com tokens OAuth
3. Credenciais salvas                -> ~/.config/kobana/credentials-<produto>-<ambiente>.enc (renovadas com o refresh token)
```

### OAuth2 Flows

**Authorization Code** (para acessar contas de terceiros):
```
1. `kobana <produto> auth login` le os escopos em /.well-known/oauth-protected-resource do host MCP do produto
   CLI abre browser -> https://account.kobana.com.br/oauth/authorize?client_id=kobana-cli&resource=<API do produto>&scope=...&code_challenge=...
2. Usuario autoriza
3. Callback em http://localhost:PORT com ?code=...
4. CLI troca code por access_token via POST /oauth/token
5. Credenciais criptografadas e salvas
```

**Client Credentials** (para apps OAuth, so no banking: `--client-id` com `--client-secret`):
```
1. POST /oauth/token com grant_type=client_credentials&client_id=...&client_secret=...
2. Recebe access_token
3. Criptografa e salva
```

### Storage

```
~/.config/kobana/                          # ou KOBANA_CONFIG_DIR
├── credentials-<produto>-<ambiente>.enc  # AES-256-GCM encrypted OAuth tokens
├── credentials-<produto>-<ambiente>.lock # trava da renovacao do token
├── .key                                  # chave, so sem keyring
└── .env                                  # variaveis de ambiente (opcional)
```

Chave de encriptacao no OS keyring (macOS Keychain, Windows Credential Manager), uma entrada por diretorio de configuracao, com fallback para o arquivo `.key` (0600). No Linux nenhum backend de keyring e compilado, entao a chave fica sempre no `.key`.

---

## Schema Introspection

```bash
kobana schema banking.charge.pix.create
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
6. `auth.rs`: Bearer token por produto (`KOBANA_<PRODUTO>_TOKEN`, credenciais salvas com refresh)
7. `schema.rs`: introspecao de endpoints
8. Flags globais: `--params`, `--json`, `--fields`, `--dry-run`, `--env <production|sandbox|development>`
9. Exit codes estruturados

### Fase 2 — Auth Completa
10. `credential_store.rs`: AES-256-GCM encryption
11. `auth_commands.rs`: `kobana <produto> auth login/logout/status/export`
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
