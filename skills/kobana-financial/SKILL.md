---
name: kobana-financial
description: "Kobana Financeiro: Contas, saldos, extratos, comandos, provedores."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana financial --help"
---

# financial — Financeiro

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana financial <resource> <method> [flags]
```

## API Resources

### accounts
- `list` — Listar Contas
- `create` — Criar uma Conta
- `get` — Visualizar uma Conta. Params: `id`
- `update` — Atualizar Conta. Params: `id`
- `delete` — Excluir uma Conta. Params: `uid`

### accounts balances
- `list` — Listar Saldos. Params: `financial_account_uid`
- `create` — Criar um Saldo. Params: `financial_account_uid`
- `get` — Visualizar um Saldo. Params: `financial_account_uid`, `balance_uid`

### accounts commands
- `list` — Listar Comandos de uma Conta. Params: `financial_account_uid`
- `get` — Visualizar um Comando. Params: `financial_account_uid`, `id`

### accounts statement
- `list` — Listar Transações do Extrato. Params: `financial_account_uid`
- `sync` — Sincronizar Extrato. Params: `financial_account_uid`

### accounts statement imports
- `list` — Listar Importações de Extrato. Params: `financial_account_uid`
- `create` — Importar Extrato. Params: `financial_account_uid`
- `get` — Visualizar Importação. Params: `financial_account_uid`, `uid`

### providers
- `list` — Listar Provedores Financeiros

## Discovering Commands

```bash
kobana financial --help
kobana financial accounts --help
kobana schema financial.accounts.balances.list
```
