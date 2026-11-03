# Kobana CLI — Especificacao de Interface

## Visao Geral

O `kobana` CLI fornece acesso a toda a API da Kobana (v1 e v2) a partir do terminal. Projetado para humanos e agentes de IA, com saida JSON estruturada, introspecao de schema, dry-run e paginacao automatica.

Escrito em **Rust** (clap + serde + reqwest + tokio). Inspirado no [gws-cli](https://github.com/googleworkspace/cli) e nos principios de [CLI design for AI agents](https://justin.poehnelt.com/posts/rewrite-your-cli-for-ai-agents/).

---

## Sintaxe Principal

```bash
kobana <product> <resource> <method> [flags]
```

Onde:
- `<product>` — produto Kobana: `banking`, `finance`, `billing`, `inbox`, `dunning`, `dashboards` ou `kadu`
- `<resource>` — recurso, podendo ser aninhado (ex: `bank-billets`, `charge pix`,
  `webhooks deliveries`). A versao da API nao entra no comando
- `<method>` — acao (ex: `list`, `get`, `create`, `update`, `delete`, `cancel`)

### Exemplos

```bash
# Listar boletos
kobana banking bank-billets list --params '{"page": 1, "per_page": 50}'

# Criar cobranca Pix
kobana banking charge pix create --json '{"amount": 1500, "payer": {...}}'

# Ver detalhes de uma transferencia
kobana banking transfer pix get --params '{"uid": "019d6b00-4751-719d-8a6f-20cb9223bea4"}'

# Consultar saldos
kobana banking financial accounts balances list --params '{"financial_account_uid": "UID"}'
```

---

## Mapeamento API -> CLI (banking)

Gerado a partir da arvore de comandos do binario. A fonte de verdade e o proprio CLI: `kobana schema --list` lista os metodos de todos os produtos, `kobana <produto> <recurso> --help` mostra os de um recurso e `kobana schema <produto>.<recurso>.<metodo>` o schema de cada um. Os outros seis produtos (`finance`, `billing`, `inbox`, `dunning`, `dashboards`, `kadu`) seguem as mesmas regras de nome.

Parametros de caminho (`{id}`, `{uid}`, `{pix_uid}`…) vao em `--params`. Um segmento depois de um parametro vira acao do recurso (`/v2/charge/pix/{uid}/cancel` → `charge pix cancel`).

### V1 — API Legada (Boletos)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `bank-billet-accounts` | `list`, `create`, `get`, `update`, `delete`, `ask`, `set-default`, `validate` | `/v1/bank_billet_accounts` |
| `bank-billet-batch-exports` | `get` | `/v1/bank_billet_batch_exports` |
| `bank-billet-batches` | `list`, `create`, `get`, `delete`, `add-bank-billets`, `pdf`, `remove-bank-billet`, `zip` | `/v1/bank_billet_batches` |
| `bank-billet-discharges` | `list`, `get` | `/v1/bank_billet_discharges` |
| `bank-billet-payments` | `list`, `create`, `get`, `delete` | `/v1/bank_billet_payments` |
| `bank-billet-registrations` | `list`, `get` | `/v1/bank_billet_registrations` |
| `bank-billet-remittances` | `list`, `get`, `delete` | `/v1/bank_billet_remittances` |
| `bank-billet-remittances occurrence` | `create` | `/v1/bank_billet_remittances/{id}/occurrence` |
| `bank-billet-remittances pending` | `list` | `/v1/bank_billet_remittances/pending` |
| `bank-billets` | `list`, `create`, `cancel-all`, `get`, `update`, `cancel`, `duplicate`, `pay`, `send-email`, `send-sms` | `/v1/bank_billets` |
| `bank-billets protest` | `update` | `/v1/bank_billets/{id}/protest` |
| `customer-subscriptions` | `list`, `create`, `get`, `update`, `delete`, `next-charge` | `/v1/customer_subscriptions` |
| `customers` | `list`, `create`, `get`, `update` | `/v1/customers` |
| `customers cnpj-cpf` | `list` | `/v1/customers/cnpj_cpf` |
| `customers email` | `list` | `/v1/customers/email` |
| `customers people-sync` | `update` | `/v1/customers/people_sync` |
| `discharges` | `list`, `create`, `get`, `download`, `pay-off`, `reprocess` | `/v1/discharges` |
| `email-deliveries` | `list`, `get`, `resend` | `/v1/email_deliveries` |
| `events` | `list`, `get` | `/v1/events` |
| `imports` | `list`, `create`, `get` | `/v1/imports` |
| `installments` | `list`, `create`, `get`, `delete` | `/v1/installments` |
| `remittances` | `list`, `create`, `bulk`, `get`, `delete`, `raw` | `/v1/remittances` |
| `reports bank-billets` | `list` | `/v1/reports/bank_billets` |
| `sms-deliveries` | `list`, `get`, `resend` | `/v1/sms_deliveries` |
| `userinfo` | `list` | `/v1/userinfo` |
| `webhook-deliveries` | `list`, `get` | `/v1/webhook_deliveries` |
| `webhook-deliveries resend` | `update` | `/v1/webhook_deliveries/resend` |
| `webhooks` | `list`, `create`, `get`, `update`, `delete` | `/v1/webhooks` |

### V2 — API Modular

#### Admin

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `admin certificates` | `list`, `create`, `delete` | `/v2/admin/certificates` |
| `admin commands` | `list`, `get` | `/v2/admin/commands` |
| `admin connections` | `list`, `create`, `get`, `update`, `delete` | `/v2/admin/connections` |
| `admin connections associations` | `create`, `delete` | `/v2/admin/connections/{uid}/associations` |
| `admin subaccounts` | `list`, `create`, `get`, `update` | `/v2/admin/subaccounts` |
| `admin users` | `list`, `create`, `update`, `delete` | `/v2/admin/users` |

#### Charge (Cobrancas)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `charge automatic-pix accounts` | `list`, `create`, `get`, `update`, `delete` | `/v2/charge/automatic_pix/accounts` |
| `charge automatic-pix locations` | `list`, `create`, `get` | `/v2/charge/automatic_pix/locations` |
| `charge automatic-pix pix` | `list`, `get`, `update`, `cancel`, `retry` | `/v2/charge/automatic_pix/pix` |
| `charge automatic-pix recurrences` | `list`, `create`, `pix`, `requests`, `get`, `update`, `cancel` | `/v2/charge/automatic_pix/recurrences` |
| `charge automatic-pix requests` | `list`, `get`, `update`, `cancel` | `/v2/charge/automatic_pix/requests` |
| `charge payments` | `list`, `create`, `get`, `delete` | `/v2/charge/payments` |
| `charge pix` | `list`, `create`, `cancel`, `update`, `get`, `delete` | `/v2/charge/pix` |
| `charge pix commands` | `get`, `list` | `/v2/charge/pix/{pix_uid}/commands` |
| `charge pix-accounts` | `list`, `create`, `get`, `update`, `delete` | `/v2/charge/pix_accounts` |

#### Payment (Pagamentos)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `payment accounts` | `list`, `create`, `get`, `update` | `/v2/payment/accounts` |
| `payment bank-billet-batches` | `create` | `/v2/payment/bank_billet_batches` |
| `payment bank-billets` | `list`, `create`, `get`, `cancel` | `/v2/payment/bank_billets` |
| `payment batches` | `list`, `get`, `approve`, `cancel`, `reprove` | `/v2/payment/batches` |
| `payment dda-accounts` | `list`, `create`, `get`, `update`, `disable`, `enable` | `/v2/payment/dda_accounts` |
| `payment dda-bank-billets` | `list`, `get`, `delete`, `reject`, `release` | `/v2/payment/dda_bank_billets` |
| `payment pix` | `list`, `create`, `get`, `cancel` | `/v2/payment/pix` |
| `payment pix-batches` | `create` | `/v2/payment/pix_batches` |
| `payment tax-batches` | `create` | `/v2/payment/tax_batches` |
| `payment taxes` | `list`, `create`, `get`, `cancel` | `/v2/payment/taxes` |
| `payment utilities` | `list`, `create`, `get`, `cancel` | `/v2/payment/utilities` |
| `payment utility-batches` | `create` | `/v2/payment/utility_batches` |

#### Transfer (Transferencias)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `transfer accounts` | `list`, `create`, `get`, `update` | `/v2/transfer/accounts` |
| `transfer batches` | `list`, `get`, `approve`, `cancel`, `reprove` | `/v2/transfer/batches` |
| `transfer internal` | `list`, `create`, `get`, `approve`, `cancel`, `reschedule` | `/v2/transfer/internal` |
| `transfer internal-batches` | `create` | `/v2/transfer/internal_batches` |
| `transfer pix` | `list`, `create`, `get`, `approve`, `cancel`, `reschedule` | `/v2/transfer/pix` |
| `transfer pix-batches` | `create` | `/v2/transfer/pix_batches` |
| `transfer ted` | `list`, `create`, `get`, `approve`, `cancel`, `reschedule` | `/v2/transfer/ted` |
| `transfer ted-batches` | `create` | `/v2/transfer/ted_batches` |

#### Financial (Financeiro)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `financial accounts` | `list`, `create`, `get`, `update`, `delete` | `/v2/financial/accounts` |
| `financial accounts balances` | `list`, `create`, `get` | `/v2/financial/accounts/{financial_account_uid}/balances` |
| `financial accounts commands` | `get`, `list` | `/v2/financial/accounts/{financial_account_uid}/commands` |
| `financial accounts statement-transactions` | `sync`, `list` | `/v2/financial/accounts/{financial_account_uid}/statement_transactions` |
| `financial accounts statement-transactions imports` | `list`, `create`, `get` | `/v2/financial/accounts/{financial_account_uid}/statement_transactions/imports` |
| `financial accounts statement-transactions summary` | `get` | `/v2/financial/accounts/{financial_account_uid}/statement_transactions/summary` |
| `financial accounts statement-transactions syncs` | `list`, `get` | `/v2/financial/accounts/{financial_account_uid}/statement_transactions/syncs` |
| `financial companies` | `list`, `create`, `get`, `update`, `delete` | `/v2/financial/companies` |
| `financial providers` | `list` | `/v2/financial/providers` |
| `financial statement-categories` | `list` | `/v2/financial/statement_categories` |

#### Mailbox (Caixa Postal)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `mailbox entries` | `list`, `create`, `files`, `get`, `update`, `delete` | `/v2/mailbox/entries` |
| `mailbox entries email` | `get`, `create`, `update`, `delete` | `/v2/mailbox/entries/{entry_uid}/email` |
| `mailbox entries email activate` | `update` | `/v2/mailbox/entries/{entry_uid}/email/activate` |
| `mailbox entries email deactivate` | `update` | `/v2/mailbox/entries/{entry_uid}/email/deactivate` |
| `mailbox entries s3` | `get`, `create`, `delete` | `/v2/mailbox/entries/{entry_uid}/s3` |
| `mailbox entries s3 activate` | `update` | `/v2/mailbox/entries/{entry_uid}/s3/activate` |
| `mailbox entries s3 deactivate` | `update` | `/v2/mailbox/entries/{entry_uid}/s3/deactivate` |
| `mailbox entries s3 fetch` | `update` | `/v2/mailbox/entries/{entry_uid}/s3/fetch` |
| `mailbox entries s3 update-password` | `update` | `/v2/mailbox/entries/{entry_uid}/s3/update_password` |
| `mailbox entries sftp` | `get`, `create`, `update`, `delete` | `/v2/mailbox/entries/{entry_uid}/sftp` |
| `mailbox entries sftp activate` | `update` | `/v2/mailbox/entries/{entry_uid}/sftp/activate` |
| `mailbox entries sftp deactivate` | `update` | `/v2/mailbox/entries/{entry_uid}/sftp/deactivate` |
| `mailbox entries sftp fetch` | `update` | `/v2/mailbox/entries/{entry_uid}/sftp/fetch` |
| `mailbox entries sftp update-password` | `update` | `/v2/mailbox/entries/{entry_uid}/sftp/update_password` |
| `mailbox entries syncthing` | `get`, `create`, `update`, `delete` | `/v2/mailbox/entries/{entry_uid}/syncthing` |
| `mailbox entries syncthing activate` | `update` | `/v2/mailbox/entries/{entry_uid}/syncthing/activate` |
| `mailbox entries syncthing deactivate` | `update` | `/v2/mailbox/entries/{entry_uid}/syncthing/deactivate` |
| `mailbox entries syncthing resend-invites` | `update` | `/v2/mailbox/entries/{entry_uid}/syncthing/resend_invites` |
| `mailbox entries syncthing update-status` | `update` | `/v2/mailbox/entries/{entry_uid}/syncthing/update_status` |
| `mailbox entries whatsapp` | `get`, `create`, `update`, `delete` | `/v2/mailbox/entries/{entry_uid}/whatsapp` |
| `mailbox entries whatsapp activate` | `update` | `/v2/mailbox/entries/{entry_uid}/whatsapp/activate` |
| `mailbox entries whatsapp deactivate` | `update` | `/v2/mailbox/entries/{entry_uid}/whatsapp/deactivate` |
| `mailbox files` | `list`, `get`, `update`, `delete` | `/v2/mailbox/files` |

#### EDI

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `edi edi-boxes` | `list`, `create`, `get`, `update` | `/v2/edi/edi_boxes` |

#### Data (Consultas)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `data bank-billet-queries` | `list`, `create` | `/v2/data/bank_billet_queries` |
| `data people` | `list`, `create`, `get`, `delete` | `/v2/data/people` |
| `data pix-qrcode-queries` | `list`, `create` | `/v2/data/pix_qrcode_queries` |

#### Security (Seguranca)

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `security access-tokens` | `create`, `update`, `delete`, `disable`, `enable`, `renew`, `revoke` | `/v2/security/access_tokens` |

#### Outros

| Recurso | Metodos | Path da API |
|---------|---------|-------------|
| `me` | `get` | `/v2/me` |
| `payments` | `list`, `create`, `barcode`, `get`, `approve`, `cancel` | `/v2/payments` |
| `transfers` | `list`, `create`, `get`, `cancel` | `/v2/transfers` |

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
| `--env <ENV>` | Ambiente: `production` (default), `sandbox`, `development` | `--env sandbox` |
| `--version`, `-V` | Versao do CLI | |
| `--verbose` | Mostra headers e detalhes da requisicao no stderr | |
| `--help`, `-h` | Ajuda contextual | |
| `--output-format` | Formato de saida: `json` (default), `table`, `csv` | `--output-format table` |
| `--idempotency-key <KEY>` | Chave de idempotencia propria (POST/PUT/PATCH; sem ela, o POST gera uma) | `--idempotency-key pedido-123` |
| `--file <CAMPO=CAMINHO>` | Arquivo enviado como parte de um formulario multipart (repetivel) | `--file file=./mensagem.eml` |
| `--lang <IDIOMA>` | Idioma das mensagens: `pt`, `en` ou `es` | `--lang en` |

---

## Autenticacao

### Metodos Suportados

Cada produto tem o proprio token, emitido pela Conta Kobana (`account.kobana.com.br`) com a API do produto como `aud`.

| Prioridade | Metodo | Configuracao |
|------------|--------|-------------|
| 1 | Token do produto (env var) | `KOBANA_<PRODUTO>_TOKEN` (kadu tambem le `KADU_TOKEN`) |
| 2 | Credenciais OAuth (arquivo) | `KOBANA_CREDENTIALS_FILE` |
| 3 | Credenciais salvas (criptografadas, por produto) | `kobana <produto> auth login` |

### Comandos de Auth

```bash
# Login interativo — mostra os escopos publicados pelo produto e os niveis de acesso
kobana billing auth login

# Client credentials (so banking)
kobana banking auth login --client-id <ID> --client-secret <SECRET>

kobana billing auth status
kobana billing auth export > credentials.json
kobana billing auth logout
```

### OAuth Flows

1. **Authorization Code + PKCE** — o CLI abre `account.<env>.kobana.com.br/oauth/authorize` com `resource=<API do produto>`, os escopos `<produto>/<recurso>` lidos de `/.well-known/oauth-protected-resource` (host MCP do produto; fallback: catalogo do AS) e os niveis `read`, `write`, `delete`, `import`, `export`. Recebe o callback em localhost e renova o token com o refresh token quando vence.

2. **Client Credentials Flow** — so banking: troca `client_id` + `client_secret` por um access token de aplicacao.

3. **Token por env var** — `KOBANA_<PRODUTO>_TOKEN`.

### Armazenamento Seguro

Credenciais sao criptografadas em repouso (AES-256-GCM) com chave armazenada no keyring do OS (ou arquivo fallback para CI/Docker).

```
~/.config/kobana/credentials-<produto>-<ambiente>.enc    # credenciais criptografadas, uma por produto e ambiente
~/.config/kobana/.key               # chave, so quando o keyring do OS nao esta disponivel
~/.config/kobana/.env               # variaveis de ambiente (opcional)
```

A pasta e a de configuracao do sistema (`~/.config/kobana` no Linux, `~/Library/Application Support/kobana` no macOS) ou `KOBANA_CONFIG_DIR`.

---

## Introspecao de Schema

```bash
# Ver schema de um endpoint
kobana schema banking.charge.pix.create
kobana schema banking.bank-billets.list

# Ver todos os produtos, recursos e metodos disponiveis
kobana schema --list
```

Retorna JSON com: parametros, corpo da requisicao, tipo de resposta, campos obrigatorios e opcionais. Alimentado pelo OpenAPI spec embutido.

---

## Paginacao

```bash
# Paginar manualmente
kobana banking charge pix list --params '{"page": 1, "per_page": 50}'

# Auto-paginacao (NDJSON — um JSON por linha)
kobana banking charge pix list --page-all

# Limitar paginas
kobana banking charge pix list --page-all --page-limit 5
```

---

## Variaveis de Ambiente

| Variavel | Descricao |
|----------|-----------|
| `KOBANA_<PRODUTO>_TOKEN` | Token Bearer de um produto (maior prioridade) |
| `KOBANA_CREDENTIALS_FILE` | Caminho para arquivo JSON de credenciais OAuth (exige o campo `product`; vale só para esse produto e para o `environment` dele) |
| `KOBANA_CLIENT_ID` | OAuth client ID |
| `KOBANA_CLIENT_SECRET` | OAuth client secret: escolhe o fluxo Client Credentials (só no `banking`, com client ID) |
| `KOBANA_ACCOUNT_URL` | Substitui a URL da Conta Kobana (servidor de autorizacao) |
| `KOBANA_WORKSPACE_ID` | Workspace enviado como `X-Workspace-Id` por `finance` (obrigatorio), `dashboards` e `dunning` (opcionais). Um valor em `--params` vence |
| `KOBANA_CONFIG_DIR` | Diretorio de configuracao (default: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `production` (default), `sandbox` ou `development`; outro valor e erro |
| `KOBANA_LANG` | `pt`, `en` ou `es` (default: `LC_ALL`/`LC_MESSAGES`/`LANG`, depois o idioma do sistema). `--lang` vence |
| `KOBANA_LOG` | Nivel de log para stderr (ex: `kobana=debug`) |
| `KOBANA_LOG_FILE` | Diretorio para logs JSON com rotacao diaria |

Variaveis tambem podem ser definidas em arquivo `.env`: o do diretorio atual (nao o das pastas acima) e o da pasta de configuracao. O `.env` do diretorio atual nao pode definir `KOBANA_ACCOUNT_URL` nem `KOBANA_CONFIG_DIR`.

---

## Codigos de Saida

| Codigo | Significado | Causa |
|--------|-------------|-------|
| `0` | Sucesso | Comando completou normalmente |
| `1` | Erro de API | Kobana retornou 4xx/5xx |
| `2` | Erro de auth | Credenciais ausentes, expiradas ou invalidas |
| `3` | Erro de validacao | Argumentos invalidos, comando ou opcao desconhecida, recurso desconhecido |
| `4` | Erro de schema | Nao conseguiu carregar spec OpenAPI |
| `5` | Erro interno | Falha inesperada |

---

## Exemplos de Uso Completos

### Boletos (V1)

```bash
# Listar boletos com filtro
kobana banking bank-billets list \
  --params '{"status": "opened", "per_page": 25}' \
  --fields "id,amount,status,due_at,customer_person_name"

# Criar boleto
kobana banking bank-billets create \
  --json '{
    "amount": 150.50,
    "expire_at": "2026-05-01",
    "customer_person_name": "Maria Silva",
    "customer_cnpj_cpf": "012.345.678-90",
    "bank_billet_account_id": 1
  }' \
  --dry-run

# Cancelar boleto
kobana banking bank-billets cancel --params '{"id": 12345}'

# Enviar boleto por email
kobana banking bank-billets send-email --params '{"id": 12345}'
```

### Cobrancas Pix (V2)

```bash
# Criar cobranca Pix
kobana banking charge pix create \
  --json '{
    "amount": 99.90,
    "pix_account_uid": "UID_HERE",
    "expire_at": "2026-05-01T23:59:59-03:00",
    "payer": {"name": "Joao", "document_number": "012.345.678-90"}
  }'

# Listar cobrancas com paginacao automatica
kobana banking charge pix list --page-all --fields "uid,amount,status,created_at"

# Cancelar cobranca
kobana banking charge pix cancel --params '{"uid": "PIX_UID"}'
```

### Pagamentos (V2)

```bash
# Criar pagamento de boleto
kobana banking payment bank-billets create \
  --json '{"barcode": "23793.38128 ...", "amount": 150.00}' \
  --dry-run

# Aprovar lote de pagamentos
kobana banking payment batches approve --params '{"uid": "BATCH_UID"}'

# Listar pagamentos Pix
kobana banking payment pix list --params '{"per_page": 20}'
```

### Transferencias (V2)

```bash
# Transferencia Pix
kobana banking transfer pix create \
  --json '{
    "amount": 500.00,
    "pix_key": "email@example.com",
    "transfer_account_uid": "UID"
  }'

# TED
kobana banking transfer ted create \
  --json '{
    "amount": 1000.00,
    "bank_code": "001",
    "agency": "1234",
    "account": "56789-0"
  }'
```

### Financeiro (V2)

```bash
# Consultar saldos
kobana banking financial accounts balances list \
  --params '{"financial_account_uid": "UID"}' \
  --fields "uid,balance,available_balance"

# Extrato
kobana banking financial accounts statement-transactions list \
  --params '{"financial_account_uid": "UID"}' \
  --page-all

# Listar provedores financeiros
kobana banking financial providers list
```

### Administracao (V2)

```bash
# Listar subcontas
kobana banking admin subaccounts list --fields "uid,name,status"

# Ver informacoes da conta
kobana banking me get
```

---

## Help Contextual

```bash
kobana --help                          # ajuda geral
kobana banking charge --help                   # recursos de cobranca
kobana banking charge pix --help               # operacoes de Pix cobranca
kobana banking charge pix create --help        # parametros de criacao
```

O `--help` de um metodo mostra o resumo e a descricao da operacao no OpenAPI spec e as opcoes. O corpo esperado, com os campos obrigatorios, os parametros e as respostas, sai de `kobana schema <produto>.<recurso>.<metodo>`.
