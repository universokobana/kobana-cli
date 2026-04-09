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
    cliHelp: "kobana transfer --help"
---

# transfer — Transferências

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana transfer <resource> <method> [flags]
```

## API Resources

### accounts
- `list` — Listar Contas de Transferência
- `get` — Buscar Conta de Transferência. Params: `uid`

### pix
- `list` — Listar Transferências Pix
- `create` — Criar uma Transferência Pix
- `get` — Visualizar uma Transferência Pix. Params: `uid`
- `cancel` — Cancelar uma Transferência Pix. Params: `uid`

### ted
- `list` — Listar Transferências TED
- `create` — Criar uma Transferência TED
- `get` — Visualizar uma Transferência TED. Params: `uid`
- `cancel` — Cancelar uma Transferência TED. Params: `uid`

### internal
- `list` — Listar Transferências Entre Contas
- `create` — Criar uma Transferência Entre Contas
- `get` — Visualizar uma Transferência. Params: `uid`
- `cancel` — Cancelar uma Transferência. Params: `uid`

### batches
- `list` — Listar Lotes de Transferência
- `get` — Visualizar um Lote. Params: `uid`
- `approve` — Aprovar um Lote. Params: `uid`
- `reprove` — Reprovar um Lote. Params: `uid`

### pix-batches
- `create` — Criar Lote de Transferência Pix

### ted-batches
- `create` — Criar Lote de Transferência TED

### internal-batches
- `create` — Criar Lote de Transferência Entre Contas

## Discovering Commands

```bash
kobana transfer --help
kobana transfer pix --help
kobana schema transfer.pix.create
```
