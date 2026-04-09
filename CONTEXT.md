# Kobana CLI (`kobana`) Context

O CLI `kobana` fornece acesso completo às APIs da Kobana (v1 e v2) a partir do terminal, gerando comandos dinamicamente a partir dos OpenAPI specs embutidos no binário.

## Rules of Engagement for Agents

* **Schema Discovery:** *Se você não conhece a estrutura exata do payload JSON, execute `kobana schema <servico>.<recurso>.<metodo>` primeiro para inspecionar o schema antes de executar.*
* **Context Window Protection:** *APIs financeiras retornam payloads extensos. SEMPRE use field masks ao listar ou buscar recursos adicionando `--fields "id,amount,status"` para evitar sobrecarregar sua context window.*
* **Dry-Run Safety:** *Sempre use a flag `--dry-run` para operações de mutação (create, update, delete, cancel) para validar seu payload JSON antes da execução real.*

## Core Syntax

```bash
kobana <servico> <recurso> <metodo> [flags]
```

Use `--help` para obter ajuda sobre os comandos disponíveis.

```bash
kobana --help
kobana <servico> --help
kobana <servico> <recurso> --help
kobana <servico> <recurso> <metodo> --help
```

### Key Flags

-   `--params '<JSON>'`: Parâmetros de query/URL (e.g., `id`, `uid`, `page`, `per_page`, filtros).
-   `--json '<JSON>'`: Corpo da requisição para POST/PUT/PATCH.
-   `--page-all`: Auto-paginação com saída NDJSON (um JSON por linha).
-   `--fields '<CAMPOS>'`: Limita campos na resposta (crítico para eficiência de context window de IA).
-   `--output <PATH>`: Salva resposta em arquivo.
-   `--dry-run`: Mostra a requisição sem executar.
-   `--sandbox` / `--production`: Seleciona o ambiente.

## Usage Patterns

### 1. Reading Data (GET/LIST)
Sempre use `--fields` para minimizar tokens.

```bash
# Listar boletos (eficiente)
kobana v1 bank-billets list --params '{"status": "opened", "per_page": 25}' --fields "id,amount,status,due_at"

# Ver detalhes de cobrança Pix
kobana charge pix get --params '{"uid": "PIX_UID"}' --fields "uid,amount,status,created_at"

# Consultar saldo
kobana financial accounts balances list --params '{"financial_account_uid": "UID"}' --fields "uid,balance,available_balance"
```

### 2. Writing Data (POST/PUT/PATCH)
Use `--json` para o corpo da requisição.

```bash
# Criar cobrança Pix
kobana charge pix create --json '{"amount": 99.90, "pix_account_uid": "UID"}'

# Criar boleto
kobana v1 bank-billets create --json '{"amount": 150.50, "expire_at": "2026-05-01", "customer_person_name": "Maria", "customer_cnpj_cpf": "012.345.678-90", "bank_billet_account_id": 1}'

# Transferência Pix
kobana transfer pix create --json '{"amount": 500, "pix_key": "email@example.com", "transfer_account_uid": "UID"}'
```

### 3. Pagination (NDJSON)
Use `--page-all` para listar coleções grandes. A saída é Newline Delimited JSON.

```bash
# Stream de todas as cobranças Pix
kobana charge pix list --page-all --fields "uid,amount,status"

# Todos os boletos com limite de páginas
kobana v1 bank-billets list --page-all --page-limit 50
```

### 4. Schema Introspection
Se não souber os parâmetros ou estrutura do body, consulte o schema:

```bash
kobana schema charge.pix.create
kobana schema v1.bank-billets.list
kobana schema financial.accounts.balances.list
```
