# Kobana CLI — Especificacao de Interface

## Visao Geral

O `kobana` CLI fornece acesso a toda a API da Kobana (v1 e v2) a partir do terminal. Projetado para humanos e agentes de IA, com saida JSON estruturada, introspecao de schema, dry-run e paginacao automatica.

Escrito em **Rust** (clap + serde + reqwest + tokio). Inspirado no [gws-cli](https://github.com/googleworkspace/cli) e nos principios de [CLI design for AI agents](https://justin.poehnelt.com/posts/rewrite-your-cli-for-ai-agents/).

---

## Sintaxe Principal

```bash
kobana <service> <resource> <method> [flags]
```

Onde:
- `<service>` — dominio da API (ex: `charge`, `payment`, `transfer`, `financial`, `admin`)
- `<resource>` — recurso dentro do dominio (ex: `pix`, `bank-billets`, `accounts`)
- `<method>` — acao (ex: `list`, `get`, `create`, `update`, `delete`, `cancel`)

### Exemplos

```bash
# Listar boletos
kobana charge bank-billets list --params '{"page": 1, "per_page": 50}'

# Criar cobranca Pix
kobana charge pix create --json '{"amount": 1500, "payer": {...}}'

# Ver detalhes de uma transferencia
kobana transfer pix get --params '{"uid": "019d6b00-4751-719d-8a6f-20cb9223bea4"}'

# Consultar saldo
kobana financial accounts balances --params '{"financial_account_uid": "UID"}'
```

---

## Mapeamento API -> CLI

### V1 — API Legada (Boletos)

| Servico | Recurso | Metodos | Path da API |
|---------|---------|---------|-------------|
| `v1` | `bank-billets` | `list`, `get`, `create`, `update`, `cancel`, `duplicate`, `pay`, `send-email`, `send-sms`, `cancel-all`, `protest` | `/v1/bank_billets` |
| `v1` | `bank-billet-accounts` | `list`, `get`, `create`, `update`, `ask`, `set-default`, `validate` | `/v1/bank_billet_accounts` |
| `v1` | `bank-billet-batches` | `list`, `get`, `create`, `add-billets`, `remove-billet`, `pdf`, `zip` | `/v1/bank_billet_batches` |
| `v1` | `bank-billet-discharges` | `list`, `get` | `/v1/bank_billet_discharges` |
| `v1` | `bank-billet-payments` | `list`, `get` | `/v1/bank_billet_payments` |
| `v1` | `bank-billet-registrations` | `list`, `get` | `/v1/bank_billet_registrations` |
| `v1` | `bank-billet-remittances` | `list`, `get`, `pending` | `/v1/bank_billet_remittances` |
| `v1` | `customers` | `list`, `get`, `create`, `update`, `by-cnpj-cpf`, `by-email` | `/v1/customers` |
| `v1` | `customer-subscriptions` | `list`, `get`, `create`, `update`, `next-charge` | `/v1/customer_subscriptions` |
| `v1` | `discharges` | `list`, `get`, `download`, `pay-off`, `reprocess` | `/v1/discharges` |
| `v1` | `remittances` | `list`, `get`, `raw`, `bulk` | `/v1/remittances` |
| `v1` | `installments` | `list`, `get`, `create` | `/v1/installments` |
| `v1` | `events` | `list`, `get` | `/v1/events` |
| `v1` | `webhooks` | `list`, `get`, `create`, `update` | `/v1/webhooks` |
| `v1` | `webhook-deliveries` | `list`, `get`, `resend` | `/v1/webhook_deliveries` |
| `v1` | `email-deliveries` | `list`, `get`, `resend` | `/v1/email_deliveries` |
| `v1` | `sms-deliveries` | `list`, `get`, `resend` | `/v1/sms_deliveries` |
| `v1` | `imports` | `list`, `get`, `create` | `/v1/imports` |
| `v1` | `reports` | `bank-billets` | `/v1/reports` |
| `v1` | `userinfo` | `get` | `/v1/userinfo` |

### V2 — API Modular

#### Admin

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `admin certificates` | `list` | `/v2/admin/certificates` |
| `admin connections` | `list`, `get`, `associations` | `/v2/admin/connections` |
| `admin subaccounts` | `list`, `get`, `create`, `update` | `/v2/admin/subaccounts` |
| `admin users` | `list`, `get`, `create`, `update` | `/v2/admin/users` |

#### Charge (Cobrancas)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `charge pix` | `list`, `get`, `create`, `update`, `cancel`, `commands` | `/v2/charge/pix` |
| `charge pix-accounts` | `list`, `get` | `/v2/charge/pix_accounts` |
| `charge automatic-pix accounts` | `list`, `get` | `/v2/charge/automatic_pix/accounts` |
| `charge automatic-pix locations` | `list`, `get` | `/v2/charge/automatic_pix/locations` |
| `charge automatic-pix pix` | `list`, `get`, `create`, `cancel`, `retry` | `/v2/charge/automatic_pix/pix` |
| `charge automatic-pix recurrences` | `list`, `get`, `create`, `update`, `cancel` | `/v2/charge/automatic_pix/recurrences` |
| `charge automatic-pix requests` | `list`, `get`, `cancel` | `/v2/charge/automatic_pix/requests` |
| `charge payments` | `list`, `get` | `/v2/charge/payments` |

#### Payment (Pagamentos)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `payment accounts` | `list`, `get` | `/v2/payment/accounts` |
| `payment bank-billets` | `list`, `get`, `create`, `cancel` | `/v2/payment/bank_billets` |
| `payment batches` | `list`, `get`, `create`, `approve`, `reprove` | `/v2/payment/batches` |
| `payment pix` | `list`, `get`, `create`, `cancel` | `/v2/payment/pix` |
| `payment taxes` | `list`, `get`, `create`, `cancel` | `/v2/payment/taxes` |
| `payment utilities` | `list`, `get`, `create`, `cancel` | `/v2/payment/utilities` |
| `payment dda-accounts` | `list`, `get`, `enable`, `disable` | `/v2/payment/dda_accounts` |
| `payment dda-bank-billets` | `list`, `get`, `reject`, `release` | `/v2/payment/dda_bank_billets` |

#### Transfer (Transferencias)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `transfer accounts` | `list`, `get` | `/v2/transfer/accounts` |
| `transfer batches` | `list`, `get`, `create`, `approve`, `reprove` | `/v2/transfer/batches` |
| `transfer pix` | `list`, `get`, `create`, `cancel` | `/v2/transfer/pix` |
| `transfer ted` | `list`, `get`, `create`, `cancel` | `/v2/transfer/ted` |
| `transfer internal` | `list`, `get`, `create`, `cancel` | `/v2/transfer/internal` |

#### Financial (Financeiro)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `financial accounts` | `list`, `get`, `create`, `update` | `/v2/financial/accounts` |
| `financial accounts balances` | `list`, `get` | `/v2/financial/accounts/{uid}/balances` |
| `financial accounts commands` | `list`, `get` | `/v2/financial/accounts/{uid}/commands` |
| `financial accounts statement` | `list`, `sync`, `imports` | `/v2/financial/accounts/{uid}/statement_transactions` |
| `financial providers` | `list` | `/v2/financial/providers` |

#### Mailbox (Caixa Postal / EDI)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `mailbox entries` | `list`, `get`, `create` | `/v2/mailbox/entries` |
| `mailbox entries files` | `list` | `/v2/mailbox/entries/{uid}/files` |
| `mailbox entries email` | `get`, `activate`, `deactivate` | `/v2/mailbox/entries/{id}/email` |
| `mailbox entries sftp` | `get`, `activate`, `deactivate`, `fetch`, `update-password` | `/v2/mailbox/entries/{id}/sftp` |
| `mailbox entries s3` | `get`, `activate`, `deactivate`, `fetch`, `update-password` | `/v2/mailbox/entries/{id}/s3` |
| `mailbox entries syncthing` | `get`, `activate`, `deactivate`, `resend-invites`, `update-status` | `/v2/mailbox/entries/{id}/syncthing` |
| `mailbox entries whatsapp` | `get`, `activate`, `deactivate` | `/v2/mailbox/entries/{id}/whatsapp` |
| `mailbox files` | `list`, `get` | `/v2/mailbox/files` |
| `edi boxes` | `list`, `get` | `/v2/edi/edi_boxes` |

#### Data (Consultas)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `data bank-billet-queries` | `create` | `/v2/data/bank_billet_queries` |
| `data pix-qrcode-queries` | `create` | `/v2/data/pix_qrcode_queries` |

#### Outros

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `me` | `get` | `/v2/me` |
| `payments` | `list`, `get`, `cancel` | `/v2/payments` |
| `transfers` | `list`, `get`, `cancel` | `/v2/transfers` |

---

## Flags Globais

| Flag | Descricao | Exemplo |
|------|-----------|---------|
| `--params '<JSON>'` | Parametros de query/URL (id, page, per_page, filtros) | `--params '{"page": 1, "per_page": 25}'` |
| `--json '<JSON>'` | Corpo da requisicao (POST/PUT/PATCH) | `--json '{"amount": 1500}'` |
| `--fields '<MASK>'` | Limita campos na resposta (protege context window de agentes) | `--fields "id,uid,status,amount"` |
| `--page-all` | Auto-paginacao com saida NDJSON | |
| `--page-limit <N>` | Maximo de paginas a buscar (default: 10) | `--page-limit 50` |
| `--page-delay <MS>` | Delay entre paginas (default: 100ms) | `--page-delay 200` |
| `--dry-run` | Valida e mostra a requisicao sem executar | |
| `--output <PATH>` | Salva resposta em arquivo (downloads) | `--output ./boleto.pdf` |
| `--sandbox` | Usa ambiente sandbox (`api-sandbox.kobana.com.br`) | |
| `--production` | Usa ambiente producao (`api.kobana.com.br`) | |
| `--version`, `-v` | Versao da API (v1 ou v2, inferida do servico) | `--version v2` |
| `--verbose` | Mostra headers e detalhes da requisicao no stderr | |
| `--help`, `-h` | Ajuda contextual | |
| `--output-format` | Formato de saida: `json` (default), `table`, `csv` | `--output-format table` |

---

## Autenticacao

### Metodos Suportados

| Prioridade | Metodo | Configuracao |
|------------|--------|-------------|
| 1 | Token de acesso (env var) | `KOBANA_TOKEN` |
| 2 | Credenciais OAuth (arquivo) | `KOBANA_CREDENTIALS_FILE` |
| 3 | Client Credentials (OAuth) | `KOBANA_CLIENT_ID` + `KOBANA_CLIENT_SECRET` |
| 4 | Credenciais salvas (criptografadas) | `kobana auth login` |

### Comandos de Auth

```bash
# Login interativo — abre browser para OAuth authorization code flow
kobana auth login

# Login com client credentials (para apps OAuth)
kobana auth login --client-id <ID> --client-secret <SECRET>

# Ver status da autenticacao
kobana auth status

# Exportar credenciais (para CI/headless)
kobana auth export > credentials.json

# Logout — remove credenciais salvas
kobana auth logout
```

### OAuth Flows

A Kobana suporta tres metodos de autenticacao:

1. **Token de Acesso** — Token obtido diretamente na interface da Kobana em *Integracoes > API > Token de API*. Diferente entre sandbox e producao. Configurado via env var `KOBANA_TOKEN`.

2. **Authorization Code Flow** — Para acessar contas de terceiros. O CLI abre o browser para autorizacao e recebe o callback em localhost.

3. **Client Credentials Flow** — Para apps OAuth. Troca `client_id` + `client_secret` por um access token.

### Armazenamento Seguro

Credenciais sao criptografadas em repouso (AES-256-GCM) com chave armazenada no keyring do OS (ou arquivo fallback para CI/Docker).

```
~/.config/kobana/credentials.enc    # credenciais criptografadas
~/.config/kobana/config.json        # configuracoes (ambiente, defaults)
```

---

## Introspecao de Schema

```bash
# Ver schema de um endpoint
kobana schema charge.pix.create
kobana schema v1.bank_billets.list

# Ver todos os servicos disponiveis
kobana schema --list

# Ver recursos de um servico
kobana schema charge --list
```

Retorna JSON com: parametros, corpo da requisicao, tipo de resposta, campos obrigatorios e opcionais. Alimentado pelo OpenAPI spec embutido.

---

## Paginacao

```bash
# Paginar manualmente
kobana charge pix list --params '{"page": 1, "per_page": 50}'

# Auto-paginacao (NDJSON — um JSON por linha)
kobana charge pix list --page-all

# Limitar paginas
kobana charge pix list --page-all --page-limit 5
```

---

## Variaveis de Ambiente

| Variavel | Descricao |
|----------|-----------|
| `KOBANA_TOKEN` | Token de acesso Bearer (maior prioridade) |
| `KOBANA_CREDENTIALS_FILE` | Caminho para arquivo JSON de credenciais OAuth |
| `KOBANA_CLIENT_ID` | OAuth client ID |
| `KOBANA_CLIENT_SECRET` | OAuth client secret |
| `KOBANA_CONFIG_DIR` | Diretorio de configuracao (default: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `sandbox` (default) ou `production` |
| `KOBANA_LOG` | Nivel de log para stderr (ex: `kobana=debug`) |
| `KOBANA_LOG_FILE` | Diretorio para logs JSON com rotacao diaria |

Variaveis tambem podem ser definidas em arquivo `.env`.

---

## Codigos de Saida

| Codigo | Significado | Causa |
|--------|-------------|-------|
| `0` | Sucesso | Comando completou normalmente |
| `1` | Erro de API | Kobana retornou 4xx/5xx |
| `2` | Erro de auth | Credenciais ausentes, expiradas ou invalidas |
| `3` | Erro de validacao | Argumentos invalidos, servico desconhecido |
| `4` | Erro de schema | Nao conseguiu carregar spec OpenAPI |
| `5` | Erro interno | Falha inesperada |

---

## Exemplos de Uso Completos

### Boletos (V1)

```bash
# Listar boletos com filtro
kobana v1 bank-billets list \
  --params '{"status": "opened", "per_page": 25}' \
  --fields "id,amount,status,due_at,customer_person_name"

# Criar boleto
kobana v1 bank-billets create \
  --json '{
    "amount": 150.50,
    "expire_at": "2026-05-01",
    "customer_person_name": "Maria Silva",
    "customer_cnpj_cpf": "012.345.678-90",
    "bank_billet_account_id": 1
  }' \
  --dry-run

# Cancelar boleto
kobana v1 bank-billets cancel --params '{"id": 12345}'

# Enviar boleto por email
kobana v1 bank-billets send-email --params '{"id": 12345}'
```

### Cobrancas Pix (V2)

```bash
# Criar cobranca Pix
kobana charge pix create \
  --json '{
    "amount": 99.90,
    "pix_account_uid": "UID_HERE",
    "payer": {"name": "Joao", "document": "012.345.678-90"}
  }'

# Listar cobrancas com paginacao automatica
kobana charge pix list --page-all --fields "uid,amount,status,created_at"

# Cancelar cobranca
kobana charge pix cancel --params '{"uid": "PIX_UID"}'
```

### Pagamentos (V2)

```bash
# Criar pagamento de boleto
kobana payment bank-billets create \
  --json '{"barcode": "23793.38128 ...", "amount": 150.00}' \
  --dry-run

# Aprovar lote de pagamentos
kobana payment batches approve --params '{"uid": "BATCH_UID"}'

# Listar pagamentos Pix
kobana payment pix list --params '{"per_page": 20}'
```

### Transferencias (V2)

```bash
# Transferencia Pix
kobana transfer pix create \
  --json '{
    "amount": 500.00,
    "pix_key": "email@example.com",
    "transfer_account_uid": "UID"
  }'

# TED
kobana transfer ted create \
  --json '{
    "amount": 1000.00,
    "bank_code": "001",
    "agency": "1234",
    "account": "56789-0"
  }'
```

### Financeiro (V2)

```bash
# Consultar saldo
kobana financial accounts balances \
  --params '{"financial_account_uid": "UID"}' \
  --fields "uid,balance,available_balance"

# Extrato
kobana financial accounts statement \
  --params '{"financial_account_uid": "UID"}' \
  --page-all

# Listar provedores financeiros
kobana financial providers list
```

### Administracao (V2)

```bash
# Listar subcontas
kobana admin subaccounts list --fields "uid,name,status"

# Ver informacoes da conta
kobana me get
```

---

## Help Contextual

```bash
kobana --help                          # ajuda geral
kobana charge --help                   # servicos de cobranca
kobana charge pix --help               # operacoes de Pix cobranca
kobana charge pix create --help        # parametros de criacao
```

Toda saida de `--help` inclui exemplos de uso e campos obrigatorios derivados do OpenAPI spec.
