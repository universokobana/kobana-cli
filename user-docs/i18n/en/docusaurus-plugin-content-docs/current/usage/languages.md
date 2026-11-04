---
id: languages
title: Languages
sidebar_label: Languages
sidebar_position: 7
description: The CLI speaks Portuguese, English and Spanish — help, messages, errors and endpoint descriptions.
---

# Languages

Everything `kobana` says to a person comes out in **Portuguese, English or Spanish**: help, progress messages, prompts, errors and the page the browser shows at the end of login. Endpoint descriptions (in `--help` and `kobana schema`) come translated from the specifications each product publishes.

## How the language is chosen

In this order:

1. `--lang pt|en|es`
2. `KOBANA_LANG`
3. `LC_ALL`, `LC_MESSAGES`, `LANG`
4. The system language

`pt*` locales give Portuguese, `es*` give Spanish and any other gives English.

```bash
kobana --lang en banking bank-billets --help
export KOBANA_LANG=es
```

## What does not change

Command and flag names (including the Portuguese helpers, such as `+emitir --valor`), JSON keys and exit codes are the same in any language — so a script written in one language runs on any machine.

The CLI also sends `Accept-Language`, and APIs that translate their own messages respond in the same language.
