# Kobana CLI

CLI para a API da [Kobana](https://kobana.com.br) — acesso completo às APIs v1 e v2 direto do terminal.

Projetado para humanos e agentes de IA, com saída JSON estruturada, introspecção de schema, dry-run e paginação automática.

```
kobana <servico> <recurso> <metodo> [flags]
```

![Kobana CLI Demo](docs/demo.gif)

## Instalação

### Homebrew (macOS e Linux)

```bash
brew tap universokobana/tap
brew install kobana
```

Para atualizar:

```bash
brew update && brew upgrade kobana
```

### Binários pré-compilados

Baixe o binário para sua plataforma na [página de Releases](https://github.com/universokobana/kobana-cli/releases/latest) e coloque no seu `PATH`.

### Nix

```bash
# Rodar direto do GitHub
nix run github:universokobana/kobana-cli

# Instalar no perfil
nix profile install github:universokobana/kobana-cli
```

### Build a partir do source

```bash
git clone https://github.com/universokobana/kobana-cli.git
cd kobana-cli
cargo install --path crates/kobana-cli
```

Requer [Rust](https://rustup.rs/) 1.70+.

## Autenticação

### Token de acesso (mais simples)

Obtenha o token em *Integracões > API > Token de API* na interface da Kobana.

```bash
export KOBANA_TOKEN=seu_token_aqui
```

### OAuth (PKCE)

O CLI usa OAuth com PKCE — funciona sem configurar nada:

```bash
# Login (abre browser, zero config)
kobana auth login

# Login com escopos específicos (default: read)
kobana auth login --scopes "read,write"

# Client credentials (para apps server-side)
kobana auth login --client-id <ID> --client-secret <SECRET>

# Ver status
kobana auth status

# Exportar credenciais (para CI)
kobana auth export > credentials.json

# Logout
kobana auth logout
```

Credenciais salvas são criptografadas com AES-256-GCM. A chave fica no keyring do OS (macOS Keychain, etc.) com fallback para arquivo.

### Prioridade de resolução

| Prioridade | Método | Configuração |
|------------|--------|--------------|
| 1 | Token direto | `KOBANA_TOKEN` |
| 2 | Arquivo de credenciais | `KOBANA_CREDENTIALS_FILE` |
| 3 | Credenciais salvas | `kobana auth login` |

### Ambientes

O CLI opera em dois ambientes. **Sandbox é o default** — use para testes sem afetar dados reais.

| Ambiente | API | OAuth | Flag |
|----------|-----|-------|------|
| Sandbox | `api-sandbox.kobana.com.br` | `app-sandbox.kobana.com.br` | `--sandbox` (default) |
| Produção | `api.kobana.com.br` | `app.kobana.com.br` | `--production` |

```bash
# Sandbox (default — não precisa de flag)
kobana charge pix list

# Produção
kobana charge pix list --production

# Via variável de ambiente
export KOBANA_ENVIRONMENT=production
kobana charge pix list

# Login em produção
kobana auth login --production
```

Os tokens são **diferentes entre ambientes** — um token de sandbox não funciona em produção e vice-versa.

## Uso

### Sintaxe

```bash
kobana <servico> <recurso> <metodo> [flags]
```

Serviços disponíveis:

| Comando | Descrição |
|---------|-----------|
| `v1` | API v1 — boletos, clientes, webhooks |
| `charge` | Cobranças — Pix, Pix automático |
| `payment` | Pagamentos — boletos, Pix, taxas, concessionárias |
| `transfer` | Transferências — Pix, TED, interna |
| `financial` | Financeiro — contas, saldos, extratos |
| `admin` | Administração — subcontas, usuários |
| `mailbox` | Caixa postal — EDI, arquivos |
| `data` | Consultas — boletos, QR codes Pix |
| `security` | Tokens de acesso |

### Exemplos

```bash
# Listar boletos com filtro
kobana v1 bank-billets list \
  --params '{"status": "opened", "per_page": 25}' \
  --fields "id,amount,status,due_at"

# Criar cobrança Pix
kobana charge pix create \
  --json '{"amount": 99.90, "pix_account_uid": "UID"}'

# Consultar saldo
kobana financial accounts balances list \
  --params '{"financial_account_uid": "UID"}'

# Transferência Pix
kobana transfer pix create \
  --json '{"amount": 500, "pix_key": "email@example.com"}'

# Listar com paginação automática (NDJSON)
kobana charge pix list --page-all --fields "uid,amount,status"

# Ver detalhes de um boleto
kobana v1 bank-billets get --params '{"id": 12345}'

# Cancelar boleto
kobana v1 bank-billets cancel --params '{"id": 12345}'

# Dry-run — ver a requisição sem executar
kobana charge pix create --json '{"amount": 100}' --dry-run

# Saída em tabela
kobana v1 bank-billets list --output-format table

# Salvar resposta em arquivo
kobana v1 bank-billets get --params '{"id": 12345}' --output boleto.json
```

### Helpers

Atalhos para operações comuns:

```bash
# Emitir boleto
kobana +emitir --valor 150.50 --vencimento 2026-05-01 \
  --nome "Maria Silva" --cpf-cnpj "012.345.678-90" --carteira 1

# Criar cobrança Pix
kobana +cobrar --valor 99.90 --conta-pix "UID" \
  --nome "João" --cpf-cnpj "012.345.678-90"

# Cancelar boletos em lote
kobana +cancelar-lote --ids "123,456,789"
```

## Introspecção de Schema

```bash
# Listar todos os serviços e recursos
kobana schema --list

# Ver schema de um endpoint específico
kobana schema charge.pix.create
kobana schema v1.bank-billets.list
```

Retorna parâmetros, campos obrigatórios, tipos e respostas — tudo derivado do OpenAPI spec embutido.

## Flags Globais

| Flag | Descrição |
|------|-----------|
| `--params '<JSON>'` | Parâmetros de query/URL (id, page, filtros) |
| `--json '<JSON>'` | Corpo da requisição (POST/PUT/PATCH) |
| `--fields '<CAMPOS>'` | Limita campos na resposta |
| `--dry-run` | Mostra a requisição sem executar |
| `--page-all` | Auto-paginação com saída NDJSON |
| `--page-limit <N>` | Máximo de páginas (default: 10) |
| `--page-delay <MS>` | Delay entre páginas (default: 100ms) |
| `--sandbox` | Ambiente sandbox (default) |
| `--production` | Ambiente produção |
| `--verbose` | Detalhes da requisição no stderr |
| `--output <PATH>` | Salva resposta em arquivo |
| `--output-format <FMT>` | Formato: `json`, `table`, `csv` |
| `--idempotency-key <KEY>` | Chave de idempotência customizada |

## Variáveis de Ambiente

| Variável | Descrição |
|----------|-----------|
| `KOBANA_TOKEN` | Token de acesso Bearer |
| `KOBANA_CREDENTIALS_FILE` | Caminho para arquivo JSON de credenciais |
| `KOBANA_CLIENT_ID` | OAuth client ID |
| `KOBANA_CLIENT_SECRET` | OAuth client secret |
| `KOBANA_CONFIG_DIR` | Diretório de config (default: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `sandbox` (default) ou `production` |
| `KOBANA_LOG` | Nível de log para stderr (ex: `kobana=debug`) |
| `KOBANA_LOG_FILE` | Diretório para logs JSON com rotação diária |

Variáveis também podem ser definidas em arquivo `.env`.

## Códigos de Saída

| Código | Significado |
|--------|-------------|
| `0` | Sucesso |
| `1` | Erro de API (4xx/5xx) |
| `2` | Erro de autenticação |
| `3` | Erro de validação |
| `4` | Erro de schema |
| `5` | Erro interno |

## Shell Completions

```bash
# Bash
kobana completions bash > /etc/bash_completion.d/kobana

# Zsh
kobana completions zsh > ~/.zfunc/_kobana

# Fish
kobana completions fish > ~/.config/fish/completions/kobana.fish

# PowerShell
kobana completions powershell > kobana.ps1
```

## CI/CD & Releases

O projeto usa GitHub Actions para CI e releases automatizadas.

### CI

Toda push e PR na `main` executa:

1. `cargo test` + `cargo clippy`
2. Build cross-platform (Linux amd64/arm64, macOS amd64/arm64, Windows amd64)

### Criar uma release

1. Atualize a versão em `crates/kobana-cli/Cargo.toml` e `crates/kobana/Cargo.toml`
2. Atualize o `CHANGELOG.md`
3. Commit com prefixo `release:`:

```bash
git add -A
git commit -m "release: v0.2.0"
git push
```

O workflow detecta o prefixo `release:` no commit message, compila os 5 targets e cria uma GitHub Release com os binários anexados.

### Download de binários

Binários pré-compilados estão disponíveis na [página de Releases](../../releases):

| Plataforma | Arquivo |
|------------|---------|
| Linux x86_64 | `kobana-linux-amd64` |
| Linux ARM64 | `kobana-linux-arm64` |
| macOS Intel | `kobana-darwin-amd64` |
| macOS Apple Silicon | `kobana-darwin-arm64` |
| Windows x86_64 | `kobana-windows-amd64.exe` |

## Arquitetura

```
kobana-cli/
├── crates/
│   ├── kobana/          # Biblioteca: HTTP client, error types, OpenAPI parsing, validação
│   └── kobana-cli/      # Binário: CLI, auth, formatação, paginação, helpers
│       └── specs/       # OpenAPI specs v1 e v2 embutidos
└── docs/                # Especificações e documentação de design
```

Comandos são gerados **dinamicamente** a partir dos OpenAPI specs da Kobana embutidos no binário. Atualizar a API = atualizar os specs + rebuild.

## Licença

MIT
