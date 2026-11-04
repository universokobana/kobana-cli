---
id: how-it-works
title: Como funciona
sidebar_label: Como funciona
sidebar_position: 4
description: Como o kobana monta os comandos a partir das especificações OpenAPI embutidas no binário.
---

# Como funciona

O `kobana` **não tem uma lista de endpoints escrita à mão**. Cada produto Kobana publica uma especificação OpenAPI 3.1, e o binário carrega essas especificações e monta a árvore de comandos delas a cada execução.

## Duas fases

1. **Descobrir o produto.** O CLI lê o primeiro argumento (`billing`, `finance`…) e carrega a especificação desse produto, já embutida no binário.
2. **Montar e executar.** Dela sai a árvore de comandos: cada caminho vira um recurso, cada operação vira um comando. Só então os argumentos restantes são lidos, a credencial do produto é resolvida, a requisição é montada e enviada.

O resultado é que `--help` em qualquer nível, o [Manual](/manual) e o [`kobana schema`](/usage/schema) respondem pela mesma fonte: a especificação.

## O que isso significa para você

- **Nenhum comando fica de fora da API.** Quando um produto publica um endpoint novo, a próxima versão do binário já o traz — sem escrever código por endpoint.
- **Sem rede para descobrir comandos.** A especificação está no binário: `--help`, `schema` e `--dry-run` funcionam offline.
- **A versão da API não aparece no comando.** `kobana banking bank-billets list` vai para `/v1/bank_billets` e `kobana banking charge pix list` vai para `/v2/charge/pix`.
- **Os nomes seguem como você fala.** Um recurso que só lê roda pelo nome (`kobana banking me`), e os ids da rota são argumentos (`bank-billets get 123`). Veja [Sintaxe](/usage/syntax).
- **Nomes antigos não quebram scripts.** Quando uma regra de nomes muda, o nome antigo continua valendo como alias.
- **Tudo que é texto vem traduzido.** As descrições da especificação são trocadas pela tradução do idioma escolhido ([Idiomas](/usage/languages)).

## Os helpers ficam de fora

Os poucos comandos escritos à mão são os [helpers](/usage/helpers) (`+emitir`, `+cobrar`…). Eles começam com `+` justamente para nunca colidirem com um comando gerado.

## Código aberto

O CLI é de código aberto (licença MIT) em [github.com/universokobana/kobana-cli](https://github.com/universokobana/kobana-cli), e é feito de duas partes: uma biblioteca (cliente HTTP, erros, leitura das especificações, validação de entrada) e o binário (comandos, autenticação, saída, paginação, helpers).
