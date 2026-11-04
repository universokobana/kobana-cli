---
id: helpers
title: Helpers
sidebar_label: Helpers
sidebar_position: 6
description: Atajos +verbo que arman la solicitud a partir de flags con nombre.
---

# Helpers

Los helpers son comandos que empiezan con `+`. Resuelven flujos que los comandos dinámicos no resuelven por sí solos: una interfaz simplificada, una operación en lote o la composición de varias llamadas. Cada helper pertenece a un producto y vive dentro de él.

Hoy todos los helpers son del **Gateway Bancario** (`banking`):

| Helper | Qué hace |
|---|---|
| `kobana banking +emitir` | Emite un boleto con flags con nombre |
| `kobana banking +cobrar` | Crea un cobro Pix con flags con nombre |
| `kobana banking +cancelar-lote` | Cancela varios boletos a la vez |

```bash
# Emitir boleto
kobana banking +emitir --valor 150.50 --vencimento 2026-12-01 \
  --nome "Maria Silva" --cpf-cnpj "012.345.678-90" --carteira 1

# Cobro Pix
kobana banking +cobrar --valor 99.90 --vencimento 2026-12-01T23:59:59-03:00 \
  --conta-pix "UID" --nome "João" --cpf-cnpj "012.345.678-90"

# Cancelar en lote
kobana banking +cancelar-lote --ids "123,456,789"
```

Los helpers respetan las [opciones globales](/manual/commands/global-options): `--dry-run`, `--fields`, `--output-format`, `--output`, `--verbose` y `--idempotency-key` funcionan como en cualquier comando. Una opción global que el helper no usa se rechaza, nunca se ignora en silencio.

Todas las flags de cada helper están en el [Manual](/manual/banking/helpers).
