---
id: index
title: Manual
sidebar_label: Visão geral
sidebar_position: 1
slug: /manual
description: Todos os comandos do kobana, por produto e recurso, gerados do próprio binário.
---

# Manual

O manual lista **todos os comandos** do `kobana`: o que cada um faz, o método e o caminho HTTP, os parâmetros, o corpo da requisição e as respostas. Ele é gerado do próprio binário — `--help`, `kobana schema` e as especificações OpenAPI embutidas —, então acompanha cada versão e cada idioma.

```bash
kobana <produto> <recurso> <método> [flags]
```

## Comandos do CLI

| Comando | Para quê |
|---|---|
| [Opções globais](/manual/commands/global-options) | As flags que todo comando aceita: `--params`, `--json`, `--fields`, `--dry-run`… |
| [`kobana schema`](/manual/commands/schema) | Parâmetros, corpo e respostas de um endpoint |
| [`kobana completions`](/manual/commands/completions) | Autocompletar para bash, zsh, fish, PowerShell e elvish |
| [`kobana update`](/manual/commands/update) | Procura uma versão nova e atualiza o CLI |

## Produtos

| Produto | Comando |
|---|---|
| Gateway Bancário | [`kobana banking`](/manual/banking) |
| Financeiro Inteligente | [`kobana finance`](/manual/finance) |
| Faturamento Automático | [`kobana billing`](/manual/billing) |
| Inbox Autônomo | [`kobana inbox`](/manual/inbox) |
| Régua de Cobrança | [`kobana dunning`](/manual/dunning) |
| Dashboard Mágico | [`kobana dashboards`](/manual/dashboards) |
| Agente Kadu | [`kobana kadu`](/manual/kadu) |

Cada produto tem uma página por recurso, com uma seção por comando.

## Como ler uma página

- **O bloco `text`** é a linha de uso, como o `--help` a imprime. `[ID]` são os identificadores do caminho, na ordem da rota; `[OPÇÕES]`, as [opções globais](/manual/commands/global-options).
- **`GET /v1/…`** é a chamada que o comando faz. A versão da API aparece só aí — nunca no comando.
- **Parâmetros de consulta, cabeçalhos e corpo** vão em `--params` e `--json`. A coluna *Obrigatório* diz o que a API exige.
- **Corpo da requisição** mostra os campos do primeiro nível. Para objetos aninhados, rode o `kobana schema` indicado na página.

## Erros comuns a todos os comandos

Todos os endpoints podem responder com estes códigos, que o manual não repete em cada comando:

| HTTP | Significado |
|---|---|
| `401` | Token ausente, inválido ou vencido — veja [Autenticação](/getting-started/authentication) |
| `403` | Token válido, mas sem permissão (ou sem o [nível de acesso](/getting-started/authentication#login-pelo-navegador-oauth--pkce) necessário) para a operação |
| `429` | Limite de requisições excedido |
| `500` | Erro inesperado no servidor |

No CLI, uma resposta 4xx ou 5xx termina com o [código de saída](/reference/exit-codes) `1`.
