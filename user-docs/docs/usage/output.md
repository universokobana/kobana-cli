---
id: output
title: Saída
sidebar_label: Saída
sidebar_position: 2
description: JSON por padrão, tabela e CSV quando quiser, escolha de campos e gravação em arquivo.
---

# Saída

A resposta vai para o **stdout**; mensagens, avisos e o `--verbose` vão para o **stderr**. Assim, `kobana … | jq` e `kobana … > arquivo` funcionam sem ruído.

## Formato

```bash
kobana banking bank-billets list                         # JSON (padrão)
kobana banking bank-billets list --output-format table   # tabela
kobana banking bank-billets list --output-format csv     # CSV
```

## Só os campos que importam

`--fields` limita a resposta a uma lista separada por vírgula. É a forma mais barata de ler dados — e de não encher a janela de contexto de um agente:

```bash
kobana banking bank-billets list --fields "id,amount,status,due_at"
```

## Filtrar com `--jq` {#jq}

`--jq` aplica uma expressão [jq](https://jqlang.github.io/jq/manual/) à saída JSON, como o `--jq` do `gh` — sem precisar do `jq` instalado. Cada resultado sai numa linha: um texto sai puro (como `jq -r`), o resto em JSON, indentado só no terminal.

```bash
kobana banking bank-billets list --jq '.[].id'
kobana billing invoices list --page-all --jq 'select(.status == "open") | [.id, .total] | @tsv'
kobana banking auth status --jq .authenticated
```

- Vale para toda saída JSON: respostas da API (inclusive a cada item do `--page-all`), `--dry-run`, `kobana schema`, `auth status` e `kobana update` (que, com `--jq`, só informa).
- Aplica-se depois de `--fields`, e não combina com `--output-format table` ou `csv`.
- A expressão é conferida antes de qualquer requisição: um erro de sintaxe sai com o código `3` sem enviar nada.
- Se a expressão falhar na resposta de uma escrita (`create`, `update`, `delete`, helpers), o CLI avisa no stderr e imprime a resposta sem o filtro, com código `0`: a operação foi feita, e repeti-la não seria seguro.
- No terminal, caracteres de controle de um texto viram `�`, para que um texto vindo da API não mande sequências de escape para a tela.
- `env` e `$ENV` não existem no `--jq`, para que uma expressão — escrita por um agente, por exemplo — não leia os tokens do ambiente; módulos (`import`, `include`) e `input` também não.

## Salvar em arquivo

```bash
kobana banking bank-billets get 12345 --output boleto.json
```

`--output` recusa caminhos com `..`, links simbólicos, pastas e dispositivos.

## Ver a conversa com a API

`--verbose` imprime no stderr a requisição e a resposta, sem o token:

```bash
kobana billing plans list --verbose
```

Para registro estruturado, use `KOBANA_LOG=kobana=debug` (stderr) ou `KOBANA_LOG_FILE=./logs` (arquivos JSON com rotação diária). Veja [Variáveis de ambiente](/reference/environment-variables).
