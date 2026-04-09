---
name: kobana-data
description: "Kobana Consultas: Consultas de boletos e QR codes Pix."
metadata:
  version: 0.1.0
  openclaw:
    category: "finance"
    requires:
      bins:
        - kobana
    cliHelp: "kobana data --help"
---

# data — Consultas

> **PREREQUISITE:** Read `../kobana-shared/SKILL.md` for auth, global flags, and security rules.

```bash
kobana data <resource> <method> [flags]
```

## API Resources

### bank-billet-queries
- `list` — Listar Consultas de Boletos
- `create` — Criar uma Consulta de Boleto

### pix-qrcode-queries
- `list` — Listar Consultas de Pix QR Code
- `create` — Criar uma Consulta de Pix QR Code

## Discovering Commands

```bash
kobana data --help
kobana schema data.bank-billet-queries.create
```
