---
id: auth
title: "kobana finance auth"
sidebar_label: "auth"
description: "Entra, sai e mostra as credenciais deste produto"
---

# kobana finance auth

Entra, sai e mostra as credenciais deste produto

Comandos de `kobana finance auth`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.

| Comando | HTTP | Descrição |
|---|---|---|
| [`auth login`](#auth-login) |  | Entra pela Conta Kobana (OAuth + PKCE) |
| [`auth logout`](#auth-logout) |  | Remove as credenciais salvas |
| [`auth status`](#auth-status) |  | Mostra o estado da autenticação |
| [`auth export`](#auth-export) |  | Exporta as credenciais em JSON |

## auth login

Entra pela Conta Kobana (OAuth + PKCE)

```text
kobana finance auth login [OPÇÕES]
```

| Opções | Descrição |
|---|---|
| `--client-id <ID>` | Client ID OAuth (padrão: kobana-cli) |
| `--client-secret <SECRET>` | Client secret OAuth: usa o fluxo Client Credentials, só no banking e com --client-id (também: KOBANA_CLIENT_SECRET) |
| `--scopes <ESCOPOS>` | Escopos OAuth separados por vírgula, ex.: billing/customers,write (padrão: os publicados pelo produto) |

## auth logout

Remove as credenciais salvas

```text
kobana finance auth logout [OPÇÕES]
```

## auth status

Mostra o estado da autenticação

```text
kobana finance auth status [OPÇÕES]
```

## auth export

Exporta as credenciais em JSON

```text
kobana finance auth export [OPÇÕES]
```
