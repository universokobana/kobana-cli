# Kobana CLI

CLI para as APIs da [Kobana](https://kobana.com.br) — os seis produtos em um
único binário, direto do terminal.

Projetado para humanos e agentes de IA, com saída JSON estruturada, introspecção
de schema, dry-run e paginação automática.

```
kobana <produto> <recurso> <metodo> [flags]
```

![Kobana CLI Demo](docs/demo.gif)

## Produtos

A Kobana não é mais uma API só. Cada produto é uma API independente, com host,
documentação e credenciais próprios — e o CLI fala com todos. O produto é sempre
o primeiro segmento do comando.

| Produto | Comando | API | Documentação |
|---------|---------|-----|--------------|
| Gateway Bancário | `banking` | `api.kobana.com.br` | [docs.banking.kobana.com.br](https://docs.banking.kobana.com.br) |
| Financeiro Inteligente | `finance` | `api.finance.kobana.com.br` | [docs.finance.kobana.com.br](https://docs.finance.kobana.com.br) |
| Faturamento Automático | `billing` | `api.billing.kobana.com.br` | [docs.billing.kobana.com.br](https://docs.billing.kobana.com.br) |
| Inbox Autônomo | `inbox` | `api.inbox.kobana.com.br` | [docs.inbox.kobana.com.br](https://docs.inbox.kobana.com.br) |
| Régua de Cobrança | `dunning` | `api.dunning.kobana.com.br` | [docs.dunning.kobana.com.br](https://docs.dunning.kobana.com.br) |
| Dashboard Mágico | `dashboards` | `api.dashboards.kobana.com.br` | [docs.dashboards.kobana.com.br](https://docs.dashboards.kobana.com.br) |

> [!IMPORTANT]
> Produtos não compartilham credenciais. Um token do Gateway Bancário não
> autentica no Faturamento Automático, e vice-versa. Veja [Autenticação](#autenticação).

### Gateway Bancário — `banking`

Operações bancárias unificadas: uma única API conectada a mais de 40 bancos para
emitir boletos, cobrar e pagar via Pix, transferir por TED, consultar extratos e
conciliar.

É o produto mais antigo da Kobana e o que originou este CLI — era "a API da
Kobana" antes da separação por produtos. Tem duas gerações de API convivendo
(v1 e v2), mas isso não aparece no comando: os recursos legados da v1
(`bank-billets`, `customers`, `webhooks`) ficam lado a lado com os domínios da
v2 (`charge`, `payment`, `transfer`, `financial`, `admin`, `mailbox`, `data`,
`security`), e o CLI resolve a versão certa na URL.

```bash
kobana banking bank-billets list          # boleto (v1)
kobana banking charge pix create          # cobrança Pix (v2)
kobana banking transfer ted create        # TED (v2)
```

### Financeiro Inteligente — `finance`

Back-office financeiro: contas a pagar, contas a receber, conciliação bancária e
multi-empresa em uma plataforma. É onde o dinheiro é classificado e explicado,
não movimentado — para movimentar, use o `banking`.

Recursos: `financial-accounts` (com `balances`), `financial-transactions`,
`payables`, `receivables`, `cash-flow`, `reconciliations`, `categories`,
`classification-centers`, `custom-classifiers`, `automatic-rules`, `people`,
`companies`, `taxes`, `attachments`, `banks`, `receipts`, `transfers`,
`fiscal-documents`, `me`.

Toda rota age sobre um espaço de trabalho, informado no cabeçalho
`X-Workspace-Id` — veja [Espaço de trabalho](#espaço-de-trabalho-x-workspace-id).

```bash
kobana finance payables list --params '{"status": "pending"}'
kobana finance cash-flow list --params '{"start_date": "2026-01-01"}'
```

Os endpoints `aggregates` de `payables`, `receivables` e
`financial-transactions` devolvem totais em vez de listas — úteis para não
encher a janela de contexto de um agente.

### Faturamento Automático — `billing`

Cobrança recorrente: assinaturas, planos, faturas, gestão de clientes e portal
próprio. Cuida do ciclo de faturamento (o que cobrar, de quem, quando e quanto);
a liquidação bancária em si acontece no `banking`.

Recursos principais: `subscriptions`, `plans`, `plan-groups`, `products`,
`invoices`, `nfes`, `payments`, `payment-methods`, `proposals`, `coupons`,
`credits`, `billing-accounts`, `customers`, `companies`, `tax-rules`,
`late-fee-policies`, `bank-accounts`, `pix-keys`, `holidays`, `usage`.

```bash
kobana billing subscriptions list --params '{"status": "active"}'
kobana billing invoices finalize --params '{"id": "INVOICE_ID"}'
```

### Inbox Autônomo — `inbox`

Caixas de entrada com agentes. Recebe e-mails em endereços dedicados
(`inboxes`), processa cada mensagem com agentes (`agents`, `agent-runs`) e
entrega o resultado via webhooks com replay de entregas.

Recursos: `workspaces`, `inboxes`, `emails`, `agents` (com `versions`),
`agent-runs`, `workflow-runs`, `secrets`, `models`, `usage`,
`webhooks` (com `deliveries`), `system-events`.

```bash
kobana inbox emails list --fields "id,subject,received_at"
kobana inbox emails reprocess --params '{"id": "EMAIL_ID"}'
```

Este produto exige mTLS além do token — veja
[mTLS no Inbox Autônomo](#mtls-no-inbox-autônomo).

### Régua de Cobrança — `dunning`

Recuperação de inadimplência: clientes devedores, cobranças, réguas de
comunicação, acordos de parcelamento, disputas, negativações e protestos.

Recursos: `people`, `charges`, `collection-rules` (com `steps`), `templates`,
`notifications`, `notification-channels`, `agreements`, `disputes`,
`negativations`, `protests`, `tasks`, `interactions`, `classifications`,
`negotiation-campaigns`, `imports`, `exports`, `metrics`, `webhook-endpoints`.

```bash
kobana dunning charges list --params '{"status": "overdue"}'
kobana dunning metrics recovery list
```

### Dashboard Mágico — `dashboards`

Dashboards e portais do cliente montados sobre fontes de dados conectadas.
Sincroniza dados de ERPs e APIs (`data-sources`), modela entidades
(`entities`), publica um portal com usuários próprios e edita as páginas pelo
`builder`.

Recursos principais: `data-sources`, `entities`, `workspaces`, `portal-users`,
`portal-settings`, `builder`, `assets`, `webhooks`, `backups`, `api-keys`.

```bash
kobana dashboards data-sources list --fields "id,name,status"
kobana dashboards data-sources sync --dry-run --params '{"id": "DATA_SOURCE_ID"}'
```

## Instalação

### Script de instalação (macOS e Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash
```

Baixa o binário da release mais recente para `~/.kobana/bin`, confere o SHA-256
publicado no GitHub e cria o link `~/.local/bin/kobana`. Para uma versão
específica, passe-a como argumento (`| bash -s -- v0.5.1`). `KOBANA_INSTALL_DIR`
e `KOBANA_BIN_DIR` trocam as pastas. As mensagens saem em português, inglês ou
espanhol conforme o idioma do sistema (`LC_ALL`, `LC_MESSAGES`, `LANG`);
`KOBANA_LANG=pt|en|es` força um deles. Depois, `kobana update` atualiza o
próprio binário.

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

Requer [Rust](https://rustup.rs/) 1.86+.

## Autenticação

Cada produto autentica separadamente. Em todos eles o token vai em
`KOBANA_TOKEN`, mas **o token de um produto não vale para outro** — e a forma de
obtê-lo muda:

| Produto | Credencial | Como obter |
|---------|-----------|------------|
| `banking` | Token de API ou OAuth | Interface da Kobana ou `kobana auth login` |
| `finance` | JWT (RS256) + `X-Workspace-Id` | Conta Kobana (`account.kobana.com.br`) |
| `billing` | JWT (RS256) | Conta Kobana (`account.kobana.com.br`) |
| `inbox` | JWT (RS256) + certificado mTLS | Conta Kobana (`account.kobana.com.br`) |
| `dunning` | JWT (RS256) | Conta Kobana (`account.kobana.com.br`) |
| `dashboards` | JWT (RS256), `X-Workspace-Id` opcional | Conta Kobana (`account.kobana.com.br`) |

### Gateway Bancário

#### Token de acesso (mais simples)

Obtenha o token em *Integracões > API > Token de API* na interface da Kobana.

```bash
export KOBANA_TOKEN=seu_token_aqui
```

#### OAuth (PKCE)

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

> [!NOTE]
> `kobana auth login` é do Gateway Bancário. Ele não emite tokens para os
> demais produtos.

### Demais produtos

`finance`, `billing`, `inbox`, `dunning` e `dashboards` só **verificam** tokens:
todos aceitam um access token **RS256 emitido pelo Authorization Server da
Kobana** (`account.kobana.com.br`), enviado como `Authorization: Bearer <jwt>`.
O `aud` do token é a URL da API do produto (`https://api.billing.kobana.com.br`,
…), então um token continua valendo para um produto só. Coloque o token do
produto em `KOBANA_TOKEN` antes de chamá-lo:

```bash
KOBANA_TOKEN=$TOKEN_FINANCE kobana finance payables list
KOBANA_TOKEN=$TOKEN_BILLING kobana billing subscriptions list
```

Como a variável é a mesma para todos os produtos, exporte-a por comando (ou use
um `.env` por projeto) se você alterna entre produtos na mesma sessão.

#### Espaço de trabalho (`X-Workspace-Id`)

No `finance`, toda rota age sobre **um** espaço de trabalho da organização do
token, escolhido pelo cabeçalho `X-Workspace-Id` — sem ele a API responde
`400 WORKSPACE_REQUIRED`. No `dashboards` e no `dunning` o cabeçalho é
opcional enquanto a organização tem um único workspace (`kobana dunning
workspaces list` mostra os que o token alcança).

O CLI preenche o cabeçalho a partir de `KOBANA_WORKSPACE_ID`. Para trocar de
espaço em uma chamada só, passe-o em `--params`, que tem precedência:

```bash
export KOBANA_WORKSPACE_ID=uuid-do-espaco
kobana finance payables list
kobana finance payables list --params '{"X-Workspace-Id": "outro-uuid"}'
```

O `--dry-run` mostra os cabeçalhos enviados (sem o token), e
`kobana schema` lista os cabeçalhos de cada endpoint em `header_params`.

#### mTLS no Inbox Autônomo

O `inbox` é o único produto atrás de **mTLS**: além do JWT, o CLI precisa
apresentar um certificado de cliente no handshake TLS. Aponte
`KOBANA_INBOX_CLIENT_CERT` para um arquivo PEM contendo a cadeia do certificado
e sua chave privada:

```bash
export KOBANA_INBOX_CLIENT_CERT=~/.config/kobana/inbox-client.pem
kobana inbox emails list
```

Sem essa variável o CLI avisa em stderr e segue — desenvolvimento local não usa
mTLS, e em produção a API responde `401` explicando. Os escopos do Inbox têm
dois eixos: o recurso, prefixado pelo app (`inbox/emails`), e o verbo global
(`write`) — escrever exige os dois.

### Prioridade de resolução

| Prioridade | Método | Configuração |
|------------|--------|--------------|
| 1 | Token direto | `KOBANA_TOKEN` |
| 2 | Arquivo de credenciais | `KOBANA_CREDENTIALS_FILE` |
| 3 | Credenciais salvas | `kobana auth login` |

### Ambientes

O CLI opera em três ambientes. **Produção é o default.** Cada produto tem seus
próprios hosts:

| Produto | `production` (default) | `sandbox` | `development` |
|---------|------------------------|-----------|---------------|
| `banking` | `api.kobana.com.br` | `api-sandbox.kobana.com.br` | `localhost:5005/api` |
| `finance` | `api.finance.kobana.com.br` | `api.finance.sandbox.kobana.com.br` | — |
| `billing` | `api.billing.kobana.com.br` | `api.billing.sandbox.kobana.com.br` | — |
| `inbox` | `api.inbox.kobana.com.br` | — | `localhost:3028/api` |
| `dunning` | `api.dunning.kobana.com.br` | `api.dunning.sandbox.kobana.com.br` | — |
| `dashboards` | `api.dashboards.kobana.com.br` | — | — |

Produtos sem host de desenvolvimento próprio caem em produção quando você passa
`--env development`. Já `--env sandbox` num produto sem sandbox (`inbox` e
`dashboards`) termina com erro, para nunca mandar um teste para produção.

O OAuth (`kobana auth login`) é do Gateway Bancário e usa `app.kobana.com.br` em
produção e `app-sandbox.kobana.com.br` em sandbox.

```bash
# Produção (default — não precisa de flag)
kobana banking charge pix list

# Sandbox
kobana banking charge pix list --env sandbox

# Development local
kobana banking charge pix list --env development

# Via variável de ambiente
export KOBANA_ENVIRONMENT=sandbox
kobana banking charge pix list

# Login em sandbox
kobana auth login --env sandbox
```

Os tokens são **diferentes entre ambientes** — um token de sandbox não funciona em produção e vice-versa.

### Idioma

O CLI fala português, inglês ou espanhol: ajuda, mensagens, erros, perguntas do
`kobana auth login` e a página que o navegador mostra no fim do login. As
descrições dos endpoints, no `--help` e no `kobana schema`, vêm traduzidas das
specs que cada produto publica nas suas docs.

O idioma vem, nesta ordem, de `--lang`, de `KOBANA_LANG`, das variáveis de idioma
do terminal (`LC_ALL`, `LC_MESSAGES`, `LANG`) e, por fim, do idioma do sistema.
Locales `pt*` dão português, `es*` dão espanhol e qualquer outro dá inglês.

```bash
kobana --lang en banking bank-billets --help
export KOBANA_LANG=es
```

O CLI também envia `Accept-Language` nas requisições, para que as APIs que
traduzem as próprias mensagens respondam no mesmo idioma. Nomes de comandos e
flags, chaves do JSON e códigos de saída não mudam com o idioma.

## Uso

### Sintaxe

```bash
kobana <produto> <recurso> <metodo> [flags]
```

- **`<produto>`** — `banking`, `finance`, `billing`, `inbox`, `dunning` ou
  `dashboards` (veja [Produtos](#produtos))
- **`<recurso>`** — pode ser aninhado: `bank-billets`, `charge pix`,
  `webhooks deliveries`
- **`<metodo>`** — a ação: `list`, `get`, `create`, `update`, `delete`, ou uma
  ação específica do recurso como `cancel`, `pause`, `replay`

A versão da API nunca entra no comando. `kobana banking bank-billets list` vai
para `/v1/bank_billets` e `kobana banking charge pix list` vai para
`/v2/charge/pix` — o CLI resolve isso a partir da spec OpenAPI.

Parâmetros de URL, query e cabeçalho vão em `--params`, corpo de requisição em `--json`:

```bash
kobana billing subscriptions pause --params '{"id": "SUBSCRIPTION_ID"}'
kobana banking charge pix create --json '{"amount": 99.90, "pix_account_uid": "UID"}'
```

Toda a superfície é descoberta pelo `--help`, em qualquer nível:

```bash
kobana --help                    # produtos
kobana finance --help            # recursos do Financeiro Inteligente
kobana finance payables --help   # métodos de contas a pagar
```

### Exemplos

#### Gateway Bancário

```bash
# Listar boletos com filtro
kobana banking bank-billets list \
  --params '{"status": "opened", "per_page": 25}' \
  --fields "id,amount,status,due_at"

# Criar cobrança Pix
kobana banking charge pix create \
  --json '{"amount": 99.90, "pix_account_uid": "UID"}'

# Consultar saldo
kobana banking financial accounts balances list \
  --params '{"financial_account_uid": "UID"}'

# Transferência Pix
kobana banking transfer pix create \
  --json '{"amount": 500, "pix_key": "email@example.com"}'

# Listar com paginação automática (NDJSON)
kobana banking charge pix list --page-all --fields "uid,amount,status"

# Ver detalhes de um boleto
kobana banking bank-billets get --params '{"id": 12345}'

# Cancelar boleto
kobana banking bank-billets cancel --params '{"id": 12345}'

# Dry-run — ver a requisição sem executar
kobana banking charge pix create --json '{"amount": 100}' --dry-run

# Saída em tabela
kobana banking bank-billets list --output-format table

# Salvar resposta em arquivo
kobana banking bank-billets get --params '{"id": 12345}' --output boleto.json
```

#### Financeiro Inteligente

```bash
# Contas a pagar em aberto
kobana finance payables list \
  --params '{"status": "pending"}' \
  --fields "id,description,amount,due_date"

# Totais agregados em vez da lista inteira
kobana finance payables aggregates list --params '{"group_by": "category"}'

# Enviar uma conta para pagamento no banco
kobana finance payables send-to-bank --dry-run --params '{"id": "PAYABLE_ID"}'

# Fluxo de caixa do mês
kobana finance cash-flow list \
  --params '{"start_date": "2026-01-01", "end_date": "2026-01-31"}'
```

#### Faturamento Automático

```bash
# Assinaturas ativas
kobana billing subscriptions list \
  --params '{"status": "active"}' \
  --fields "id,status,plan_id,next_billing_at"

# Pausar e retomar uma assinatura
kobana billing subscriptions pause --params '{"id": "SUBSCRIPTION_ID"}'
kobana billing subscriptions resume --params '{"id": "SUBSCRIPTION_ID"}'

# Fechar uma fatura e emitir a NF-e
kobana billing invoices finalize --dry-run --params '{"id": "INVOICE_ID"}'
kobana billing invoices issue-nfe --dry-run --params '{"id": "INVOICE_ID"}'
```

#### Inbox Autônomo

```bash
# E-mails recebidos
kobana inbox emails list --fields "id,subject,received_at"

# Reprocessar um e-mail com os agentes
kobana inbox emails reprocess --params '{"id": "EMAIL_ID"}'

# Reenviar uma entrega de webhook que falhou
kobana inbox webhooks deliveries replay \
  --params '{"id": "WEBHOOK_ID", "deliveryId": "DELIVERY_ID"}'
```

#### Régua de Cobrança

```bash
# Cobranças vencidas
kobana dunning charges list \
  --params '{"status": "overdue"}' \
  --fields "id,person_id,current_amount,due_date"

# Notificar o devedor de uma cobrança
kobana dunning charges notify --dry-run --params '{"id": "CHARGE_ID"}'

# Aceitar um acordo de parcelamento
kobana dunning agreements accept --dry-run --params '{"id": "AGREEMENT_ID"}'
```

#### Dashboard Mágico

```bash
# Fontes de dados e sincronização
kobana dashboards data-sources list --fields "id,name,status"
kobana dashboards data-sources sync --dry-run --params '{"id": "DATA_SOURCE_ID"}'

# Registros de uma entidade
kobana dashboards entities records get --params '{"id": "ENTITY_ID"}'
```

### Helpers

Atalhos para operações comuns do **Gateway Bancário**:

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
# Listar todos os produtos e seus recursos
kobana schema --list

# Ver schema de um endpoint específico
kobana schema banking.charge.pix.create
kobana schema finance.payables.create
kobana schema billing.subscriptions.create
kobana schema inbox.emails.list
kobana schema dunning.charges.create
kobana schema dashboards.data-sources.sync
```

Retorna parâmetros, campos obrigatórios, tipos e respostas — tudo derivado dos
OpenAPI specs embutidos. O caminho segue a mesma forma do comando:
`<produto>.<recurso>.<metodo>`.

## Flags Globais

| Flag | Descrição |
|------|-----------|
| `--params '<JSON>'` | Parâmetros de query/URL/cabeçalho (id, page, filtros, `X-Workspace-Id`) |
| `--json '<JSON>'` | Corpo da requisição (POST/PUT/PATCH) |
| `--fields '<CAMPOS>'` | Limita campos na resposta |
| `--dry-run` | Mostra a requisição sem executar |
| `--page-all` | Auto-paginação com saída NDJSON |
| `--page-limit <N>` | Máximo de páginas (default: 10) |
| `--page-delay <MS>` | Delay entre páginas (default: 100ms) |
| `--env <ENV>` | Ambiente: `production` (default), `sandbox`, `development` |
| `--verbose` | Detalhes da requisição no stderr |
| `--output <PATH>` | Salva resposta em arquivo |
| `--output-format <FMT>` | Formato: `json`, `table`, `csv` |
| `--idempotency-key <KEY>` | Chave de idempotência customizada |

## Variáveis de Ambiente

| Variável | Descrição |
|----------|-----------|
| `KOBANA_TOKEN` | Token do produto que você vai chamar (Bearer). Não é compartilhado entre produtos |
| `KOBANA_INBOX_CLIENT_CERT` | Caminho para o PEM (certificado + chave) usado no mTLS do `inbox` |
| `KOBANA_WORKSPACE_ID` | Espaço de trabalho enviado em `X-Workspace-Id` pelo `finance` (obrigatório), pelo `dashboards` e pelo `dunning` |
| `KOBANA_CREDENTIALS_FILE` | Caminho para arquivo JSON de credenciais |
| `KOBANA_CLIENT_ID` | OAuth client ID (Gateway Bancário) |
| `KOBANA_CLIENT_SECRET` | OAuth client secret (Gateway Bancário) |
| `KOBANA_CONFIG_DIR` | Diretório de config (default: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `production` (default), `sandbox` ou `development` |
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
│       └── specs/       # OpenAPI specs embutidos (<produto>-<versao>.json)
└── docs/                # Especificações e documentação de design
```

Comandos são gerados **dinamicamente** a partir dos OpenAPI specs dos seis
produtos, embutidos no binário. Atualizar a API = atualizar o spec + rebuild;
adicionar um produto = um spec novo mais uma entrada no registro de produtos.

## Licença

MIT
