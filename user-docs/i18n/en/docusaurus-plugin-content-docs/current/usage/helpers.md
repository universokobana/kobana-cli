---
id: helpers
title: Helpers
sidebar_label: Helpers
sidebar_position: 6
description: +verb shortcuts that build the request from named flags.
---

# Helpers

Helpers are commands that start with `+`. They solve flows that the dynamic commands do not solve on their own — a simplified interface, a batch operation or the composition of several calls. Each helper belongs to a product and lives inside it.

Today all helpers belong to **Banking Gateway** (`banking`):

| Helper | What it does |
|---|---|
| `kobana banking +emitir` | Issues a bank slip (boleto) with named flags |
| `kobana banking +cobrar` | Creates a Pix charge with named flags |
| `kobana banking +cancelar-lote` | Cancels several bank slips at once |

```bash
# Issue a bank slip
kobana banking +emitir --valor 150.50 --vencimento 2026-12-01 \
  --nome "Maria Silva" --cpf-cnpj "012.345.678-90" --carteira 1

# Pix charge
kobana banking +cobrar --valor 99.90 --vencimento 2026-12-01T23:59:59-03:00 \
  --conta-pix "UID" --nome "João" --cpf-cnpj "012.345.678-90"

# Cancel in batch
kobana banking +cancelar-lote --ids "123,456,789"
```

Helpers respect the [global options](/manual/commands/global-options): `--dry-run`, `--fields`, `--output-format`, `--output`, `--verbose` and `--idempotency-key` work as in any command. A global option that the helper does not use is rejected, never silently ignored.

All the flags for each helper are in the [Manual](/manual/banking/helpers).
