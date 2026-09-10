---
name: kobana-payment
description: "Kobana Pagamentos: Boletos, Pix, tributos, contas de consumo, DDA, lotes."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana banking payment --help"
---

# payment — Pagamentos

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana banking payment <resource> <method> [flags]
```

## API Resources

### accounts
- `list` — Listar Contas de Pagamento
- `get` — Buscar Conta de Pagamento. Params: `uid`

### bank-billets
- `list` — Listar Pagamentos de Boleto
- `create` — Criar um Pagamento de Boleto
- `get` — Visualizar um Pagamento. Params: `uid`
- `cancel` — Cancelar um Pagamento. Params: `uid`

### batches
- `list` — Listar Lotes de Pagamentos
- `get` — Visualizar um Lote. Params: `uid`
- `approve` — Aprovar um Lote. Params: `uid`
- `reprove` — Reprovar um Lote. Params: `uid`

### pix
- `list` — Listar Pagamentos Pix
- `create` — Criar um Pagamento Pix
- `get` — Visualizar um Pagamento Pix. Params: `uid`
- `cancel` — Cancelar um Pagamento Pix. Params: `uid`

### taxes
- `list` — Listar Pagamentos de Tributo
- `create` — Criar Pagamento de Tributo
- `get` — Exibir Pagamento de Tributo. Params: `uid`
- `cancel` — Cancelar Pagamento de Tributo. Params: `uid`

### utilities
- `list` — Listar Pagamentos de Contas de Consumo
- `create` — Criar Pagamento de Conta de Consumo
- `get` — Visualizar Pagamento. Params: `uid`
- `cancel` — Cancelar Pagamento. Params: `uid`

### dda-accounts
- `list` — Listar Contas DDA
- `create` — Criar Conta DDA
- `get` — Buscar Conta DDA. Params: `uid`
- `update` — Atualizar Conta DDA. Params: `uid`
- `enable` — Habilitar Conta DDA. Params: `uid`
- `disable` — Desabilitar Conta DDA. Params: `uid`

### dda-bank-billets
- `list` — Listar Boletos DDA
- `get` — Buscar Boleto DDA. Params: `uid`
- `delete` — Excluir Boleto DDA. Params: `uid`
- `reject` — Rejeitar Boleto DDA. Params: `uid`
- `release` — Liberar Boleto DDA para Pagamento. Params: `uid`

### bank-billet-batches
- `create` — Criar Lote de Pagamento de Boletos

### pix-batches
- `create` — Criar Lote de Pagamento Pix

### tax-batches
- `create` — Criar Lote de Pagamentos de Tributo

### utility-batches
- `create` — Criar Lote de Pagamento de Contas de Consumo

## Discovering Commands

```bash
kobana banking payment --help
kobana banking payment pix --help
kobana schema banking.payment.pix.create
```
