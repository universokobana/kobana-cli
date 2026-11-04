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
