---
id: troubleshooting
title: Solução de problemas
sidebar_label: Solução de problemas
sidebar_position: 3
description: Os erros mais comuns do kobana — autenticação, permissões, ambiente, espaço de trabalho — e como resolver.
---

# Solução de problemas

Erros de API, de autenticação e de validação saem como JSON no stdout, com `error.code` e `error.message`; o [código de saída](/reference/exit-codes) do processo diz a categoria. Rode com `--verbose` para ver a requisição e a resposta (sem o token).

## Autenticação (código de saída 2)

### `No authentication configured for <produto>`

Não há token para aquele produto. Cada produto tem o seu:

```bash
kobana billing auth login            # ou
export KOBANA_BILLING_TOKEN=...
kobana billing auth status           # confere o que está valendo
```

`auth status` termina com código `0` mesmo sem credencial: confira o campo `authenticated` da saída. Um token de outro produto não serve — veja [Autenticação](/getting-started/authentication).

### `401` da API

O token existe, mas está vencido, é de outro produto ou de outro ambiente. Os tokens de sandbox e de produção são diferentes. Refaça o login (`kobana <produto> auth login`) no ambiente certo (`--env`).

### `403` da API

O token é válido, mas não tem permissão. Causas comuns:

- **Nível de acesso faltando.** `DELETE` exige `delete`; importação exige `import`; escrita exige `write`. Faça o login de novo escolhendo os níveis, ou use `--scopes`.
- **Escopo do recurso faltando.** O login lista os escopos do produto (`billing/customers`, `inbox/emails`…); um recurso fora deles é recusado.

### O login abre o navegador no lugar errado

O login usa a Conta Kobana do ambiente (`account.kobana.com.br`, `account.sandbox.kobana.com.br`). `KOBANA_ACCOUNT_URL` troca o endereço; confira se não há uma variável antiga exportada.

## Ambiente

### `<produto> has no sandbox environment`

`inbox`, `dashboards` e `kadu` não têm sandbox, e o CLI recusa em vez de mandar o teste para produção. Rode sem `--env sandbox`, com cuidado, ou use `--dry-run`.

## Espaço de trabalho

### `400 WORKSPACE_REQUIRED` no `finance`

Toda rota do `finance` precisa de `X-Workspace-Id`:

```bash
export KOBANA_WORKSPACE_ID=uuid-do-espaco
# ou, numa chamada só:
kobana finance payables list --params '{"X-Workspace-Id": "uuid"}'
```

`kobana dunning workspaces list` mostra os espaços que o token alcança.

## Linha de comando (código de saída 3)

### `unrecognized subcommand` ou `unexpected argument`

O comando ou a flag não existe. Veja o que há naquele nível:

```bash
kobana billing --help
kobana billing plans --help
```

Nomes de comandos mudaram em versões antigas, mas os nomes antigos continuam valendo como alias. Se o comando "sumiu", procure no [Manual](/manual).

### `Validation error: id cannot contain …`

O CLI recusa identificadores com `/`, `\`, `..`, `?`, `#`, `%` ou caracteres de controle antes de chamar a API. É proposital: protege contra injeção de caminho. Passe só o id.

### JSON inválido em `--params` ou `--json`

Use aspas simples por fora e duplas por dentro (`--json '{"amount": 99.90}'`). No PowerShell, escape as aspas duplas internas ou use um arquivo.

## Variáveis e `.env`

### Um token no `.env` foi ignorado

O `.env` da pasta atual não pode definir tokens nem credenciais, porque pode vir de um repositório de terceiros. O CLI avisa. Defina-os no ambiente ou no `.env` da pasta de configuração (`~/.config/kobana`). Veja [Variáveis de ambiente](/reference/environment-variables).

## Saída

### Resposta enorme

Use `--fields` para limitar as colunas e `--page-limit` para limitar as páginas. Para somas, use os endpoints `aggregates`.

### `--output` recusado

O caminho não pode ter `..`, ser link simbólico, pasta ou dispositivo.

### O aviso "Nova versão disponível"

É um aviso no stderr, uma vez por dia. `kobana update` instala. Não afeta o stdout nem o código de saída.

## Ainda não resolveu?

Rode o comando com `--verbose` e `KOBANA_LOG=kobana=debug`, e abra uma [issue](https://github.com/universokobana/kobana-cli/issues) com o comando (sem tokens), a saída e `kobana --version`.
