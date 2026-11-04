---
id: accessibility
title: Acessibilidade
sidebar_label: Acessibilidade
sidebar_position: 9
description: Cores desligáveis e de alto contraste, modo leitor de tela, perguntas simples e saída para scripts — no mesmo modelo do GitHub CLI.
---

# Acessibilidade

O `kobana` segue as práticas de acessibilidade do [GitHub CLI](https://cli.github.com/manual/gh_help_accessibility) (`gh accessibility`) para quem usa leitor de tela, linha braille ou alto contraste, ou tem sensibilidade a movimento. Tudo é ligado por uma opção ou por uma variável de ambiente; para manter a escolha, ponha a variável no `.env` da pasta de configuração (`~/.config/kobana/.env`).

```bash
kobana accessibility    # ou: kobana a11y — as opções e o que está ligado neste terminal
```

## Cores

As cores só reforçam o que o texto já diz: toda mensagem colorida tem uma palavra que diz a mesma coisa (`erro:`, `Aviso:`, `dica:`). O CLI usa só as 16 cores ANSI do terminal — que você redefine nas preferências dele —, nunca cinzas nem cores de 24 bits.

| Como | Efeito |
|---|---|
| `--no-color`, `NO_COLOR=1` (qualquer valor) ou `CLICOLOR=0` | Sem cores nem estilos |
| `CLICOLOR_FORCE=1` (qualquer valor) | Cores mesmo quando a saída não é um terminal (por exemplo, `\| less -R`) |
| `KOBANA_ACCESSIBLE_COLORS=1` | Alto contraste: negrito e sublinhado na cor do texto do seu tema, sem matizes — o contraste é o que o seu tema garante, claro ou escuro |

Fora de um terminal (pipe, arquivo) não há cores, a menos que `CLICOLOR_FORCE` peça; num terminal, também não há com `TERM=dumb` ou sem `TERM`. A ordem é `--no-color`, `NO_COLOR`, `CLICOLOR_FORCE`, `CLICOLOR` — as mesmas regras da ajuda, para que tudo concorde.

## Leitor de tela

```bash
export KOBANA_SCREEN_READER=1
```

Deixa a saída só com texto estático, que um leitor de tela ou uma linha braille lê do jeito que foi escrito:

- **Sem símbolos decorativos** (como `✓` e `└─`), que seriam lidos pelo nome.
- **Tabelas sem bordas** (`--output-format table`), com uma linha por registro: nem as bordas são soletradas, nem uma célula longa quebra o registro em duas linhas.
- **Perguntas simples** (veja abaixo).

O `kobana` não tem spinners nem animações: o progresso é sempre uma linha de texto nova, nunca uma linha redesenhada.

## Perguntas

O `kobana auth login` pergunta quais escopos pedir, e o `kobana update`, se pode instalar. Por padrão são listas interativas, redesenhadas a cada tecla. Com `KOBANA_ACCESSIBLE_PROMPTER=1` (ou `KOBANA_SCREEN_READER=1`), a pergunta é escrita uma vez e lida numa linha só:

```text
Níveis de acesso (valem para todos os recursos escolhidos)
  1. read: Leitura
  2. write: Criação e alteração
  3. delete: Exclusão
  4. import: Importação em volume
  5. export: Exportação em volume
Digite os números separados por vírgula (ex.: 1, 3, 5-7) ou "todos". Enter mantém: 1. 1, 2
Escolhidos: read, write
```

Para não perguntar nada, defina `KOBANA_PROMPT_DISABLED=1`.

## Scripts e agentes de IA

Sem uma pessoa no terminal — `stdin`, `stdout` e `stderr` não são todos um terminal, como num pipe, no CI ou num agente de IA — ou com `KOBANA_PROMPT_DISABLED=1`, o `kobana` não pergunta nada e usa o que a linha de comando diz:

- `auth login` pede todos os recursos publicados com acesso de leitura (`--scopes` escolhe outros);
- `update` só mostra o comando que instalaria;
- o aviso de versão nova não aparece (nem consulta a rede); `KOBANA_NO_UPDATE_NOTIFIER=1` o desliga também no terminal.

A saída é JSON (o padrão). `--fields` escolhe os campos e `--jq` filtra com uma expressão [jq](https://jqlang.github.io/jq/manual/), sem precisar do `jq` instalado — veja [Saída](/usage/output#filtrar-com---jq).

## Resumo

| Opção ou variável | `gh` equivalente | Efeito |
|---|---|---|
| `--no-color`, `NO_COLOR`, `CLICOLOR=0` | `NO_COLOR`, `CLICOLOR=0` | Sem cores |
| `CLICOLOR_FORCE` | `CLICOLOR_FORCE` | Cores fora do terminal |
| `KOBANA_ACCESSIBLE_COLORS` | `GH_ACCESSIBLE_COLORS` | Alto contraste, sem matizes |
| `KOBANA_SCREEN_READER` | `GH_ACCESSIBLE_PROMPTER` + `GH_SPINNER_DISABLED` | Texto estático: sem símbolos decorativos, tabelas sem bordas, perguntas simples |
| `KOBANA_ACCESSIBLE_PROMPTER` | `GH_ACCESSIBLE_PROMPTER` | Só as perguntas simples |
| `KOBANA_PROMPT_DISABLED` | `GH_PROMPT_DISABLED` | Nunca pergunta |
| `KOBANA_NO_UPDATE_NOTIFIER` | `GH_NO_UPDATE_NOTIFIER` | Sem aviso de versão nova |
| `--jq <EXPRESSÃO>` | `--jq` | Filtra o JSON da saída |

As variáveis valem quando têm um valor ligado: qualquer um, menos vazio, `0`, `false`, `no` e `off`. `NO_COLOR`, como manda o padrão dele, vale com qualquer valor não vazio.
