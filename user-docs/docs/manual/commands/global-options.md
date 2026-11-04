---
id: global-options
title: "Opções globais"
sidebar_label: "Opções globais"
sidebar_position: 1
---

# Opções globais

Valem para todos os comandos, antes ou depois do nome do recurso.

| Opções | Descrição |
|---|---|
| `--params <JSON>` | Parâmetros de URL e de consulta, em JSON |
| `--json <JSON>` | Corpo da requisição, em JSON |
| `--file <CAMPO=CAMINHO>` | Arquivo enviado como parte de um formulário multipart, campo=caminho (repetível; os campos de --json viram partes de texto) |
| `--fields <CAMPOS>` | Campos a incluir na resposta, separados por vírgula |
| `--dry-run` | Mostra a requisição sem executar |
| `--env <AMBIENTE>` | Ambiente da API: production, sandbox ou development (cada produto tem os seus hosts). Também pode vir de KOBANA_ENVIRONMENT. |
| `--verbose` | Mostra os detalhes da requisição e da resposta no stderr |
| `--output <CAMINHO>` | Salva a resposta em um arquivo |
| `--output-format <FORMATO>` | Formato da saída: json (padrão), table, csv |
| `--idempotency-key <CHAVE>` | Chave de idempotência própria para alterações |
| `--page-all` | Percorre todas as páginas e imprime NDJSON |
| `--page-limit <N>` | Máximo de páginas a buscar (padrão: 10) |
| `--page-delay <MS>` | Intervalo entre páginas em ms (padrão: 100) |
| `--jq <EXPRESSÃO>` | Filtra a saída JSON com uma expressão jq, ex.: '.data[].id' ou, com --page-all (um item por vez), '.id'; não precisa do jq instalado |
| `--no-color` | Desliga as cores (também: NO_COLOR ou CLICOLOR=0); veja `kobana accessibility` |
| `--lang <IDIOMA>` | Idioma das mensagens: pt, en ou es (padrão: KOBANA_LANG ou o do sistema) |
| `-h, --help` | Mostra a ajuda |
| `-V, --version` | Mostra a versão |
