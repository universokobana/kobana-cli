---
id: auth
title: "kobana kadu auth"
sidebar_label: "auth"
description: "Log in, log out and show this product's credentials"
---

# kobana kadu auth

Log in, log out and show this product's credentials

Commands of `kobana kadu auth`. Every global option ([Global options](/manual/commands/global-options)) applies here.

| Command | HTTP | Description |
|---|---|---|
| [`auth login`](#auth-login) |  | Log in through the Conta Kobana (OAuth + PKCE) |
| [`auth logout`](#auth-logout) |  | Remove saved credentials |
| [`auth status`](#auth-status) |  | Show the authentication status |
| [`auth export`](#auth-export) |  | Export the credentials as JSON |

## auth login

Log in through the Conta Kobana (OAuth + PKCE)

```text
kobana kadu auth login [OPTIONS]
```

| Options | Description |
|---|---|
| `--client-id <ID>` | OAuth client ID (default: kobana-cli) |
| `--client-secret <SECRET>` | OAuth client secret: uses the Client Credentials flow, banking only, with --client-id (also: KOBANA_CLIENT_SECRET) |
| `--scopes <SCOPES>` | Comma-separated OAuth scopes, e.g. billing/customers,write (default: the ones the product publishes) |

## auth logout

Remove saved credentials

```text
kobana kadu auth logout [OPTIONS]
```

## auth status

Show the authentication status

```text
kobana kadu auth status [OPTIONS]
```

## auth export

Export the credentials as JSON

```text
kobana kadu auth export [OPTIONS]
```
