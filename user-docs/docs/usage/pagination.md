---
id: pagination
title: Paginação
sidebar_label: Paginação
sidebar_position: 3
description: Percorra todas as páginas de uma listagem com --page-all e receba NDJSON.
---

# Paginação

Uma listagem devolve uma página por chamada. Com `--page-all` o CLI percorre as páginas e imprime **um registro por linha**, em [NDJSON](https://github.com/ndjson/ndjson-spec):

```bash
kobana billing invoices list --page-all --fields "id,status,total"
```

Cada linha é um objeto JSON completo, então a saída pode ser processada enquanto chega:

```bash
kobana kadu executions list --page-all | jq -r 'select(.status == "failed") | .id'
```

## Como ele pagina

- Se o endpoint declara um parâmetro `cursor`, o CLI segue `meta.next_cursor`.
- Caso contrário, usa `page` e `per_page`.

## Limites

| Flag | Padrão | Para quê |
|---|---|---|
| `--page-limit <N>` | `10` | Máximo de páginas buscadas |
| `--page-delay <MS>` | `100` | Intervalo entre páginas, em milissegundos |

Aumente o `--page-limit` de propósito: ele existe para que uma listagem enorme não vire milhares de chamadas sem você perceber.

```bash
kobana banking charge pix list --page-all --page-limit 50 --page-delay 250
```
