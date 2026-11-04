---
id: helpers
title: Helpers
sidebar_label: Helpers
sidebar_position: 6
description: Atalhos +verbo que montam a requisição a partir de flags nomeadas.
---

# Helpers

Helpers são comandos que começam com `+`. Eles resolvem fluxos que os comandos dinâmicos não resolvem sozinhos — uma interface simplificada, uma operação em lote ou a composição de várias chamadas. Cada helper pertence a um produto e fica dentro dele.

Hoje todos os helpers são do **Gateway Bancário** (`banking`):

| Helper | O que faz |
|---|---|
| `kobana banking +emitir` | Emite um boleto com flags nomeadas |
| `kobana banking +cobrar` | Cria uma cobrança Pix com flags nomeadas |
| `kobana banking +cancelar-lote` | Cancela vários boletos de uma vez |

```bash
# Emitir boleto
kobana banking +emitir --valor 150.50 --vencimento 2026-12-01 \
  --nome "Maria Silva" --cpf-cnpj "012.345.678-90" --carteira 1

# Cobrança Pix
kobana banking +cobrar --valor 99.90 --vencimento 2026-12-01T23:59:59-03:00 \
  --conta-pix "UID" --nome "João" --cpf-cnpj "012.345.678-90"

# Cancelar em lote
kobana banking +cancelar-lote --ids "123,456,789"
```

Os helpers respeitam as [opções globais](/manual/commands/global-options): `--dry-run`, `--fields`, `--output-format`, `--output`, `--verbose` e `--idempotency-key` funcionam como em qualquer comando. Uma opção global que o helper não usa é recusada, nunca ignorada em silêncio.

Todas as flags de cada helper estão no [Manual](/manual/banking/helpers).
