---
id: languages
title: Idiomas
sidebar_label: Idiomas
sidebar_position: 7
description: O CLI fala português, inglês e espanhol — ajuda, mensagens, erros e descrições dos endpoints.
---

# Idiomas

Tudo que o `kobana` diz a uma pessoa sai em **português, inglês ou espanhol**: ajuda, mensagens de progresso, perguntas, erros e a página que o navegador mostra no fim do login. As descrições dos endpoints (no `--help` e no `kobana schema`) vêm traduzidas das especificações que cada produto publica.

## Como o idioma é escolhido

Nesta ordem:

1. `--lang pt|en|es`
2. `KOBANA_LANG`
3. `LC_ALL`, `LC_MESSAGES`, `LANG`
4. O idioma do sistema

Locales `pt*` dão português, `es*` dão espanhol e qualquer outro dá inglês.

```bash
kobana --lang en banking bank-billets --help
export KOBANA_LANG=es
```

## O que não muda

Nomes de comandos e flags (inclusive os helpers em português, como `+emitir --valor`), chaves do JSON e códigos de saída são os mesmos em qualquer idioma — por isso um script escrito num idioma roda em qualquer máquina.

O CLI também envia `Accept-Language`, e as APIs que traduzem as próprias mensagens respondem no mesmo idioma.
