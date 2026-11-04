---
id: exit-codes
title: Códigos de saída
sidebar_label: Códigos de saída
sidebar_position: 3
description: O que cada código de saída do kobana significa, para tratar erros em scripts.
---

# Códigos de saída

| Código | Significado |
|---|---|
| `0` | Sucesso |
| `1` | Erro de API (resposta 4xx ou 5xx) |
| `2` | Erro de autenticação |
| `3` | Erro de validação, inclusive linha de comando inválida (comando ou flag desconhecidos, argumento faltando) |
| `4` | Erro de schema |
| `5` | Erro interno |

Erros de API, de autenticação e de validação saem como um objeto JSON `{"error": {"code": …, "message": …}}`. Uma linha de comando que não faz sentido (comando ou flag desconhecidos) sai em texto, no stderr, com o código `3`.

```bash
if ! kobana billing invoices get "$ID" > fatura.json; then
  case $? in
    1) echo "A API recusou o pedido" ;;
    2) echo "Entre de novo: kobana billing auth login" ;;
    3) echo "Comando ou argumento inválido" ;;
  esac
fi
```
