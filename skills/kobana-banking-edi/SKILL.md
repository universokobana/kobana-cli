---
name: kobana-banking-edi
description: "Kobana EDI: Caixas EDI para troca de arquivos com os bancos."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana banking edi --help"
---

# edi — Caixas EDI

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana banking edi <resource> <method> [flags]
```

## API Resources

### edi-boxes
- `list` — Listar Caixas Postais
- `create` — Criar Caixa Postal
- `get` — Visualizar Caixa Postal. Params: `uid`
- `update` — Atualizar Caixa Postal. Params: `uid`

## Discovering Commands

```bash
kobana banking edi --help
kobana banking edi edi-boxes --help
kobana schema banking.edi.edi-boxes.create
```
