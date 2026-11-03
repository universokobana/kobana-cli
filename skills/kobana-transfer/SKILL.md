---
name: kobana-transfer
description: "Kobana Transferências: Pix, TED, entre contas, lotes."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana banking transfer --help"
---

# transfer — Transferências

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana banking transfer <resource> <method> [flags]
```

## API Resources

### accounts
- `list` — Listar Contas de Transferência
- `create` — Criar Conta de Transferência
- `get` — Buscar Conta de Transferência. Params: `uid`
- `update` — Atualizar Conta de Transferência. Params: `uid`

### batches
- `list` — Listar Lotes de Transferência
- `get` — Visualizar um Lote de Transferência. Params: `uid`
- `approve` — Aprovar um Lote de Transferência. Params: `uid`
- `cancel` — Cancelar um Lote de Transferência. Params: `uid`
- `reprove` — Reprovar um Lote de Transferência. Params: `uid`

### internal
- `list` — Listar Transferências Entre Contas
- `create` — Criar uma Transferência Entre Contas
- `get` — Visualizar uma Transferência Entre Contas. Params: `uid`
- `approve` — Liberar (aprovar) uma Transferência interna. Params: `uid`
- `cancel` — Cancelar uma Transferência Entre Contas. Params: `uid`
- `reschedule` — Reagendar (mudar a data) uma Transferência interna. Params: `uid`

### internal-batches
- `create` — Criar um Lote de Transferência Entre Contas

### pix
- `list` — Listar Transferências Pix
- `create` — Criar uma Transferência Pix
- `get` — Visualizar uma Transferência Pix. Params: `uid`
- `approve` — Liberar (aprovar) uma Transferência Pix. Params: `uid`
- `cancel` — Cancelar uma Transferência Pix. Params: `uid`
- `reschedule` — Reagendar (mudar a data) uma Transferência Pix. Params: `uid`

### pix-batches
- `create` — Criar um Lote de Transferência Pix

### ted
- `list` — Listar Transferências TED
- `create` — Criar uma Transferência TED
- `get` — Visualizar uma Transferência TED. Params: `uid`
- `approve` — Liberar (aprovar) uma Transferência TED. Params: `uid`
- `cancel` — Cancelar uma Transferência TED. Params: `uid`
- `reschedule` — Reagendar (mudar a data) uma Transferência TED. Params: `uid`

### ted-batches
- `create` — Criar um Lote de Transferência TED

## Transferências de qualquer tipo: `kobana banking transfers <method>`

- `list` — Listar Transferências
- `create` — Criar uma Transferência
- `get` — Visualizar uma Transferência. Params: `uid`
- `cancel` — Cancelar uma Transferência. Params: `uid`

## Discovering Commands

```bash
kobana banking transfer --help
kobana banking transfer pix --help
kobana schema banking.transfer.pix.create
```
