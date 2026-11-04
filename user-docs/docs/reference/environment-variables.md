---
id: environment-variables
title: Variáveis de ambiente
sidebar_label: Variáveis de ambiente
sidebar_position: 2
description: Todas as variáveis que o kobana lê — credenciais, configuração e logs — e as regras do arquivo .env.
---

# Variáveis de ambiente

## Autenticação

| Variável | Descrição |
|---|---|
| `KOBANA_<PRODUTO>_TOKEN` | Token (Bearer) de um produto: `KOBANA_BANKING_TOKEN`, `KOBANA_FINANCE_TOKEN`… Tem a maior prioridade. O Kadu também lê `KADU_TOKEN` |
| `KOBANA_CREDENTIALS_FILE` | Caminho de um JSON de credenciais (o formato de `auth export`). Precisa do campo `product`; vale só para esse produto e para o `environment` do arquivo (sem ele, qualquer ambiente) |
| `KOBANA_CLIENT_ID` | Client ID do login (padrão: `kobana-cli`) |
| `KOBANA_CLIENT_SECRET` | Client secret. Como `--client-secret`, escolhe o fluxo Client Credentials (só no `banking`); o login de uma pessoa (PKCE) não usa secret |
| `KOBANA_ACCOUNT_URL` | Endereço da Conta Kobana usada no login (padrão: o do ambiente) |
| `KOBANA_WORKSPACE_ID` | Espaço de trabalho enviado em `X-Workspace-Id` pelo `finance` (obrigatório), pelo `dashboards` e pelo `dunning`. Um valor em `--params` vence |

## Configuração

| Variável | Descrição |
|---|---|
| `KOBANA_CONFIG_DIR` | Pasta de configuração (padrão: `~/.config/kobana`) |
| `KOBANA_ENVIRONMENT` | `production` (padrão), `sandbox` ou `development`; outro valor é erro |
| `KOBANA_LANG` | `pt`, `en` ou `es`. `--lang` vence |

## Logs

| Variável | Descrição |
|---|---|
| `KOBANA_LOG` | Nível de log no stderr, por exemplo `kobana=debug`. Desligado por padrão |
| `KOBANA_LOG_FILE` | Pasta para logs JSON com rotação diária. Desligado por padrão |

## Arquivo `.env`

Qualquer variável também pode viver num `.env`: o da pasta atual (nunca o das pastas acima) e o da pasta de configuração. Uma variável já definida no ambiente vence os dois.

> **O `.env` da pasta atual não define credenciais.** Ele pode vir de um repositório de terceiros, que poderia mandar o login para outro servidor, guardar credenciais dentro dele ou fazer o CLI agir com o token de outra pessoa. Por isso `KOBANA_ACCOUNT_URL`, `KOBANA_CONFIG_DIR`, `KOBANA_CREDENTIALS_FILE`, `KOBANA_CLIENT_ID`, `KOBANA_CLIENT_SECRET` e todos os tokens (`KOBANA_*_TOKEN`, `KADU_TOKEN`) são ignorados nele — o CLI avisa. Defina essas no ambiente ou no `.env` da pasta de configuração.
