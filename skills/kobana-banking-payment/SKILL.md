---
name: kobana-banking-payment
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
- `create` — Criar Conta de Pagamento
- `get` — Buscar Conta de Pagamento. Params: `uid`
- `update` — Atualizar Conta de Pagamento. Params: `uid`

### bank-billet-batches
- `create` — Criar um Lote de Pagamento de Boletos

### bank-billets
- `list` — Listar Pagamentos de Boleto
- `create` — Criar um Pagamento de Boleto
- `get` — Visualizar um Pagamento de Boleto. Params: `uid`
- `cancel` — Cancelar um Pagamento de Boleto. Params: `uid`

### batches
- `list` — Listar Lotes de Pagamentos
- `get` — Visualizar um Lote de Pagamento. Params: `uid`
- `approve` — Aprovar um Lote de Pagamento. Params: `uid`
- `cancel` — Cancelar um Lote de Pagamento. Params: `uid`
- `reprove` — Reprovar um Lote de Pagamento. Params: `uid`

### dda-accounts
- `list` — Listar Contas de DDA
- `create` — Criar Conta de DDA
- `get` — Buscar Conta de DDA. Params: `uid`
- `update` — Atualizar Conta de DDA. Params: `uid`
- `disable` — Desabilitar Conta de DDA. Params: `uid`
- `enable` — Habilitar Conta de DDA. Params: `uid`

### dda-bank-billets
- `list` — Listar Boletos DDA
- `get` — Buscar Boleto DDA. Params: `uid`
- `delete` — Excluir Boleto DDA. Params: `uid`
- `reject` — Rejeitar Boleto DDA. Params: `uid`
- `release` — Liberar Boleto DDA para Pagamento. Params: `uid`

### pix
- `list` — Listar Pagamentos Pix
- `create` — Criar um Pagamento Pix
- `get` — Visualizar um Pagamento Pix. Params: `uid`
- `cancel` — Cancelar um Pagamento Pix. Params: `uid`

### pix-batches
- `create` — Criar um Lote de Pagamento Pix

### tax-batches
- `create` — Criar Lote de Pagamentos de Tributo

### taxes
- `list` — Listar Pagamentos de Tributo
- `create` — Criar Pagamento de Tributo
- `get` — Exibir Pagamento de Tributo. Params: `uid`
- `cancel` — Cancelar um Pagamento de Tributo. Params: `uid`

### utilities
- `list` — Listar Pagamentos de Contas de Consumo
- `create` — Criar um Pagamento de Conta de Consumo
- `get` — Visualizar um Pagamento de Conta de Consumo. Params: `uid`
- `cancel` — Cancelar um Pagamento de Conta de Consumo. Params: `uid`

### utility-batches
- `create` — Criar um Lote de Pagamento de Contas de Consumo

## Pagamentos de qualquer tipo: `kobana banking payments <method>`

- `list` — Listar Pagamentos
- `create` — Criar um Pagamento
- `barcode` — Criar um Pagamento por código de barras
- `get` — Visualizar um Pagamento. Params: `uid`
- `approve` — Liberar (aprovar) um Pagamento. Params: `uid`
- `cancel` — Cancelar um Pagamento. Params: `uid`

## Discovering Commands

```bash
kobana banking payment --help
kobana banking payment pix --help
kobana schema banking.payment.pix.create
```
