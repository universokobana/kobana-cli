---
name: kobana-banking-financial
description: "Kobana Financeiro: Contas, saldos, extratos, comandos, provedores."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana banking financial --help"
---

# financial — Financeiro

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana banking financial <resource> <method> [flags]
```

## API Resources

### accounts
- `list` — Listar Contas
- `create` — Criar uma Conta
- `get` — Visualizar uma Conta. Params: `uid`
- `update` — Atualizar Conta. Params: `uid`
- `delete` — Excluir uma Conta. Params: `uid`

### accounts balances
- `list` — Listar saldos. Params: `financial_account_uid`
- `create` — Criar um saldo. Params: `financial_account_uid`
- `get` — Visualizar um Saldo. Params: `financial_account_uid`, `balance_uid`

### accounts commands
- `get` — Visualizar um Comando. Params: `financial_account_uid`, `uid`
- `list` — Listar Comandos de uma Conta. Params: `financial_account_uid`

### accounts statement-transactions
- `sync` — Sincronizar Extrato. Params: `financial_account_uid`
- `list` — Listar Transações. Params: `financial_account_uid`

### accounts statement-transactions imports
- `list` — Listar Importações de extrato. Params: `financial_account_uid`
- `create` — Importar Extrato. Params: `financial_account_uid`
- `get` — Visualizar Importação de Extrato. Params: `financial_account_uid`, `uid`

### accounts statement-transactions summary
- `get` — Resumir Extrato. Params: `financial_account_uid`

### accounts statement-transactions syncs
- `list` — Listar Sincronizações de Extrato. Params: `financial_account_uid`
- `get` — Consultar Sincronização de Extrato. Params: `financial_account_uid`, `uid`

### companies
- `list` — Listar Empresas
- `create` — Criar uma Empresa
- `get` — Consultar uma Empresa. Params: `uid`
- `update` — Atualizar uma Empresa. Params: `uid`
- `delete` — Excluir uma Empresa. Params: `uid`

### providers
- `list` — Listar Provedores

### statement-categories
- `list` — Listar Categorias de Extrato

## Discovering Commands

```bash
kobana banking financial --help
kobana banking financial accounts --help
kobana schema banking.financial.accounts.balances.list
```
